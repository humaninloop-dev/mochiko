# Wave 2 — the joint build: the census, the homes, and the delta-files rule supersessions

**Rulings:** as wave 1 (`wave1-joint-build.md` header), plus every build-time ruling in `build-log.md`.
**Build surface:** this record § Build surface 2 · the delta record § Build surface 1 · the seams record's
routed items (seams 2–5 and the smaller one).
**Lead:** the session lead; owns this plan, `build-log.md`, sequence allocation, dispatch, and the gates.
**Floors in force:** as wave 1.
**Branch:** `joint-hook-delta` at `91b3838` (wave 1 committed: crate 0.3.0, grammar 1..2, migrations through 0023).
**No `plugin.json` bump in this wave.** The bump, the gate audit of every schema unit from waves 2 and 3, the
contract suite and the CHANGELOG entry all land at the wave-3 close. Nothing merges to `main` before then.

## 0. What this wave decides and what it only writes

The census table is the one artifact in this wave the user ratifies. Everything else is written against rulings
already made. The order is therefore: facts first, then the table, then its review, then the user's ruling, and
only then the home migrations. The delta-files rule supersessions do not depend on the table's numbers, so they
run in parallel from the start.

## 1. Seats and ownership (strictly disjoint write sets)

| Seat | Persona (tier) | Owns (write set) |
|---|---|---|
| S3 | `tech-lead` (default `opus`) | phase A: `reports/w2-census-facts.md` · `reports/w2-census-table.md`; phase B, after ratification: `plugins/mochiko/migrations/0032-*.yaml` … `0035-*.yaml` (home documents and the templates that carry entry grammar) · `.mochiko/schema-views/**` (regenerated at the wave close, never hand-edited) |
| S4 | `tech-lead` (default `opus`) | `reports/w2-delta-sweep.md` · `plugins/mochiko/migrations/0024-*.yaml` … `0031-*.yaml` (the delta-files rule supersessions and rewords, split one decision per file) · no strip entries: rules and templates are schema content, recorded by the log (`primitive-edits.md`; corrected at S4's Q1, `build-log.md`) |
| S5 | `staff-engineer` (default `opus`) | `crates/mochiko-cli/tests/fidelity.rs` and any crate test pin the new migrations move · `evals/contract/**` and `evals/plan/**` only where a pinned rule id or census figure moves (field-scoped; `evals/plan/` added at S4's re-plan, A8) |
| R1 | fresh `tech-lead` peer | reads only; writes `reports/w2-census-review.md` |
| P3–P5 | fresh peers of each producer's persona | plan grades per `mochiko:review-seat-plan`; write nothing |

Reports sit in this session's `reports/` directory, follow the report envelope (`mochiko-cli template
report-envelope`), and are dry-run through `check` before each write. Transport: lead-relayed, no mesh, one writer
per file, each seat formats only its own files. No seat installs a binary: seats run the branch's
`target/debug/mochiko-cli` (built by `cargo build`), never the 0.2.0 on `PATH`, for anything that reads the new
grammar.

**Order.** S3 and S4 plan-only in parallel, graded, GO. S4 executes throughout. S3 executes phase A. R1 reviews the
table; the lead puts it to the user. On the user's ruling S3 executes phase B. S5 plans once S3 and S4 have
landed, then executes. The lead runs the gates.

## 2. S3 phase A — the census facts and the table

**2a. Facts (`w2-census-facts.md`).** Three enumerations, each with its source cited:
1. **The write sets the primitives will have after the delta rewrites** (the smaller seam): every path a rule,
   template or command in the log tells a seat to write under `.mochiko/`, read from the migration log through the
   branch binary (`rules`, `template`, `home`), with the delta record's D4 withdrawals applied (feature, epic and
   lane homes lose `baseline-delta.md`, `data-model.md`, `constraints-and-decisions.md`, `contracts/`, and the
   epic's shared-baseline copies; the epic's `implement-log.md` goes, per the field review's D6).
2. **Both trees:** every directory and file pattern under `.mochiko/` in this repo and in kinako
   (`~/Documents/GitHub/kinako`), marked declared, undeclared-but-legitimate, or legacy (the field review's S11
   list), with counts.
3. **The cumulative stores as they will stand after the delta record's D5 cleanup** (seam 3): for kinako's
   `data-model.md`, `constraints-and-decisions.md` and `quickstart.md`, each entry's size once the two
   `## The … landing fold` blocks and run 4's fold text (`reports/landing-fold-text-run4-2026-09-23.md` §A) are
   merged into the entries they extend. Report the heading level that marks an entry in each file, the largest
   entry (the record's 177 lines is the floor to beat), the distribution, and the text outside entries (the
   summary tables, the Relationships table, `## Validation rules`, and the dated reconciliation sections in
   `contracts/ipc.md`). This is a measurement, not a rewrite: nothing in kinako is changed.

**2b. The table (`w2-census-table.md`).** One ratifiable artifact. Every row carries a recommendation and its
reason, and every choice the user makes is a row:
- the declared file set of every home, the new `archive/`, `runs/` (raw-output, R5 run-id), `schema-views/`,
  `strips/`, `decisions/`, `memory/` and any other directory 2a found, the archived-ledger shape under
  `archive/` included (seam 2), and each sub-directory said to be allowed by location where it has no home
  document;
- the feature, epic and lane home sets after the delta record's D4;
- per cumulative store: the entry heading, `entry_max_lines` (sized so the largest honest post-D5 entry fits,
  never below it), `section_max_lines` for text outside entries or the reason there is none (seam 3), and whether
  `Lifecycle:`, `Raised:` and `Weighed:` lines are exempt (`entry_exempt_fields`, seam 4);
- `quickstart.md` and `design/design.md`: their entry grammar, or the reason they keep a whole-file or section
  bound (seam 5); the design truth part's writer is left to the rehoming brainstorm and the row says so;
- the `Lifecycle:` field's placement and the build-raised fields' placement per store (delta D2/D3b);
- `contracts/*` stay outside the size scope (delta D6a), stated;
- a "fold shape" row is not needed: the delta record's D1 answered it (no fold).
The table states, for each existing kinako and mochiko file, whether the new rules would deny a future write to
it, so the user sees the consequence before ruling.

## 3. S4 — the delta-files rule supersessions

**3a. The sweep first (`w2-delta-sweep.md`).** A full-text sweep of the migration log for "delta", "fold",
"appliable", "in place", "in-flight" and "FEAT-XXX". Every hit is classified: on the delta record's D6d list,
missing from it but in scope, or untouched (D6d's own untouched list: `impl.landing-delta`,
`impl.delta-reverification`, `impl.gates-fold`, `authoring-feature-map.delivered-sticky-rows-fold`,
`patterns-sound-loop.no-delta-card-exemption`, `review-sufficiency.clause9-delta-inapplicable`). A hit that D6d
missed and that is in scope is a stop to the lead: the list was ruled `Assumed` complete until this sweep, and the
lead rules each addition.

**3b. The migrations, one decision per file (0024–0031, gaps legal), anchored `2026-09-24
delta-files-vs-direct-baseline-edits D<n>`:**
- D1/D7: `impl.baselines-never-in-place` and `authoring-architecture-store.sign-off-is-write-gate` superseded by
  new-id floors (in place with a marker; the sign-off flips `proposed` to `in-flight`); `impl.fail.baseline-in-place`
  to the unmarked write; `impl.fail.ungraded-fold` to the unreviewed diff; `impl.graded-fold` to the pinned-base diff
  review.
- D2: `impl.baseline-delta-grammar` to the entry grammar with the marker; `authoring-architecture-store`
  `lifecycle-statuses` gaining `proposed` and the key set (`FEAT-XXX`, `EPIC-XXX`, `lane-<slug>`), `orphan-rule`,
  `fold-duty` as the landing flip over every baseline, and the spine template's status legend and check.
- D3: the pinned base at run-open and at sign-off, the duplicate-id check, the unmarked-write test and its
  exclusions (verify N1), **plus seam R2**: the run-open base in `sufficiency-report.md` and both bases in the
  final-validation report.
- D4/D6b: the rules D6d names on design outputs, epics and `review-plan-artifacts`' register rows.
- D6d's remaining sites as listed, and each sweep addition the lead rules in.
Protected content (floors, fails, anchored rules) leaves only by `supersede-rule`. Every file is stamped with
`mochiko-cli migrate stamp` and validated with the branch binary (0 rejecting). No strip entries: schema content is
recorded by the log itself (corrected at S4's Q1; the supersede dispositions and each file's intent carry the record).

**3c. Not in S4's set:** the home documents (S3), the field review's rule rewords (`impl.artifact-home`, the render
targets, `testing-end-user`, R1's boundary line — wave 3), and skill prose outside the log (wave 3).

## 4. R1 — the review of the table

A fresh `tech-lead` peer who wrote nothing in this wave checks the table against its obligations, one line each:
every directory in 2a has a row · every home's set matches the post-delta write sets · every store budget admits
its largest post-D5 entry · seams 2–5 each have a row · `contracts/*` scope stated · no row decides something the
records already ruled · the consequence column is present and right for a sample of ten files it checks itself.
Findings go in `reports/w2-census-review.md`. The lead folds or rejects each finding, then puts the table to the
user.

## 5. S5 — the pins

After S3 phase B and S4 land: `fidelity.rs`'s sequence list and census figures move by the exact supersede and mint
counts (read from the landed migrations, never assumed); any other crate pin the new log moves; contract-suite
pins only where a rule id or figure they name moves. The contract suite's gate-workspace `.git` fix is wave 3's,
not this wave's.

## 6. Wave-2 gates (lead)

`cargo test --all` · `cargo fmt --all --check` · `cargo clippy --all-targets -- -D warnings` · `cargo audit --deny
warnings` · `target/debug/mochiko-cli migrate validate --report --plugin-root plugins/mochiko` 0 rejecting · views
≡ replay (S3 regenerates, the lead diffs a fresh emit). The session's hooks read the installed plugin copy's log
(`${CLAUDE_PLUGIN_ROOT}`), not this working tree, so the grammar-2 migrations do not reach the running gate; but the
0.2.0 on `PATH` halts on any command pointed at `plugins/mochiko`, by design, so every such command uses the branch
binary.

## 7. Stops (halt to the user or the lead)

A sweep hit D6d missed (to the lead) · a store whose largest honest entry no single budget can fit — the split into
per-entry files is the recorded fallback, and choosing it is the user's · a migration that cannot express a ruled
change under grammar 2 · a census row that would deny an existing file's next honest write with no amnesty route ·
any edit to kinako in this wave · a need to install a binary.
