#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.9"
# dependencies = ["pyyaml"]
# ///
"""Similar-rule detector over the six command content schemas — layer 1 of the
similar-items grooming system (proposed 2026-08-28, conversation-born; no session
record yet — the merge POLICY for near-duplicates is unruled and gets ruled when the
first real merge set is on the table, evidence-first).

The detector PROPOSES candidate clusters; it never merges, never edits, never gates.
Combining is judgment: layer 2 is an agent/lead pass that disposes each cluster
(promote-to-common / merge-in-place / keep-distinct), layer 3 is the user ruling plus
the standard landing ceremony (strips, author≠grader audit, tombstones per ontology
D11, `extends:` stubs per D8). Advisory only, exit 0 by default — GI-019 advisory
carve-out, same standing as check-command-schema.py.

How it scores, all deterministic and offline:
  1. Extract every live rule from the six pairs (tombstones skipped), with its
     resolved text — an `extends: common.<slug>` stub reads the block's text/labels/
     pointer per D8/C3, so a plain rule in one command that matches a stub's resolved
     text in another surfaces as a promotion candidate (an EXTEND-GAP).
  2. Bucket by `kind:` (ontology D1; absent reads constraint). Cross-kind pairs are
     never scored — a duty and a fail predicate with similar words are different
     ontological roles, not merge candidates.
  3. Normalize text for scoring only (report shows raw): lowercase, `${var}` → «var»,
     `/mochiko:<cmd>` → «cmd», own-prefix ID citations → «self».<slug>, punctuation
     stripped. Cross-schema rules that differ only by their command's own names score
     as the same wording.
  4. Pair score = max(sequence ratio, token-sort ratio) via difflib, plus a capped
     structural bonus (+0.08 same non-null `pointer:`, +0.04 same section slug,
     +0.04 labels Jaccard ≥ 0.5; bonus cap 0.12, combined cap 1.00 — 1.00 reads
     "identical"). Threshold default 0.60 on the combined score. Short texts are
     ratio-noisy, so when either side has fewer than 6 normalized tokens the pair
     needs text similarity ≥ 0.80 — one-line fail predicates sharing only their
     grammatical frame ("A run without…" / "A failing…") stay apart. Pairs where
     both sides extend the SAME common block are skipped — already combined.
  5. Clusters = union-find over above-threshold pairs. A cluster can chain (A~B~C
     with A≁C), so the report prints the weakest internal edge alongside the best.

Cluster classifications:
  COMMON-CANDIDATE  members span 3+ commands — meets the D8 exact-duplicate bar's
                    count; text near-match still needs the unruled near-dup policy
  CROSS-PAIR        spans exactly 2 commands — below the D8 bar, watchlist
  INTRA-SCHEMA      one schema — merge-in-place candidate
  EXTEND-GAP        (additional tag) some members already extend a common block,
                    others carry their own text — promotion candidate
Members carrying `class: floor` are flagged ⚑floor: merging a floor is a recorded
supersession-by-ruling, never a quiet edit.

Allowlist: scripts/similar-rules-allowlist.yaml — adjudicated keep-distinct pairs,
each {ids: [a, b], reason}. Suppressed edges never pair, so reruns stay quiet after
a judgment pass. A stale entry (an ID no longer live) is named in a warning.

Exit codes: 0 always, unless --exit-signal is passed (then 1 when any cluster is
found — the optional exit-code signal GI-019 licenses). Never a required CI gate.

Run:  uv run scripts/find-similar-rules.py                 (all six pairs)
  or: uv run scripts/find-similar-rules.py --min 0.7       (tighter threshold)
  or: uv run scripts/find-similar-rules.py --full          (untrimmed texts)
  or: uv run scripts/find-similar-rules.py --json          (machine-readable, for the
                                                            layer-2 judgment pass)
"""

import argparse
import difflib
import json
import re
import sys
from itertools import combinations
from pathlib import Path

try:
    import yaml
except ImportError:
    print("FINDING: PyYAML unavailable — run via `uv run`, or `pip install pyyaml`")
    sys.exit(1)

PAIRS = ("architecture", "brainstorm", "feature", "implement", "setup", "specify")
KINDS = ("constraint", "duty", "gate", "reservation", "binding", "bound",
         "routing", "fail", "latitude")
INHERITED_FIELDS = ("text", "labels", "pointer")
BONUS_CAP = 0.12
SHORT_TOKENS = 6       # below this, difflib ratios are frame-noise, not similarity
SHORT_TEXT_SIM = 0.80  # …so a short pair must be near-exact to pair

VAR_RE = re.compile(r"\$\{[A-Za-z0-9_]+\}")
CMD_RE = re.compile(r"/mochiko:[a-z0-9-]+")


class Rule:
    __slots__ = ("schema", "prefix", "rid", "section", "kind", "cls", "labels",
                 "pointer", "extends", "text", "norm")

    def __init__(self, schema, prefix, rid, section, kind, cls, labels, pointer,
                 extends, text):
        self.schema = schema
        self.prefix = prefix
        self.rid = rid
        self.section = section          # trailing slug of the section ID
        self.kind = kind
        self.cls = cls
        self.labels = frozenset(labels or [])
        self.pointer = pointer
        self.extends = extends          # common.<slug> or None
        self.text = text                # resolved (post-extends) raw text
        self.norm = norm_for_sim(text, prefix)


def norm_for_sim(text: str, prefix: str) -> str:
    """Scoring-only normalization — the report always shows raw text."""
    t = str(text).lower()
    t = VAR_RE.sub("«var»", t)
    t = CMD_RE.sub("«cmd»", t)
    # Own-prefix citations (rule, section, fail segment) read as self-reference so
    # cross-schema rules differing only by their own prefix score as one wording.
    t = re.sub(rf"\b{re.escape(prefix)}\.((?:sec|fail)\.)?", "«self».", t)
    t = re.sub(r"[^a-z0-9«»\.\s]", " ", t)
    return re.sub(r"\s+", " ", t).strip()


def load_common(path: Path):
    if not path.exists():
        return {}
    doc = yaml.safe_load(path.read_text(encoding="utf-8")) or {}
    return {b["id"]: b for b in doc.get("rules", [])
            if isinstance(b, dict) and b.get("id")}


def schema_paths(root: Path, schemas_dir) -> list:
    """The six shipped pairs, or every non-common non-labels schema in --schemas-dir."""
    if schemas_dir is None:
        return [(name, root / f"plugins/mochiko/schemas/{name}.yaml") for name in PAIRS]
    return sorted(
        (p.stem, p) for p in Path(schemas_dir).glob("*.yaml")
        if p.name not in ("common.yaml", "command-labels.yaml")
    )


def load_rules(paths: list, common: dict, warnings: list):
    rules = []
    for name, path in paths:
        if not path.exists():
            warnings.append(f"{path.name}: absent, skipped")
            continue
        doc = yaml.safe_load(path.read_text(encoding="utf-8")) or {}
        sections = doc.get("sections") or []
        prefix = None
        for sec in sections:
            sid = str(sec.get("id") or "")
            if ".sec." in sid:
                prefix = sid.split(".sec.")[0]
                break
        if prefix is None:
            warnings.append(f"{path.name}: no section IDs, skipped")
            continue
        for sec in sections:
            sec_slug = str(sec.get("id") or "").split(".sec.")[-1]
            for r in sec.get("rules") or []:
                if not isinstance(r, dict) or not r.get("id"):
                    continue
                eff = {f: r.get(f) for f in INHERITED_FIELDS}
                target = r.get("extends")
                if target and target in common:
                    for f in INHERITED_FIELDS:
                        if eff[f] is None:
                            eff[f] = common[target].get(f)
                text = str(eff["text"] or "").strip()
                if not text:
                    warnings.append(f"{r['id']}: empty resolved text, skipped")
                    continue
                rules.append(Rule(
                    schema=name, prefix=prefix, rid=str(r["id"]),
                    section=sec_slug, kind=str(r.get("kind") or "constraint"),
                    cls=str(r.get("class") or ""), labels=eff["labels"],
                    pointer=eff["pointer"], extends=target, text=text,
                ))
    return rules


def load_allowlist(path: Path, live_ids: set, warnings: list) -> set:
    """Unordered ID pairs adjudicated keep-distinct."""
    suppressed = set()
    if not path.exists():
        return suppressed
    doc = yaml.safe_load(path.read_text(encoding="utf-8")) or {}
    for j, entry in enumerate(doc.get("suppressions") or []):
        ids = entry.get("ids") if isinstance(entry, dict) else None
        if not (isinstance(ids, list) and len(ids) == 2):
            warnings.append(f"allowlist suppressions[{j}]: needs `ids: [a, b]`")
            continue
        a, b = str(ids[0]), str(ids[1])
        for rid in (a, b):
            if rid not in live_ids:
                warnings.append(f"allowlist names {rid}: not a live rule ID (stale entry?)")
        if not str(entry.get("reason") or "").strip():
            warnings.append(f"allowlist pair ({a}, {b}): no `reason` recorded")
        suppressed.add(frozenset((a, b)))
    return suppressed


def text_sim(a: str, b: str, floor: float) -> float:
    sm = difflib.SequenceMatcher(None, a, b)
    if sm.real_quick_ratio() < floor or sm.quick_ratio() < floor:
        return 0.0
    r = sm.ratio()
    ts_a = " ".join(sorted(a.split()))
    ts_b = " ".join(sorted(b.split()))
    return max(r, difflib.SequenceMatcher(None, ts_a, ts_b).ratio())


def struct_bonus(x: Rule, y: Rule) -> float:
    bonus = 0.0
    if x.pointer and y.pointer and x.pointer == y.pointer:
        bonus += 0.08
    if x.section == y.section:
        bonus += 0.04
    if x.labels and y.labels:
        jac = len(x.labels & y.labels) / len(x.labels | y.labels)
        if jac >= 0.5:
            bonus += 0.04
    return min(bonus, BONUS_CAP)


def score_pairs(rules, threshold: float, suppressed: set):
    """Above-threshold edges, bucketed by kind."""
    by_kind = {}
    for r in rules:
        by_kind.setdefault(r.kind, []).append(r)
    edges = []
    scored = 0
    suppressed_hits = 0
    for bucket in by_kind.values():
        for x, y in combinations(bucket, 2):
            if x.extends and x.extends == y.extends:
                continue  # both read the same common block — already combined
            scored += 1
            # Bonus is capped, so text alone must clear threshold - cap to matter.
            sim = text_sim(x.norm, y.norm, threshold - BONUS_CAP)
            if sim == 0.0:
                continue
            if min(len(x.norm.split()), len(y.norm.split())) < SHORT_TOKENS \
                    and sim < SHORT_TEXT_SIM:
                continue
            total = min(1.0, sim + struct_bonus(x, y))
            if total < threshold:
                continue
            if frozenset((x.rid, y.rid)) in suppressed:
                suppressed_hits += 1
                continue
            edges.append((total, sim, x, y))
    return edges, scored, suppressed_hits


def cluster(edges, rules):
    parent = {r.rid: r.rid for r in rules}

    def find(a):
        while parent[a] != a:
            parent[a] = parent[parent[a]]
            a = parent[a]
        return a

    for _, _, x, y in edges:
        ra, rb = find(x.rid), find(y.rid)
        if ra != rb:
            parent[ra] = rb
    groups = {}
    for total, sim, x, y in edges:
        groups.setdefault(find(x.rid), []).append((total, sim, x, y))
    clusters = []
    for group_edges in groups.values():
        members = {}
        for total, sim, x, y in group_edges:
            members[x.rid] = x
            members[y.rid] = y
        clusters.append({
            "members": sorted(members.values(), key=lambda r: (r.schema, r.rid)),
            "edges": sorted(group_edges, key=lambda e: -e[0]),
        })
    return clusters


def classify(c) -> list:
    schemas = {m.schema for m in c["members"]}
    tags = []
    if len(schemas) >= 3:
        tags.append("COMMON-CANDIDATE")
    elif len(schemas) == 2:
        tags.append("CROSS-PAIR")
    else:
        tags.append("INTRA-SCHEMA")
    extends = [bool(m.extends) for m in c["members"]]
    if any(extends) and not all(extends):
        tags.append("EXTEND-GAP")
    return tags


def cluster_sort_key(c):
    schemas = len({m.schema for m in c["members"]})
    best = c["edges"][0][0]
    return (-schemas, -best)


def trim(text: str, full: bool, width: int = 140) -> str:
    t = " ".join(str(text).split())
    if full or len(t) <= width:
        return t
    return t[: width - 1] + "…"


def print_report(clusters, rules, scored, suppressed_hits, warnings, a):
    print(f"=== similar-rule clusters (threshold {a.min:.2f}) ===")
    if not clusters:
        print("none — no pair clears the threshold")
    for i, c in enumerate(sorted(clusters, key=cluster_sort_key), 1):
        tags = " + ".join(classify(c))
        kinds = sorted({m.kind for m in c["members"]})
        best = c["edges"][0][0]
        worst = c["edges"][-1][0]
        edge_note = f"best {best:.2f}" if len(c["edges"]) == 1 else \
            f"best {best:.2f} · weakest edge {worst:.2f}"
        print(f"\n[{i}] {tags} · kind: {'/'.join(kinds)} · {edge_note}")
        for m in c["members"]:
            marks = []
            if m.cls == "floor":
                marks.append("⚑floor")
            if m.extends:
                marks.append(f"extends {m.extends}")
            mark = ("  [" + ", ".join(marks) + "]") if marks else ""
            print(f"    {m.rid}  ({m.schema} · sec.{m.section} · {m.cls or '?'}){mark}")
            print(f"      {trim(m.text, a.full)}")
    print("\n=== stats ===")
    counts = {}
    for c in clusters:
        counts[classify(c)[0]] = counts.get(classify(c)[0], 0) + 1
    print(f"rules scanned: {len(rules)} · in-kind pairs scored: {scored} · "
          f"clusters: {len(clusters)} "
          f"({', '.join(f'{k} {v}' for k, v in sorted(counts.items())) or 'none'})")
    if suppressed_hits:
        print(f"allowlist-suppressed edges: {suppressed_hits}")
    for w in warnings:
        print(f"warning: {w}")


def json_report(clusters, a):
    out = []
    for c in sorted(clusters, key=cluster_sort_key):
        out.append({
            "tags": classify(c),
            "best": round(c["edges"][0][0], 3),
            "weakest_edge": round(c["edges"][-1][0], 3),
            "members": [{
                "id": m.rid, "schema": m.schema, "section": m.section,
                "kind": m.kind, "class": m.cls, "extends": m.extends,
                "pointer": m.pointer, "text": m.text,
            } for m in c["members"]],
            "edges": [{"a": x.rid, "b": y.rid, "score": round(total, 3),
                       "text_sim": round(sim, 3)}
                      for total, sim, x, y in c["edges"]],
        })
    print(json.dumps(out, ensure_ascii=False, indent=2))


def main() -> int:
    root = Path(__file__).resolve().parent.parent
    p = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    p.add_argument("--min", type=float, default=0.60,
                   help="combined-score threshold (default 0.60)")
    p.add_argument("--full", action="store_true", help="untrimmed rule texts")
    p.add_argument("--json", action="store_true", help="machine-readable clusters")
    p.add_argument("--exit-signal", action="store_true",
                   help="exit 1 when any cluster is found (GI-019 optional signal)")
    p.add_argument("--schemas-dir", type=Path, default=None,
                   help="scan every schema in this directory instead of the six "
                        "shipped pairs (fixture/test use)")
    p.add_argument("--common", type=Path, default=None,
                   help="shared block library (default: beside the schemas)")
    p.add_argument("--allowlist", type=Path,
                   default=root / "scripts/similar-rules-allowlist.yaml")
    a = p.parse_args()

    if a.common is None:
        base = a.schemas_dir if a.schemas_dir else root / "plugins/mochiko/schemas"
        a.common = Path(base) / "common.yaml"

    warnings = []
    common = load_common(a.common)
    rules = load_rules(schema_paths(root, a.schemas_dir), common, warnings)
    suppressed = load_allowlist(a.allowlist, {r.rid for r in rules}, warnings)
    edges, scored, suppressed_hits = score_pairs(rules, a.min, suppressed)
    clusters = cluster(edges, rules)

    if a.json:
        json_report(clusters, a)
    else:
        print_report(clusters, rules, scored, suppressed_hits, warnings, a)
    return 1 if (a.exit_signal and clusters) else 0


if __name__ == "__main__":
    sys.exit(main())
