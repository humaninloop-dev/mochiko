---
name: review-sufficiency
description: This skill MUST be invoked when grading a capability-batch's guidance sufficiency at `/mochiko:implement` entry — the ten-clause check per selected work row, collapsing to a three-clause form per delta card under delta scope, over the spec, the architecture store, and the product baselines, emitting a binding per-row `sufficient` verdict or the gap list that scopes the in-run design phase. SHOULD also invoke on 'sufficiency check', 'enough guidance', 'sufficiency verdict', or 'gap list'. Never reads code, `tasks.md`, `**TEST:**` cases, or cycle reports. Defaults to FAIL — a row is insufficient until every clause is graded; run by a seat that authored none of the graded sources.
---

# Grading Guidance Sufficiency

Binding pre-build gate over one unit of selected work: **does the guidance that already
exists carry enough for a builder to build it?** The check is size-adaptive by construction —
the unit is the map's own unit of scope — and its answer either licenses the build directly
or hands the design phase an exact, named scope. The design phase that closes a gap is a
different seat.

## Rules — load the schema first

Your first action at invoke, before any grading step: **Read `schema.yaml` (this skill's own
directory) and `../../schemas/skill-review-common.yaml` raw, in full, in the same first
action.** The schema is the source of truth for this skill's binding rules; this body carries
identity and procedure only. Its rules are nested in six sections, each addressable by its
section ID: `review-sufficiency.sec.independence` (author/grader separation) ·
`review-sufficiency.sec.scope` (the grading unit and the delta collapse) ·
`review-sufficiency.sec.inputs` (the read fence and its one carve) ·
`review-sufficiency.sec.verdict` (the gate semantics, the ten clauses, the grading floors) ·
`review-sufficiency.sec.output` (the report contract) · `review-sufficiency.sec.reserved`
(what only the user rules).

Read the rule grammar along with the rules: a rule's `kind:` names what it is, and an absent
`kind:` reads `constraint`; a rule carrying `when:` binds only where its terms hold against
the schema's declared `conditions:`, except that a `class: floor` rule is always read and
always delivered — `when:` gates when its obligation applies, never whether it reaches you.
Where a rule carries `extends: review-common.<slug>`, the stub inherits `text` / `labels` /
`pointer` only from `skill-review-common.yaml` — `class` and `kind` are always this schema's
own, and the stub's `review-sufficiency.*` ID stays the citable ID; `${verdict}` in inherited
text substitutes from this schema's `vars:`. Labels come from
`../../schemas/skill-labels.yaml`.

The schema carries **the 8 rules of `class: floor`**. State the floor count back before the
first procedural step; a skipped or partial schema read is a halt-and-surface, never a silent
continue.

## Procedure

Resolve the scope first — selection or delta — because it fixes both the unit and the clause
set. Then, per unit: read the fenced source set whole, walk every applicable clause in order
(testable criteria → contract exposure → data exposure → structural trigger → NFR targets →
commodity exposure → dependency order → UX trace → delivered-feature exposure → in-flight
exposure), and record what each clause yields — a hold, a gap in that clause's own gap form,
or a justified n/a.

Close by assembling the report: per-unit verdicts, the clause-keyed gap list, the
store-consult result, and everything routed onward — trips, in-flight conflicts, `[MODIFY]`
amendment namings. The design phase, cards, and build all key off this report, never off the
conversation.
