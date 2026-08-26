#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.9"
# dependencies = ["pyyaml"]
# ///
"""Advisory checker for command content schemas — command-content-schema D13.

Checks, all deterministic:
  1. schema + labels files parse as YAML (a parse break is a finding — the M5
     cost line names block scalars dense in ':'/'**'/backticks as the risk)
  2. `kind:` discriminators present (`command` / `command-labels`)
  2b. section grammar (D14): top-level `sections:` list, each {id, title, intent,
      rules}; section IDs `<cmd>.sec.<slug>`, minted once like rule IDs; a
      top-level flat `rules:` key is a finding (grammar superseded by D14)
  3. rule ID uniqueness + dotted-slug format (D6, as amended / D11 / D14)
  4. every rule label exists in the label registry (D8)
  5. every ${var} placeholder bound in vars: — an orphan placeholder is a
     finding; an unused var and a stray {{...}} sigil (the template-skeleton
     convention, never var substitution — D5) are warnings
  5b. referential closure (D15) — rule text carrying a deictic marker ("these
      rules", "this section", "above"/"below", …) is flagged: a reference whose
      referent lives outside the block dangles when the rule is quoted alone.
      Heuristic, warning-class only; curated list, grown on observed recurrence
  6. every ruling: anchor resolves against DECISIONS.md (D6)
  7. the command .md's Not-done line hard-codes a count equal to the number of
     rules labeled `fail-condition` (D7, C2 guard)
  8. tombstone integrity (D11) — vacuous while the schema carries no
     tombstones: key (first mint)

ruling: anchors resolve by DATE + SESSION-SLUG only. An anchor
"YYYY-MM-DD <session-slug> [D#]" resolves iff DECISIONS.md holds a table row
starting `| YYYY-MM-DD |` whose text contains <session-slug>. DECISIONS.md rows
carry no machine IDs, so a trailing D# is human-readable and machine-unverified
(limitation accepted at the build-wave ruling on the plan's FLAG 3).

Exit codes: 0 = clean, 1 = findings. ADVISORY ONLY — never a required CI gate,
never gating pipeline progress, never dispatching agents (GI-019 advisory
carve-out). Crate extension reserved per D9 — this script is standalone. Its
output is cited in the charter-audit brief as the deterministic pre-pass.

Run:  uv run scripts/check-command-schema.py
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
NOT_DONE_RE = re.compile(r"the (\d+) rules labeled `fail-condition`")
CLASSES = {"floor", "must", "advisory"}


def load_yaml(path: Path, findings: list):
    try:
        with open(path, encoding="utf-8") as f:
            return yaml.safe_load(f)
    except FileNotFoundError:
        findings.append(f"{path}: file not found")
    except yaml.YAMLError as e:
        findings.append(f"{path}: YAML parse break — {str(e).splitlines()[0]}")
    return None


def main() -> int:
    root = Path(__file__).resolve().parent.parent
    p = argparse.ArgumentParser(description="Advisory checker for command content schemas (D13)")
    p.add_argument("--schema", type=Path, default=root / "plugins/mochiko/schemas/implement.yaml")
    p.add_argument("--labels", type=Path, default=root / "plugins/mochiko/schemas/command-labels.yaml")
    p.add_argument("--md", type=Path, default=root / "plugins/mochiko/commands/implement.md")
    p.add_argument("--decisions", type=Path, default=root / "DECISIONS.md")
    a = p.parse_args()

    findings: list = []
    warnings: list = []

    # 1. parse
    schema = load_yaml(a.schema, findings)
    registry = load_yaml(a.labels, findings)
    if schema is None or registry is None:
        return report(findings, warnings, {})

    # 2. kind: discriminators
    if schema.get("kind") != "command":
        findings.append(f"{a.schema.name}: `kind: command` missing (got {schema.get('kind')!r})")
    if not schema.get("command"):
        findings.append(f"{a.schema.name}: `command:` name missing")
    if registry.get("kind") != "command-labels":
        findings.append(f"{a.labels.name}: `kind: command-labels` missing (got {registry.get('kind')!r})")

    reg_labels = registry.get("labels") or {}
    if not isinstance(reg_labels, dict) or not reg_labels:
        findings.append(f"{a.labels.name}: `labels:` mapping missing or empty")
        reg_labels = {}

    # 2b. section grammar (D14) — flat top-level rules: is the superseded shape
    if "rules" in schema:
        findings.append(f"{a.schema.name}: top-level `rules:` — flat grammar superseded by sections (D14)")
    sections = schema.get("sections") or []
    if not isinstance(sections, list) or not sections:
        findings.append(f"{a.schema.name}: `sections:` list missing or empty (D14)")
        sections = []
    vars_block = schema.get("vars") or {}

    # 3. ID uniqueness + slug format · section + rule shape
    seen = {}
    rules = []          # (rule, section_id) pairs flattened for per-rule checks
    sec_stats = []
    for j, s in enumerate(sections):
        if not isinstance(s, dict):
            findings.append(f"sections[{j}]: not a mapping")
            continue
        sid = s.get("id", f"<sections[{j}] missing id>")
        if "id" not in s:
            findings.append(f"sections[{j}]: `id` missing (D14)")
        elif not SEC_ID_RE.match(sid):
            findings.append(f"{sid}: section id fails `<cmd>.sec.<slug>` format (D14)")
        if sid in seen:
            findings.append(f"{sid}: duplicate id (D11 — minted once)")
        seen[sid] = f"sections[{j}]"
        for field in ("title", "intent"):
            if not str(s.get(field) or "").strip():
                findings.append(f"{sid}: `{field}` missing or empty (D14)")
        s_rules = s.get("rules")
        if not isinstance(s_rules, list) or not s_rules:
            findings.append(f"{sid}: `rules` list missing or empty (D14)")
            s_rules = []
        sec_stats.append((sid, len(s_rules)))
        rules.extend((r, sid) for r in s_rules)
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

        # 6. ruling: anchors
        ruling = r.get("ruling")
        if ruling is not None:
            m = RULING_RE.match(str(ruling).strip())
            if not m:
                findings.append(f"{rid}: ruling anchor {ruling!r} malformed — want 'YYYY-MM-DD <session-slug> [D#]'")
            else:
                date, slug = m.group(1), m.group(2)
                if not resolve_anchor(a.decisions, date, slug):
                    findings.append(
                        f"{rid}: ruling anchor '{date} {slug}' resolves to no DECISIONS.md row (D6)"
                    )

    for v in vars_block:
        if v not in used_vars:
            warnings.append(f"vars.{v}: declared but unused by any rule text")
    for lab, n in label_use.items():
        if n == 0:
            warnings.append(f"label {lab!r}: zero members in {a.schema.name} (registry-legal; watch at rollout)")

    # 7. .md Not-done count vs fail-condition set (C2 guard)
    try:
        md_text = re.sub(r"\s+", " ", a.md.read_text(encoding="utf-8"))
        m = NOT_DONE_RE.search(md_text)
        if not m:
            findings.append(f"{a.md.name}: no Not-done line hard-coding the `fail-condition` count (D7 C2 guard)")
        elif int(m.group(1)) != fail_count:
            findings.append(
                f"pair out of sync: {a.md.name} pins {m.group(1)} fail-condition rules, "
                f"schema carries {fail_count} (D7 C2 guard)"
            )
    except FileNotFoundError:
        findings.append(f"{a.md}: file not found — cannot run the C2 count guard")

    # 8. tombstone integrity (D11) — vacuous while no tombstones: key exists
    tombstones = schema.get("tombstones")
    if tombstones is not None:
        if not isinstance(tombstones, list):
            findings.append("tombstones: must be a list (D11)")
        else:
            t_seen = set()
            for j, t in enumerate(tombstones):
                if not isinstance(t, dict) or not t.get("id") or not t.get("disposition"):
                    findings.append(f"tombstones[{j}]: entry needs `id` + `disposition` (D11)")
                    continue
                tid = t["id"]
                if tid in t_seen:
                    findings.append(f"tombstones[{j}]: duplicate tombstone for {tid} (D11)")
                t_seen.add(tid)
                if tid in seen:
                    findings.append(f"{tid}: both live and tombstoned — an ID is minted once (D11)")

    stats = {
        "sections": len(sections),
        "rules": len(rules),
        "floor": class_counts["floor"],
        "must": class_counts["must"],
        "advisory": class_counts["advisory"],
        "fail-condition": fail_count,
        "vars": len(vars_block),
        "labels": len(reg_labels),
    }
    return report(findings, warnings, stats, sec_stats)


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
