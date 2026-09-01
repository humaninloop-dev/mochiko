---
name: review-plan-artifacts
description: This skill MUST be invoked to grade the design-phase output package against the sufficiency report's gap list — conformance (every named gap closed, nothing materially past the gap list; material divergence auto-FAILs — BLOCKING) and honesty of disclosed rung claims against `mochiko:patterns-plan-minimalism` (advisory), plus completeness (coverage, measurability, cycle-card quality, consistency) within scope. Emits a 3-state verdict (ready / needs-revision / critical-gaps). Does NOT cover feasibility (`review-feasibility`); defaults to FAIL; run by an independent validator, never the author.
---

# Reviewing Design-Phase Artifacts

Independent completeness grader of the design-phase output package — the mirror-checklist
half of the design-phase review pair. This seat walks fixed checklists over what the caller
supplies and grades what is present, measurable, and consistent; the adversarial
contradiction hunt belongs to its sibling.

## Rules — load the schema first

Your first action at invoke, before any grading step: **Read `schema.yaml` (this skill's own
directory) and `../../schemas/skill-review-common.yaml` raw, in full, in the same first
action.** The schema is the source of truth for this skill's binding rules; this body carries
identity and procedure only. Its rules are nested in six sections, each addressable by its
section ID: `review-plan-artifacts.sec.independence` (author/grader separation) ·
`review-plan-artifacts.sec.scope` (jurisdiction, routing, and what never shrinks the
review) · `review-plan-artifacts.sec.inputs` (pre-asserts, checklist bindings, read duties) ·
`review-plan-artifacts.sec.verdict` (the blocking lenses, the count mapping, the grading
floors) · `review-plan-artifacts.sec.output` (report contracts) ·
`review-plan-artifacts.sec.reserved` (decisions this seat never takes).

Read the rule grammar along with the rules: a rule's `kind:` names what it is, and an absent
`kind:` reads `constraint`; a rule carrying `when:` binds only where its terms hold against
the schema's declared `conditions:`, except that a `class: floor` rule is always read and
always delivered — `when:` gates when its obligation applies, never whether it reaches you.
Where a rule carries `extends: review-common.<slug>`, the stub inherits `text` / `labels` /
`pointer` only from `skill-review-common.yaml` — `class` and `kind` are always this schema's
own, and the stub's `review-plan-artifacts.*` ID stays the citable ID; `${verdict}` in
inherited text substitutes from this schema's `vars:`. Labels come from
`../../schemas/skill-labels.yaml`. A `pointer:` rule binds you to that file's or skill's
content, referenced never restated.

The schema carries **the 11 rules of `class: floor`**. State the floor count back before the
first procedural step; a skipped or partial schema read is a halt-and-surface, never a silent
continue.

## Procedure

Walk the four lenses in order: **conformance** against the sufficiency report's gap list ·
**adopt-first disclosure** over commodity-category decisions · **rung-claim honesty** over
each seat's disclosed ladder stops · **completeness within scope** through the mirror
checklists of [ARTIFACT-CHECKLISTS.md](references/ARTIFACT-CHECKLISTS.md) (analysis · store
delta or the no-delta claim · design · cross-artifact).

Run shape: Tier-1 pre-assert → every applicable checklist over the supplied sets → classify
and shape issues per [ISSUE-TEMPLATES.md](references/ISSUE-TEMPLATES.md) → verdict → report.

In an incremental pass the work narrows rather than repeats: the {new} artifacts get the full
walk, the {prior} artifacts a consistency spot-check — entity names, requirement IDs,
decision references — escalating where the spot-check turns up trouble instead of silently
absorbing it.
