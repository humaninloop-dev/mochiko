---
report: review
round: 1
grader: w3-gate-grader
tier: opus
wave: 3
units: [1, 2, 3, 4, 5, 6, 7]
contract: mochiko-cli rules validation-primitive-edit (preamble + six sections, binary 0.4.0, plugin 0.122.0) — floor pin 11, read back before the first step
---

## Pre-pass (shared, run first-hand by the grader)

- Contract read-back before the first step: `- class: floor · 11 rules`; floors author-grader · plain-seat-explicit-tier · rendered-contract-only · from-file-floor · pre-pass-first-hand · binary-verdict · default-fail · tamper-proof-clause · gate-loop-bound · evidence-floor · second-fail-user. Every block carried its version-triple and end line; section totals 4 + 3 + 3 + 7 + 3 + 1 = 21.
- `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` (exit 0), tail: `pointer resolution: 91 checked against plugins/mochiko` · `similar-rule clusters (threshold 0.60): none` · `rules scanned: 1283 · in-kind pairs scored: 225052 · clusters: 0 (none)` · `allowlist-suppressed edges: 182` · `mochiko-cli migrate validate · 0 rejecting · 118 advisory`.
- Baseline `c936678` (`git archive` into scratch, validated with `--plugin-root`): `0 rejecting · 118 advisory`, `pointer resolution: 90`, `rules scanned: 1281`, `allowlist-suppressed edges: 186`. A line diff of the two reports, budget lines aside, differs only in pointer count (+1, the `stakes-band` pointer), rules scanned and suppressed edges. No new `condition-coverage` line; no `stakes` value uncovered.
- Budget advisory lines moved: `command/implement` 131 rules · 48332 → 133 · 53725; `skill/patterns-sound-loop` 21 · 9145 → 21 · 9326; `skill/review-sufficiency` 30 · 8684 → 30 · 8767. Transport-floor unchanged (14 · 3143).
- `mochiko-cli migrate status` — `sequences 1..104 (92 migrations)` · `state sha256:94ff194eabf0…` · `94 documents · 1283 rules`. `0104` header `hash: sha256:7bfc3eac921e…`.
- `mochiko-cli views emit --out <scratch>` then `diff -r` against `.mochiko/schema-views` — identical across every document.
- Implement preamble now vs baseline: pins `kind: fail · 17 rules` and `class: floor · 40 rules` both unchanged, floors line byte-identical; the diff is the new `stakes` condition line, the run-open moment gaining "the stakes line", tools 50 → 51 and ways-of-working 15 → 16.
- Skill renders now vs baseline (all seven blocks each, `diff`): sound-loop differs only in `stakes-band` (sentence appended, `pointer:` printed); review-sufficiency only in `stakes-proposal` (the parenthesis); transport-floor identical.
- Char-budget, canonical snippet (body, description) plus the seven render blocks as characters of stdout, now vs `c936678`:

  | skill | body | desc | preamble | sec 2 | sec 3 | sec 4 | sec 5 | sec 6 | sec 7 | render | payload |
  |---|---|---|---|---|---|---|---|---|---|---|---|
  | patterns-sound-loop now | 3,315 | 746 | 2,436 | 4,407 | 698 | 5,096 | 423 | 1,938 | 929 | 15,927 | 19,242 |
  | patterns-sound-loop base | 3,315 | 746 | 2,436 | 4,184 | 698 | 5,096 | 423 | 1,938 | 929 | 15,704 | 19,019 |
  | review-sufficiency now | 3,262 | 686 | 1,827 | 514 | 1,839 | 2,225 | 6,521 | 1,297 | 1,021 | 15,244 | 18,506 |
  | review-sufficiency base | 3,262 | 686 | 1,827 | 514 | 1,839 | 2,225 | 6,440 | 1,297 | 1,021 | 15,163 | 18,425 |
  | patterns-transport-floor now | 3,135 | 450 | 2,161 | 1,595 | 932 | 3,187 | 447 | 444 | 447 | 9,213 | 12,348 |
  | patterns-transport-floor base | 3,114 | 450 | 2,161 | 1,595 | 932 | 3,187 | 447 | 444 | 447 | 9,213 | 12,327 |

- Transport-floor prefix check: the log reduced to `0001-genesis.yaml` alone renders 9,213 (`--log-dir`); `0001` is the only migration naming the skill; `git log 62aa99d..HEAD` shows no edit to `0001-genesis.yaml` since the commit that recorded 9,450, and many to `crates/mochiko-cli/src/`.
- Implement is a command and unbudgeted; `references/` is exempt; the router is unbudgeted.
- Locate routing: no `Explore` was spawned; every read was a direct targeted read of a known path, run on-seat (the dispatch ladder's first rung).

## Unit 1 — schema content

```
VALIDATE: unit 1 — plugins/mochiko/migrations/0104-implement-stakes.yaml + git diff .mochiko/schema-views/{commands/implement,skills/patterns-sound-loop,skills/review-sufficiency}.yaml (schema content; no allowlist row)
Checklist run: validation-primitive-edit.judgment-items-schema — the AM-2 five
Evidence read: 0104-implement-stakes.yaml (whole); the three view diffs (whole); record.md D1, D2 (as amended), D3, D5, D6, D7, D8 and Defaults DF1–DF11; research/floor-inventory.md result table (line 19); wave3-implement-ship.md (whole); the sound-loop render's stakes-key, stakes-band, never-thins, trail-of-lowerings, user-rules-the-plan
Pre-pass: migrate validate --report — "0 rejecting · 118 advisory", 0 clusters, 182 suppressed; views emit ≡ committed views; migrate status 1..104, 1283 rules
PASS intent stated — the header names every change and its cards (D1, D2 as amended, D3, D5 as amended, D6, D7, DF2, DF5, DF7–DF10, the inventory), the two-wordings split, and "implement 40 floors · 17 fails, rules 131 to 133; sound-loop 10 · 21 and review-sufficiency 8 · 30 unchanged", which the preambles and budget lines reproduce.
PASS anchor present where the exit requires one — header anchor `2026-10-27 implement-lead-adaptive-rigor` covers the three floor rewords and the cleared `when:`; each ruling covers its exit: `impl.attempt-per-grade` appends DF10 nearly verbatim; `impl.fail.skip-unstated` widens to every thinned or skipped step at every scope (D5); `impl.fail.gap-finding-missing` takes the inventory's stated-skip form, its report leg carried by skip-unstated. Mints carry `… D1` and `… D3`.
PASS ID lifecycle right — 2 mints (`impl.lowering-trail`, `impl.smoke-probe`) minted once; 19 rewords (17 implement, 1 each skill); 12 set-rule-field (9 `when:` keys, the `when: null` clear and `enforces:` on skip-unstated, the `stakes-band` pointer). Every reworded rule keeps id, class, kind, labels and anchor in the view; no tombstone. Every cited id resolves, cross-skill ones included.
PASS floor and fail survival — implement 40 · 17 with the floors line unchanged; sound-loop 10 and review-sufficiency 8 unchanged; no floor minted, retired or lowered. The cleared `when:` widens skip-unstated from delta and lane to every scope, a raise. Gap-finding-missing stays floor in its stated-skip form, as the inventory's "moves to a stated-skip form, not away" reads.
PASS register — texts trace the cards: D5's thin set (per-cycle grades, card review, per-cycle suite, sweep breadth, design pair, plan-QA) and skip set (gap-finding, design audit, lens, blind list at one or two gaps), each with "money, data, security: <full form>"; the cold run unconditional with the smoke inside it (D5 S7, D6 row 5); the never-move steps left unkeyed; DF8 one smoke per epic; DF9 in the smoke's revert clause. The whole-list `when:` follows the plan's carriers convention and the `0102`/`0103` precedent.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 2 — command pair `implement`

```
VALIDATE: unit 2 — plugins/mochiko/commands/implement.md + mochiko-cli rules implement render (preamble + six sections) (command pair)
Checklist run: validation-primitive-edit.judgment-items-pair, held to the canonical-scaffold criteria 1–11 of .claude/rules/mochiko/primitive-edits.md
Evidence read: implement.md (whole Rules block and Adaptive Goal Protocol) and its git diff; the preamble render (whole) now and at baseline; the touched rules in the view; ADR 2026-08-13-charter-plan-implement ruling 3 as quoted in criterion 7
Pre-pass: migrate validate --report — "0 rejecting · 118 advisory"; `budget · command/implement · - · 133 rules · 53725` (unbudgeted command)
PASS scaffold headings and order — frontmatter keys, `# Implement — the Single Downstream Run`, `## Identity & Mission`, `## Rules — delivered by mochiko-cli`, `## Adaptive Goal Protocol` with Entry, Goal, Not done last; the diff touches one Entry sentence only.
PASS rules-block enumeration — the six `!` lines name `impl.sec.roles · reserved · tools · ways-of-working · boundaries · fail-conditions`; the new prose token `impl.sec.reserved` resolves.
PASS FAIL survival and pin citation — 17 `kind: fail`, each `.fail.*` id kind fail; Not-done cites the `kind: fail` pin and the halt clause, no number, unchanged.
PASS ID continuity — no id vanishes; two mints; no surviving text references a tombstoned node.
PASS floor survival and provenance — 40 floors, three reworded under the header anchor, rulings cover them (unit 1).
PASS substance and preserved responsibilities — reserved content (the stakes read, review form, plan of thinnings, later lowering) sits in `impl.user-runopen-rulings` in `impl.sec.reserved`; `impl.sound-loop-floor` kept; independence holds — smoke by the QA seat that wrote no product code, every thinned review still non-author; the clause is a pure addition, no strip owed.
PASS done-condition branch, implement — the stakes line rides the existing run-open confirmation ("One confirmation, no negotiation", unchanged) beside batch, scope, attempt bounds and done condition; the run-open moment text names it; no new gate; Goal fixed and unchanged.
PASS ontology grammar — `stakes` declared `moment-resolved(run-open)` on a declared moment; ten rules carry `when: {stakes: [six]}` (nine set plus the minted smoke-probe), `impl.design-finish` two-term; DECLARE fits, as the text carries the band split; both fail rules' `enforces:` resolve; skip-unstated has no `when:`, so it binds at every scope.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 3 — skill pair `patterns-sound-loop`

```
VALIDATE: unit 3 — plugins/mochiko/skills/patterns-sound-loop/SKILL.md (unchanged) + render (preamble + six sections) + references/STANDARD-THIN-PLAN.md (new) (skill pair)
Checklist run: validation-primitive-edit.judgment-items-pair, held to the skill-pair criteria 1–12
Evidence read: SKILL.md lines 1–26 and git status (unchanged); the full render now and at baseline, diffed; STANDARD-THIN-PLAN.md (whole); the ledger row (git diff); record D5 statement and D8 placement sentence
Pre-pass: migrate validate --report — "0 rejecting · 118 advisory", `pointer resolution: 91`, no `pointer-unresolved`; canonical snippet — body 3,315 · description 746 · render 15,927 · payload 19,242 (shared table)
PASS load-first, enumeration, floor pin read-back — SKILL.md unchanged since W1's PASS; pin `class: floor · 10 rules` and floors line unchanged from baseline.
PASS floor survival and ID continuity — `stakes-band` is `class: must`, reworded with id, `when:`, labels and anchor kept; prior text a verbatim prefix.
PASS description untouched — 746, unchanged, under the 1,536 cap.
PASS pointer resolution — `references/STANDARD-THIN-PLAN.md` resolves base-dir-relative (the CLI counts +1 and rejects nothing).
PASS reference file register and no restated rule — one table of D5's eight example rows (step, full, thin, what still catches), "an example, never a menu", the never-move set pointed at by id (`patterns-sound-loop.never-thins`), one example line in DF5's grammar. It binds nothing; see the notes for two small paraphrases.
PASS argued overage — +6,854 over 12,388; +6,631 ruled HOLDS by W1, and +223 this wave, render only, trigger section only. Reproduced: payload 19,242, render 15,927 with all seven counts, baseline 19,019, delta 4,407 − 4,184 = 223, which splits into the sentence (+179) and the printed `pointer:` field (+44). The argument holds. D5 puts the worked example "in the skill" for the lead to cite by name. D8 allows a sound-loop reference file. The pointer plus one sentence is the least that makes the exempt file reachable from the band rule. No playbook prose is restored.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 4 — skill pair `review-sufficiency`

```
VALIDATE: unit 4 — plugins/mochiko/skills/review-sufficiency/SKILL.md (unchanged) + render (preamble + six sections) (skill pair)
Checklist run: validation-primitive-edit.judgment-items-pair, held to the skill-pair criteria 1–12
Evidence read: the full render now and at baseline, diffed; `review-sufficiency.fence-read-set` and the in-flight carve; the view diff; the ledger row (git diff); W2 audit note on unit 5
Pre-pass: migrate validate --report — "0 rejecting · 118 advisory"; canonical snippet — body 3,262 · description 686 · render 15,244 · payload 18,506 (shared table)
PASS load-first, enumeration, floor pin read-back — SKILL.md unchanged; pin 8 and floors line unchanged.
PASS floor survival and ID continuity — `stakes-proposal` (must · duty) reworded with id, labels, pointer `mochiko:patterns-sound-loop` and anchor `2026-10-27 implement-lead-adaptive-rigor D2` kept; only the parenthesis changed.
PASS substance — the widened list matches `fence-read-set` (spec, store, baselines, map entries, the delta card under delta scope); the old list made a delta-scope proposal read uncited.
PASS description untouched — 686, unchanged.
PASS argued overage — +3,083 over 15,423; +3,002 ruled HOLDS by W2, +81 this wave, verdict section only (6,521 − 6,440). Reproduced exactly. The argument holds as D2's obligation: the class is proposed with a line cited from inside the fence. Without the change, the delta card could not be cited. It closes W2's advisory.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 5 — skill pair `patterns-transport-floor`

```
VALIDATE: unit 5 — plugins/mochiko/skills/patterns-transport-floor/SKILL.md (lines 13, 49) + render (unchanged) + .mochiko/strips/patterns-transport-floor.md [v0.123.0] (skill pair)
Checklist run: validation-primitive-edit.judgment-items-pair, held to the skill-pair criteria 1–12 plus strip presence and fidelity
Evidence read: SKILL.md git diff and baseline lines 13 and 49; the strip entry (whole); record D8 statement and accepted risk, D6 row 2; `grep -rn "who produces" plugins/mochiko`; sound-loop SKILL.md Overview; the ledger row (git diff); the genesis-only replay
Pre-pass: migrate validate --report — "0 rejecting · 118 advisory", `budget · skill/patterns-transport-floor · - · 14 rules · 3143` unchanged; canonical snippet — body 3,135 · description 450 · render 9,213 · payload 12,348 (shared table)
PASS load-first, enumeration, floor pin, grammar — Rules block, seven `!` lines and read-back sentence untouched; render byte-identical to baseline.
PASS preserved responsibilities — both paraphrases keep "who produces" and "who reviews" and add "how deep"; the transport remit is unchanged.
PASS strip presence and fidelity — `## [v0.123.0]`, newest-first; Disposition superseded; Tier failed n/a, citing D8 (its accepted-risk sentence quoted exactly as the record has it) with D6 row 2 and the DECISIONS row; Content is verbatim to `c936678` lines 13 and 49; Kept deliberately and Consumers assessed both present. The grep reproduces: only these two lines plus sound-loop's own Overview, which already says "how deep".
PASS description untouched — 450, unchanged.
PASS budget — 12,348 inside 12,564 by 216; body +21 reproduced (3,114 → 3,135). The "−237 render, unattributed to a migration" claim is honest. Genesis alone renders 9,213. Genesis is unedited since the commit that recorded 9,450. The binary's render code changed many times in between. So the drop is the binary's render shape, not the log.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 6 — prose primitive, router `skills/mochiko/SKILL.md`

```
VALIDATE: unit 6 — plugins/mochiko/skills/mochiko/SKILL.md (prose primitive)
Checklist run: validation-primitive-edit.judgment-items-prose — coherence and preserved responsibilities
Evidence read: git diff (line 70 only); rows 70–72 in full; the sound-loop render (stakes-key, stakes-band, never-thins, review-form-invariants, trail-of-lowerings)
Pre-pass: migrate validate --report — "0 rejecting · 118 advisory" (no rule set; router unbudgeted)
PASS coherence — the inserted clause matches the render: a product run's harm class, ruled by the user on the run-open line, keys how deep the legs run after the trigger fires, never whether, with the never-thins set, five review-form invariants and the trail; it reads in the row's list grammar and leaves the transport-floor row's sibling wording true.
PASS preserved responsibilities — every prior word of the row survives in order; pure addition, no strip owed; no other line changed.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 7 — contract freeze `expected-skills.json` + `README.md`

```
VALIDATE: unit 7 — evals/contract/expected-skills.json + evals/contract/README.md (freeze replacement)
Checklist run: README freeze rule ("A replacement moves floor_ids and floor_pin, and nothing else") and accuracy against the grader's own render
Evidence read: git diff of both files (whole); a JSON field-by-field comparison against c936678 over all 36 rows; mochiko-cli rules patterns-sound-loop --section preamble (floors line and pin); each new floor's class line in the render
Pre-pass: migrate validate --report — "0 rejecting · 118 advisory"; render pin `class: floor · 10 rules`, floors line 10 ids
PASS field scope — only `patterns-sound-loop.floor_ids` and `floor_pin` differ; every other field of that row and every other row is identical; the textual diff is one hunk.
PASS floor set — `floor_ids` equals the render's floors line as a set, ten ids, sorted; `floor_pin` 10 equals the pin.
PASS README count — the floor_pin sum over 36 rows is 270, and "270 across thirty-six" matches it.
FAIL README note accuracy — the note credits "6 → 10" to five items: "the band, the never-thins set, the five invariants, the trail, the user's ruling over the plan". Only four floors were added: `never-thins`, `review-form-invariants`, `trail-of-lowerings`, `user-rules-the-plan`. "The band" is `patterns-sound-loop.stakes-band`, which is `class: must`, not a floor.
VERDICT: FAIL
Issues requiring fix:
- README note accuracy — the parenthetical names a non-floor rule among the floors it accounts for. Fix: drop "the band, " so it reads "(the never-thins set, the five invariants, the trail, the user's ruling over the plan)". Nothing else moves.
```

## Outcome lines

- `audit: unit 1 — 0104-implement-stakes.yaml + three view diffs (schema content) · w3-gate-grader · opus · 4 files · 1 rounds · 0 blocking`
- `audit: unit 2 — implement.md + render (command pair) · w3-gate-grader · opus · 2 files · 1 rounds · 0 blocking`
- `audit: unit 3 — patterns-sound-loop SKILL.md + render + references/STANDARD-THIN-PLAN.md (skill pair) · w3-gate-grader · opus · 3 files · 1 rounds · 0 blocking`
- `audit: unit 4 — review-sufficiency SKILL.md + render (skill pair) · w3-gate-grader · opus · 2 files · 1 rounds · 0 blocking`
- `audit: unit 5 — patterns-transport-floor SKILL.md + render + strip (skill pair) · w3-gate-grader · opus · 3 files · 1 rounds · 0 blocking`
- `audit: unit 6 — skills/mochiko/SKILL.md router (prose primitive) · w3-gate-grader · opus · 1 files · 1 rounds · 0 blocking`
- `audit: unit 7 — expected-skills.json + README.md (contract freeze) · w3-gate-grader · opus · 2 files · 1 rounds · 1 blocking`

## Failure narrative

Unit 7 fails on one blocking finding, and everything else in it passes. The frozen set is exact.
The README's 270 count is exact. The appended note names five things as the source of
`patterns-sound-loop`'s four new floors. One of them, "the band", is `stakes-band`, which is
`class: must`. A reader counting floors from that note would count a non-floor. The fix is the
two-word deletion given in the unit's block. The re-audit reads only the README line.

## Notes of note

- Advisory, unit 3: `STANDARD-THIN-PLAN.md` line 5 paraphrases the band ("for money, data or security none of it applies"). Line 15 paraphrases `impl.minimalism-advisory`. The example line's `never:` lists "the cold run", which `never-thins` does not hold, though line 18 says that segment names members of that set. It belongs to `impl.cold-verification`. None binds, so none blocks.
- Advisory, unit 3: `stakes-band` carries the reference path twice, once in its sentence and once in `pointer:`, about 35 characters of the +223.
- Advisory, unit 1: "provisionally until the cold smoke" in the six before-smoke rules does not redo thinned earlier steps after a red smoke. `impl.smoke-probe` reverts later steps and the rework only. That matches DF9. The word "provisionally" carries the lead's reading of D5, which the build log records.
- Advisory, unit 1: `impl.fail.gap-finding-missing` passes a skip "on the plan the user ruled". The plan's `thins on clean:` segment makes a skip after a red smoke off-plan, so the rule holds. That rests on the line's grammar, not on the fail text.
- Advisory: suppressed edges fell 186 → 182. Four allowlist rows no longer clear 0.60 after the rewords. They are stale but harmless.
- Count note: the brief named nine stakes-keyed implement rules. The view holds ten, because the minted `impl.smoke-probe` is keyed too.
- Brief note: the brief carried lead's notes beyond the unit, paths and pre-pass. Every figure above was reproduced first-hand and none was taken from the brief.

## Round 2 — unit 7 re-audit (same seat, resumed)

```
VALIDATE: unit 7, round 2 — evals/contract/README.md (the 6 → 10 note) and what it could break: the 270 total and the expected-skills.json row
Checklist run: the round-1 fix item only, per validation-primitive-edit.gate-loop-bound
Evidence read: git diff -- evals/contract/README.md (still one line out, one in); the note as it now reads; the JSON field-by-field comparison against c936678 re-run; mochiko-cli rules patterns-sound-loop --section preamble (pin and floors line)
Pre-pass: unchanged from round 1 — the fix touches no schema content or primitive
PASS README note accuracy — the note now reads "(the never-thins set, the five invariants, the trail, the user's ruling over the plan)", four items for the four new floors; "the band, " is gone.
PASS README count — "270 across thirty-six" is unchanged, and the floor_pin sum over 36 rows is still 270.
PASS JSON row — only the sound-loop floor_ids and floor_pin differ from c936678; ten ids matching the render as a set, sorted, pin 10.
VERDICT: PASS
Issues requiring fix: none
```

- `audit: unit 7 — expected-skills.json + README.md (contract freeze) · w3-gate-grader · opus · 2 files · 2 rounds · 0 blocking`

## Round 2 — units 1 and 2 re-audit (same seat, resumed)

Trigger: the release gates' `no_shipped_section_renders_past_the_inline_ceiling` was red on `implement · impl.sec.tools`. The lead ruled a fix, and P1 landed it in `0104` (re-stamped `sha256:7cd24f21…`). This block reads only what changed and what it could have broken. The comparison base is my round-1 render of all seven implement blocks, kept in scratch, because the migration is untracked.

- Pre-pass re-run: `migrate validate --report` exit 0, `0 rejecting · 118 advisory`, `clusters: 0`, `allowlist-suppressed edges: 182`; findings identical to round 1 apart from `budget · command/implement · - · 133 rules · 51621` (was 53725). `migrate status` 1..104, `state sha256:84802fd7bf33…`, 1283 rules. `views emit` ≡ committed views. `cargo test -p mochiko-cli --test render`: 39 passed, 0 failed, the ceiling test among them.
- Section sizes, characters of stdout: `impl.sec.tools` 29,609 (50 rules), `impl.sec.ways-of-working` 9,770 (17 rules), against `CEILING: usize = 30_000`. The other three skill renders are byte-identical to round 1.
- Render diff, round 1 to round 2: `impl.smoke-probe` moved from tools to ways-of-working with header and text unchanged. Nine stakes-keyed rules now read as their shipped `c936678` text verbatim plus " In the band it thins per the ruled plan, on a clean cold smoke (impl.smoke-probe)." A script compared each against the baseline render. `impl.gap-finding-scope` keeps its shipped meaning: "Every skip, by scope or by band, is stated in the final-validation report — never a silent no-op." `impl.cold-verification` and `impl.sufficiency-report` carry shorter appends. Every other rule is unchanged, `attempt-per-grade`'s DF10 append included. The op count is still 35.

```
VALIDATE: unit 1, round 2 — 0104-implement-stakes.yaml (as re-stamped) + .mochiko/schema-views/commands/implement.yaml (regenerated)
Checklist run: the AM-2 five, on the changed ops only
Evidence read: the migration header and op layout; the round-1 vs round-2 render diff (whole); the per-rule comparison against c936678; record DF7 and D5 statement; the testing-end-user smoke-procedure and cold-run-always, review-seat-plan once-per-seat-in-band and testing-gap-finding band-skip-stated renders
Pre-pass: as above — 0 rejecting · 118 advisory; views ≡ replay; 1..104, 1283 rules
PASS intent stated — the reworded header says each thinnable step is "worded per the ruled plan on a clean cold smoke (DF7)", that the lighter forms live in the worked example and the W2 spokes, and that the revert lives in the smoke probe; the counts stay 40 · 17, 131 to 133.
PASS anchor present — header anchor unchanged; the three floor rewords and the cleared `when:` are as in round 1.
PASS ID lifecycle right — the same 2 mints, 19 rewords and 12 set-rule-field; `impl.smoke-probe` changes section only (mint target `impl.sec.ways-of-working`), with id, class, labels, `when:`, pointer and D3 anchor unchanged.
PASS floor and fail survival — pins 17 and 40, floors line byte-identical to c936678; no floor touched by the fix.
PASS register — the uniform clause is DF7's own wording ("worded 'per the ruled plan, on a clean cold smoke'"). The lighter forms that left implement were never shipped. Their invariants still bind elsewhere: the smoke's full automated suite (`testing-end-user.smoke-procedure`), the cold run always on the accepted head (`cold-run-always`), the stated band skip (`band-skip-stated`), once-per-seat plan-QA (`review-seat-plan`), TEST: execution and the non-author read (`never-thins`), legs thin and never vanish (`stakes-band`).
VERDICT: PASS
Issues requiring fix: none
```

```
VALIDATE: unit 2, round 2 — plugins/mochiko/commands/implement.md (unchanged since round 1) + the implement render as regenerated
Checklist run: canonical-scaffold criteria 6, 7 and 11 on the changed render, and 2 and 3 for what the move could break
Evidence read: git diff of implement.md (the round-1 clause only); the preamble and the tools and ways-of-working blocks (whole); record DF7, D5, DF9
Pre-pass: as above
PASS rules-block enumeration and FAIL pin — six sections unchanged in name; end lines read tools 50 and ways-of-working 17, matching the preamble; `kind: fail · 17 rules` and the Not-done citation unchanged.
PASS substance (criterion 6) — reserved content is still in `impl.user-runopen-rulings`, and independence still holds. The clause leaves who thins and how to the user's ruled plan under sound-loop's band, which matches D5's "invariants as rules, forms as examples". No reserved decision moves to the lead.
PASS done-condition branch (criterion 7) — the stakes line still rides the existing run-open confirmation; the fix adds no gate, and the Entry clause still describes the ruled line correctly.
PASS ontology grammar (criterion 11) — ten `when: {stakes: [six]}` keys unchanged, `impl.design-finish` still two-term; `impl.smoke-probe`'s new section is declared; both fail rules' `enforces:` still resolve; skip-unstated still has no `when:`.
VERDICT: PASS
Issues requiring fix: none
```

- `audit: unit 1 — 0104-implement-stakes.yaml + three view diffs (schema content) · w3-gate-grader · opus · 4 files · 2 rounds · 0 blocking`
- `audit: unit 2 — implement.md + render (command pair) · w3-gate-grader · opus · 2 files · 2 rounds · 0 blocking`

Round-2 advisories, none blocking:
- Six before-smoke steps now say "on a clean cold smoke" with no "provisionally". The provisional timing binds only through `review-seat-plan.once-per-seat-in-band` and the revert clause in `impl.smoke-probe`. A literal reader would not thin the other five before the smoke, which fails safe. This replaces round 1's "provisionally" note.
- Round-1 thin-form details that left implement are now only in the non-binding example: card review keeping a **TEST:** case per promise line, the sweep losing only hand-run gates, the blind list skipped at one or two gaps, the design-audit reason in its block. The acceptance table still shows each promise line beside its test.
- `impl.sec.tools` sits 391 characters under the 30,000 ceiling, so the next append there will trip it.
- `impl.cold-verification` now says "the head the user accepts", dropping D5's "exact". The meaning holds.

## Unit 8 — crate tests, the non-author code review

Precedent read: the census and matrix comment paragraphs already in the three files, in the shape of the engine-follow-up `7555e3d` re-pins. I authored nothing in `crates/`.

- Diff read whole: `git diff --stat -- crates/` shows three test files, `79 insertions(+), 18 deletions(-)`. No file under `crates/mochiko-cli/src/` changed. Only pins, the sequence list, its description string and comment paragraphs move.
- `cargo test --all --no-fail-fast`, the per-binary `test result:` lines: lib 0 · main 0 · anchor_grammar 5 · cli 62 · conform 61 · fidelity 17 · home 44 · hook 41 · ids 182 · ids_definition_lines 1 · matrix_command 2 · matrix_similar 48 · matrix_skill 3 · migration 38 · render 39 · replay 64 · shell 42 · validate 105 · views 11 · doc-tests 0. Each reads `ok … passed; 0 failed`.
- `MOCHIKO_FULL_SIMILAR=1 cargo test -p mochiko-cli --test matrix_similar`: `test result: ok. 48 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 250.72s`, the live-corpus figures test among them.
- `cargo fmt --all --check` exit 0. `cargo clippy --all-targets -- -D warnings` finishes with no warning.
- Replayed state: `migrate status` reads `sequences 1..104 (92 migrations)` and `94 documents · 1283 rules`. `migrate validate --report` reads `rules scanned: 1283 · in-kind pairs scored: 225052 · clusters: 0` and `allowlist-suppressed edges: 182`, with `pointer resolution: 91`.
- Raw runs with no allowlist, on a scratch copy of the plugin with the log cut by `--log-dir`. At `1..101`: `clusters: 76`, `185 edges unsuppressed`. At `1..103`: `77`, `186`. At `1..104`: `76`, `182`. From `1..101` to `1..103` the listing differs by one added cluster, `testing-end-user.smoke-seat` with `testing-gap-finding.seat-devils-advocate`. From `1..103` to `1..104` the plan-approval cluster shrinks to `brainstorm` and `impl`, and the `arch`/`impl`/`spec` staffing-latitude cluster is gone. Those are four edges, each named by an allowlist row (lines 140, 142, 186, 188), and the allowlist is unchanged this wave.

```
VALIDATE: unit 8 — crates/mochiko-cli/tests/{fidelity,validate,matrix_similar}.rs (crate non-author review)
Checklist run: rust-cli.md quality gate (test · fmt · clippy, the full similarity sweep) and an independent non-author read of the diff, every pin and comment claim reproduced from the replayed state
Evidence read: git diff -- crates/ (whole); 0102, 0103, 0104 op lists (mint ids, schemas, classes, kinds; rewords; conditions; reword-section; set-rule-field; set-moment); per-kind live-rule counts from .mochiko/schema-views at the tree and at 8a90439; migrate status and validate; three raw cluster listings; scripts/similar-rules-allowlist.yaml
Pre-pass: as above
PASS census pins — command 424 → 426 and skill 842 → 857 (+7 from 0102, +8 from 0103, +2 from 0104), total 1,283, skill floors 266 → 270 (0102's four floors), command floors 130 and fail nodes 41 unmoved, as both renders and the implement pins show.
PASS census comments — every named mint matches its migration: 0102's three musts and four floors by id, 0103's five testing-end-user mints and one each on testing-gap-finding, review-seat-plan and review-sufficiency, 0104's impl.lowering-trail and impl.smoke-probe; the 23 rewords split 3, 1 and 19 (17 on implement); two-part-trigger, attempt-per-grade and both fails stay floors; 0103's reword-section, five stakes conditions (1, 3 and 1), 0104's moment and twelve set-rule-field carry no rule.
PASS sequence pin — the list gains 102, 103 and 104, and the description string names each; docs 94; "import none" holds, as no import op exists in the three.
PASS corpus matrix pin — the mints' kinds are constraint 6, duty 5, binding 3, reservation 1, latitude 2. Live counts move constraint 592 → 598, duty 168 → 173, binding 187 → 190, reservation 67 → 68 and latitude 12 → 14 at 8a90439 and now. The pair deltas 3,567 + 850 + 564 + 67 + 25 = 5,073 bring 219,979 to 225,052. Suppressed 185 → 182 is +1 then −4, as the raw runs show.
PASS command-family pin — `impl.lowering-trail` is duty and `impl.smoke-probe` constraint; command duty moves 46 → 47 and constraint 179 → 180, and 46 + 179 = 225 brings 21,068 to 21,293; suppressed 59 → 55 through the four named pairs; clusters 0.
PASS pointer pin — 89 → 91, from `testing-end-user.smoke-report` (0103) and the `stakes-band` pointer (0104); validate prints 91.
PASS house style — the paragraphs follow the precedent: dated build, migration span, mint and reword tallies, by-kind arithmetic, the SUPPRESSED-zero raw-run line naming old, middle and new logs; no production code touched; fmt clean.
VERDICT: PASS
Issues requiring fix: none
```

- `audit: unit 8 — crate tests (fidelity.rs, validate.rs, matrix_similar.rs · crate non-author review) · w3-gate-grader · opus · 3 files · 1 rounds · 0 blocking`
