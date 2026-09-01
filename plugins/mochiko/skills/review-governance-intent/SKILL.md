---
name: review-governance-intent
description: This skill MUST be invoked when serving as a cold INTENT REVIEWER in a `/mochiko:setup` run — stress-testing the frozen interrogation synthesis (`.mochiko/memory/governance-intent.md`) BEFORE the user ratifies it, spawned at the sizing gate, never a participant in the session. SHOULD also invoke for the verify pass over folded dispositions or the bounded delta-pass on a material post-review edit. Run by an independent reviewer, never the session lead; defaults to a FAIL posture.
---

# Intent Review — Stress-Testing the Governance Synthesis

Cold reviewer of the frozen `governance-intent.md` — fact profile, floor-expression and
deck rulings, minted intents, waivers, modules, exclusions, each with a GI-ID and a
lead-assigned confidence mark (`Confident / Assumed / Contested / Unsure / Deferred`) — a
**traceable contract** on the surface-set producer. A lens brief may scope you to
*coverage* (agenda surface: missed dimensions, convergence-skip audits, card-acceptance +
waiver/module sweeps) or *coherence* (fact↔risk↔ruling alignment, mark/echo-rationale
audit, reality-conflict resolutions against the analysis, cross-element contradictions).

## Rules — load the schema first

Your first action, before any protocol step: **Read `schema.yaml` (this skill's own
directory) and `../../schemas/skill-review-common.yaml` raw, in full, in the same first
action.** The schema is the source of truth for this skill's binding rules, nested in six
sections, each addressable by its section ID:
`review-governance-intent.sec.independence` · `review-governance-intent.sec.scope` ·
`review-governance-intent.sec.inputs` · `review-governance-intent.sec.verdict` ·
`review-governance-intent.sec.output` · `review-governance-intent.sec.reserved`. Interpret
it live: a rule's `kind:` names what it is, and an absent `kind:` reads `constraint`; a
rule carrying `when:` binds only where its terms hold against the schema's declared
`conditions:`, except that a `class: floor` rule is always read and always delivered —
`when:` gates when its obligation applies, never whether it reaches you; a `pointer:` rule
binds you to that file's or skill's procedure, referenced never restated; `${var}`
substitutes from this schema's `vars:` at read time; labels come from
`plugins/mochiko/schemas/skill-labels.yaml`. A rule carrying
`extends: review-common.<slug>` inherits text/labels/pointer from
`skill-review-common.yaml` only — `class` and every absence-meaningful field are local —
and the stub's `review-governance-intent.*` ID stays the citable ID. The floor pin:
the 16 rules of `class: floor` are non-waivable. Before the first protocol step, state the floor
count back — a skipped or partial read leaves that count blank: halt and surface it, and
halt likewise if the schema's `class: floor` count disagrees with the pin.

## Protocol

Form the attack sequestered, then read the frozen synthesis, the agenda, and — brownfield —
the codebase analysis. Work every hunt class the lens brief admits, the over-governance
hunt included; shape each finding to the contract; cross-examine in a pair; close with the
survivor report and the recommended status.
