---
report: review
round: 1
grader: w2-gate-grader
tier: opus
wave: 2
units: [1, 2, 3, 4, 5]
contract: mochiko-cli rules validation-primitive-edit (preamble + six sections, binary 0.4.0, plugin 0.123.0) — floor pin 11, read back before the first step
---

## Pre-pass (shared, run first-hand by the grader)

- Contract read-back before the first step: `- class: floor · 11 rules`; floors author-grader · plain-seat-explicit-tier · rendered-contract-only · from-file-floor · pre-pass-first-hand · binary-verdict · default-fail · tamper-proof-clause · gate-loop-bound · evidence-floor · second-fail-user.
- `mochiko-cli migrate validate --report --plugin-root plugins/mochiko`: `pointer resolution: 91 checked against plugins/mochiko` · `similar-rule clusters (threshold 0.60) … none — no pair clears the threshold` · `rules scanned: 1288 · in-kind pairs scored: 227063 · clusters: 0 (none)` · `allowlist-suppressed edges: 182` · `mochiko-cli migrate validate · 0 rejecting · 121 advisory`.
- `mochiko-cli migrate status`: `sequences 1..106 (94 migrations)` · `state sha256:1bc8198c676d…` · `94 documents · 1288 rules`. Control (scratch copy of `plugins/mochiko`, `0106` removed, `specify.md` / router / `review-specifications` `SKILL.md` at `e9a13e2`): `sequences 1..105 (93 migrations)` · `state sha256:e30d0f115ded…` · `1284 rules` · `0 rejecting · 118 advisory`.
- Live vs control report diff, cluster listing aside (the copy outside the repo finds no allowlist): `enforces-coverage · command/specify` gains `spec.fail.skip-unstated` (15 → 16, the same shape every specify fail node already carries) · `budget · command/specify · 73 rules · 16986` → `76 rules · 19194` · `budget · skill/review-seat-plan · 16 rules · 4967` → `5185` · `budget · skill/review-specifications · 33 rules · 7601` → `34 rules · 8842` · three new `condition-coverage · skill/review-specifications · stakes · value "money" | "data" | "security" … named by no rule's when:` (the +3 advisory). No new pointer, `zero-member-label` or cite finding.
- `mochiko-cli views emit --out <scratch>` then `diff -r` against `.mochiko/schema-views`: `VIEWS IDENTICAL` (derived view ≡ replay). `scripts/similar-rules-allowlist.yaml` unmodified (`git status`).
- Rule-id set diff, control vs live renders: specify `+ spec.fail.skip-unstated · + spec.lowering-trail · + spec.stress-test-quick-read`; review-specifications `+ review-specifications.quick-read`; review-seat-plan none. Floors-line diff: specify `+ spec.fail.skip-unstated` only; the two skills unchanged. Pins: specify fail 10 → 11, floor 19 → 20; section end lines 14 + 8 + 25 + 14 + 4 + 11 = 76.
- Char budget, canonical snippet (body, description) plus the seven render blocks as characters of stdout:

  | skill | body | desc | preamble | indep. | scope | inputs | verdict | output | reserved | render | payload | budget |
  |---|---|---|---|---|---|---|---|---|---|---|---|---|
  | review-specifications now | 3,876 | 535 | 2,335 | 529 | 1,809 | 3,516 | 6,089 | 1,650 | 513 | 16,441 | 20,317 | 16,174 |
  | review-specifications control | 3,471 | 535 | 2,008 | 529 | 1,809 | 2,110 | 6,089 | 1,650 | 513 | 14,708 | 18,179 | |
  | review-seat-plan now | 3,534 | 839 | 1,974 | 1,531 | 2,085 | 1,080 | 2,309 | 894 | 696 | 10,569 | 14,103 | 12,701 |
  | review-seat-plan control | 3,534 | 839 | 1,894 | 1,531 | 1,875 | 1,080 | 2,309 | 894 | 696 | 10,279 | 13,813 | |

  Router body (unbudgeted) 50,337 → 50,483, description 206 unchanged. Every figure in the two ledger rows and the router strip comment reproduces exactly.
- Specs-home conformance (lead's ask a): spec bodies built from `mochiko-cli template spec`, header `> Spec:` · `> Created:` · `> Status:` and then (i) nothing, (ii) two `> Lowered: <step> · stakes: cosmetic — "<cited line>" · covered by: …` lines, (iii) `> Lowered: none`, each dry-run `mochiko-cli check --plugin-root plugins/mochiko --path .mochiko/specs/004-list-export-text/spec.md --content -`: all three `"permissionDecision":"allow"`, exit 0. Negative control: the same body plus `## Bogus` denies (`` `## Bogus` is not a declared heading``), exit 4, so the check was live. No file written under `.mochiko/specs/`.
- Routing disclosure: no `Explore` subagent was dispatched. The locate reads (ledger snippet, router row lines) were a handful of greps, done inline; every other read was interpretive.

## Unit 1 — schema content: `0106-specify-planning-stakes.yaml` + view diff

```
VALIDATE: unit 1 — plugins/mochiko/migrations/0106-specify-planning-stakes.yaml + git diff of .mochiko/schema-views/{commands/specify,skills/review-specifications,skills/review-seat-plan}.yaml (schema content)
Checklist run: AM-2 five — intent stated · anchor present where required · ID lifecycle right · floor and fail survival · register
Evidence read: 0106 whole; the three view diffs whole; record.md D1-planning-quick-read (statement, items 1–11), D2-borrow-product-label, D3-lead-proposes-label, DF1–DF6, OQ1–OQ3; wave2-specify.md §3; rendered patterns-sound-loop planning-quick-read, trail-of-lowerings, stakes-key, stakes-band, raise-free-lower-ruled, review-form-invariants, never-thins (0105); specify, review-specifications, review-seat-plan renders, live and control
Pre-pass: migrate validate 0 rejecting · 121 advisory; views emit ≡ .mochiko/schema-views; status 1..106 · 1288 rules
PASS intent stated — header names every op and its card: stakes condition and intent moment (D2, D3, DF5), reserved-to-user (D3), stress-test and blind list keyed (D1, DF4), quick-read mint (D1 + S4), trail in spec.md's header (OQ3), skip-unstated (trail-of-lowerings), two enforces widenings (D1 item 7), review-specifications condition + quick-read + complete-coverage carve (D1 items 1, 10), review-seat-plan note + latitude (DF4). Its counts (specify 73→76, floors 19→20, fails 10→11; review-specifications 33→34, floors 8; review-seat-plan 16, floors 5) match the pins and validate's budget lines.
PASS anchor present — header `2026-10-28 stakes-non-product-surfaces`; all four mints carry `… D1`; the new floor fail carries it; rewords keep prior anchors under the header (spec.blind-journey-list keeps `2026-10-10 … D4`, once-per-seat-in-band keeps `2026-10-27 … D5`). No floor exits, so no exit anchor owed.
PASS ID lifecycle — four mints with new slugs; five rewords keep id, class, kind, labels, pointer, anchor (view diff touches text, `when:` and `enforces:` only, as the header says); no tombstone; id-set diff shows additions only.
PASS floor and fail survival — no floor retired or lowered; all ten prior fails present; blocking-gap and screens-flows keep their text and gain the quick read in `enforces:` (D1 item 7); every review-specifications floor stands and the quick-read rule says so; the bend is limited to the D8 completed-hunt rule, low label, this review only (D1 item 1).
PASS register — each lowering is gated on the user's ruling, never on a low label alone: one-pass "only where the user ruled that on the stakes line", blind list "where the user ruled that on the stakes line", quick-read mint "Where the user ruled it on the stakes line", skill rule "Where the caller's run ruled the planning quick read". The key, band, raise and trail are cited (stakes-key, planning-quick-read, trail-of-lowerings), never restated; DF6 and the higher-label rule ride stakes-key by reference.
VERDICT: PASS
Issues requiring fix: none
```

Advisory, not blocking: (1) `spec.handed-in-slate` still reads "never the blind journey list" with no qualifier. The new `spec.blind-journey-list` text reconciles the two ("that is its only skip — a handed-in slate never skips it"), but the reconciling clause sits in one rule only. (2) The +3 `condition-coverage` advisories on `review-specifications.stakes` are legal: the condition carries the ruled class whole, and only the quick read keys on it.

## Unit 2 — command pair: `specify.md` + render

```
VALIDATE: unit 2 — plugins/mochiko/commands/specify.md + mochiko-cli rules specify (preamble + six sections) + .mochiko/strips/specify.md [v0.124.0] (command pair)
Checklist run: judgment-items-pair on the canonical-scaffold criteria — scaffold · preserved responsibilities · floor survival · independence · reserved-to-user · done-condition branch (run) · argued overage; lead's asks (a) and (b)
Evidence read: specify.md whole; git diff e9a13e2 of specify.md; all seven rendered blocks (preamble pins, floors line, every stakes-bearing rule read in full); strip entry [v0.124.0]; spec template (`mochiko-cli template spec`); the three conformance dry runs above
Pre-pass: migrate validate 0 rejecting · 121 advisory; pins kind: fail · 11, class: floor · 20; end lines sum 76
PASS scaffold headings and order — frontmatter keys, `# Specify — Feature Specification`, Identity & Mission, Rules block with seven ! lines, Adaptive Goal Protocol Entry → Goal → Not done; untouched apart from the Goal clause.
PASS preserved responsibilities — the Goal's stress-test clause keeps every prior word (strip quotes the pre-edit clause verbatim and matches `git show e9a13e2`); the added words are keyed on the user's ruling ("where the user ruled the quick read on the stakes line"); "with no blocking gap left open" binds both forms (D1 item 7).
PASS floor survival — 19 prior floors in the floors line, + spec.fail.skip-unstated; Not-done cites the printed `kind: fail` pin with its halt clause, no hard number, so the eleventh fail needs no prose edit.
PASS independence — spec.stress-test-quick-read: "one fresh seat that authored none of it", under planning-quick-read's invariants (1)–(4); spec.author-grader-default-fail stands.
PASS reserved-to-user — spec.reserved-to-user (sec.reserved) now holds the stakes line at the intent confirm and any later lowering as a checkpoint ruling (D3, DF5); the quick reader's raise is free, as raise-free-lower-ruled allows.
PASS done-condition branch (run, fixed) — Entry unchanged; the Goal's done condition is still fixed, with the quick read as the ruled alternative; Not done is pin-cited.
PASS argued overage — n/a: command rules are unbudgeted.
PASS ask (a) — the specs-home check admits `> Lowered: …` lines and `> Lowered: none` beneath `> Status:`.
PASS ask (b) — the Goal need not name the `> Lowered:` block. The Goal lists section-level content and names no header line (`> Spec:`, `> Status:`), nor Overview or Key Entities. The trail is bound by spec.lowering-trail, and its failure by spec.fail.skip-unstated, which Not done pulls in through the pin. Naming it in the Goal would restate a rule. The seat's argument holds.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 3 — skill pair: `review-specifications`

```
VALIDATE: unit 3 — plugins/mochiko/skills/review-specifications/SKILL.md + mochiko-cli rules review-specifications (seven blocks) + its primitive-cost-budgets.md row + .mochiko/strips/review-specifications.md [v0.124.0] (skill pair)
Checklist run: judgment-items-pair — scaffold · preserved responsibilities · floor survival · independence · reserved-to-user · done-condition branch · argued overage
Evidence read: SKILL.md whole; git diff e9a13e2 of SKILL.md; all seven rendered blocks in full; ledger row diff; strip entry and wave-context comment; control render at 1..105
Pre-pass: migrate validate 0 rejecting · 121 advisory; payload 20,317 vs budget 16,174 (table above)
PASS scaffold headings and order — frontmatter (description byte-identical, 535 ≤ 1,536), title, opening paragraph, Rules block with seven ! lines in family order, read-back sentence, Procedure; the Rules block and read-back are untouched.
PASS preserved responsibilities — both qualified sentences are kept byte for byte, and the strip quotes them verbatim; the additions are keyed on the ruling ("Where a /mochiko:specify run's user ruled the quick read on its stakes line"; "Where the caller's run ruled the quick read on its stakes line"). They keep the body true: without them the Procedure's unconditional six-class hunt would contradict the delivered rule.
PASS floor survival — floors line unchanged (8 ids); quick-read ends "every floor of this skill stands"; gap-bound, input-not-verdict, map-git-baseline and evidence-floor do not collide with the dropped hunt.
PASS independence — author-grader floor unchanged; the quick reader is a non-author seat by spec.stress-test-quick-read.
PASS reserved-to-user — reserved section unchanged (input-not-verdict); the rule lets the reader raise the label, never lower it (D3).
PASS done-condition branch — n/a: a skill carries no Goal protocol or Not-done pin.
PASS argued overage — +2,138 this wave reproduced exactly (render +1,733 = preamble +327 + inputs +1,406, from 0106's three ops on this skill, which the row names in full; body +405); standing +4,143. Ruled HOLDS: D1 items 1 and 10 rule the quick-read form into this skill. The body sentences are the minimum that keeps the prose true, and no playbook prose is restored.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 4 — skill pair: `review-seat-plan`

```
VALIDATE: unit 4 — plugins/mochiko/skills/review-seat-plan/SKILL.md + mochiko-cli rules review-seat-plan (seven blocks) + its primitive-cost-budgets.md row (skill pair)
Checklist run: judgment-items-pair — scaffold · preserved responsibilities · floor survival · independence · reserved-to-user · done-condition branch · argued overage
Evidence read: SKILL.md (git diff empty; grep for stakes/run-open/smoke/band/product finds nothing, so no body text is made false); all seven rendered blocks, the stakes note and once-per-seat-in-band in full; ledger row diff; control render at 1..105
Pre-pass: migrate validate 0 rejecting · 121 advisory; payload 14,103 vs budget 12,701
PASS scaffold headings and order — SKILL.md untouched; description 839 unchanged.
PASS preserved responsibilities — the reword keeps every prior clause (smoke-provisional thinning, re-staff / re-scope / post-FAIL regrade, outside-band full grade, band and trail referenced) and adds the planning branch (DF4: the plan check lightens, never vanishes); no strip owed, since no prose was removed and schema content takes none.
PASS floor survival — floors line unchanged (5 ids); the latitude stays must · latitude.
PASS independence — fresh-peer-grader floor unchanged.
PASS reserved-to-user — the thinning rides "the plan the user ruled at run-open — on a planning output, at the command's confirmation point".
PASS done-condition branch — n/a: skill.
PASS argued overage — +290, render only (preamble +80, scope +210), from 0106's two ops on this skill, both named in the row; standing +1,402. Ruled HOLDS: a DF4 ruled obligation, not restored prose.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 5 — prose primitive: router `mochiko` `SKILL.md`

```
VALIDATE: unit 5 — plugins/mochiko/skills/mochiko/SKILL.md (/mochiko:specify row, line 140) + .mochiko/strips/mochiko.md [v0.124.0] (prose primitive)
Checklist run: judgment-items-prose — coherence · preserved responsibilities
Evidence read: git diff e9a13e2 of the router (one row); strip comment and entry; the router's review-specifications (67) and review-seat-plan (134) rows, checked for statements the wave made false; body measured 50,337 → 50,483
Pre-pass: migrate validate 0 rejecting · 121 advisory (router carries no rules)
PASS coherence — both additions are keyed on the user's ruling ("where you rule the skip on a low stakes line"; "a quick read where you rule it on a low stakes line, the open gap hunt not run") and agree with spec.blind-journey-list and spec.stress-test-quick-read; rows 67 and 134 state no fixed depth, so neither is made false.
PASS preserved responsibilities — strip quotes both phrases verbatim, kept byte for byte, cites D1, DF4, D3; line pointer :140 resolves; consumers assessed.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 6 — crate tests, the non-author code review

```
VALIDATE: unit 6 — crates/mochiko-cli/tests/{fidelity,matrix_similar,validate}.rs (crate non-author review)
Checklist run: .claude/rules/mochiko/rust-cli.md quality gate (test · fmt · clippy · the opt-in full similarity sweep) and an independent read of the diff, with every pin and comment claim reproduced
Evidence read: git diff -- crates (whole, three test files, no src/ change); W1 precedent git show e9a13e2 -- crates; 0106 op list; unit-1 pre-pass (validate, status, renders); raw no-allowlist validate runs over scratch copies at 1..105 and 1..106; per-kind rule counts from the views (≡ replay); scripts/similar-rules-allowlist.yaml searched for every touched or minted id (no match)
Pre-pass: `cargo test --all --no-fail-fast` 20 `test result:` lines, 765 passed · 0 failed, exit 0 · `cargo fmt --all --check` exit 0 · `cargo clippy --all-targets -- -D warnings` exit 0 · `MOCHIKO_FULL_SIMILAR=1 cargo test --release --test matrix_similar the_detector_reproduces_the_live_runs_figures_over_the_corpus`: `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 47 filtered out`
PASS census pins — four mints, no retire: command 426 → 429, skill 858 → 859, total 1288 (status prints 1288), command floors 130 → 131 (the six command pins 25 + 9 + 16 + 40 + 21 + 20), fail nodes 41 → 42 (specify kind: fail 11), skill floors held at 271 (36 skill pins sum to 271).
PASS census comments — mints (3 specify + 1 review-specifications), six reword-rules, four set-rule-fields, three set-conditions and one set-moment all match 0106's ops by id and count. The two fail nodes that gain an `enforces:` entry stay floors.
PASS sequence pin — the list gains 106; docs held at 94; "import none" holds (0106 has no import op); the assertion message names 0106.
PASS matrix pins — per-kind counts from the views: command constraint 181, duty 48, fail 42; corpus constraint 601, duty 174, fail 42, as the comments state. Command family +180 + 47 + 41 = +268 gives 21,293 → 21,561; corpus +1,199 + 173 + 41 = +1,413 gives 225,650 → 227,063 (validate prints 227063); clusters 0; suppressed 182 and 55 held.
PASS SUPPRESSED-zero comments — raw runs read `clusters: 76` and `allowlist: none (182 edges unsuppressed)` at both 1..105 and 1..106, and the two cluster listings diff identical; no allowlist row names a rule 0106 mints or rewords.
PASS provenance — every new comment cites the 2026-10-28 stakes-non-product-surfaces session: fidelity "waves 1 and 2" and "the same session's D1"; matrix_similar "the 2026-10-28 stakes-non-product-surfaces wave 2 (`0106`)"; validate "`0106` (its wave 2)", directly after the 0105 paragraph that names the session.
PASS house style — follows the e9a13e2 precedent paragraph for paragraph (dated wave, migration, mint/reword tally, by-kind arithmetic, raw-run line); no production code touched.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 7 — contract freeze

```
VALIDATE: unit 7 — evals/contract/run.py (EXPECTED["specify"]) + evals/contract/README.md:274 + evals/contract/expected-skills.json (unchanged) (contract freeze)
Checklist run: every frozen floor set and pin against the live render's `floors:` line · README tally coherent with the ruling · provenance comment present
Evidence read: git diff of run.py and README.md; run.py EXPECTED parsed with ast (all six commands); expected-skills.json `skills` parsed (all 36 rows); every live preamble's floors line and floor pin
Pre-pass: migrate validate 0 rejecting · 121 advisory; `git diff --quiet -- evals/contract/expected-skills.json` true (untouched)
PASS run.py frozen sets — each command's frozen id set equals its live floors line: architecture 25, brainstorm 9, feature 16, implement 40, setup 21, specify 20 (gains spec.fail.skip-unstated), totalling 131. The frozen baseline bytes (specify 23_434) are untouched, which is correct for a freeze; the provenance comment cites `0106-specify-planning-stakes` ("2026-10-28 stakes-non-product-surfaces D1").
PASS expected-skills.json — all 36 rows equal their render in floor_ids and floor_pin (total 271): review-specifications 8/8, review-seat-plan 5/5, patterns-sound-loop 11/11. No row is owed a change, since 0106 moves no skill floor.
PASS README row — commands 130 → 131, with "`spec.fail.skip-unstated` by the 2026-10-28 stakes-non-product-surfaces ruling" appended to the chain; skills column unchanged at 271.
VERDICT: PASS
Issues requiring fix: none
```

The deterministic contract suite (Docker sandbox) was not run here. It remains a release gate at the bump, and a SKIPPED run is not green.

## Notes of note

- Pre-existing, outside these units: a spec filled from `mochiko-cli template spec`'s skeleton with all six Intent bullets and the skeleton's `---` separator is denied (`` `## Intent` is 11 lines against a budget of 10``). The control body with no `> Lowered:` line denies identically, so 0106 did not cause it. Worth a backlog line: either the template or its Intent budget is off by the separator.
- `> Lowered: none` has no fail behind it. spec.fail.skip-unstated reads only a thinned step with no line, so a run where nothing thinned and the line is missing fails nothing. This is legal under the ruling and is noted only.
- The two strip headings use "a … run rules" / "the user rules". Both are keyed on a ruling; neither says a low label alone triggers the lighter form.

## Outcome lines

- `audit: unit 1 — 0106-specify-planning-stakes.yaml + view diff (schema content) · w2-gate-grader · opus · 4 files · 1 rounds · 0 blocking`
- `audit: unit 2 — specify.md + render (command pair) · w2-gate-grader · opus · 2 files · 1 rounds · 0 blocking`
- `audit: unit 3 — review-specifications SKILL.md + render (skill pair) · w2-gate-grader · opus · 3 files · 1 rounds · 0 blocking`
- `audit: unit 4 — review-seat-plan SKILL.md + render (skill pair) · w2-gate-grader · opus · 2 files · 1 rounds · 0 blocking`
- `audit: unit 5 — mochiko router SKILL.md (prose primitive) · w2-gate-grader · opus · 2 files · 1 rounds · 0 blocking`
- `audit: unit 6 — crate tests (fidelity.rs, matrix_similar.rs, validate.rs · crate non-author review) · w2-gate-grader · opus · 3 files · 1 rounds · 0 blocking`
- `audit: unit 7 — run.py + contract README, expected-skills.json cross-checked (contract freeze) · w2-gate-grader · opus · 3 files · 1 rounds · 0 blocking`
