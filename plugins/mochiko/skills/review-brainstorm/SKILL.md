---
name: review-brainstorm
description: This skill MUST be invoked when serving as a cold END-STAGE REVIEWER of a thinking session's decision record (`record.md`) — paired or solo, never in the room. Protocol: a blind angle map yielding coverage findings, independent cold read, the six hunt classes, then cross-examination; return severity-classified survivors and a status. SHOULD also invoke for the verify pass, a synthesis fidelity sample, or a one-shot cold review. Independent reviewer, never a co-author; defaults to FAIL.
---

# End-Stage Review of a Live Thinking Session

Cold reviewer of a frozen `record.md`. A lens brief shapes the depth of your read; you
recommend, and the ruling is never yours to make.

## Rules — load the schema first

Your first action, before any protocol step: **Read `schema.yaml` (this skill's own
directory) and `../../schemas/skill-review-common.yaml` raw, in full, in the same first
action.** The schema is the source of truth for this skill's binding rules, nested in six
sections, each addressable by its section ID: `review-brainstorm.sec.independence` ·
`review-brainstorm.sec.scope` · `review-brainstorm.sec.inputs` ·
`review-brainstorm.sec.verdict` · `review-brainstorm.sec.output` ·
`review-brainstorm.sec.reserved`. Interpret it live: a rule's `kind:` names what it is, and
an absent `kind:` reads `constraint`; a rule carrying `when:` binds only where its terms
hold against the schema's declared `conditions:`, except that a `class: floor` rule is
always read and always delivered — `when:` gates when its obligation applies, never whether
it reaches you; a `pointer:` rule binds you to that file's or skill's procedure, referenced
never restated; labels come from `plugins/mochiko/schemas/skill-labels.yaml`. A rule
carrying `extends: review-common.<slug>` inherits text/labels/pointer from
`skill-review-common.yaml` only — `class` and every absence-meaningful field are local —
and the stub's `review-brainstorm.*` ID stays the citable ID. The floor pin:
the 9 rules of `class: floor` are non-waivable. Before the first protocol step, state the floor count
back — a skipped or partial read leaves that count blank: halt and surface it, and halt
likewise if the schema's `class: floor` count disagrees with the pin.

## Protocol

Blind angle map first, then the cold read — scenario stress and the six hunt classes per
decision: unchallenged assumption · missing intra-decision dimension · passive acceptance ·
rejected-road steelman · inconsistency · excess machinery. Ground what the record claims,
grade its fitness, then diff your blind map against the record for coverage findings.
Cross-examination follows in a pair; the survivor report closes the pass.

**Verify pass:** the same discipline over folded dispositions instead of a fresh cold
read; a requested `synthesis.md` gets the fidelity sample.
