# Charter anatomy extends to plan and implement (v0.69.0)

**Status:** ruled
**Date:** 2026-08-13

## Context

At v0.68.0, `pm-role-and-feature-derivation` D10 re-formatted `commands/feature.md` from the v8
Goal · Harness · Bindings anatomy into the six-section charter (Identity & Mission · Adaptive
Goal Protocol · Roles & Responsibilities · Tools · Ways of Working · Boundaries), with the v8
supersession recorded **this command only** and the desk-vs-pipeline-stage distinction on record
at the I7 fold. Working with the desk charter the next day, the user ruled the same anatomy
should carry to the two pipeline commands — `plan.md` already held the posture's seed (lead =
delivery manager of the goal, `plan-structure-yagni` D1, cited by feature.md's own symmetry
note).

## Decision

`commands/plan.md` and `commands/implement.md` re-format to the six-section charter, under four
rulings taken 2026-08-13:

1. **Ruling home** — this ADR + a `DECISIONS.md` row; no brainstorm session (design settled —
   the charter shape shipped at v0.68.0; this extends it).
2. **Behavior-preserving** — every current obligation survives, re-homed into charter sections;
   strips record shape supersession only; the author≠grader audit checks preserved
   responsibilities against the prior text.
3. **Goal protocol maps existing gates — no new ceremony.** Plan: the plan-the-plan proposal
   approval IS the convergence; the approved artifact list is the run's done condition and
   default-FAIL floor; delta scope keeps its collapse. Implement: run-open confirmation (batch,
   scope type, attempt bound at its one redeclaration point, done condition stated); the done
   condition is fixed.
4. **Door open** — `specify`, `brainstorm`, and `setup` stay v8; each converts later on its own
   ruling if wanted. The 3-charter + 3-v8 split is accepted, not a defect.

Consequences:

- D10's "v8 shape superseded **this command only**" clause is **superseded** — the charter now
  covers the three delivery-facing commands; the uniformity cost D10's I7 fold priced is
  re-priced at this ruling.
- The audit criteria for the two commands re-key to **floor present + run goal contract
  present** (the run-shaped analog of feature.md's per-visit contract); the definition lives in
  `.claude/rules/mochiko/primitive-edits.md`. The check must not demand a per-run negotiated
  goal — that would be the new ceremony ruling 3 prohibits — and must not demand
  Goal/Harness/Bindings sections.
- Both charters keep a protocol step literally labeled **Entry**, so `feature.md`'s "the same
  split `/mochiko:plan` and `/mochiko:implement` name at their Entry" and the D8 build rider's
  recorded wording stay true with `feature.md` untouched.
- The first-live-run watch for the charter form rides the existing `plan-structure-yagni`
  dual-probe BACKLOG item; no new watch item.

## Rationale

The Delivery-Manager posture was already half-adopted: plan's lead has been "delivery manager of
the goal" since `plan-structure-yagni` D1, and feature.md's charter cites that symmetry. The
charter makes the posture structural — the always-happens floor carried as owned
responsibilities, the goal contract explicit per run — where v8 carried it as harness bullets.
Extending it keeps the three commands that route, plan, and land delivery speaking one anatomy.

## Alternatives considered

- **Stay v8 for the pipeline commands** — rejected by the user's ruling; the desk charter's
  posture is wanted on the delivery runs.
- **Desk-style per-run goal negotiation** — rejected: plan and implement arrive with the goal
  fixed by the dispatched batch; their existing gates already are the convergence moments.
- **Convert all six commands now** — rejected: the other three have no felt pain; the door is
  left open instead.
- **Full brainstorm session** — rejected: no open design question; the shape is proven at
  v0.68.0 and the conversion is behavior-preserving.
