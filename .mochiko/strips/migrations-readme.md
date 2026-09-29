# Strip notes — `plugins/mochiko/migrations/README.md`

Entry formats: `strips/README.md`. The migration log's README documents the log's grammar for its
authors. This file opens with the first edit that superseded any of its shipped text.

<!-- Wave context: the joint hook/delta build, wave 3 (v0.116.0) — the README documents grammar 2 as
built: the home document's fields, the eight path tokens with `<run-id>`, `form: entries` with its
fields and the store preamble bound (census row P1), the raw-output home, and the log's grammar
reported as the highest any file declares (O2). The `### Home documents` subsection, its
`#### Grammar 2` paragraph, the `grammar` row's added sentence and the `home` paragraph's pointer
are pure additions and take no entry. Every statement was checked against the crate at the S6 diff
the wave lead released (`crates/mochiko-cli/src/`). Pre-edit verbatim text:
`git show 5558fd7:plugins/mochiko/migrations/README.md`. -->

## [v0.116.0] "the grammar version" — re-keyed to the log's grammar, the highest any applied file declares (O2)

- **Disposition:** superseded →
  - the `migrate status` line of *Working on the log* reads "the state hash, the sequences, the
    log's grammar";
  - the rendering-paths paragraph reads "`replay::load_full`, which also carries the log's grammar,
    the highest any applied file declares".
- **Tier failed:** n/a — supersession by ruling. O2 was raised by S8 and routed to S6 at the wave-3
  plan grading (`build-log.md`, `## 2026-09-29 · S8 plan returned · Q1–Q6 ruled · O2/O3 routed as
  graded addenda`), then folded into S6's passed plan v3 as §2 I7. It is built in
  `crates/mochiko-cli/src/replay.rs:265` (`out.grammar = out.grammar.max(Some(m.grammar))`, doc
  `:98–103`) and printed by `cli.rs:718–720`.
- **Content (superseded text, verbatim):**

  ```
  mochiko-cli migrate status                # the state hash, the last sequence, the grammar version
  Rendering paths call `replay::load` (or `replay::load_full`, which also carries the log's grammar
  version).
  ```

- **Kept deliberately:** the three `migrate` commands and the `load` / `load_full` contract (every
  op applied, and the state passes the hard set). "The last sequence" also leaves: `status` prints
  the applied span (`sequences 1..40 (38 migrations)`), not one number.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` and
  `.claude/rules/mochiko/rust-cli.md` point at "the log's README" for the grammar, and they still
  resolve. `session-start.sh` parses `· grammar N ·` from `migrate status` and is unaffected by
  which file's grammar N is.

## [v0.116.0] Adding-an-op paragraph — "the D5 range `1..1`" and the grammar-bump criterion superseded

- **Disposition:** superseded →
  - "the D5 range is frozen at the first publish — `1..2` in this tree — with whatever ops its
    grammars carry by then";
  - "a grammar bump is reserved for a change an older binary would misread rather than reject: one
    that would make an existing file mean something different, or a new home field, which an older
    binary ignores (*Grammar 2*, above)".
- **Tier failed:** n/a — supersession by ruling:
  - The user approved the wave-1 plan (`build-log.md`, `## 2026-09-29 · joint build opened — seams
    ruled, wave 1 plan approved`). Its §2.8 (`wave1-joint-build.md:106–108`) builds grammar 2 as
    grammar 1 plus the home fields, with the range `(1, 2)`.
  - The home fields execute the hook field review's D2 (per-entry budgets on the cumulative stores)
    and D4 as amended (the run folder). The rows are `DECISIONS.md` 2026-09-23 (field review) and
    2026-09-29 (joint build), pointing to `.mochiko/brainstorms/hook-enforcement-field-review/record.md`.
  - Built: `crates/mochiko-cli/src/migration.rs:21` (`GRAMMAR_RANGE: (u32, u32) = (1, 2)`), `:28–34`
    (`GRAMMAR_2_HOME_FIELDS`) and `:936–949` (a grammar-1 file carrying one is rejected at parse).
    `home.rs:138–143` gives the reason: a home decodes with unknown keys ignored.
- **Content (superseded text, verbatim):**

  ```
  deployed reader that could meet a file it cannot understand: the D5 range `1..1` is frozen at the
  first publish with whatever ops the grammar carries by then, and the first published binary reads
  every one of them.
  … which is why a new op is additive here and a
  grammar bump is reserved for a change that would make an existing file mean something different.
  ```

- **Kept deliberately:**
  - the paragraph's claim that adding an op does not bump the grammar while no binary is published;
  - its `reword-section` and `mint-rule` precedents;
  - the older-binary-rejects-loudly reasoning;
  - "the version contract working, not a gap in it".
  The criterion is widened, not replaced. An existing file changing meaning still bumps. A new home
  field is added because it is the one change an older binary ignores rather than rejects.
- **Consumers assessed:** no primitive restates the range. The crate's doc comments
  (`migration.rs:12–21`, `home.rs:138–143`) state the same criterion and range. The ledger's GI-020
  amendments name no range number.
