# Plan run seeds absent product baselines; implement folds surface seeding gaps (v0.66.0)

**Status:** ruled (defect close)
**Date:** 2026-08-11

## Context

Dogfood run (mochiko-app): after one feature's implementation finished and a second feature's
plan run opened, `.mochiko/product/` had never been seeded — no `data-model.md`, no
`contracts/`, none of the five baselines. The D10 bootstrap clause
(`.mochiko/brainstorms/feature-sizing-and-entry-points/record.md`, `Assumed`) says greenfield
baselines "are seeded by the first plan run", and `setup.md` records the same split — but
`plan.md` itself carried no seeding obligation: its Goal asserted the baselines exist ("The
package exists. **Product baselines live at `.mochiko/product/`**") and bound the run to read
them as design input, with no branch for the first run where nothing exists yet. The first
plan run had nothing telling it to seed, so it didn't; the second plan run then opened onto
missing design input.

## Decision

Make the first-plan-run seed an explicit `plan.md` obligation, and give `implement.md`'s fold
a defined empty-baseline behavior:

- `plan.md` Goal: absent baselines at run open → "**this run seeds them before design input
  is read** (Baseline-seed binding)". Not-done gains "absent at close", merged into the
  existing product-baseline clause.
- `plan.md` new **Baseline-seed** binding: `.mochiko/product/` missing or missing a baseline
  file at run open → seed the absent set before design reads it. Greenfield with no delivered
  code: empty scaffolds stating so. Delivered code exists (a prior feature landed without the
  fold, or setup skipped the bootstrap): reconstruct from code and confirm with the user,
  mirroring the `ARCHITECTURE.md` bootstrap. The seed is the baseline write; the feature's
  own design still lands as deltas, folded at acceptance — never merged into the seed.
- `implement.md` Acceptance-landing binding: a delta whose baseline file is absent at fold
  time folds into a fresh `.mochiko/product/` file (three-way diff with an empty pre-fold
  side) and the absence is surfaced as a seeding gap.

User ruling during the fix: no provenance citations in the shipped command text (they create
a dependency on record internals) — the `Assumed`/D10 citations drafted into the Goal and the
binding were removed before audit; provenance lives here and in `DECISIONS.md`. A second
user ruling at landing (2026-08-12): shipped wording tightened for per-run token cost — the
greenfield/brownfield division-of-labor aside and the failure-cause parentheticals live in
this record and `setup.md`, not in the command text.

Pure additions — no strip entries owed. Independent author≠grader audits
(`mochiko:validator`, one per command, grading each against its own text): results recorded
in `DECISIONS.md` row. Commands carry no char budget (excluded per
`.mochiko/memory/primitive-cost-budgets.md`).

## Rationale

The seed's designed home was always the first plan run (D10 bootstrap clause) — the defect is
that the design lived only in the record and `setup.md`'s greenfield aside, not in the command
that has to execute it. A command obligation the command doesn't state doesn't happen.
`implement.md`'s empty-pre-fold clause closes the downstream half: a run that still arrives at
fold time without a baseline (legacy runs, skipped seeds) now has defined behavior plus a
surfaced gap instead of an undefined three-way diff.

The open thread 4 risk (first-plan seeding produces a partial baseline claiming to describe
what the product HAS) is unchanged and stays booked as the `BACKLOG.md`
"Setup baseline-bootstrap hardening" item; the greenfield empty-scaffold path deliberately
states "nothing delivered yet" rather than claiming completeness.
