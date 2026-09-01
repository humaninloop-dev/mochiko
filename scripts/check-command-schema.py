#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.9"
# dependencies = ["pyyaml"]
# ///
"""Advisory checker for command content schemas — command-content-schema D13,
extended by command-md-scaffold-standardization D6-R1 (as review-widened) and by
command-schema-ontology D1-D11 (the kind/conditions/moments/citation/enforces/extends
grammar, build item 6).

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
  4. every rule label exists in the label registry (D8). The `fail-condition` label is
     retired by the ontology wave (build item 4) — naming it anywhere, in the registry,
     on a rule, or in the .md, is a superseded-grammar finding
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
     rules carrying `kind: fail` (D7, C2 guard, re-keyed off the retired label by
     ontology build item 4), with the pluralization the count calls for — "the 1
     rule of `kind: fail`" / "the N rules of `kind: fail`" (D6-R4)
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
  9. `kind:` vocabulary (ontology D1) — one of constraint · duty · gate · reservation ·
     binding · bound · routing · fail · latitude; an absent kind reads `constraint`.
     Fail keying is bidirectional and replaces the two retired label checks like for
     like: a `<prefix>.fail.*` ID carries an explicit `kind: fail`, never defaulted,
     and `kind: fail` never appears outside that segment (I4, re-keyed)
 10. `conditions:` / `when:` (ontology D3) — a `when:` is a mapping of dimension →
     value or list of values, a conjunction with no boolean algebra; every term names
     a declared dimension and a declared value of it; a `moment-resolved(<moment>)`
     resolution point names a moment the `moments:` block declares; a declared
     dimension or value no `when:` uses is a warning. MOVE and DECLARE are
     indistinguishable here by design — both are `when:`-bearing rules (D3, J-1).
     The unused-value warning and the coverage report ask different questions and are
     worded apart: the warning fires when NO rule names a value, floors included; the
     report answers C4's question — which non-floor rules activate — so a value carried
     only by a floor reads "(no rule activates)" there and raises no warning here
 11. `moments:` (ontology D4) — a declared moment named by no moment-resolved condition
     and mentioned in no rule text is a warning. `at:` does not ship (D4 as amended,
     I5), so a prose mention is the only other way a moment is used
 12. in-text ID citations (ontology D5, scan surface pinned at J-11) — every
     `<prefix>.<slug>` token in a rule's resolved text, bare or parenthetical, over the
     six command prefixes plus this schema's own; file-suffix tokens (`.md` / `.yaml`)
     are paths, not citations (M3). The resolution set is the token prefix's rule IDs,
     its section IDs, and its tombstones — a tombstone hit is a superseded reference and
     an unresolvable token dangles, both error-class. A foreign-prefix citation cannot
     be resolved against one pair and is named in a warning
 13. `enforces:` (ontology D6) — every `kind: fail` node carries the field, and nothing
     else does; each listed ID resolves to a live rule in the same schema (a tombstoned
     target is an error); an empty list carries its reason the way the converted schemas
     carry it — a `# D6 empty-with-reason: …` comment directly above the `enforces: []`
     line, read off the raw file because PyYAML discards comments — so an empty mirror is
     always a statement, never an omission
 14. `extends:` (ontology D8 as amended by C3) — the named `common.<slug>` block exists;
     the stub declares `class:` locally, since class is never inherited; a local `text`
     identical to the block's is a pointless override; a common block carrying
     `when:`/`kind:`/`enforces:` is an error, those fields being always local; blocks
     bound by no stub are named. Every text-side check runs against RESOLVED text — the
     run reads the inherited text, so its placeholders, deixis and citations are the
     stub's own

Two advisory stdout sections, warning-class and never findings: the per-dimension
coverage report (D3), which makes no coverage claim over `class: floor` rules because a
floor is always delivered whatever its `when:` (C4); and the `enforces:` reverse-coverage
report, which is input to the deferred Desk FAIL-set widening pass (D6, I7).

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
NOT_DONE_RE = re.compile(r"the (\d+) (rule|rules) of `kind: fail`")
# The count-pinned line's own opener, line-anchored so its placement can be checked. The
# numbered-list prefix is optional: the line is protocol step 3, but the step numbering is
# not what D2 pins — the position is.
NOT_DONE_LINE_RE = re.compile(r"^\s*(?:\d+\.\s+)?\*\*Not done — default FAIL:\*\*")
# .md "nested in N sections" phrase — optional, but when present must match len(sections)
SECTION_COUNT_RE = re.compile(r"nested in ([a-z]+|\d+) sections")
WORD_NUMS = {w: i for i, w in enumerate(
    "zero one two three four five six seven eight nine ten eleven twelve".split())}
CLASSES = {"floor", "must", "advisory"}
# The closed kind set (ontology D1). `constraint` is the omitted default: an absent
# `kind:` reads constraint and is never written.
KINDS = {
    "constraint", "duty", "gate", "reservation", "binding",
    "bound", "routing", "fail", "latitude",
}
DEFAULT_KIND = "constraint"
# Retired by the ontology wave's fail re-key — `kind: fail` is the operative selector.
RETIRED_LABEL = "fail-condition"
# The retired label cited in prose. The `fail-conditions` section slug is live vocabulary,
# so the plural is excluded rather than matched and filtered.
RETIRED_PROSE_RE = re.compile(r"fail-condition(?!s)")
# An empty `enforces:`, written either way. Its reason lives in a YAML comment directly
# above it — PyYAML discards comments, so this one check reads the raw file.
EMPTY_ENFORCES_RE = re.compile(r"^\s*enforces:\s*(?:\[\s*\])?\s*(?:#.*)?$")
# The reason marker the converted schemas carry (setup.yaml's two empty mirrors).
EMPTY_REASON_MARKER = "D6 empty-with-reason:"
# D3's closed resolution-point set. `moment-resolved(<moment>)` carries its moment name.
RESOLUTIONS = {"entry-derived", "surface-presence", "user-ruled", "standing-trigger"}
MOMENT_RESOLVED_RE = re.compile(r"^moment-resolved\(([a-z0-9]+(?:-[a-z0-9]+)*)\)$")
# A presence dimension declares `values: presence` and its rules name one of two poles.
PRESENCE_POLES = ("present", "absent")
PRESENCE_ALIASES = {"true": "present", "false": "absent"}
# PyYAML reads YAML 1.1, where `yes`/`no`/`on`/`off` are booleans — so `values: [yes, no]`
# arrives as booleans while a quoted "yes" arrives as a string. Both fold to one token.
BOOL_ALIASES = {"yes": "true", "on": "true", "no": "false", "off": "false"}
# The six frozen command prefixes (D4). A pair's own prefix is added at check time, so a
# schema outside the shipped six still has its citations resolved (J-11: the scan is
# all-prefix, and a token whose prefix is not this pair's cannot be resolved here).
COMMAND_PREFIXES = ("impl", "feat", "spec", "arch", "setup", "brainstorm")
# The sidecar's discriminator, both sides of the skill-content-schema wave's rename
# (command-provenance → primitive-provenance once skill entries join the file) —
# mirrored in check-skill-schema.py.
PROVENANCE_KINDS = ("command-provenance", "primitive-provenance")
# `spec.md` is a path, not a citation (M3) — a token ending in a file suffix is excluded.
CITATION_SUFFIXES = {"md", "yaml"}
# C3 precedence: `extends:` inherits these three fields and nothing else. `class`, `kind`,
# `when` and `enforces` are always local, so their absence stays meaningful.
INHERITED_FIELDS = ("text", "labels", "pointer")
COMMON_ID_RE = re.compile(r"^common\.[a-z0-9]+(-[a-z0-9]+)*$")
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


def norm_text(text) -> str:
    """Whitespace-collapsed text, for comparing a stub's override against its block."""
    return re.sub(r"\s+", " ", str(text or "")).strip()


def norm_val(v) -> str:
    """One `when:` or `values:` token, canonicalized.

    A YAML 1.1 boolean and its quoted spelling must land on the same token, or specify's
    `values: [yes, no]` would never match a rule's `when: {ux_bearing: yes}`.
    """
    if v is True:
        s = "true"
    elif v is False:
        s = "false"
    else:
        s = str(v).strip().lower()
    return BOOL_ALIASES.get(s, s)


def dim_token(v, presence: bool) -> str:
    """A dimension value, canonicalized — presence dimensions fold true/false to their poles."""
    s = norm_val(v)
    return PRESENCE_ALIASES.get(s, s) if presence else s


def cite_re(prefixes) -> "re.Pattern":
    """The all-prefix citation scanner (D5, scan surface pinned at J-11).

    Longest alternative first so a prefix that is a prefix of another still matches whole.
    """
    alt = "|".join(sorted(set(prefixes), key=lambda p: (-len(p), p)))
    return re.compile(rf"\b({alt})((?:\.[a-z0-9]+(?:-[a-z0-9]+)*)+)\b")


def cite_tokens(text: str, pattern) -> list:
    """Every rule-ID citation in `text` — bare or parenthetical, file paths excluded.

    Section tokens are left to the 5c lint, which already owns them; reporting a bad
    section reference twice would bury the second copy.
    """
    out = []
    for prefix, rest in pattern.findall(str(text)):
        tok = prefix + rest
        if rest.rsplit(".", 1)[-1] in CITATION_SUFFIXES:
            continue
        if SEC_ID_RE.match(tok):
            continue
        if tok not in out:
            out.append(tok)
    return out


def parse_moments(schema, findings: list) -> dict:
    """The declared `moments:` block (D4) — name → one navigation line, unordered."""
    block = schema.get("moments")
    if block is None:
        return {}
    if not isinstance(block, dict) or not block:
        findings.append("moments: must be a non-empty mapping of name → one navigation line (D4)")
        return {}
    out = {}
    for name, line in block.items():
        if not str(line or "").strip():
            findings.append(f"moments.{name}: navigation line missing or empty (D4)")
        out[str(name)] = str(line or "")
    return out


def parse_conditions(schema, moments: dict, findings: list) -> dict:
    """The declared `conditions:` block (D3) — dimension → legal tokens + resolution point."""
    dims = {}
    block = schema.get("conditions")
    if block is None:
        return dims
    if not isinstance(block, dict) or not block:
        findings.append("conditions: must be a non-empty mapping of dimension → declaration (D3)")
        return dims
    for name, decl in block.items():
        if not isinstance(decl, dict):
            findings.append(
                f"conditions.{name}: declaration must be a mapping carrying `values` "
                f"and `resolution` (D3)"
            )
            continue
        raw = decl.get("values")
        presence = isinstance(raw, str) and raw.strip().lower() == "presence"
        # `values: [yes, no]` canonicalizes to true/false so a rule's `when:` can match it
        # whichever way YAML 1.1 read it — but the report shows the schema's own spelling.
        spelling = {}
        if presence:
            tokens = set(PRESENCE_POLES)
        elif isinstance(raw, list) and raw:
            tokens = {norm_val(v) for v in raw}
            spelling = {norm_val(v): str(v) for v in raw}
        else:
            findings.append(
                f"conditions.{name}: `values` must be a non-empty list of closed values, "
                f"or the word `presence` (D3)"
            )
            tokens = set()
        res = str(decl.get("resolution") or "").strip()
        m = MOMENT_RESOLVED_RE.match(res)
        if m and m.group(1) not in moments:
            findings.append(
                f"conditions.{name}: resolution names moment {m.group(1)!r}, which the "
                f"`moments:` block does not declare (D3/D4)"
            )
        elif not m and res not in RESOLUTIONS:
            findings.append(
                f"conditions.{name}: resolution {res!r} is not one of "
                f"{' · '.join(sorted(RESOLUTIONS))} · moment-resolved(<moment>) (D3)"
            )
        dims[str(name)] = {
            "tokens": tokens,
            "presence": presence,
            "spelling": spelling,
            "moment": m.group(1) if m else None,
        }
    return dims


def check_when(rid: str, when, dims: dict, findings: list) -> dict:
    """A rule's `when:`, resolved against the declared block — dimension → chosen tokens.

    The grammar is a conjunction of `dimension: value | [values]` terms in declared
    vocabulary: no boolean algebra, no negation beyond a declared value (D3).
    """
    terms = {}
    if not isinstance(when, dict) or not when:
        findings.append(
            f"{rid}: `when:` must be a non-empty mapping of dimension → value "
            f"— a conjunction, never a list or a string (D3)"
        )
        return terms
    for dim, val in when.items():
        if dim not in dims:
            findings.append(
                f"{rid}: `when:` names dimension {dim!r}, which this schema's "
                f"`conditions:` block does not declare (D3)"
            )
            continue
        raw = val if isinstance(val, list) else [val]
        if not raw:
            findings.append(f"{rid}: `when: {{{dim}: []}}` names no value (D3)")
            continue
        chosen = set()
        for v in raw:
            if isinstance(v, (dict, list)):
                findings.append(
                    f"{rid}: `when.{dim}` carries nested structure — declared values only, "
                    f"no boolean algebra (D3)"
                )
                continue
            tok = dim_token(v, dims[dim]["presence"])
            if tok not in dims[dim]["tokens"]:
                legal = " · ".join(sorted(dims[dim]["tokens"])) or "(none declared)"
                findings.append(
                    f"{rid}: `when: {{{dim}: {tok}}}` — not a declared value of {dim!r} "
                    f"({legal}) (D3)"
                )
                continue
            chosen.add(tok)
        if chosen:
            terms[dim] = chosen
    return terms


def load_common(path: Path, needed: bool, findings: list, warnings: list):
    """The shared block library (D8) — absent is a finding only where a stub binds it."""
    if not path.exists():
        if needed:
            findings.append(
                f"{path}: shared block library absent — `extends:` stubs cannot resolve (D8)"
            )
        return None
    doc = load_yaml(path, findings)
    if doc is None:
        return None
    blocks = {}
    if doc.get("kind") != "command-common":
        findings.append(f"{path.name}: `kind: command-common` missing (got {doc.get('kind')!r})")
    entries = doc.get("rules")
    if not isinstance(entries, list) or not entries:
        findings.append(f"{path.name}: `rules:` list of common blocks missing or empty (D8)")
        return blocks
    for j, b in enumerate(entries):
        if not isinstance(b, dict) or not b.get("id"):
            findings.append(f"{path.name}: rules[{j}] needs an `id` (D8)")
            continue
        bid = str(b["id"])
        if not COMMON_ID_RE.match(bid):
            findings.append(f"{bid}: block id fails `common.<slug>` format (D8)")
        if bid in blocks:
            findings.append(f"{bid}: duplicate block id (D11 — minted once)")
        blocks[bid] = b
        if not str(b.get("text") or "").strip():
            findings.append(f"{bid}: `text` missing or empty (D8)")
        for field in ("kind", "when", "enforces"):
            if field in b:
                findings.append(
                    f"{bid}: carries `{field}:` — an absence-meaningful field is never "
                    f"inherited and is always local to the stub (D8/C3)"
                )
        if "class" in b:
            warnings.append(
                f"{bid}: carries `class:` — every stub declares its own, so this is "
                f"inherited-but-always-overridden dead weight (C3)"
            )
    return blocks


def resolve_extends(rid: str, r: dict, common, common_path: Path, bound: set,
                    findings: list, warnings: list) -> dict:
    """A rule's effective text/labels/pointer after `extends:` — those three only (C3).

    The run reads resolved text, so every text-side check downstream reads it too.
    """
    eff = {f: r.get(f) for f in INHERITED_FIELDS}
    if "extends" not in r:
        return eff
    target = str(r.get("extends"))
    if not COMMON_ID_RE.match(target):
        findings.append(f"{rid}: `extends: {target}` — want `common.<slug>` (D8)")
        return eff
    if common is None:
        findings.append(f"{rid}: `extends: {target}` unresolvable — {common_path.name} did not load (D8)")
        return eff
    block = common.get(target)
    if block is None:
        findings.append(f"{rid}: `extends: {target}` names no block in {common_path.name} (D8)")
        return eff
    bound.add(target)
    if "class" not in r:
        findings.append(
            f"{rid}: `extends:` stub declares no local `class:` — class is never inherited, "
            f"so a floor's bindingness must stay readable from its own file (D8/C3)"
        )
    for field in INHERITED_FIELDS:
        if eff[field] is None:
            eff[field] = block.get(field)
        elif field == "text" and norm_text(eff[field]) == norm_text(block.get(field)):
            warnings.append(
                f"{rid}: local `text` is identical to {target}'s — a pointless override (D8)"
            )
    return eff


def empty_enforces_reasons(path: Path) -> dict:
    """Rule ID → the comment block sitting directly above its empty `enforces:` line.

    D6 makes an empty mirror legal only with a stated reason, and the converted schemas
    state it in a YAML comment — which PyYAML discards, so this one check reads the raw
    file. The scan is consulted only for rules PyYAML parsed as empty, so a block-list
    `enforces:` picked up here is harmless.
    """
    out = {}
    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except OSError:
        return out
    rid = None
    for i, line in enumerate(lines):
        m = re.match(r"^\s*-\s+id:\s*(\S+)\s*$", line)
        if m:
            rid = m.group(1)
        elif rid and EMPTY_ENFORCES_RE.match(line):
            comment = []
            for j in range(i - 1, -1, -1):
                stripped = lines[j].strip()
                if not stripped.startswith("#"):
                    break
                comment.insert(0, stripped.lstrip("#").strip())
            out[rid] = " ".join(comment).strip()
    return out


def coverage_report(dims: dict, activation: dict, floors: set) -> list:
    """The per-dimension coverage report (D3) — advisory stdout, never a finding.

    Floors are listed apart and no coverage claim is made over them: under C4 a floor is
    always read and always delivered whatever its `when:`, so a value covered only by a
    floor is not covered in the sense this report means.
    """
    out = []
    for dim in sorted(dims):
        spelling = dims[dim].get("spelling") or {}
        out.append(f"coverage — {dim}:")
        for tok in sorted(dims[dim]["tokens"]):
            hits = sorted(activation.get((dim, tok), set()) - floors)
            out.append(f"    {spelling.get(tok, tok)}: "
                       + (" · ".join(hits) if hits else "(no rule activates)"))
        floor_hits = sorted(
            {rid for (d, _), rids in activation.items() if d == dim for rid in rids} & floors)
        if floor_hits:
            out.append("    floor — always delivered, no coverage claim: " + " · ".join(floor_hits))
    return out


def enforces_report(mirrored: set, floors: set, gates: set, fails: set) -> list:
    """Reverse coverage (D6) — which floors and gates no `kind: fail` node mirrors.

    Per I7 this is input to the deferred Desk FAIL-set widening pass and is never audit
    pressure: the pair audit's FAIL-survival handle covers the EXISTING fail sets only.
    """
    uncovered = sorted((floors | gates) - mirrored - fails)
    out = ["enforces reverse coverage — input to the deferred Desk FAIL-set widening pass, "
           "not a finding:"]
    if uncovered:
        out.append(f"    {len(uncovered)} floor/gate rules no fail node enforces: "
                   + " · ".join(uncovered))
    else:
        out.append("    every floor and gate is mirrored by a fail node")
    return out


def main() -> int:
    root = Path(__file__).resolve().parent.parent
    p = argparse.ArgumentParser(description="Advisory checker for command content schemas (D13)")
    p.add_argument("--schema", type=Path, default=root / "plugins/mochiko/schemas/implement.yaml")
    p.add_argument("--labels", type=Path, default=root / "plugins/mochiko/schemas/command-labels.yaml")
    p.add_argument("--provenance", type=Path, default=root / ".mochiko/provenance.yaml")
    p.add_argument("--md", type=Path, default=root / "plugins/mochiko/commands/implement.md")
    p.add_argument("--decisions", type=Path, default=root / "DECISIONS.md")
    p.add_argument("--common", type=Path, default=root / "plugins/mochiko/schemas/common.yaml")
    p.add_argument("--all", action="store_true",
                   help="check all six shipped pairs; --schema/--md are ignored")
    a = p.parse_args()

    if not a.all:
        return check_pair(a.schema, a.md, a)

    # Orphan common blocks are a corpus-wide question, not a per-pair one: a block bound by
    # brainstorm and not by feature is not orphaned on feature's run. Under --all the bound
    # set accumulates across the six pairs and the orphan report is emitted once, at the end.
    worst = 0
    bound_all = set()
    for name in PAIRS:
        print(f"=== {name} ===")
        rc = check_pair(
            root / f"plugins/mochiko/schemas/{name}.yaml",
            root / f"plugins/mochiko/commands/{name}.md",
            a,
            bound_acc=bound_all,
        )
        worst = max(worst, rc)

    print("=== common.yaml ===")
    common = load_common(a.common, False, [], [])
    orphans = sorted(set(common or {}) - bound_all)
    if orphans:
        print("warning: common blocks bound by no `extends:` stub in any of the six pairs: "
              + ", ".join(orphans))
    else:
        print(f"stats: common blocks {len(common or {})} · all bound by at least one stub")
    return worst


def check_pair(schema_path: Path, md_path: Path, a, bound_acc: set = None) -> int:
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
    if RETIRED_LABEL in reg_labels:
        findings.append(
            f"{a.labels.name}: label {RETIRED_LABEL!r} still registered — retired by the "
            f"ontology wave's fail re-key; `kind: fail` is the selector (D1, build item 4)"
        )

    # 10/11. the declared condition and moment vocabularies (D3/D4). Parsed ahead of the
    # rule walk because every `when:` resolves against them.
    moments = parse_moments(schema, findings)
    dims = parse_conditions(schema, moments, findings)

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
            elif RETIRED_PROSE_RE.search(str(s[field])):
                warnings.append(
                    f"{sid}: `{field}` names the retired `fail-condition` selector — the "
                    f"Not-done set is keyed on `kind: fail` (D1, build item 4)"
                )

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
        # The rule's index within its OWN section travels with it: the findings below name
        # the section, so a flattened index would point at a position that section has not got.
        rules.extend((r, sid, k) for k, r in enumerate(s_rules))

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

    # 14. `extends:` (D8/C3) — the shared library loads only where a stub binds it, and an
    #     absent file is a finding only in that case (a plugin-standalone checkout has none).
    binds_common = any(isinstance(r, dict) and "extends" in r for r, _, _ in rules)
    common = load_common(a.common, binds_common, findings, warnings) if binds_common else None
    bound_blocks = set()

    label_use = {name: 0 for name in reg_labels}
    used_vars = set()
    segment_count = 0
    class_counts = {c: 0 for c in CLASSES}
    kind_counts = {}
    texts = []              # (rule id, resolved text) for the post-loop citation pass
    enforces_by_rule = {}   # fail node → the IDs it claims to mirror
    activation = {}         # (dimension, value) → the rule IDs that activate on it
    floors, gates, when_rules, fail_nodes = set(), set(), set(), set()
    reasons = empty_enforces_reasons(schema_path)
    for r, sid, i in rules:
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

        # 14. `extends:` resolution first — every text-side and label-side check below
        #     reads the RESOLVED value, because that is what the run reads (C3).
        eff = resolve_extends(rid, r, common, a.common, bound_blocks, findings, warnings)

        cls = r.get("class")
        if cls not in CLASSES:
            findings.append(f"{rid}: `class` must be floor|must|advisory (got {cls!r})")
        else:
            class_counts[cls] += 1
            if cls == "floor":
                floors.add(rid)

        # 9. kind vocabulary (D1) — an absent `kind:` reads `constraint` and is never written
        kind = r.get("kind", DEFAULT_KIND)
        if kind not in KINDS:
            findings.append(
                f"{rid}: `kind: {kind}` is not one of {' · '.join(sorted(KINDS))} (D1)"
            )
        kind_counts[kind] = kind_counts.get(kind, 0) + 1
        if kind == "gate":
            gates.add(rid)

        # 9. the fail keying, bidirectional and explicit — this replaces the two retired
        #    label checks like for like (I4). `constraint` is the only defaulted kind, so a
        #    `.fail.` rule that omits `kind:` would silently read constraint.
        if ".fail." in rid:
            segment_count += 1
            if "kind" not in r:
                findings.append(
                    f"{rid}: under the .fail. segment with no explicit `kind: fail` — the "
                    f"fail kind is never defaulted (D1, build item 4)"
                )
            elif kind != "fail":
                findings.append(
                    f"{rid}: under the .fail. segment but carries `kind: {kind}` (D1)"
                )
        elif kind == "fail":
            findings.append(f"{rid}: `kind: fail` outside the .fail. segment (D1)")

        # 4. labels ⊆ registry, against the resolved set
        rl = eff["labels"]
        if not isinstance(rl, list) or not rl:
            findings.append(f"{rid}: `labels` missing or empty")
            rl = []
        for lab in rl:
            if lab == RETIRED_LABEL:
                findings.append(
                    f"{rid}: carries the retired {RETIRED_LABEL!r} label — `kind: fail` is "
                    f"the selector for the Not-done set (D1, build item 4)"
                )
            elif lab not in reg_labels:
                findings.append(f"{rid}: label {lab!r} not in {a.labels.name} (D8)")
            else:
                label_use[lab] += 1

        # 13. `enforces:` is a fail-node field and only a fail node's (D6)
        if kind == "fail":
            fail_nodes.add(rid)
            if "enforces" not in r:
                findings.append(f"{rid}: `kind: fail` node carries no `enforces:` (D6)")
            elif not isinstance(r["enforces"], list):
                findings.append(f"{rid}: `enforces:` must be a list of local rule IDs (D6)")
            elif not r["enforces"]:
                reason = reasons.get(rid, "")
                if EMPTY_REASON_MARKER not in reason or not reason.split(
                        EMPTY_REASON_MARKER, 1)[1].strip():
                    findings.append(
                        f"{rid}: `enforces: []` with no stated reason — an empty mirror is "
                        f"legal only with a `# {EMPTY_REASON_MARKER} …` comment directly "
                        f"above it, so absence is a statement, never an omission (D6)"
                    )
            else:
                enforces_by_rule[rid] = [str(t) for t in r["enforces"]]
        elif "enforces" in r:
            findings.append(
                f"{rid}: carries `enforces:` on a `kind: {kind}` node — the field is a fail "
                f"node's mirror link and nothing else's (D6)"
            )

        # 10. `when:` (D3) — resolved against this schema's own declared dimensions
        if "when" in r:
            when_rules.add(rid)
            for dim, chosen in check_when(rid, r["when"], dims, findings).items():
                for tok in chosen:
                    activation.setdefault((dim, tok), set()).add(rid)

        text = eff["text"]
        if not text or not str(text).strip():
            findings.append(f"{rid}: `text` missing or empty")
            text = ""
        texts.append((rid, str(text)))
        if RETIRED_PROSE_RE.search(str(text)):
            warnings.append(
                f"{rid}: text names the retired `fail-condition` selector — the Not-done set "
                f"is keyed on `kind: fail` (D1, build item 4)"
            )

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

    # 12. in-text ID citations (D5, scan surface pinned all-prefix at J-11). The resolution
    #     set is the token prefix's own rule IDs and section IDs — so a foreign-prefix token
    #     cannot be resolved against this pair either way and is named in a warning, never a
    #     dangle. Section tokens are handled by the 5c lint and skipped by cite_tokens.
    cite_pat = cite_re(list(COMMAND_PREFIXES) + ([prefix] if prefix else []))
    foreign_cites = set()
    for rid, text in texts:
        for tok in cite_tokens(text, cite_pat):
            if prefix and tok.split(".")[0] != prefix:
                foreign_cites.add(tok)
            elif tok in tombstoned:
                findings.append(
                    f"{rid}: text cites {tok}, which is tombstoned — a superseded reference "
                    f"(D5; re-key it or drop it)"
                )
            elif tok not in seen:
                findings.append(f"{rid}: text cites {tok}, which resolves to no node in "
                                f"{schema_path.name} (D5 — a dangling reference)")
    if foreign_cites:
        warnings.append(
            f"{schema_path.name}: citations with foreign prefixes, unresolvable against this "
            f"pair: " + ", ".join(sorted(foreign_cites))
        )

    # 13. `enforces:` targets resolve locally; a tombstoned target is an error (D5 semantics)
    mirrored = set()
    for rid, targets in enforces_by_rule.items():
        for t in targets:
            if t in tombstoned:
                findings.append(f"{rid}: `enforces: {t}` names a tombstoned rule (D6/D5)")
            elif t not in seen:
                findings.append(
                    f"{rid}: `enforces: {t}` resolves to no rule in {schema_path.name} (D6)")
            elif SEC_ID_RE.match(t):
                findings.append(
                    f"{rid}: `enforces: {t}` names a section — the mirror link points at the "
                    f"rule the fail node is the contrapositive of (D6)")
            else:
                mirrored.add(t)

    # 11. declared moments (D4) — `at:` does not ship (I5), so a moment is used by a
    #     moment-resolved condition or by a prose mention, and by nothing else.
    moment_resolved = {d["moment"] for d in dims.values() if d["moment"]}
    for name in moments:
        if name in moment_resolved:
            continue
        # A bare substring match, so a moment whose name is a common word ("close",
        # "acceptance", "entry") reads as used on any incidental mention. The check is
        # therefore weak in one direction only: it under-reports unused moments, never
        # invents one.
        if any(name in text for _, text in texts):
            continue
        warnings.append(
            f"moments.{name}: declared but named by no moment-resolved condition and "
            f"mentioned in no rule text (D4)"
        )

    # 10. declared-but-unused dimensions and values (D3). Both are warning-class: C.7 records
    #     values that are correctly empty (implement `depth: low`, specify `ux_bearing: no`,
    #     `seats: single` everywhere) beside the two real holes the wave takes to the user gate.
    #
    #     These warnings ask a STRONGER question than the coverage report above them: here a
    #     value is unused when NO rule's `when:` names it at all, floors included. The report
    #     asks the C4 question — which non-floor rules activate — and prints "(no rule
    #     activates)" for a value carried only by floors, which is not the same thing and is
    #     worded differently on purpose. implement's `scope: lane` is the live case: the F6-1
    #     landing hole shows in the report, while `impl.lane-never-widens` keeps it out of
    #     this list.
    for dim, decl in dims.items():
        used = {tok for (d, tok) in activation if d == dim}
        if not used:
            warnings.append(f"conditions.{dim}: declared but no rule's `when:` names it (D3)")
            continue
        for tok in sorted(decl["tokens"] - used):
            shown = (decl.get("spelling") or {}).get(tok, tok)
            warnings.append(
                f"conditions.{dim}: value {shown!r} declared but named by no rule's `when:` "
                f"— a coverage hole or a deliberate absence (D3)"
            )

    # 14. orphan common blocks. Per-pair here; under --all the question is corpus-wide and
    #     main() answers it once, from the accumulated bound set.
    if bound_acc is not None:
        bound_acc.update(bound_blocks)
    elif common:
        for orphan in sorted(set(common) - bound_blocks):
            warnings.append(
                f"{orphan}: bound by no `extends:` stub in {schema_path.name} (D8)")

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
            if prov.get("kind") not in PROVENANCE_KINDS:
                findings.append(
                    f"{a.provenance.name}: `kind:` must be one of "
                    f"{' · '.join(PROVENANCE_KINDS)} (got {prov.get('kind')!r})")
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

    # 7. the .md side — count-pin, scaffold, token resolution. The pin is keyed on the
    #    `kind: fail` count, which is what the re-keyed line claims (build item 4); the
    #    `.fail.` segment count is reported beside it and the two agree once the schema
    #    passes the bidirectional keying check above.
    fail_count = kind_counts.get("fail", 0)
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
        "kind: fail": fail_count,
        ".fail. segment": segment_count,
        "when": len(when_rules),
        "conditions": len(dims),
        "moments": len(moments),
        "extends": len(bound_blocks),
        "vars": len(vars_block),
        "labels": len(reg_labels),
        "anchors": anchor_count,
    }
    advisory = []
    if dims:
        advisory += coverage_report(dims, activation, floors)
    advisory += enforces_report(mirrored, floors, gates, fail_nodes)
    return report(findings, warnings, stats, sec_stats, advisory)


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
        findings.append(
            f"{md_path.name}: no Not-done line hard-coding the `kind: fail` count — want "
            f"'the {fail_count} {'rule' if fail_count == 1 else 'rules'} of `kind: fail`' "
            f"(D7 C2 guard, re-keyed at build item 4)"
        )
    else:
        pinned, word = int(m.group(1)), m.group(2)
        if pinned != fail_count:
            findings.append(
                f"pair out of sync: {md_path.name} pins {pinned} rules of `kind: fail`, "
                f"schema carries {fail_count} (D7 C2 guard)"
            )
        want = "rule" if pinned == 1 else "rules"
        if word != want:
            findings.append(
                f"{md_path.name}: Not-done line reads 'the {pinned} {word} of' — "
                f"want 'the {pinned} {want} of' (D6-R4)"
            )

    # The retired selector cited anywhere in the .md — the pre-re-key wording is exactly
    # "the N rules labeled `fail-condition`", which the pin regex above no longer matches,
    # so this names the cause instead of leaving only a missing-line finding.
    if RETIRED_PROSE_RE.search(raw):
        findings.append(
            f"{md_path.name}: names the retired `fail-condition` label — the Not-done set is "
            f"keyed on `kind: fail` (D1, build item 4)"
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


def report(findings: list, warnings: list, stats: dict, sec_stats: list = None,
           advisory: list = None) -> int:
    for f in findings:
        print(f"FINDING: {f}")
    for w in warnings:
        print(f"warning: {w}")
    for line in advisory or []:
        print(line)
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
