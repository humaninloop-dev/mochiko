---
report: review
round: 1
grader: w2-gate-grader
tier: opus
wave: 2
units: [1, 2, 3, 4, 5, 6, 7, 8]
---

## Shared pre-pass

This seat ran every command below itself, from the repo root, on `engine-follow-up-build` at HEAD
`fccae34`, with the wave's writes uncommitted: 24 porcelain lines (20 modified paths, the four new
migrations untracked). The same 24 lines stood after grading; this seat wrote only under its
scratchpad and this report.

Contract: `mochiko-cli rules validation-primitive-edit`, the preamble and all six sections
(`sec.independence` · `scope` · `inputs` · `verdict` · `output` · `reserved`), rendered first-hand
with `--plugin-root plugins/mochiko`. Every block carried its opening line
(`mochiko-cli rules validation-primitive-edit · section <id> · binary 0.4.0 · grammar 2 · plugin
0.120.0`) and its end line. Floor read-back: `class: floor · 11 rules`, namely `author-grader` ·
`plain-seat-explicit-tier` · `rendered-contract-only` · `from-file-floor` · `pre-pass-first-hand` ·
`binary-verdict` · `default-fail` · `tamper-proof-clause` · `gate-loop-bound` · `evidence-floor` ·
`second-fail-user`. The path rule `.claude/rules/mochiko/primitive-edits.md` was read whole.

Baseline: `git archive fccae34 plugins/mochiko scripts/similar-rules-allowlist.yaml`, extracted to
the scratchpad and replayed with `--log-dir` (the allowlist resolves by walking up from the log
directory, `similar.rs:981`, so the pre-wave run used the pre-wave allowlist).

- `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` (exit 0):
  `mochiko-cli migrate validate · 0 rejecting · 118 advisory` · `rules scanned: 1260 · in-kind pairs
  scored: 217663 · clusters: 0 (none)` · `allowlist-suppressed edges: 185` · `pointer resolution: 89
  checked against plugins/mochiko`. Pre-wave replay: `0 rejecting · 118 advisory`, `rules scanned:
  1254 · in-kind pairs scored: 216338 · clusters: 0`, `allowlist-suppressed edges: 186`. The only
  non-budget advisory line that moved is implement's `enforces-coverage`, 33 → 34 floor/gate rules,
  the addition being `impl.fail.cards-outside-order` (a fail node, listed like the other sixteen).
  Advisory `budget` lines (rule text only, never the ledger comparison): implement `125 rules ·
  42183` → `131 rules · 47861` · review-sufficiency `7745` → `8001` · testing-gap-finding `8095` →
  `8424`.
- `mochiko-cli migrate status --plugin-root plugins/mochiko`: `log plugins/mochiko/migrations ·
  grammar 2 · sequences 1..95 (85 migrations)` · `state
  sha256:ea8e4a6c1842f8d2c7963cae2e47878e139d23c41a3cceabf7a7896b7f57d46e · 94 documents · 1260
  rules`. Pre-wave: `sequences 1..89 (81 migrations)` · `state sha256:5ee3c452… · 94 documents ·
  1254 rules`. Sequences 0090–0091 are an unused gap.
- Views ≡ replay: `mochiko-cli views emit · 94 documents · <scratch>/views`, then `diff -r
  .mochiko/schema-views <scratch>/views` printed nothing, exit 0. `git diff --stat fccae34 --
  .mochiko/schema-views` lists three views: `commands/implement.yaml`,
  `skills/review-sufficiency.yaml`, `skills/testing-gap-finding.yaml`. The engine render
  (`analysis-iterative`, preamble and six sections) is byte-identical pre and post, and the common
  view is unchanged (default 19).
- Allowlist necessity: the post-wave log validated against the pre-wave allowlist reports `[1]
  CROSS-PAIR · kind: fail · best 0.71` (`feat.fail.card-outside-set` · `impl.fail.cards-outside-order`),
  `allowlist-suppressed edges: 184`; with the new row, 0 clusters and 185.
- Char budgets, the ledger's canonical snippet for the body plus the characters of each block the
  seven `!` lines render: `review-sufficiency` body 3,262 + render 14,352 = **17,614** (pre 17,360;
  preamble 1,827 · independence 514 · scope 1,839 · inputs 2,225 · verdict 5,726 · output 1,200 ·
  reserved 1,021), budget 15,423, **+2,191**, description 686. `testing-gap-finding` body 7,005 +
  render 16,624 = **23,629** (pre 23,305; preamble 2,484 · independence 1,200 · scope 3,428 · inputs
  3,518 · verdict 2,251 · output 2,237 · reserved 1,506), budget 19,977, **+3,652**, description
  770. Both descriptions are unchanged and under the 1,536 cap. Unbudgeted: router body 48,776 →
  48,995; implement's seven blocks 60,392 → 66,907 characters.
- `cargo test --all`: 765 passed, 0 failed. `cargo fmt --all --check`: exit 0.
  `cargo clippy --all-targets -- -D warnings`: exit 0. The opt-in sweep
  (`MOCHIKO_FULL_SIMILAR=1 cargo test --all`): `matrix_similar` 48 passed in 248.80s, against 21.50s
  on the skip path, so the sweep ran.
- `PYTHONDONTWRITEBYTECODE=1 uv run evals/run.py command check-rubric implement`: `rubric OK: 75
  observable, 56 out-of-instrument, 131 total`, exit 0. `check-fixtures implement`: `fixture
  consistency: OK`, exit 0.
- Contract suite, host-only, on a scratch mirror (`plugins/`, `scripts/`, `evals/contract/`, the
  crate source and `target/release/mochiko-cli` copied, so `evals/.work/` landed in the
  scratchpad): `contract suite: 7/7 cases passed` · `FILTERED — 7 of 99 declared cases. Not a gate
  run.` It included `the pre-registered floor set matches the implement render (40 ids)` and `rec
  largest render — implement · impl.sec.tools — 28,174 chars / 28,443 bytes, 93.9% of the ceiling`.

## Unit 1 — 0092 and the implement view diff (its share)

VALIDATE: schema content. The unit is `plugins/mochiko/migrations/0092-implement-card-order.yaml`
with its share of `.mochiko/schema-views/commands/implement.yaml`: the `card-order` moment,
`impl.card-order`, the `impl.card-review-before-confirm` reword and `impl.fail.cards-outside-order`.

Checklist run: `validation-primitive-edit.judgment-items-schema`, the AM-2 five. Each is held
against D5-card-order-asked (statement as changed at review, its changed-at-review line, QR2, RB2,
V2, and W1 with QR7) and defaults 4 and 5 as changed.

Evidence read: the migration, read whole · `git diff fccae34 --
.mochiko/schema-views/commands/implement.yaml` · implement's preamble, `impl.sec.reserved`,
`impl.sec.tools` and `impl.sec.fail-conditions` rendered pre and post · record D5 (`:587–629`),
defaults 4–5 (`:976–980`), QR2 (`:1247–1268`), RB2 (`:1375–1379`), QR7 (`:1470–1479`) · wave plan
§3.1 · `mochiko-cli template tasks` (its `## Header`).

Pre-pass: the shared pre-pass above. `0092` carries no exit op.

- **Intent stated — PASS.** The header names D5 as changed at review (S4/S8 with QR2, RB2, V1, V2,
  W1 with QR7) and defaults 4 and 5, then each op. Its claim that `impl.gate-card-confirm` and the
  `card-confirm` moment are unchanged holds: the gate's block is byte-identical pre and post, and the
  moment's line is untouched in the preamble diff.
- **Anchor present where the exit requires one — PASS.** No op exits anything. The header anchor is
  `2026-10-15 engine-follow-up`, and both mints carry `… D5`. The reword keeps the rule's anchor
  state; it had no anchor before.
- **ID lifecycle right — PASS.** Two fresh mints, with `mint-once` and `id-duplicate` silent. The
  reword keeps its id, class and kind: the header line `[class: must · labels: independence]` is
  identical pre and post. The fail's `enforces: [impl.card-order, impl.card-review-before-confirm]`
  names two live rules.
- **Floor and fail survival — PASS.** Pins move from `kind: fail · 16` / `class: floor · 39` to 17
  / 40. The prior 39 floor ids keep their order, and the new id is appended. No implement id vanished
  (125 → 131; `comm` shows additions only).
- **Register — PASS.** `impl.card-order` carries D5's statement and its review lines: a rough order
  shown as a mock · a question only on a real choice (which outcome first, or big steps against
  small) · none where the skeleton and the dependencies decide · the ruled set saved as one line in
  `tasks.md`'s header before the card-authoring seat opens it · no order on a delta or lane run (V2)
  · a lane run slicing several cycles raised to the user (QR7, the visit-file branch dropped) ·
  "after step n you can…" with each `open` line listed (default 5). The reword appends the
  ruled-order check and default 5's meaning-kept check. Both new rules sit outside `impl.sec.tools`.
  No `when:` was owed: the delta and lane clauses are content of the rule, and a `scope` guard would
  strand the lane clause.
- Advisory, not blocking: the `tasks` template's `## Header` documents a title and two
  generated-from pointers, and the order line rides as an unnamed third line. That is the lead's
  ruling A3 (no template change), and conformance checks nothing above `## Overview`. A first
  `tasks.md` write by the lead still has to carry the template's required headings.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 1 (schema content · 0092 + implement view diff) · w2-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 2 — 0093 and the implement view diff (its share)

VALIDATE: schema content. The unit is `plugins/mochiko/migrations/0093-implement-asking-line.yaml`
with its share of the implement view: the `cycle-checkpoint` moment, `impl.asking-line`,
`impl.ruling-places`, `impl.promises-kept-table`, and the rewords of `impl.escalation-batching`,
`impl.finding-severity-routing` and `impl.beyond-spec-disposition`.

Checklist run: the AM-2 five. Every id `impl.ruling-places` names was checked live, and its writer
settings were held against the Implement row of *The re-sort under D10*. The source is D1 as amended
(by D6 and D10), D6 (RB1), D10 (QR3, V5) and defaults 1–3.

Evidence read: the migration, read whole · the implement view diff · implement's full post render,
the six section blocks, swept mechanically for every `impl.*` token · `feat.ruling-places` and
`feat.asking-line` in feature's render · record D1 (`:207–246`), D6 (`:672–702`), D10
(`:1144–1200`), the re-sort's Implement row (`:1208–1220`), the defaults batch (`:971–980`) and RB1
· wave plan §3.2.

Pre-pass: the shared pre-pass above.

- **Intent stated — PASS.** The header names every ruling, the place-by-place sort and each reword
  with its anchor state. It says the floor keeps "its text, force and anchor", and
  `impl.gate-final-acceptance` is byte-identical pre and post.
- **Anchor present where the exit requires one — PASS.** No exit. The stub and `impl.ruling-places`
  carry `… D10`, and `impl.promises-kept-table` carries `… D6`. `impl.beyond-spec-disposition`
  keeps `2026-08-19 qa-gap-finding-verification`, unchanged in the view.
- **ID lifecycle right — PASS.** Three fresh mints, and the rewords keep id, class and kind. Every
  id `impl.ruling-places` names resolves to a live `###` node. The 22 are `impl.asking-line` ·
  `card-order` · `card-review-before-confirm` · `infeasible-card-escalation` ·
  `ambiguity-escalation` · `adopt-first-user-call` · `constraint-challenge` ·
  `beyond-spec-disposition` · `deviation-gate` · `constitution-supremacy` · `user-runopen-rulings` ·
  `scope-escalation-fail` · `finding-kind-disputed` · `gap-rework-bound` · `epic-member-halt` ·
  `attempt-exemption-user-only` · `sufficiency-disputed-clause` · `desk-offer` · `absent-surfaces` ·
  `design-first-write` · `gate-final-acceptance` · `landing-verifier-folds`. The sweep over the
  whole render found no `impl.*` token outside the 131 live rules and six section ids.
- **Writer settings against the re-sort's Implement row — PASS, 16 of 16.**
  - *On*: the card order (graded by the card review) · the checkpoint rulings, four kinds (graded by
    the cycle's verification seat) · the final-validation dispositions (graded by the landing
    verifier's `gates.md` fold check) · the deviation gate (graded by the existing non-author delta
    grade).
  - *Held*: the governance conflict (the ledger path) · the run-open rulings (the sufficiency
    report).
  - *Off*: scope escalation · finding kind disputed · gap-rework bound · epic member halt · attempt
    exemption (V5's re-set) · disputed clause · desk offer · absent-surface offers.
  - *G · held*: design-first-write and acceptance, by `impl.landing-verifier-folds`, as D10 names it.
- **Floor and fail survival — PASS.** No floor or fail is touched. D6 lands as a `must` duty beside
  the floor, and the floor keeps its force.
- **Register — PASS.**
  - The stub mirrors `feat.asking-line`: a must in ways-of-working. `impl.ruling-places` mirrors
    `feat.ruling-places`: must, `binding`, `[user-gate]`, in reserved.
  - Default 1: the "one batch" phrase is replaced by the asking line, which carries
    own-turn-or-batch in its subset. The fact-question clause and the interrupt clause are both kept.
    The same false phrase is fixed in the moment and in `impl.finding-severity-routing`.
  - Default 2: the clause is appended, and the reservation kept.
  - Default 3 needs no rule. The asking line binds every outside-stage ruling to `question-form`
    ("plain words … one concrete example") and `mock-in-message` ("a worked scenario … never a file
    or code"), and inside the stage the engine binds whole.
  - D6: all elements are present. Lines from the slate, `design-stage.md` and the feature-desk visit
    file are quoted verbatim beside the test and its result. An open line reads "open — no target
    set", a measured value is shown, and there is no pass / fail column. The amend names rows, each
    put as fix now or book. The table is composed in the conversation in the user-facing register,
    with no new file, and the report and the diff stay beneath it.
- Advisory, not blocking: "the landing verification seat checking the `gates.md` fold" is bound
  only here. `impl.landing-verifier-folds` names the flip and the lane map-delta check, not the fold
  (`impl.gates-fold` names qa-engineer as the fold's author). The line carries the re-sort's own
  words and binds as written. A seat brief built from `impl.landing-verifier-folds` alone would miss
  it.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 2 (schema content · 0093 + implement view diff, ruling places checked live) · w2-gate-grader · opus · 2 files · 1 rounds · 0 blocking

## Unit 3 — 0094 (implement half), 0095, their view diffs and the allowlist row

VALIDATE: schema content. The unit is the implement half of
`plugins/mochiko/migrations/0094-implement-reads-desk.yaml` (one mint and six rewords),
`0095-sufficiency-reads-visit.yaml` (two rewords), the implement and
`skills/review-sufficiency.yaml` view diffs, and the row appended to
`scripts/similar-rules-allowlist.yaml`.

Checklist run: the AM-2 five, held against default 12 as changed (RB3 with S10), D7 as changed
(QR4), default 24 (QR6), D8's implement side, and the lead's rulings L2, L3 and L4 (build log,
2026-10-18 entries).

Evidence read: both migrations, read whole · both view diffs · a render word-diff of
`review-sufficiency`, pre against post · `review-sufficiency`'s preamble and six sections · the
allowlist diff · the W1 `feature-visit` template (`mochiko-cli template feature-visit`) · record
D7 (`:740–801`), D8 (`:835–864`), QR4, QR6, RB3 · build log `:300–353`.

Pre-pass: the shared pre-pass above, plus the allowlist-necessity replay.

- **Intent stated — PASS.** Both headers name their rulings and every op. 0094's intent gives each
  reworded rule's earlier anchor, and 0095's gives the `plan-stage-utility` anchor and the floor it
  keeps clear of.
- **Anchor present where the exit requires one — PASS.** No exit. `impl.feature-visit-input` takes
  the bare session anchor, since it carries a confirmed default (wave plan §1). Each reword keeps its
  own anchor, all visible as unchanged context lines in the views:
  - `impl.design-inputs` and `impl.desk-offer` — `architecture-brainstorm-interview D5`;
  - `impl.design-stage-inputs` — `implement-brainstorm-led`;
  - `impl.design-stage-agenda` — `… D6`;
  - `impl.delivered-territory-routing` and `impl.gap-finding-blind-dispatch` —
    `qa-gap-finding-verification`;
  - both sufficiency rules — `plan-stage-utility`.

  The sufficiency lines are traceable in `DECISIONS.md`. Their superseded wording rides the
  migration (log-carried verbatim) and the strip's wave-context note; schema content takes no strip
  entry.
- **ID lifecycle right — PASS.** One fresh mint. All eight rewords keep id, class and kind; their
  header lines are identical pre and post.
- **Floor and fail survival — PASS.** No implement floor is touched. `review-sufficiency` keeps
  `class: floor · 8 rules`, and the word-diff changes only `clause9-delta-inapplicable` and
  `fence-read-set`. L2 holds against `review-sufficiency.fence-never-reads`:
  - the card is read in `.mochiko/features/desk/<date-slug>/visit.md`, outside "this batch's own
    `FEAT-XXX/` run-output directory";
  - it is not `tasks.md`;
  - the read stops at "its scope and acceptance criteria, a bug's its reproduction, never its
    **TEST:** cases";
  - the card is desk-authored before dispatch, so it is upstream of the verdict and the floor's
    circularity reason does not reach it.
- **Register — PASS.**
  - `impl.feature-visit-input` carries default 12 as changed: every scope, whatever the gap count,
    the card part read (D7's change), and the desk card a delta or lane run builds among the inputs.
  - Design inputs and stage inputs name the feature-desk file beside the architecture desk's.
  - `impl.desk-offer` is qualified to the architecture desk's file (L3). Otherwise a feature-desk
    file would suppress the offer.
  - Default 24's note rides `impl.design-stage-agenda` ("the need in the user's words and the run
    it came from") and `impl.delivered-territory-routing` (L4: "the finding and the run it came
    from"). The template's header already reads "the run's note from `/mochiko:implement`".
  - D8's implement side adds the quoted lines "never their ruled pieces or cards", and every other
    limb of the dispatch is verbatim.
  - `clause9`'s "carrying the marked delta on the entry" was made false by QR4 and now reads as D7
    ruled.
- **Allowlist row — PASS.** It is needed: the replay without it reports the one `kind: fail`
  cluster. Its reason names a different artifact, a different moment, and the `kind: fail` grammar
  limit, in the shape of the W1 rows.
- Advisory, not blocking: `impl.feature-visit-input` reads "any" feature-desk visit file keyed to
  the FEAT. Dated visits accumulate per FEAT, and nothing scopes the read to the visit the selected
  rows point at (`0095`'s `fence-read-set` does use that pointer). It carries the record's own
  "keyed to the run" and mirrors `impl.design-inputs`' shipped "any desk visit file keyed to the
  run's feature". Separately, two older allowlist rows now suppress nothing:
  `impl.card-review-before-confirm` × `impl.design-review-pair`, and
  `impl.beyond-spec-disposition` × `review-design-audit.disposition-at-checkpoint`. They are left in
  place and documented in `matrix_similar.rs`.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 3 (schema content · 0094 implement half + 0095 + implement and review-sufficiency view diffs + allowlist row) · w2-gate-grader · opus · 5 files · 1 rounds · 0 blocking

## Unit 4 — the testing-gap-finding skill pair and 0094's skill half

VALIDATE: schema content plus skill pair. The unit is the two `testing-gap-finding` rewords in
`0094`, the `.mochiko/schema-views/skills/testing-gap-finding.yaml` diff, and
`plugins/mochiko/skills/testing-gap-finding/SKILL.md` with its render (preamble and six sections).

Checklist run: the AM-2 five for the migration half, and `validation-primitive-edit.judgment-items-pair`
for the pair, with the skill-pair criteria of the path rule. The unit also takes a ruling on L1 and
on the named overage.

Evidence read: the skill half of `0094` · the view diff · `SKILL.md`, read whole (`git diff
fccae34` is empty for it) · the render pre and post, preamble diffed and sections word-diffed · the
ledger row `:92` · record D8 and Q8 (`:811–864`) · build log `:300–319`.

Pre-pass: the shared pre-pass above. The preamble is byte-identical pre and post.

- **Scaffold headings and order — PASS.** `SKILL.md` is unchanged: the load-first section's seven
  `!` lines, and the read-back sentence citing the `class: floor` pin and the `floors:` line.
- **Floor survival — PASS, and L1 stays inside D8: no floor is lowered.**
  - The word-diff shows the inclusion list gaining one insertion after `spec.md` (271 characters)
    and nothing removed.
  - `two-message-dispatch` changes "only" to "only, and the user's lines the list admits as quoted
    text" (53 characters), and nothing is removed.
  - Every other limb stands verbatim: blind to code and cards, the two-artifacts guard, the
    concern-rows guard, the marker reader, and the `structural-exclusions` and
    `delegated-reads-inherit-fence` floors.
  - D8 says the finder "gains one input in its first message … The rest of the fence and the
    two-message dispatch are unchanged". The rewords add exactly that input. Without them, the closed
    list and "references only" would contradict D8.
  - At Q8 the user rejected "the fence as built".
  - The added input is fenced tighter than D8's text ("never those files, their ruled pieces or their
    cards"), and message 1 still carries "never a card, code, or `**TEST:**` path".
  - Both floors keep id, `class: floor`, kind and `qa-gap-finding-verification D3`.
  - The skip rule ("their sole expectation source is the card, which the fence forbids") stays true.
- **Preserved responsibilities — PASS.** No limb leaves. The [v0.103.0] transferred protection is
  honoured as a recorded supersession of wording by D8, carried by the migration and noted in the
  strip.
- **Independence and the reserved section — PASS.** Neither is touched (`sec.independence` and
  `sec.reserved` are identical pre and post).
- **Description — PASS.** 770 characters, unchanged.
- **Argued overage — HOLDS.** The payload is 23,629, +3,652 over 19,977. That is the standing
  +3,328, ruled HOLDS at the 0.120.0 W3 gate (unit 6d), plus +324 from this wave, all render, all
  `0094` (+271 and +53, measured). It is the user's ruled D8 obligation. The body is unchanged, so no
  playbook prose is restored.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 4 (schema content and skill pair · testing-gap-finding — 0094 skill half, view diff, SKILL.md and render) · w2-gate-grader · opus · 3 files · 1 rounds · 0 blocking

## Unit 5 — the implement command pair

VALIDATE: command pair. The unit is `plugins/mochiko/commands/implement.md` (unchanged since
`fccae34`) with the full `implement` render: the preamble and the six section blocks.

Checklist run: `validation-primitive-edit.judgment-items-pair`, with the canonical-scaffold criteria
of the path rule. Also checked: the tools ceiling, every slot of `analysis-iterative.caller-slots`,
every engine id the asking line cites, and the two floors.

Evidence read: `implement.md`, read whole · the seven blocks rendered pre and post, each per-block
byte and character count · the `analysis-iterative` render, all 21 rules · the contract host
cases' implement lines.

Pre-pass: the shared pre-pass above.

- **Scaffold headings and order — PASS.** Frontmatter keys, `# Implement — …`, `## Identity &
  Mission`, `## Rules — delivered by mochiko-cli`, and `## Adaptive Goal Protocol` with Entry, Goal
  and Not done, in order. On the suite side, `the Bash grant the ! lines need is in the frontmatter`
  and `the ! lines enumerate every section, in the render's order` are ok.
- **FAIL survival — PASS.** The pin is `kind: fail · 17 rules`, and the fail-conditions end line
  says 17. The Not-done line cites the pin with no number and keeps its halt clause. The new node is
  `.fail.` and `kind: fail` both ways.
- **Floor survival — PASS.** Floors move 39 → 40. `impl.gate-card-confirm` and
  `impl.gate-final-acceptance` are byte-identical pre and post.
- **Independence — PASS.** `impl.sec.roles` is byte-identical (5,322 bytes). No seat grades its own
  row: the card order's meaning is graded by the card review, and the dispositions by the landing
  verifier.
- **Reserved-to-user content in the reserved section — PASS.** `impl.card-order`,
  `impl.ruling-places` and `impl.promises-kept-table` sit in `impl.sec.reserved` (12 → 15). The
  bindings `impl.asking-line` and `impl.feature-visit-input` sit in ways-of-working (13 → 15).
  `impl.sec.boundaries` is byte-identical (5,991 bytes), and the sound-loop pointer
  `impl.sound-loop-floor` is live.
- **Done-condition branch — PASS.** This is the run branch: the done condition is fixed and closes
  at the existing acceptance gate. D6's table is that gate's form, not a new ceremony. The run-open
  confirmation is unchanged.
- **Preserved responsibilities — PASS.**
  - Seven of the ten rewords are pure insertions.
  - `impl.escalation-batching` and `impl.finding-severity-routing` trade "one batch" and "joins the
    checkpoint batch" for the asking line (default 1); the interrupt clause is kept.
  - `impl.desk-offer` narrows the file's name only (L3).
  - No `.md` line is made false. Entry's delta card names its source and role, not its home, and
    Goal's accept / amend / reject is the gate D6 keeps.
- **Tools ceiling — PASS.** `impl.sec.tools` is 28,443 bytes (28,174 characters): 27,651 → 28,443,
  557 under the 29,000-byte line and under the 30,000-character `!` ceiling. No new rule is in tools.
- **Caller slots — PASS, 6 of 6**, for implement's thinking stage:
  - opening — `impl.design-stage-inputs`, widened here;
  - unit — `impl.design-stage-unit`;
  - deliverable and home — `impl.design-stage-file`;
  - contract on and the next writer — `impl.design-stage`;
  - blind list — `impl.design-stage-blind-list`;
  - independent leg — `impl.design-review-pair`.

  The outside-stage rulings are the named subset by pointer (D10, QR3), not a stage. The card order
  still names its unit (the order and step size), its home (one `tasks.md` header line), its writer
  (the card-authoring seat) and its leg (the card review). No engine rule changed (the engine render
  is byte-identical), and no slot was added.
- **Asking-line ids — PASS, 11 of 11 live** in the engine render: `fact-before-fork` ·
  `one-fork-per-turn` · `options-rule` · `question-form` · `mock-in-message` · `own-turn-or-batch` ·
  `ratified-streak` · `vague-answer` · `user-rules` · `contract-pen` · `contract-inside`.
- **Argued overage — n/a.** Commands are unbudgeted. The seven blocks grow 60,392 → 66,907
  characters.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 5 (command pair · implement) · w2-gate-grader · opus · 1 files · 1 rounds · 0 blocking

## Unit 6 — the review-sufficiency skill pair

VALIDATE: skill pair. The unit is `plugins/mochiko/skills/review-sufficiency/SKILL.md` (unchanged
since `fccae34`) with its render, the preamble and six sections. The schema half of `0095` is
graded in unit 3.

Checklist run: `validation-primitive-edit.judgment-items-pair`, with the skill-pair criteria. Also
the named overage, and L2's wording against the floor `review-sufficiency.fence-never-reads`.

Evidence read: `SKILL.md`, read whole · the render pre and post, word-diffed · ledger row `:80` ·
record default 12, RB3 and QR4 · the build log's L2 entries (`:311–353`).

Pre-pass: the shared pre-pass above. The preamble is byte-identical pre and post.

- **Scaffold headings and order — PASS.** `SKILL.md` is unchanged: seven `!` lines and the floor
  read-back sentence.
- **Floor survival — PASS.** `class: floor · 8 rules`, the ids unchanged and every floor's text
  unchanged.
- **L2 against `fence-never-reads` — PASS.** The widened read cannot contradict the floor. It is
  outside the batch's `FEAT-XXX/` run-output directory, it is not `tasks.md`, and it is "never its
  **TEST:** cases". The card is upstream of the verdict, so the floor's "downstream … circular"
  reason does not reach it. The per-card form's three clauses (`unit-and-collapse`) need the card's
  scope and criteria, not its tests.
- **Preserved responsibilities — PASS.** `clause9` keeps "the `[MODIFY]` instrument … folding at
  landing … re-fires the design phase". Only the home phrase changed, and it changed as D7's QR4
  ruled. The description's "Never reads code, `tasks.md`, `**TEST:**` cases, or cycle reports" and
  the Procedure's "read the fenced source set whole" stay true.
- **Independence and the reserved section — PASS.** Both are identical pre and post.
- **Description — PASS.** 686 characters, unchanged.
- **Argued overage — HOLDS.** The payload is 17,614, +2,191 over 15,423. That is the standing
  +1,937, ruled HOLDS at the 0.120.0 W3 gate (unit 6c), plus +254 from this wave, all render: scope
  +56, inputs +198. It is default 12's ruled obligation with D7's change: the per-card form reads the
  card where it now sits. The body is unchanged, so nothing is restored.
- Advisory, pre-existing (the lead's N1, booked at landing): the `tasks` card shape keeps an
  improvement card's criteria only per case inside the **TEST:** bundle. The criteria-testable clause
  can therefore find no admissible object and grades gap. It fails safe.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 6 (skill pair · review-sufficiency) · w2-gate-grader · opus · 1 files · 1 rounds · 0 blocking

## Unit 7 — router, strips, budget ledger, changelog, ripple

VALIDATE: prose primitives and records. The unit is `plugins/mochiko/skills/mochiko/SKILL.md`, the
strips `implement.md`, `review-sufficiency.md`, `testing-gap-finding.md` and `mochiko.md`,
`.mochiko/memory/primitive-cost-budgets.md`, `CHANGELOG.md` `## [0.121.0]`, and the ripple files
`README.md`, `ARCHITECTURE.md` and `GLOSSARY.md`.

Checklist run: `validation-primitive-edit.judgment-items-prose` — coherence and preserved
responsibilities. Every figure is reproduced first-hand. P2's "unchanged" calls and W1's carried
note are tested against the built rules.

Evidence read:
- the `git diff fccae34` of each changed file;
- router rows `:85`, `:90`, `:100`, `:101`, `:117`, `:141` and `:143`;
- `README.md:134`;
- `ARCHITECTURE.md:230–300`, plus greps over all three ripple files for the card confirm,
  acceptance, the fence, the visit file and the run's note;
- `GLOSSARY.md:16`;
- the `feature-visit` template overview and header;
- the `DECISIONS.md` 2026-10-15 row.

Pre-pass: the shared pre-pass above. Router body 48,776 → 48,995 (+219), measured.

- **Router — PASS.**
  - `:117` copies the closed inclusion list, so an unchanged copy would understate the fence. The
    insertion matches the rule, and the row's existing "never … cards" carries the rest.
  - `:143`'s "three routing gates" stays true: the card order is an asking-line ruling (D10's E), not
    a gate.
  - "Zero gaps → straight to cards" stays true: the design phase is what is skipped.
  - `:85`, `:90` and `:100` are not made false.
  - The change is a pure addition, so no strip entry is owed.
- **Strips — PASS.** Each wave-context comment's reword list matches the migrations: implement's
  ten rewords and the moment, each under its ruling; two each for sufficiency and gap-finding; router
  `:117`. No prose line left a shipped primitive, so no `[v0.121.0]` entry is owed.
- **Ledger rows `:80` and `:92` — PASS.** Payloads, section splits, the 1..89 control figures
  (14,098 and 16,300) and the per-rule deltas (+198 / +56, +271 / +53) all reproduce exactly. The
  verdict slots `[lead: the unit-6 verdict]` are the lead's to fill.
- **CHANGELOG `## [0.121.0]` W2 part — PASS.** Every figure reproduces:
  - 1,254 → 1,260;
  - pins 17 and 40;
  - floors 8 and 9 unchanged;
  - the six mints and ten rewords;
  - payloads 17,614 and 23,629;
  - implement 60,392 → 66,907;
  - tools 27,651 → 28,443;
  - router 48,776 → 48,995.

  The `[lead: …]` placeholders are expected.
- **Ripple — PASS: P2's call holds.**
  - `README.md:134`, a flow summary, is not false.
  - `ARCHITECTURE.md:285`'s gate list ("run-open routing · design sign-off · card confirm · deviation
    consent · final acceptance") is not false.
  - `ARCHITECTURE.md:293`'s "desk-confirmed card directly" is not false.
  - No ripple line restates the fence or the delta card's home.
- **W1's carried note — both lines TRUE now.**
  - `GLOSSARY.md:16` ("a card visit's ruled set and its cards, read by implement") holds by
    `impl.feature-visit-input`.
  - The template overview ("the dispatched run reads the cards and their set here") holds by the same
    rule. Its header's "the run's note from `/mochiko:implement`" now has its writers (default 24).
- Advisory, not blocking:
  - The router strip note says `:90` "names no read set". The row names its graded sources and "the
    design baseline is read too", so the comment's wording is loose, but the row is not false.
  - Under this audit's numbering, the testing-gap-finding overage is unit 4's verdict, not unit 6's.

VERDICT: PASS

Issues requiring fix: none.

audit: unit 7 (prose primitives and records · router, four strips, budget ledger, CHANGELOG, README, ARCHITECTURE, GLOSSARY) · w2-gate-grader · opus · 10 files · 1 rounds · 0 blocking

## Unit 8 — crate tests, contract expectations, plan kits

VALIDATE: tests and kits. The unit is `crates/mochiko-cli/tests/fidelity.rs`, `validate.rs`,
`render.rs` and `matrix_similar.rs` · `evals/contract/run.py` and `README.md` ·
`evals/plan/README.md` and `evals/plan/implement/observable.yaml`. This seat authored none of
them, so this is also the crate's independent non-author review.

Checklist run: coherence against the replayed state, and the arithmetic in every comment. Also
whether each frozen figure moved only where the log moved, and the kit partition.

Evidence read: the `git diff fccae34` of all eight files · `migrate status` and the census
figures · the render's `floors:` line · the pre and post validate stats · `run.py`'s
`EXPECTED["implement"]`, compared by script with the render.

Pre-pass: the shared pre-pass above — 765/0, sweep 48/0, fmt and clippy 0, rubric and fixtures
OK, host contract 7/7.

- **Crate tests — PASS.**
  - `fidelity.rs`: the sequences gain 92–95, with docs at 94 (status: 94 documents). The census is
    command 418, total 1,260, command floors 130, fail nodes 41. The "fourteen rewords — 1 + 3 + 8 +
    2" match the migrations.
  - `validate.rs`: the same census. The six mints are named with the correct classes.
  - `render.rs`: `IMPLEMENT_FLOORS` has 40 entries, the new id last, as the `floors:` line ends. The
    widest index grows +32 characters and +33 bytes, which is `len(" · impl.fail.cards-outside-order")`
    with one two-byte middle dot.
  - `matrix_similar.rs`: the pair arithmetic holds. Binding C(64,2)−C(62,2) = 125 · duty 89 · fail
    40 · constraint 175 − 1 = 174, for +428 in the command family and +1,325 in the corpus.
    Suppressed edges go 186 → 185, the same as this seat's replays (186 pre; 184 post on the old
    allowlist; 185 with the row).
  - Comment and constant edits only, in the files' own idiom.
- **Contract suite — PASS.** `EXPECTED["implement"]` equals the render's floor set (40 = 40, empty
  symmetric difference). The 44,266 freeze baseline is untouched, as it should be. `README.md`'s
  floor count goes 129 → 130 and names the id and its ruling.
- **Plan kits — PASS.**
  - `observable.yaml` partitions all 131 live ids (75 + 56). The six new ids are placed: four
    observable, two out-of-instrument. `impl.feature-visit-input` is out-of-instrument as conditional
    (no feature-desk visit file planted), by the lead's rulings 2 and 4, in line with
    `impl.design-stage-inputs`.
  - The `impl.escalation-batching` why is re-cut to the asking line.
  - The footer gives the bucket shape and the preregistration reason: the kit has run, per §3.6 and
    W1's setup precedent, and the next grid amends it first.
  - `README.md` reads "implement 75 + 56".

VERDICT: PASS

Issues requiring fix: none.

audit: unit 8 (crate tests, contract expectations, plan kits · crate non-author review) · w2-gate-grader · opus · 8 files · 1 rounds · 0 blocking

## Notes of note

- No `Explore` dispatch was made. Every read either decided a verdict (completeness-sensitive,
  kept on this seat by the routing rule) or was a single targeted grep.
- The contract suite ran host-only, on a scratch mirror: 7/7, "Not a gate run". The full sandbox
  suite belongs to the bump gate, and a SKIPPED suite blocks. `cargo audit` was not run; it was not
  in the brief and belongs to the lead's wave close.
- The advisories in units 1, 2, 3, 6 and 7 block nothing. They are candidates for the lead to place.
