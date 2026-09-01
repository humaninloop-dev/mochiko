---
name: review-specifications
description: This skill MUST be invoked when reviewing an already-drafted specification for gaps — missing requirements, ambiguities, unstated assumptions, and missing edge cases — including its feature layer and the Screens & Flows of a UX-bearing spec. Reach for it on 'review spec', 'find gaps', 'what's missing', or 'is the spec complete'. Produces gap-finding INPUT, not a clearing PASS/FAIL verdict. For enriching a sparse feature idea before a spec exists, use mochiko:analysis-iterative instead.
---

# Reviewing Specifications

Gap-finder over a drafted spec — severity-bucketed gaps plus clarifying questions a
stakeholder can answer. The seat hunts what is missing, ambiguous, assumed, or contradictory
in a spec that already exists; enriching a sparse idea before a spec exists is a different
skill's work.

## Rules — load the schema first

Your first action at invoke, before any hunting: **Read `schema.yaml` (this skill's own
directory) and `../../schemas/skill-review-common.yaml` raw, in full, in the same first
action.** The schema is the source of truth for this skill's binding rules; this body carries
identity and procedure only. Its rules are nested in six sections, each addressable by its
section ID: `review-specifications.sec.independence` (author/grader separation) ·
`review-specifications.sec.scope` (jurisdiction, routing, and what never gets added) ·
`review-specifications.sec.inputs` (coverage duties, baselines, external claims) ·
`review-specifications.sec.verdict` (the hunt taxonomy, the check sets, the severity
grammar) · `review-specifications.sec.output` (question craft and report contracts) ·
`review-specifications.sec.reserved` (the clearing this seat never issues).

Read the rule grammar along with the rules: a rule's `kind:` names what it is, and an absent
`kind:` reads `constraint`; a rule carrying `when:` binds only where its terms hold against
the schema's declared `conditions:`, except that a `class: floor` rule is always read and
always delivered — `when:` gates when its obligation applies, never whether it reaches you.
Where a rule carries `extends: review-common.<slug>`, the stub inherits `text` / `labels` /
`pointer` only from `skill-review-common.yaml` — `class` and `kind` are always this schema's
own, and the stub's `review-specifications.*` ID stays the citable ID. Labels come from
`../../schemas/skill-labels.yaml`. A `pointer:` rule binds you to that file's or skill's
content, referenced never restated.

The schema carries **the 8 rules of `class: floor`**. State the floor count back before the
first procedural step; a skipped or partial schema read is a halt-and-surface, never a silent
continue.

## Procedure

Hunt the user-facing categories for the six requirement-defect classes: **missing
requirements** (mentioned-not-specified · implicit expectations · dependencies on undefined
behavior) · **ambiguities** (unquantified terms · open interpretation · unclear limits) ·
**edge cases** (empty states · cancelled mid-flow · missing permissions · unstated limits) ·
**assumption gaps** (assumptions that should be requirements, and the reverse · hidden
dependencies) · **contradictions** (conflicting requirements · inconsistent terminology ·
mutually exclusive acceptance criteria) · **excess / unpaid scope** (no user need or ratified
driver pays for it).

Then widen to the spec's other layers where they exist: the feature layer (the map checks,
graded from the git baseline) and the Screens & Flows of a UX-bearing spec (walk the
prototype as a skeptic, then run the check sets). Bucket findings by severity, shape the
questions as decisions, and land everything in the report shapes the schema binds.
