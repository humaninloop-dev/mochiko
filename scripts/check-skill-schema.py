#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.9"
# dependencies = ["pyyaml"]
# ///
"""Advisory checker for skill content schemas — skill-content-schema D7, the sibling
road of that ruling's extend-or-sibling latitude. A sibling of check-command-schema.py,
never a fork of it (the D8 idiom): the two grammars diverge on the section set, the pin
target (`class: floor` here, `kind: fail` there), the kind set (eight kinds, `fail`
retired by census §E), in-directory discovery (D2), cross-directory pointer resolution
(census J-7), and the SKILL.md scaffold — and D5 forbids cross-grammar sharing, so the
checkers stay apart the way the data files do. Shared regex idiom is duplicated
knowingly; the command checker is untouched.

Checks, all deterministic:
  1. schema + labels + common files parse as YAML
  2. discriminators — schema `kind: skill` with a `skill:` name matching its directory
     stem (D2 in-dir + the R-b filename-stem ruling); registry `kind: skill-labels`;
     common `kind: skill-common` (generic — every family's file ships under the same
     kind; the `<family>-common.` block prefix carries the family)
  3. section grammar — the skill's FAMILY section set, set-wise, with
     `<stem>.sec.<slug>` IDs. The family derives from the directory-name stem prefix:
     `authoring-*` → the authoring set minted at census-authoring J-1 (independence ·
     scope · inputs · artifact · output · reserved); everything else → the review set
     minted at census §H (independence · scope · inputs · verdict · output ·
     reserved — the small families reuse it by ruling, and further families add a
     prefix branch by their own ruling). `rules: []` with a one-line `note:` is a
     deliberate empty marker and valid; empty with no note is a finding
  4. rule-ID uniqueness + dotted-slug format + stem prefix (every rule and section ID
     leads with the skill's directory name)
  5. tombstone integrity — an ID is never both live and tombstoned
  6. every rule label exists in plugins/mochiko/schemas/skill-labels.yaml; a stub
     inheriting from a block that itself carries no labels resolves label-less by
     design and warns rather than fails (the census assigned some posture blocks no
     label — the block is the single home of that ruling; a LOCAL empty `labels:` is
     still a finding); the zero-member claim is sweep-scoped — labels used by NO
     swept schema are named once, at the end of a sweep (a single-skill run makes no
     zero-member claim: per-family labels are legally absent from any one member)
  7. `kind:` vocabulary — the eight-kind skill set (census §E: constraint · duty ·
     gate · reservation · binding · bound · routing · latitude; `constraint` the
     omitted default). `kind: fail` and `enforces:` are retired from the skill grammar
     and each is an error wherever it appears; a `moments:` block is command grammar
     and an error here (D3/D4 — procedure stays prose, no phase anchors shipped)
  8. `conditions:` / `when:` — same conjunction grammar as the command side, minus
     moments: every term names a declared dimension and value; resolution points are
     entry-derived · surface-presence · user-ruled · standing-trigger
     (`moment-resolved(...)` is command grammar); unused dimensions/values warn.
     The per-dimension coverage report excludes floors (the C4 semantics: a floor is
     always delivered whatever its `when:`)
  9. `extends:` — the named `<family>-common.<slug>` block exists in the schema's OWN
     family library (review → plugins/mochiko/schemas/skill-review-common.yaml ·
     authoring → plugins/mochiko/schemas/skill-authoring-common.yaml); a stub naming
     another family's prefix is a finding (D5 per-family library, cross-family
     sharing forbidden); the stub declares `class:` locally; a common block carrying
     `kind:`/`when:`/`enforces:` is an error; blocks bound by no stub in any swept
     skill are named once, at the end of a sweep, per family and only where at least
     one swept schema belongs to that family (the orphan question is family-wide, so
     a single-skill run — or a sweep with no members of the family — makes no orphan
     claim). Text-side checks run against RESOLVED text — the run reads the
     inherited text
 10. `${var}` closure against `vars:`; unused vars warn; deixis lint (the command D15
     idiom — a reference that dangles when the rule is quoted alone)
 11. `pointer:` path resolution (census J-7) — a path-shaped pointer (carries a `/` or
     ends `.md`) resolves base-directory-relative from the skill's own directory,
     INCLUDING `../<other-skill>/references/...` climbs; a pointer that resolves only
     from the plugin root gets its own finding (the schema ships base-dir-relative
     per D2); `mochiko:<skill>` pointers are names, not paths, and are skipped
 12. in-text ID citations — own-stem tokens resolve against this schema's rules,
     sections, and tombstones (a tombstone hit is a superseded reference, an
     unresolvable token dangles — both findings); foreign-stem tokens warn
 13. the SKILL.md floor-count pin (D6, as amended I1 — a desync guard, not a delivery
     tripwire): the literal line "the N rules of `class: floor`" matches the schema's
     floor count, with the pluralization the count calls for
 14. the SKILL.md "Rules — load the schema first" section is present and enumerates
     exactly the schema's live section IDs, set-wise
 15. all-token resolution — every own-stem `<stem>.sec.*` token anywhere in SKILL.md
     resolves to a live section node; tombstoned tokens are findings
 16. provenance sidecar — the D8/C4 protection transfers land as skill-prefixed
     entries in .mochiko/provenance.yaml, keyed by rule ID: every entry whose key
     carries this skill's stem must name a live rule (a tombstoned or absent target
     is a finding), be well-formed ('YYYY-MM-DD <session-slug> [D#]'), and resolve
     against DECISIONS.md. Foreign-prefix entries are skipped silently — command
     entries are the command checker's, sibling skills' are their own run's. An
     absent sidecar is a warning only (plugin-standalone checkout, the D16 posture).
     The file's `kind:` is accepted as either `command-provenance` or
     `primitive-provenance` — the wave renames it, and the check works on both sides

Exit codes: 0 = clean, 1 = findings. ADVISORY ONLY — never a required CI gate,
never gating pipeline progress, never dispatching agents (GI-019 advisory
carve-out). Its output is cited in the skill-pair audit brief as the deterministic
pre-pass.

Run:  uv run scripts/check-skill-schema.py                        (sweep every
                                            plugins/mochiko/skills/*/schema.yaml)
  or: uv run scripts/check-skill-schema.py --skill review-brainstorm
  or: python3 scripts/check-skill-schema.py   (needs: pip install pyyaml)
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

# Skill stems are directory names and carry hyphens (`review-brainstorm`), unlike the
# command prefixes — every ID grammar below admits them.
STEM = r"[a-z][a-z0-9]*(?:-[a-z0-9]+)*"
ID_RE = re.compile(rf"^{STEM}\.[a-z0-9]+(?:-[a-z0-9]+)*$")
SEC_ID_RE = re.compile(rf"^{STEM}\.sec\.[a-z0-9]+(?:-[a-z0-9]+)*$")
# Any `<stem>.sec.<slug>` token wherever it appears — SKILL.md prose, rule text.
SEC_TOKEN_RE = re.compile(rf"\b({STEM})\.sec\.([a-z0-9]+(?:-[a-z0-9]+)*)\b")
VAR_RE = re.compile(r"\$\{([A-Za-z0-9_]+)\}")
SKELETON_SIGIL_RE = re.compile(r"\{\{[^}]*\}\}")
# D15 curated deixis markers — references that dangle when a rule is quoted alone.
# "this schema" / "the run" are legal self-reference and stay off this list.
DEIXIS_RE = re.compile(
    r"\b(these rules|this section|the section (above|below)|as stated (above|earlier)"
    r"|see (above|below)|aforementioned|there is no \S+ section)\b",
    re.IGNORECASE,
)
# The D6 desync guard, re-keyed to floors: skills have no Not-done set, and floors are
# the content whose silent loss hurts most.
FLOOR_PIN_RE = re.compile(r"the (\d+) (rule|rules) of `class: floor`")
RULING_RE = re.compile(r"^(\d{4}-\d{2}-\d{2})\s+(\S+)(?:\s+D\d+.*)?$")
# The sidecar's discriminator, both sides of the wave's rename (command-provenance →
# primitive-provenance once skill entries join the file).
PROVENANCE_KINDS = ("command-provenance", "primitive-provenance")
CLASSES = {"floor", "must", "advisory"}
# The eight-kind skill set (census §E): `fail` retired by census evidence, `constraint`
# the omitted default — an absent `kind:` reads constraint and is never written.
KINDS = {
    "constraint", "duty", "gate", "reservation", "binding",
    "bound", "routing", "latitude",
}
DEFAULT_KIND = "constraint"
# D3's resolution set minus `moment-resolved(...)` — skills declare no moments.
RESOLUTIONS = {"entry-derived", "surface-presence", "user-ruled", "standing-trigger"}
MOMENT_RESOLVED_RE = re.compile(r"^moment-resolved\([a-z0-9-]+\)$")
PRESENCE_POLES = ("present", "absent")
PRESENCE_ALIASES = {"true": "present", "false": "absent"}
# PyYAML reads YAML 1.1, where `yes`/`no`/`on`/`off` are booleans — so `values: [yes, no]`
# arrives as booleans while a quoted "yes" arrives as a string. Both fold to one token.
BOOL_ALIASES = {"yes": "true", "on": "true", "no": "false", "off": "false"}
# C3 precedence, adopted wholesale by D5: `extends:` inherits these three fields and
# nothing else. `class`, `kind` and `when` are always local, so their absence stays
# meaningful; `enforces` does not exist in this grammar at all.
INHERITED_FIELDS = ("text", "labels", "pointer")
# Per-family section sets, each minted once by its family's census-backed ruling —
# review at census §H (the grader lifecycle), authoring at census-authoring J-1
# (`artifact` replaces `verdict`: producers have no clearing grammar, and the produced
# artifact's binding grammar needs a home). The family derives from the directory-name
# stem prefix; everything without a minted prefix falls through to the review set (the
# small families reuse it by ruling).
FAMILY_SECTION_SETS = {
    "review": ("independence", "scope", "inputs", "verdict", "output", "reserved"),
    "authoring": ("independence", "scope", "inputs", "artifact", "output", "reserved"),
}
COMMON_ID_RES = {
    family: re.compile(rf"^{family}-common\.[a-z0-9]+(?:-[a-z0-9]+)*$")
    for family in FAMILY_SECTION_SETS
}


def family_of(stem: str) -> str:
    """The skill's grammar family, from its directory-name stem prefix."""
    return "authoring" if stem.startswith("authoring-") else "review"


def family_common_path(a, family: str) -> Path:
    """The family's own block library — D5: per-family file, never shared."""
    return a.authoring_common if family == "authoring" else a.common
RULES_HEADING = "## Rules — load the schema first"
CITATION_SUFFIXES = {"md", "yaml", "sh"}


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
    """Every `<stem>.sec.<slug>` token in `text`, in order of appearance."""
    return [f"{p}.sec.{slug}" for p, slug in SEC_TOKEN_RE.findall(str(text))]


def norm_text(text) -> str:
    """Whitespace-collapsed text, for comparing a stub's override against its block."""
    return re.sub(r"\s+", " ", str(text or "")).strip()


def norm_val(v) -> str:
    """One `when:` or `values:` token, canonicalized (YAML 1.1 booleans folded)."""
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
    """The citation scanner over the given stems — longest alternative first, so a stem
    that is a prefix of another still matches whole."""
    alt = "|".join(re.escape(p) for p in sorted(set(prefixes), key=lambda p: (-len(p), p)))
    return re.compile(rf"\b({alt})((?:\.[a-z0-9]+(?:-[a-z0-9]+)*)+)\b")


def cite_tokens(text: str, pattern) -> list:
    """Every rule-ID citation in `text` — bare or parenthetical, file paths excluded.

    Section tokens are left to the section-token lint, which already owns them;
    reporting a bad section reference twice would bury the second copy.
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


def parse_conditions(schema, findings: list) -> dict:
    """The declared `conditions:` block — dimension → legal tokens + resolution point."""
    dims = {}
    block = schema.get("conditions")
    if block is None:
        return dims
    if not isinstance(block, dict) or not block:
        findings.append("conditions: must be a non-empty mapping of dimension → declaration (D4)")
        return dims
    for name, decl in block.items():
        if not isinstance(decl, dict):
            findings.append(
                f"conditions.{name}: declaration must be a mapping carrying `values` "
                f"and `resolution` (D4)"
            )
            continue
        raw = decl.get("values")
        presence = isinstance(raw, str) and raw.strip().lower() == "presence"
        spelling = {}
        if presence:
            tokens = set(PRESENCE_POLES)
        elif isinstance(raw, list) and raw:
            tokens = {norm_val(v) for v in raw}
            spelling = {norm_val(v): str(v) for v in raw}
        else:
            findings.append(
                f"conditions.{name}: `values` must be a non-empty list of closed values, "
                f"or the word `presence` (D4)"
            )
            tokens = set()
        res = str(decl.get("resolution") or "").strip()
        if MOMENT_RESOLVED_RE.match(res):
            findings.append(
                f"conditions.{name}: resolution {res!r} is command grammar — skills declare "
                f"no `moments:`, so no condition resolves at one (D3/D4 skill delta)"
            )
        elif res not in RESOLUTIONS:
            findings.append(
                f"conditions.{name}: resolution {res!r} is not one of "
                f"{' · '.join(sorted(RESOLUTIONS))} (D4)"
            )
        dims[str(name)] = {"tokens": tokens, "presence": presence, "spelling": spelling}
    return dims


def check_when(rid: str, when, dims: dict, findings: list) -> dict:
    """A rule's `when:`, resolved against the declared block — dimension → chosen tokens.

    A conjunction of `dimension: value | [values]` terms in declared vocabulary: no
    boolean algebra, no negation beyond a declared value.
    """
    terms = {}
    if not isinstance(when, dict) or not when:
        findings.append(
            f"{rid}: `when:` must be a non-empty mapping of dimension → value "
            f"— a conjunction, never a list or a string (D4)"
        )
        return terms
    for dim, val in when.items():
        if dim not in dims:
            findings.append(
                f"{rid}: `when:` names dimension {dim!r}, which this schema's "
                f"`conditions:` block does not declare (D4)"
            )
            continue
        raw = val if isinstance(val, list) else [val]
        if not raw:
            findings.append(f"{rid}: `when: {{{dim}: []}}` names no value (D4)")
            continue
        chosen = set()
        for v in raw:
            if isinstance(v, (dict, list)):
                findings.append(
                    f"{rid}: `when.{dim}` carries nested structure — declared values only, "
                    f"no boolean algebra (D4)"
                )
                continue
            tok = dim_token(v, dims[dim]["presence"])
            if tok not in dims[dim]["tokens"]:
                legal = " · ".join(sorted(dims[dim]["tokens"])) or "(none declared)"
                findings.append(
                    f"{rid}: `when: {{{dim}: {tok}}}` — not a declared value of {dim!r} "
                    f"({legal}) (D4)"
                )
                continue
            chosen.add(tok)
        if chosen:
            terms[dim] = chosen
    return terms


def load_common(path: Path, needed: bool, findings: list, warnings: list,
                family: str = "review"):
    """The family block library (D5) — absent is a finding only where a stub binds it."""
    if not path.exists():
        if needed:
            findings.append(
                f"{path}: family block library absent — `extends:` stubs cannot resolve (D5)"
            )
        return None
    doc = load_yaml(path, findings)
    if doc is None:
        return None
    blocks = {}
    if doc.get("kind") != "skill-common":
        findings.append(f"{path.name}: `kind: skill-common` missing (got {doc.get('kind')!r})")
    entries = doc.get("rules")
    if not isinstance(entries, list) or not entries:
        findings.append(f"{path.name}: `rules:` list of common blocks missing or empty (D5)")
        return blocks
    for j, b in enumerate(entries):
        if not isinstance(b, dict) or not b.get("id"):
            findings.append(f"{path.name}: rules[{j}] needs an `id` (D5)")
            continue
        bid = str(b["id"])
        if not COMMON_ID_RES[family].match(bid):
            findings.append(f"{bid}: block id fails `{family}-common.<slug>` format (D5)")
        if bid in blocks:
            findings.append(f"{bid}: duplicate block id (minted once)")
        blocks[bid] = b
        if not str(b.get("text") or "").strip():
            findings.append(f"{bid}: `text` missing or empty (D5)")
        for field in ("kind", "when", "enforces"):
            if field in b:
                findings.append(
                    f"{bid}: carries `{field}:` — an absence-meaningful field is never "
                    f"inherited and is always local to the stub (D5)"
                )
        if "class" in b:
            warnings.append(
                f"{bid}: carries `class:` — every stub declares its own, so this is "
                f"inherited-but-always-overridden dead weight (D5)"
            )
    return blocks


def resolve_extends(rid: str, r: dict, common, common_path: Path, bound: set,
                    findings: list, warnings: list, family: str = "review") -> dict:
    """A rule's effective text/labels/pointer after `extends:` — those three only (D5).

    The run reads resolved text, so every text-side check downstream reads it too.
    A stub binds only its OWN family's library: another family's prefix is the D5
    cross-family sharing the per-family files exist to forbid.
    """
    eff = {f: r.get(f) for f in INHERITED_FIELDS}
    if "extends" not in r:
        return eff
    target = str(r.get("extends"))
    if not COMMON_ID_RES[family].match(target):
        findings.append(
            f"{rid}: `extends: {target}` — want `{family}-common.<slug>` "
            f"(D5 per-family library; cross-family sharing forbidden)"
        )
        return eff
    if common is None:
        findings.append(
            f"{rid}: `extends: {target}` unresolvable — {common_path.name} did not load (D5)")
        return eff
    block = common.get(target)
    if block is None:
        findings.append(f"{rid}: `extends: {target}` names no block in {common_path.name} (D5)")
        return eff
    bound.add(target)
    if "class" not in r:
        findings.append(
            f"{rid}: `extends:` stub declares no local `class:` — class is never inherited, "
            f"so a floor's bindingness must stay readable from its own file (D5)"
        )
    for field in INHERITED_FIELDS:
        if eff[field] is None:
            eff[field] = block.get(field)
        elif field == "text" and norm_text(eff[field]) == norm_text(block.get(field)):
            warnings.append(
                f"{rid}: local `text` is identical to {target}'s — a pointless override (D5)"
            )
    return eff


def check_pointer(rid: str, pointer, skill_dir: Path, findings: list) -> bool:
    """J-7 pointer resolution. Returns True when a path-shaped pointer was checked.

    A pointer is path-shaped when it carries a `/` or ends `.md`; `mochiko:<skill>`
    pointers are names, not paths. Resolution is base-directory-relative from the
    skill's own directory (D2) — cross-directory climbs (`../<other-skill>/...`)
    resolve the same way. A pointer that resolves only from the plugin root is its own
    finding: the installed-cache read is base-dir-relative, so a root-relative path
    would dangle exactly where the first-live-run watch probes (J-7 / M1).
    """
    p = str(pointer or "").strip()
    if not p or ("/" not in p and not p.endswith(".md")):
        return False
    if p.startswith("/"):
        findings.append(f"{rid}: `pointer: {p}` is absolute — paths ship base-dir-relative (J-7)")
        return True
    if (skill_dir / p).exists():
        return True
    plugin_root = skill_dir.parent.parent
    if (plugin_root / p).exists():
        findings.append(
            f"{rid}: `pointer: {p}` resolves only from the plugin root — J-7 wants it "
            f"base-dir-relative to {skill_dir.name}/ (prefix the climb explicitly)"
        )
    else:
        findings.append(
            f"{rid}: `pointer: {p}` resolves to no file base-dir-relative to "
            f"{skill_dir.name}/ (J-7)"
        )
    return True


def coverage_report(dims: dict, activation: dict, floors: set) -> list:
    """The per-dimension coverage report — advisory stdout, never a finding.

    Floors are listed apart and no coverage claim is made over them: a floor is always
    read and always delivered whatever its `when:` (the C4 semantics, adopted by D4).
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


def main() -> int:
    root = Path(__file__).resolve().parent.parent
    p = argparse.ArgumentParser(description="Advisory checker for skill content schemas (D7)")
    p.add_argument("--skill", default=None,
                   help="check one skill by directory name; default sweeps every "
                        "skills/*/schema.yaml")
    p.add_argument("--dir", type=Path, default=None,
                   help="skills root to sweep instead of plugins/mochiko/skills "
                        "(fixture/test use)")
    p.add_argument("--labels", type=Path,
                   default=root / "plugins/mochiko/schemas/skill-labels.yaml")
    p.add_argument("--common", type=Path,
                   default=root / "plugins/mochiko/schemas/skill-review-common.yaml")
    p.add_argument("--authoring-common", type=Path,
                   default=root / "plugins/mochiko/schemas/skill-authoring-common.yaml")
    p.add_argument("--provenance", type=Path, default=root / ".mochiko/provenance.yaml")
    p.add_argument("--decisions", type=Path, default=root / "DECISIONS.md")
    a = p.parse_args()

    skills_root = a.dir if a.dir else root / "plugins/mochiko/skills"
    if a.skill:
        paths = [skills_root / a.skill / "schema.yaml"]
    else:
        paths = sorted(skills_root.glob("*/schema.yaml"))
        if not paths:
            # The sweep is legal on a pre-conversion tree: no schemas means nothing to
            # check, not a defect.
            print(f"no skill schemas under {skills_root} — nothing to check")
            print("check-skill-schema: 0 findings, 0 warnings — PASS (advisory)")
            return 0
    # The citation scanner's stem set covers every sibling skill directory, converted or
    # not — a cross-skill citation is foreign-warned either way, in both modes.
    stems = {sp.parent.name for sp in paths}
    if skills_root.is_dir():
        stems |= {d.name for d in skills_root.iterdir() if d.is_dir()}

    worst = 0
    bound_all = set()
    labels_used = set()
    for sp in paths:
        if not a.skill:
            print(f"=== {sp.parent.name} ===")
        rc = check_pair(sp, a, stems,
                        bound_acc=bound_all if not a.skill else None,
                        label_acc=labels_used if not a.skill else None)
        worst = max(worst, rc)

    # Orphan blocks are a family-wide question: a block bound by review-brainstorm and
    # not by review-feasibility is not orphaned on feasibility's run — so a single-skill
    # run makes no orphan claim, and the sweep answers it once, at the end, per family
    # library and only where the sweep saw at least one member of the family.
    if not a.skill:
        swept_families = {family_of(sp.parent.name) for sp in paths}
        for family in FAMILY_SECTION_SETS:
            path = family_common_path(a, family)
            print(f"=== {path.name} ===")
            if family not in swept_families:
                print(f"no {family}-family schemas swept — no orphan claim")
                continue
            common = load_common(path, False, [], [], family) if path.exists() else None
            if common is None:
                print("family block library absent — no orphan claim")
            else:
                orphans = sorted(set(common) - bound_all)
                if orphans:
                    print("warning: common blocks bound by no `extends:` stub in any swept "
                          "skill: " + ", ".join(orphans))
                else:
                    print(f"stats: common blocks {len(common)} · all bound by at least one stub")

        # The zero-member label claim is sweep-scoped for the same reason: per-family
        # labels are legally absent from any one member, so only a label no swept
        # schema carries is worth naming — once, here.
        registry = load_yaml(a.labels, [])
        reg_labels = (registry or {}).get("labels") or {}
        unused = sorted(set(reg_labels) - labels_used)
        if unused:
            print("warning: labels with zero members across the swept schemas "
                  "(registry-legal; watch at rollout): " + ", ".join(unused))
    return worst


def check_pair(schema_path: Path, a, stems: set, bound_acc: set = None,
               label_acc: set = None) -> int:
    findings: list = []
    warnings: list = []
    skill_dir = schema_path.parent
    stem = skill_dir.name
    family = family_of(stem)
    md_path = skill_dir / "SKILL.md"

    # 1. parse
    schema = load_yaml(schema_path, findings)
    registry = load_yaml(a.labels, findings)
    if schema is None or registry is None:
        return report(findings, warnings, {})

    # 2. discriminators — the schema's name must be its directory's (D2 + R-b: the stem
    #    is the ID prefix AND the shipping directory, one name).
    if schema.get("kind") != "skill":
        findings.append(f"{schema_path.name}: `kind: skill` missing (got {schema.get('kind')!r})")
    declared = schema.get("skill")
    if not declared:
        findings.append(f"{schema_path.name}: `skill:` name missing")
    elif str(declared) != stem:
        findings.append(
            f"{schema_path.name}: `skill: {declared}` does not match its directory "
            f"{stem!r} — the stem is minted from the directory name (D2, R-b)"
        )
    if registry.get("kind") != "skill-labels":
        findings.append(f"{a.labels.name}: `kind: skill-labels` missing (got {registry.get('kind')!r})")

    reg_labels = registry.get("labels") or {}
    if not isinstance(reg_labels, dict) or not reg_labels:
        findings.append(f"{a.labels.name}: `labels:` mapping missing or empty")
        reg_labels = {}

    # 7. retired/foreign grammar at the document level — `moments:` is command grammar.
    if "moments" in schema:
        findings.append(
            f"{schema_path.name}: carries a `moments:` block — procedure stays prose and "
            f"no phase anchors ship; moments are command grammar (D3/D4 skill delta)"
        )

    # 8. the declared condition vocabulary, parsed ahead of the rule walk because every
    #    `when:` resolves against it.
    dims = parse_conditions(schema, findings)

    # 3. section grammar — flat top-level rules: is command-side superseded shape too
    if "rules" in schema:
        findings.append(f"{schema_path.name}: top-level `rules:` — rules nest in sections (D4)")
    sections = schema.get("sections") or []
    if not isinstance(sections, list) or not sections:
        findings.append(f"{schema_path.name}: `sections:` list missing or empty (D4)")
        sections = []
    vars_block = schema.get("vars") or {}

    # 5. tombstone integrity — hoisted above the rule loop so the token lints can tell
    #    a tombstoned node from a node that never existed.
    tombstoned = set()
    tombstones = schema.get("tombstones")
    if tombstones is not None:
        if not isinstance(tombstones, list):
            findings.append("tombstones: must be a list")
        else:
            for j, t in enumerate(tombstones):
                if not isinstance(t, dict) or not t.get("id") or not t.get("disposition"):
                    findings.append(f"tombstones[{j}]: entry needs `id` + `disposition`")
                    continue
                tid = t["id"]
                if tid in tombstoned:
                    findings.append(f"tombstones[{j}]: duplicate tombstone for {tid}")
                tombstoned.add(tid)

    # 3/4. section shape · ID uniqueness + stem prefix · empty markers
    seen = {}
    rules = []          # (rule, section_id, index-within-section)
    sec_stats = []
    section_ids = []
    empty_sections = 0
    for j, s in enumerate(sections):
        if not isinstance(s, dict):
            findings.append(f"sections[{j}]: not a mapping")
            continue
        sid = s.get("id", f"<sections[{j}] missing id>")
        if "id" not in s:
            findings.append(f"sections[{j}]: `id` missing")
        elif not SEC_ID_RE.match(sid):
            findings.append(f"{sid}: section id fails `<stem>.sec.<slug>` format (D4)")
        elif not sid.startswith(f"{stem}.sec."):
            findings.append(
                f"{sid}: section id does not lead with this skill's stem {stem!r} (R-b)")
        else:
            section_ids.append(sid)
        if sid in seen:
            findings.append(f"{sid}: duplicate id (minted once)")
        seen[sid] = f"sections[{j}]"
        for field in ("title", "intent"):
            if not str(s.get(field) or "").strip():
                findings.append(f"{sid}: `{field}` missing or empty (D4)")

        note = str(s.get("note") or "").strip()
        s_rules = s.get("rules")
        if "rules" not in s:
            findings.append(f"{sid}: `rules` key missing (D4)")
            s_rules = []
        elif s_rules is None or (isinstance(s_rules, list) and not s_rules):
            if note:
                empty_sections += 1
                if s_rules is None:
                    warnings.append(f"{sid}: empty section written as `rules:` — prefer explicit `rules: []`")
            else:
                findings.append(
                    f"{sid}: empty with no `note:` — a deliberately empty section carries a "
                    f"one-line note naming the emptiness deliberate (D4 explicit empty marker)"
                )
            s_rules = []
        elif not isinstance(s_rules, list):
            findings.append(f"{sid}: `rules` must be a list (D4)")
            s_rules = []
        sec_stats.append((sid, len(s_rules)))
        rules.extend((r, sid, k) for k, r in enumerate(s_rules))

    # 3. the family section set, set-wise — minted once by the family's census ruling
    #    (review: census §H · authoring: census-authoring J-1).
    slugs = FAMILY_SECTION_SETS[family]
    live_sections = set(section_ids)
    expected = {f"{stem}.sec.{slug}" for slug in slugs}
    for missing in sorted(expected - live_sections):
        findings.append(
            f"{schema_path.name}: canonical section {missing} absent — every "
            f"{family}-family schema carries all six, empty ones explicitly (the family "
            f"set, minted once by its census ruling)"
        )
    for extra in sorted(live_sections - expected):
        findings.append(
            f"{extra}: not one of the six canonical {family}-family sections "
            f"({' · '.join(slugs)}) — the family set is minted once by its census ruling"
        )

    # 9. `extends:` — the schema's OWN family library loads only where a stub binds it,
    #    and an absent file is a finding only in that case (a plugin-standalone checkout
    #    has none).
    binds_common = any(isinstance(r, dict) and "extends" in r for r, _, _ in rules)
    common_path = family_common_path(a, family)
    common = (load_common(common_path, binds_common, findings, warnings, family)
              if binds_common else None)
    bound_blocks = set()

    label_use = {name: 0 for name in reg_labels}
    used_vars = set()
    class_counts = {c: 0 for c in CLASSES}
    kind_counts = {}
    texts = []              # (rule id, resolved text) for the post-loop citation pass
    activation = {}         # (dimension, value) → the rule IDs that activate on it
    floors, when_rules = set(), set()
    pointers_checked = 0
    for r, sid, i in rules:
        if not isinstance(r, dict):
            findings.append(f"{sid}: rules[{i}] not a mapping")
            continue
        rid = r.get("id", f"<{sid} rules[{i}] missing id>")
        if "id" not in r:
            findings.append(f"{sid}: rules[{i}] `id` missing")
        elif not ID_RE.match(rid):
            findings.append(f"{rid}: id fails dotted-slug format (D4)")
        elif not rid.startswith(f"{stem}."):
            findings.append(
                f"{rid}: rule id does not lead with this skill's stem {stem!r} (R-b)")
        if rid in seen:
            findings.append(f"{rid}: duplicate id (first at {seen[rid]}, again in {sid})")
        seen[rid] = sid

        # 9. `extends:` resolution first — every text-side and label-side check below
        #    reads the RESOLVED value, because that is what the run reads (D5).
        eff = resolve_extends(rid, r, common, common_path, bound_blocks, findings,
                              warnings, family)

        cls = r.get("class")
        if cls not in CLASSES:
            findings.append(f"{rid}: `class` must be floor|must|advisory (got {cls!r})")
        else:
            class_counts[cls] += 1
            if cls == "floor":
                floors.add(rid)

        # 7. kind vocabulary — eight kinds; `fail` and `enforces:` are retired grammar
        #    here and get their own messages so the census §E ruling is citable from the
        #    finding itself.
        kind = r.get("kind", DEFAULT_KIND)
        if kind == "fail":
            findings.append(
                f"{rid}: `kind: fail` — retired from the skill-side kind set by census "
                f"evidence (§E: no member asserts a run-fail predicate); verdict-earning "
                f"content is `constraint` or `gate`"
            )
        elif kind not in KINDS:
            findings.append(
                f"{rid}: `kind: {kind}` is not one of {' · '.join(sorted(KINDS))} (census §E)"
            )
        kind_counts[kind] = kind_counts.get(kind, 0) + 1
        if "enforces" in r:
            findings.append(
                f"{rid}: carries `enforces:` — the field left the skill grammar with "
                f"`kind: fail`, its only carrier (census §E)"
            )

        # 6. labels ⊆ registry, against the resolved set. One carve: a stub inheriting
        #    from a block that itself carries no labels resolves label-less by design
        #    (the census assigned some posture blocks no label; the block is the single
        #    home of that ruling) — a warning keeps it visible without failing the pair.
        #    A LOCAL empty `labels:` is still a finding, block labels or not.
        rl = eff["labels"]
        if not isinstance(rl, list) or not rl:
            target = str(r.get("extends", ""))
            block = (common or {}).get(target)
            if rl is None and block is not None and not block.get("labels"):
                warnings.append(
                    f"{rid}: resolves with no labels — its block {target} carries none "
                    f"(inherited absence)")
            else:
                findings.append(f"{rid}: `labels` missing or empty")
            rl = []
        for lab in rl:
            if lab not in reg_labels:
                findings.append(f"{rid}: label {lab!r} not in {a.labels.name} (D4)")
            else:
                label_use[lab] += 1

        # 8. `when:` — resolved against this schema's own declared dimensions
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

        # 10. ${var} closure + stray skeleton sigils + deixis
        for m in VAR_RE.finditer(str(text)):
            used_vars.add(m.group(1))
            if m.group(1) not in vars_block:
                findings.append(f"{rid}: orphan placeholder ${{{m.group(1)}}} — unbound in vars:")
        if SKELETON_SIGIL_RE.search(str(text)):
            warnings.append(f"{rid}: contains a {{{{...}}}} sigil — skeleton convention, not var substitution")
        dm = DEIXIS_RE.search(str(text))
        if dm:
            warnings.append(
                f"{rid}: deictic reference {dm.group(0)!r} — referent lives outside the block; "
                f"name it via the addressable namespace (D15 idiom)"
            )

        # 15-adjacent. rule text must name live own-stem section nodes only
        for tok in sec_tokens(text):
            if not tok.startswith(f"{stem}."):
                continue  # foreign-stem tokens are warned once, in the citation pass
            if tok in live_sections:
                continue
            if tok in tombstoned:
                findings.append(f"{rid}: text names tombstoned section {tok} (relocate the reference)")
            else:
                findings.append(f"{rid}: text names section {tok}, which is not a node in {schema_path.name}")

        # 11. pointer resolution (J-7), against the resolved pointer
        if check_pointer(rid, eff["pointer"], skill_dir, findings):
            pointers_checked += 1

    # 12. in-text ID citations — own-stem tokens resolve here; foreign stems (other
    #     swept skills included — they resolve on their own run) are named in a warning.
    cite_pat = cite_re(set(stems) | {stem})
    foreign_cites = set()
    for rid, text in texts:
        for tok in cite_tokens(text, cite_pat):
            if tok.split(".")[0] != stem:
                foreign_cites.add(tok)
            elif tok in tombstoned:
                findings.append(
                    f"{rid}: text cites {tok}, which is tombstoned — a superseded reference "
                    f"(re-key it or drop it)"
                )
            elif tok not in seen:
                findings.append(f"{rid}: text cites {tok}, which resolves to no node in "
                                f"{schema_path.name} (a dangling reference)")
    if foreign_cites:
        warnings.append(
            f"{schema_path.name}: citations with foreign stems, unresolvable against this "
            f"pair: " + ", ".join(sorted(foreign_cites))
        )

    # 8. declared-but-unused dimensions and values — warning-class, same split as the
    #    coverage report: the warning asks whether ANY rule names a value, floors
    #    included; the report asks which non-floor rules activate.
    for dim, decl in dims.items():
        used = {tok for (d, tok) in activation if d == dim}
        if not used:
            warnings.append(f"conditions.{dim}: declared but no rule's `when:` names it (D4)")
            continue
        for tok in sorted(decl["tokens"] - used):
            shown = (decl.get("spelling") or {}).get(tok, tok)
            warnings.append(
                f"conditions.{dim}: value {shown!r} declared but named by no rule's `when:` "
                f"— a coverage hole or a deliberate absence (D4)"
            )

    if bound_acc is not None:
        bound_acc.update(bound_blocks)

    for v in vars_block:
        if v not in used_vars:
            warnings.append(f"vars.{v}: declared but unused by any rule text")
    # The zero-member label claim is sweep-scoped (a per-family label is legally absent
    # from any one member) — this run only reports what it used, into the accumulator.
    if label_acc is not None:
        label_acc.update(lab for lab, n in label_use.items() if n)

    # 16. provenance sidecar — the D8/C4 protection transfers, keyed by rule ID. Only
    #     this skill's stem is validated here: command entries belong to the command
    #     checker, sibling skills' entries to their own pair run.
    anchor_count = 0
    if not a.provenance.exists():
        warnings.append(
            f"{a.provenance}: provenance sidecar absent — anchor checks skipped "
            f"(plugin-standalone?)")
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
            for key, anchor in p_anchors.items():
                if str(key).split(".")[0] != stem:
                    continue  # another primitive's entry — validated on its own run
                anchor_count += 1
                if key in tombstoned:
                    findings.append(
                        f"{a.provenance.name}: entry {key!r} names a tombstoned rule — "
                        f"the protection transfer re-homes with the ID, never onto a "
                        f"tombstone (D8/C4)")
                elif key not in seen:
                    findings.append(
                        f"{a.provenance.name}: dangling entry {key!r} — no such rule in "
                        f"{schema_path.name} (D8/C4)")
                m = RULING_RE.match(str(anchor).strip())
                if not m:
                    findings.append(
                        f"{key}: anchor {anchor!r} malformed — want "
                        f"'YYYY-MM-DD <session-slug> [D#]'")
                elif not resolve_anchor(a.decisions, m.group(1), m.group(2)):
                    findings.append(
                        f"{key}: anchor '{m.group(1)} {m.group(2)}' resolves to no "
                        f"DECISIONS.md row")

    # 13/14/15. the SKILL.md side
    check_md(md_path, stem, live_sections, tombstoned, len(floors), findings, warnings)

    # 5. tombstone integrity — an ID is never both live and tombstoned
    for tid in sorted(tombstoned):
        if tid in seen:
            findings.append(f"{tid}: both live and tombstoned — an ID is minted once")

    stats = {
        "sections": len(sections),
        "empty-sections": empty_sections,
        "rules": len(rules),
        "floor": class_counts["floor"],
        "must": class_counts["must"],
        "advisory": class_counts["advisory"],
        "when": len(when_rules),
        "conditions": len(dims),
        "extends": len(bound_blocks),
        "pointers": pointers_checked,
        "vars": len(vars_block),
        "labels": len(reg_labels),
        "anchors": anchor_count,
    }
    advisory = coverage_report(dims, activation, floors) if dims else []
    return report(findings, warnings, stats, sec_stats, advisory)


def check_md(md_path: Path, stem: str, live_sections: set, tombstoned: set,
             floor_count: int, findings: list, warnings: list) -> None:
    try:
        raw = md_path.read_text(encoding="utf-8")
    except FileNotFoundError:
        findings.append(f"{md_path}: file not found — cannot run the D6 floor-count pin")
        return
    # The pin phrase is matched against a whitespace-collapsed copy so it survives a
    # line wrap; the heading checks read the raw text, which keeps line anchors intact.
    flat = re.sub(r"\s+", " ", raw)

    # 13. the floor-count pin + the pluralization the count calls for (D6, I1 desync
    #     guard). Every occurrence is validated, not just the first — a second,
    #     disagreeing pin elsewhere in the body is exactly the desync the guard exists
    #     to catch.
    matches = list(FLOOR_PIN_RE.finditer(flat))
    if not matches:
        findings.append(
            f"{md_path.name}: no line pinning the `class: floor` count — want "
            f"'the {floor_count} {'rule' if floor_count == 1 else 'rules'} of `class: floor`' "
            f"(D6 desync guard)"
        )
    for m in matches:
        pinned, word = int(m.group(1)), m.group(2)
        if pinned != floor_count:
            findings.append(
                f"pair out of sync: {md_path.name} pins {pinned} rules of `class: floor`, "
                f"schema carries {floor_count} (D6 desync guard)"
            )
        want = "rule" if pinned == 1 else "rules"
        if word != want:
            findings.append(
                f"{md_path.name}: floor pin reads 'the {pinned} {word} of' — "
                f"want 'the {pinned} {want} of'"
            )

    # 14. the load-first section is present and enumerates the schema's live sections
    lines = raw.splitlines()
    heading_at = next((i for i, ln in enumerate(lines) if ln.rstrip() == RULES_HEADING), None)
    enumerated = set()
    if heading_at is None:
        findings.append(
            f"{md_path.name}: canonical heading `{RULES_HEADING}` absent — the load-first "
            f"block is the delivery guard (D6)"
        )
    else:
        end = next((i for i in range(heading_at + 1, len(lines))
                    if lines[i].startswith("## ")), len(lines))
        block = "\n".join(lines[heading_at:end])
        enumerated = {t for t in sec_tokens(block) if t.startswith(f"{stem}.")}
        for missing in sorted(live_sections - enumerated):
            findings.append(
                f"{md_path.name}: Rules block does not enumerate {missing} — the block "
                f"names every section the schema carries (D6)"
            )
        for extra in sorted(enumerated - live_sections):
            if extra in tombstoned:
                findings.append(
                    f"{md_path.name}: Rules block enumerates {extra}, which is "
                    f"tombstoned — re-key the reference (D6)"
                )
            else:
                findings.append(
                    f"{md_path.name}: Rules block enumerates {extra}, which is not a "
                    f"section in the paired schema (D6)"
                )

    # 15. all-token resolution — anywhere in the SKILL.md, not only the Rules block.
    # Tokens the enumeration check already named are skipped so one dangle is not
    # reported twice.
    already_named = enumerated - live_sections
    foreign = set()
    for tok in dict.fromkeys(sec_tokens(raw)):
        if tok in already_named:
            continue
        if not tok.startswith(f"{stem}."):
            foreign.add(tok.split(".")[0])
        elif tok not in live_sections:
            if tok in tombstoned:
                findings.append(f"{md_path.name}: names tombstoned section {tok} — re-key the reference")
            else:
                findings.append(f"{md_path.name}: names section {tok}, which is not a node in the paired schema")
    if foreign:
        warnings.append(
            f"{md_path.name}: section tokens with foreign stems, unresolvable against this pair: "
            + ", ".join(sorted(foreign))
        )


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
    print(f"check-skill-schema: {len(findings)} findings, {len(warnings)} warnings — {verdict} (advisory)")
    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
