---
report: verification
feature: hook-enforced-artifact-schema
round: 5
wave: 5
scope: the mochiko violator pass — re-home half only; the size half rewrites nothing by ruling
verdict: mochiko is clean on both non-relaxable measures — path 0, set 0
files_moved: 83
pointer_lines_repointed: 111
plan: wave5-plan.md
---

## What the pass did

| class | move | files |
|---|---|---|
| 1 | `wave<n>-reports/` → the session's `reports/`, renamed `wave<n>-<name>` | 51 |
| 2a | the 13 wave plans → their session root as `wave<n>-<slug>.md` | 13 |
| 2b | `wave0-fixtures/` → `research/wave0-fixtures/` | 5 |
| 2c | session working files → the session's `research/` | 11 |
| 3 | `review.md`, `review-lens-a.md`, `review-lens-b.md` → the session's `reports/` | 3 |
| 4 | pointer repoint across records, indexes, reports, DECISIONS, strips, rules and the suite | 111 lines in 74 files |

Every `wave<n>-reports/` and `wave0-fixtures/` directory is gone. Nine `research/` directories and
one `reports/` directory were created. 49 files were stamped with a `report:` type during the move
— 22 `cycle`, 24 `review`, 1 `verification`, following the convention the four reports already in
`hook-enforced-artifact-schema/reports/` had set. Bodies were not otherwise touched, verified by
diffing a stamped file against its blob at HEAD.

## Inventory

Each row is one synthetic `Write` per existing file under `.mochiko/**` plus the root docs, with
`cwd` pointed at an empty directory so no baseline exists and the first-touch amnesty never fires.
Script: `…/scratchpad/wave5/inventory.py`.

| stage | files | deny | path | set | shape | size |
|---|---|---|---|---|---|---|
| before the pass | 474 | 143 | 69 | 14 | 2 | 58 |
| after class 1 | 475 | 95 | 18 | 14 | 3 | 60 |
| after class 2 | 475 | 67 | 0 | 3 | 3 | 61 |
| after classes 3 and 4 | 475 | 65 | **0** | **0** | 2 | 63 |

The one added file is `wave5-plan.md`, written after the first run. Path and file set are the two
measures with no amnesty, so their reaching zero is the pass's whole purpose: every remaining deny
is on a file that stays editable when the gate goes live.

## The amnestied residue

| kind | files | disposition |
|---|---|---|
| `record.md` / `synthesis.md` whole-file | 44 | the pending C1 amendment makes these unbounded; nothing to do either way |
| `wave<n>-<slug>.md` whole-file | 13 | 4 would still exceed 300 under the amendment |
| report section over the envelope's 15 | 5 | re-sectionable under `extra_headings: allow`; left |
| `build-log.md` whole-file | 1 | unclassified by the three proposed rows — needs naming in the ruling |
| missing required heading | 2 | `.mochiko/memory/governance-intent.md`, `.mochiko/product/architecture/spine.md` |

Per the lead's ruling the size half rewrites nothing. Three of these surfaced only because the pass
removed the path deny that had been masking them, so they are disclosures, not regressions.

## Left as history

A path a reader is meant to follow was repointed. A path recording what the tree looked like then
was not. Left deliberately:

- `wave3-census-raw.md` — 128 lines, the raw census of the pre-pass tree. The whole file is a
  then-state record; repointing it would destroy the measurement it exists to hold.
- `build-log.md` — 13 mentions. Dated entries, and the lead's own file; single writer.
- `wave5-plan.md` — 3 mentions, this pass's own move table and pointer counts.
- Eight individual lines across six reports where a finding states the then-location, for example
  *"`wave2-reports/` is not a declared sub-directory"* and *"Under `wave6-reports/`, P3's files
  are …"*. Repointing these would make a true finding read as false.

One class was repointed that could be argued either way, and it is disclosed here rather than
buried: instructions inside approved plans, of the form *"the report goes to `wave4-reports/…`"*.
A reader following them wants the report, which now lives elsewhere, so they were repointed.

## Failure narrative

My first pointer pass had two bugs, both caught before the class was reported and both fixed.
Replacing needle by needle let replacements chain: five sessions each own a `run-costs.md`, so one
`<session>/run-costs.md` reference collected all five session prefixes in turn. The same pass also
dropped the session segment for files that had sat directly under a session directory, turning
`command-content-schema/implement-rewrite.md` into `research/implement-rewrite.md`. Both landed in
`index.md` only. The corrected pass resolves a single regex alternation through a lookup table, so
inserted text is never rescanned, and it is idempotent — a second run changes nothing.

The first pass also reached `.claude/worktrees/cheap-subagents/`, a stale worktree outside this
pass. Both index files were restored from their committed blobs, read through `git show` and
written back, and the worktree is clean again. The corrected pass excludes that tree.

## Notes of note

**The placeholder finding was fixed while the pass ran.** I reported that the check flagged `<n>`
inside a frontmatter value quoting the pattern `wave<n>-<slug>.md`. Commit `c56ca1c` makes the
check skip backticked code spans in frontmatter values and headings, and the deny is gone. One
consequence for these figures: the after-class-1 and after-class-2 rows were measured against the
older binary, so only the first and last rows are strictly comparable.

**A move is a fresh write, which is why this pass had to run before the upgrade.** None of the 83
moved files carried a baseline at its new path. Under the gate each would have had to arrive fully
conforming — frontmatter, headings and budgets. Pre-gate they land as-is and are amnestied from
then on. 49 of them would have been denied on frontmatter alone.

**Still open for kinako:** the 161 evidence files with no legal location, `B61`, and
`EPIC-001/landing/`. The lead has ruled the latter two by analogy; the first needs the user.
