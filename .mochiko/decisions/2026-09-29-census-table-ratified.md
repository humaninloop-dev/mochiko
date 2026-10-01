# The joint build's census table ratified — three earlier rulings amended in part

**Date:** 2026-09-29
**Status:** ruled (user) 2026-09-29, every row "as recommended", after the review loop's bound
(R1 reviewed, S3 revised once, R1 re-reviewed); **built 2026-09-29 at plugin v0.116.0** (wave 2
phase B, migrations 0032–0035, and the wave-3 preamble bound; branch `joint-hook-delta`); the kinako
rows — items 2–4 below — **built 2026-10-01 at wave 4** (kinako PR #24; item 4 raised 14, K1 having
written the other 3 at `###`: build log, 2026-09-30)
**Driver:** the field review's OQ1 and D2 require the census table to be user-ratified as one table;
the delta record's D2 routes the per-store marker fields to it.

## Context

Seat S3 took the census and wrote the table
(`.mochiko/brainstorms/hook-enforcement-field-review/reports/w2-census-table.md`, with its facts in
`w2-census-facts.md`); a fresh reviewer, R1, graded it (`w2-census-review.md`: 7 blocking and 12
advisory findings, all folded in one revision; its re-review closed them and raised one new blocking
finding, RB1, put to the user here). The table holds 27 choice rows and 11 ruled rows. The user
ruled from a plain-language list of the 27, with the four recommendations that amend an earlier
user ruling asked separately.

## Decision

1. **All 27 choice rows ratified as recommended** — the home sets (0032–0035), the per-entry
   budgets at the 177 floor (kept, though the entry it was measured from counts 83 lines under the
   gate: row M1), the store preamble bound (row P1, crate work in wave 3), the marker rows F1–F4 and
   E1 (carried by the one re-cut of migration 0025), and the archive, views and strips homes.
2. **Field review S11 amended in part** (rows L1, L2): kinako's `features/B53`, `features/B61` and
   `features/FEAT-006/reviews` are kept in place as closed record — neither declared nor moved out.
   Every write into them is refused.
3. **Field review D9(b) amended in part** (row N1a and the ruled N1d; R1's RB1): kinako's four
   FEAT-001 groom snapshots at the archive root and `archive/product-baselines/2026-09-22/
   spine-groom.md` stay in place, editable under the amnesty, rather than sorted into a declared set
   or out of `.mochiko/`. Any new file at the archive root is refused.
4. **Delta D5(iv) amended in part** (row S2, option I): the wave-4 cleanup also raises the 17
   constraint and decision headings written at `####` in kinako's `constraints-and-decisions.md` to
   `###`, two of them (D-025, D-037) outside the moved fold blocks, so every id is its own entry.

## Rationale

- The amendments keep closed records closed rather than moving them: nothing writes those files,
  so a refusal on every write is the right verdict, as for the closed epic directories (delta I9),
  and no move means no link to rewrite.
- Declaring a name pattern for the groom files would reopen the archive root to unchecked files,
  the displacement the field review's D3 closed (a 400-line report was probed through it).
- Raising the headings makes each decision its own entry; kept at `####`, six decisions share one
  169-line entry and the next one written at that level is refused.

## Alternatives considered

- S11 kept as written: delete the three directories at wave 4 (git history keeps them) or move
  them (every destination either fails the report envelope or needs an open archive root).
- D9(b) kept as written: delete the five files at wave 4, or declare a `<slug>-groom-<slug>.md`
  pattern at the archive root.
- D5(iv) kept as written: leave the heading levels, accepting that the next `####` decision denies.
