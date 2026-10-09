---
report: review
pass: gate
seat: gate-grader
tier: opus
contract: mochiko-cli rules validation-primitive-edit (binary 0.3.0 · grammar 2 · plugin 0.116.0) — floor pin 11, read back before the first unit
wave: brainstorm-target-state wave 1
round: 1
units_graded: 8
units_pass: 8
units_fail: 0
blocking: 0
overage_ruling: review-brainstorm +1,398 HOLDS — genuine new obligation (D8, D10, D16, D17), nothing the v0.83.0 cut removed returns
---

## Notes of note

- Tiering: no `Explore` seat spawned. Every read was grade-deciding (rule text against cards, figures against first-hand output) and stayed on the seat tier.
- Ledger: the `review-brainstorm` row reads "argued in the gate-audit brief"; the ruling above (HOLDS) is the lead's to write into the row and into the CHANGELOG placeholder.
- `build-log.md` (P1 entries) gives renders 20,693 and 11,126; first-hand they are 20,700 and 11,133 — a constant 7, one trailing newline per block. CHANGELOG and ledger carry the first-hand figures.
- Advisory, not a defect: `analysis-iterative`'s Overview ("Conclude with a structured synthesis document") and General analysis ("concludes with a synthesis document") still state the synthesis default. They read true under the repaired Output line's "when it names none", sit under the v0.63.0 kept line, and D21 ruled two sentences only. A later ruling could align them.
- Advisory: `review-brainstorm.verify-pass-grade` now reads only changed cards and gists, while `brainstorm.reopen-born-verify` still names "record-fitness" for a reopen-born decision. D17 ruled the narrowing; whole-record fitness on a reopen is a candidate watch, not a build defect.
- Advisory: the new `condition-coverage` finding (`map_timing` value "end" named by no rule) is by design — the end case is the floor's default form.
- Carried lead reading: "one probe" (D8) and "asked once more" (D18) are card wording, read as forms and not as D22 counted limits; build-log says it is to be told to the user.
- Not run here: the contract suite's Docker run and the full similarity sweep — release gates the lead runs (wave plan §5).

## Pre-pass (shared run, first-hand)

- `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` gives `mochiko-cli migrate validate · 0 rejecting · 113 advisory`; sweep `none — no pair clears the threshold`, `rules scanned: 1168 · in-kind pairs scored: 188198 · clusters: 0 (none)`, `allowlist-suppressed edges: 185`; `pointer resolution: 83 checked against plugins/mochiko`.
- Advisory delta against a replay of the `2e4c57c` plugin tree (also `0 rejecting · 113 advisory`): +1 `condition-coverage · skill/review-brainstorm · map_timing · value "end"`, −1 `zero-member-label · command/brainstorm · scope-entry`. Nothing else moved.
- `mochiko-cli migrate status` gives `sequences 1..45 (43 migrations)` and `state sha256:9d106d4025368f15370c1d3eefdbaec2a8c7c52a623f7015d5644ef8528d792d · 87 documents · 1168 rules`; pre-wave `sequences 1..43 (41 migrations)`, `sha256:71c099ee… · 87 documents · 1139 rules`.
- Views: `mochiko-cli views emit` into scratch, `diff -r` against `.mochiko/schema-views/` — identical, all 87 documents.
- Pins: the `kind: fail` / `class: floor` pin lines and the `floors:` line hash identically pre vs post for `brainstorm` (`4 rules` · `8 rules`) and `review-brainstorm` (`9 rules`).
- Char budget, ledger canonical snippet (characters of the parsed value; render = the seven `!`-line blocks summed): `review-brainstorm` body 3,089 · description 490 · render 11,133 · payload 14,222 (pre-wave body 2,833 · render 9,733 · payload 12,566); `analysis-iterative` body 4,222 (pre 4,120) · description 476 (unchanged).
- Renders (unbudgeted, recorded): `brainstorm` 11,829 → 20,700; `review-brainstorm` 9,733 → 11,133.
- Crate: `cargo test -p mochiko-cli` 581 passed, 0 failed (every binary `test result: ok`); `cargo fmt --all --check` exit 0; `cargo clippy --all-targets -- -D warnings` exit 0.
- Kit: `rubric OK: 45 observable, 10 out-of-instrument, 55 total`; `fixture consistency: OK`; `command partition brainstorm --old-ref 2e4c57c` gives unchanged 26 · changed 4 · removed 0 · added 25.

## Unit 1 — schema content `0044-brainstorm-target-state`

VALIDATE: schema content `plugins/mochiko/migrations/0044-brainstorm-target-state.yaml` + `.mochiko/schema-views/commands/brainstorm.yaml` diff + `scripts/similar-rules-allowlist.yaml` (three rows)
Checklist run: the AM-2-required-cli-dependency five, with wave plan §3.0 (D22) as the register bar
Evidence read: the migration whole · the view diff · the allowlist diff · the render of all seven `brainstorm` blocks · record cards D1–D15, D17–D19, D22 (`record.md:205–764`) · `wave1-target-state.md` §3 · `build-log.md` P1 entries
Pre-pass: `0 rejecting`. `brainstorm` advisories are the pre-existing set (`km_file`/`seats` coverage, two unused moments, `enforces-coverage` 7, `budget 55 rules`). Sections 8 · 8 · 9 · 25 · 1 · 4 = 55; end lines agree with the preamble.
- PASS intent stated: the header names the decisions, the three counted limits, the `size` condition at a new moment, the moved blind map with the small arm kept, four rewords, the mints, and "No floor and no fail is minted, retired or lowered". The ops match: 2 `set-moment`, 1 `set-condition`, 4 `reword-rule`, 1 `set-rule-field`, 25 `mint-rule`.
- PASS anchor present where required: header `2026-10-06 brainstorm-target-state`; each single-decision mint carries its D; `user-steers` carries the bare session anchor (D4/D6/D12/D15). `blind-map-dispatch` keeps "`2026-08-10 cold-review-gap-challenge D6`" by the lead's F-1 ruling; the reword rides the header anchor and intent, and the cold-review record and `DECISIONS.md` row are annotated "D2/D6 amended in part 2026-10-06". No exit occurs, so no exit anchor is owed.
- PASS ID lifecycle: the four rewords keep their ids (partition `changed` lists exactly them); 25 new ids; no tombstone; 30 → 55. `small-session-review` is D11's mint for the small arm, not a split — the parent id lives on, reworded and `when:`-gated.
- PASS floor and fail survival: 8 floors and 4 fails, `floors:` line identical pre/post; no reword touches a floor or fail; `fail.survivor-undispositioned` still enforces `non-coverage-survivors`, whose routing survives the reword.
- PASS register: every mint is one or two plain sentences in second person; no "first … then", no numbered step; counted limits only in `ratified-yes`, `stop-rule`, `fix-one-card-verify-once`. Each text sits inside its card, review notes included: S2 (`blind-map-fold`'s late angle), S5 (`question-form`'s open question), S6 (`facts-quoted`), V3 (`stop-rule`'s blind-map clause), S16 (`blind-map-dispatch`'s pair), D22 on D12 (`dependency-order` leaves the order to the lead).
- PASS allowlist rows: three distinct-object reasons, all three live (suppressed 182 → 185, clusters 0).
VERDICT: PASS
Issues requiring fix: none

## Unit 2 — schema content `0045-review-brainstorm-target-state`

VALIDATE: schema content `plugins/mochiko/migrations/0045-review-brainstorm-target-state.yaml` + `.mochiko/schema-views/skills/review-brainstorm.yaml` diff
Checklist run: the AM-2-required-cli-dependency five
Evidence read: the migration whole · the view diff · the render of all seven `review-brainstorm` blocks · record D8, D10 (with S2, S7, S16 notes), D11 S3, D16 (with S4 and the Q23 build note), D17 · `wave1-target-state.md` §3.2
Pre-pass: `0 rejecting`; `budget · skill/review-brainstorm · 34 rules · 3800`; new advisory `map_timing` "end" uncovered. Sections 4 · 6 · 6 · 13 · 3 · 2 = 34.
- PASS intent stated: names D8, D10, D16, D17, the condition, the floor reword "(id, class, kind and anchor kept)", the narrowed verify, four mints, "No floor is minted, retired or lowered". Ops match: 1 `set-condition`, 2 `reword-rule`, 4 `mint-rule`.
- PASS anchor: header anchor present; `resumed-end-read` D10, `card-parts-first` D16, `builder-test` D16, `ratified-first` bare (D8 with D16); the reworded floor keeps `2026-08-10 cold-review-gap-challenge`.
- PASS ID lifecycle: two rewords keep ids; four new ids; no tombstone; 30 → 34.
- PASS floor survival: `class: floor · 9 rules` and the `floors:` line identical pre/post. The floor's fence — its own deliverable before any record contact — survives; its input widens from "the topic only" to the two brief forms, which D10 (problem, destination, out-of-scope) and D11 S3 (small keeps today's map; floor not changed) rule. The fallback seat is governed by `resumed-end-read`'s "draw no second map" — coherent.
- PASS register: plain short texts; S7's stake note correctly left as accepted risk, not minted as a duty.
VERDICT: PASS
Issues requiring fix: none

## Unit 3 — command pair `brainstorm`

VALIDATE: command pair `brainstorm` — `plugins/mochiko/commands/brainstorm.md` + `mochiko-cli rules brainstorm` (seven blocks)
Checklist run: canonical-scaffold criteria 1–11, run-command branch of item 7
Evidence read: `brainstorm.md` (file + diff) · the seven rendered blocks · `.mochiko/strips/brainstorm.md` [v0.117.0] entries · record D6, D9
Pre-pass: as Unit 1. Pins `kind: fail · 4 rules` · `class: floor · 8 rules`; six end lines agree.
- PASS scaffold headings and order: frontmatter key set, `# Brainstorm — Think Together, Review Cold`, Identity & Mission, the Rules block with seven `!` lines, Entry → Goal → Not done — unchanged; only two prose passages moved.
- PASS preserved responsibilities: "one question at a time" leaves by D6, the per-decision standard widens to the card by D9; both strip entries quote the verbatim prior text, matching the diff; the confidence marks stay verbatim.
- PASS floor survival: 8 floors and 4 fails unchanged.
- PASS independence: the review seat is never the author; the blind-second-list seat is "never the review seat"; the resumed seat's stake in its own angles is D10 S7's ruled risk.
- PASS reserved-to-user: the three new reservations sit in `brainstorm.sec.reserved`; the acceptance floor is unchanged.
- PASS done-condition, run branch: Entry unchanged; the Goal is fixed and its `brainstorm.sec.tools` token resolves to a live section; Not done cites the pin, no hard-coded count, halt clause kept.
- PASS argued overage: commands are unbudgeted; none applies.
VERDICT: PASS
Issues requiring fix: none

## Unit 4 — skill pair `review-brainstorm`

VALIDATE: skill pair `review-brainstorm` — `SKILL.md` + render (seven blocks) + `references/RECORD-FITNESS.md` + strip entries + ledger row
Checklist run: skill-pair criteria 1–12, with the named overage
Evidence read: `SKILL.md` (file + diff) · `RECORD-FITNESS.md` diff · the seven rendered blocks · `.mochiko/strips/review-brainstorm.md` [v0.117.0] entries and the [v0.83.0] cut entry · the `review-brainstorm` row of `.mochiko/memory/primitive-cost-budgets.md` · record D16 with its Q23 note
Pre-pass: `0 rejecting`; pointer `references/RECORD-FITNESS.md` resolves (83 checked); payload 14,222 vs budget 12,824.
- PASS load-first section and read-back: seven `!` lines, no raw Read, the read-back cites the pin and the `floors:` line — unchanged.
- PASS preserved responsibilities: the Protocol strip keeps "Blind angle map first, then the cold read", the six hunt classes verbatim and in order (`hunt-classes-per-decision` cites them), and the Verify pass byte for byte. The fitness intro strip keeps all seven items byte for byte. The Card parts section carries the user's ruling in the record's words: "a missing part does not itself block `ready`".
- PASS floor survival: 9 floors; the one floor reword is Unit 2's.
- PASS independence and reserved: `sec.independence` and `sec.reserved` keep their floors; no seat grades its own output.
- PASS `description:` untouched: no frontmatter change in the diff; 490 ≤ 1,536.
- PASS done-condition: no branch applies to a skill pair.
- PASS argued overage, HOLDS: +1,398 = render +1,400 (9,733 → 11,133 first-hand, all from `0045`) + body +256 (2,833 → 3,089) − 258 pre-existing slack (pre-wave payload 12,566 against 12,824). Each `0045` addition carries a ruled decision (D8, D10, D16, D17). The body names where each new step falls. The [v0.83.0] cut deleted none of these concepts — the card checklist, ratified-first, the builder test and the front map all postdate it.
VERDICT: PASS
Issues requiring fix: none

## Unit 5 — prose primitive `analysis-iterative`

VALIDATE: prose primitive `plugins/mochiko/skills/analysis-iterative/SKILL.md` + `.mochiko/strips/analysis-iterative.md` [v0.117.0] + its ledger row
Checklist run: coherence · preserved responsibilities · char budget
Evidence read: `SKILL.md` whole + diff · the strip diff (three entries, the v0.63.0 kept line's appended pointer) · the ledger row · record D19 (with S5, V4) and D21
Pre-pass: `0 rejecting`; body 4,222 ≤ 4,928; description 476 unchanged.
- PASS coherence: exactly the two D21 sentences changed. "The adaptive questioning the Overview describes" resolves to Overview line 10. Output defers to the caller and keeps the synthesis when none is named. Setup names `governance-intent.md`, so its output is unchanged; specify's enrichment shape is untouched. The residual Overview wording is advisory (Notes).
- PASS preserved responsibilities: the Common Mistakes rows are byte-identical. D19 supersedes them for brainstorm only, through `brainstorm.own-rules-win`; the strip carries the verbatim table. The v0.63.0 kept line gains a pointer and loses nothing. The floor line and the confidence table are untouched.
VERDICT: PASS
Issues requiring fix: none

## Unit 6 — router, ripple files, ledger, changelog

VALIDATE: prose — `plugins/mochiko/skills/mochiko/SKILL.md` (one row) · `.mochiko/strips/mochiko.md` · `README.md` · `ARCHITECTURE.md` · `.mochiko/memory/primitive-cost-budgets.md` rows · `CHANGELOG.md` 0.117.0 (placeholders ungraded)
Checklist run: coherence · preserved responsibilities · every figure against first-hand output
Evidence read: the six diffs · `ARCHITECTURE.md:156–181` · repo grep for "one question at a time / per turn", "two-message", "Phase 0" across `README.md`, `ARCHITECTURE.md` and `plugins/mochiko/{commands,skills,agents,templates}`
Pre-pass: as above.
- PASS router row: " one question at a time" removed (23 characters, matching the strip's 47,326 → 47,303); the strip entry is complete.
- PASS ripple: `README.md:61`, `:130` and `ARCHITECTURE.md:161`, `:175` are true against the rules. Remaining "one question per turn" hits are setup/specify surfaces D19 keeps (`INTERROGATION-AGENDA.md:5,27`, `SPECIFICATION-INPUT.md:13`, the kept Common Mistakes row).
- PASS ledger rows: both reproduce first-hand (Unit 4 arithmetic; `analysis-iterative` 4,120 → 4,222).
- PASS changelog: the census (335 → 360, 804 → 808, 1,168, 87 documents), `0 rejecting · 113 advisory`, clusters 0, suppressed 182 → 185, renders, payload, family tuple, pointers 82 → 83 and rubric 45 of 55 all match. The clauses keep the cards' words (D15's "says so and offers"), and the user's 2026-10-06 ruling is credited.
VERDICT: PASS
Issues requiring fix: none

## Unit 7 — crate test re-key (independent non-author code review)

VALIDATE: `crates/mochiko-cli/tests/fidelity.rs` · `validate.rs` · `matrix_similar.rs`; `evals/contract/` freeze check
Checklist run: `.claude/rules/mochiko/rust-cli.md` quality gate; pins against the replay
Evidence read: the three diffs · `git status` of `crates/mochiko-cli/src` and `Cargo.toml` (clean) · `git diff 2e4c57c -- evals/contract` (empty) · `expected-skills.json` `review-brainstorm` row · `run.py` `EXPECTED["brainstorm"]` and `report()`
Pre-pass: 581 passed, 0 failed; fmt exit 0; clippy exit 0.
- PASS source untouched: pin literals and comments only.
- PASS pins reproduce: 360 / 808 / 1,168 (status `1168 rules`); sequences end `44, 45`; family pairs +2,065 +177 +54 +51 +5 = +2,352, so 13,050 → 15,402; corpus +7,581 +1,415 +180 +167 +23 = +9,366, so 178,832 → 188,198 (validate `188198`); suppressed 185 and pointers 83 equal validate's output.
- PASS contract freeze: the `review-brainstorm` floor_ids (9) and the `brainstorm` frozenset (8) equal the rendered `floors:` lines; `baseline_bytes` are pre-conversion constants; read cost is `report()` (recorded, never asserted).
VERDICT: PASS
Issues requiring fix: none

## Unit 8 — eval kits

VALIDATE: `evals/plan/brainstorm/{observable.yaml,evals.json,preregistration.md}` · `evals/plan/README.md` · `evals/review-brainstorm/{rules.json,rekey.md}`
Checklist run: `evals/plan/README.md` and the kit audit's conventions (sibling buckets; old-ref named; control retires on the next edit)
Evidence read: the six diffs · the partition JSON · a script comparing `rules.json` with the render
Pre-pass: rubric OK 45 · 10 · 55; fixtures OK; partition 26 · 4 · 0 · 25.
- PASS partition: 23 of 25 additions observable. `own-rules-win` is out as `precedence`, a term defined in the header; `size-shapes` is conditional. Siblings share buckets as the 2026-09-19 audit asked.
- PASS preregistration: 3 × 45 = 135 pairs; ≤ 20 of 135 is the 15 % line; three judge chunks of ≤ 15; six by-design unreachable pairs named. The positive control is `frame-card`, at `--old-ref 2e4c57c`, and the prior control is retired under its own rule. The README figure is corrected: the old "21 of 30" was stale, since 22 · 8 per the preregistration.
- PASS goldens: `args` and `control_prompt` unchanged; expected output and assertions re-keyed to the new rules and the expected size.
- PASS `rules.json`: id order, texts and classes equal the render for all 34; floors 9; `rekey.md` gains a section and edits no prior one.
VERDICT: PASS
Issues requiring fix: none

## Outcome lines

- audit: 0044-brainstorm-target-state (schema content) · gate-grader · opus · 6 files · 1 rounds · 0 blocking
- audit: 0045-review-brainstorm-target-state (schema content) · gate-grader · opus · 4 files · 1 rounds · 0 blocking
- audit: brainstorm (command pair) · gate-grader · opus · 3 files · 1 rounds · 0 blocking
- audit: review-brainstorm (skill pair) · gate-grader · opus · 5 files · 1 rounds · 0 blocking
- audit: analysis-iterative (prose primitive) · gate-grader · opus · 4 files · 1 rounds · 0 blocking
- audit: router, ripple, ledger and changelog (prose) · gate-grader · opus · 6 files · 1 rounds · 0 blocking
- audit: crate test re-key (code review) · gate-grader · opus · 5 files · 1 rounds · 0 blocking
- audit: brainstorm and review-brainstorm eval kits · gate-grader · opus · 6 files · 1 rounds · 0 blocking
