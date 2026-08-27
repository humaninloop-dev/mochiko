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

The fixtures are synthetic — a `demo` command that exists only in a temp directory — so the
matrix never depends on the shipped pairs and stays green while the real library is mid-wave.

Run:  uv run scripts/test-check-command-schema.py
"""

import subprocess
import sys
import tempfile
from pathlib import Path

import yaml

CHECKER = Path(__file__).resolve().parent / "check-command-schema.py"

CANONICAL_SLUGS = ("roles", "reserved", "tools", "ways-of-working", "boundaries", "fail-conditions")


def rule(rid, labels, text, cls="must"):
    return {"id": rid, "labels": list(labels), "class": cls, "text": text}


def baseline_schema():
    """A synthetic pair that satisfies every assertion — the positive control."""
    populated = {
        "roles": [rule("demo.lead-owns", ["role"], "The lead owns the run and never produces.")],
        "tools": [rule("demo.read-first", ["binding"], "Read ${target} in full before the first action.")],
        "ways-of-working": [rule("demo.one-question", ["binding"], "Ask one question at a time.")],
        "boundaries": [rule("demo.no-silent-writes", ["binding"], "Never write outside ${target}.", "floor")],
        "fail-conditions": [rule("demo.fail.no-approval", ["fail-condition"],
                                 "Closing without the user's approval fails the run.", "floor")],
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
        "sections": sections,
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

Read `plugins/mochiko/schemas/demo.yaml` in full before your first action. It carries the
run's binding rules, nested in six sections, each addressable by its section ID:
`demo.sec.roles` (who leads) · `demo.sec.reserved` (decisions held by the user) ·
`demo.sec.tools` (tool bindings) · `demo.sec.ways-of-working` · `demo.sec.boundaries` (the
non-waivable floor) · `demo.sec.fail-conditions` (the Not-done set).

## Adaptive Goal Protocol

1. **Entry** — `$ARGUMENTS` names the thing under demonstration.
2. **Goal** — the demonstration closed with the user's approval.
3. **Not done — default FAIL:** the 1 rule labeled `fail-condition` in
   `plugins/mochiko/schemas/demo.yaml` (section `demo.sec.fail-conditions`) — any one
   standing fails the run.
"""

# The Not-done block as the baseline carries it, for probes that move or displace it.
NOT_DONE_BLOCK = """3. **Not done — default FAIL:** the 1 rule labeled `fail-condition` in
   `plugins/mochiko/schemas/demo.yaml` (section `demo.sec.fail-conditions`) — any one
   standing fails the run.
"""

BASELINE_LABELS = {
    "kind": "command-labels",
    "labels": {
        "role": "who holds a seat",
        "binding": "an obligation on the run",
        "fail-condition": "a clause of the Not-done set",
    },
}

BASELINE_PROVENANCE = {
    "kind": "command-provenance",
    "anchors": {"demo.lead-owns": "2026-08-27 demo-session D1"},
}

BASELINE_DECISIONS = "| 2026-08-27 | A synthetic row for demo-session | ruled | [x](y) |\n"


# --- probes: (name, schema mutation, md mutation, expected output substring) ---------------

def drop_section(slug):
    def mutate(s):
        s["sections"] = [n for n in s["sections"] if n["id"] != f"demo.sec.{slug}"]
    return mutate


def add_tombstone(sid, s):
    s.setdefault("tombstones", []).append({"id": sid, "disposition": "retired at the scaffold wave"})


def probes():
    """Each probe breaks exactly one assertion and names the finding it must produce."""
    p = []

    def add(name, expected, schema=None, md=None, clean=False):
        p.append((name, schema, md, expected, clean))

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

    # --- 5. Not-done count-pin and its pluralization (D7 C2 guard, D6-R4) ---
    add("count-pin names the wrong number",
        "pins 3 fail-condition rules, schema carries 1",
        md=lambda m: m.replace("the 1 rule labeled", "the 3 rules labeled"))
    add("count-pin plural where the count is 1",
        "want 'the 1 rule labeled'",
        md=lambda m: m.replace("the 1 rule labeled", "the 1 rules labeled"))

    def two_fail_rules(s):
        s["sections"][-1]["rules"].append(
            rule("demo.fail.no-evidence", ["fail-condition"], "Closing without evidence fails.", "floor"))
    add("count-pin singular where the count is 2",
        "want 'the 2 rules labeled'",
        schema=two_fail_rules,
        md=lambda m: m.replace("the 1 rule labeled", "the 2 rule labeled"))

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
        s["sections"][0]["rules"][0]["text"] = "The lead owns the run, per demo.sec.legacy."
    add("rule text names a tombstoned node",
        "text names tombstoned section demo.sec.legacy",
        schema=text_names_tombstone)

    def text_names_ghost(s):
        s["sections"][0]["rules"][0]["text"] = "The lead owns the run, per demo.sec.ghost."
    add("rule text names a node that never existed",
        "text names section demo.sec.ghost, which is not a node",
        schema=text_names_ghost)

    # --- 8. regression sweep: every pre-existing check still fires ---
    def inline_ruling(s):
        s["sections"][0]["rules"][0]["ruling"] = "2026-08-27 demo-session"
    add("[regression] inline ruling: field",
        "inline `ruling:`", schema=inline_ruling)

    def dangling_anchor(s):
        s["sections"][0]["rules"][0]["id"] = "demo.lead-steers"
    add("[regression] dangling provenance entry",
        "dangling entry 'demo.lead-owns'", schema=dangling_anchor)

    def unknown_label(s):
        s["sections"][0]["rules"][0]["labels"] = ["not-a-real-label"]
    add("[regression] label outside the registry",
        "not in demo-labels.yaml", schema=unknown_label)

    def orphan_var(s):
        s["sections"][0]["rules"][0]["text"] = "The lead owns ${nonexistent}."
    add("[regression] orphan ${var} placeholder",
        "orphan placeholder ${nonexistent}", schema=orphan_var)

    def deixis(s):
        s["sections"][0]["rules"][0]["text"] = "The lead owns the run; these rules bind."
    add("[regression] deictic reference is a warning",
        "deictic reference", schema=deixis, clean=True)

    def duplicate_id(s):
        s["sections"][2]["rules"][0]["id"] = "demo.lead-owns"
    add("[regression] duplicate rule ID",
        "duplicate id", schema=duplicate_id)

    def flat_rules(s):
        s["rules"] = [rule("demo.flat", ["binding"], "A rule at the top level.")]
    add("[regression] flat top-level rules:",
        "flat grammar superseded by sections", schema=flat_rules)

    def bad_section_id(s):
        s["sections"][2]["id"] = "demo-tools"
    add("[regression] malformed section ID",
        "section id fails", schema=bad_section_id)

    def fail_label_off_segment(s):
        s["sections"][-1]["rules"][0]["id"] = "demo.no-approval"
    add("[regression] fail-condition off the .fail. segment",
        "not under the .fail. segment", schema=fail_label_off_segment)

    return p


def run_probe(tmp: Path, schema_mut, md_mut):
    schema = baseline_schema()
    if schema_mut:
        schema_mut(schema)
    md = BASELINE_MD
    if md_mut:
        md = md_mut(md)

    (tmp / "demo.yaml").write_text(yaml.safe_dump(schema, sort_keys=False), encoding="utf-8")
    (tmp / "demo.md").write_text(md, encoding="utf-8")
    (tmp / "demo-labels.yaml").write_text(yaml.safe_dump(BASELINE_LABELS, sort_keys=False), encoding="utf-8")
    (tmp / "provenance.yaml").write_text(yaml.safe_dump(BASELINE_PROVENANCE, sort_keys=False), encoding="utf-8")
    (tmp / "DECISIONS.md").write_text(BASELINE_DECISIONS, encoding="utf-8")

    proc = subprocess.run(
        [sys.executable, str(CHECKER),
         "--schema", str(tmp / "demo.yaml"),
         "--md", str(tmp / "demo.md"),
         "--labels", str(tmp / "demo-labels.yaml"),
         "--provenance", str(tmp / "provenance.yaml"),
         "--decisions", str(tmp / "DECISIONS.md")],
        capture_output=True, text=True,
    )
    return proc.returncode, proc.stdout + proc.stderr


def main() -> int:
    failures = []
    matrix = probes()
    for name, schema_mut, md_mut, expected, clean in matrix:
        with tempfile.TemporaryDirectory() as td:
            code, out = run_probe(Path(td), schema_mut, md_mut)
        problems = []
        if expected not in out:
            problems.append(f"expected {expected!r} in the output")
        if clean and code != 0:
            problems.append(f"expected a clean exit, got {code}")
        if not clean and code == 0:
            problems.append("expected a finding (exit 1), got a clean exit")
        if problems:
            failures.append((name, problems, out))
            print(f"FAIL  {name}")
            for pr in problems:
                print(f"        {pr}")
        else:
            print(f"ok    {name}")

    print(f"\nnegative-test matrix: {len(matrix) - len(failures)}/{len(matrix)} probes passed")
    if failures:
        print("\n--- output of the first failing probe ---")
        print(failures[0][2])
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
