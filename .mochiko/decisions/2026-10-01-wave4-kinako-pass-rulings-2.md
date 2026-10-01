# The kinako pass's rulings, continued — bare contract names are names, not links

**Date:** 2026-10-01
**Status:** ruled 2026-10-01. W8 and W9 by the user, each "as recommended"; L20 by the lead. It continues
`2026-09-30-wave4-kinako-pass-rulings.md` (W1–W7, L1–L19), which is at its 150-line bound.
**Built 2026-10-01** (wave 4 of the joint hook/delta build; kinako PR #24, branch `mochiko-0.116-cleanup`).
**Driver:** peer grader PK4's second grade of seat K4's link plan (step K4, delta D5(v)). PK4's
independent census found 312 mentions of the deleted contract files' names in 67 files that K4's
tables did not cover.

## Context

K4 re-points or annotates every link to a path this pass moved or deleted (D5(v), L4, W7). PK4
searched for the five deleted contract names in relative form (`contracts/engine-port.md`,
`plugin-bridge.md`, `ipc.md`, `harness-store.md`, `corpus-format.md`) and found 312 uncovered hits.
The lead's check of kinako at `1313c9e` sorted them:
- 28 are markdown hyperlinks that resolve to a missing file. K4's tables already cover 27; one,
  `features/FEAT-001/reports/records-groom-review-2026-09-22.md:295`, is missing.
- The rest are names in backticks, mostly in traceability and gap-coverage table cells, for example
  "T-12 → `contracts/ipc.md`". The product keeps contracts of exactly these names, which now hold the
  folded content. 19 of the mentions are in the product itself, where they already name live files.

## Decision

- **W8 — bare contract names stay; the one missed hyperlink is fixed.** A backtick name such as
  `contracts/ipc.md` is a name, not a link. It now reads as the product contract of that name, just as
  a bare `baseline-delta.md` reads as the archived ledger (L4). K4 adds a row for
  `records-groom-review-2026-09-22.md:295`, the one dead hyperlink its tables missed. K5's
  dead-pointer scan checks resolvable links (markdown link targets and path-qualified paths), not
  bare names. Extends L4 and W7.
- **W9 — `docs/quickstart.md`'s four contract links repaired** (ruled at PK4's third check). Lines
  19–21 link `contracts/{plugin-bridge,ipc,engine-port,corpus-format}.md` relative to `docs/`, which
  has never had a `contracts/` folder in any commit. The pass did not break them, and `docs/` is
  outside K5's scan. They are repaired anyway, re-pointed to `../.mochiko/product/contracts/<name>.md`
  as L6 repaired `FEAT-001/tasks.md:114`, within K4's live sub-steps.

**Lead reading (L).**
- **L20 (L5's "dated report", at D5's diff read).** `features/FEAT-002/sufficiency-report.md` is a
  dated report by substance: frontmatter `report: review`, round 1, 2026-09-10. Its 30 line cites
  stay as written, under L5. Its path links are still handled, under W7. K4's own "dated by
  substance" reading already says this; its line-cite table contradicted it.

## Rationale

- Each bare name still picks out a real file: the product contract holding what the feature copy
  held.
- Re-pointing or annotating about 311 table cells in 67 files would add a full grade-and-read cycle
  for no reader gain. Annotating would also be wrong for the product's own 19 mentions, which name
  live files.
- The rule matches the one already ruled for bare ledger names (L4), so one principle covers both.
- W9: four links in the user-facing quickstart cost one file and three lines, the same kind of
  repair L6 already made.

## Alternatives considered

- Re-point every bare name to its full product path, annotating the ones that describe past build
  steps.
- Append "(in history at `44635f1`)" to every bare name.
- W9: book the quickstart links in kinako's `BACKLOG.md` for later.
