---
report: review
round: 2
grader: w1-gate-grader
tier: opus
wave: 1
units: [1, 2, 3, 4, 8a, 5, 5b, 6, 7, 8]
---

## Shared pre-pass — units 1–4 and 8a

Every command below was run first-hand by this seat, from the repo root, against the working tree
on `engine-desk-build` (HEAD `5162f1a` plus the uncommitted W1 P1 writes). The baseline is a HEAD
copy of `plugins/mochiko/` and `scripts/similar-rules-allowlist.yaml` (via `git archive HEAD`) in
the session scratchpad, replayed by the same binary.

```
$ mochiko-cli --version
mochiko-cli 0.4.0 · grammar 1..2

$ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
pointer resolution: 89 checked against plugins/mochiko
=== similar-rule clusters (threshold 0.60) ===
none — no pair clears the threshold
rules scanned: 1211 · in-kind pairs scored: 205579 · clusters: 0 (none)
allowlist-suppressed edges: 184
mochiko-cli migrate validate · 0 rejecting · 115 advisory

$ mochiko-cli migrate status --plugin-root plugins/mochiko
log plugins/mochiko/migrations · grammar 2 · sequences 1..62 (60 migrations)
state sha256:a978b6d111f31c2399b8f6528395ed25fed40e72fef281810e51bd842e01d66a · 88 documents · 1211 rules

$ mochiko-cli views emit --plugin-root plugins/mochiko --out <scratch>/gg-views.eopi
mochiko-cli views emit · 88 documents · <scratch>/gg-views.eopi
$ diff -r <scratch>/gg-views.eopi .mochiko/schema-views
(no output: the committed views equal the replay)

$ mochiko-cli ids --check plugins/mochiko/migrations/0058-…0062-… --plugin-root plugins/mochiko
mochiko-cli ids --check · 0 bare · 0 drift
$ mochiko-cli ids --check .mochiko/brainstorms/implement-brainstorm-led/research/obligation-map.md --plugin-root plugins/mochiko
mochiko-cli ids --check · 0 bare · 0 drift
```

Baseline at HEAD (same binary, HEAD allowlist): `sequences 1..57 (55 migrations)` · `state
sha256:603bb294…` · `87 documents · 1196 rules` · `0 rejecting · 113 advisory` · `rules scanned:
1196` · `allowlist-suppressed edges: 185`. The sorted advisory diff, HEAD to now, is exactly: + `budget ·
skill/analysis-iterative · 21 rules · 5133 resolved characters of rule text` · + `condition-coverage ·
skill/analysis-iterative · contract · value "false" is declared but named by no rule's when:` (the
binary folds `off` onto `false`, `crates/mochiko-cli/src/model.rs:1241`; the render prints `values:
on|off`) · `enforces-coverage` for brainstorm and specify each gain their `thinking-stage` stub ·
budget lines move: common 1,837 → 2,042 · brainstorm 10,996 → 9,252 · specify 15,616 → 14,696 ·
review-specifications 7,559 → 7,601. Nothing else moved.

Renders: `mochiko-cli rules <p> --section <id> --plugin-root plugins/mochiko` for `preamble` and
every section the preamble lists, for `analysis-iterative`, `brainstorm`, `specify`, `setup`,
`review-specifications` — all exit 0, every section end-line count equal to the preamble's
section line. A whole-render comparison of all 49 primitives (6 commands, 43 skills) at HEAD
against now differs for exactly four: `brainstorm`, `specify`, `analysis-iterative`,
`review-specifications`. `setup`'s render is byte-identical (no setup rule edited, frame change 3).

Char budget (the ledger's canonical snippet; the render counted as all seven blocks). Method check:
it reproduces the ledger's `review-specifications` v0.119.0 row exactly at HEAD (body 3,441 +
render 14,664 = 18,105). Now: `review-specifications` body 3,441 · description 490 · render 14,708
· payload 18,149 against budget 16,174 — the standing +1,931 ruled HOLDS at v0.119.0 plus **+44**
this wave, render only, all from 0062. `analysis-iterative`: rule text 5,133 (validate's advisory
budget line), render 10,691, body 4,222 and description 476 unchanged so far — the pair payload is
unit 5's. Commands carry no ledger budget (brainstorm render 18,788 chars, specify 27,160, both
under the ~30k inline ceiling).

## Unit 1 — 0058 and the new analysis-iterative view

VALIDATE: schema content — `plugins/mochiko/migrations/0058-thinking-engine.yaml` with its
regenerated view `.mochiko/schema-views/skills/analysis-iterative.yaml` (new, untracked).

Checklist run: `validation-primitive-edit.judgment-items-schema`, the AM-2 five — intent stated ·
anchor present where the exit requires one · ID lifecycle right · floor and fail survival ·
register. Each rule was also held against the ruling it executes: record D1 (as changed at review
S2, S13 and verify V1), D2 (S4), D4, the *Defaults batch* and the plan chain base → rev1 → rev2,
with the disclosed build deviation (1).

Evidence read: `plugins/mochiko/migrations/0058-thinking-engine.yaml`, read whole · the new view
`.mochiko/schema-views/skills/analysis-iterative.yaml`, read in part and proven equal to the
replay by `diff -r` · the rendered `analysis-iterative` preamble and six sections · the current
`plugins/mochiko/skills/analysis-iterative/SKILL.md` (lines 20–24, the source of the two relocated
lines) · `.mochiko/strips/analysis-iterative.md` (v0.63.0 and v0.117.0 entries) · record
`## Decisions` D1–D4, *Defaults batch*, *Supersessions owed*, *Touch set* · plan files
`w1-p1-plan.md` §6.1, `w1-p1-plan-rev1.md` §6.1, `w1-p1-plan-rev2.md` §6.1 (hashes match the
build log's pins) · `research/obligation-map.md` · `crates/mochiko-cli/src/model.rs:1224–1253`.

Pre-pass: the shared pre-pass above. Render pins: `class: floor · 2 rules` · `floors:
analysis-iterative.open-unknowns · analysis-iterative.contract-inside` · conditions `contract ·
values: on|off · resolution: entry-derived` · sections independence 2 · scope 2 · inputs 2 ·
verdict 9 · output 3 · reserved 3 (21).

- intent stated — PASS. The header names the conversion, the review six-set, the one caller-set
  condition and its off default, and "twenty-one rules" — the count the render prints. Every
  rule falls under one of the header's named groups: the asking half, the slots, the provenance
  vocabulary, the relocated floor and deliverable, the switchable contract.
- anchor present where the exit requires one — PASS. `import-document` exits nothing. The header
  anchor is well formed. Each D1 or D4 rule carries its decision segment, and each confirmed
  default takes the bare session anchor (`lead-holds-conversation`, `engine-precedence`). The two
  relocated protected lines keep their origin rulings: `open-unknowns` keeps `2026-08-10
  validator-scope-and-verbosity`, the v0.63.0 floor line, and `deliverable` keeps `2026-10-06
  brainstorm-target-state D21`, the v0.117.0 repair. Both trace in the strips file.
- ID lifecycle right — PASS. All 21 ids and six section ids are fresh mints in a document that
  did not exist at HEAD (mint-once clean at 0 rejecting). Nothing is tombstoned, and none is owed:
  the skill had no schema before. Every built text equals the plan chain's winning entry: rev1's
  for `lead-holds-conversation`, `fact-before-fork`, `mock-in-message`, `own-turn-or-batch`,
  `ratified-streak`, `stop-rule` and `contract-inside`; rev2's (the base wording) for
  `engine-precedence`; the base's for the rest. The one exception is the disclosed deviation (1)
  on `user-rules`, "pull any default out" built as "pull a default out". It holds: in a
  permission clause "a default" reads as any one default, the wording is brainstorm's own prior
  text, and the map records G17, G35 and G50 with "the meaning is unchanged".
- floor and fail survival — PASS. The prose floor at `SKILL.md:24` is relocated word for word as
  `class: floor` `open-unknowns`, and the deliverable line at `SKILL.md:22` word for word as
  `deliverable`. `contract-inside` is a raise to `class: floor`, the lead's lean taken. Its
  `when: contract=on` gates application only, and the legend prints that floors are delivered
  whatever their `when:` — the F23 exposure, ruled no problem at S4. No `kind: fail` and no
  `enforces:` appear, as skill grammar requires (F15).
- register — PASS. Every rule speaks to the lead in the second person, as a guardrail with no
  step order. Each runs one or two sentences in plain words, with the user-facing nouns the D1
  card uses: fork, ruled set, next writer. The labels come from the skill registry (no label
  finding). The `contract` note states its off default. `pointer: SYNTHESIS.md` resolves
  (pointer resolution 89 checked, 88 at HEAD; no `pointer-unresolved`). `open-unknowns` keeps "the principal"
  as a verbatim relocation of protected text, which is correct.
- coverage of the ruling (context, not a sixth item). Every limb of D1's asking half has a
  carrier: one fork per turn, fact first, two options with the cheaper one, the mock, the case
  against with a one-line pick, own-turn-or-batch, the status line, the streak, the vague answer,
  the stop rule with its stall clause, provenance, the fenced blind list, inputs in hand with
  skip-never-the-close. So do the contract half (as changed at S2, with R12's by-name latitude),
  D4's slot list and the V1 defaults: contract off, the costly-fork list, marks saved where the
  caller writes, else kept in the conversation.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 1 (schema content · 0058 + analysis-iterative view) · w1-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 2 — 0059 and the common view diff

VALIDATE: schema content — `plugins/mochiko/migrations/0059-thinking-stage-block.yaml` with the
view diff of `.mochiko/schema-views/common/common.yaml`.

Checklist run: `validation-primitive-edit.judgment-items-schema`, the AM-2 five, held against
the default *Slots declared where* (the transport-floor shape, F20) and D3's floor-class pointer.

Evidence read: `plugins/mochiko/migrations/0059-thinking-stage-block.yaml`, read whole · `git diff
-- .mochiko/schema-views/common/common.yaml`, read whole (one block added, nothing else) · the two
resolved stubs in the brainstorm and specify renders (`brainstorm.sec.boundaries`,
`spec.sec.boundaries`) · plan `w1-p1-plan.md` §6.2 · record *Defaults batch* and D3.

Pre-pass: the shared pre-pass above. Validate's `command-common/common` budget line moves from
10 rules · 1,837 to 11 rules · 2,042.

- intent stated — PASS. The header names the block, its pointer, D1 and D3, and that it names no
  contract mode. The block's text indeed names none, since a stub inherits text only.
- anchor present where the exit requires one — PASS. A pure mint exits nothing. The block
  carries `2026-10-11 implement-brainstorm-led D1`, which makes it protected content.
- ID lifecycle right — PASS. `common.thinking-stage` is fresh. Both stubs that extend it resolve
  (no `extends-unresolved`), and each declares `class: floor` locally (C3; no
  `extends-class-local`).
- floor and fail survival — PASS. Nothing existing is touched. The block is classless by grammar,
  and the floor rides each stub, as D3 rules.
- register — PASS. One sentence in the pointer-block shape of `common.transport-floor`:
  "referenced, never restated". It carries labels `floor-pointer` and `user-gate`, which the
  stubs inherit, so no `labels-inherited` finding is raised, and it has `pointer:
  mochiko:analysis-iterative`. Extraction bar: the record ruled this block for the four movers
  (D3, *Slots declared where*). W1 binds two of them, and W2 and W3 bind the desk and implement
  before the one 0.120.0 bump. That makes the 3+-command bar met at ship time — see the notes.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 2 (schema content · 0059 + common view diff) · w1-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 3 — 0060 and the brainstorm view diff

VALIDATE: schema content — `plugins/mochiko/migrations/0060-brainstorm-on-engine.yaml` with the
view diff of `.mochiko/schema-views/commands/brainstorm.yaml`.

Checklist run: `validation-primitive-edit.judgment-items-schema`, the AM-2 five, held against D3
(as changed at S7), the default *Precedence*, *Supersessions owed* (brainstorm-target-state
D19-brainstorm-only-reach), the plan chain and the disclosed deviations (2) and (3). The
`set-rule-field when: null` was judged under the ontology-grammar criterion, as the brief asks.

Evidence read: `plugins/mochiko/migrations/0060-brainstorm-on-engine.yaml`, read whole · `git
diff -- .mochiko/schema-views/commands/brainstorm.yaml`, read whole (every before-text checked
against its after-text) · the full brainstorm render, preamble and six sections · the HEAD
brainstorm preamble, replayed · plan `w1-p1-plan.md` §6.3 and §6.6 · `research/obligation-map.md`
(B1–B35, L1–L16, the build-time table) · `plugins/mochiko/migrations/0058-thinking-engine.yaml` (each
named carrier) · `scripts/similar-rules-allowlist.yaml` (the size-arm precedent row).

Pre-pass: the shared pre-pass above. Pins, HEAD to now: `kind: fail · 4 rules` → 4 · `class:
floor · 8 rules` → 9 (`brainstorm.thinking-stage` added, all eight kept by id). Sections 8 · 8 ·
9 · 25 · 1 · 4 (55) → 7 · 8 · 9 · 19 · 2 · 4 (49). That is −1 roles (`own-rules-win`), −6
ways-of-working (the six asking rules) and +1 boundaries (the stub).

- intent stated — PASS. The ops are one mint, seven supersessions (`own-rules-win` plus six),
  six rewords and one `when:` clear, and the header names each. Its "six rewords" is the
  disclosed deviation (3), and the count matches the ops.
- anchor present where the exit requires one — PASS. All seven `supersede-rule` ops carry
  `2026-10-11 implement-brainstorm-led D3`, and `protected-exit` is clean. Judgment: D3 covers
  every exit. It replaces "each one's own copy of the asking rules … by the shared pointer line
  plus its domain part". The `own-rules-win` flip is the *Precedence* default, listed under
  *Supersessions owed*, and its disposition names both that default and D19.
- ID lifecycle right — PASS. Each tombstone's claimed lifecycle happened. `options-rule` and
  `vague-answer-reasked` are "verbatim" in the engine: their texts are identical character for
  character. `fact-before-decision`'s "every limb kept" holds, with only decision → fork
  changed (L5). `own-turn-or-batch`, `question-form` and `show-before-asking` land in
  `own-turn-or-batch`, `question-form` and `mock-in-message`, and every named carrier is live.
  The six rewords keep their ids. No surviving rule text or `enforces:` list names a superseded
  id (grep over `.mochiko/schema-views` and `plugins/mochiko`; 0 rejecting). Disclosed deviation
  (2), the `brainstorm.user-steers` reword, holds. Its pull-a-default limb is verbatim in
  `analysis-iterative.user-rules`, which holds in every turn, as "In any reply" did. The rule
  keeps map edit, next question, stop and the frame-change clause. Its prose cite of
  `analysis-iterative.user-rules` resolves to a live node, and command-side citation scanning
  does not reach skill ids, so this was confirmed by hand.
- floor and fail survival — PASS. None of the seven retired rules was a floor or a fail (all
  `class: must`). The four fails and their `enforces:` lists are untouched and resolve. The stub
  adds the D3 floor.
- register — PASS. The rewords carry only brainstorm's domain part, in the record's voice. The
  ontology call on `when: null`: under its old guard the rule would not bind a small session, so
  the engine's default (a list on every costly fork) would have reached small sessions, a gain
  no one ruled. Cleared, the rule binds at every size and states both arms of brainstorm's
  cadence slot, "none in a small one" included. No rule-level activation guard is left to MOVE:
  the size words are the slot's value, not a condition on whether the rule applies. Extracting
  them into `when:` would falsify the "none" arm. No condition-coverage finding moves.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 3 (schema content · 0060 + brainstorm view diff) · w1-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 4 — 0061, 0062 and their view diffs

VALIDATE: schema content — `plugins/mochiko/migrations/0061-specify-on-engine.yaml` and
`plugins/mochiko/migrations/0062-spec-review-boundary.yaml`, with the view diffs of
`.mochiko/schema-views/commands/specify.yaml` and
`.mochiko/schema-views/skills/review-specifications.yaml`.

Checklist run: `validation-primitive-edit.judgment-items-schema`, the AM-2 five, applied to each
migration with its view diff. Held against D3, the defaults *Slots declared where* (specify names
the contract on) and *Precedence*, *Supersessions owed* (specify-brainstorm-discovery
D5-specify-own-rules), D2 for 0062, the plan chain (rev2 §6.4 for `spec.slate-binds-authoring`)
and the disclosed deviations (4)–(6).

Evidence read: `plugins/mochiko/migrations/0061-specify-on-engine.yaml` and
`plugins/mochiko/migrations/0062-spec-review-boundary.yaml`, read whole · `git diff --
.mochiko/schema-views/commands/specify.yaml` and `git diff --
.mochiko/schema-views/skills/review-specifications.yaml`, read whole · the full specify render
(the story- and intent-stage rules, `spec.slate-home`, `spec.gate-slate-confirm`,
`spec.intent-stage-first`, `spec.fail.intent-unconfirmed`, `spec.fail.story-outside-slate`) · the
review-specifications render · the HEAD preambles of both, replayed · plan `w1-p1-plan.md` §6.4–§7
and `w1-p1-plan-rev2.md` §6.4 · `research/obligation-map.md` S1–S15 and the build-time table ·
`.mochiko/strips/review-specifications.md` (the v0.88.0 disjoint-triggers keep) · the specify
`vars:` block.

Pre-pass: the shared pre-pass above. Specify pins: `kind: fail · 10 rules` → 10 · `class: floor ·
18 rules` → 19 (`spec.thinking-stage` added, all eighteen kept by id). Sections 13 · 7 · 25 · 13 ·
3 · 10 (71) → 13 · 7 · 25 · 12 · 4 · 10 (71). Review-specifications pins `class: floor · 8 rules`
unchanged, sections unchanged at 1 · 6 · 7 · 12 · 6 · 1. Its payload grows +44, render only (see
the shared pre-pass).

- intent stated — PASS. 0061 lists one mint, one supersession and nine rewords (story-stage plus
  eight), and names the two rules left untouched. The ops match: `spec.intent-stage-first` and
  `spec.blind-journey-list` are absent from them. 0062 states its one reword and why: D2 strips
  the enrichment variant the line routed to.
- anchor present where the exit requires one — PASS. `spec.own-rules-win`'s supersession carries
  `… D3`, and its disposition names D5-specify-own-rules "as to this clause" and the *Precedence*
  default. Neither file has another exit. 0062's reword keeps `2026-08-05
  ponytail-concepts-integration`, and its authority is the header anchor plus D2. The v0.88.0
  strip's "disjoint-triggers boundary — survives" keep is honoured in substance: the pre-spec
  route survives, re-pointed. Only the "disjoint triggers by design" claim leaves, and that claim
  is about the description D2 rewrites.
- ID lifecycle right — PASS. `spec.own-rules-win` is tombstoned once, its carriers live. The ten
  rewords keep their ids. Each limb that left a reworded rule has an engine carrier, checked
  before-text against after-text (the obligation map's S rows agree):
  - `spec.lead-holds-story-stage`'s "yourself, inline … no teammate" goes to `lead-holds-conversation`.
  - `spec.analyst-brief-and-pen`'s transcription, pen, quoted words and exact-text limbs go to `contract-pen`.
  - `spec.slate-binds-authoring`'s meaning-kept and ask-before-writing limbs go to `contract-inside`, at rev2's text.
  - `spec.stage-inputs-in-hand`'s facts-not-re-asked and no-skip-with-nothing limbs go to `inputs-in-hand`.
  - `spec.intent-probe-discipline`'s streak and vague limbs go to `ratified-streak` and `vague-answer`.

  The specify-specific words stay put: "one-row" mock, the do-less fewer-or-none, the slate
  screen, journeys, the Story-cell position, `$ARGUMENTS`, no size gate. No `${var}` binding was
  dropped and no literal duplicates a declared var (the `vars:` block has no analyst-seat var).
  The disclosed deviations hold. (4) "before any authoring" restates `spec.intent-stage-first`'s
  order, so it adds no new obligation. (5) "the next writer's brief" is bound by
  `spec.story-stage` ("the requirements-analyst seat the next writer"), the binding D1's
  accepted risk requires. (6) the reshaped opener drops no clause.
- floor and fail survival — PASS. `spec.own-rules-win` was `class: must`, and no floor or fail is
  touched. `spec.fail.story-outside-slate` keeps `enforces: spec.story-stage,
  spec.slate-binds-authoring, spec.gate-slate-confirm`, all live. The limbs that left
  `slate-binds-authoring` are now a `class: floor` engine rule, switched on by `spec.story-stage`,
  which that fail enforces. "Never authored before ruled in" also stands in
  `spec.gate-slate-confirm` ("No story is authored before it"). Protection is raised, never
  lowered.
- register — PASS. Every reword is in specify's own voice, plain, with the contract named *on* in
  `spec.story-stage`'s text: a stub inherits text only, so the mode never rides the shared line.
  `spec.story-forks` keeps `when: slate_source=forks`. 0062's line keeps the not-for list's form.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 4 (schema content · 0061 + 0062 + specify and review-specifications view diffs) · w1-gate-grader · opus · 4 files · 1 rounds · 0 blocking

## Unit 8a — the allowlist diff

VALIDATE: gate file — the `scripts/similar-rules-allowlist.yaml` diff (one row and its comment
line removed).

Checklist run: no unit kind of the four fits a gate file. The nearest, the prose items
(`validation-primitive-edit.judgment-items-prose`: coherence and preserved responsibilities), was
used, plus the wave plan's P1 ownership line: rows only for edges the sweep reports, or rows naming
an id this wave retires.

Evidence read: `git diff -- scripts/similar-rules-allowlist.yaml`, read whole (3 lines deleted) · a
grep of `scripts/similar-rules-allowlist.yaml` for every retired id, every `analysis-iterative.*`
id and `thinking-stage` (no hit) · the two tombstones in the brainstorm and specify views ·
`crates/mochiko-cli/tests/matrix_similar.rs:1029–1041` and `:1185–1200` (pins that describe the
row, read for later units).

Pre-pass: the shared pre-pass above. Suppressed edges 185 → 184, clusters 0, no stale-row
finding.

- coherence — PASS. The removed row named `[brainstorm.own-rules-win, spec.own-rules-win]`, and
  both ids are now tombstoned by 0060 and 0061. A row naming dead ids would only warn stale. No
  remaining row names a retired id or a W1 mint, and the sweep is quiet without it, so no edge
  went unsuppressed.
- preserved responsibilities — PASS. The row's reason recorded a ruled drift allowance
  (D5-specify-own-rules, "drift allowed until then"). The thing it suppressed, two precedence
  clauses, has left the log by recorded supersession. The allowance has nothing left to cover.
  The removal is inside P1's write set.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 8a (gate file · similar-rules allowlist diff) · w1-gate-grader · opus · 1 files · 1 rounds · 0 blocking

## Shared pre-pass — units 5, 5b, 6 and 7

Round 1 for these units. Every command was run first-hand at the current tree, after P2's build
(P3 builds in parallel inside `crates/mochiko-cli/tests/**` and `evals/**`, none of these units'
files).

```
$ mochiko-cli migrate validate --report --plugin-root plugins/mochiko
rules scanned: 1211 · in-kind pairs scored: 205579 · clusters: 0 (none)
allowlist-suppressed edges: 184
mochiko-cli migrate validate · 0 rejecting · 115 advisory

$ mochiko-cli migrate status --plugin-root plugins/mochiko
log plugins/mochiko/migrations · grammar 2 · sequences 1..62 (60 migrations)
state sha256:a978b6d111f31c2399b8f6528395ed25fed40e72fef281810e51bd842e01d66a · 88 documents · 1211 rules

$ mochiko-cli views emit --plugin-root plugins/mochiko --out <scratch>/gg-views2.iJBt
$ diff -r <scratch>/gg-views2.iJBt .mochiko/schema-views
(no output: the committed views equal the replay)

$ mochiko-cli ids --check <file> --plugin-root plugins/mochiko   (each edited strip, the map, every edited prose file)
.mochiko/strips/analysis-iterative.md:152:3 · bare · D21
.mochiko/memory/primitive-cost-budgets.md:53:384 · bare · D1
(every other file: 0 bare · 0 drift)

$ mochiko-cli check --path <file> --content - --plugin-root plugins/mochiko < <file>   (the eight edited records)
"permissionDecision":"allow"   (all eight)
```

Char budget, by the ledger's canonical snippet. The render is the seven blocks, counted as
characters of stdout. The per-block figures reproduce the ledger's exactly.

| primitive | body | description | render | payload | budget | over |
|---|---|---|---|---|---|---|
| `analysis-iterative` | 4,222 → 5,376 | 476 → 594 (budget 595) | 10,691 | 16,067 | 4,928 prose, re-seeded 16,067 | +11,139 against the prose budget |
| `review-specifications` | 3,441 → 3,471 | 490 → 535 (budget 613) | 14,664 → 14,708 | 18,179 | 16,174 | +2,005 (standing +1,931, this wave +74) |
| router (unbudgeted) | 47,617 → 48,037 | 206 | — | — | — | — |

The engine's blocks: preamble 1,650 · independence 1,145 · scope 1,195 · inputs 1,017 · verdict
3,004 · output 1,212 · reserved 1,468, summing to 10,691. The command renders (unbudgeted), by the
same per-block method, HEAD copy → now:
- brainstorm: **21,198 → 19,074 bytes**, or 20,888 → 18,788 characters.
- specify: 28,405 → 27,560 bytes, or 27,999 → 27,160 characters.

## Unit 5 — the analysis-iterative skill pair

VALIDATE: skill pair — `plugins/mochiko/skills/analysis-iterative/SKILL.md` with its render
(preamble and six sections). Also graded: the deletions of `SPECIFICATION-INPUT.md` and
`ENRICHMENT.md`, the kept `SYNTHESIS.md` and `references/ADAPTIVE-EXAMPLES.md`, and the strip
`.mochiko/strips/analysis-iterative.md`. Preserved responsibilities were read against
`.mochiko/brainstorms/implement-brainstorm-led/research/obligation-map.md`.

Checklist run: `validation-primitive-edit.judgment-items-pair`, held to skill-pair criteria 1–12
of `.claude/rules/mochiko/primitive-edits.md`. The brief also names two checks: (i) delivery and
(ii) *Change control*.

Evidence read:
- `plugins/mochiko/skills/analysis-iterative/SKILL.md`, read whole, and its `git diff`. It equals
  the approved plan's §3.1 text byte for byte (a `diff` against the plan's fenced block).
- The seven rendered blocks.
- The strip's wave comment and eight [v0.120.0] entries, read whole.
- The obligation map: rows P1–P8 and every carrier cell.
- `SYNTHESIS.md`, read whole, and `references/ADAPTIVE-EXAMPLES.md`, read by a structure grep.
  `git diff --stat` is empty for both.
- The two deleted files at HEAD, measured at 4,763 and 2,403 characters, as the strip says.
- `plugins/mochiko/hooks/scripts/dependency-halt.sh`, read whole.
- The brainstorm, specify and setup renders.
- The ledger's `analysis-iterative` rows and its [v0.120.0] block.
- The record's D2 *Accepted risk*, the *Touch set* and *What analysis-iterative keeps*.

Pre-pass: the shared pre-pass above. The render pins `class: floor · 2 rules`; its `floors:`
line lists `analysis-iterative.open-unknowns` and `analysis-iterative.contract-inside`.

- scaffold headings and order — PASS.
  - The frontmatter has `name`, `description` and `allowed-tools: Bash(mochiko-cli *)`.
  - The headings run in order: the title, Overview, `## Rules — delivered by mochiko-cli`, When
    NOT to Use, Confidence marks, Common Mistakes, Reference.
  - The Rules block matches `analysis-codebase`'s byte for byte once the name is swapped
    (`diff`), the no-raw-Read clause included.
  - The seven `!` lines name `preamble` and then the six section ids, in the preamble's printed
    order.
  - The read-back sentence cites the `class: floor` pin and the `floors:` line and hard-codes no
    count.
  - No `analysis-iterative.sec.*` token appears outside the `!` lines.
- preserved responsibilities — PASS.
  - Every removed span has a [v0.120.0] supersession entry holding its verbatim content, or for
    the two deleted files a faithful compression with the path pinned. The spans are: the
    enrichment variant and its two files, `## Two output shapes`, `## Output`, the title and
    Overview, two When NOT rows, two Common Mistakes rows, the Reference line and the
    `description:`.
  - The protected content is reconciled: both [v0.25.0] `KEPT:` entries, and the [v0.63.0]
    keep-set member by member. The [v0.117.0] brainstorm-only supersession ends with
    `brainstorm.own-rules-win` tombstoned.
  - Map rows P1–P8 each hold. P1, the Overview's adaptation sentences, is kept. P2, P6 and P7
    are reworded by ruling (V5, S3). P3 and P4 now sit in the rules. P5 and P8, the table and
    five rows, are byte-identical.
  - The deleted files had no consumer. A grep for both file names over `plugins/mochiko` (the
    log aside), the crate's `src` and `tests`, `evals/contract`, `scripts`, `.github`,
    `README.md`, `ARCHITECTURE.md` and `CLAUDE.md` finds nothing.
- floor survival — PASS. The prose floor at HEAD `SKILL.md:24` is
  `analysis-iterative.open-unknowns`, with its origin anchor, delivered in `sec.output`.
  `contract-inside` is raised to a floor. No floor left unrecorded.
- independence, no seat grading its own row — PASS. The skill grades nothing.
  `lead-holds-conversation` keeps "the caller's own independent leg still runs".
- reserved-to-user content in the reserved section — PASS. `user-rules` (`kind: reservation`)
  sits in `sec.reserved`, beside the contract pair.
- the matching done-condition branch — n/a. A skill carries no done-condition branch; that is
  the command scaffold's criterion 7.
- argued overage — PASS: the argument holds.
  - The payload is 16,067, measured: body 5,376 plus render 10,691.
  - The rule text is 5,133 characters (validate's figure). Of that, 280 are the two relocated
    Output lines (131 + 149); the other 4,853 are D1's rules and the confirmed defaults.
  - Another 5,558 is the delivery format every converted skill pays.
  - The body arithmetic holds: 4,222 + 2,072 − 1,662 + 744 = 5,376.
  - The 4,853 put in one home what brainstorm and specify carried in their own words; their
    renders shrink by 2,124 bytes and 839 characters (same method). The rest is the gains the
    user ruled "Accept all".
  - No playbook prose is restored. The four format templates stay out, per the strip's `KEPT:`
    reconciliation. The re-seed takes no headroom, the ledger's third path.
  - The ledger paragraph quotes brainstorm's shrink from a mixed-method figure. That is unit 7's
    fix, and the argument holds on the correct figure.
- `description:` (criterion 7) — PASS by ruling. It is 594 characters, against a 595 budget and
  the 1,536 cap. It is not byte-identical, and D2 covers the change: its *Accepted risk* reads
  "The skill's `description:` and the router's lines … are rewritten around the stage", and the
  *Touch set* lists "`description:` rewritten". The old value advertised the stripped triad. The
  strip records the supersession, and the MUST/SHOULD grading and trigger phrases are kept.
- (i) delivery — PASS.
  - `brainstorm.thinking-stage`, `brainstorm.lead-inline-questioning` and `spec.thinking-stage`
    each render `pointer: mochiko:analysis-iterative`.
  - `spec.story-stage`'s text names "the thinking stage with its ruled-set contract on".
  - The name pointer resolves, checked by hand: `plugins/mochiko/skills/analysis-iterative/SKILL.md`
    carries `name: analysis-iterative`.
  - The seven `!` lines were run verbatim, with `CLAUDE_PLUGIN_ROOT` set to the tree's plugin.
    Each exits 0. They print seven open lines and end lines 0 · 2 · 2 · 2 · 9 · 3 · 3, with 21
    `###` rule headings.
  - The hook, given a simulated `PreToolUse`/`Skill` payload for `mochiko:analysis-iterative`,
    returns `"mochiko-cli present · rules delivered by the skill's own render"`. With the binary
    off `PATH` it returns `deny` ("mochiko-cli is not installed …"). At the HEAD copy it exits
    silent, since the skill was prose.
  - The hook renders nothing, by design: its header says it "confirms presence in one line and
    stops". The 21 rules therefore reach the model through the slot, and the hook gates on their
    absence.
  - The map names 20 engine ids as carriers, and all 20 are among the 21 delivered. The 21st,
    `caller-slots`, is named by no row.
- (ii) Change control — PASS.
  - Re-read: brainstorm's whole render; specify's story-stage, intent-stage and PM-frame rules;
    setup's `interrogation-inline`, `interrogation-inputs`, `user-card-rulings`,
    `coverage-survivor-routing`, `gate-synthesis-ratification`, `synthesis-artifact`,
    `stress-test-cold-seat` and `register`.
  - No caller rule contradicts an engine rule. Setup's card-by-card deck is the user's Q2 slot
    fill. `spec.capability-frame-at-intent`'s frame is "agreed with the user" through the lead
    (map G21).
  - Setup's render is byte-identical to HEAD.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 5 (skill pair · analysis-iterative) · w1-gate-grader · opus · 7 files · 1 rounds · 0 blocking

## Unit 5b — the review-specifications skill pair

VALIDATE: skill pair — `plugins/mochiko/skills/review-specifications/SKILL.md` (lines 3 and
11–12) with its render, and the strip `.mochiko/strips/review-specifications.md` [v0.120.0].

Checklist run: `validation-primitive-edit.judgment-items-pair`, held to skill-pair criteria 1–12.

Evidence read:
- `plugins/mochiko/skills/review-specifications/SKILL.md`, read whole, and its `git diff`.
- The render, preamble and six sections, with the `not-for` rule `0062` rewords (unit 4).
- The strip's wave comment and new entry, with the earlier [v0.63.0] and [v0.88.0] keep notes.
- The ledger's body and description rows.
- The plan's §3.4 and §5.5.

Pre-pass: the shared pre-pass above.

- scaffold headings and order — PASS. Nothing changed outside lines 3 and 11–12. The Rules block
  and its seven `!` lines are untouched.
- preserved responsibilities — PASS. The sibling boundary, kept at [v0.63.0] and [v0.88.0],
  survives repointed. Both old texts are verbatim in the entry. The body, the description and
  `not-for` now agree.
- floor survival — PASS. The pin reads `class: floor · 8 rules`, with the same floor ids as HEAD.
- independence, no seat grading its own row — PASS. Unchanged: the `author-grader` and
  `input-not-verdict` floors.
- reserved-to-user content in the reserved section — PASS. `sec.reserved` is unchanged, one rule.
- the matching done-condition branch — n/a (a skill).
- argued overage — PASS: the argument holds.
  - The payload is 18,179 (body 3,471 + render 14,708) against the 16,174 budget.
  - The standing +1,931 was ruled HOLDS at v0.119.0.
  - This wave adds +74: render +44 from 0062 and body +30.
  - Both are the one boundary D2 forces off a stripped target onto the live route. There is no
    new playbook prose, and the description is 535 against its 613 budget.
- `description:` (criterion 7) — PASS by ruling. This is not a conversion. The value's last
  sentence routed to the variant D2 strips (F27), so D2 made it false, and the strip records the
  repoint.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 5b (skill pair · review-specifications) · w1-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 6 — the brainstorm and specify command pairs

VALIDATE: command pairs — `plugins/mochiko/commands/brainstorm.md` and
`plugins/mochiko/commands/specify.md`, each with its render. Both `.md`s are unchanged this wave
(`git diff --stat -- plugins/mochiko/commands/` is empty), so they are graded as they stand after
0060 and 0061.

Checklist run: `validation-primitive-edit.judgment-items-pair`, held to the canonical-scaffold
criteria 1–11, with criterion 7's run-command branch.

Evidence read: both `.md`s, read whole · both renders, read whole (re-read this round, beside
units 3 and 4) · both preambles' pins.

- scaffold headings and order — PASS, both.
  - Each frontmatter has `description`, `argument-hint`, `disable-model-invocation: true` and
    `allowed-tools: Bash(mochiko-cli *)`.
  - Each runs `# <Name> — <epithet>`, `## Identity & Mission`, `## Rules — delivered by
    mochiko-cli`, then `## Adaptive Goal Protocol` with Entry, Goal and Not done last.
  - Each has seven `!` lines, matching the six-set section ids set-wise.
  - Not done cites the `kind: fail` pin and carries the halt clause, with no hard-coded count.
- preserved responsibilities — PASS. No `.md` line is made false.
  - `brainstorm.md`'s "You run the questioning yourself, inline" agrees with
    `analysis-iterative.lead-holds-conversation`, reached through the floor stub.
  - Its Goal's confidence marks are brainstorm's card form, set beside the engine's provenance
    marks.
  - `specify.md`'s "the story set is ruled with the user into a confirmed slate before any story
    is authored" agrees with `spec.story-stage` and `spec.gate-slate-confirm`.
  - Every obligation the renders gave up is carried by the engine (units 3 and 4, and unit 5's
    delivery check).
- floor survival — PASS. Brainstorm goes from 8 to 9 floors and specify from 18 to 19, with every
  prior id kept. The fail pins read 4 and 10, and each fail-conditions end line equals its pin.
- independence, no seat grading its own row — PASS.
  - Brainstorm: `record-review-independence`, `author-grader-default-fail` and
    `plan-approval-producers`.
  - Specify: `author-grader-default-fail` and `fail.self-graded`.
- reserved-to-user content in the reserved section — PASS.
  - `brainstorm.sec.reserved` has 8 rules. `user-steers` cites `analysis-iterative.user-rules`
    for the moved pull-a-default limb, kept by the user's "Keep all".
  - `spec.sec.reserved` has 7 rules, unchanged.
- the matching done-condition branch — PASS. Both are run commands with a fixed done condition.
  Entry takes `$ARGUMENTS` and the empty case, Goal is fixed, and Not done defaults to FAIL,
  count-pinned. The scaffold demands no sound-loop pointer on these two.
- argued overage — n/a: commands are unbudgeted.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 6 (command pairs · brainstorm, specify) · w1-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 7 — router, agenda, KM template, strips, ripple, ledger, changelog

VALIDATE: prose primitives and records:
- the router `plugins/mochiko/skills/mochiko/SKILL.md` (:66, :124);
- `plugins/mochiko/skills/authoring-constitution/references/INTERROGATION-AGENDA.md` (:39, :40,
  :44–46, :165–167);
- `plugins/mochiko/templates/constitution-modules/knowledge-management.md` (:5);
- the strips `.mochiko/strips/mochiko.md`, `authoring-constitution.md` and
  `knowledge-management-module.md`;
- the ripple in `ARCHITECTURE.md` (:62, :372);
- `.mochiko/memory/primitive-cost-budgets.md`;
- `CHANGELOG.md` `## [0.120.0]`, the W1 part.

Checklist run: `validation-primitive-edit.judgment-items-prose` (coherence, preserved
responsibilities). Every figure in the ledger and the changelog was re-derived first-hand.

Evidence read:
- Each file's `git diff`, read whole.
- The agenda's lines 1–30 and its dimension table, and the KM template's head.
- The three strip entries and their wave comments, read whole.
- The ledger: rows 53 and 63, the two description rows and the [v0.120.0] block.
- The `CHANGELOG.md` entry, read whole.
- The plan's §3.3–§3.6, §4.2–§4.4 and §5–§6.
- The build log's 2026-10-11 entries (Q5, "Keep all").
- `DECISIONS.md`: row 13 and the 2026-07-17 row.
- The skill census: 43 directories, 36 with `!` lines, 6 prose plus the router.

Pre-pass: the shared pre-pass above.

- coherence — FAIL.
  - Correct as built:
    - Router :66 and :124 equal the plan text.
    - The agenda's Q5 lines follow the user's two rulings: "default-on" kept as the offer's name,
      with the yes clause after it.
    - Agenda :165–167 points at the engine's asking rules.
    - KM template :5 is the Q-B widening.
    - `ARCHITECTURE.md` :62's counts check out (43 · 42 · 36 · 6), and :372 is right.
    - The ledger's two rows and two description rows reproduce.
    - In the [v0.120.0] block, the per-block figures, the 280, the 5,558 and the body arithmetic
      all reproduce.
    - The changelog's other claims reproduce: 21 rules · seven supersessions and six rewords ·
      nine specify rewords · 1,196 → 1,211 · the pins · 13 strip entries (8 + 1 + 1 + 2 + 1) ·
      the `ARCHITECTURE.md` counts · the budgets · specify 27,999 → 27,160 characters.
  - Two defects:
    - (a) The ledger's [v0.120.0] block and the changelog's W1 budgets paragraph both say
      "brainstorm 21,472 → 19,074 bytes". That pairs two measuring methods. 21,472 is the
      build-open figure, taken by another method; the P1 plan's §4 says "the build log's figures
      run ~240–270 bytes higher by another method", and P2's own §5.2 marks it "lead's baseline".
      The per-block loop that gives 19,074 gives 21,198 at the HEAD copy, so the shrink is −2,124
      bytes, not −2,398.
    - (b) Ledger row 53 cites "`implement-brainstorm-led` D1 obligations" bare (`ids --check`:
      `:53:384 · bare · D1`), while every other cite in the row and the block carries its slug.
- preserved responsibilities — PASS.
  - Each removed line has a [v0.120.0] entry with its verbatim text: the router's two lines, the
    agenda's active-decline clause and format pointer, and the KM header.
  - The active-decline clause is protected: it traces to `setup-operating-docs-scaffolding`
    D2-elective-default-module (`DECISIONS.md` 2026-07-17). It leaves on the user's own Q5
    ruling, recorded in the build log and cited in both entries.
  - Kept: "default-on" as the name (the user's ruling), "recorded either way", and core taken
    whole.
  - Each entry assesses its consumers. `ARCHITECTURE.md` is hand-maintained and owes no strip.

VERDICT: FAIL

Issues requiring fix:
1. coherence — the ledger's [v0.120.0] block (`.mochiko/memory/primitive-cost-budgets.md`) and
   the `CHANGELOG.md` `## [0.120.0]` W1 budgets paragraph pair brainstorm's build-open render
   figure, 21,472 (another method), with the per-block after-figure, 19,074. Fix: in both files,
   replace "brainstorm 21,472 → 19,074 bytes" (the changelog's "`brainstorm`'s render 21,472 →
   19,074 bytes") with the same-method pair "21,198 → 19,074 bytes". Characters would also do,
   to match specify's unit: "20,888 → 18,788 characters".
2. coherence — ledger row 53 cites "`implement-brainstorm-led` D1 obligations" bare. Fix: write
   "`implement-brainstorm-led` D1-engine-asking-contract obligations", then re-run
   `mochiko-cli ids --check .mochiko/memory/primitive-cost-budgets.md --plugin-root
   plugins/mochiko` to 0 bare.

audit: unit 7 (prose primitives and records · router, agenda, KM template, three strips, ARCHITECTURE, budget ledger, CHANGELOG) · w1-gate-grader · opus · 9 files · 1 rounds · 2 blocking

## Unit 8 — crate tests, contract expectations and the plan kits

VALIDATE: P3's 13 files:
- the crate tests `crates/mochiko-cli/tests/{fidelity,validate,render,views,matrix_similar}.rs`;
- the contract expectations `evals/contract/{run.py,expected-skills.json,README.md}`;
- the plan kits `evals/plan/{brainstorm,specify}/{observable.yaml,preregistration.md}` and
  `evals/plan/README.md`.

This is no plugin primitive. It is graded two ways: as the independent non-author code review
that `.claude/rules/mochiko/rust-cli.md` requires for a crate unit, and under the gate's evidence
standard (default FAIL, binary verdict).

Checklist run: the rust-cli rule (bright line, quality gate, lead-approved plan, non-author
review), the approved plan's write set, figures and stops, and `validation-primitive-edit`'s
evidence floors.

Evidence read:
- `git diff -- crates evals`, read whole: all 13 files, 854 lines.
- The approved plan, read whole: `w1-p3-plan.md` (sha256 `47c907d6…72c5ff`) and
  `w1-p3-plan-rev1.md` (`13debd4f…eaea42`). Both hashes match the build log's pin.
- The build log's P3 entries: round 2 PASS with the lead's §8 rulings, and P3 build landed.
- The derived views, tallied by kind at `HEAD` and in the working tree.
- `skills/analysis-iterative.yaml`, and the reworded rules' texts in `commands/brainstorm.yaml`
  and `commands/specify.yaml`.
- `git show 2e4c57c:` of the brainstorm view.
- The `PROBE_ARGUMENTS` table in `run.py`, `evals/plan/specify/preregistration.md:70–92`, and
  the brainstorm preregistration's fill log.

Pre-pass, run first-hand by this seat from the repo root:
- `cargo test --all` — rc 0 · 765 passed · 0 failed · 0 ignored, 20 suites.
- `MOCHIKO_FULL_SIMILAR=1 cargo test -p mochiko-cli --test matrix_similar` — `test result: ok. 48
  passed; 0 failed; … finished in 228.30s`.
- `cargo fmt --all --check` — rc 0, no output.
- `cargo clippy --all-targets -- -D warnings` — rc 0, `Finished`.
- `cargo audit --deny warnings` — rc 0, `Scanning Cargo.lock for vulnerabilities (31 crate
  dependencies)`, nothing reported.
- `git diff --stat -- crates/mochiko-cli/src crates/mochiko-cli/Cargo.toml Cargo.lock` — empty.
- `PYTHONDONTWRITEBYTECODE=1 python3 evals/contract/run.py --case converted-shape` — `contract
  suite: 1/1 cases passed`, including "the pre-registered floor set matches the
  analysis-iterative render (2 ids)" and its brainstorm (9 ids) and specify (19 ids) twins.
  Filtered, not a gate run.
- `… run.py --host-only` — `contract suite: 7/7 cases passed, 7 ran`. Filtered, not a gate run.
- `… run.py --list` — 99 cases, `analysis-iterative-delivery` and `analysis-iterative-absence`
  among them.
- `uv run evals/run.py command check-rubric brainstorm` — `rubric OK: 40 observable, 9
  out-of-instrument, 49 total`.
- `… check-rubric specify` — `rubric OK: 57 observable, 14 out-of-instrument, 71 total`.
- `check-fixtures brainstorm` and `… specify` — `fixture consistency: OK`, both.
- `python3 -c 'import json;json.load(open("evals/contract/expected-skills.json"))'` — parses.
- `… partition brainstorm --old-ref 2e4c57c` gives 25 · 5 · 0 · 19, and `… partition specify
  --old-ref 32c1ed5` gives 44 · 7 · 0 · 20.
- `mochiko-cli migrate validate --report --log-dir`, run raw over allowlist-free copies of both
  logs:
  - the `HEAD` log: `clusters: 78` · `allowlist: none (185 edges unsuppressed)`;
  - the current log: `clusters: 77` · `allowlist: none (184 edges unsuppressed)`.

Judgment items:

- plan conformance — PASS.
  - The tree's 13 entries are exactly W1–W13, with W13 as the lead's Q3 widening.
  - There is no change to `src/`, `Cargo.toml` or `Cargo.lock`, and none to a fixture,
    `evals.json` or `freeze_expectations.py`.
- bright line — PASS. The edits are test-only: figures, sequence lists and comments. No crate
  behaviour moves, and no judgment enters the crate.
- quality gate — PASS. The four crate layers and the opt-in full sweep are green, all run
  first-hand above.
- crate figures and comments — PASS. Each figure was re-derived from a tool, not from the plan.
  - Views by kind, `HEAD` to now: command rules 381 → 375 (`constraint` 170 → 165, `duty`
    34 → 33).
  - Corpus rules 1,196 → 1,211 (`constraint` 570 → 577, `duty` 149 → 153, `binding` 169 → 171,
    `bound` 25 → 26, `reservation` 63 → 64).
  - Both `matrix_similar.rs` pair sums close: −835 − 33 − 1 = −869, and +4,010 + 602 + 339 + 25
    + 63 = +5,039.
  - `analysis-iterative` carries 21 rules, 2 of them floors (`open-unknowns`,
    `contract-inside`). Its one path pointer, from `analysis-iterative.deliverable` to
    `SYNTHESIS.md`, is `validate.rs`'s 89th.
  - The eight superseded rules are all `class: must`: seven `constraint`, plus
    `brainstorm.show-before-asking` as `duty`. Both stubs are `class: floor` and `constraint`,
    and both extend `common.thinking-stage`.
  - The ops count 6 + 9 + 1 rewords, 7 + 1 supersessions, and one `set-rule-field` on `when` in
    0060.
  - Raw clusters went from 78 to 77, and the membership diff drops only
    `brainstorm.own-rules-win` and `spec.own-rules-win`. This settles deviation (3) first-hand.
- contract expectations — PASS.
  - `EXPECTED` gains `brainstorm.thinking-stage` and `spec.thinking-stage`, and `converted-shape`
    matches both renders.
  - `expected-skills.json`, checked against `HEAD`'s parse:
    - `git diff -U0` shows 3 hunks;
    - every existing row and every other family is equal, and the provenance is extended, never
      rewritten;
    - the members list and the `skills` keys are sorted;
    - the new row carries `review-design-audit`'s post-freeze key set, zeroed byte columns (Q5)
      and the review family (Q2), cited to D2-iterative-becomes-engine.

    This settles deviation (1).
  - `PROBE_ARGUMENTS` agrees with the engine's Overview ("a question thought through with the
    user") and with `analysis-iterative.deliverable` ("when it names none, generate the
    synthesis"). This settles deviation (2).
  - The README's figures check out:
    - 99 cases agree with `--list`;
    - 178 sessions is 174 + 4, and 179 metered is 175 + 4;
    - the seven schema-less skills, the router plus six prose, agree with the directory census
      (43 directories, 36 converted);
    - the v0.120.0 stamp is Q4's.
- plan kits re-key — PASS.
  - Both rubrics and both fixture checks are OK.
  - Each re-cut `why` matches its rule's new view text. The whys left alone still hold:
    `decision-map`, `spec.story-stage`, `spec.stage-inputs-in-hand` ("there is no size gate")
    and `spec.intent-probe-discipline`.
  - The specify foot comment's ten facets are the ten it lists.
  - The brainstorm partition intersected with `observable` gives 18 · 4 · 0 · 18, as written.
  - So 3 × 40 gives 120 pairs, and the 15 % cap is ≤ 18 of them. For specify, 3 × 57 gives 171.
  - Specify's control is untouched: the removed bucket at `32c1ed5` is 0.
- Q1, the preregistration amendment — PASS. It keeps the kit's retirement rule honest.
  - The rule says "that edit's added or removed rules become the control".
  - The amendment declines to apply it, and says so in three places: the edit-under-read
    paragraph, the control paragraph and a new dated fill-log line. That line cites a ruling
    that exists (build log, "W1 P3 plan round 2", Q1 option A).
  - Its premise is true.
    - At `2e4c57c`, `brainstorm.lead-inline-questioning` already reads "inline, via
      mochiko:analysis-iterative", with `pointer: mochiko:analysis-iterative`, so the stub
      cannot read `absent` in `pre`.
    - The seven moved rules sit in no bucket: the removed bucket is empty.
    - `frame-card` is absent at that ref and sits in `added`, so it still discriminates.
  - It lands before any grid: there is no `runs/` in either kit.
  - Earlier fill-log lines are kept, and the re-key count stays 0 on the stopping rule's own
    terms.
  - Its cost, one grid reading 0044, 0047 and 0060 together, is disclosed.
- deviations (4) and (5) — PASS.
  - (4) is cosmetic. Seven added README lines and five added brainstorm-preregistration lines
    run past 100 characters. Both files already carried such lines (37 and 4), and nothing
    gates on line length.
  - (5): `cargo audit` was run here and is clean. The sandbox suite is still owed at the bump
    close, where a SKIPPED suite blocks.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 8 (crate tests, contract expectations, plan kits · crate non-author review) · w1-gate-grader · opus · 13 files · 1 rounds · 0 blocking

## Unit 7, round 2 — the fix re-read

VALIDATE: unit 7's fix, read only where it touched and where it could break something:
- `.mochiko/memory/primitive-cost-budgets.md`: row 53 and the [v0.120.0] block (:276–286);
- `CHANGELOG.md`: the `## [0.120.0]` W1 budgets paragraph (:62–67);
- beyond the fix order, `.mochiko/strips/analysis-iterative.md:151–152`, a unit-5 file.

Checklist run: `validation-primitive-edit.judgment-items-prose` (coherence, preserved
responsibilities), limited to round 1's two issues and the one extra edit.

Evidence read:
- Ledger row 53 and lines 276–286, read whole.
- `CHANGELOG.md` lines 62–70.
- The strip's [v0.120.0] `## Output` entry (:125–156).
- The `analysis-iterative.deliverable` anchor in `.mochiko/schema-views/skills/analysis-iterative.yaml`.
- `.mochiko/brainstorms/brainstorm-target-state/record.md:619`.

Pre-pass, run first-hand by this seat:
- `mochiko-cli ids --check .mochiko/memory/primitive-cost-budgets.md CHANGELOG.md
  .mochiko/strips/analysis-iterative.md --plugin-root plugins/mochiko` — `0 bare · 0 drift`.
- The per-block render loop (the preamble plus six sections), run with `--plugin-root` at the
  `git archive HEAD` copy and then at `plugins/mochiko`:
  - brainstorm: `21198` bytes / `20888` chars before, `19074` / `18788` after;
  - specify: `28405` / `27999` before, `27560` / `27160` after.
- `grep -rnE '2,398|21,472'` over the ledger, `CHANGELOG.md`, `.mochiko/strips/`,
  `ARCHITECTURE.md` and `plugins/mochiko` — no match (rc 1).
- `mochiko-cli check --path <file> --content -` on each of the three files — `allow`, all three.

Judgment items:

- issue 1 — fixed.
  - Ledger :281–282 and `CHANGELOG.md`:66 now read "21,198 → 19,074 bytes", a pair taken by one
    method.
  - Specify's before-figure, 27,999 characters, is `HEAD`'s per-block figure, as the producer
    said.
  - No figure derived from the old 21,472 survives.
- issue 2 — fixed. Row 53 reads "`implement-brainstorm-led` D1-engine-asking-contract
  obligations", and `ids --check` reports 0 bare.
- the extra strip edit — PASS, and unit 5's block stands.
  - The edit sits in this wave's own [v0.120.0] entry, which `HEAD` does not carry; no shipped
    entry was touched.
  - The log's anchor is unchanged: `analysis-iterative.deliverable` still carries
    `2026-10-06 brainstorm-target-state D21`.
  - The record's :619 gives D21's slug as D21-questioning-skill-repairs, so the line names the
    same anchor. The deliverable line's protected status still rides that anchor.
  - The only loss is the literal anchor value, which is no longer quoted. The log carries it, so
    the edit stays reconstructible (GI-006).
- coherence — PASS. Both of round 1's defects are gone, and nothing the fix touched contradicts
  another figure.
- preserved responsibilities — PASS. The fix removes nothing; round 1's reading stands.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 7 (prose primitives and records · router, agenda, KM template, three strips, ARCHITECTURE, budget ledger, CHANGELOG) · w1-gate-grader · opus · 9 files · 2 rounds · 0 blocking

## Failure narrative

Unit 7 failed on two record defects; none is in a shipped primitive.

The ledger's [v0.120.0] block and the CHANGELOG's W1 budgets paragraph give brainstorm's render
shrink as 21,472 → 19,074 bytes. The before-figure is the build-open number, taken by a method
the P1 plan's §4 records as running about 240–270 bytes high. The P2 plan's §5.2 carried it as
"lead's baseline", and the build copied it without re-measuring. Replaying the plan's own §5.1
per-block loop on a `git archive HEAD` copy gives 21,198 bytes (20,888 characters): the true
shrink is −2,124 bytes. Unit 5's overage argument holds on that figure.

Separately, row 53's new text cites D1 without its slug. Both were fixed as text only; round 2
re-read the touched files, and unit 7 now passes.

## Notes of note

- Owed later, the budget: `review-specifications` +44 render (0062) over a standing +1,931 HOLDS. §4 lists no review-specifications pair unit, and P2 edits its SKILL.md lines 3 and 11. The argued overage must be named and graded at whichever unit carries that pair.
- Owed at unit 5, delivery: the obligations units 3–4 moved reach brainstorm and specify only through the engine's render. `analysis-iterative/SKILL.md` has no `!` lines yet, so unit 5 must confirm they land.
- Owed before the bump, under the D3 reservation: the map's "not shown" items reach the user. These are L15, L16, the L10 and L11 widenings, and the build-time table: the `user-steers` reword, "the next writer's brief" and the two specify rewords. This is the user's to rule; it is not a gate item.
- Watch, `common.thinking-stage`: it has two extenders at W1. The 3+ bar is met only once the W2 desk stub and the W3 implement stub land before the bump.
- Advisory, the `contract` note: it reads per command ("the calling command's own rule text names … on"), while specify names it on for its story stage only. `spec.story-stage` scopes it, so this is no defect today. A later engine edit could say "for the stage it runs".
- Unit 8, discharged: the `matrix_similar.rs` pins (57 command-family and 184 corpus suppressed edges) and both kits' `observable.yaml` rows are re-keyed, and every figure was re-derived first-hand.
- Units 5–7, discharged: carry-forward (b), delivery, at unit 5. The D3 reservation went to the user, who answered "Keep all" (build log 2026-10-11).
- Unit 5's strip line, `.mochiko/strips/analysis-iterative.md:151–152`, was reworded in unit 7's fix round to reach 0 bare. It still names the same D21 anchor (judged in unit 7, round 2), so unit 5's PASS stands.
- Watch for W2/W3: the engine's description and Overview, router :66 and `ARCHITECTURE.md` :372 already name the desk's focus stage and implement's design stage as callers. They are true only once those waves land, before the bump.
- Owed before the bump, already known (Q-D and Q-C): the counts at `primitive-edits.md:75` and `:241–242`, the `dependency-halt.sh:76` comment (it still says "the seven prose skills"; there are now six), `BACKLOG.md:158`, `ARCHITECTURE.md:128`, and the agenda's "one question per turn" at :5 and :27.
- The build log's line 176, the lead's entry, carries the same mixed brainstorm figure as unit 7's issue 1.
- Owed at the bump close, from unit 8's deviation (5): the sandbox contract suite. A SKIPPED suite blocks the bump. `cargo audit --deny warnings` already ran here: rc 0, 31 crate dependencies.
- Advisory, not held (unit 8): `evals/plan/specify/preregistration.md:75–77` still gives the authoring-time buckets (51 · 0 · 0 · 1). Today's `partition specify --old-ref 32c1ed5` gives 44 · 7 · 0 · 20, with `spec.thinking-stage` in `added`. The kit says every grid recomputes, and the stale figure predates this wave.
