# Goal-shape rebuild — step-1 build execution + floor checkpoint (rulings A/B)

**Status:** ruled · **Date:** 2026-07-30

## Context

Executing CS-D10 step 1 (shape v5 + grader revision, plugin v0.33.0), the command-architect's
per-command parameter-floor arithmetic — measured by drafting two full v5 skeletons (brainstorm,
plan; plan reproduces at 0.4% error) — came out at **7,713 words** for the six commands, above
the record's ~5–6k projection (+28.6% over the top of the band). Per CS-D10 step 1, a floor
exceeding the projection changes the anatomy or the ambition **before** the pilot: a user
checkpoint. The architect also raised, without taking it, one unruled remedy: Layer 1's
sized-review block (378 w / 2,628 B) binds in only 2 of 6 commands — a genuinely conditional
relocation under CS-D4's own test (nameable skip path: the in-loop-critique branch).

## Decision (user-ruled, A+B)

- **A — ambition re-keyed, anatomy untouched:** the pilot's calibration target is the measured
  **7,713-word floor (−47.5% from today's 14,697)**; the record's ~5–6k projection is
  **superseded** as an unmeasured pre-anatomy estimate (amendment marker at the record's D5
  rationale). Still not pass/fail, per CS-D2′. Landing materially *under* a floor row is as much
  a finding as landing over (under = dropped content; CS-D8 applies).
- **B — sized-review conditional relocation:** the sized-review block moves out of the
  always-read shape into `templates/sized-end-stage-review.md`, loaded only when a workflow
  binds a sized end-stage review. CS-D4's `templates/`-forbidden-destination letter is
  deliberately set aside for this split: the rationale D4 protects (a real nameable skip path)
  is met, and the strip note records the set-aside. Measured effect: shared always-read floor
  +5,387 → **+3,537 B/run**; slice flips net-better; specify's regression halves; brainstorm —
  which binds the review — ends +2,976 B/run (worse than pre-split; logged plainly: the thinnest
  command amortizes neither the anatomy's fixed cost nor the split overhead); six-run total
  −25,699 B.

## Rationale

CS-D2′ made goal-shape conformance the criterion and demoted every percentage to intent, so the
honest measured maximum at ruled fidelity replaces the estimate rather than bending the
anatomy to hit it. The relocation passes the wave's own sham-cut test and repairs most of the
shared-floor growth.

## Alternatives considered

- **C — thin the anatomy's ~440-word fixed scaffolding:** not taken — it reopens CS-D5 on a
  first measurement, before the pilot has tested the anatomy at all.
- **Reject the relocation (keep the block always-read):** not taken — fails D4's own
  conditionality test in reverse; the skip path is real and nameable.
