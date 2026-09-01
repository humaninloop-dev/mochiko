---
name: review-feasibility
description: This skill MUST be invoked to grade design-phase analysis/design artifacts for cross-artifact FEASIBILITY — hunting contradictions, impossibilities, buildability conflicts, plus unjustified structure / wrong altitude; plus the architecture pass when the design-phase package carries an architecture-store delta. Emits a 3-state `feasible / needs-revision / infeasible` verdict. The adversarial half of the design-phase review pair; its sibling `review-plan-artifacts` grades coverage/measurability/presence, this grades contradiction/buildability. Never defaults to `feasible`; not the constitution.
---

# Reviewing Feasibility

Adversarial cross-artifact review: **can these artifacts be built together?** Hunt the
impossible combination no single artifact reveals — judgment, never a checklist; looking
buildable is not being buildable.

## Rules — load the schema first

Your first action, before any hunting: **Read `schema.yaml` (this skill's own directory),
`../../schemas/skill-review-common.yaml`, and `references/FEASIBILITY-LENS.md` raw, in
full, in the same declared first action** — schema, then common, then lens. The schema is
the source of truth for this skill's binding rules, nested in six sections, each
addressable by its section ID: `review-feasibility.sec.independence` ·
`review-feasibility.sec.scope` · `review-feasibility.sec.inputs` ·
`review-feasibility.sec.verdict` · `review-feasibility.sec.output` ·
`review-feasibility.sec.reserved`. Interpret it live: a rule's `kind:` names what it is,
and an absent `kind:` reads `constraint`; a rule carrying `when:` binds only where its
terms hold against the schema's declared `conditions:`, except that a `class: floor` rule
is always read and always delivered — `when:` gates when its obligation applies, never
whether it reaches you; a `pointer:` rule binds you to that file's or skill's procedure,
referenced never restated; `${var}` substitutes from this schema's `vars:` at read time;
labels come from `plugins/mochiko/schemas/skill-labels.yaml`. A rule carrying
`extends: review-common.<slug>` inherits text/labels/pointer from
`skill-review-common.yaml` only — `class` and every absence-meaningful field are local —
and the stub's `review-feasibility.*` ID stays the citable ID. The floor pin:
the 9 rules of `class: floor` are non-waivable. Before the first hunting step, state the floor count
back — a skipped or partial read leaves that count blank: halt and surface it, and halt
likewise if the schema's `class: floor` count disagrees with the pin.

## The hunt

Classes 1–6, class 7 (excess / wrong altitude, remove-shaped), and the architecture pass
A1–A3 all live in the lens file, worked examples and reviewer guardrails included. Hunt
each class across the package, then fill the report and hand the verdict up.
