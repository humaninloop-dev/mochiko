---
name: review-code-minimalism
description: This skill MUST be invoked when independently grading a cycle's produced code against the pre-code ladder — the code-minimalism lens run by the verification seat: read the cycle's git diff AND `cycle-report.md`, grade each rung claim against `mochiko:patterns-code-minimalism` (the standard, never restated here), and emit a `minimalism:` findings block. Rungs 2, 3, and 5 carry a codebase-read obligation. Findings are ADVISORY, never a cycle-failing gate. Scope is the minimalism lens ONLY.
---

# Review — Code Minimalism Lens

**Diff shows what was written; disclosure shows what the builder says they checked;
neither alone shows what should never have been written.**

## Overview

The per-cycle over-engineering lens: an independent, static read of a cycle's produced code
against the pre-code ladder. Cycle diffs are small, context is fresh, rework is cheapest at
cycle close.

## Rules — load the schema first

Your first action, before any procedure step: **Read `schema.yaml` (this skill's own
directory) and `../../schemas/skill-review-common.yaml` raw, in full, in the same first
action.** The schema is the source of truth for this skill's binding rules, nested in six
sections, each addressable by its section ID: `review-code-minimalism.sec.independence` ·
`review-code-minimalism.sec.scope` · `review-code-minimalism.sec.inputs` ·
`review-code-minimalism.sec.verdict` · `review-code-minimalism.sec.output` ·
`review-code-minimalism.sec.reserved`. Interpret it live: a rule's `kind:` names what it
is, and an absent `kind:` reads `constraint`; a `class: floor` rule is always read and
always delivered; a `pointer:` rule binds you to that skill's procedure, referenced never
restated; labels come from `plugins/mochiko/schemas/skill-labels.yaml`. A rule carrying
`extends: review-common.<slug>` inherits text/labels/pointer from
`skill-review-common.yaml` only — `class` and every absence-meaningful field are local —
and the stub's `review-code-minimalism.*` ID stays the citable ID. The floor pin:
the 3 rules of `class: floor` are non-waivable. Before the first procedure step, state the floor
count back — a skipped or partial read leaves that count blank: halt and surface it, and
halt likewise if the schema's `class: floor` count disagrees with the pin.

## Procedure

Locate the diff first: files created/modified per the cycle report locate it; `git diff`
over those paths.

**1. Read the disclosure.** Every decomposition task should carry a rung; the disclosure
surface exists so this lens can grade it — and it also shows whether rung-zero reading
happened at all.

**2. Grade each rung claim against the standard.** Open
`mochiko:patterns-code-minimalism`; per task, ask: does the code sit on the claimed rung,
and does a higher rung apply that the builder descended past?

**3. Verify rung-2/3/5 claims against the codebase.**
- **Rung 2:** targeted greps around the diff for existing helpers/utilities the new code
  duplicates — against the *current* codebase, which also catches cross-cycle accretion
  (cycle 5 duplicating cycle 2's helper reads as a rung-2 violation now).
- **Rung 3:** does the language's standard library already provide the written behavior?
- **Rung 5:** check the dependency manifest — does an installed dependency already cover
  it?

**4. Check the floor line.**

**5. Emit findings** as the schema's output contract shapes them, one line of evidence
each (the grep hit, the stdlib call, the manifest entry).
