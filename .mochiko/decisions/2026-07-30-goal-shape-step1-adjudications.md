# Goal-shape step-1 audit adjudications — KM reference · run-cost element

**Status:** ruled · **Date:** 2026-07-30

## Context

The step-1 audit (shape v5 + grader revision, v0.33.0) routed two of the author's flagged
deviations to the user: (c) check 1's KM clause — v5 homes the generic KM ritual in the shape,
so a conformant goal-shaped command could stop naming `.mochiko/memory/knowledge-management.md`
and the grader would FAIL it; (d) the shape's run-cost element — bound in 0/6 commands since
shape v3 introduced it, enforced by nothing.

## Decision (user-ruled)

- **KM reference stays mandatory** (recommendation adopted): every KM-carrying command keeps
  naming the project-pinned copy; check 1's clause stands as conservatively encoded. First live
  test: the plan pilot (KM-carrying).
- **The run-cost element is dropped from the shape** (user ruled AGAINST the P17-slotting
  recommendation): retired by explicit supersession-by-ruling strip entry, never by omission.
  Supersedes shape v3's run-cost element (the workflow-token-reduction wave-1 D2 manual-baseline
  carrier at the command layer).

## Rationale

The pinned KM copy is the runtime source commands resolve — dropping the naming would trade a
real obligated read for grader ambiguity. The run-cost element was dead-letter surface: never
bound, never enforced, recorded once in its lifetime; the wave's own measurement rides strip
notes + floor arithmetic, and the token epic's OTel probe remains the future per-run cost path.

## Alternatives considered

- **Slot run-cost as P17 at the step-4 ceremony** (the validator's and lead's recommendation):
  rejected by the user — enforcement would revive a dead letter rather than retire it.
- **Keep run-cost unslotted as-is:** rejected — unenforced shape surface is exactly the class
  this rebuild removes.

**Kept deliberately:** existing `run-costs.md` artifacts remain valid history; the OTel
probe-then-graduate item (token epic) unaffected.
