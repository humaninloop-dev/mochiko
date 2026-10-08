# Wave 3 — the joint build: rule rewords, prose, crate fixes, the governance PATCH, and the bump

**Rulings:** as waves 1 and 2 (`wave1-joint-build.md`, `wave2-joint-census.md`), plus every build-time
ruling in `build-log.md`, the seams records (`.mochiko/decisions/2026-09-29-joint-hook-delta-build-seams.md`,
`…-landed-removal-stub.md`) and the census ratification (`…-census-table-ratified.md`).
**Lead:** the session lead; owns this plan, `build-log.md`, sequence allocation, dispatch, the gates, the release
ceremony and the landing ritual.
**Branch:** `joint-hook-delta` at `5558fd7` (wave 2 committed: migrations through 0035, views at 87).
**This wave carries the joint bump:** `plugin.json` 0.115.0 → **0.116.0** (MINOR — new homes, new rules, a new
grammar), shipped under the AM-5-field-review-fold exception row (seams R4). No `mochiko-cli-v*` tag; the crate stays unpublished
at 0.3.0. Nothing merges to `main` before the bump commit, and the merge itself is the user's call.

## 0. Carried into this wave (from the build log)

Rule text still naming the retired model or unbuilt routes · crate defects found in waves 1–2 · stale eval content ·
prose that restates superseded rules · the ledger's pre-ruled strike PATCH · the release gates.

## 1. Seats and ownership (strictly disjoint write sets)

| Seat | Persona | Owns (write set) |
|---|---|---|
| S6 | `staff-engineer` | `crates/mochiko-cli/src/**` and the crate tests of its own changes (new tests; no pin S9 owns) · `Cargo.toml` / `Cargo.lock` only if a version line must move |
| S7 | `tech-lead` | migrations `0036-*.yaml` … `0041-*.yaml` (rule rewords, one ruling per file, gaps legal; 0040 granted at S7's Q1; 0041 carries seam R11, granted at the user's ruling on S7's second FAIL; 0042 is V1's 0029 fix round; 0043 is V2's 0037 fix round) · `.mochiko/schema-views/**` regenerated after its files land |
| S8 | `tech-lead` | prose primitives: `plugins/mochiko/skills/**/SKILL.md` and `references/**` · `patterns-entity-modeling/scripts/validate-model.py` (the `Entity:` regex; S8's Q2) · `templates/constitution-modules/knowledge-management.md`, `templates/report-format.md`, `commands/architecture.md` (stale lines only; P8's B7) · `plugins/mochiko/hooks/scripts/*.sh` · `plugins/mochiko/migrations/README.md` · `.mochiko/strips/<primitive>.md` for every prose removal or supersession |
| S9 | `staff-engineer` | after S6 and S7 land: crate and eval pins the wave-3 log moves · `evals/**` content made stale by 0024–0035 · the contract suite's gate-workspace `.git` fix and its run |
| S10 | `tech-lead` | `.mochiko/memory/governance-ledger.md` (the pre-ruled strike PATCH, v3.2.1) · the `CLAUDE.md` governance region lines that PATCH moves · `.claude/rules/mochiko/rust-cli.md` build-state pointer |
| Lead | — | `CHANGELOG.md` · `plugin.json` · `marketplace.json` · mochiko's `.gitignore` (`.mochiko/runs/`) · `.mochiko/memory/primitive-cost-budgets.md` (the gate sweep and the rows; S8's Q5) · the landing ritual (`DECISIONS.md`, `BACKLOG.md`, the trail, `ROADMAP.md`, the brainstorms index) · `build-log.md` |
| P6–P10 | fresh peers | plan grades per `mochiko:review-seat-plan`; write nothing |
| G3 | fresh, non-author | code review of S6's crate diff (`rust-cli.md`, GI-004-primitive-audit-ratchet) |
| V1–V3 | fresh plain seats (`opus`) | the gate audit, `mochiko:validation-primitive-edit`, per unit (§6) |

Transport as waves 1–2: lead-relayed, one writer per file, each seat formats only its own files, every write into
`plugins/mochiko/migrations/` and every `cargo` run under a lead slot, no binary install, the branch binary only.

## 2. S6 — crate fixes (grammar 2 is unreleased, so no grammar bump)

1. **P1 (ratified):** text above a store's first heading is bounded at the entry budget (177 today), counted as a
   span of its own, no new home field.
2. **RA3:** `settle` compares a standing fault against the first fault of its key, so a later, larger entry sharing
   a heading makes an unchanged rewrite deny (kinako's `FEAT-002/reports/driver-fix-report.md` is a live case).
3. **P5 A4:** `cli.rs` `shipped_log()` resolves `<repo>/migrations`; two shipped-log tests skip silently. Re-point
   and let them run; a real failure they expose is a stop to the lead.
4. **Deny wording:** a new file under an emptied contracts home names that home as "the nearest home"; a bad run id
   loses its run-key reason under the root nested-docs home; the `evidence/` deny reason (field D9); `home` prints
   "bounds: per template section" for a per-entry home.
5. **G1 R2 (a regression):** `((echo a); tee <home>)` is read as arithmetic and missed — a deny before wave 1's fix
   round. The other wave-1 advisories (R1, R3, R4, R5, A8) go to BACKLOG unless S6's plan shows one is cheap.
G3 reviews the diff; S6's version line moves only if `rust-cli.md` requires it for an unpublished crate.

## 3. S7 — rule rewords (0036–0039, anchored to field-review D-rulings; intents name the seams)

- `impl.artifact-home`: the run log moves to `.mochiko/runs/<run-id>/` (field D6); R1's boundary (a ruling that
  changes a baseline lives on its entry; `.mochiko/decisions/` keeps only rulings that touch none); no `DECISIONS.md`
  row per checkpoint; the withdrawn `implement-log.md` clause.
- The run folder's life: the run-open step creates `<owner>-run<n>` (R5) and states the `.gitignore` line it needs;
  the run-close step — the lead removes the folder after the user's acceptance (R6).
- The other `*.artifact-home` render targets (field F8: `brainstorm.artifact-home` renders `<slug>/record.md`), each
  checked against the wave-1 resolver.
- `testing-end-user`'s scratch path to the run folder, and the pre-write dry run as a rule (field D8).
- How a report cites a reproducing commit for uncommitted work (the "smaller two" — **the user rules at this plan's
  approval**).
- Reads that may move wording: `authoring-technical-requirements.decision-technique-routing`;
  `authoring-architecture-store.store-home`'s spine-budget pointer (R1 A11); the brownfield rule's file name for
  structural D-XXX rows (census N1d); a check that R2's final-validation base lines landed in 0026.
S7 regenerates the views after its files land.

## 4. S8 — prose, hooks, README

- Skill prose restating superseded rules: the store `SKILL.md` lifecycle line, `review-plan-artifacts`'s
  `references/ARTIFACT-CHECKLISTS.md` register rows, the skills on delta D6d's list, and the three prose skills Q8
  names (pointers to `impl.baseline-entry-grammar` for the marker grammar).
- Hooks: `seat-reminder.sh` (`SubagentStart`) gains the run-scratch route; `session-start.sh` adds one reminder
  line: the count of `runs/*` folders present and at most five of their names (OQ5; amended at P8's B5). A hook never blocks; the 5-second floor holds.
- `plugins/mochiko/migrations/README.md` documents grammar 2 (`raw_output`, `<run-id>`, `form: entries`).
- One strip entry per removal or supersession in prose, version-stamped 0.116.0.

## 5. S9 — pins, evals, the contract suite (after S6 and S7 land)

Pins the wave-3 log and crate move (the S5 method) · eval content stale since 0024–0035 (P5 A1, G2 A1–A2) · the
gate-workspace `.git` fix (`evals/contract/run.py:4531`; G1 R5) · the contract suite's deterministic set run
(release gate 6: a SKIPPED suite is not green).

## 6. The gate audit (V1–V3, after every unit lands)

`mochiko:validation-primitive-edit` against each edited unit, held to `.claude/rules/mochiko/primitive-edits.md`:
**V1** schema content 0024–0029 (each migration plus its view diff) · **V2** schema content 0032–0041 · **V3** prose
units (S8's skills, references, hooks, README, with their strips) · **V4** S10's governance PATCH diff, graded by a
fresh seat against its passed plan (E1–E14) and the built binary; the PATCH itself is the user's to approve at the
bump. One fix round and one re-audit per unit; a second FAIL goes to the user.

## 7. S10 — the governance PATCH (pre-ruled at AM-5-field-review-fold)

The strike trigger fires at this bump (ledger GI-019-kernel-tooling-admission build-state line, `:489–494`): strike the build-state line and
the `rust-cli.md` pointer, and correct drift between the AM-5-field-review-fold text and what was built in the same amendment-log row —
at least R7's struck `## Header` limb (`:452`, `:473`), ".md write" where every extension is sniffed (`:472`), and
"What the gate reads" for `Write` (`:478`). PATCH v3.2.1, the v3.0.1–v3.0.3 idiom: mints no principle, no fresh
`/mochiko:setup` amend. S10's plan states the full drift list from a clause-by-clause read against the built binary.

## 8. Lead — the ceremony and the landing

`.gitignore` gains `.mochiko/runs/` · `CHANGELOG.md` 0.116.0 citing the AM-5-field-review-fold exception row (R4) · `plugin.json` and
`marketplace.json` 0.116.0 · the landing ritual: the field-review, delta and seams rows to built, the census and R9
records, the BACKLOG build item to the trail with the new items booked (G1 R1/R3/R4/A8 unless taken, the five 0013
`observable.yaml` ids, the strips-home door, a `withdraw-document` op, the design-truth items to the rehoming
brainstorm), `ROADMAP.md` touched, both index entries. The wave-4 constraint stands: crate 0.3.0 reaches no machine
before the plugin carrying 0032–0035.

## 9. Order and gates

1. S6, S7, S8, S10 plan-only in parallel, graded, GO. S7's migrations land under slots; S7 regenerates views.
2. S9 plans once S6 and S7 have landed; graded; executes (pins, evals, contract suite).
3. G3 on S6's diff; V1–V3 on every unit. Fix rounds bounded.
4. Lead gates (release gates 1–6, GI-012-release-gates-module): audits PASS · strips recorded · landing complete · CHANGELOG ·
   marketplace synced · views ≡ replay · `cargo test --all` plus fmt, clippy, audit, the opt-in similarity test ·
   the contract suite's deterministic set green. Then the bump commit, put to the user; the merge to `main` is theirs.

## 10. Stops

A ruling a migration cannot express · a prose edit that removes protected content without a recorded supersession ·
a crate test failure that is not a moved pin · a SKIPPED contract suite · a gate audit's second FAIL · any kinako
write (wave 4's) · a need to install a binary.
