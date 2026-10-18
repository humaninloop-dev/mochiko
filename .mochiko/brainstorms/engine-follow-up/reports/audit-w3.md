---
report: review
round: 1
grader: w3-gate-grader
tier: opus
wave: 3
units: [1, 2, 3, 4, 5, 6, 7]
---

## Shared pre-pass

This seat ran every command below itself, from the repo root, on `engine-follow-up-build` at HEAD
`437b654`, with the wave's writes uncommitted. Brief hash checked first: `6998a5ed…881a65`, 113
lines, matching. Frozen state at start: 27 porcelain lines, identical to `scratchpad/lead/w3-frozen.txt`;
`git diff | shasum -a 256` = `cc95f22a2244b14c79ff8aff62d43e83c4871788c157f746109f72c39136eb9d`;
the diff without the build log = `0d4f6b99d3cb496ae8da1584f92604883fc2ffdf2edf19250384d34f1990296f`.
At the end of grading the diff without the build log still hashed `0d4f6b99…1990296f`, the 27 lines
were unchanged, and the four untracked migrations kept their hashes (`0101` = `f991b04f…`, as the
build log records). Only the build log moved (full-diff hash `6c30e3cc…`). This seat wrote only
under its scratchpad and this report.

Contract: `mochiko-cli rules validation-primitive-edit`, the preamble and all six sections
(`sec.independence` · `scope` · `inputs` · `verdict` · `output` · `reserved`), rendered first-hand
with `--plugin-root plugins/mochiko`. Every block carried its opening line (`… · binary 0.4.0 ·
grammar 2 · plugin 0.120.0`) and its end line. Floor read-back: `class: floor · 11 rules` —
`author-grader` · `plain-seat-explicit-tier` · `rendered-contract-only` · `from-file-floor` ·
`pre-pass-first-hand` · `binary-verdict` · `default-fail` · `tamper-proof-clause` ·
`gate-loop-bound` · `evidence-floor` · `second-fail-user`. Path rules read whole:
`.claude/rules/mochiko/primitive-edits.md`, `.claude/rules/mochiko/rust-cli.md`. Binary:
`mochiko-cli 0.4.0 · grammar 1..2`, byte-identical to `target/release/mochiko-cli`; no crate source
changed since `437b654`.

Baseline: `git archive 437b654 plugins/mochiko scripts/similar-rules-allowlist.yaml`, extracted to
the scratchpad and replayed from there (so the pre-wave allowlist was used).

- `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` (exit 0):
  `mochiko-cli migrate validate · 0 rejecting · 118 advisory` · `rules scanned: 1266 · in-kind pairs
  scored: 219979 · clusters: 0 (none)` · `allowlist-suppressed edges: 185` · `pointer resolution: 89
  checked`. Pre-wave: `0 rejecting · 118 advisory`, `1260 · 217663 · clusters: 0`, 185. The diff of
  the two reports is the four command `budget` lines (architecture `64 rules · 16612` → `66 · 18156`,
  implement `131 · 47861` → `131 · 48332`, setup `42 · 9662` → `44 · 11298`, specify `71 · 14696` →
  `73 · 16986`) and the scan line. Every `enforces-coverage` line is unchanged: no floor or gate
  rule was added. With an empty allowlist both logs report `clusters: 76`, the listings identical.
- `mochiko-cli migrate status --plugin-root plugins/mochiko`: `sequences 1..101 (89 migrations)` ·
  `state sha256:300e26a8e1d17bf7de880cda571824f678307ec01c10197be3debdbe16908bef · 94 documents ·
  1266 rules`. Sequences 0096–0097 are an unused gap.
- Views ≡ replay: `mochiko-cli views emit · 94 documents · <scratch>/views`, then `diff -r
  .mochiko/schema-views <scratch>/views` printed nothing.
- Char budgets: no budgeted class is touched. The four migrations name only `command/*` schemas, no
  skill schema, agent or command `.md` changed, and the one `SKILL.md` edited is the unbudgeted
  router (ledger, *Unbudgeted primitives*). Canonical snippet on the router: body 48,995 → 49,956,
  description 206 → 206. Command renders, seven blocks, characters: specify 27,160 → 29,636 · setup
  18,002 → 19,821 · architecture 28,000 → 29,718 · implement 66,907 → 67,376 · feature 24,641
  unchanged. P2's call that no budgeted primitive is touched holds.
- `cargo test --all --no-fail-fast`: 765 passed, 0 failed. `MOCHIKO_FULL_SIMILAR=1 cargo test
  --test matrix_similar`: 48 passed in 251.38s, so the sweep ran. `cargo fmt --all --check` exit 0;
  `cargo clippy --all-targets -- -D warnings` exit 0. Target dir: the default `target/`, in the tree.
- Kit checks, `PYTHONDONTWRITEBYTECODE=1 uv run evals/run.py command check-rubric <kit>` and
  `check-fixtures <kit>`, all exit 0: specify `59 observable, 14 out-of-instrument, 73 total` ·
  setup `36, 8, 44` · architecture `60, 6, 66` · implement `75, 56, 131` · feature `57, 4, 61`;
  `fixture consistency: OK` ×5.
- Contract host run, `python3 -B evals/contract/run.py --host-only`, on a scratch mirror
  (`plugins/`, `scripts/`, `evals/contract/`, crate source, manifests and `target/release/mochiko-cli`
  copied with their times kept, so `evals/.work/` landed in the scratchpad): `contract suite: 7/7
  cases passed` · `FILTERED — 7 of 99 declared cases. Not a gate run.` Floor sets matched: specify
  19 · setup 21 · architecture 25 · implement 40. `rec largest render — implement · impl.sec.tools —
  28,174 chars / 28,443 bytes, 93.9% of the ceiling`.

## Unit 1 — 0098 and the specify view diff

VALIDATE: schema content. `plugins/mochiko/migrations/0098-specify-asking-line.yaml` with `git
diff 437b654 -- .mochiko/schema-views/commands/specify.yaml`.

Checklist run: `validation-primitive-edit.judgment-items-schema`, the AM-2 five. Every id that
`spec.ruling-places` names was checked live. Each place's way and writer setting was held against the
re-sort's Specify row. The reword was read against default 23 (RB13 with QR6) and the lead's R2.

Evidence read: the migration, read whole · the specify view diff · specify's seven blocks rendered
pre and post and diffed · record D4 (`:499–532`), D10 with its changed-at-review line
(`:1144–1200`), the re-sort's Specify row (`:1221–1227`) and count (`:1241–1245`), defaults 16 and 18
(`:969–1009`), QR6 (`:1343–1366`), RB13 (`:1447–1451`) · wave plan §3.0–§3.1 · build log W3 P1
entries (R2, A1).

Pre-pass: the shared pre-pass above. `0098` carries no exit op.

- **Intent stated — PASS.** The header names D10 (QR3), D4, the Specify row, defaults 16, 18 and 23
  (RB13, QR6), and each op. Its claim that `spec.gate-selection` keeps its text, class, kind and
  anchor holds: the rule is byte-identical pre and post (`class: floor · kind: gate`, anchor
  `pm-role-and-feature-derivation D5`).
- **Anchor present where the exit requires one — PASS.** There is no exit. The header anchor is
  `2026-10-15 engine-follow-up`, and both mints carry `… D10`. The reword keeps its anchor
  `2026-10-10 specify-brainstorm-discovery`.
- **ID lifecycle right — PASS.** Two fresh mints; `mint-once` and `id-duplicate` are silent. The
  reword keeps its id, class (`must`) and anchor. All 12 rule ids `spec.ruling-places` cites are
  live: `asking-line` · `confirm-frame-post-stories` · `stress-test-one-pass` · `reserved-to-user` ·
  `filter-disagreement-escalates` · `filter-rejections-recorded` · `story-stage` · `gate-selection` ·
  `selection-card` · `gate-acceptance` · `staged-derivation` · `lockstep-prototyping`. The reword's
  cites `analysis-iterative.inputs-in-hand` and the writer side `feat.carry-forward` are live too.
- **Ways and writer settings against the Specify row — PASS, 7 of 7.** Framing on an empty
  argument: E, an open probe, off. Clarification: E, a fact question, held by the slate's contract
  (`spec.story-stage`: the requirements-analyst's pen under the slate). Filter disagreement and a
  rejected `in` row: E, held. Post-stories frame adjust: E, on — the PM's map derivation writes
  from it, and `spec.stress-test-one-pass` (which grades the derivation and the map delta) checks
  it. Selection: E, held, moved off "stays a gate" by the row alone — `spec.gate-selection`'s
  `kind: gate` is a blocking checkpoint, its text names no yes / no form, and `spec.selection-card`
  makes it a pick with a recommendation. Acceptance: G, held. Prototype clicking: outside the line.
- **Floor and fail survival — PASS.** Pins unchanged: `kind: fail · 10` and `class: floor · 19`.
  The `floors:` index line is byte-identical pre and post. The render diff is the two additions and
  the one reword.
- **Register — PASS.** The stub mirrors `feat.asking-line` and `impl.asking-line` (must, in
  ways-of-working). `spec.ruling-places` mirrors them too (must, `binding`, `[user-gate]`, in
  reserved), and `${pm_seat}` resolves to `product-manager`. The reword keeps every clause of the
  old text and adds the hand-in: the desk sitting's `derivation.md`, its visit goal and the user's
  words so far, "inputs in hand from the intent stage on" by `analysis-iterative.inputs-in-hand`
  ("never re-asked … a skip skips the forks, never the close"). That is default 23 as RB13 changed
  it (no visit file, the existing `derivation.md`), and it reaches the intent stage as R2 ruled. The
  intent confirm stands. Default 16 holds: no rule turns intent's contract on. The literal
  `.mochiko/features/desk/<date-slug>/` path matches `feat.artifact-home`'s own spelling; specify
  declares no `features_dir` var.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 1 (schema content · 0098 + specify view diff, ruling places checked live) · w3-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 2 — 0099 and the setup view diff

VALIDATE: schema content. `plugins/mochiko/migrations/0099-setup-asking-line.yaml` with `git diff
437b654 -- .mochiko/schema-views/commands/setup.yaml`.

Checklist run: the AM-2 five. Every id that `setup.ruling-places` names was checked live and held
against the re-sort's Setup row. Defaults 15 and 16 were checked as *held*, with no contract turned
on.

Evidence read: the migration, read whole · the setup view diff · setup's seven blocks rendered pre
and post and diffed · the re-sort's Setup row (`:1228–1232`) · defaults 15–17 (`:993–996`) · D10
(`:1144–1200`) · wave plan §3.2.

Pre-pass: the shared pre-pass above. `0099` carries no exit op and no reword.

- **Intent stated — PASS.** The header names D10 (QR3), D4, the Setup row, defaults 15, 17 and 18,
  and states the claims that hold: no place set on, defaults 15 and 16 read as held, and the
  interrogation left on its stage through `setup.interrogation-inline` and `setup.thinking-stage`.
- **Anchor present where the exit requires one — PASS.** There is no exit. The header anchor is
  `2026-10-15 engine-follow-up`, and both mints carry `… D10`.
- **ID lifecycle right — PASS.** Two fresh mints and no reword. All 10 cited ids are live:
  `asking-line` · `user-conflict-rulings` · `fail.unclosed-trace` · `user-mode-ruling` ·
  `coverage-survivor-routing` · `gate-loop-bound` · `gate-synthesis-ratification` ·
  `gate-final-acceptance` · `validate-seat-form` · `user-card-rulings`.
- **Ways and writer settings against the Setup row — PASS, 6 of 6.** Mode: E, off. Conflicts: E,
  held by the trace (`setup.fail.unclosed-trace`). Coverage routing: E, off. A second FAIL at the
  gate loop: E, off. Ratification and final acceptance: G, held by that trace and the validate
  seat's grade.
- **Defaults 15 and 16 — PASS.** No setup place is set on, so `contract-pen` and `contract-inside`
  stay off by `common.asking-line`'s own terms. `setup.thinking-stage` and
  `setup.interrogation-inline` are byte-identical pre and post. The holders default 15 names (the
  ratification gate, the trace fail, the validate seat, the flagged-proposal acceptance) are the
  ones the rule cites.
- **Floor and fail survival — PASS.** Pins unchanged: `kind: fail · 6` and `class: floor · 21`. The
  `floors:` line is identical, and the render diff is the two additions.
- **Register — PASS.** It mirrors the W1/W2 shape. The interrogation's card, module and waiver
  rulings are named as its own, outside the list, as §3.2 says.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 2 (schema content · 0099 + setup view diff, ruling places checked live) · w3-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 3 — 0100 and the architecture view diff

VALIDATE: schema content. `plugins/mochiko/migrations/0100-desk-asking-line.yaml` with `git diff
437b654 -- .mochiko/schema-views/commands/architecture.yaml`.

Checklist run: the AM-2 five. Every id that `arch.ruling-places` names was checked live and held
against the re-sort's Architecture-desk row. R1 (the shelf-scope override, writer on) was read
against the row and D10's *on*.

Evidence read: the migration, read whole · the architecture view diff · architecture's seven blocks
rendered pre and post and diffed (`arch.user-reserved-rulings`, `arch.shelf-scope-source`,
`arch.seat-tech-lead-grader`, `arch.floor-precedence`, `arch.dispatch-setup-waiver`,
`arch.dm-drift-dispatch`, `arch.dm-route-triggers` read in full) · the re-sort's desk row
(`:1238–1240`) · D10 (`:1144–1200`) · the first D4 table, T18–T20 (`:957–959`) · wave plan §3.3 ·
build log R1.

Pre-pass: the shared pre-pass above. `0100` carries no exit op and no reword.

- **Intent stated — PASS.** The header names D10 (QR3), D4, the desk row and default 18. It also
  states that the row stances, baseline confirmation and amendments stay in the focus stage, as the
  re-sort says.
- **Anchor present where the exit requires one — PASS.** There is no exit. Both mints carry `… D10`.
- **ID lifecycle right — PASS.** Two fresh mints. All 8 cited ids are live: `asking-line` ·
  `dm-drift-dispatch` · `seat-tech-lead-grader` · `shelf-scope-source` · `dm-route-triggers` ·
  `floor-precedence` · `dispatch-setup-waiver`, plus `${grader_seat}`, which resolves to
  `tech-lead`.
- **Ways and writer settings against the desk row — PASS, 4 of 4.** Drift dispositions: E, on,
  with tech-lead's grade. Shelf-scope override: E, on. A trigger ruled closed: E, off. The floor
  waiver: G, held by the ledger through `/mochiko:setup`.
- **R1 — faithful.** The row reads "shelf-scope override: E · on" and names no grader. The rule
  puts the override under *Writer on*, so `contract-pen` and `contract-inside` hold, and it lands
  as the exact text of the store write to the `Scope:` line (`arch.shelf-scope-source`: "an ordinary
  store write to that line"). No grader is invented. `arch.seat-tech-lead-grader` covers the
  architect's judgment writes and treats transcription as the landing audit's, so the exact-text
  landing is the only check, as R1 rules. Default 19 holds: no slot was added.
- **Floor and fail survival — PASS.** Pins unchanged: `kind: fail · 2` and `class: floor · 25`. The
  `floors:` line is identical.
- **Register — PASS.** It mirrors the W1/W2 shape. `arch.user-reserved-rulings` keeps its
  reservation untouched; the new binding sets the writer switches only.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 3 (schema content · 0100 + architecture view diff, ruling places checked live) · w3-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 4 — 0101 and the implement view diff

VALIDATE: schema content. `plugins/mochiko/migrations/0101-implement-residuals.yaml` with `git diff
437b654 -- .mochiko/schema-views/commands/implement.yaml`.

Checklist run: the AM-2 five, plus the narrowed `impl.feature-visit-input`, the `gates.md`-fold
home, A4's intent sentence and the tools line.

Evidence read: the migration, read whole · the implement view diff (three hunks, the three rewords
only) · implement's seven blocks rendered pre and post and diffed · `impl.gates-fold`,
`impl.beyond-spec-disposition`, `impl.baseline-diff-review` in full · `0026`'s header and its
`impl.graded-fold` supersession, `0001`'s original "graded folds" text · the consumers'
"keyed to the run" lines (`impl.design-inputs`, `impl.design-stage-inputs`,
`impl.gap-finding-blind-dispatch`, `testing-gap-finding.blindness-fence-inclusion-list`, the
`review-sufficiency` view's `:72`) · RB3 (`:1380–1383`), D10 · wave plan §3.4 · W2 audit unit 2's
advisory.

Pre-pass: the shared pre-pass above, plus the tools measure.

- **Intent stated — PASS.** It names default 12 (RB3, S10) and D10's re-sort line. It gives each
  reword's provenance: `0094`'s mint, and the no-anchor rule last reworded under
  `delta-files-vs-direct-baseline-edits` D3, which matches `0026`'s anchor.
- **Anchor present where the exit requires one — PASS.** There is no exit. `impl.ruling-places`
  keeps `… D10`, `impl.feature-visit-input` keeps `2026-10-15 engine-follow-up`, and
  `impl.landing-verifier-folds` stays unanchored (must class, no protected exit).
- **ID lifecycle right — PASS.** Three rewords. Ids, classes and kinds are unchanged, and the
  header lines are identical pre and post.
- **Narrowed `impl.feature-visit-input` — PASS.** "any … keyed to the run's FEAT" becomes the file
  the run's own entry points at: a selected work row's pointer, each member's on an epic, the delta
  card's marked work row, the lane's file. That is the same read the `review-sufficiency` view
  makes at its `:72`. "In every scope … whatever the gap count" is kept. The four consumer cites
  ("keyed to the run (impl.feature-visit-input)") stay true and untouched.
- **`gates.md` fold, one home — PASS.** `impl.landing-verifier-folds` now names "the `gates.md` fold
  of impl.gates-fold, each finding ruled fix-now or backlog folded with its ruling's meaning kept
  and none the user accepted as designed". That matches `impl.gates-fold`'s exclusion and D10's *on*
  check. `impl.ruling-places` points at it ("checking the `gates.md` fold
  (impl.landing-verifier-folds)"), as a pure insertion. Its 22 cited ids are all still live.
- **A4's intent sentence — PASS.** It is true. `0001`'s rule read "checks the graded folds", and
  `0026` superseded `impl.graded-fold` (the baseline delta fold) with the pinned-base diff review
  and the flip. The `gates.md` fold of gap findings is a different object, so nothing retired is
  restored.
- **Tools line — PASS.** `mochiko-cli rules --section impl.sec.tools … implement`: 28,443 bytes by
  `wc -c` (28,174 characters), 50 headings, byte-identical to the pre-wave render. That is 557 under
  the 29,000-byte line, under the 30,000-character `!` ceiling, with no reworded rule in tools.
- **Floor and fail survival — PASS.** Pins unchanged: `kind: fail · 17` and `class: floor · 40`. The
  `floors:` line is identical. The flip and lane clauses of `impl.landing-verifier-folds` survive
  verbatim.
- **Register — PASS.** Short insertions, no mechanic restated. `${features_dir}` is kept.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 4 (schema content · 0101 + implement view diff, tools line measured) · w3-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 5 — the specify, setup and architecture command pairs, and implement's 0101 delta

VALIDATE: command pairs. `plugins/mochiko/commands/specify.md`, `setup.md` and `architecture.md`,
each with its full render, plus `implement.md` against its 0101 render delta. All four `.md`s are
unchanged since `437b654` (`git diff --stat` empty; none touched by the build since `c840469`).

Checklist run: `validation-primitive-edit.judgment-items-pair` with the canonical-scaffold criteria,
plus the asking line's cited ids, lines made false, and A1's four unlisted places.

Evidence read: the four `.md`s, read whole · the seven blocks of each command rendered pre and post
and diffed · all five `*.asking-line` renders hashed (identical) · `common.asking-line` in the common
view · D10's changed-at-review subset (`:1182–1192`) · the A1 rules (`spec.missing-map-surfaced`,
`spec.governance-region-absent`, `setup.stress-test-cold-seat`, `arch.diagram-signoff`) in full ·
the re-sort (`:1202–1245`) · build log A1.

Pre-pass: the shared pre-pass above. The contract host suite's `converted-shape` case passed.

- **Scaffold headings and order — PASS.** All four `.md`s are unchanged. Each carries the canonical
  frontmatter and headings, and the Rules block enumerates the six sections set-wise.
- **Asking line's engine ids — PASS.** The three new stubs render the same text as
  `feat.asking-line` and `impl.asking-line` (identical hash). All 11 cited engine ids are live:
  `fact-before-fork` · `one-fork-per-turn` · `options-rule` · `question-form` · `mock-in-message` ·
  `own-turn-or-batch` · `ratified-streak` · `vague-answer` · `user-rules`, plus `contract-pen` and
  `contract-inside`. That is D10's named subset exactly.
- **Floor survival / no floor or fail retired or lowered — PASS.** All four pins are unchanged
  (10/19 · 6/21 · 2/25 · 17/40), and all four `floors:` lines are identical. The only rule-text
  changes are six must mints and four must rewords.
- **Lines made false — PASS; P2's "unchanged" call holds.** Specify: Entry's "Empty → ask the user
  for the description" is the writer-off open probe, and Goal's "the user's selection" and "the user
  accepted the whole" match the held pick and the held gate. Setup: "propose the mode from what the
  workspace shows" holds (asked when ambiguous), and ratification and acceptance stay gates.
  Architecture: "one drift disposition — converges in a single exchange" is about the visit goal;
  the disposition is then asked on the asking line, so it is not falsified. Implement: Entry names no
  visit file, and Goal's "acceptance landing executed whole" holds with the verifier's added check.
- **Preserved responsibilities / independence — PASS.** No seat grades its own row: the
  stress-test, tech-lead and the landing verifier grade other seats' writes. Reserved content sits
  in `*.sec.reserved`.
- **Done-condition branch — PASS.** Specify and setup are fixed run contracts. The desk is
  per-visit. Implement keeps its existing run-open and acceptance gates; no new ceremony.
- **A1's four unlisted places — faithful.** The confirmed re-sort is 39 rows, and its count closes
  exactly across W1–W3: on 10 · held 13 · off 15 · unchanged 1. Specify 7, setup 6 and the desk 4
  give W3's 17 (on 3 · held 8 · off 5 · unchanged 1). Adding rows would go past the user's
  confirmed table. Each `*.ruling-places` opens with "The run's rulings outside its … take the
  asking line", and the stub binds "Every ruling this command asks of the user outside its thinking
  stage". So the four are reached with `contract-pen`/`contract-inside` off, the same E · off that
  their re-sort analog (implement's absent-surface offers) carries. The lead's one-line note to the
  user at the bump carries the gap to its owner.

VERDICT: PASS

Issues requiring fix: none.

Advisory, not blocking: A1 closes only when the bump note reaches the user. The four places stay
unsorted rows until then.

audit: unit 5 (command pairs · specify, setup, architecture + implement 0101 delta) · w3-gate-grader · opus · 4 files · 1 rounds · 0 blocking

## Unit 6 — router, strips, ripple, budget ledger, changelog

VALIDATE: prose primitive and ripple. The router `plugins/mochiko/skills/mochiko/SKILL.md`; strips
`.mochiko/strips/specify.md`, `implement.md`, `mochiko.md`; `ARCHITECTURE.md`; `README.md` and
`GLOSSARY.md` (untouched); `.mochiko/memory/primitive-cost-budgets.md` (unchanged); `CHANGELOG.md`
`## [0.121.0]`, its W3 part.

Checklist run: `validation-primitive-edit.judgment-items-prose` (coherence, preserved
responsibilities), each insertion and fix against its source rule, the default-21 rule as the lead
applied it, and `mochiko-cli ids --check`.

Evidence read: every diff from `437b654` · a mechanical prefix/suffix test of the router's five
changed lines · source rules `review-sufficiency.user-added-regrade`, `arch.tools-system-design`,
`DIAGRAM-CONVENTIONS.md` "Taste — every diagram", `review-plan-artifacts.meaning-kept`,
`arch.feature-delta-visit`, `arch.gate-focus-confirm`, `impl.design-stage-unit`,
`impl.baselines-in-place-marked`, `impl.baseline-entry-grammar`, `impl.baseline-diff-review`, the
`feature` and `feature-desk` homes · `BACKLOG.md:43–65` · `git diff --stat c840469` over the ripple,
agents, templates and skills · the store spine (scaffold only, so `ARCHITECTURE.md` stays
hand-maintained).

Pre-pass: router body 48,995 → 49,956, description 206 unchanged, unbudgeted.

- **Router R1–R5 — PASS.** All five are pure insertions, confirmed mechanically; nothing is removed.
  R1 (`:90`) matches `user-added-regrade`: graded on the clause set, each gap marked user-added (L5),
  new behaviour never a gap. R2 (`:95`) matches `arch.tools-system-design` and the taste section.
  R3 (`:98`) matches `meaning-kept`: a ruled item changed without a ruling blocks, and the result
  fills the checkpoint's landing table. R4 (`:142`) matches `arch.feature-delta-visit` (it writes
  nothing to the store) and `arch.gate-focus-confirm` (focus stage on delta visits), with
  `impl.design-inputs` reading it. R5 (`:162`) matches `impl.design-stage-unit`.
- **Strips — PASS.** Three wave-context comments; no entry is owed (schema content, or pure
  addition). Their claims check out: pins `17` and `10`, the router line numbers
  (`:90/:95/:98/:117/:142/:143/:162`), anchors, and joined slugs. `ids --check` shows 0 drift, and
  `D3-product-sequence-ids`, `D6-outside-need-asked` and `D5-visit-file-input` match the record
  headings.
- **`ARCHITECTURE.md` A1–A4, L1, L2 — PASS.** A1 (the design node: in-place baseline entries and
  store elements, `design-stage.md`, signed `architecture.md`) matches the feature home and
  `impl.baselines-in-place-marked`. A2 adds the design stage and card order to the user's
  touchpoints (`impl.card-order`). A3 replaces the false `:301` with in-place build-raised entries
  graded from the pinned-base diff (`impl.baseline-diff-review`); "too" is dropped (L6). L1 makes
  `:306–308` read the landing flip. A4 adds `design-stage.md`, the signed drawing and the desk
  sittings (`feature-desk` home, W1's own ripple, per L8). L2 makes `:390–393` read in place and
  flipped from the pinned-base diff, sourced as ruled. No claim goes past its source rule.
- **README / GLOSSARY — PASS.** `README.md` is untouched by the build, so `:106` and `:108` (false,
  pre-existing) stay for the narrowed item and are listed in the CHANGELOG (L3). `GLOSSARY.md` was
  touched at W1, but neither BACKLOG item names it, and its **visit file** line still holds after
  the narrowing. No residual is owed. `devils-advocate.md` and `templates/report-format.md` are
  untouched, so they are left, as default 21 says.
- **Ledger and CHANGELOG — PASS.** No budgeted primitive moved, so no ledger row is owed. The W3
  part's figures reproduce first-hand: 1,260 → 1,266, the four pins, the four render figures, tools
  28,443, and router 48,995 → 49,956. `[lead: …]` slots are present as expected.
- **`ids --check` over the six touched files** — `2 bare · 0 drift`, at `strips/implement.md:41` (D5)
  and `:43` (D6). **Ruling: default 21 does not reach them.** They sit in W2's own comment,
  committed at `437b654` and passed by W2's gate. Neither BACKLOG item lists them, and W3's edit to
  that file is a new comment above them. `ids --check` is advisory (human-readable-ids
  D18-build-done-check).

VERDICT: PASS

Issues requiring fix: none.

Advisory, not blocking: (a) join `strips/implement.md:41` → `architecture-brainstorm-interview`
D5-visit-file-input and `:43` → `implement-brainstorm-led` D6-outside-need-asked at the landing, or
book them in the narrowed item. (b) The CHANGELOG's W3 budgets line reads "no skill body or
description changes", but the router body did change (stated at that paragraph's end). At the
slot fill, read "no budgeted skill body".

audit: unit 6 (prose primitives and ripple · router, three strips, ARCHITECTURE, README, GLOSSARY, budget ledger, CHANGELOG) · w3-gate-grader · opus · 9 files · 1 rounds · 0 blocking

## Unit 7 — crate tests, contract expectations, plan kits

VALIDATE: tests and kits. `crates/mochiko-cli/tests/{fidelity,validate,matrix_similar}.rs`;
`evals/contract/` (unchanged); `evals/plan/{specify,setup,architecture,implement,feature}/observable.yaml`;
the specify and architecture `preregistration.md`; `evals/plan/README.md`. This is also the crate's
non-author review: the diff is comments and asserts only, with no source touched.

Checklist run: each moved figure reproduced from the census, the contract call, each new `why`
against its rule and planting, the bucket shapes against the partition tool, every re-quote
byte-equal, the preregistration call, and the default-21 kit residuals.

Evidence read: every diff from `437b654` · per-kind counts from the pre and post views · the
`partition` tool at `32c1ed5`, `794cea8` and `437b654`, crossed with each kit's observable list ·
a YAML stray-key scan and a raw-line byte compare of every re-quote, pre against post · the setup,
specify and architecture kit headers and `evals.json` goldens · the fixtures' `Scope:` lines ·
`wave2-implement.md` §3.6 · `runs/` presence per kit.

Pre-pass: the shared pre-pass above (cargo 765/0, sweep 48/0, fmt and clippy 0, kits OK ×5,
contract host 7/7).

- **Crate figures — PASS.** Commands 418 → 424 and total 1,260 → 1,266 match `migrate status`.
  Per-kind counts: commands `binding` 64 → 67 and `constraint` 176 → 179; corpus 184 → 187 and
  589 → 592. Pairs: C(67,2)−C(64,2) = 195; C(179,2)−C(176,2) = 531 − 9 = 522; 20,351 + 717 =
  21,068. Corpus: 555 + (1,770 − 9) = 2,316, and 217,663 + 2,316 = 219,979, matching the validate
  scan. The 9 skipped pairs are C(5,2) − C(2,2) among the asking-line stubs. Suppressed hits 185 and
  59 hold, and the empty-allowlist run gives 76 clusters both sides, the listings identical. Floors
  130, fails 41, skills 842/266 unmoved. `fidelity.rs` adds `98, 99, 100, 101`, with 0096–0097 a gap.
- **Contract expectations unchanged — PASS.** `git diff 437b654 -- evals/contract` is empty. No
  floor, fail or template moved, and the host run's floor sets match the renders.
- **New whys — PASS, 6 of 6.** Each matches its rule limb by limb. The plantings hold:
  every specify golden carries a description (so the empty-arguments limb is unplanted), and s1 and
  s3 are UX-bearing. Setup's s2 plants three reality-versus-README conflicts, and all three modes
  are planted. The desk's g3 is the drift visit, g2 plants the fired AX-008 and "mark it n-a" on the
  floor-asserted rate-limiting row, and every fixture's `Scope:` is `backend-service`.
- **Bucket shapes — PASS, all reproduced.** specify @32c1ed5 44 (33) · 7 (6) · 0 · 22 (20); @437b654
  70 (56) · 1 (1) · 0 · 2 (2). architecture @794cea8 38 (35) · 9 (9) · 0 · 19 (16); @437b654 64 (58)
  · 0 · 0 · 2 (2). setup @437b654 42 (34) · 0 · 0 · 2 (2). implement @437b654 128 (73) · 3 (2) · 0
  · 0. The README figures, 59 of 73 and 60 of 66, match `check-rubric`.
- **Re-quotes byte-equal — PASS, 65 of 65.** Pre-wave stray-key counts were specify 1, setup 2,
  architecture 33, implement 18 and feature 11; post, 0 ×6. Each new double-quoted string equals the
  old raw text byte for byte, carries no `"` or `\`, and parses to it. No other why changed. Each
  footer is pinned, and specify's is singular ("the one why … is double-quoted whole, its words
  unchanged").
- **Preregistration call — PASS.** Specify and architecture have no `runs/` and are recounted, on
  the W1 precedent. Setup and implement have `runs/` and are left, with the reason in their footers.
  That is `wave2-implement.md` §3.6's rule. L2's clause is present, and `0057`/`0061` do carry those
  rewords.
- **Default-21 kit residuals — PASS.** Taken: four bare `D3` lines, joined as `D3-four-move-now`
  (`fidelity.rs`, both architecture files, the specify kit); the 18 implement whys; the 47 others
  (L1). Left: the brainstorm kit's three bare `D3` lines (`ids --check`: 3 bare), since that kit is
  untouched by the build; crate-source items stay out. `ids --check` over the touched tests and kits:
  0 bare, 0 drift.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 7 (tests and kits · three crate tests, contract expectations, five plan kits, two preregistrations, README · crate non-author review) · w3-gate-grader · opus · 11 files · 1 rounds · 0 blocking
