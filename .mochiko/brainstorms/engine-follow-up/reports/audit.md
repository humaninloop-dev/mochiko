---
report: review
round: 1
grader: w1-gate-grader
tier: opus
wave: 1
units: [1, 2, 3, 4, 5, 6, 7, 8]
---

## Shared pre-pass

This seat ran every command below itself, from the repo root. The tree was the working tree on
`engine-follow-up-build` at HEAD `c840469`, with the wave's 37 paths uncommitted plus the two session
files. The seat read its contract first-hand: `mochiko-cli rules validation-primitive-edit`,
the preamble and all six sections. Every block carried its version-triple line and its end line.
Floor read-back: `class: floor · 11 rules`, namely `author-grader` · `plain-seat-explicit-tier` ·
`rendered-contract-only` · `from-file-floor` · `pre-pass-first-hand` · `binary-verdict` ·
`default-fail` · `tamper-proof-clause` · `gate-loop-bound` · `evidence-floor` ·
`second-fail-user`. The baseline is `git archive c840469 plugins/mochiko
scripts/similar-rules-allowlist.yaml`, extracted into the session scratchpad and replayed by the
same binary. The run halted once on a usage limit before this file was written. It resumed on a tree the
lead confirmed unchanged, and the seat checked that claim itself: `git status` showed 39 entries, a second
`migrate validate` printed byte-identical output, and the payloads were re-measured identical.

```
$ mochiko-cli --version
mochiko-cli 0.4.0 · grammar 1..2

$ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
pointer resolution: 89 checked against plugins/mochiko
=== similar-rule clusters (threshold 0.60) ===
none — no pair clears the threshold
rules scanned: 1254 · in-kind pairs scored: 216338 · clusters: 0 (none)
allowlist-suppressed edges: 186
mochiko-cli migrate validate · 0 rejecting · 118 advisory

$ mochiko-cli migrate status --plugin-root plugins/mochiko
log plugins/mochiko/migrations · grammar 2 · sequences 1..89 (81 migrations)
state sha256:5ee3c452923bee57c1b5f3368dca78490fae51cfe8f846626b93f56c03c23e1d · 94 documents · 1254 rules

baseline (c840469 replay): sequences 1..84 (76 migrations) · sha256:58f165c8… · 93 documents ·
1243 rules · validate 0 rejecting · 116 advisory · scanned 1243 · scored 213340 · suppressed 184
```

The advisory delta, from a sorted diff of the full pre and post `--report` outputs, is +2. Both new
lines are `condition-coverage · command/feature · visit_kind`, for the values `"other"` and
`"ruling"`: each is declared, but no rule's `when:` names it. The `enforces-coverage` lines for feature
and setup now list `feat.thinking-stage`, `feat.fail.card-outside-set` and `setup.thinking-stage`.
The budget lines moved for common, feature, setup and authoring-feature-map. Nothing else
moved. Advisory findings never gate.

Views equal the replay. `mochiko-cli views emit --plugin-root plugins/mochiko --out <scratch>/views`
reported 94 documents, and `diff -r .mochiko/schema-views <scratch>/views` printed nothing. Compared
with the baseline emit (93 documents), six views differ. Five changed: `commands/feature.yaml`,
`commands/setup.yaml`, `common/common.yaml`, `homes/feature-desk.yaml` and
`skills/authoring-feature-map.yaml`. One is new: `templates/feature-visit.yaml`.

Crate gates, run from the workspace root:
- `cargo test --all` exited 0, with 765 passed and 0 failed across 20 result lines.
- `cargo fmt --all --check` exited 0.
- `cargo clippy --all-targets -- -D warnings` exited 0.
- The opt-in full similarity sweep, `MOCHIKO_FULL_SIMILAR=1 cargo test --test matrix_similar`, also
  passed: 48 passed, 0 failed, and `the_detector_reproduces_the_live_runs_figures_over_the_corpus ... ok`
  in 247.60s.

Kit checks, run with `PYTHONDONTWRITEBYTECODE=1 uv run evals/run.py command …`:
- `check-rubric feature`: `rubric OK: 57 observable, 4 out-of-instrument, 61 total`.
- `check-fixtures feature`: `fixture consistency: OK`.
- `check-rubric setup`: `rubric OK: 34 observable, 8 out-of-instrument, 42 total`.
- `check-fixtures setup`: `fixture consistency: OK`.

Char budgets were measured with the canonical snippet from `.mochiko/memory/primitive-cost-budgets.md`
§How to measure. Each count is characters of the parsed value. Each render is the seven
`!`-line blocks' stdout+stderr, summed.

| Primitive | Body | Description | Render | Payload | Budget | Over |
|---|---|---|---|---|---|---|
| `analysis-iterative` at `c840469` | 5,376 | 594 | 10,691 | 16,067 | — | — |
| `analysis-iterative` now | 5,588 | 713 | 10,691 | 16,279 | 16,067 | +212 payload |
| `analysis-iterative` description | — | 713 | — | — | 595 | +118 (cap 1,536) |
| `authoring-feature-map` at `c840469` | 6,027 | 598 | 16,829 | 22,856 | — | — |
| `authoring-feature-map` now | 6,027 | 598 | 16,977 | 23,004 | 22,323 | +681 |

- `analysis-iterative`'s render is identical before and after, section by section: preamble 1,650 ·
  independence 1,145 · scope 1,195 · inputs 1,017 · verdict 3,004 · output 1,212 · reserved 1,468.
- `authoring-feature-map` now renders preamble 2,208 · independence 595 · scope 2,094 · inputs 1,040 ·
  artifact 9,183 · output 1,059 · reserved 798. Its artifact section was 9,035 at baseline.
- A control replay of the working-tree log with `0089` removed measures render 16,829 and payload 22,856,
  so the full +148 is `0089`'s.
- The router body is unbudgeted and measures 48,744 → 48,776. Commands are unbudgeted. The feature render
  measures 18,507 → 24,641 characters, `feat.sec.tools` measures 7,584 bytes, and the setup render
  measures 17,661 → 18,002.

## Unit 1 — 0085 and the common view diff

VALIDATE: schema content. The unit is `plugins/mochiko/migrations/0085-asking-line.yaml` with its view diff
`.mochiko/schema-views/common/common.yaml` (one block appended).

Checklist run: `validation-primitive-edit.judgment-items-schema`, the AM-2 five, each held against record D10
as changed at review (QR3) and defaults 18–19. The engine ids the block cites are checked live (D10's
accepted risk).

Evidence read: the migration, read whole · `git diff c840469 -- .mochiko/schema-views/common/common.yaml` ·
the common view, read whole (the `common.thinking-stage` sibling and its labels) · the full
`analysis-iterative` render, preamble and six sections · record QR3 and D10 · wave plan §3.1.

Pre-pass: the shared pre-pass above. `0085` is one `mint-rule` with no exit.

- **Intent stated — PASS.** The header names the block, D10 with QR3 and defaults 18 and 19, the
  subset, and the contract pair "only where the command's own rules set the next writer on". It
  also states "No engine rule changes; it names no command and no place". The text carries each
  of those claims.
- **Anchor present where the exit requires one — PASS.** No op exits anything. The header anchor is
  `2026-10-15 engine-follow-up`, and the rule-level anchor is `… D10`.
- **ID lifecycle right — PASS.** `common.asking-line` is a fresh mint, and `mint-once` and
  `id-duplicate` are silent in the pre-pass. Every cited engine id resolves to a live `###` node in the
  `analysis-iterative` render: `fact-before-fork` (inputs) · `one-fork-per-turn`, `options-rule`,
  `question-form`, `mock-in-message`, `own-turn-or-batch`, `ratified-streak`, `vague-answer`
  (verdict) · `user-rules`, `contract-pen`, `contract-inside` (reserved). That is 11 of 11, and they
  match QR3's nine plus the contract pair exactly.
- **Floor and fail survival — PASS.** Nothing leaves. The engine render is unchanged, at 10,691 characters
  in both trees, so default 19 holds.
- **Register — PASS.** The block is one sentence plus one. In order, it names the subset, the *on* /
  *held* / *off* switch in D10's own definitions, the stage-only four (the blind list, the status
  line, the stop rule, provenance marks), and the yes / no gate keeping its form. It carries no class,
  which is correct for a common block, since a stub declares it locally. Its labels
  `[floor-pointer, user-gate]` mirror `common.thinking-stage`.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 1 (schema content · 0085 + common view diff) · w1-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 2 — 0086, the feature-visit template and the feature-desk home

VALIDATE: schema content. The unit is `plugins/mochiko/migrations/0086-feature-visit-home.yaml` with the new view
`.mochiko/schema-views/templates/feature-visit.yaml`, the `homes/feature-desk.yaml` diff, and the
`template` and `home` renders.

Checklist run: the AM-2 five, held against D7 as changed at review (QR4), D3, and defaults 11 and 13.
Parallel wording was checked against the `architecture-visit` template and the `architecture-desk` home.

Evidence read: the migration, read whole · `git diff c840469 -- .mochiko/schema-views/homes/feature-desk.yaml` ·
`.mochiko/schema-views/templates/architecture-visit.yaml` · `homes/architecture-desk.yaml` ·
`mochiko-cli template feature-visit --plugin-root plugins/mochiko` and the same with `--check`, both exit 0 ·
`mochiko-cli home .mochiko/features/desk/2026-10-16-feat-001-sample/visit.md --plugin-root
plugins/mochiko`, exit 0, which resolved "`visit.md`, a declared deliverable · template `feature-visit`" ·
`mochiko-cli template feature-entry`, for the row grammar.

Pre-pass: the shared pre-pass above. `0086` is one `import-document` and one `replace-document`.

- **Intent stated — PASS.** The header names the template, the home title fix (default 13), `visit.md`
  under the 300 whole-file bound, and the two-capability case resolved as "two folders, each a legal
  `<date-slug>`; no grammar change". The render matches every clause.
- **Anchor present where the exit requires one — PASS.** No rule exits anything. The replaced home keeps
  `derivation.md` (300 lines, no template) and `reports/` (`report-envelope`, `by_type: {}`) byte for byte.
  Only the title changed and the one deliverable was added.
- **ID lifecycle right — PASS.** `template/feature-visit` is new. The home id is unchanged.
- **Floor and fail survival — PASS.** The template and the home carry no rules.
- **Register — PASS.** The sections are Header · Ruled set · Defaults batch (optional) · Blind list
  (optional) · Cards · Verdict. They mirror `architecture-visit`, which has Header · Focus set · Defaults
  batch · Blind list · Verdict, and drop the store-only "Outside claims" and "Rows written".
  - The Ruled set carries D3's unit: kind · the user's change · "I'd know it works if…" or `open`, never
    filled in · a bug's reproduction as handed in · one provenance mark in the engine's vocabulary.
  - Cards are "in the card shape `mochiko-cli template tasks` renders — never a second card grammar", as D7
    and plan §3.2 require.
  - The folder slug carries the lower-cased joined FEAT id or `lane-<slug>`, and a two-capability visit
    writes one file each (default 11).
  - The title now reads "One feature-desk visit; a card visit keeps one folder per capability or lane"
    (default 13).
  - The fixtures `feature-visit.producer.txt` and `.check.txt` equal the renders minus the trailing
    `schemas: replayed from` line. That is the same convention as the `architecture-visit` pair, and
    `render.rs` strips that line.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 2 (schema content · 0086 + feature-visit template + feature-desk home diff) · w1-gate-grader · opus · 3 files · 1 rounds · 0 blocking

## Unit 3 — 0087, 0089, the allowlist rows and the authoring-feature-map overage

VALIDATE: schema content. The unit is `plugins/mochiko/migrations/0087-feature-on-engine.yaml` with the
`commands/feature.yaml` diff · `0089-entry-points-at-visit.yaml` with the `skills/authoring-feature-map.yaml`
diff · the two rows appended to `scripts/similar-rules-allowlist.yaml` · the named overage +681.

Checklist run: the AM-2 five for each migration, held row by row against wave plan §3.3–§3.4 and the §3.8
coverage table. The record is the source: D2 with QR5, D3, D7 with QR4, D10's re-sort Feature row,
defaults 6–11 and 23–24 as changed at review (RB12, RB13, RB2). The overage was ruled under
`primitive-edits.md`'s argued-overage path.

Evidence read:
- Both migrations, read whole · `git diff c840469` of both views and the allowlist · the full `feature`
  render, preamble and six sections.
- `.mochiko/schema-views/commands/architecture.yaml` (`visit_kind`, `arch.focus-stage`,
  `arch.focus-blind-list`, `arch.fail.write-outside-focus`, `arch.thinking-stage`).
- The `patterns-sound-loop` render (`default-seat-wiring`: "desk delta cards: desk-authored (review leg
  only), `devils-advocate` reviews").
- The `authoring-feature-map` render, all seven blocks · `mochiko-cli template feature-entry` (Work rows
  grammar) · `plugins/mochiko/migrations/README.md` ops table · `crates/mochiko-cli/src/validate.rs:1854–1871`.

Pre-pass: the shared pre-pass above. Across `0085`–`0089` the ops are `mint-rule` ×12 · `reword-rule` ×4 ·
`set-condition` · `set-moment` · `import-document` · `replace-document`. There is no `supersede-rule` or
`tombstone-rule`. Feature pins moved from `kind: fail · 1` and `class: floor · 14` to 2 and 16.

- **Intent stated — PASS.** `0087`'s header names the condition, the moment, both stubs, six domain rules,
  `feat.carry-forward`, the fail, and the three rewords, each with its earlier ruling. `0089`'s header
  quotes QR4 and states why the template's row grammar has no place for the pointer. That claim was
  confirmed: the grammar is "`pending` — {{increment}} · acceptance: {{criteria}} · cut by …". The header
  also says that one skill reword suffices.
- **Anchor present where the exit requires one — PASS.** No exit. Each mint carries its decision segment
  (D2, D3, D7, D10) or the bare session anchor for a confirmed default. `reword-rule` takes no
  `anchor`/`disposition` field (README table: `schema`, `id`, `text`). The earlier rulings ride the
  migration headers and the strip wave-context notes, which follows the `0068` precedent.
- **ID lifecycle right — PASS.** Ten fresh `feat.*` mints and one condition and moment. Four rewords keep id,
  class, kind and anchor (`feat.artifact-home` stays `class: floor`). Each reword keeps every prior clause
  and adds the visit-file limb:
  - `feat.artifact-home` adds "a card visit's is `visit.md` bound to the `feature-visit` template".
  - `feat.delta-cards` adds "written in the visit file's card part".
  - `feat.stable-ground-triage` reads "card on the entry" as "the entry's marked work row pointing at the
    card in the visit file", which is D7's changed-at-review line.
  - `authoring-feature-map.growth-rides-work-row` keeps its first clause word for word.

  `feat.dispatch-scope-split` and `feat.dispatch-specify` are untouched and still true.
- **Floor and fail survival — PASS.** Nothing leaves or lowers. `feat.user-reserved`'s reservation, the
  grooming-door ceiling and the growth-door routing are untouched. The new floor fail
  `feat.fail.card-outside-set` enforces `feat.card-stage`, `feat.visit-file` and `feat.card-writer`, all
  live, which mirrors `arch.fail.write-outside-focus`.
- **Register and coverage — PASS.** Each §3.8 row has its carrier in the render:
  - D2 with QR5: `visit_kind`, the `card-set` moment, and `feat.card-stage`, which has the contract on,
    the visit goal as opening, a single-card clear ask closing on the goal nod, and its own screen at more
    than one card or any fork.
  - D3: `feat.card-unit`.
  - D7: `feat.visit-file`, which has the pen for header and set and then the card's writer, the
    per-capability file, and the slug.
  - Defaults 6 and 7: `feat.card-writer`. Its wiring follows the desk, and `devils-advocate` grades
    sameness of meaning, which matches the sound-loop default.
  - Default 10: `feat.card-blind-list`.
  - Default 8: `feat.thinking-stage` (floor, fifth extender) and the fail.
  - D10's Feature row: `feat.ruling-places`, with grooming and epic writes *on*, route override and
    growth-door trigger *off*, and selection *held*, exactly the re-sort. Also `feat.asking-line` (must).
  - Defaults 23 and 24: `feat.carry-forward`. Its specify half hands in `derivation.md` and writes no visit
    file (RB13).

  Wording parallels the architecture desk. No engine mechanic is restated.
- **Allowlist rows — PASS.** Two rows, each naming the new `kind: fail` edge with the reason recorded.
  Suppressed edges went 184 → 186 and clusters stayed 0. The decisive reason is a different artifact per
  member. The `validate.rs:1854–1871` citation resolves to the block-carries-`kind:` check (Notes of note).
- **Named overage +681 on `authoring-feature-map` — HOLDS.** Measured first-hand at 23,004 against 22,323.
  It is the standing +533, already ruled HOLDS at v0.119.0, plus +148 render from `0089` alone by control
  replay. Body (6,027) and description (598) are unchanged. The growth is D7's ruled obligation as changed
  at review (QR4: "the entry's marked delta points at it"), a new pointer duty, not restored playbook
  prose.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 3 (schema content · 0087 + 0089 + feature and authoring-feature-map view diffs + allowlist rows) · w1-gate-grader · opus · 5 files · 1 rounds · 0 blocking

## Unit 4 — 0088 and the setup view diff

VALIDATE: schema content. The unit is `plugins/mochiko/migrations/0088-setup-thinking-stage.yaml` with the
`commands/setup.yaml` diff (one stub appended to `setup.sec.boundaries`).

Checklist run: the AM-2 five, held against default 17 and the re-sort's Setup row (defaults 15 and 16 read
as *held*).

Evidence read: the migration, read whole · `git diff c840469 -- .mochiko/schema-views/commands/setup.yaml` ·
the full `setup` render · the six `extends: common.thinking-stage` stubs across the command views.

Pre-pass: the shared pre-pass above. Setup pins moved from `class: floor · 20` to 21. `kind: fail` stays at 6.

- **Intent stated — PASS.** The header says "sixth extender", "contract not named, so it stays off",
  and "`setup.interrogation-inline` keeps its pointer and no other setup rule changes". All three hold.
  The extenders are brainstorm, specify, architecture, implement, feature and setup.
- **Anchor present where the exit requires one — PASS.** No exit. The bare session anchor is used, which
  fits a confirmed default.
- **ID lifecycle right — PASS.** `setup.thinking-stage` is a fresh mint, and no other setup id moved.
- **Floor and fail survival — PASS.** Nothing leaves, and the floor set grows by one.
- **Register — PASS.** The stub inherits `common.thinking-stage`'s text verbatim, and its class is local
  `floor`.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 4 (schema content · 0088 + setup view diff) · w1-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 5 — the feature command pair

VALIDATE: command pair. The unit is `plugins/mochiko/commands/feature.md` with the full `feature` render
(preamble and six sections), read against the `analysis-iterative` render and its
`caller-slots`.

Checklist run: `validation-primitive-edit.judgment-items-pair`, which covers scaffold headings and order ·
preserved responsibilities · floor survival · independence · reserved-to-user content · the desk
done-condition branch · overage, which does not apply because commands are unbudgeted. The mechanical items
were read from the pre-pass.

Evidence read: `feature.md`, read whole, plus `git diff c840469` (the one Goal-step sentence) · the full render,
whose section end-lines sum 14 + 2 + 19 + 11 + 13 + 2 = 61, matching the budget line's 61 rules · the
`analysis-iterative` render · `.mochiko/strips/feature.md` [v0.121.0].

Pre-pass: the shared pre-pass above, 0 rejecting.

- **Scaffold headings and order — PASS.** The frontmatter key set is complete. The headings are
  `# Feature — The Product Desk` · Identity & Mission · Rules (preamble plus the six-set) · Adaptive Goal
  Protocol (Entry, then Goal, then Not done last). The prose cites `feat.card-stage` and
  `feat.sec.fail-conditions`, and both are live.
- **Preserved responsibilities — PASS.** The protected Goal-step line is kept word for word, re-pointed at the
  one-confirm close (default 14, QR5), and recorded as a supersession-by-ruling strip entry. No other line
  changed. No line says "card on the entry", so plan §3.6's visit-file rename had nothing to land on.
- **Floor survival — PASS.** The Not-done line cites the CLI pin (`kind: fail · 2 rules`) and keeps its
  halt clause. It carries no hard-coded count.
- **Independence — PASS.** No seat grades its own row. Desk-authored cards take `devils-advocate`
  (`feat.card-writer`, `feat.sound-loop-floor`). `feat.no-self-graded-writes` stands. Producing seats plan
  first per `mochiko:review-seat-plan` (`feat.author-grader`). The sound-loop pointer is present, since
  feature is DM-chartered.
- **Reserved-to-user content — PASS.** `feat.sec.reserved` holds `feat.user-reserved` unchanged and
  `feat.ruling-places`.
- **Done-condition branch, desk — PASS.** Each visit converges to a one-line goal and its done condition and
  closes with a verdict against it. The card set's close sits inside that condition.
- **Caller slots — PASS, every slot filled or defaulted.**
  - opening: the visit goal (`feat.card-stage`)
  - unit: `feat.card-unit`
  - where the set is saved: `visit.md` (`feat.visit-file`, `feat.card-stage`)
  - contract on and next writer: the card's writer (`feat.card-stage`, `feat.card-writer`)
  - blind list: cadence and inputs set by `feat.card-blind-list`; its job defaulted to the engine's
  - independent leg: `devils-advocate`'s meaning grade
  - provenance marks: a place in the template
  - `contract-inside`'s by-name latitude: not granted, which the engine allows

VERDICT: PASS

Issues requiring fix: none.

audit: unit 5 (command pair · feature) · w1-gate-grader · opus · 1 files · 1 rounds · 0 blocking

## Unit 6 — the setup command pair and the analysis-iterative skill pair

VALIDATE: command pair `plugins/mochiko/commands/setup.md` with the full `setup` render, and skill pair
`plugins/mochiko/skills/analysis-iterative/SKILL.md` with its render. The skill's `description:` and
Overview changed. Two overages are named.

Checklist run: `judgment-items-pair` for both, plus the run done-condition branch for setup. Every caller's
render was re-read for a line the skill edit makes false.

Evidence read:
- `setup.md`, read whole, unchanged since `c840469` · the full `setup` render (end-lines 5 + 6 + 11 + 6 + 8 + 6 = 42).
- `SKILL.md`, read whole, plus `git diff c840469` · the `analysis-iterative` render (identical to baseline).
- Full renders of brainstorm (49 rules), specify (71), architecture (64), implement (125), setup and feature,
  scanned for every engine, caller and feature-desk mention.
- The `analysis-iterative` ledger rows · `.mochiko/strips/analysis-iterative.md`.

Pre-pass: the shared pre-pass above. Payload 16,279 is +212 over budget, and the description is 713, which is
+118 over 595 and under the 1,536 cap.

**Setup pair:**
- **Scaffold — PASS.** Headings and order are canonical.
- **Floor survival — PASS.** The Not-done line cites the pin, which reads 6 with no hard-coded count.
- **Preserved responsibilities — PASS.** The pair is unchanged.
- **Independence — PASS.** `setup.validate-seat-form` and the cold stress-test seat stand.
- **Reserved — PASS.** Six rules are in `setup.sec.reserved`.
- **Run branch — PASS.** The done condition is fixed, Entry handles `$ARGUMENTS`, and Not done stays last.
- **New stub — PASS.** `setup.thinking-stage` makes no `setup.md` line false.

**Skill pair:**
- **Load-first block and floor read-back — PASS.** Both are unchanged and cite the pin and the `floors:` line.
- **Preserved responsibilities — PASS.** Both edits are pure insertions. The description gains "the feature
  desk's card visits" and the asking-line clause. The Overview gains the `/mochiko:feature` caller and one
  sentence naming `common.asking-line`.
- **Description length — PASS.** The description is ≤ 1,536.
- **No rule, slot or body mechanic touched — PASS.** The render is identical, so default 19 holds.

**Caller re-read — PASS.** No line is now false.
- Brainstorm `:56`, `:80`, `:118`, specify `:160`, `:224`, architecture `:193` and implement `:138`, `:192`
  still describe their own stage.
- The six `*.thinking-stage` stubs read the same.
- Setup's interrogation still reaches the engine "through its own pointers, the contract off".

**Named overages — both HOLD.** D2 makes feature a caller and QR3 makes the asking line a pointer to this
skill. Both are routing obligations that the MUST trigger list and the Overview's caller list exist to carry.
Neither is restored playbook prose.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 6 (command pair · setup; skill pair · analysis-iterative, description and Overview) · w1-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 7 — router, strips, ripple, budget ledger, changelog

VALIDATE: the following units:
- Prose primitive `plugins/mochiko/skills/mochiko/SKILL.md`.
- Strips `.mochiko/strips/feature.md`, `analysis-iterative.md`, `mochiko.md` and `authoring-feature-map.md`.
- Ripple: `ARCHITECTURE.md` and `GLOSSARY.md`.
- `.mochiko/memory/primitive-cost-budgets.md`.
- `CHANGELOG.md` `## [0.121.0]`.

Checklist run: `judgment-items-prose`, which covers coherence and preserved responsibilities. Strip
conformance follows `.mochiko/strips/README.md`. Every figure was checked against first-hand measurement.

Evidence read: `git diff c840469` of all nine files · router rows `:66`, `:85`, `:124`, `:141`, `:142` ·
`.claude/rules/mochiko/operating-docs.md:20` · `.mochiko/product/architecture/spine.md` (scaffold only) ·
`git grep "brainstorm ceremony"` · `commands/architecture.md:57–61`.

- **Router — PASS.** The `analysis-iterative` row gains "the product desk's card visits", a pure insertion.
  The `/mochiko:feature` row `:141` makes no claim the wave falsifies. The body measures 48,744 → 48,776,
  as its strip note says.
- **Strips — PASS.**
  - `feature.md` has one [v0.121.0] supersession-by-ruling entry, with Disposition, Tier failed (default 14
    with QR5), verbatim Content, Kept deliberately (the [v0.97.0] and [v0.105.0] protected line), and
    Consumers assessed. The `git grep` confirms its consumer claim.
  - The other three carry wave-context notes for pure additions or schema-only edits, each naming its ruling
    and pre-edit ref. Default 20's reword record is in `feature.md` and `authoring-feature-map.md`.
- **Ripple — PASS.**
  - `ARCHITECTURE.md` is still hand-maintained legacy, because the store is scaffold-only
    (`operating-docs.md:20`), so the edit is legal.
  - `GLOSSARY.md`'s **visit file** is widened to both desks. It also corrects the stale `focus.md` to
    `visit.md`, which agrees with the `architecture-desk` home.
- **Ledger — PASS.** The `analysis-iterative` rows (16,279 = 5,588 + 10,691; description 713) match this
  seat's measurement. So does the `authoring-feature-map` row: 23,004 = 6,027 + 16,977, with the per-section
  split, the control replay at 16,829, and the log state `5ee3c452…` · 94 · 1,254. Commands correctly carry
  no row. The `[lead: …]` slots are expected.
- **CHANGELOG — PASS.** Every figure checks first-hand:
  - pins 1 → 2 and 14 → 16, and setup 20 → 21
  - log 1,243 → 1,254 and 94 documents
  - feature render 18,507 → 24,641 and setup render 17,661 → 18,002
  - router 48,744 → 48,776
  - payloads and the description length
  - "1 strip entry"

  The `[lead: …]` placeholders are expected at this stage.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 7 (prose primitives and records · router, four strips, ARCHITECTURE, GLOSSARY, budget ledger, CHANGELOG) · w1-gate-grader · opus · 9 files · 1 rounds · 0 blocking

## Unit 8 — crate tests, template fixtures, contract expectations, plan kits

VALIDATE: the following files:
- Tests: `crates/mochiko-cli/tests/fidelity.rs`, `matrix_similar.rs`, `render.rs`, `validate.rs` and `views.rs`.
- Fixtures: `crates/mochiko-cli/tests/fixtures/template/feature-visit.check.txt` and `.producer.txt`.
- Contract: `evals/contract/run.py` and `README.md`.
- `evals/plan/README.md`.
- Feature kit: `evals/plan/feature/evals.json`, `observable.yaml` and `preregistration.md`.
- `evals/plan/setup/observable.yaml`.

This unit is also the crate's independent non-author code review (`rust-cli.md`).

Checklist run: coherence and preserved responsibilities. Every pinned figure was checked against
`migrate status`, the renders and the pre-pass, never against a seat's report. Plan §3.7 scope applies:
no crate source and no crate version bump.

Evidence read: `git diff c840469` of all twelve tracked files · both fixtures diffed against the live renders ·
`git diff --stat c840469 -- crates`, which shows tests only · `src/validate.rs:1854–1871` · a token check
that each of the 57 observable feature ids is planted on some golden (0 missing) · the setup
preregistration's subset lines.

Pre-pass: `cargo test --all` 765/0, fmt, clippy, the opt-in full sweep, and all four kit checks green (above).

- **Crate tests — PASS.**
  - Docs 93 → 94 and sequences through 89.
  - Command rules 401 → 412 and total 1,243 → 1,254.
  - Command floors 126 → 129 (`feat.thinking-stage`, `feat.fail.card-outside-set`, `setup.thinking-stage`)
    and fail nodes 39 → 40. Skill counts stay at 842 and 266.
  - Similarity pins `(412, 19_923, 0, 59)` and `1254 / 216_338 / 0 / 186`.
  - The `SHIPPED_TEMPLATES` list grows 15 → 16.

  Each figure agrees with the pre-pass, and the comments' per-kind arithmetic agrees with the minted kinds
  (binding +4 · duty +3 · fail +1 · constraint +3). Only assertions and comments changed. Nothing is
  weakened or skipped.
- **Fixtures — PASS.** Each equals its render minus the `schemas:` source line.
- **Contract — PASS.** `EXPECTED` gains `feat.thinking-stage`, `feat.fail.card-outside-set` and
  `setup.thinking-stage` with their anchors. `TEMPLATE_NAMES` gains `feature-visit`. The frozen byte
  baselines are untouched. The README floor figure is 126 → 129. `expected-skills.json` is rightly unchanged,
  since no skill floor moved.
- **Plan kits — PASS.**
  - Feature is 57 · 4 of 61, with nine minted ids observable, each with its why, and the fail out of
    instrument.
  - Four new unplanted facets are declared.
  - The planted sets are s1 44 · s2 50 · s3 47 (141 of 171).
  - `evals.json`'s s1 golden phrase made false by the rewords is repaired. It reaches no session.
  - Setup is 34 · 8 of 42.
  - The README reads "feature 57 of 61".

VERDICT: PASS

Issues requiring fix: none.

audit: unit 8 (crate tests, template fixtures, contract expectations, plan kits · crate non-author review) · w1-gate-grader · opus · 14 files · 1 rounds · 0 blocking

## Notes of note

- Advisory only: feature's `visit_kind` values `ruling` and `other` gate no rule. They classify, as
  default 9 ruled; the desk's own `other` is likewise unused.
- Forward reference: the template overview and `GLOSSARY.md` say the dispatched run reads the cards
  in `visit.md`. That is D7's ruling, but implement's reader lands in W2 (default 12); if W2 misses
  the bump, both lines go false. The W2 gate should re-read them.
- The allowlist reasons say no shared block can converge the fails because a common block cannot
  carry `kind:`. True, but a stub could add `kind: fail` locally; the rows stand on their first
  clause, a different artifact for each desk.
- `evals/plan/setup/preregistration.md` still reads a 32-rule subset, stale before this wave and
  kept as the ontology grid's record (its `observable.yaml` says so). Not this wave's defect.
- This seat did not run the contract suite: the brief did not list it, and the bump gate owns it
  (`sbx` access is the user's). A SKIPPED suite blocks the bump.
