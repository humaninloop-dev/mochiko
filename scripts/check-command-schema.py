#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.9"
# dependencies = ["pyyaml"]
# ///
"""Advisory checker for command content schemas — command-content-schema D13,
extended by command-md-scaffold-standardization D6-R1 (as review-widened).

Checks, all deterministic:
  1. schema + labels files parse as YAML (a parse break is a finding — the M5
     cost line names block scalars dense in ':'/'**'/backticks as the risk)
  2. `kind:` discriminators present (`command` / `command-labels`)
  2b. section grammar (D14): top-level `sections:` list, each {id, title, intent,
      rules}; section IDs `<cmd>.sec.<slug>`, minted once like rule IDs; a
      top-level flat `rules:` key is a finding (grammar superseded by D14)
  2c. breadth invariant (scaffold D5): the schema carries EXACTLY the six
      canonical section IDs for its prefix — roles · reserved · tools ·
      ways-of-working · boundaries · fail-conditions. Set-wise, not by count:
      under D5 every schema carries six, so a count-vs-count check is vacuous.
      A section with `rules: []` AND a one-line `note:` is a deliberate empty
      marker and valid; `rules: []` without a note is a finding.
  3. rule ID uniqueness + dotted-slug format (D6, as amended / D11 / D14)
  4. every rule label exists in the label registry (D8)
  5. every ${var} placeholder bound in vars: — an orphan placeholder is a
     finding; an unused var and a stray {{...}} sigil (the template-skeleton
     convention, never var substitution — D5) are warnings
  5b. referential closure (D15) — rule text carrying a deictic marker ("these
      rules", "this section", "above"/"below", …) is flagged: a reference whose
      referent lives outside the block dangles when the rule is quoted alone.
      Heuristic, warning-class only; curated list, grown on observed recurrence
  5c. tombstone-reference lint (scaffold D6-R1) — a rule text naming a
      `<prefix>.sec.<slug>` node that is tombstoned, or that names no node at
      all, is a finding. The D14 relocation precedent was same-vocabulary; the
      scaffold wave retires section nodes, so surviving text can dangle.
  6. provenance sidecar (D16): schemas carry runtime content only — an inline
     `ruling:` field is a finding (grammar superseded by D16). Anchors live
     repo-side in .mochiko/provenance.yaml (never shipped with the plugin),
     keyed by rule ID; every entry whose key matches this schema's prefix must
     name an existing rule ID (dangling = finding), be well-formed, and
     resolve against DECISIONS.md (D6 semantics unchanged). Sidecar absent =
     warning only (plugin-standalone checkout)
  7. the command .md's Not-done line hard-codes a count equal to the number of
     rules labeled `fail-condition` (D7, C2 guard), with the pluralization the
     count calls for — "the 1 rule labeled" / "the N rules labeled" (D6-R4)
  7b. the .md's "nested in N sections" phrase (optional) matches the schema's
      section count (D14 guard). Kept beside 2c: set-wise section assertion
      cannot see a stale prose numeral, which is what this phrase carries
  7c. canonical `.md` scaffold (scaffold D2/D6-R1) — the three canonical
      headings present and in D2 order, the Rules block enumerating exactly
      the schema's live section IDs, and the count-pinned Not-done line
      closing the Adaptive Goal Protocol (D2 pins it last, everywhere)
  7d. all-token resolution (scaffold D6-R1) — every `<prefix>.sec.*` token
      ANYWHERE in the .md, not only inside the Rules block, resolves to a live
      section node; a foreign-prefix token is named in a warning
  8. tombstone integrity (D11) — an ID is never both live and tombstoned

ruling: anchors resolve by DATE + SESSION-SLUG only. An anchor
"YYYY-MM-DD <session-slug> [D#]" resolves iff DECISIONS.md holds a table row
starting `| YYYY-MM-DD |` whose text contains <session-slug>. DECISIONS.md rows
carry no machine IDs, so a trailing D# is human-readable and machine-unverified
(limitation accepted at the build-wave ruling on the plan's FLAG 3).

Exit codes: 0 = clean, 1 = findings. ADVISORY ONLY — never a required CI gate,
never gating pipeline progress, never dispatching agents (GI-019 advisory
carve-out). Crate extension reserved per D9 — this script is standalone. Its
output is cited in the charter-audit brief as the deterministic pre-pass.

Run:  uv run scripts/check-command-schema.py            (the implement pair)
  or: uv run scripts/check-command-schema.py --all      (all six pairs)
  or: python3 scripts/check-command-schema.py   (needs: pip install pyyaml)
"""

import argparse
import re
import sys
from pathlib import Path

try:
    import yaml
except ImportError:
    print("FINDING: PyYAML unavailable — run via `uv run`, or `pip install pyyaml`")
    sys.exit(1)

ID_RE = re.compile(r"^[a-z][a-z0-9]*(\.fail)?\.[a-z0-9]+(-[a-z0-9]+)*$")
SEC_ID_RE = re.compile(r"^[a-z][a-z0-9]*\.sec\.[a-z0-9]+(-[a-z0-9]+)*$")
# Any `<prefix>.sec.<slug>` token wherever it appears — .md prose, rule text.
SEC_TOKEN_RE = re.compile(r"\b([a-z][a-z0-9]*)\.sec\.([a-z0-9]+(?:-[a-z0-9]+)*)\b")
VAR_RE = re.compile(r"\$\{([A-Za-z0-9_]+)\}")
SKELETON_SIGIL_RE = re.compile(r"\{\{[^}]*\}\}")
# D15 curated deixis markers — references that dangle when a rule is quoted alone.
# "this schema" / "the run" are legal self-reference and stay off this list.
DEIXIS_RE = re.compile(
    r"\b(these rules|this section|the section (above|below)|as stated (above|earlier)"
    r"|see (above|below)|aforementioned|there is no \S+ section)\b",
    re.IGNORECASE,
)
RULING_RE = re.compile(r"^(\d{4}-\d{2}-\d{2})\s+(\S+)(?:\s+D\d+.*)?$")
NOT_DONE_RE = re.compile(r"the (\d+) (rule|rules) labeled `fail-condition`")
# The count-pinned line's own opener, line-anchored so its placement can be checked. The
# numbered-list prefix is optional: the line is protocol step 3, but the step numbering is
# not what D2 pins — the position is.
NOT_DONE_LINE_RE = re.compile(r"^\s*(?:\d+\.\s+)?\*\*Not done — default FAIL:\*\*")
# .md "nested in N sections" phrase — optional, but when present must match len(sections)
SECTION_COUNT_RE = re.compile(r"nested in ([a-z]+|\d+) sections")
WORD_NUMS = {w: i for i, w in enumerate(
    "zero one two three four five six seven eight nine ten eleven twelve".split())}
CLASSES = {"floor", "must", "advisory"}
# The canonical six-section vocabulary every command schema carries (scaffold D4/D5).
CANONICAL_SLUGS = (
    "roles", "reserved", "tools", "ways-of-working", "boundaries", "fail-conditions",
)
# The canonical .md scaffold headings, in the order D2 fixes them.
CANONICAL_HEADINGS = (
    "## Identity & Mission",
    "## Rules — load the schema first",
    "## Adaptive Goal Protocol",
)
RULES_HEADING = CANONICAL_HEADINGS[1]
# The six shipped pairs, for --all. Schema and .md share a file stem.
PAIRS = ("architecture", "brainstorm", "feature", "implement", "setup", "specify")


def load_yaml(path: Path, findings: list):
    try:
        with open(path, encoding="utf-8") as f:
            return yaml.safe_load(f)
    except FileNotFoundError:
        findings.append(f"{path}: file not found")
    except yaml.YAMLError as e:
        findings.append(f"{path}: YAML parse break — {str(e).splitlines()[0]}")
    return None


def sec_tokens(text: str) -> list:
    """Every `<prefix>.sec.<slug>` token in `text`, in order of appearance."""
    return [f"{p}.sec.{slug}" for p, slug in SEC_TOKEN_RE.findall(str(text))]


def main() -> int:
    root = Path(__file__).resolve().parent.parent
    p = argparse.ArgumentParser(description="Advisory checker for command content schemas (D13)")
    p.add_argument("--schema", type=Path, default=root / "plugins/mochiko/schemas/implement.yaml")
    p.add_argument("--labels", type=Path, default=root / "plugins/mochiko/schemas/command-labels.yaml")
    p.add_argument("--provenance", type=Path, default=root / ".mochiko/provenance.yaml")
    p.add_argument("--md", type=Path, default=root / "plugins/mochiko/commands/implement.md")
    p.add_argument("--decisions", type=Path, default=root / "DECISIONS.md")
    p.add_argument("--all", action="store_true",
                   help="check all six shipped pairs; --schema/--md are ignored")
    a = p.parse_args()

    if not a.all:
        return check_pair(a.schema, a.md, a)

    worst = 0
    for name in PAIRS:
        print(f"=== {name} ===")
        rc = check_pair(
            root / f"plugins/mochiko/schemas/{name}.yaml",
            root / f"plugins/mochiko/commands/{name}.md",
            a,
        )
        worst = max(worst, rc)
    return worst


def check_pair(schema_path: Path, md_path: Path, a) -> int:
    findings: list = []
    warnings: list = []

    # 1. parse
    schema = load_yaml(schema_path, findings)
    registry = load_yaml(a.labels, findings)
    if schema is None or registry is None:
        return report(findings, warnings, {})

    # 2. kind: discriminators
    if schema.get("kind") != "command":
        findings.append(f"{schema_path.name}: `kind: command` missing (got {schema.get('kind')!r})")
    if not schema.get("command"):
        findings.append(f"{schema_path.name}: `command:` name missing")
    if registry.get("kind") != "command-labels":
        findings.append(f"{a.labels.name}: `kind: command-labels` missing (got {registry.get('kind')!r})")

    reg_labels = registry.get("labels") or {}
    if not isinstance(reg_labels, dict) or not reg_labels:
        findings.append(f"{a.labels.name}: `labels:` mapping missing or empty")
        reg_labels = {}

    # 2b. section grammar (D14) — flat top-level rules: is the superseded shape
    if "rules" in schema:
        findings.append(f"{schema_path.name}: top-level `rules:` — flat grammar superseded by sections (D14)")
    sections = schema.get("sections") or []
    if not isinstance(sections, list) or not sections:
        findings.append(f"{schema_path.name}: `sections:` list missing or empty (D14)")
        sections = []
    vars_block = schema.get("vars") or {}

    # 8. tombstone integrity (D11) — hoisted above the rule loop so the 5c/7d
    #    lints can tell a tombstoned node from a node that never existed.
    tombstoned = set()
    tombstones = schema.get("tombstones")
    if tombstones is not None:
        if not isinstance(tombstones, list):
            findings.append("tombstones: must be a list (D11)")
        else:
            for j, t in enumerate(tombstones):
                if not isinstance(t, dict) or not t.get("id") or not t.get("disposition"):
                    findings.append(f"tombstones[{j}]: entry needs `id` + `disposition` (D11)")
                    continue
                tid = t["id"]
                if tid in tombstoned:
                    findings.append(f"tombstones[{j}]: duplicate tombstone for {tid} (D11)")
                tombstoned.add(tid)

    # 3. ID uniqueness + slug format · section + rule shape · empty markers
    seen = {}
    rules = []          # (rule, section_id) pairs flattened for per-rule checks
    sec_stats = []
    section_ids = []
    empty_sections = 0
    for j, s in enumerate(sections):
        if not isinstance(s, dict):
            findings.append(f"sections[{j}]: not a mapping")
            continue
        sid = s.get("id", f"<sections[{j}] missing id>")
        if "id" not in s:
            findings.append(f"sections[{j}]: `id` missing (D14)")
        elif not SEC_ID_RE.match(sid):
            findings.append(f"{sid}: section id fails `<cmd>.sec.<slug>` format (D14)")
        else:
            section_ids.append(sid)
        if sid in seen:
            findings.append(f"{sid}: duplicate id (D11 — minted once)")
        seen[sid] = f"sections[{j}]"
        for field in ("title", "intent"):
            if not str(s.get(field) or "").strip():
                findings.append(f"{sid}: `{field}` missing or empty (D14)")

        # 2c. empty-marker recognition (D5): `rules: []` + a `note:` is deliberate.
        note = str(s.get("note") or "").strip()
        s_rules = s.get("rules")
        if "rules" not in s:
            findings.append(f"{sid}: `rules` key missing (D14)")
            s_rules = []
        elif s_rules is None or (isinstance(s_rules, list) and not s_rules):
            if note:
                empty_sections += 1
                if s_rules is None:
                    warnings.append(f"{sid}: empty section written as `rules:` — prefer explicit `rules: []` (D5)")
            else:
                findings.append(
                    f"{sid}: empty with no `note:` — a deliberately empty section carries a "
                    f"one-line note naming the emptiness deliberate (D5 breadth invariant)"
                )
            s_rules = []
        elif not isinstance(s_rules, list):
            findings.append(f"{sid}: `rules` must be a list (D14)")
            s_rules = []
        sec_stats.append((sid, len(s_rules)))
        rules.extend((r, sid) for r in s_rules)

    # 2c. breadth invariant — set-wise, against the six canonical IDs for this prefix
    prefix = derive_prefix(section_ids, seen, schema_path, findings)
    live_sections = set(section_ids)
    if prefix:
        expected = {f"{prefix}.sec.{slug}" for slug in CANONICAL_SLUGS}
        for missing in sorted(expected - live_sections):
            findings.append(
                f"{schema_path.name}: canonical section {missing} absent — every schema carries "
                f"all six, empty ones explicitly (D5 breadth invariant)"
            )
        for extra in sorted(live_sections - expected):
            findings.append(
                f"{extra}: not one of the six canonical sections for prefix {prefix!r} "
                f"({' · '.join(CANONICAL_SLUGS)}) — D4 unified vocabulary"
            )

    label_use = {name: 0 for name in reg_labels}
    used_vars = set()
    fail_count = 0
    class_counts = {c: 0 for c in CLASSES}
    for i, (r, sid) in enumerate(rules):
        if not isinstance(r, dict):
            findings.append(f"{sid}: rules[{i}] not a mapping")
            continue
        rid = r.get("id", f"<{sid} rules[{i}] missing id>")
        if "id" not in r:
            findings.append(f"{sid}: rules[{i}] `id` missing")
        elif not ID_RE.match(rid):
            findings.append(f"{rid}: id fails dotted-slug format (D6)")
        if rid in seen:
            findings.append(f"{rid}: duplicate id (first at {seen[rid]}, again in {sid})")
        seen[rid] = sid

        cls = r.get("class")
        if cls not in CLASSES:
            findings.append(f"{rid}: `class` must be floor|must|advisory (got {cls!r})")
        else:
            class_counts[cls] += 1

        # 4. labels ⊆ registry
        rl = r.get("labels")
        if not isinstance(rl, list) or not rl:
            findings.append(f"{rid}: `labels` missing or empty")
            rl = []
        for lab in rl:
            if lab not in reg_labels:
                findings.append(f"{rid}: label {lab!r} not in {a.labels.name} (D8)")
            else:
                label_use[lab] += 1
        if "fail-condition" in rl:
            fail_count += 1
            if ".fail." not in rid:
                findings.append(f"{rid}: labeled fail-condition but not under the .fail. segment (D6)")
        elif ".fail." in rid:
            findings.append(f"{rid}: under .fail. segment but not labeled fail-condition (D7)")

        text = r.get("text")
        if not text or not str(text).strip():
            findings.append(f"{rid}: `text` missing or empty")
            text = ""

        # 5. ${var} closure + stray skeleton sigils
        for m in VAR_RE.finditer(str(text)):
            used_vars.add(m.group(1))
            if m.group(1) not in vars_block:
                findings.append(f"{rid}: orphan placeholder ${{{m.group(1)}}} — unbound in vars: (D5)")
        if SKELETON_SIGIL_RE.search(str(text)):
            warnings.append(f"{rid}: contains a {{{{...}}}} sigil — skeleton convention, not var substitution (D5)")

        # 5b. referential closure — deixis lint (D15)
        dm = DEIXIS_RE.search(str(text))
        if dm:
            warnings.append(
                f"{rid}: deictic reference {dm.group(0)!r} — referent lives outside the block; "
                f"name it via the addressable namespace (D15)"
            )

        # 5c. tombstone-reference lint — rule text must name live section nodes only
        for tok in sec_tokens(text):
            if tok in live_sections:
                continue
            if tok in tombstoned:
                findings.append(f"{rid}: text names tombstoned section {tok} (D11 — relocate the reference)")
            else:
                findings.append(f"{rid}: text names section {tok}, which is not a node in {schema_path.name}")

        # 6. runtime-only schemas (D16): inline ruling: is superseded grammar
        if "ruling" in r:
            findings.append(f"{rid}: inline `ruling:` — provenance lives in provenance.yaml (D16)")

    for v in vars_block:
        if v not in used_vars:
            warnings.append(f"vars.{v}: declared but unused by any rule text")
    for lab, n in label_use.items():
        if n == 0:
            warnings.append(f"label {lab!r}: zero members in {schema_path.name} (registry-legal; watch at rollout)")

    # 6. provenance sidecar (D16) — anchors keyed by rule ID, repo-side
    anchor_count = 0
    prefixes = {rid.split(".")[0] for rid in seen if "." in rid}
    if not a.provenance.exists():
        warnings.append(f"{a.provenance}: provenance sidecar absent — anchor checks skipped (plugin-standalone?)")
    else:
        prov = load_yaml(a.provenance, findings)
        if prov is not None:
            if prov.get("kind") != "command-provenance":
                findings.append(f"{a.provenance.name}: `kind: command-provenance` missing (got {prov.get('kind')!r})")
            p_anchors = prov.get("anchors") or {}
            if not isinstance(p_anchors, dict):
                findings.append(f"{a.provenance.name}: `anchors:` mapping missing")
                p_anchors = {}
            skipped_prefixes = set()
            for key, anchor in p_anchors.items():
                if key.split(".")[0] not in prefixes:
                    skipped_prefixes.add(key.split(".")[0])
                    continue  # another command's entry — validated on its own run
                anchor_count += 1
                if key not in seen:
                    findings.append(f"{a.provenance.name}: dangling entry {key!r} — no such rule in {schema_path.name} (D16)")
                m = RULING_RE.match(str(anchor).strip())
                if not m:
                    findings.append(f"{key}: anchor {anchor!r} malformed — want 'YYYY-MM-DD <session-slug> [D#]'")
                elif not resolve_anchor(a.decisions, m.group(1), m.group(2)):
                    findings.append(f"{key}: anchor '{m.group(1)} {m.group(2)}' resolves to no DECISIONS.md row (D6)")
            if skipped_prefixes:
                warnings.append(
                    "provenance entries with foreign prefixes skipped (validated on their own runs): "
                    + ", ".join(sorted(skipped_prefixes))
                )

    # 7. the .md side — count-pin, scaffold, token resolution
    check_md(md_path, prefix, live_sections, tombstoned, fail_count, len(sections),
             findings, warnings)

    # 8. tombstone integrity (D11) — an ID is never both live and tombstoned
    for tid in sorted(tombstoned):
        if tid in seen:
            findings.append(f"{tid}: both live and tombstoned — an ID is minted once (D11)")

    stats = {
        "sections": len(sections),
        "empty-sections": empty_sections,
        "rules": len(rules),
        "floor": class_counts["floor"],
        "must": class_counts["must"],
        "advisory": class_counts["advisory"],
        "fail-condition": fail_count,
        "vars": len(vars_block),
        "labels": len(reg_labels),
        "anchors": anchor_count,
    }
    return report(findings, warnings, stats, sec_stats)


def derive_prefix(section_ids: list, seen: dict, schema_path: Path, findings: list):
    """The schema's rule-ID prefix, read off its own section IDs.

    D4 froze the prefixes and the grammar carries no `prefix:` field, so the section IDs
    are the source. A schema whose sections disagree cannot be checked set-wise.
    """
    found = {sid.split(".")[0] for sid in section_ids}
    if len(found) == 1:
        return found.pop()
    if not found:
        findings.append(f"{schema_path.name}: no well-formed section IDs — cannot derive the rule prefix")
        return None
    findings.append(
        f"{schema_path.name}: section IDs disagree on the rule prefix ({', '.join(sorted(found))}) "
        f"— one schema carries one prefix (D4)"
    )
    return None


def check_md(md_path: Path, prefix, live_sections: set, tombstoned: set,
             fail_count: int, section_count: int, findings: list, warnings: list) -> None:
    try:
        raw = md_path.read_text(encoding="utf-8")
    except FileNotFoundError:
        findings.append(f"{md_path}: file not found — cannot run the C2 count guard")
        return
    # The count-pin phrase is matched against a whitespace-collapsed copy so it survives
    # a line wrap; every structural check below reads the raw text, which keeps line
    # anchors intact.
    flat = re.sub(r"\s+", " ", raw)

    # 7. Not-done count-pin + the pluralization the count calls for (D7 C2 guard, D6-R4)
    m = NOT_DONE_RE.search(flat)
    if not m:
        findings.append(f"{md_path.name}: no Not-done line hard-coding the `fail-condition` count (D7 C2 guard)")
    else:
        pinned, word = int(m.group(1)), m.group(2)
        if pinned != fail_count:
            findings.append(
                f"pair out of sync: {md_path.name} pins {pinned} fail-condition rules, "
                f"schema carries {fail_count} (D7 C2 guard)"
            )
        want = "rule" if pinned == 1 else "rules"
        if word != want:
            findings.append(
                f"{md_path.name}: Not-done line reads 'the {pinned} {word} labeled' — "
                f"want 'the {pinned} {want} labeled' (D6-R4)"
            )

    # 7b. "nested in N sections" phrase, when present, must match the section count
    sm = SECTION_COUNT_RE.search(flat)
    if sm:
        claimed = WORD_NUMS.get(sm.group(1), None)
        if claimed is None and sm.group(1).isdigit():
            claimed = int(sm.group(1))
        if claimed is not None and claimed != section_count:
            findings.append(
                f"pair out of sync: {md_path.name} says 'nested in {sm.group(1)} sections', "
                f"schema carries {section_count} (D14 section-count guard)"
            )

    # 7c. canonical scaffold — headings present, in D2 order
    heading_lines = [ln.rstrip() for ln in raw.splitlines() if ln.startswith("## ")]
    where = {}
    for h in CANONICAL_HEADINGS:
        if h in heading_lines:
            where[h] = heading_lines.index(h)
        else:
            findings.append(f"{md_path.name}: canonical heading `{h}` absent (scaffold D2)")
    present = [h for h in CANONICAL_HEADINGS if h in where]
    if len(present) > 1 and [where[h] for h in present] != sorted(where[h] for h in present):
        observed = " · ".join(sorted(present, key=lambda h: where[h]))
        findings.append(
            f"{md_path.name}: canonical headings out of D2 order — observed {observed} "
            f"(want Identity & Mission · Rules · Adaptive Goal Protocol)"
        )

    # 7c. the count-pinned Not-done line closes the Adaptive Goal Protocol (D2: "always last").
    # The search is anchored to the region below the protocol heading, never to the file's first
    # numbered list: an Identity & Mission section may carry a numbered list of its own
    # (implement.md does), and a scan keyed to list position would read that one instead.
    lines = raw.splitlines()
    protocol_at = next(
        (i for i, ln in enumerate(lines) if ln.rstrip() == CANONICAL_HEADINGS[2]), None)
    if protocol_at is not None:
        in_protocol = next(
            (i for i in range(protocol_at + 1, len(lines)) if NOT_DONE_LINE_RE.match(lines[i])),
            None,
        )
        above_protocol = any(NOT_DONE_LINE_RE.match(ln) for ln in lines[:protocol_at])
        if in_protocol is not None:
            trailing = [ln for ln in lines[in_protocol + 1:] if ln.startswith("## ")]
            if trailing:
                findings.append(
                    f"{md_path.name}: `{trailing[0].rstrip()}` follows the Not-done line — the "
                    f"count-pinned line is always last (scaffold D2)"
                )
        elif above_protocol:
            findings.append(
                f"{md_path.name}: the Not-done line sits above `{CANONICAL_HEADINGS[2]}` — its "
                f"home is protocol step 3, everywhere (scaffold D2)"
            )
        elif m:
            findings.append(
                f"{md_path.name}: the count-pin is not on a `**Not done — default FAIL:**` line "
                f"— its placement cannot be checked (scaffold D2)"
            )

    # 7c. the Rules block enumerates exactly the schema's live section IDs
    already_named = set()
    if RULES_HEADING in where:
        enumerated = set(sec_tokens(rules_block(raw)))
        already_named = enumerated - live_sections
        for missing in sorted(live_sections - enumerated):
            findings.append(
                f"{md_path.name}: Rules block does not enumerate {missing} — the block names "
                f"every section the schema carries (scaffold D6-R1)"
            )
        for extra in sorted(enumerated - live_sections):
            findings.append(
                f"{md_path.name}: Rules block enumerates {extra}, which is not a section in the "
                f"paired schema (scaffold D6-R1)"
            )

    # 7d. all-token resolution — anywhere in the .md, not only the Rules block. Tokens the
    # enumeration check above already named are skipped: this lint exists to reach the sites
    # that check misses (I2), and reporting one dangling token twice buries the second one.
    foreign = set()
    for tok in dict.fromkeys(sec_tokens(raw)):
        tok_prefix = tok.split(".")[0]
        if tok in already_named:
            continue
        if prefix and tok_prefix != prefix:
            foreign.add(tok_prefix)
        elif tok not in live_sections:
            if tok in tombstoned:
                findings.append(f"{md_path.name}: names tombstoned section {tok} — re-key the reference (D11)")
            else:
                findings.append(f"{md_path.name}: names section {tok}, which is not a node in the paired schema")
    if foreign:
        warnings.append(
            f"{md_path.name}: section tokens with foreign prefixes, unresolvable against this pair: "
            + ", ".join(sorted(foreign))
        )


def rules_block(raw: str) -> str:
    """The `## Rules — load the schema first` block: its heading to the next `## ` heading."""
    lines = raw.splitlines()
    start = next(i for i, ln in enumerate(lines) if ln.rstrip() == RULES_HEADING)
    end = len(lines)
    for i in range(start + 1, len(lines)):
        if lines[i].startswith("## "):
            end = i
            break
    return "\n".join(lines[start:end])


def resolve_anchor(decisions: Path, date: str, slug: str) -> bool:
    try:
        for line in decisions.read_text(encoding="utf-8").splitlines():
            if line.startswith(f"| {date} |") and slug in line:
                return True
    except FileNotFoundError:
        return False
    return False


def report(findings: list, warnings: list, stats: dict, sec_stats: list = None) -> int:
    for f in findings:
        print(f"FINDING: {f}")
    for w in warnings:
        print(f"warning: {w}")
    if stats:
        print(
            "stats: "
            + " · ".join(f"{k} {v}" for k, v in stats.items())
        )
    if sec_stats:
        print("per-section: " + " · ".join(f"{sid} {n}" for sid, n in sec_stats))
    verdict = "FAIL" if findings else "PASS"
    print(f"check-command-schema: {len(findings)} findings, {len(warnings)} warnings — {verdict} (advisory)")
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
