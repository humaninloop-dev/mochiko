---
report: review
round: 1
grader: w3-gate-grader
tier: opus
wave: 3
units: [1, 2, 3, 4, 5, 6]
contract: mochiko-cli rules validation-primitive-edit (preamble + six sections, binary 0.4.0, plugin 0.123.0) — floor pin 11, read back before the first step
---

## Pre-pass — unit 1 (run first-hand by the grader)

- Contract read-back before the first step: `- class: floor · 11 rules`; floors author-grader · plain-seat-explicit-tier · rendered-contract-only · from-file-floor · pre-pass-first-hand · binary-verdict · default-fail · tamper-proof-clause · gate-loop-bound · evidence-floor · second-fail-user.
- `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` (binary `mochiko-cli 0.4.0 · grammar 1..2`): `pointer resolution: 91 checked against plugins/mochiko` · `similar-rule clusters (threshold 0.60) … none — no pair clears the threshold` · `rules scanned: 1294 · in-kind pairs scored: 228700 · clusters: 0 (none)` · `allowlist-suppressed edges: 183` · `mochiko-cli migrate validate · 0 rejecting · 121 advisory`.
- `mochiko-cli migrate status --plugin-root plugins/mochiko`: `sequences 1..107 (95 migrations)` · `state sha256:bfa93263769a…d792` · `94 documents · 1294 rules`. Control (scratch copy of `plugins/mochiko`, `0107` removed): `sequences 1..106 (94 migrations)` · `state sha256:1bc8198c676d…0258` · `1288 rules` · `0 rejecting · 121 advisory`.
- Control vs live advisory diff (both copies outside the repo): `enforces-coverage · command/architecture` 23 → 24 (+ `arch.fail.skip-unstated`) · `command/feature` 16 → 17 (+ `feat.fail.skip-unstated`) · `budget · command/architecture · 66 rules · 18156` → `69 rules · 20567` · `budget · command/feature · 61 rules · 14503` → `64 rules · 16569`. No new pointer, cite, label or condition-coverage finding.
- Raw empty-allowlist clusters: control `clusters: 76 (… CROSS-PAIR 41 …)` · `allowlist: none (182 edges unsuppressed)`; live `clusters: 77 (… CROSS-PAIR 42 …)` · `(183 edges unsuppressed)`. Gone: `[50] CROSS-PAIR · kind: reservation · best 0.62` `feat.user-reserved` ↔ `setup.user-card-rulings`. New: `[25] CROSS-PAIR · kind: fail · best 0.99` arch/feat `fail.skip-unstated`; `[36] CROSS-PAIR · kind: duty · best 0.71` arch/feat `lowering-trail`. `spec.fail.skip-unstated` and `spec.lowering-trail` appear in no live cluster, as both new allowlist reasons claim.
- `mochiko-cli views emit --plugin-root plugins/mochiko --out <scratch>` then `diff -r` against `.mochiko/schema-views`: identical (derived view ≡ replay).
- Pins, live vs control renders: feature `kind: fail · 2 → 3`, `class: floor · 16 → 17`; architecture `2 → 3`, `25 → 26`. Section end-lines: feature 15 + 2 + 19 + 12 + 13 + 3 = 64; architecture 17 + 3 + 19 + 15 + 12 + 3 = 69.
- Char budget: no budgeted class touched. `0107` writes only `command/feature` and `command/architecture`, and `.mochiko/memory/primitive-cost-budgets.md` reads "Command schema data files stay unbudgeted with their commands". The rule-text figures above are advisory only.
- Visit-home conformance (lead's ask). Filled bodies were built from `mochiko-cli template feature-visit` / `architecture-visit`, each carrying `**Lowered**: <step> · stakes: <class> — "<cited line>" · covered by: …` beneath `**Handed in**:`. Each was dry-run with `mochiko-cli check --plugin-root plugins/mochiko --path .mochiko/{features,architecture}/desk/2026-10-28-…/visit.md --content -`, and both returned `"permissionDecision":"allow"`, exit 0. Negative controls returned `deny`, exit 4: a body at `…/notes.md` (`` `notes.md` is not a declared deliverable``) and a 426-line architecture body (`whole-file bound of 300`). The check was live. Nothing was written under either home.
- Routing disclosure: no `Explore` subagent was dispatched. The locate reads were a few targeted greps and awk extracts of named ids, done inline. Every other read drove the decision.

## Unit 1 — schema content: `0107-desk-planning-stakes.yaml` + view diff + allowlist rows

```
VALIDATE: unit 1 — plugins/mochiko/migrations/0107-desk-planning-stakes.yaml + git diff of .mochiko/schema-views/commands/{feature,architecture}.yaml + git diff of scripts/similar-rules-allowlist.yaml (schema content) · seat w3-gate-grader
Checklist run: AM-2 five — intent stated · anchor present where required · ID lifecycle right · floor and fail survival · register; lead's asks (a) scope, (b) greenfield carve, (c) W2 register lesson
Evidence read: 0107 whole; both view diffs whole; allowlist diff; record.md D1-planning-quick-read (statement, items 1–11, revert route), D2, D3, DF1–DF6, OQ2; wave3-desks.md whole; rendered patterns-sound-loop planning-quick-read, trail-of-lowerings, stakes-key, raise-free-lower-ruled; spec.stress-test-quick-read, spec.lowering-trail, spec.fail.skip-unstated (W2 precedent); arch.feature-delta-visit, arch.first-visit-stage, arch.seat-tech-lead-grader, arch.grader-checks-joined-ids, arch.focus-unit, arch.visit-file, arch.focus-context, arch.sound-loop-floor, arch.dm-converge-goal; feat.card-unit, feat.delta-cards, feat.grader-checks-joined-ids, feat.visit-file, feat.card-stage; architecture vars block; both visit templates; architecture render, all seven blocks
Pre-pass: migrate validate 0 rejecting · 121 advisory; views emit ≡ .mochiko/schema-views; status 1..107 · 1294 rules
PASS intent stated — the header names every op and its source (D1 statement and item 11, D2, D3, DF4–DF6). Its counts (feature 61→64, floors 16→17, fails 2→3; architecture 66→69, 25→26, 2→3) match the pins, the end-lines and validate's budget lines. It declares arch.seat-tech-lead-grader and arch.dm-converge-goal untouched, and the view diff agrees.
PASS anchor present — header `2026-10-28 stakes-non-product-surfaces`; the six mints carry `… D1`; rewords keep prior anchors (feat.card-writer `2026-10-15 engine-follow-up`, arch.rows-from-visit-file `2026-10-11 … D3`, arch.focus-blind-list `… D4`); no floor exits, so no exit anchor owed.
PASS ID lifecycle — six new slugs; six rewords keep id, class, kind, labels and anchor; no tombstone; mint-once and cite checks clean per validate.
PASS floor and fail survival — no floor retired or lowered; both prior fails per command stand; the new fails carry `enforces:` to live local rules (lowering-trail, the quick read, the blind list).
FAIL register — arch.rows-from-visit-file's reword replaced the schema's seat vars with literals: `${producer_seat}` → "principal-architect" and `${grader_seat}` → "tech-lead" (view diff lines 187–189). The new arch.store-grade-quick-read also writes "tech-lead" twice as a literal. The render is identical today (vars resolve to the same words), but the architecture schema's rule texts go from 15 `${…}` tokens to 13. A later `set-var grader_seat` or `producer_seat` would silently miss these two rules. The intent does not disclose the change: its "rewords keep … ids, classes, kinds, labels, pointers and anchors" names no var change. The rest of the register holds; see (c) below.
VERDICT: FAIL
Issues requiring fix:
- register / arch.rows-from-visit-file: two var bindings are dropped in the reword. Fix: in 0107's reword text write "the ${producer_seat} seat authors" and "${grader_seat} grades them", as the prior text had.
- register / arch.store-grade-quick-read: the seat is named as a literal. Fix: write "${grader_seat}'s grade of a ruling visit's store writes" and "${grader_seat}, a seat that authored none of them". Then re-hash 0107, re-emit views, re-run validate. Leave the `focus` moment's literal "principal-architect": it was literal before 0107.
```

Lead's asks, ruled:
- (a) Scope narrowing is faithful to the card. D1 says the lever reaches "the architecture desk's store writes — and nothing else" and that a store write inside implement's design phase "follows implement's own run stakes". arch.feature-delta-visit has "the desk writes nothing to the store for it". The wave plan names no visit kind (§3 "as feature's, read at the visit goal line"), and the card wins. The keying holds the narrowing: the stakes clause in arch.focus-blind-list says "on a ruling visit"; the quick read, the trail and the fail are `when: visit_kind: ruling`; the blind-list rule's `when:` stays `[ruling, delta]` with no stakes term, so a delta visit's list never goes inapplicable for want of a stakes line.
- (b) The greenfield carve is register-faithful and a free raise. DF4's skip is a latitude ("may be skipped"). The carve withholds it only where taking it would break arch.first-visit-stage ("greenfield, the shelf rows and the blind list supply its candidates"), and it leaves that anchored rule whole. Narrowing toward more rigor needs no ruling ("Raising rigor is always free", raise-free-lower-ruled). "That stakes-line skip" scopes to the second disjunct only. The single-row-ask clause is byte-identical to before. Brownfield first visits are rightly unaffected: their fact base is the derivation, not the list. The lead's disclosure to the user is the right route.
- (c) The W2 lesson holds in every new and reworded text. feat.card-writer and arch.rows-from-visit-file say "only where the user ruled that on the stakes line, the full grade running otherwise". Both quick reads say "Where the user ruled it on the stakes line". feat.card-blind-list says "where the user ruled that on the stakes line". arch.focus-blind-list says "where the user ruled the skip on the stakes line". Both condition notes say "proposed by you … ruled by the user". The reserved rules say "only their checkpoint ruling lowers it later" and "with any later lowering of it". No lowering keys on a low label alone.

Advisory, not blocking: (1) The allowlist row `[feat.user-reserved, setup.user-card-rulings]` (line 242) now suppresses no edge. Rows outnumber edges (246 vs 183), so a stale row is library-normal, but it can be pruned. (2) The feature condition note's "or lane" goes beyond D2's "capabilities … it touches". It is needed for `lane-<slug>` card visits, which touch no capability, and stays a harm read of the product work steered. (3) Both new allowlist reasons hold: a `kind: fail` stays the caller's (analysis-iterative.caller-slots, live), and the trail pair is 2-member, below the 3+ bar.

## Unit 1 — round 2 re-audit (same seat, resumed; fix scope only)

```
VALIDATE: unit 1, round 2 — plugins/mochiko/migrations/0107-desk-planning-stakes.yaml (the arch.rows-from-visit-file reword, arch.store-grade-quick-read, arch.fail.skip-unstated) + git diff of .mochiko/schema-views/commands/architecture.yaml + the two new allowlist rows (schema content) · seat w3-gate-grader
Checklist run: the round-1 register FAIL, and what the fix could break — ID lifecycle, floor and fail survival, view ≡ replay, cluster set, allowlist reasons, feature view
Evidence read: 0107 lines 25, 155, 165–250 (grep of every seat token); architecture view, round-1 emit vs round-2 emit (diff), and vs HEAD; feature view, round-1 emit vs round-2 emit; architecture render, all seven blocks; allowlist rows 640–646; raw empty-allowlist cluster listing, round 1 vs round 2
Pre-pass: `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` — `rules scanned: 1294 · in-kind pairs scored: 228700 · clusters: 0 (none)` · `allowlist-suppressed edges: 183` · `0 rejecting · 121 advisory`; `budget · command/architecture · - · 69 rules · 20586` (was 20567); feature `64 rules · 16569` unchanged; every other line identical to round 1. `migrate status`: `sequences 1..107 (95 migrations)` · `state sha256:c24258e3d088…4f95` · `1294 rules`. 0107 `hash: sha256:b047e9b7…357f` (validate accepts it). `views emit` to scratch ≡ `.mochiko/schema-views`. `git diff --numstat`: architecture view 67/6, feature 61/6.
PASS register — arch.rows-from-visit-file reads "the ${producer_seat} seat authors" and "${grader_seat} grades them". Its HEAD-vs-live view diff no longer touches those lines; only the stakes clause and `when:` change. arch.store-grade-quick-read reads "${grader_seat}'s grade of a ruling visit's store writes" and "${grader_seat}, a seat that authored none of them". Round-1 vs round-2 view diff: only these var swaps and their reflow, no other word moved. Architecture `${…}` tokens 13 → 18 (HEAD 15 + 3 new); no `-` line in the HEAD diff carries a `${`. Render carries 0 unresolved `${`.
PASS arch.fail.skip-unstated (P1's sweep, beyond my list) — "whose ${producer_seat} plan grade" renders "whose principal-architect plan grade thinned under `review-seat-plan.once-per-seat-in-band`". Same seat as round 1's "architect plan grade", now var-bound; class floor, kind fail, `enforces:` and anchor unchanged.
PASS nothing broken — feature view byte-identical to round 1 (no added feature line names an architect seat); `focus` moment literal, cited id arch.seat-tech-lead-grader and allowlist row 242 untouched; raw clusters still `77 (… CROSS-PAIR 42 …)` · `183 edges unsuppressed`, member set identical to round 1. Only the score moved: `[25] CROSS-PAIR · kind: fail · best 0.99` → `best 1.00`; trail pair `[36] … best 0.71` unchanged. Neither new allowlist reason quotes a score (the one decimal in rows 642–646 is `v0.124.0` in the comment), so nothing is stale.
PASS intent, anchor, ID lifecycle, floor and fail survival — unchanged from round 1; pins and counts the same (validate 69 / 64 rules, 1294 total).
VERDICT: PASS
Issues requiring fix: none
```

Round-1 advisories stand, non-blocking: allowlist row 242 is stale, and the feature condition note's "or lane" goes beyond D2's wording but is needed for lane cards.

## Pre-pass — units 2–4 (run first-hand by the grader)

- `mochiko-cli migrate validate --report --plugin-root plugins/mochiko`: `pointer resolution: 91 checked against plugins/mochiko` · `rules scanned: 1294 · in-kind pairs scored: 228700 · clusters: 0 (none)` · `allowlist-suppressed edges: 183` · `0 rejecting · 121 advisory`. Byte-identical to the unit-1 round-2 report. `migrate status`: `state sha256:c24258e3d088…4f95 · 94 documents · 1294 rules`.
- `git diff --quiet HEAD -- plugins/mochiko/commands/feature.md plugins/mochiko/commands/architecture.md`: both unchanged against `07123f6`. The router diff is 1 line in, 1 line out. The only edit is an insertion at line 142 after "a blind list for what the shelf lacks": `" (skippable, stated, where you rule the skip on a ruling visit's low stakes line, never on a greenfield first visit)"`.
- Renders, every section the `.md` names, its seven `!` lines: preamble · roles · reserved · tools · ways-of-working · boundaries · fail-conditions. That matches the preamble's six `sec.` ids set-wise. Characters of stdout: feature **27,734**, architecture **33,286**, both matching the lead's figures. End-lines: feature 15 · 2 · 19 · 12 · 13 · 3 = 64; architecture 17 · 3 · 19 · 15 · 12 · 3 = 69. Pins: feature `kind: fail · 3 rules` · `class: floor · 17 rules`; architecture `kind: fail · 3 rules` · `class: floor · 26 rules`.
- Char budget: commands are unbudgeted ("Command schema data files stay unbudgeted with their commands"), and so is the router. The canonical snippet in `primitive-cost-budgets.md` (body after the fence; description parsed) gives router body **50,483 → 50,599** (+116, the insertion exactly) and description **206 → 206**, frontmatter unchanged.
- `mochiko-cli check --path .mochiko/strips/mochiko.md --content -` on the strip file as it stands: `allow`, exit 0.

## Unit 2 — command pair: `feature.md` + render

```
VALIDATE: unit 2 — plugins/mochiko/commands/feature.md (unchanged) + mochiko-cli rules feature, all seven blocks (command pair) · seat w3-gate-grader
Checklist run: judgment-items-pair on the canonical-scaffold criteria — scaffold · preserved responsibilities · floor survival · independence · reserved-to-user · done-condition branch (desk) · argued overage; lead's ask: must the Goal name the stakes line
Evidence read: feature.md whole; the seven rendered blocks (pins, floors line, end-lines; feat.dm-*, feat.user-reserved, feat.ruling-places, feat.card-stage, feat.card-writer, feat.card-grade-quick-read, feat.card-blind-list, feat.lowering-trail, feat.sound-loop-floor, feat.fail.skip-unstated read in full); the stakes condition note and the card-set moment
Pre-pass: migrate validate 0 rejecting · 121 advisory; pins fail 3 · floor 17; end-lines sum 64; render 27,734 chars
PASS scaffold headings and order — frontmatter key set with `allowed-tools: Bash(mochiko-cli *)`, `# Feature — The Product Desk`, Identity & Mission, Rules block (seven `!` lines, halt-on-non-delivery clause), Adaptive Goal Protocol Entry → Goal → Not done; untouched.
PASS preserved responsibilities — the file is unchanged, and every sentence stays true under 0107. Identity "you write no capability truth alone" holds: the stakes line and any lowering are the user's (feat.user-reserved). Goal "state the goal and its done condition, get the nod, and go" and "closes its thinking stage on the goal nod, one confirm (`feat.card-stage`)" hold: the stakes line is "ruled by the user on the visit goal line" (condition note) and rides that same nod, and DF5 says "never a new gate". The Goal names neither the card grade nor the blind list, so no grade-form sentence can go false.
PASS floor survival — floors line 17 = the 16 prior + feat.fail.skip-unstated. Not-done cites "the `kind: fail` line under `pins`" with its halt clause and no hard number, so the third fail needs no prose edit.
PASS independence — feat.card-grade-quick-read: "the same non-author seat"; feat.no-self-graded-writes and feat.author-grader stand.
PASS reserved-to-user content in the reserved section — feat.user-reserved (feat.sec.reserved) carries the stakes line and its later lowering.
PASS done-condition branch (desk, per visit) — "a one-line visit goal and its explicit done condition", "closes with a verdict against it"; no fixed done condition demanded. The sound-loop pointer rides feat.sound-loop-floor.
PASS argued overage — none owed: commands are unbudgeted.
VERDICT: PASS
Issues requiring fix: none
```

Lead's ask, ruled: the Goal need not name the stakes line. specify.md's Goal took its W2 edit because it named the stress-test, whose form 0106 changed, so that sentence would have gone incomplete. feature.md's and architecture.md's Goals name no grade and no blind list, so no sentence goes false. The stakes line rides the existing goal confirmation (DF5, "never a new gate"). The rules carry it in the condition note, the moment and the reserved rule, and restating it in the `.md` would duplicate rendered rules (GI-017).

## Unit 3 — command pair: `architecture.md` + render

```
VALIDATE: unit 3 — plugins/mochiko/commands/architecture.md (unchanged) + mochiko-cli rules architecture, all seven blocks (command pair) · seat w3-gate-grader
Checklist run: judgment-items-pair on the canonical-scaffold criteria — scaffold · preserved responsibilities · floor survival · independence · reserved-to-user · done-condition branch (desk) · argued overage
Evidence read: architecture.md Identity and Adaptive Goal Protocol whole, Rules block lines; the seven rendered blocks (pins, floors line, end-lines; arch.dm-converge-goal, arch.gate-focus-confirm, arch.user-reserved-rulings, arch.rows-from-visit-file, arch.store-grade-quick-read, arch.focus-blind-list, arch.lowering-trail, arch.sound-loop-floor, arch.fail.skip-unstated read in full); the stakes condition note and the focus moment
Pre-pass: migrate validate 0 rejecting · 121 advisory; pins fail 3 · floor 26; end-lines sum 69; render 33,286 chars (fail-conditions now renders "principal-architect plan grade", +10 against the lead's earlier "unchanged" relay — that relay was wrong, the render was not)
PASS scaffold headings and order — `# Architecture …` heading, Identity & Mission, Rules block with seven `!` lines, Entry → Goal → Not done; untouched.
PASS preserved responsibilities — the file is unchanged, and every sentence stays true. Identity "every stance, baseline, and amendment is the user's ruling, taken on a produced-and-graded proposal" holds: the quick read is a grade by a non-author seat (planning-quick-read). Goal "a crisp ask … converges in a single exchange" holds: the stakes line is ruled with the goal (focus moment) and is not a new screen. arch.gate-focus-confirm keeps the set's confirm "never the goal's". "A first-visit baseline walk is a long visit with the same contract" holds: the greenfield carve keeps its blind list. Entry's `<FEAT-ID>` delta visit states no stakes line (condition note), and Entry says nothing about the grade's depth.
PASS floor survival — floors line 26 = the 25 prior + arch.fail.skip-unstated; arch.dm-converge-goal untouched. Not-done cites the printed pin with its halt clause, no number.
PASS independence — arch.store-grade-quick-read: "${grader_seat}, a seat that authored none of them"; arch.author-grader-separation stands.
PASS reserved-to-user content in the reserved section — arch.user-reserved-rulings (arch.sec.reserved) carries "a ruling visit's stakes line, with any later lowering of it".
PASS done-condition branch (desk, per visit) — goal and explicit done condition per visit, closing verdict; the sound-loop pointer rides arch.sound-loop-floor.
PASS argued overage — none owed: commands are unbudgeted.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 4 — prose primitive: router row (`SKILL.md`:142) + strip `[v0.124.0]`

```
VALIDATE: unit 4 — plugins/mochiko/skills/mochiko/SKILL.md (row 142 edited; rows 140–142 read) + .mochiko/strips/mochiko.md (W3 comment + [v0.124.0] entry) (prose primitive) · seat w3-gate-grader
Checklist run: judgment-items-prose — coherence · preserved responsibilities; lead's asks: keyed on the user's ruling, delta sentence does not inherit the skip, row 141 holds
Evidence read: git diff of SKILL.md; character-level diff of line 142 against `git show 07123f6`; rows 140–142 whole; router lines naming stakes, blind list, tech-lead or devils-advocate (70, 117, 140, 142, 160, 161); strip diff whole; grep of plugins/ for the row phrase (router only); repo README.md:133; canonical-snippet measurement
Pre-pass: migrate validate 0 rejecting · 121 advisory (pointer resolution 91); router body 50,483 → 50,599, description 206 unchanged (unbudgeted)
PASS coherence — the qualifier keys on the user's ruling: "where you rule the skip on a ruling visit's low stakes line", the router's "you" being the user. That matches arch.focus-blind-list's "where the user ruled the skip on the stakes line" and the specify row's W2 wording ("where you rule the skip on a low stakes line"). "never on a greenfield first visit" agrees with the row's own greenfield clause, "elicits it … from the shelf rows and a blind list". The delta sentence's "works its focus set the same way" does not inherit the skip: the qualifier names "a ruling visit's" stakes line, a delta visit states none, and the row already sets "A visit that rules something" against "A feature's delta visit". Row 141 (/mochiko:feature) names no card grade, blind list or grade depth, so nothing in it goes false. Lines 160–161 describe the personas' roles, not a grade form, and hold.
PASS preserved responsibilities — a pure insertion: the prior phrase and every other byte of the row stand. The strip entry quotes the qualified phrase verbatim, cites the ruling (D1 statement and item 11, DF4, DF5, D3; `DECISIONS.md` 2026-10-28 row) and names its carrier rule. Kept deliberately and Consumers assessed both hold against my own grep (router only) and README.md:133 ("tech-lead grades" holds). The pre-edit ref `07123f6` is HEAD, and the body figures in the comment reproduce exactly.
VERDICT: PASS
Issues requiring fix: none
```

Advisory, not blocking: the router row names only the stakes-line skip. The older single-row-ask skip was never in the row, and "skippable … where" does not claim to be the only route, so nothing is false. The `/mochiko:feature` row likewise never named its card grade, so the quick read's absence there matches its altitude.

## Unit 5 — crate code, non-author review: `fidelity.rs` · `matrix_similar.rs` · `validate.rs`

```
VALIDATE: unit 5 — git diff -- crates (crates/mochiko-cli/tests/{fidelity,matrix_similar,validate}.rs) (crate code, non-author review per .claude/rules/mochiko/rust-cli.md) · seat w3-gate-grader
Checklist run: rust-cli.md quality gate (test · fmt · clippy · audit · opt-in full similarity sweep); every pinned figure against the unit-1 pre-pass; per-kind arithmetic re-derived independently; ruling citation on each new comment; bright line (tests only)
Evidence read: the three-file diff whole, with the W1/W2 comment paragraphs it chains from (fidelity.rs 925–945, matrix_similar.rs 1105–1124 and 1385–1406, validate.rs 1240–1258); precedent `git show 07123f6 -- crates` shape; kind counts parsed from the live views and from a pre-0107 emit
Pre-pass: `cargo test --all --no-fail-fast` exit 0 — every binary `test result: ok`, 765 passed · 0 failed (fidelity 17 · matrix_similar 48 · validate 105 …). `cargo fmt --all --check` exit 0. `cargo clippy --all-targets -- -D warnings` exit 0 (tests re-checked after a touch). `MOCHIKO_FULL_SIMILAR=1 cargo test --release --test matrix_similar the_detector_reproduces_the_live_runs_figures_over_the_corpus` → `test … ok` · `1 passed; 0 failed`. `cargo audit --deny warnings` exit 0 (31 crate dependencies). `git diff --quiet HEAD -- crates/mochiko-cli/src crates/mochiko-cli/Cargo.toml Cargo.lock`: unchanged.
PASS figures against the unit-1 pre-pass — census command 435 · total 1294 · command floors 133 · fails 44 · skill rules 859 · skill floors 271 (floor pins summed across the six commands = 133, cross-checked in unit 6; the fail figure 44 is asserted by the passing census tests). Sequence list ends `…, 105, 106, 107` (status `1..107`). Corpus `1294 / 228_700 / 0 / 183` = validate's `rules scanned: 1294 · in-kind pairs scored: 228700 · clusters: 0 · allowlist-suppressed edges: 183`. Command family `(435, 22_106, 0, 56)` is asserted by the passing test.
PASS per-kind arithmetic — my own count from the views: commands constraint 181 → 183, duty 48 → 50, fail 42 → 44 (the comment's figures); corpus constraint 601 → 603, duty 174 → 176, fail 42 → 44. Pair deltas from the same counts: commands 21,661 → 22,206 = **+545**, corpus 227,337 → 228,974 = **+1,637**, both matching the comments. My absolute pair totals run 100 / 274 higher than the detector's, because it scores fewer pairs than all in-kind pairs; the deltas are what the comments claim. The two quick reads are the kind-less constraints, matching 0107.
PASS suppressed and edge narrative — +2 −1 = 56 (family) and 183 (corpus). The pairs named: fail 1.00 (round 2's `best 1.00`), trail 0.71, and `feat.user-reserved` / `setup.user-card-rulings` under the threshold. Matches my raw 76/182 → 77/183 runs.
PASS ruling citation — each new comment chains to the session named in the paragraph directly above it, the form the W2 comments used and that passed W2's gate. fidelity.rs's sequence comment names "the 2026-10-28 stakes-non-product-surfaces waves 1–3". Its census comment says "Its wave 3 carries `0107`, its mints anchored to the same session's D1". matrix_similar.rs says "Re-measured after its wave 3 (`0107`)", under "the 2026-10-28 stakes-non-product-surfaces wave 2". validate.rs says "`0107` (its wave 3)", under "`0105` (the 2026-10-28 stakes-non-product-surfaces wave 1)". The appended clause in validate.rs's floor-history comment and the run-message tail ("then its wave 3 — 0107's desk planning stakes") follow the same chain.
PASS bright line — tests and comments only; no `src/`, manifest or lockfile change.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 6 — contract freeze: `run.py` `EXPECTED` + `README.md`:274

```
VALIDATE: unit 6 — git diff -- evals (evals/contract/run.py EXPECTED["feature"], EXPECTED["architecture"]; evals/contract/README.md:274); evals/contract/expected-skills.json unchanged (contract freeze) · seat w3-gate-grader
Checklist run: each frozen command set against its live `floors:` line; the 36 skill rows against their renders; provenance comments; README cell; baselines untouched
Evidence read: the evals diff whole; EXPECTED parsed via `ast` (no import, no execution) for all six commands; `expected-skills.json` `skills` map, all 36 rows; `mochiko-cli rules <primitive> --section preamble` for 6 commands + 36 skills
Pre-pass: `git diff --quiet HEAD -- evals/contract/expected-skills.json` unchanged; run.py parses (`ast.parse`)
PASS command freeze — every frozen set equals its live `floors:` line set-wise: architecture 26 · brainstorm 9 · feature 17 · implement 40 · setup 21 · specify 20, total **133** = the frozen total, each matching its `- class: floor · N rules` pin. The two additions are `arch.fail.skip-unstated` and `feat.fail.skip-unstated`, each with a provenance comment naming `0107-desk-planning-stakes` and "2026-10-28 stakes-non-product-surfaces D1", in the shape of its neighbours. The byte baselines (23,026 · 21,020) are untouched, as freeze baselines.
PASS skill freeze — all 36 rows: `floor_ids` = the live `floors:` line and `floor_pin` = its count, 0 mismatching, 271 in total (README skills cell 271).
PASS README:274 — commands cell 131 → 133. The closing clause reads "`spec.fail.skip-unstated`, `feat.fail.skip-unstated` and `arch.fail.skip-unstated` by the 2026-10-28 stakes-non-product-surfaces ruling"; every other byte of the row is unchanged.
VERDICT: PASS
Issues requiring fix: none
```

Not run here: the plugin contract suite (`python3 evals/contract/run.py`, Docker sandbox). It is gate 6 of the `plugin.json` bump, not this unit. Per the contract-suite prereqs it runs from the `~/sbx/mochiko` copy, and a SKIPPED suite is not green.

## Failure narrative

Round 1 (fixed in round 2, above). Unit 1 failed on one register item. The reword of arch.rows-from-visit-file and the new arch.store-grade-quick-read name the architect and grader seats as literal words, where every other architecture rule uses the schema's `${producer_seat}` / `${grader_seat}` vars. The prior text of arch.rows-from-visit-file carried both vars, so the reword drops a binding the intent does not disclose. The likely cause is new text copied from the render, which resolves vars. Today's render is unaffected; the defect is latent and costs a two-line fix in 0107 plus re-hash and re-emit. Every other item passes, and so do the lead's three asks. Re-audit scope: the two rule texts, the 0107 hash, the regenerated architecture view, and validate.
