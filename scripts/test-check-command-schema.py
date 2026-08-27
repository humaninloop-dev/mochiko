#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.9"
# dependencies = ["pyyaml"]
# ///
"""Negative-test matrix for scripts/check-command-schema.py.

Every assertion the checker carries is exercised twice: once against a synthetic pair that
satisfies it (the positive control, which must come back clean) and once against a mutation
that breaks exactly that assertion (which must produce the named finding). A check that
cannot be made to fail is not a check.

The fixtures are synthetic — a `demo` command that exists only in a temp directory, with its
own label registry and its own shared block library — so the matrix never depends on the
shipped pairs and stays green while the real library is mid-wave.

Four mutation slots, because the checker reads four surfaces: `schema` and `md` mutate the
parsed fixtures, `common` mutates the shared block library, and `yamltext` mutates the dumped
YAML string — the only way to place a YAML comment, which is where D6's empty-`enforces:`
reason lives.

Run:  uv run scripts/test-check-command-schema.py
"""

import subprocess
import sys
import tempfile
from pathlib import Path

import yaml

CHECKER = Path(__file__).resolve().parent / "check-command-schema.py"

CANONICAL_SLUGS = ("roles", "reserved", "tools", "ways-of-working", "boundaries", "fail-conditions")


def rule(rid, labels, text, cls="must", **fields):
    """A rule block. `id` leads so the dumped YAML keeps the `- id: <rid>` opener the
    checker's raw-comment scan keys on."""
    return {"id": rid, "labels": list(labels), "class": cls, **fields, "text": text}


def baseline_schema():
    """A synthetic pair that satisfies every assertion — the positive control."""
    populated = {
        "roles": [
            rule("demo.lead-owns", ["role"],
                 "The lead owns the run, never produces, and states the verdict at close.",
                 kind="duty"),
        ],
        "tools": [
            rule("demo.read-first", ["binding"],
                 "Read ${target} in full before the first action.", kind="binding"),
            # An `extends:` stub: no local text, no local labels — both inherited. If
            # resolution did not run, this rule would report both as missing.
            {"id": "demo.register", "extends": "common.register", "class": "must",
             "kind": "binding"},
        ],
        "ways-of-working": [
            rule("demo.one-question", ["binding"], "Ask one question at a time."),
            rule("demo.deep-probe", ["binding"],
                 "Probe ${target} exhaustively.", when={"mode": "deep"}),
            rule("demo.quick-probe", ["binding"],
                 "One pass over ${target} is enough.", when={"mode": "shallow"}),
            rule("demo.map-read", ["binding"],
                 "The existing map is an obligated read.", when={"demo_map": "present"}),
            rule("demo.map-absent", ["binding"],
                 "A missing map is surfaced, never minted.", when={"demo_map": "absent"}),
            # The dependency idiom D5 ratified: a parenthetical rule-ID citation.
            rule("demo.escalate", ["binding"],
                 "An unresolved question escalates to the lead (demo.lead-owns)."),
        ],
        "boundaries": [
            rule("demo.no-silent-writes", ["binding"], "Never write outside ${target}.", "floor"),
            {"id": "demo.transport-floor", "extends": "common.transport-floor",
             "class": "floor", "when": {"seats": "multi"}},
        ],
        "fail-conditions": [
            rule("demo.fail.no-approval", ["user-gate"],
                 "Closing without the user's approval fails the run.", "floor",
                 kind="fail", enforces=["demo.lead-owns"]),
        ],
    }
    sections = []
    for slug in CANONICAL_SLUGS:
        node = {
            "id": f"demo.sec.{slug}",
            "title": slug.replace("-", " ").title(),
            "intent": f"the {slug} the run is bound by",
        }
        if slug in populated:
            node["rules"] = populated[slug]
        else:
            # `reserved` is the deliberate empty marker (D5 breadth invariant).
            node["rules"] = []
            node["note"] = "no reserved rulings yet — deliberately empty, not an omission"
        sections.append(node)
    return {
        "kind": "command",
        "command": "demo",
        "vars": {"target": "plugins/mochiko/schemas/demo.yaml"},
        "conditions": {
            "mode": {
                "values": ["deep", "shallow"],
                "resolution": "moment-resolved(open)",
                "note": "ruled at the open; rules gated on it wait until it resolves.",
            },
            "demo_map": {
                "values": "presence",
                "resolution": "surface-presence",
                "note": "the demo map file.",
            },
            "seats": {
                "values": ["single", "multi"],
                "resolution": "standing-trigger",
                "note": "fires the moment the run composes more than one seat.",
            },
        },
        "moments": {
            "open": "Where the demonstration's mode is ruled.",
            "close": "Where the lead states the verdict.",
        },
        "sections": sections,
    }


BASELINE_COMMON = {
    "kind": "command-common",
    "rules": [
        {"id": "common.register", "labels": ["reporting"],
         "text": "User-facing prose follows the house register."},
        {"id": "common.transport-floor", "labels": ["floor-pointer"],
         "text": "The transport floor governs multi-seat composition, referenced never restated.",
         "pointer": "mochiko:patterns-transport-floor"},
    ],
}


# Identity carries a numbered list of its own, the shape implement.md has. A protocol-step scan
# keyed to the file's first numbered list would read this one and misreport the pair, so the
# baseline every probe runs against carries the hazard.
IDENTITY_BLOCK = """## Identity & Mission

You lead the demo desk and steward nothing real. Two standing obligations:

1. **Sufficiency check** — refuse to open on an under-specified brief.
2. **Seat discipline** — you never produce what you grade.

"""

BASELINE_MD = """---
description: A synthetic command used only by the checker's negative-test matrix.
argument-hint: "[thing]"
disable-model-invocation: true
---

# Demo — The Demo Desk

""" + IDENTITY_BLOCK + """## Rules — load the schema first

Read `plugins/mochiko/schemas/demo.yaml` in full before your first action, and
`plugins/mochiko/schemas/common.yaml` in the same first action. It carries the
run's binding rules, nested in six sections, each addressable by its section ID:
`demo.sec.roles` (who leads) · `demo.sec.reserved` (decisions held by the user) ·
`demo.sec.tools` (tool bindings) · `demo.sec.ways-of-working` · `demo.sec.boundaries` (the
non-waivable floor) · `demo.sec.fail-conditions` (the Not-done set).

A rule carrying `when:` binds only when the run's declared shape matches every term.

## Adaptive Goal Protocol

1. **Entry** — `$ARGUMENTS` names the thing under demonstration.
2. **Goal** — the demonstration closed with the user's approval.
3. **Not done — default FAIL:** the 1 rule of `kind: fail` in
   `plugins/mochiko/schemas/demo.yaml` (section `demo.sec.fail-conditions`) — any one
   standing fails the run.
"""

# The Not-done block as the baseline carries it, for probes that move or displace it.
NOT_DONE_BLOCK = """3. **Not done — default FAIL:** the 1 rule of `kind: fail` in
   `plugins/mochiko/schemas/demo.yaml` (section `demo.sec.fail-conditions`) — any one
   standing fails the run.
"""

BASELINE_LABELS = {
    "kind": "command-labels",
    "labels": {
        "role": "who holds a seat",
        "binding": "an obligation on the run",
        "user-gate": "a decision or checkpoint reserved to the user",
        "reporting": "where reports land and the register they use",
        "floor-pointer": "a binding that points at a skill-owned floor",
    },
}

BASELINE_PROVENANCE = {
    "kind": "command-provenance",
    "anchors": {"demo.lead-owns": "2026-08-27 demo-session D1"},
}

BASELINE_DECISIONS = "| 2026-08-27 | A synthetic row for demo-session | ruled | [x](y) |\n"


# --- probe helpers -------------------------------------------------------------------------

def find_rule(schema, rid):
    for sec in schema["sections"]:
        for r in sec.get("rules") or []:
            if r["id"] == rid:
                return r
    raise AssertionError(f"fixture has no rule {rid}")


def drop_section(slug):
    def mutate(s):
        s["sections"] = [n for n in s["sections"] if n["id"] != f"demo.sec.{slug}"]
    return mutate


def add_tombstone(sid, s):
    s.setdefault("tombstones", []).append({"id": sid, "disposition": "retired at the scaffold wave"})


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


def comment_above(anchor, *comment_lines):
    """Put YAML comment lines directly above `anchor`, at the anchor's own indentation.

    The dump's indentation is safe_dump's to choose, so it is read off the anchor line
    rather than assumed — a hardcoded indent breaks the parse.
    """
    def mutate(text):
        out = []
        for line in text.splitlines():
            if line.strip() == anchor:
                pad = line[: len(line) - len(line.lstrip())]
                out += [f"{pad}# {c}" for c in comment_lines]
            out.append(line)
        return "\n".join(out) + "\n"
    return mutate


def probes():
    """Each probe breaks exactly one assertion and names the finding it must produce."""
    p = []

    def add(name, expected, schema=None, md=None, common=None, labels=None, provenance=None,
            yamltext=None, clean=False, absent=None, omit=()):
        # `absent` is the other half of a "stays clean" probe: asserting only that no finding
        # appeared would also pass if the check never ran at all, so the probes that exist to
        # prove a NON-finding name the message that must not be there.
        # `omit` withholds a fixture file entirely — the only way to reach the file-absent
        # branches ("schema" · "md" · "common" · "provenance").
        p.append({"name": name, "schema": schema, "md": md, "common": common, "labels": labels,
                  "provenance": provenance, "yamltext": yamltext, "expected": expected,
                  "clean": clean, "absent": absent, "omit": omit})

    # --- the positive control ---
    add("baseline pair is clean", "0 findings", clean=True)

    # --- 1. set-wise section assertion (scaffold D5/D4) ---
    add("canonical section absent",
        "canonical section demo.sec.tools absent",
        schema=drop_section("tools"))

    def extra_section(s):
        s["sections"].append({"id": "demo.sec.extras", "title": "Extras", "intent": "x",
                              "rules": [rule("demo.stray", ["binding"], "A stray rule.")]})
    add("section outside the canonical six",
        "demo.sec.extras: not one of the six canonical sections",
        schema=extra_section)

    def mixed_prefix(s):
        s["sections"][2]["id"] = "other.sec.tools"
    add("section IDs disagree on the prefix",
        "section IDs disagree on the rule prefix",
        schema=mixed_prefix)

    # --- 2. empty-marker recognition (D5 breadth invariant) ---
    def empty_without_note(s):
        for n in s["sections"]:
            if n["id"] == "demo.sec.reserved":
                n.pop("note")
    add("empty section carrying no note",
        "demo.sec.reserved: empty with no `note:`",
        schema=empty_without_note)

    # --- 3. .md scaffold headings ---
    add("canonical heading absent",
        "canonical heading `## Identity & Mission` absent",
        md=lambda m: m.replace("## Identity & Mission", "## Who You Are"))

    def swap_heading_order(m):
        return m.replace(IDENTITY_BLOCK, "").replace(
            "## Adaptive Goal Protocol", IDENTITY_BLOCK + "## Adaptive Goal Protocol")
    add("canonical headings out of D2 order",
        "canonical headings out of D2 order",
        md=swap_heading_order)

    # --- 4. Rules-block enumeration, set-wise against the schema ---
    add("Rules block omits a live section",
        "Rules block does not enumerate demo.sec.boundaries",
        md=lambda m: m.replace("`demo.sec.boundaries` (the\nnon-waivable floor) · ", ""))
    add("Rules block names a section the schema lacks",
        "Rules block enumerates demo.sec.ghost",
        md=lambda m: m.replace("`demo.sec.roles` (who leads)",
                               "`demo.sec.roles` (who leads) · `demo.sec.ghost`"))

    # --- 5. Not-done count-pin, re-keyed off the retired label to `kind: fail` (D7 C2 guard,
    #        D6-R4, ontology build item 4) ---
    add("count-pin names the wrong number",
        "pins 3 rules of `kind: fail`, schema carries 1",
        md=lambda m: m.replace("the 1 rule of", "the 3 rules of"))
    add("count-pin plural where the count is 1",
        "want 'the 1 rule of'",
        md=lambda m: m.replace("the 1 rule of", "the 1 rules of"))

    def two_fail_rules(s):
        s["sections"][-1]["rules"].append(
            rule("demo.fail.no-evidence", ["user-gate"], "Closing without evidence fails.",
                 "floor", kind="fail", enforces=["demo.read-first"]))
    add("count-pin singular where the count is 2",
        "want 'the 2 rules of'",
        schema=two_fail_rules,
        md=lambda m: m.replace("the 1 rule of", "the 2 rule of"))
    add("a second `kind: fail` rule re-pins the count cleanly",
        "0 findings",
        schema=two_fail_rules,
        md=lambda m: m.replace("the 1 rule of", "the 2 rules of"),
        clean=True)

    add("count-pin absent entirely",
        "no Not-done line hard-coding the `kind: fail` count",
        md=lambda m: m.replace("the 1 rule of `kind: fail`", "every rule that matters"))

    add("stale 'nested in N sections' numeral",
        "says 'nested in three sections'",
        md=lambda m: m.replace("nested in six sections", "nested in three sections"))

    # --- 5b. the Not-done line closes the protocol (D2: "always last") ---
    def not_done_above_protocol(m):
        # specify.md's F4 drift: the count-pin floated above the protocol heading.
        return m.replace(NOT_DONE_BLOCK, "").replace(
            "## Rules — load the schema first", NOT_DONE_BLOCK + "\n## Rules — load the schema first")
    add("Not-done line sits above the protocol heading",
        "sits above `## Adaptive Goal Protocol`",
        md=not_done_above_protocol)

    add("a section heading follows the Not-done line",
        "`## Boundaries` follows the Not-done line",
        md=lambda m: m + "\n## Boundaries\n\nSomething trailing the count-pin.\n")

    # implement.md's shape: a numbered list in Identity ahead of the protocol's own. The step
    # scan is anchored to the region below the protocol heading, so the identity list must not
    # be mistaken for protocol steps — a false PASS here would be silent.
    add("a second numbered list above the protocol heading stays clean",
        "0 findings",
        md=lambda m: m.replace(
            "2. **Seat discipline** — you never produce what you grade.",
            "2. **Seat discipline** — you never produce what you grade.\n"
            "3. **Standing brief** — you re-read the brief at every visit."),
        clean=True)

    # --- 6. all-token resolution, outside the Rules block (the I2 case) ---
    add("dangling token outside the Rules block",
        "names section demo.sec.ghost, which is not a node",
        md=lambda m: m.replace("2. **Goal** — the demonstration closed",
                               "2. **Goal** (`demo.sec.ghost`) — the demonstration closed"))

    def tombstone_legacy(s):
        add_tombstone("demo.sec.legacy", s)
    add("tombstoned token in the .md",
        "names tombstoned section demo.sec.legacy",
        schema=tombstone_legacy,
        md=lambda m: m.replace("2. **Goal** — the demonstration closed",
                               "2. **Goal** (`demo.sec.legacy`) — the demonstration closed"))
    add("foreign-prefix token is a warning, not a finding",
        "section tokens with foreign prefixes",
        md=lambda m: m.replace("2. **Goal** — the demonstration closed",
                               "2. **Goal** (`other.sec.roles`) — the demonstration closed"),
        clean=True)

    # --- 7. tombstone-reference lint over rule text (I8) ---
    def text_names_tombstone(s):
        add_tombstone("demo.sec.legacy", s)
        find_rule(s, "demo.lead-owns")["text"] = "The lead owns the run, per demo.sec.legacy."
    add("rule text names a tombstoned node",
        "text names tombstoned section demo.sec.legacy",
        schema=text_names_tombstone)

    add("rule text names a node that never existed",
        "text names section demo.sec.ghost, which is not a node",
        schema=set_text("demo.lead-owns", "The lead owns the run, per demo.sec.ghost."))

    # --- 8. `kind:` vocabulary (ontology D1) ---
    add("kind outside the closed set",
        "demo.lead-owns: `kind: sproing` is not one of",
        schema=set_field("demo.lead-owns", "kind", "sproing"))

    def every_legal_kind(s):
        # One rule per kind that is not already exercised by the baseline, to prove the
        # vocabulary admits the whole set and not just the members the fixture happens to use.
        sec = s["sections"][3]["rules"]
        for i, k in enumerate(("gate", "reservation", "bound", "routing", "latitude",
                               "constraint")):
            sec.append(rule(f"demo.kind-{k}", ["binding"], f"A rule of kind {k}.", kind=k))
    add("every legal kind is admitted",
        "0 findings", schema=every_legal_kind, clean=True)

    add("an absent kind reads constraint, never a finding",
        "0 findings", schema=drop_field("demo.lead-owns", "kind"), clean=True,
        absent="is not one of")

    # --- 9. the fail re-key, bidirectional and never defaulted (build item 4, I4) ---
    add("a .fail. rule with no explicit kind",
        "under the .fail. segment with no explicit `kind: fail`",
        schema=drop_field("demo.fail.no-approval", "kind"))
    add("a .fail. rule carrying some other kind",
        "under the .fail. segment but carries `kind: gate`",
        schema=set_field("demo.fail.no-approval", "kind", "gate"))

    def fail_kind_off_segment(s):
        find_rule(s, "demo.one-question")["kind"] = "fail"
    add("kind: fail outside the .fail. segment",
        "`kind: fail` outside the .fail. segment",
        schema=fail_kind_off_segment)

    # --- 10. the retired `fail-condition` label, wherever it is named ---
    add("retired label on a rule",
        "carries the retired 'fail-condition' label",
        schema=set_field("demo.one-question", "labels", ["fail-condition"]))
    add("retired label still in the registry",
        "label 'fail-condition' still registered",
        labels=lambda lb: lb["labels"].__setitem__(
            "fail-condition", "a clause of the Not-done set"))
    add("retired label named in the .md",
        "names the retired `fail-condition` label",
        md=lambda m: m.replace("the 1 rule of `kind: fail`",
                               "the 1 rule labeled `fail-condition`"))

    # The selector named in prose rather than as a label is warning-class: it is superseded
    # wording, not a superseded data key. The `fail-conditions` section slug is live
    # vocabulary and must not trip either probe.
    add("the retired selector named in a section intent",
        "demo.sec.fail-conditions: `intent` names the retired `fail-condition` selector",
        schema=lambda s: s["sections"][5].__setitem__(
            "intent", "The fail-condition set — any one standing fails the run."),
        clean=True)
    add("the retired selector named in a rule text",
        "demo.one-question: text names the retired `fail-condition` selector",
        schema=set_text("demo.one-question",
                        "Ask one question at a time, and mind every fail-condition."),
        clean=True)

    # --- 11. `conditions:` / `when:` (ontology D3) ---
    add("when: names an undeclared dimension",
        "`when:` names dimension 'ghost'",
        schema=set_field("demo.deep-probe", "when", {"ghost": "deep"}))
    add("when: names an undeclared value",
        "not a declared value of 'mode'",
        schema=set_field("demo.deep-probe", "when", {"mode": "sideways"}))
    add("when: written as a list, not a conjunction mapping",
        "`when:` must be a non-empty mapping",
        schema=set_field("demo.deep-probe", "when", ["mode", "deep"]))
    add("when: carrying nested structure (boolean algebra)",
        "carries nested structure",
        schema=set_field("demo.deep-probe", "when", {"mode": [{"not": "shallow"}]}))
    add("when: naming a list of declared values stays clean",
        "0 findings",
        schema=set_field("demo.deep-probe", "when", {"mode": ["deep", "shallow"]}),
        clean=True)
    add("when: on a presence dimension, both poles clean",
        "0 findings",
        schema=set_field("demo.map-read", "when", {"demo_map": "present"}),
        clean=True)

    add("when: naming a dimension with an empty value list",
        "`when: {mode: []}` names no value",
        schema=set_field("demo.deep-probe", "when", {"mode": []}))

    # The `conditions:` container itself, before any dimension is read.
    add("conditions: written as something other than a mapping",
        "conditions: must be a non-empty mapping",
        schema=lambda s: s.__setitem__("conditions", ["mode", "seats"]))
    add("a dimension declared as a bare string",
        "conditions.mode: declaration must be a mapping",
        schema=lambda s: s["conditions"].__setitem__("mode", "deep or shallow"))
    add("a dimension whose values: is neither a list nor `presence`",
        "conditions.mode: `values` must be a non-empty list",
        schema=lambda s: s["conditions"]["mode"].__setitem__("values", "sometimes"))

    def bad_resolution(s):
        s["conditions"]["mode"]["resolution"] = "whenever"
    add("resolution point outside D3's closed set",
        "resolution 'whenever' is not one of",
        schema=bad_resolution)

    def moment_resolved_ghost(s):
        s["conditions"]["mode"]["resolution"] = "moment-resolved(nowhere)"
    add("moment-resolved names an undeclared moment",
        "`moments:` block does not declare",
        schema=moment_resolved_ghost)

    def unused_dimension(s):
        s["conditions"]["unused_dim"] = {"values": ["a", "b"], "resolution": "entry-derived"}
    add("declared dimension no rule uses is a warning",
        "conditions.unused_dim: declared but no rule's `when:` names it",
        schema=unused_dimension, clean=True)

    def unused_value(s):
        find_rule(s, "demo.quick-probe").pop("when")
    add("declared value named by no rule's when: is a warning",
        "conditions.mode: value 'shallow' declared but named by no rule's `when:`",
        schema=unused_value, clean=True)

    # The two questions the checker asks about a value are deliberately different: the warning
    # asks whether ANY rule names it, the C4 coverage report asks which NON-floor rules do. A
    # value carried only by a floor must therefore stay out of the warning list and still read
    # "(no rule activates)" in the report — implement's `scope: lane` is the live case.
    def value_carried_only_by_a_floor(s):
        find_rule(s, "demo.quick-probe").pop("when")
        find_rule(s, "demo.no-silent-writes")["when"] = {"mode": "shallow"}
    add("a value carried only by a floor makes no coverage claim either way",
        "shallow: (no rule activates)",
        schema=value_carried_only_by_a_floor, clean=True,
        absent="value 'shallow' declared but named by no rule's `when:`")

    # --- 12. `moments:` (ontology D4) ---
    add("moments: written as something other than a mapping",
        "moments: must be a non-empty mapping",
        schema=lambda s: s.__setitem__("moments", ["open", "close"]))
    add("a moment declared with no navigation line",
        "moments.open: navigation line missing or empty",
        schema=lambda s: s["moments"].__setitem__("open", ""))

    def unused_moment(s):
        s["moments"]["landing"] = "Where the run's outputs land."
    add("declared moment named by nothing is a warning",
        "moments.landing: declared but named by no moment-resolved condition",
        schema=unused_moment, clean=True)

    def moment_used_in_prose(s):
        s["moments"]["landing"] = "Where the run's outputs land."
        find_rule(s, "demo.one-question")["text"] = "Ask one question at a time, through landing."
    add("a moment mentioned only in prose counts as used",
        "0 findings", schema=moment_used_in_prose, clean=True,
        absent="moments.landing: declared but named by")

    # --- 13. the per-dimension coverage report (D3, advisory stdout) ---
    add("coverage report names an uncovered value",
        "(no rule activates)", schema=unused_value, clean=True)
    add("coverage report makes no claim over floors",
        "floor — always delivered, no coverage claim", clean=True)

    # --- 14. in-text ID citations (ontology D5, scan surface pinned at J-11) ---
    add("a fabricated citation dangles",
        "text cites demo.ghost-rule, which resolves to no node",
        schema=set_text("demo.escalate", "Escalate to the lead (demo.ghost-rule)."))

    def cite_tombstoned_rule(s):
        add_tombstone("demo.legacy-rule", s)
        find_rule(s, "demo.escalate")["text"] = "Escalate as demo.legacy-rule says."
    add("a citation of a tombstoned rule is a superseded reference",
        "text cites demo.legacy-rule, which is tombstoned",
        schema=cite_tombstoned_rule)

    # J-11's first named negative test: `feat.staffing-latitude` cites a SECTION id, and a
    # resolver that knows only rule IDs reports a false dangle.
    add("a section-ID citation resolves",
        "0 findings",
        schema=set_text("demo.escalate",
                        "The bare minimum is carried as the desk rules in demo.sec.roles."),
        clean=True, absent="demo.sec.roles, which")

    # J-11's second: `spec.md` is a path, not a citation (M3).
    add("a file-suffix token is not a citation",
        "0 findings",
        schema=set_text("demo.escalate",
                        "The seat writes spec.md and demo.yaml, never a report."),
        clean=True, absent="cites spec.md")

    # J-11's third: three live citations are bare inline, not parenthetical.
    add("the bare (non-parenthetical) citation form is scanned",
        "text cites demo.ghost-rule, which resolves to no node",
        schema=set_text("demo.escalate", "Escalation is bounded by demo.ghost-rule at all times."))

    add("a foreign-prefix citation is a warning, not a dangle",
        "citations with foreign prefixes",
        schema=set_text("demo.escalate", "The specify run owns this, per spec.gate-acceptance."),
        clean=True)

    # --- 15. `enforces:` (ontology D6) ---
    add("a kind: fail node with no enforces:",
        "`kind: fail` node carries no `enforces:`",
        schema=drop_field("demo.fail.no-approval", "enforces"))
    add("an enforces: target that resolves to nothing",
        "`enforces: demo.ghost` resolves to no rule",
        schema=set_field("demo.fail.no-approval", "enforces", ["demo.ghost"]))

    def enforces_tombstoned(s):
        add_tombstone("demo.legacy-rule", s)
        find_rule(s, "demo.fail.no-approval")["enforces"] = ["demo.legacy-rule"]
    add("an enforces: target that is tombstoned",
        "`enforces: demo.legacy-rule` names a tombstoned rule",
        schema=enforces_tombstoned)

    add("enforces: on a node that is not kind: fail",
        "the field is a fail node's mirror link",
        schema=set_field("demo.one-question", "enforces", ["demo.lead-owns"]))
    add("enforces: written as a bare string, not a list",
        "`enforces:` must be a list of local rule IDs",
        schema=set_field("demo.fail.no-approval", "enforces", "demo.lead-owns"))
    add("enforces: naming a section rather than a rule",
        "`enforces: demo.sec.roles` names a section",
        schema=set_field("demo.fail.no-approval", "enforces", ["demo.sec.roles"]))

    add("an empty enforces: with no stated reason",
        "`enforces: []` with no stated reason",
        schema=set_field("demo.fail.no-approval", "enforces", []))

    # The shape the converted schemas actually use (setup.yaml's two empty mirrors): the
    # reason is a YAML comment, which PyYAML discards — so the checker reads the raw file.
    add("an empty enforces: carrying its D6 reason comment",
        "0 findings",
        schema=set_field("demo.fail.no-approval", "enforces", []),
        yamltext=comment_above(
            "enforces: []",
            "D6 empty-with-reason: the obligation is owned by a pointer skill,",
            "bound at demo.read-first — no local rule states it."),
        clean=True)

    add("reverse coverage is labelled as the deferred pass's input, not a finding",
        "input to the deferred Desk FAIL-set widening pass", clean=True)

    # --- 16. `extends:` (ontology D8 as amended by C3) ---
    add("extends: names no block in the library",
        "`extends: common.ghost` names no block",
        schema=set_field("demo.register", "extends", "common.ghost"))
    add("extends: target malformed",
        "want `common.<slug>`",
        schema=set_field("demo.register", "extends", "register"))
    add("an extends: stub declaring no local class",
        "declares no local `class:`",
        schema=drop_field("demo.register", "class"))
    add("a stub whose local text repeats the block's",
        "pointless override",
        schema=set_text("demo.register", "User-facing prose follows the house register."),
        clean=True)

    def orphan_block(s):
        s["sections"][2]["rules"] = [r for r in s["sections"][2]["rules"]
                                     if r["id"] != "demo.register"]
    add("a common block bound by no stub",
        "common.register: bound by no `extends:` stub",
        schema=orphan_block, clean=True)

    for field, value in (("kind", "binding"), ("when", {"seats": "multi"}),
                         ("enforces", ["demo.lead-owns"])):
        add(f"a common block carrying `{field}:`",
            "an absence-meaningful field is never inherited",
            common=(lambda f, v: (lambda c: c["rules"][0].__setitem__(f, v)))(field, value))

    add("a common block carrying `class:` is a warning",
        "inherited-but-always-overridden dead weight",
        common=lambda c: c["rules"][0].__setitem__("class", "must"), clean=True)

    # --- 16b. the library's own document and block guards (D8) ---
    # An absent library is a finding only where a stub binds it, and it makes every stub
    # unresolvable — one fixture, two distinct asserts.
    add("the shared library is absent while a stub binds it",
        "shared block library absent", omit=("common",))
    add("an extends: stub whose library never loaded",
        "unresolvable — common.yaml did not load", omit=("common",))

    add("the library missing its kind: discriminator",
        "`kind: command-common` missing",
        common=lambda c: c.__setitem__("kind", "command-rules"))
    add("the library carrying no rules: list",
        "`rules:` list of common blocks missing or empty",
        common=lambda c: c.__setitem__("rules", []))
    add("a common block with no id",
        "needs an `id`",
        common=lambda c: c["rules"][0].pop("id"))
    add("a common block id outside the common.<slug> format",
        "block id fails `common.<slug>` format",
        common=lambda c: c["rules"][0].__setitem__("id", "register"))

    def duplicate_common_id(c):
        c["rules"].append({"id": "common.register", "labels": ["reporting"],
                           "text": "A second block minting the same id."})
    add("a duplicate common block id",
        "duplicate block id", common=duplicate_common_id)

    add("a common block with no text",
        "common.register: `text` missing or empty",
        common=lambda c: c["rules"][0].__setitem__("text", "   "))

    # Text-side checks must run against RESOLVED text, and attribute to the STUB — otherwise
    # a defect inherited from the library is invisible on every binding command (C3).
    add("an orphan ${var} inherited from a common block is attributed to the stub",
        "demo.register: orphan placeholder ${nonexistent}",
        common=lambda c: c["rules"][0].__setitem__(
            "text", "User-facing prose follows ${nonexistent}."))
    add("deixis inherited from a common block is attributed to the stub",
        "demo.register: deictic reference",
        common=lambda c: c["rules"][0].__setitem__(
            "text", "User-facing prose follows these rules."), clean=True)

    # --- 17. regression sweep: every pre-existing check still fires ---
    add("[regression] inline ruling: field",
        "inline `ruling:`", schema=set_field("demo.lead-owns", "ruling", "2026-08-27 demo-session"))

    add("[regression] dangling provenance entry",
        "dangling entry 'demo.lead-owns'", schema=set_field("demo.lead-owns", "id", "demo.lead-steers"))

    add("[regression] label outside the registry",
        "not in demo-labels.yaml", schema=set_field("demo.lead-owns", "labels", ["not-a-real-label"]))

    add("[regression] orphan ${var} placeholder",
        "orphan placeholder ${nonexistent}",
        schema=set_text("demo.lead-owns", "The lead owns ${nonexistent}."))

    add("[regression] deictic reference is a warning",
        "deictic reference",
        schema=set_text("demo.lead-owns", "The lead owns the run; these rules bind."), clean=True)

    add("[regression] duplicate rule ID",
        "duplicate id", schema=set_field("demo.read-first", "id", "demo.lead-owns"))

    def flat_rules(s):
        s["rules"] = [rule("demo.flat", ["binding"], "A rule at the top level.")]
    add("[regression] flat top-level rules:",
        "flat grammar superseded by sections", schema=flat_rules)

    def bad_section_id(s):
        s["sections"][2]["id"] = "demo-tools"
    add("[regression] malformed section ID",
        "section id fails", schema=bad_section_id)

    # --- 18. document-level guards on each of the four files the checker reads ---
    add("the schema file is missing entirely",
        "demo.yaml: file not found", omit=("schema",))
    add("the schema does not parse as YAML",
        "YAML parse break",
        yamltext=lambda y: y + "\n  this: [is not\n   valid: yaml\n")
    add("the .md file is missing entirely",
        "demo.md: file not found", omit=("md",))
    add("the schema missing its kind: discriminator",
        "`kind: command` missing",
        schema=lambda s: s.__setitem__("kind", "command-schema"))
    add("the schema missing its command: name",
        "`command:` name missing", schema=lambda s: s.pop("command"))
    add("the registry missing its kind: discriminator",
        "`kind: command-labels` missing",
        labels=lambda lb: lb.__setitem__("kind", "labels"))
    add("the registry carrying no labels mapping",
        "`labels:` mapping missing or empty",
        labels=lambda lb: lb.__setitem__("labels", {}))
    add("the schema carrying no sections",
        "`sections:` list missing or empty",
        schema=lambda s: s.__setitem__("sections", []))

    # --- 19. tombstone shape and integrity (D11) ---
    add("tombstones: written as something other than a list",
        "tombstones: must be a list",
        schema=lambda s: s.__setitem__("tombstones", "demo.sec.legacy"))
    add("a tombstone entry missing its disposition",
        "entry needs `id` + `disposition`",
        schema=lambda s: s.__setitem__("tombstones", [{"id": "demo.sec.legacy"}]))

    def duplicate_tombstone(s):
        add_tombstone("demo.sec.legacy", s)
        add_tombstone("demo.sec.legacy", s)
    add("the same node tombstoned twice",
        "duplicate tombstone for demo.sec.legacy", schema=duplicate_tombstone)
    add("an ID both live and tombstoned",
        "both live and tombstoned",
        schema=lambda s: add_tombstone("demo.lead-owns", s))

    # --- 20. section shape (D14) ---
    add("a section entry that is not a mapping",
        "not a mapping", schema=lambda s: s["sections"].append("demo.sec.stray"))
    add("a section missing its id",
        "`id` missing", schema=lambda s: s["sections"][2].pop("id"))
    add("two sections minting the same id",
        "demo.sec.roles: duplicate id",
        schema=lambda s: s["sections"][2].__setitem__("id", "demo.sec.roles"))
    add("a section missing its title",
        "`title` missing or empty", schema=lambda s: s["sections"][2].pop("title"))
    add("a section missing its rules key",
        "`rules` key missing", schema=lambda s: s["sections"][2].pop("rules"))
    add("a section whose rules: is not a list",
        "`rules` must be a list",
        schema=lambda s: s["sections"][2].__setitem__("rules", "demo.read-first"))

    def empty_rules_written_as_none(s):
        for n in s["sections"]:
            if n["id"] == "demo.sec.reserved":
                n["rules"] = None
    add("an empty section written as `rules:` rather than `rules: []`",
        "prefer explicit `rules: []`", schema=empty_rules_written_as_none, clean=True)

    def all_section_ids_malformed(s):
        for i, n in enumerate(s["sections"]):
            n["id"] = f"section-{i}"
    add("no section ID well-formed enough to derive the prefix",
        "cannot derive the rule prefix", schema=all_section_ids_malformed)

    # --- 21. rule shape ---
    # The index is the rule's position within its OWN section — tools carries read-first at 0
    # and register at 1, so the appended entry is 2. A flattened index would say 3.
    add("a rule entry that is not a mapping",
        "demo.sec.tools: rules[2] not a mapping",
        schema=lambda s: s["sections"][2]["rules"].append("demo.stray"))
    add("a rule missing its id",
        "`id` missing", schema=lambda s: find_rule(s, "demo.read-first").pop("id"))
    add("a rule id outside the dotted-slug format",
        "id fails dotted-slug format",
        schema=set_field("demo.read-first", "id", "Demo_Read_First"))
    add("a class outside floor|must|advisory",
        "`class` must be floor|must|advisory",
        schema=set_field("demo.read-first", "class", "mandatory"))
    add("a rule carrying no labels",
        "`labels` missing or empty",
        schema=set_field("demo.read-first", "labels", []))
    add("a rule carrying no text",
        "demo.read-first: `text` missing or empty",
        schema=set_text("demo.read-first", "   "))
    add("a {{...}} skeleton sigil in rule text is a warning",
        "skeleton convention, not var substitution",
        schema=set_text("demo.read-first", "Read {{target}} in full before the first action."),
        clean=True)

    # --- 22. the two rollout warnings ---
    add("a declared var no rule text uses",
        "vars.unused_var: declared but unused",
        schema=lambda s: s["vars"].__setitem__("unused_var", "nothing/reads/this"),
        clean=True)
    add("a registry label with no members here",
        "label 'stewardship': zero members",
        labels=lambda lb: lb["labels"].__setitem__("stewardship", "living-artifact care"),
        clean=True)

    # --- 23. the provenance sidecar (D16) ---
    add("the sidecar absent is a warning, not a finding",
        "provenance sidecar absent", omit=("provenance",), clean=True)
    add("the sidecar missing its kind: discriminator",
        "`kind: command-provenance` missing",
        provenance=lambda pv: pv.__setitem__("kind", "provenance"))
    add("the sidecar carrying no anchors mapping",
        "`anchors:` mapping missing",
        provenance=lambda pv: pv.__setitem__("anchors", ["demo.lead-owns"]))
    add("an anchor that is malformed",
        "malformed — want 'YYYY-MM-DD <session-slug> [D#]'",
        provenance=lambda pv: pv["anchors"].__setitem__("demo.lead-owns", "some time ago"))
    add("an anchor resolving to no DECISIONS.md row",
        "resolves to no DECISIONS.md row",
        provenance=lambda pv: pv["anchors"].__setitem__(
            "demo.lead-owns", "2099-01-01 no-such-session D1"))
    add("sidecar entries for another command are skipped, with a warning",
        "provenance entries with foreign prefixes skipped",
        provenance=lambda pv: pv["anchors"].__setitem__(
            "other.rule", "2026-08-27 demo-session D1"),
        clean=True)

    # --- 24. the count-pin's placement, where the phrase is not on a Not-done line ---
    add("the count phrase sitting on a line that is not the Not-done line",
        "the count-pin is not on a `**Not done — default FAIL:**` line",
        md=lambda m: m.replace(
            NOT_DONE_BLOCK,
            "3. **Closing** — the 1 rule of `kind: fail` in\n"
            "   `plugins/mochiko/schemas/demo.yaml` applies.\n"))

    return p


def run_probe(tmp: Path, probe):
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

    schema_yaml = yaml.safe_dump(schema, sort_keys=False)
    if probe["yamltext"]:
        schema_yaml = probe["yamltext"](schema_yaml)

    omit = probe["omit"]
    if "schema" not in omit:
        (tmp / "demo.yaml").write_text(schema_yaml, encoding="utf-8")
    if "md" not in omit:
        (tmp / "demo.md").write_text(md, encoding="utf-8")
    if "common" not in omit:
        (tmp / "common.yaml").write_text(yaml.safe_dump(common, sort_keys=False), encoding="utf-8")
    (tmp / "demo-labels.yaml").write_text(yaml.safe_dump(labels, sort_keys=False), encoding="utf-8")
    if "provenance" not in omit:
        (tmp / "provenance.yaml").write_text(yaml.safe_dump(prov, sort_keys=False), encoding="utf-8")
    (tmp / "DECISIONS.md").write_text(BASELINE_DECISIONS, encoding="utf-8")

    proc = subprocess.run(
        [sys.executable, str(CHECKER),
         "--schema", str(tmp / "demo.yaml"),
         "--md", str(tmp / "demo.md"),
         "--common", str(tmp / "common.yaml"),
         "--labels", str(tmp / "demo-labels.yaml"),
         "--provenance", str(tmp / "provenance.yaml"),
         "--decisions", str(tmp / "DECISIONS.md")],
        capture_output=True, text=True,
    )
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
