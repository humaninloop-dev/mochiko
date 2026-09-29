# A run's own sweep edits outside every entry are not unmarked writes — joint-build seam R11

**Date:** 2026-09-29
**Status:** ruled (user) 2026-09-29, "rule it now", at the second plan FAIL of seat S7 in wave 3 of the
joint build (branch `joint-hook-delta`); **built 2026-09-29 at plugin v0.116.0** (wave 3, migration 0041)
**Driver:** seat S8's plan observation O3, routed to S7; P7's re-review of S7's v3 (N2b) named it a
reconciliation of two user-ruled decisions, the same kind as the seams record's R8
([2026-09-29-joint-hook-delta-build-seams.md](2026-09-29-joint-hook-delta-build-seams.md)).

## Context

The delta record's D3 test (migration 0026, `impl.fail.unmarked-baseline-write`) fails any baseline
hunk that sits outside every entry marked for the run's key. The same record's D2 has the run sweep
the rows that list its entries — an entity summary table, an index row, a count, a relationship
table, a validation-rule list. Those rows live outside every entry, so a run that adds entity
`Invoice` (marked `**Lifecycle:** proposed (FEAT-007-run1)`) and its `Invoice` row in the Entity
Summary table fails D3's test on the row D2 told it to write. Neither ruling said which wins.

## Decision

**R11 —** A hunk is excused from D3's unmarked test only when all four hold:

1. **Where the file is:** it is one of the baselines whose sweep rows D2 names: `data-model.md`,
   `constraints-and-decisions.md`, `quickstart.md`, and the architecture store's `spine.md` and
   `concerns.md`. Every other baseline file keeps D3's test unchanged.
2. **Where the line is:** it is outside every entry. That means above the file's first entry
   heading, or after a heading of fewer `#` than the file's entry level and before the next entry
   heading. The entry level is the one `mochiko-cli home <file>` prints. A `####` sub-heading
   inside an entry is still inside that entry.
3. **What the line says:** each changed line names the id of an entry that the same diff marks for
   the run's key, or it is a count that a recount at the diff's head confirms.
4. **Who gets it:** only the run's own sweep. A line inside an entry is always that entry's
   amendment and needs the entry's marker, and naming a marked id never excuses it.

What leaves D3's test: sweep hunks meeting all four, which fail today. Everything else that fails
today still fails, including an edit to another entry's summary row, such as deleting it, which
names no id the diff marks.

## Rationale

D2's sweep rows are a direct consequence of marked entries, so the id test ties each excused line
to a marker in the same diff. A stray edit to a summary row still fails, because it names no marked
id, and a changed count must recount true. Limiting the excuse to the named files and to lines
outside every entry keeps D3's test whole inside entries, where the marker belongs.

## Alternatives considered

- Defer: ship 0.116.0 with the false failure and fix it after its own ruling. The first run that
  edits a summary table, such as kinako's wave-4 run, would hit it.
- Narrow D3's domain to lines inside entries, with lines outside every entry never checked. Simpler
  to apply, but a stray summary-row edit goes uncaught.
