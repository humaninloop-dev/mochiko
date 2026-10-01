# The kinako pass's rulings — four user rulings on K0's gaps, and the lead's readings

**Date:** 2026-09-30
**Status:** ruled 2026-09-30 and 2026-10-01. W1–W7 by the user, each "as recommended". L1–L19 by the lead, as
readings of rulings already made. **Built 2026-10-01** (wave 4 of the joint hook/delta build; kinako
PR #24, branch `mochiko-0.116-cleanup`).
**Driver:** seat K0's read-only inventory of kinako's `.mochiko/` tree (wave 4, step K0). It found
gaps the wave plan and the rulings it executes do not cover. Verification of K0's facts by a fresh
peer (PK0) was running when these rulings were made; a fact PK0 overturns reopens the ruling that
rests on it.

## Context

The wave plan (`.mochiko/brainstorms/hook-enforcement-field-review/wave4-kinako-pass.md`) executes
delta D5 (i)–(v), field review D9(b) as amended, and census ratification items 2–4. K0 recounted
every figure within a recount's tolerance and found no open implement run. It also found:
- the fold-block preambles carry live facts, where the census draft drops them as scaffolding;
- run 4's decision write-ups exist only in a copy D5(iii) deletes;
- three contract files carry fold blocks D5(iv) does not name;
- links to retired paths sit in kinako Rust comments;
- the K4 link scope is larger than the records' counts;
- the op probe shows the ledger `git mv` is the only needed shape the gate refuses.

## Decision

**User rulings (W).**
- **W1 — fold-block notes kept as dated notes.** The preamble facts of each fold block moved at K3
  stay as dated lines in the section their block joins. These are reserved id bands, "`C-003` stays
  reserved", and the run 3 and run 4 id renumbering maps. Only the `## The … landing fold` headings
  go, as D5(iv) literally rules. Amends the census draft's "preamble dropped as scaffolding" in part.
- **W2 — run 4's decision write-ups move into the product.** Before K2 deletes
  `features/FEAT-001/constraints-and-decisions.md`, each D-050…D-061 write-up moves, text unchanged,
  under its entry in the product `constraints-and-decisions.md`. The why lives on the entry (delta
  D3b). A non-author diff read checks each body is present once, unchanged, and within the entry's
  177-line budget. Amends delta D5(iii)'s "content folded" premise in part.
- **W3 — the three EPIC-002 contract fold blocks re-homed too.** These are `engine-port.md:357`,
  `ipc.md:420` and `plugin-bridge.md:1189`. They take the same entry-by-entry move and diff read as
  D5(iv). Extends D5(iv).
- **W4 — code comments left as history pointers.** The links to retired paths in kinako's Rust
  files stay. The user was told 8 links in six files; PK0 recounted 12 lines in 8 files (2 ledger,
  5 copy, 5 evidence). The ruling's ground, code in a records PR, is unchanged. A kinako
  `BACKLOG.md` item books their re-point at the next code change, so the pass stays records-only.
- **W5 — FEAT-006's live content moves into the product before its copies go** (ruled at K2's stop).
  EPIC-001's landing (`ac21ce8`) changed no product baseline. The product's FEAT-006 layer,
  reconstructed 2026-08-22 at index grain, names FEAT-006's copies six times as the home of content it
  omits. W2's move applies to what it names: the rationale of the rows still in force (D-001, D-003,
  D-008…D-014, C-001); IP-001…005 in full; `CorpusLocator`; and corpus-format § 1.1, the locator
  format. Each goes in as its own `###` entry, with L13's mapping and a Dk read. The superseded parts go to
  history at deletion: D-002, D-004…D-007, and v1 corpus-format §§ 1–9. So does the engine-port and
  ipc residue, which the product routes to code. Amends delta D5(iii)'s FEAT-006 premise.
- **W6 — FEAT-006's per-entity sensitivity tables move too** (ruled at K2b's stop). These are the
  retention, access and deviation tables for 8 entities, about 54 lines, which the product lacks. Each
  goes under its product entity as `#### Sensitivity Details`, the shape its FEAT-001 entities already
  use, within K2b and its diff read. Extends W5.
- **W7 — dated reports' path links are handled too** (ruled at K4's plan). L5 keeps a dated report's
  line cites. It does not exempt its links to moved or deleted paths: 222 lines in 73 files at
  `1313c9e`. Ledger links are re-pointed to `archive/ledgers/`. Copy, evidence and fold-heading links
  gain "(in history at `44635f1`)", with the text otherwise unchanged. K5's scan then covers them.

**Lead readings (L).**
- **L1.** §A is applied as current truth. Every count and code line cite in it is re-verified at
  HEAD and written as it stands. For example, the refusal-code set is now 25, after B129 and B137,
  not the 22 recorded at run 4. Where a later landed change supersedes a §A statement, the later
  state wins and the entry notes the run-4 figure: B137's eight marker keys against A4's "six →
  seven". Derived counts that K1 makes stale, such as `contracts/README.md:21`, are updated in the
  same step. A graded text is never applied as a false fact. (Widened at PK0's B4.)
- **L2.** §A3 and §A4 go entry by entry into the sections they extend (D5(i)), never as a new fold
  block. A3's "See § The FEAT-001 landing fold" is re-pointed to where the entry lands.
- **L3.** K3 uses the placed (P) shape, the literal D5(iv) move. `#### C-002 · AMENDED` stays inside
  `C-002`'s entry, giving the ratified 17 raised headings. A `## Declarations` section heading is
  added only as the product home's template names it; if the template names none, K3 stops for a
  ruling.
- **L4.** K4 covers every link class to a moved or deleted path: ledger, feature copy, evidence, and
  removed fold heading. D5(v)'s "every link" already rules this, and K0's larger counts are a
  recount. Bare `baseline-delta.md` citations stay, as `BD-` citations into the archived ledgers.
- **L5.** Line citations into the two product baselines are re-pointed by entry id in live, writable
  files only. A dated report or a closed record cites the file as it stood at its date and is left.
- **L6.** K4 separates citation links, which are re-pointed, from historical instructions, which
  are annotated "in history at `44635f1`" and never re-worded to change their meaning (for example
  `FEAT-001/tasks.md:1045`). The pre-existing dead link at `tasks.md:114` is repaired.
- **L7.** K5's dead-pointer scan covers live, writable files. It excludes the 41 write-refused closed
  records, the epic top-level files (delta I9), the frozen archive snapshots and the archived
  ledgers. `.gitleaksignore:7` is never edited.
- **L8.** The three ledgers move by one user-run `!` script: `git -C <worktree> mv` ×3 with absolute
  paths, keeping rename history. Its path list is graded against K0's move list first. Seats do not
  byte-copy the 301 KB ledger through Write.
- **L9.** Run 4's two spine-diagram additions and FEAT-002's two reciprocal lines are outside §A and
  outside this pass. They stay booked in kinako's `ROADMAP.md` run-4 row.
- **L10.** In kinako's own records: B119 is closed as superseded (field review D3/D4, census row R1),
  and B121 is narrowed to its three open limbs, following kinako's own landing ritual. No step
  touches `brainstorms/index.md`, where the peer worktree `brainstorm-b1` holds uncommitted edits.
- **L11.** Seats keep raw captures in their session scratchpad. Kinako's `.gitignore` carries no
  `.mochiko/runs/` line, and a worktree's own run folder is refused.

**Lead readings on K0's revised inventory (`inventory-v2.md`).** L3's stop fired, and K0 found four
more places where a literal move would write false text.
- **L12 (L3's stop).** The product home binds no template. K3 adds `## Declarations (INT-XXX ·
  DS-XXX)` after the existing sections, named after the feature template's Part 4.
- **L13 (W2's wording).** "Unchanged" yields to L1. The 12 bodies carry 29 package-local ids that
  collide with product ids, 18 short-form feature paths, and 10 design-era code cites, of which 5
  are wrong at HEAD.
  - The ids are mapped by §A2's F-01 id map.
  - The paths are re-pointed as L4 rules.
  - The code cites are re-verified as L1 rules.
  - Each body gains one dated provenance line naming its source (the feature copy at `44635f1`).
  - Each body is its own `###` entry. Nested at `####` under Live rows it is denied at 213 lines.
  - The diff read checks each body against the copy, with only the mapped, re-pointed and
    re-verified tokens differing.
  - The user was told, and may reopen W2.
- **L14.** AppSettings is written as "eleven at run 4", with a dated note that B137 (`b4fc60b`) added
  two fields in code whose rows kinako B138 owes. K1 does not write B138's rows.
- **L15.** §A2's D-057 calls `MarkerRead` private, but it is `pub` (`corpus/settings.rs:1394`). K1
  writes the true fact with a dated correction note, and the decision itself is unchanged.
- **L16.** L5 extends to line cites into the three contracts. Live, writable files are re-pointed by
  section or entry. Dated reports and closed records are left.
- **L17.** K1 writes §A's own links, bare report names and line cites in their final form, so that
  K4's list does not grow. The package-local ids cited in code comments (for example `D-008` at
  `corpus/settings.rs:1380`) join W4's kinako BACKLOG item.
- **L18 (W5's order).**
  - K2 commits without FEAT-006: the ledgers, FEAT-001 and FEAT-002's seven copies, and the 327
    evidence files.
  - K2b then moves W5's content and deletes FEAT-006's five copies in its own commit, before K3 opens.
    K3 then raises headings over a settled file, and K4 knows which copy links are gone.
- **L19 (W6's Corpus row).** Corpus's table (one `rootPath` row) moves too. The product still keeps
  `rootPath` on Corpus as in memory and Confidential (dm `:36`, `:74`, `:132`), and only its
  Attributes table omits it, as unpersisted.

## Rationale

- W1: the notes are current truth (ids that must never be re-minted), not fold history, and D5(iv)'s
  own text removes only the headings.
- W2: under the in-place model a decision's reason lives on its entry, and deleting the only copy
  would leave twelve decisions with no reason in the tree.
- W3: a baseline carrying a fold block reads two-shaped, which is what D5 exists to end.
- W4: comment pointers are cheap to fix at the next code change, and code in a records PR brings
  kinako's code gates into a pass that changes no behaviour.
- W5: W2's ground, applied where the product itself names the copies as the home of what it omits.
- W7: D5(v) says "every link", S11 annotates each evidence pointer, and K5's scan is to come back clean.
- The L readings apply rulings already made, or keep the pass minimal, and never widen what it
  writes.

## Alternatives considered

- W1: keep only the reserved-id notes; or drop every note as the census draft has it.
- W2: delete the copy and point each one-liner at history.
- W3: leave the three blocks and book them.
- W4: edit the comments now and run kinako's Rust gates.
- W5: delete all five and point the product's six lines at history; or keep all five and book them.
- W7: leave dated reports as written, with the scan skipping them; or re-point only the ledger links.
