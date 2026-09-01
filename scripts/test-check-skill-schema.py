#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.9"
# dependencies = ["pyyaml"]
# ///
"""Negative-test matrix for scripts/check-skill-schema.py.

Every assertion the checker carries is exercised twice: once against a synthetic pair that
satisfies it (the positive control, which must come back clean) and once against a mutation
that breaks exactly that assertion (which must produce the named finding). A check that
cannot be made to fail is not a check.

The fixtures are synthetic — a `demo-grader` skill that exists only in a temp directory,
with its own label registry, its own family block library, and a sibling `other-skill`
directory whose reference file exists solely so the J-7 cross-directory pointer climb has a
real target. The matrix never depends on the shipped skills and stays green while the real
library is mid-wave.

Mutation slots mirror the surfaces the checker reads: `schema` and `md` mutate the parsed
fixtures, `common` mutates the family block library, `labels` the registry, and `yamltext`
the dumped YAML string (parse-break probes). `omit` withholds a file entirely, and `sweep`
runs the checker in sweep mode (no --skill) — the only mode that makes the orphan-block
claim.

Run:  uv run scripts/test-check-skill-schema.py
"""

import subprocess
import sys
import tempfile
from pathlib import Path

import yaml

CHECKER = Path(__file__).resolve().parent / "check-skill-schema.py"

CANONICAL_SLUGS = ("independence", "scope", "inputs", "verdict", "output", "reserved")


def rule(rid, labels, text, cls="must", **fields):
    """A rule block. `id` leads so the dumped YAML keeps the `- id:` opener readable."""
    return {"id": rid, "labels": list(labels), "class": cls, **fields, "text": text}


def baseline_schema():
    """A synthetic pair that satisfies every assertion — the positive control."""
    populated = {
        "independence": [
            rule("demo-grader.never-author", ["independence"],
                 "Never author or fix what you grade.", "floor"),
        ],
        "scope": [
            rule("demo-grader.carve-out", ["boundary"],
                 "General code review is out of scope; route it to the sibling seat.",
                 kind="routing"),
            rule("demo-grader.ladder-home", ["floor-pointer"],
                 "The grading ladder lives at its own skill, referenced never restated.",
                 pointer="mochiko:patterns-code-minimalism"),
        ],
        "inputs": [
            rule("demo-grader.read-report", ["fence"],
                 "Read the report file itself, never a relay of it.", kind="duty"),
            rule("demo-grader.checks-file", ["binding"],
                 "Run the check set per its reference file.", kind="binding",
                 pointer="references/CHECKS.md"),
            # The J-7 shape: a stub pointer climbing out of the skill directory.
            rule("demo-grader.shared-claims", ["binding"],
                 "External claims verify per the shared reference, never argued.",
                 kind="binding", pointer="../other-skill/references/SHARED.md"),
            rule("demo-grader.manifest-read", ["fence"],
                 "The manifest is an obligated read.", kind="duty",
                 when={"demo_manifest": "present"}),
        ],
        "verdict": [
            # An `extends:` stub: no local text, no local labels — both inherited. If
            # resolution did not run, this rule would report both as missing.
            {"id": "demo-grader.default-fail", "extends": "review-common.default-fail",
             "class": "floor"},
            rule("demo-grader.verdict-vocab", ["verdict"],
                 "The verdict vocabulary is pass or fail, nothing softer."),
            rule("demo-grader.deep-hunt", ["verdict"],
                 "Hunt every class of ${target} exhaustively.", when={"depth": "deep"}),
            rule("demo-grader.shallow-hunt", ["verdict"],
                 "One pass over ${target} is enough.", when={"depth": "shallow"}),
            # The dependency idiom: a parenthetical rule-ID citation.
            rule("demo-grader.dispute-route", ["verdict"],
                 "A disputed grade escalates, never self-clears (demo-grader.never-author)."),
        ],
        "output": [
            rule("demo-grader.report-home", ["reporting", "evidence"],
                 "Verdict and dispositions land in the reviewed artifacts themselves.",
                 "floor", kind="binding"),
        ],
        # `reserved` is the deliberate empty marker (the census §H set holds it open).
    }
    sections = []
    for slug in CANONICAL_SLUGS:
        node = {
            "id": f"demo-grader.sec.{slug}",
            "title": slug.title(),
            "intent": f"the {slug} obligations the grade is bound by",
        }
        if slug in populated:
            node["rules"] = populated[slug]
        else:
            node["rules"] = []
            node["note"] = "no reserved rulings yet — deliberately empty, not an omission"
        sections.append(node)
    return {
        "kind": "skill",
        "skill": "demo-grader",
        "vars": {"target": "the graded artifact set"},
        "conditions": {
            "depth": {
                "values": ["deep", "shallow"],
                "resolution": "entry-derived",
                "note": "named by the dispatching brief.",
            },
            "demo_manifest": {
                "values": "presence",
                "resolution": "surface-presence",
                "note": "the manifest file beside the graded artifact.",
            },
        },
        "sections": sections,
    }


BASELINE_COMMON = {
    "kind": "skill-common",
    "rules": [
        {"id": "review-common.default-fail", "labels": ["verdict"],
         "text": "Never default to a clearing verdict — earned only by a completed hunt; "
                 "absence of looking is never evidence."},
        # Bound by no stub in the baseline — exists so the sweep-mode orphan probe has a
        # real orphan, and single-skill mode can prove it makes no claim about it.
        {"id": "review-common.author-grader", "labels": ["independence"],
         "text": "Never author, fix, or revise what you grade."},
    ],
}

BASELINE_LABELS = {
    "kind": "skill-labels",
    "labels": {
        "independence": "who is never whom",
        "boundary": "a sibling/jurisdiction line",
        "fence": "a read-boundary",
        "binding": "an obligation on the run",
        "verdict": "the clearing grammar and its posture",
        "reporting": "where reports land",
        "evidence": "where review evidence must live",
        "floor-pointer": "a binding that points at a skill-owned floor",
        "user-gate": "a decision reserved to the user",
    },
}

# Three floors in the baseline: never-author · the default-fail stub · report-home.
BASELINE_MD = """---
description: A synthetic grader used only by the checker's negative-test matrix.
---

# Demo Grader

## Rules — load the schema first

Read `schema.yaml` (this skill's own directory) in full before your first action, and
`plugins/mochiko/schemas/skill-review-common.yaml` in the same first action. State the
floor count back before the first procedural step: the schema carries the 3 rules of
`class: floor`. A rule carrying `when:` binds only when the run's declared shape matches
every term; floors are always delivered. Sections, each addressable by its ID:
`demo-grader.sec.independence` · `demo-grader.sec.scope` · `demo-grader.sec.inputs` ·
`demo-grader.sec.verdict` · `demo-grader.sec.output` · `demo-grader.sec.reserved`.

## Procedure

1. Read the graded artifacts whole.
2. Grade, hunt, and report per the schema's obligations.
"""

CHECKS_MD = "# Checks\n\nA reference file the in-directory pointer resolves to.\n"
SHARED_MD = "# Shared claims\n\nSingle source. Consumed by: demo-grader.\n"

# The baseline ships the post-rename discriminator; a probe proves the pre-rename one
# is accepted too (the check works on both sides of P5's rename).
BASELINE_PROVENANCE = {
    "kind": "primitive-provenance",
    "anchors": {"demo-grader.never-author": "2026-09-01 skill-content-schema D8"},
}

BASELINE_DECISIONS = "| 2026-09-01 | A synthetic row for skill-content-schema | ruled | [x](y) |\n"


# --- probe helpers -------------------------------------------------------------------------

def find_rule(schema, rid):
    for sec in schema["sections"]:
        for r in sec.get("rules") or []:
            if r["id"] == rid:
                return r
    raise AssertionError(f"fixture has no rule {rid}")


def drop_section(slug):
    def mutate(s):
        s["sections"] = [n for n in s["sections"] if n["id"] != f"demo-grader.sec.{slug}"]
    return mutate


def add_tombstone(sid, s):
    s.setdefault("tombstones", []).append({"id": sid, "disposition": "retired at the wave"})


def set_text(rid, text):
    def mutate(s):
        find_rule(s, rid)["text"] = text
    return mutate


def set_field(rid, field, value):
    def mutate(s):
        find_rule(s, rid)[field] = value
    return mutate


def drop_field(rid, field):
    def mutate(s):
        find_rule(s, rid).pop(field, None)
    return mutate


def probes():
    """Each probe breaks exactly one assertion and names the finding it must produce."""
    p = []

    def add(name, expected, schema=None, md=None, common=None, labels=None,
            provenance=None, yamltext=None, clean=False, absent=None, omit=(),
            sweep=False):
        # `absent` is the other half of a "stays clean" probe: asserting only that no
        # finding appeared would also pass if the check never ran at all, so the probes
        # that exist to prove a NON-finding name the message that must not be there.
        # `omit` withholds a fixture file entirely; `sweep` drops --skill.
        p.append({"name": name, "schema": schema, "md": md, "common": common,
                  "labels": labels, "provenance": provenance, "yamltext": yamltext,
                  "expected": expected, "clean": clean, "absent": absent, "omit": omit,
                  "sweep": sweep})

    # --- the positive control ---
    add("baseline pair is clean", "0 findings", clean=True)

    # --- 1. discriminators + file-level guards ---
    add("the schema missing its kind: discriminator",
        "`kind: skill` missing",
        schema=lambda s: s.__setitem__("kind", "skill-schema"))
    add("the schema missing its skill: name",
        "`skill:` name missing", schema=lambda s: s.pop("skill"))
    add("the skill: name disagreeing with its directory",
        "does not match its directory 'demo-grader'",
        schema=lambda s: s.__setitem__("skill", "demo-validator"))
    add("the registry missing its kind: discriminator",
        "`kind: skill-labels` missing",
        labels=lambda lb: lb.__setitem__("kind", "labels"))
    add("the registry carrying no labels mapping",
        "`labels:` mapping missing or empty",
        labels=lambda lb: lb.__setitem__("labels", {}))
    add("the library missing its kind: discriminator",
        "`kind: skill-common` missing",
        common=lambda c: c.__setitem__("kind", "review-common"))
    add("the schema does not parse as YAML",
        "YAML parse break",
        yamltext=lambda y: y + "\n  this: [is not\n   valid: yaml\n")
    add("the SKILL.md file is missing entirely",
        "SKILL.md: file not found", omit=("md",))
    add("the schema carrying no sections",
        "`sections:` list missing or empty",
        schema=lambda s: s.__setitem__("sections", []))
    add("a flat top-level rules: key",
        "top-level `rules:` — rules nest in sections",
        schema=lambda s: s.__setitem__(
            "rules", [rule("demo-grader.flat", ["binding"], "A rule at the top level.")]))

    # --- 2. the six-section skill set, set-wise (census §H) ---
    add("canonical section absent",
        "canonical section demo-grader.sec.verdict absent",
        schema=drop_section("verdict"))

    def extra_section(s):
        s["sections"].append({"id": "demo-grader.sec.tools", "title": "Tools", "intent": "x",
                              "rules": [rule("demo-grader.stray", ["binding"], "A stray rule.")]})
    add("a section outside the canonical six (the command six-set is not this grammar)",
        "demo-grader.sec.tools: not one of the six canonical skill sections",
        schema=extra_section)

    def foreign_stem_section(s):
        s["sections"][2]["id"] = "other-skill.sec.inputs"
    add("a section ID leading with a foreign stem",
        "does not lead with this skill's stem 'demo-grader'",
        schema=foreign_stem_section)

    # --- 3. empty-marker recognition ---
    def empty_without_note(s):
        for n in s["sections"]:
            if n["id"] == "demo-grader.sec.reserved":
                n.pop("note")
    add("an empty section carrying no note",
        "demo-grader.sec.reserved: empty with no `note:`",
        schema=empty_without_note)

    def empty_rules_written_as_none(s):
        for n in s["sections"]:
            if n["id"] == "demo-grader.sec.reserved":
                n["rules"] = None
    add("an empty section written as `rules:` rather than `rules: []`",
        "prefer explicit `rules: []`", schema=empty_rules_written_as_none, clean=True)

    # --- 4. rule IDs — uniqueness, format, stem prefix ---
    add("a duplicate rule ID",
        "duplicate id", schema=set_field("demo-grader.read-report", "id", "demo-grader.never-author"))
    add("a rule id outside the dotted-slug format",
        "id fails dotted-slug format",
        schema=set_field("demo-grader.read-report", "id", "Demo_Read_Report"))
    add("a rule id leading with a foreign stem",
        "demo-validator.read-report: rule id does not lead with this skill's stem",
        schema=set_field("demo-grader.read-report", "id", "demo-validator.read-report"))
    add("a rule missing its id",
        "`id` missing", schema=lambda s: find_rule(s, "demo-grader.read-report").pop("id"))
    add("a class outside floor|must|advisory",
        "`class` must be floor|must|advisory",
        schema=set_field("demo-grader.read-report", "class", "mandatory"))
    add("a rule carrying no labels",
        "`labels` missing or empty",
        schema=set_field("demo-grader.read-report", "labels", []))
    add("a rule carrying no text",
        "demo-grader.read-report: `text` missing or empty",
        schema=set_text("demo-grader.read-report", "   "))
    add("a label outside the registry",
        "label 'not-a-real-label' not in skill-labels.yaml",
        schema=set_field("demo-grader.read-report", "labels", ["not-a-real-label"]))

    # --- 5. the eight-kind vocabulary; fail/enforces/moments are retired-or-foreign ---
    add("a kind outside the eight-kind set",
        "`kind: sproing` is not one of",
        schema=set_field("demo-grader.read-report", "kind", "sproing"))
    add("kind: fail names its census retirement, not a generic vocabulary miss",
        "retired from the skill-side kind set by census evidence",
        schema=set_field("demo-grader.read-report", "kind", "fail"))
    add("enforces: anywhere in a skill schema",
        "carries `enforces:` — the field left the skill grammar",
        schema=set_field("demo-grader.read-report", "enforces", ["demo-grader.never-author"]))
    add("a moments: block in a skill schema",
        "carries a `moments:` block",
        schema=lambda s: s.__setitem__("moments", {"open": "Where grading opens."}))

    def every_legal_kind(s):
        # One rule per kind not already exercised, to prove the vocabulary admits the
        # whole eight-kind set and not just the members the fixture happens to use.
        sec = s["sections"][3]["rules"]
        for k in ("gate", "reservation", "bound", "latitude", "constraint"):
            sec.append(rule(f"demo-grader.kind-{k}", ["binding"], f"A rule of kind {k}.", kind=k))
    add("every legal kind is admitted",
        "0 findings", schema=every_legal_kind, clean=True)
    add("an absent kind reads constraint, never a finding",
        "0 findings", schema=drop_field("demo-grader.carve-out", "kind"), clean=True,
        absent="is not one of")

    # --- 6. conditions / when ---
    add("when: names an undeclared dimension",
        "`when:` names dimension 'ghost'",
        schema=set_field("demo-grader.deep-hunt", "when", {"ghost": "deep"}))
    add("when: names an undeclared value",
        "not a declared value of 'depth'",
        schema=set_field("demo-grader.deep-hunt", "when", {"depth": "sideways"}))
    add("when: written as a list, not a conjunction mapping",
        "`when:` must be a non-empty mapping",
        schema=set_field("demo-grader.deep-hunt", "when", ["depth", "deep"]))
    add("a moment-resolved resolution point is command grammar",
        "is command grammar — skills declare no `moments:`",
        schema=lambda s: s["conditions"]["depth"].__setitem__(
            "resolution", "moment-resolved(open)"))
    add("a resolution point outside the skill set",
        "resolution 'whenever' is not one of",
        schema=lambda s: s["conditions"]["depth"].__setitem__("resolution", "whenever"))

    def unused_dimension(s):
        s["conditions"]["unused_dim"] = {"values": ["a", "b"], "resolution": "entry-derived"}
    add("a declared dimension no rule uses is a warning",
        "conditions.unused_dim: declared but no rule's `when:` names it",
        schema=unused_dimension, clean=True)

    def unused_value(s):
        find_rule(s, "demo-grader.shallow-hunt").pop("when")
    add("a declared value named by no rule's when: is a warning",
        "conditions.depth: value 'shallow' declared but named by no rule's `when:`",
        schema=unused_value, clean=True)
    add("the coverage report makes no claim over floors",
        "floor — always delivered, no coverage claim",
        schema=set_field("demo-grader.never-author", "when", {"depth": "deep"}), clean=True)

    # --- 7. extends: against the family library ---
    add("extends: names no block in the library",
        "`extends: review-common.ghost` names no block",
        schema=set_field("demo-grader.default-fail", "extends", "review-common.ghost"))
    add("extends: target outside the review-common.<slug> format",
        "want `review-common.<slug>`",
        schema=set_field("demo-grader.default-fail", "extends", "common.default-fail"))
    add("an extends: stub declaring no local class",
        "declares no local `class:`",
        schema=drop_field("demo-grader.default-fail", "class"))
    add("the family library absent while a stub binds it",
        "family block library absent", omit=("common",))

    for field, value in (("kind", "constraint"), ("when", {"depth": "deep"}),
                         ("enforces", ["x"])):
        add(f"a common block carrying `{field}:`",
            "an absence-meaningful field is never inherited",
            common=(lambda f, v: (lambda c: c["rules"][0].__setitem__(f, v)))(field, value))

    add("a common block id outside the review-common.<slug> format",
        "block id fails `review-common.<slug>` format",
        common=lambda c: c["rules"][0].__setitem__("id", "review.default-fail"))
    add("a stub whose local text repeats the block's",
        "pointless override",
        schema=set_text("demo-grader.default-fail",
                        "Never default to a clearing verdict — earned only by a completed "
                        "hunt; absence of looking is never evidence."),
        clean=True)
    add("an orphan ${var} inherited from a common block is attributed to the stub",
        "demo-grader.default-fail: orphan placeholder ${nonexistent}",
        common=lambda c: c["rules"][0].__setitem__(
            "text", "Never default to ${nonexistent}."))

    # Orphan blocks: the sweep makes the claim once; a single-skill run never does.
    add("[sweep] a common block bound by no stub in any swept skill",
        "bound by no `extends:` stub in any swept skill: review-common.author-grader",
        sweep=True, clean=True)
    add("a single-skill run makes no orphan claim",
        "0 findings", clean=True,
        absent="bound by no `extends:` stub")

    # --- 8. pointer resolution (J-7) ---
    add("the in-directory and cross-directory pointers of the baseline resolve",
        "0 findings", clean=True, absent="resolves to no file")
    add("a pointer to a file that does not exist",
        "`pointer: references/GHOST.md` resolves to no file base-dir-relative",
        schema=set_field("demo-grader.checks-file", "pointer", "references/GHOST.md"))
    add("a cross-directory climb to a file that does not exist",
        "`pointer: ../other-skill/references/GHOST.md` resolves to no file",
        schema=set_field("demo-grader.shared-claims", "pointer",
                         "../other-skill/references/GHOST.md"))
    add("an absolute pointer path",
        "is absolute — paths ship base-dir-relative",
        schema=set_field("demo-grader.checks-file", "pointer", "/etc/hosts"))
    add("a skill-name pointer is a name, not a path — skipped",
        "0 findings",
        schema=set_field("demo-grader.ladder-home", "pointer", "mochiko:patterns-ghost"),
        clean=True, absent="resolves to no file")

    # --- 9. citations + section tokens in rule text ---
    add("a fabricated citation dangles",
        "text cites demo-grader.ghost-rule, which resolves to no node",
        schema=set_text("demo-grader.dispute-route",
                        "A disputed grade escalates (demo-grader.ghost-rule)."))

    def cite_tombstoned_rule(s):
        add_tombstone("demo-grader.legacy-rule", s)
        find_rule(s, "demo-grader.dispute-route")["text"] = \
            "A disputed grade escalates as demo-grader.legacy-rule says."
    add("a citation of a tombstoned rule is a superseded reference",
        "text cites demo-grader.legacy-rule, which is tombstoned",
        schema=cite_tombstoned_rule)
    add("a section-ID citation resolves",
        "0 findings",
        schema=set_text("demo-grader.dispute-route",
                        "Escalation is bounded by demo-grader.sec.reserved at all times."),
        clean=True, absent="demo-grader.sec.reserved, which")
    add("a foreign-stem citation is a warning, not a dangle",
        "citations with foreign stems",
        schema=set_text("demo-grader.dispute-route",
                        "The sibling seat owns this, per other-skill.carve-out."),
        clean=True)
    add("rule text naming a section that never existed",
        "text names section demo-grader.sec.ghost, which is not a node",
        schema=set_text("demo-grader.dispute-route",
                        "Escalate per demo-grader.sec.ghost."))

    def text_names_tombstone(s):
        add_tombstone("demo-grader.sec.legacy", s)
        find_rule(s, "demo-grader.dispute-route")["text"] = \
            "Escalate per demo-grader.sec.legacy."
    add("rule text naming a tombstoned section",
        "text names tombstoned section demo-grader.sec.legacy",
        schema=text_names_tombstone)

    # --- 10. vars + deixis ---
    add("an orphan ${var} placeholder",
        "orphan placeholder ${nonexistent}",
        schema=set_text("demo-grader.deep-hunt", "Hunt every class of ${nonexistent}."))
    add("a declared var no rule text uses",
        "vars.unused_var: declared but unused",
        schema=lambda s: s["vars"].__setitem__("unused_var", "nothing reads this"),
        clean=True)
    add("a deictic reference is a warning",
        "deictic reference",
        schema=set_text("demo-grader.deep-hunt", "Hunt every class; these rules bind."),
        clean=True)

    # --- 11. tombstones ---
    add("an ID both live and tombstoned",
        "both live and tombstoned",
        schema=lambda s: add_tombstone("demo-grader.never-author", s))
    add("a tombstone entry missing its disposition",
        "entry needs `id` + `disposition`",
        schema=lambda s: s.__setitem__("tombstones", [{"id": "demo-grader.legacy"}]))

    # --- 12. the SKILL.md floor pin (D6 desync guard) ---
    add("the pin names the wrong number",
        "pins 5 rules of `class: floor`, schema carries 3",
        md=lambda m: m.replace("the 3 rules of", "the 5 rules of"))
    add("the pin absent entirely",
        "no line pinning the `class: floor` count",
        md=lambda m: m.replace("the 3 rules of\n`class: floor`", "every floor that matters"))

    def single_floor(s):
        find_rule(s, "demo-grader.never-author")["class"] = "must"
        find_rule(s, "demo-grader.report-home")["class"] = "must"
    add("the pin plural where the count is 1",
        "want 'the 1 rule of'",
        schema=single_floor,
        md=lambda m: m.replace("the 3 rules of", "the 1 rules of"))
    add("a re-pinned count survives cleanly",
        "0 findings",
        schema=single_floor,
        md=lambda m: m.replace("the 3 rules of", "the 1 rule of"),
        clean=True)
    add("a second, disagreeing pin elsewhere in the body is caught",
        "pins 7 rules of `class: floor`, schema carries 3",
        md=lambda m: m + "\nRemember: the 7 rules of `class: floor` bind throughout.\n")

    # --- 13. the SKILL.md load-first section + enumeration ---
    add("the Rules heading absent",
        "canonical heading `## Rules — load the schema first` absent",
        md=lambda m: m.replace("## Rules — load the schema first", "## Rules"))
    add("the Rules block omits a live section",
        "Rules block does not enumerate demo-grader.sec.output",
        md=lambda m: m.replace("`demo-grader.sec.output` · ", ""))
    add("the Rules block names a section the schema lacks",
        "Rules block enumerates demo-grader.sec.ghost",
        md=lambda m: m.replace("`demo-grader.sec.reserved`",
                               "`demo-grader.sec.reserved` · `demo-grader.sec.ghost`"))
    add("a tombstoned token inside the Rules block gets the tombstone message",
        "Rules block enumerates demo-grader.sec.legacy, which is tombstoned",
        schema=lambda s: add_tombstone("demo-grader.sec.legacy", s),
        md=lambda m: m.replace("`demo-grader.sec.reserved`",
                               "`demo-grader.sec.reserved` · `demo-grader.sec.legacy`"),
        absent="demo-grader.sec.legacy, which is not a section")
    add("a dangling token outside the Rules block",
        "names section demo-grader.sec.ghost, which is not a node",
        md=lambda m: m.replace("1. Read the graded artifacts whole.",
                               "1. Read the graded artifacts whole (`demo-grader.sec.ghost`)."))

    def tombstone_legacy(s):
        add_tombstone("demo-grader.sec.legacy", s)
    add("a tombstoned token in the SKILL.md",
        "names tombstoned section demo-grader.sec.legacy",
        schema=tombstone_legacy,
        md=lambda m: m.replace("1. Read the graded artifacts whole.",
                               "1. Read the graded artifacts whole (`demo-grader.sec.legacy`)."))
    add("a foreign-stem token in the SKILL.md is a warning",
        "section tokens with foreign stems",
        md=lambda m: m.replace("1. Read the graded artifacts whole.",
                               "1. Read the graded artifacts whole (`other-skill.sec.inputs`)."),
        clean=True)

    # --- 14. the provenance sidecar (D8/C4 protection transfers) ---
    add("the sidecar absent is a warning, not a finding",
        "provenance sidecar absent", omit=("provenance",), clean=True)
    add("a dangling skill-prefixed entry",
        "dangling entry 'demo-grader.ghost-rule'",
        provenance=lambda pv: pv["anchors"].__setitem__(
            "demo-grader.ghost-rule", "2026-09-01 skill-content-schema D8"))

    def entry_for_tombstoned(pv):
        pv["anchors"]["demo-grader.legacy-rule"] = "2026-09-01 skill-content-schema D8"
    add("an entry naming a tombstoned rule",
        "entry 'demo-grader.legacy-rule' names a tombstoned rule",
        schema=lambda s: add_tombstone("demo-grader.legacy-rule", s),
        provenance=entry_for_tombstoned)
    add("an anchor that is malformed",
        "malformed — want 'YYYY-MM-DD <session-slug> [D#]'",
        provenance=lambda pv: pv["anchors"].__setitem__(
            "demo-grader.never-author", "some time ago"))
    add("an anchor resolving to no DECISIONS.md row",
        "resolves to no DECISIONS.md row",
        provenance=lambda pv: pv["anchors"].__setitem__(
            "demo-grader.never-author", "2099-01-01 no-such-session D1"))
    add("a foreign-prefix entry is skipped silently",
        "0 findings",
        provenance=lambda pv: pv["anchors"].__setitem__(
            "implement.ghost", "2099-01-01 no-such-session D1"),
        clean=True, absent="implement.ghost")
    add("the pre-rename command-provenance kind is accepted",
        "0 findings",
        provenance=lambda pv: pv.__setitem__("kind", "command-provenance"),
        clean=True, absent="`kind:` must be one of")
    add("a kind outside both provenance discriminators",
        "`kind:` must be one of command-provenance · primitive-provenance",
        provenance=lambda pv: pv.__setitem__("kind", "provenance"))

    return p


def run_probe(tmp: Path, probe):
    skills = tmp / "skills"
    skill_dir = skills / "demo-grader"
    (skill_dir / "references").mkdir(parents=True)
    other_refs = skills / "other-skill" / "references"
    other_refs.mkdir(parents=True)
    (skill_dir / "references" / "CHECKS.md").write_text(CHECKS_MD, encoding="utf-8")
    (other_refs / "SHARED.md").write_text(SHARED_MD, encoding="utf-8")

    schema = baseline_schema()
    if probe["schema"]:
        probe["schema"](schema)
    common = yaml.safe_load(yaml.safe_dump(BASELINE_COMMON))
    if probe["common"]:
        probe["common"](common)
    md = BASELINE_MD
    if probe["md"]:
        md = probe["md"](md)
    labels = yaml.safe_load(yaml.safe_dump(BASELINE_LABELS))
    if probe["labels"]:
        probe["labels"](labels)
    prov = yaml.safe_load(yaml.safe_dump(BASELINE_PROVENANCE))
    if probe["provenance"]:
        probe["provenance"](prov)

    schema_yaml = yaml.safe_dump(schema, sort_keys=False, allow_unicode=True)
    if probe["yamltext"]:
        schema_yaml = probe["yamltext"](schema_yaml)

    omit = probe["omit"]
    (skill_dir / "schema.yaml").write_text(schema_yaml, encoding="utf-8")
    if "md" not in omit:
        (skill_dir / "SKILL.md").write_text(md, encoding="utf-8")
    if "common" not in omit:
        (tmp / "skill-common.yaml").write_text(
            yaml.safe_dump(common, sort_keys=False, allow_unicode=True), encoding="utf-8")
    (tmp / "skill-labels.yaml").write_text(
        yaml.safe_dump(labels, sort_keys=False, allow_unicode=True), encoding="utf-8")
    if "provenance" not in omit:
        (tmp / "provenance.yaml").write_text(
            yaml.safe_dump(prov, sort_keys=False, allow_unicode=True), encoding="utf-8")
    (tmp / "DECISIONS.md").write_text(BASELINE_DECISIONS, encoding="utf-8")

    args = [sys.executable, str(CHECKER),
            "--dir", str(skills),
            "--labels", str(tmp / "skill-labels.yaml"),
            "--common", str(tmp / "skill-common.yaml"),
            "--provenance", str(tmp / "provenance.yaml"),
            "--decisions", str(tmp / "DECISIONS.md")]
    if not probe["sweep"]:
        args += ["--skill", "demo-grader"]
    proc = subprocess.run(args, capture_output=True, text=True)
    return proc.returncode, proc.stdout + proc.stderr


def main() -> int:
    failures = []
    matrix = probes()
    for probe in matrix:
        with tempfile.TemporaryDirectory() as td:
            code, out = run_probe(Path(td), probe)
        problems = []
        if probe["expected"] not in out:
            problems.append(f"expected {probe['expected']!r} in the output")
        if probe["absent"] and probe["absent"] in out:
            problems.append(f"expected {probe['absent']!r} NOT to appear")
        if probe["clean"] and code != 0:
            problems.append(f"expected a clean exit, got {code}")
        if not probe["clean"] and code == 0:
            problems.append("expected a finding (exit 1), got a clean exit")
        if problems:
            failures.append((probe["name"], problems, out))
            print(f"FAIL  {probe['name']}")
            for pr in problems:
                print(f"        {pr}")
        else:
            print(f"ok    {probe['name']}")

    print(f"\nnegative-test matrix: {len(matrix) - len(failures)}/{len(matrix)} probes passed")
    if failures:
        print("\n--- output of the first failing probe ---")
        print(failures[0][2])
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
