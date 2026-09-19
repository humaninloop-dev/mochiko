---
report: review
round: 2
seat: gate-grader-w2
tier: opus
wave: author-grader-consolidation wave 2 (producer-plan-enforcement D8)
units: 14 of 14
verdict: 14 PASS · 0 blocking (unit 1 FAILed at round 1, fixed and cleared at round 2)
---

## Failure narrative

One unit fails: the `patterns-sound-loop` pair is over its measured char budget with no argued
overage. Migration `0009-plan-qa-leg.yaml` rewords two of its rules, adding 430 characters of
rule text (`leg-1-seat-produces` +177, `disclosure-line` +253). The delivered-at-invoke payload
measures 12,581 (body 2,812 + render 9,769) against the ledger's 12,388, a standing overage of
+193. Nothing in `.mochiko/memory/primitive-cost-budgets.md` records it, and the audit brief
named no overage, so the skill-pair criterion 8 escape hatch is not open. The arithmetic is
self-consistent: the recorded render figure 9,576 was seeded before the v0.107.0 render-format
change, and 9,576 − 237 + 430 = 9,769 exactly, where −237 is the format constant every untouched
control skill in this run reproduced. The growth is a genuine new obligation from a user-ruled
floor reword, so the fix is a recorded standing-overage row, not a content cut. Everything else
in the wave is clean, including the sibling `patterns-plan-minimalism`, which grew 97 characters
and still sits 128 under its budget.

## Floor read-back

The preamble's `class: floor` pin prints **11** rules. Ids: `author-grader` ·
`plain-seat-explicit-tier` · `rendered-contract-only` · `from-file-floor` · `pre-pass-first-hand` ·
`binary-verdict` · `default-fail` · `tamper-proof-clause` · `gate-loop-bound` · `evidence-floor` ·
`second-fail-user`.

Seat: one plain persona-less `general-purpose` seat, `model: opus` explicit, authored nothing in
any unit, taking every unit of the wave. No split was needed — every unit's files fit this
context.

## Pre-pass — run first-hand, quoted

All four commands were executed by this seat, from the repo root, before any unit was graded.

`mochiko-cli migrate validate --report --plugin-root plugins/mochiko`:

```
pointer resolution: 84 checked against plugins/mochiko
=== similar-rule clusters (threshold 0.60) ===
none — no pair clears the threshold
rules scanned: 1082 · in-kind pairs scored: 161376 · clusters: 0 (none)
allowlist-suppressed edges: 170
mochiko-cli migrate validate · 0 rejecting · 106 advisory
```

## Pre-pass — status, views, budgets

`mochiko-cli migrate status --plugin-root plugins/mochiko`:

```
log plugins/mochiko/migrations · grammar 1 · sequences 1..9 (9 migrations)
state sha256:d5cb369aef8f31d3b82e9f5a8bd7498e671cfc7d7156eda6e3514083ca7a81fd · 75 documents · 1082 rules
```

Views ≡ replay: `mochiko-cli views emit --plugin-root plugins/mochiko --out .mochiko/schema-views`
reported `75 documents`, and the `git diff HEAD -- .mochiko/schema-views` sha256 was
`6537d240c4747f718450e0048cec694abcef93652b6e7c04c201378a53d253cc` both before and after the
emit. No new change; the six modified view files and the one new file are exactly the replay.

## Pre-pass — char-budget measurement

Measured with the canonical python3 snippet from `.mochiko/memory/primitive-cost-budgets.md`,
characters of the parsed value, payload = body + the seven rendered blocks the `!` lines deliver.

| primitive | payload measured | budget | standing |
|---|---|---|---|
| patterns-sound-loop | 12,581 (body 2,812 + render 9,769) | 12,388 | **+193 over** |
| patterns-plan-minimalism | 10,696 (body 3,846 + render 6,850) | 10,824 | 128 under |
| review-seat-plan | 12,701 (body 3,534 + render 9,167) | 12,701 (birth seed) | at seed |

## Pre-pass — measurement calibration

Four untouched review-family skills were measured as controls and each landed exactly 237 under
its recorded budget — `review-brainstorm` 12,587/12,824,
`review-code-minimalism` 10,428/10,664, `review-sufficiency` 15,186/15,423,
`validation-constitution` 14,780/15,017. The constant is the v0.107.0 render-format change the
ledger documents, so the measurement basis matches the ledger's.

## Unit 1 — skill pair `patterns-sound-loop`

VALIDATE: skill pair `patterns-sound-loop` — `SKILL.md` plus its seven rendered blocks.
Checklist run: `.claude/rules/mochiko/primitive-edits.md` skill-pair items 1–12; mechanical limbs read from the pre-pass, judgment limbs graded here.
Evidence read: `plugins/mochiko/skills/patterns-sound-loop/SKILL.md` · `mochiko-cli rules patterns-sound-loop --section {preamble, .sec.trigger, .sec.scope, .sec.discipline, .sec.inputs, .sec.disclosure, .sec.reserved} --plugin-root plugins/mochiko` · `.mochiko/schema-views/skills/patterns-sound-loop.yaml` against `git show HEAD:` of the same path.
Pre-pass: `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` → `0 rejecting · 106 advisory`; the only finding naming this document is `budget · skill/patterns-sound-loop · - · 14 rules · 4490 resolved characters of rule text`, advisory and rule-text-only, never the ledger comparison.

## Unit 1 — item grades

- PASS — scaffold and load-first: `## Rules — delivered by mochiko-cli` present, seven `!` lines, `allowed-tools: Bash(mochiko-cli *)`, no-raw-Read clause, read-back sentence citing both the pin and the `floors:` line.
- PASS — section enumeration set-wise: the `!` lines name `trigger · scope · discipline · inputs · disclosure · reserved`; the preamble prints the same six in that order. Patterns-family set, `inputs` carrying its explicit empty marker.
- PASS — floor survival and pin: pin prints `class: floor · 6 rules`, unchanged as D8 item 1 required; all six ids survive, `leg-1-seat-produces` reworded with its id kept and its `2026-08-13 charter-ritual-balance` anchor intact.
- PASS — preserved responsibilities: the reword replaces only the approval clause; "production sits with a seat, never the lead", the additive-to-leg-2 sentence and the whole `floor: clear` fallback survive. The disclosure reword appends the `plans:` segment and keeps the bare-clear rule.
- PASS — independence: no rule seats a grader on its own output; `leg-2-non-author-review` stands unchanged as the non-author leg.
- PASS — reserved section: `leg-3-user-gate-stays` holds the user gate, `class: floor`, `kind: reservation`.
- **FAIL — argued overage:** payload 12,581 against budget 12,388, +193 over, with no overage named in the brief and no ledger row.
- n/a — done-condition branch: a skill carries no Adaptive Goal Protocol (skill-pair block, no Not-done count-pin).

VERDICT: FAIL

## Unit 1 — issues requiring fix

Issues requiring fix:
1. **Item 8, budget = delivered-at-invoke payload.** Missing: any record of the +193 standing
   overage on `patterns-sound-loop`. Fix: add a standing-overage note to that primitive's row in
   `.mochiko/memory/primitive-cost-budgets.md`, in the shape the `patterns-model-tiering`
   `[v0.111.0]` row already uses — payload 12,581 = body 2,812 + render 9,769, measured with the
   canonical snippet at log state `sha256:d5cb369a…` · 75 documents · 1,082 rules · floor pin 6,
   binary 0.2.0. The whole delta is render, attributed to `0009-plan-qa-leg.yaml` rewording two
   rules under anchor `2026-09-03 producer-plan-enforcement D8` (`leg-1-seat-produces` +177,
   `disclosure-line` +253 = +430, less the v0.107.0 render-format constant −237, net +193); ruled
   a genuine new obligation, never restored prose, since both rewords are user-ruled landing-set
   items D8 1 and D8 4. No content cut is warranted.

Round-1 outcome, superseded by the round-2 line below: `audit: patterns-sound-loop pair · gate-grader-w2 · opus · 3 files · 1 rounds · 1 blocking`

## Unit 1 — round 2, delta re-audit

VALIDATE: skill pair `patterns-sound-loop`, round 2 — same grader seat resumed, reading only what the fix touched and what it could have broken.
Checklist run: skill-pair item 8 (the failed item), plus the coherence and preserved-responsibilities re-check unit 11 owes because the fix edits that unit's file.
Evidence read: `git diff HEAD -- .mochiko/memory/primitive-cost-budgets.md` (12 insertions, 1 deletion — the whole change) · the rewritten `patterns-sound-loop` row read in full · the `patterns-model-tiering` `[v0.111.0]` row re-read as the shape comparator.
Pre-pass: re-run first-hand. `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` → `0 rejecting · 106 advisory`, `rules scanned: 1082 · clusters: 0 (none)`. `mochiko-cli migrate status --plugin-root plugins/mochiko` → `sha256:d5cb369a… · 75 documents · 1082 rules`, unchanged. Views diff sha256 `6537d240…`, unchanged. Char budget re-measured with the canonical snippet: payload **12,581** = body 2,812 + render 9,769, description 505.

## Unit 1 — round 2, item grades

- PASS — item 8, argued overage: the row now carries a standing **+193 ruled-HOLDS overage at [v0.112.0]**. Every figure it asserts reproduces against my own measurement — payload 12,581, body 2,812, render 9,769, description 505, floor pin 6.
- PASS — the components sum exactly: `leg-1-seat-produces` +177 and `disclosure-line` +253 against the v0.107.0 render-format constant −237 gives +193. I reached the same three numbers independently in round 1 from the view diff, by a different route than the row's log-replay method, and they agree.
- PASS — the overage argument holds: both rewords are user-ruled landing-set items (D8 items 1 and 4) under anchor `2026-09-03 producer-plan-enforcement D8`, so this is a genuine new obligation. No restored playbook prose is involved, and no content cut is warranted.
- PASS — nothing the fix could have broken did: the budget column stays `12,388 (no headroom)`, as a ruled-HOLDS overage requires; the prior row's text survives verbatim as the row's history tail; the floor pin, the ids, the body and the description are all unmoved; and the run-wide pre-pass, the log state and the views diff are byte-identical to round 1.
- PASS — unit 11 re-check: the edit is one row replacement plus the round-1 additions, and no other row, figure, note or seeding path in the ledger moved. Unit 11's round-1 PASS stands unchanged.

VERDICT: PASS

Issues requiring fix: none.

outcome: `audit: patterns-sound-loop pair · gate-grader-w2 · opus · 4 files · 2 rounds · 0 blocking`

## Unit 2 — skill pair `patterns-plan-minimalism`

VALIDATE: skill pair `patterns-plan-minimalism` — `SKILL.md` plus its seven rendered blocks.
Checklist run: skill-pair items 1–12, judgment limbs.
Evidence read: `plugins/mochiko/skills/patterns-plan-minimalism/SKILL.md` · `mochiko-cli rules patterns-plan-minimalism --section {preamble + the six} --plugin-root plugins/mochiko` · `.mochiko/schema-views/skills/patterns-plan-minimalism.yaml` against `git show HEAD:`.
Pre-pass: `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` → `0 rejecting · 106 advisory`; `budget · skill/patterns-plan-minimalism · - · 10 rules · 2309 resolved characters of rule text`, advisory.

## Unit 2 — item grades

- PASS — scaffold and load-first: seven `!` lines, the grant, the read-back sentence, the Ladder and Sibling body sections intact.
- PASS — section enumeration set-wise: patterns six-set, `reserved` carrying its explicit empty marker.
- PASS — floor survival: pin `class: floor · 2 rules`, `rung-1-never-deletes` and `floor-both-ways` both untouched. The reworded `grading-routing` is `class: must`, so no anchor is owed.
- PASS — preserved responsibilities: the reword adds the `review-seat-plan` presence limb and keeps the `review-plan-artifacts` honesty-advisory and gap-list-blocking clauses verbatim in substance.
- PASS — independence: the rule splits grading across two non-author skills and seats no self-grade.
- PASS — reserved section: deliberately empty with its census-cited note, unchanged.
- PASS — argued overage: none needed. Payload 10,696 against budget 10,824, 128 under.

VERDICT: PASS

outcome: `audit: patterns-plan-minimalism pair · gate-grader-w2 · opus · 3 files · 1 rounds · 0 blocking`

## Unit 3 — command pair `architecture`

VALIDATE: command pair `architecture` — `plugins/mochiko/commands/architecture.md` plus its seven rendered blocks.
Checklist run: canonical-scaffold criteria 1–11, judgment limbs; mechanical limbs from the pre-pass.
Evidence read: `plugins/mochiko/commands/architecture.md` · `mochiko-cli rules architecture --section {preamble, arch.sec.roles, .reserved, .tools, .ways-of-working, .boundaries, .fail-conditions} --plugin-root plugins/mochiko` · `.mochiko/schema-views/commands/architecture.yaml` against `git show HEAD:`.
Pre-pass: `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` → `0 rejecting · 106 advisory`; for this document only `zero-member-label · command/architecture · attempt-economy · registered but carried by no rule in this document — registry-legal; watch at rollout`, advisory and pre-existing.

## Unit 3 — item grades

- PASS — scaffold headings and order: frontmatter with all four required keys, `# Architecture — The Product Architecture Desk`, `## Identity & Mission`, `## Rules — delivered by mochiko-cli`, `## Adaptive Goal Protocol` with Entry, Goal, Not done last. No `**Goal:**` opener, no extra top-level section.
- PASS — preserved responsibilities: all three rewords extend the existing clause rather than replace it. `seat-architect-producer` keeps "Recommends with reasons; never rules"; `author-grader-separation` keeps the full produces-list and the no-self-clearing clause; `sound-loop-floor` keeps the user's-ruling-never-substitutes clause, the transcription carve and the referenced-never-restated tail.
- PASS — floor survival: pin `class: floor · 23 rules`; `author-grader-separation` and `sound-loop-floor` both survive as floors with their ids, and `author-grader-separation` gains the `2026-09-03 producer-plan-enforcement D8` anchor it previously lacked.
- PASS — independence, no seat grading its own row: `arch.seat-tech-lead-grader` remains the separate grading seat; the reworded producer rule names a fresh peer, never the author.
- PASS — reserved-to-user content: `arch.sec.reserved` carries `arch.user-reserved-rulings`; nothing reserved leaked into another section.
- PASS — done-condition branch (desk): step 2 converges per visit to a one-line goal and its explicit done condition and closes with a verdict against it. No fixed done condition is demanded here.
- PASS — argued overage: commands carry no budgeted class (skill body, skill description, agent description only).

VERDICT: PASS

outcome: `audit: architecture command pair · gate-grader-w2 · opus · 3 files · 1 rounds · 0 blocking`

## Unit 4 — command pair `feature`

VALIDATE: command pair `feature` — `plugins/mochiko/commands/feature.md` plus its seven rendered blocks.
Checklist run: canonical-scaffold criteria 1–11, judgment limbs.
Evidence read: `plugins/mochiko/commands/feature.md` · `mochiko-cli rules feature --section {preamble + the six} --plugin-root plugins/mochiko` · `.mochiko/schema-views/commands/feature.yaml` against `git show HEAD:`.
Pre-pass: `0 rejecting · 106 advisory`; for this document only `zero-member-label · command/feature · attempt-economy`, advisory and pre-existing.

## Unit 4 — item grades

- PASS — scaffold headings and order: canonical set in canonical order, `$ARGUMENTS` handled in Entry, Not done last and citing the printed pin rather than a hard-coded count.
- PASS — preserved responsibilities: `feat.author-grader` keeps its three-artifact produces-list and the no-self-clearing clause; `feat.sound-loop-floor` keeps the bug-and-improvement-alike delta-card clause and the referenced-never-restated tail.
- PASS — floor survival: pin `class: floor · 14 rules`; `feat.sound-loop-floor` survives as a floor with its id and its `2026-08-13 charter-ritual-balance` anchor. `feat.author-grader` is `class: must`, so no anchor is owed.
- PASS — independence: no rule seats a grader on its own output; the reworded producer rule names a fresh peer of the author's persona type.
- PASS — reserved-to-user content: `feat.sec.reserved` carries `feat.user-reserved`.
- PASS — done-condition branch (desk): step 2 converges per visit and closes with a verdict against the condition.
- PASS — argued overage: not a budgeted class.

VERDICT: PASS

outcome: `audit: feature command pair · gate-grader-w2 · opus · 3 files · 1 rounds · 0 blocking`

## Unit 5 — command pair `implement`

VALIDATE: command pair `implement` — `plugins/mochiko/commands/implement.md` plus its seven rendered blocks.
Checklist run: canonical-scaffold criteria 1–11, judgment limbs.
Evidence read: `plugins/mochiko/commands/implement.md` · `mochiko-cli rules implement --section {preamble + the six} --plugin-root plugins/mochiko` · `.mochiko/schema-views/commands/implement.yaml` against `git show HEAD:`.
Pre-pass: `0 rejecting · 106 advisory`; for this document only `zero-member-label · command/implement · stewardship`, advisory and pre-existing.

## Unit 5 — item grades

- PASS — scaffold headings and order: canonical set in canonical order; Not done cites the printed `kind: fail` pin and obliges the halt when a delivered section's end-line count disagrees.
- PASS — preserved responsibilities: all four rewords are additive. `plan-approval-producers` keeps its exemption set with `verification` intact; `design-gaps-only` keeps "exactly the named gaps, nothing more" and its rung-justification pointer; `builder-decompose-disclose` keeps the disclosure and test-first clauses; `sound-loop-floor` keeps "This run's seat wiring already carries it end to end".
- PASS — floor survival: pin `class: floor · 20 rules`; `impl.sound-loop-floor` survives as a floor with its id and gains the `2026-09-03 producer-plan-enforcement D8` anchor it previously lacked.
- PASS — independence: `impl.builder-never-designs` and `impl.seat-verification-independence` stand; the exemption set keeps grading and verification seats out of the plan loop, so no seat grades its own plan.
- PASS — reserved-to-user content: `impl.sec.reserved` carries eight rules including the three user gates and the run-open rulings.
- PASS — done-condition branch (run, plus the implement rider): step 2 states a fixed done condition; Entry names the existing run-open confirmation with batch, scope type, both attempt bounds at their only redeclaration point, and the done condition stated, closing at the existing final-acceptance gate. No new ceremony was added.
- PASS — argued overage: not a budgeted class.

VERDICT: PASS

outcome: `audit: implement command pair · gate-grader-w2 · opus · 3 files · 1 rounds · 0 blocking`

## Unit 6 — command pairs `brainstorm`, `setup`, `specify`

VALIDATE: three command pairs changed only through the reworded common stub `common.plan-approval-producers`.
Checklist run: canonical-scaffold criteria 1–11, judgment limbs, on each pair.
Evidence read: `plugins/mochiko/commands/{brainstorm,setup,specify}.md` · `mochiko-cli rules {brainstorm,setup,specify} --section {preamble + the six} --plugin-root plugins/mochiko` · `.mochiko/schema-views/common/common.yaml` against `git show HEAD:`.
Pre-pass: `0 rejecting · 106 advisory`; `budget · command-common/common · - · 10 rules · 1837 resolved characters of rule text`, advisory. The pre-existing `unused-moment`, `enforces-coverage` and `zero-member-label` findings on `command/specify` and `command/brainstorm` are advisory and untouched by this wave.

## Unit 6 — item grades

- PASS — scaffold headings and order: all three carry the canonical set in canonical order, the four frontmatter keys, seven `!` lines, and Not done last citing the printed pin.
- PASS — section enumeration set-wise: each `.md` names its own six-set (`brainstorm.sec.*`, `setup.sec.*`, `spec.sec.*`); every token resolved against a live node in the render.
- PASS — preserved responsibilities: the stub reword keeps "grading and fact-finding seats are exempt" verbatim and extends only the approval clause. Inheritance resolves correctly on all three — `brainstorm.plan-approval-producers`, `setup.plan-approval-producers` and `spec.plan-approval` all render the new text.
- PASS — the lead, never DM (D8 item 2, M5): the inherited text reads "the lead approves only a passed plan", against the DM-form "you approve" that `impl.plan-approval-producers` keeps locally.
- PASS — floor survival: pins print `class: floor · 8`, `17` and `35` rules; the stub is `class: must` on all three, so no anchor is owed and no floor moved.
- PASS — independence and reserved content: reserved sections carry 5, 7 and 5 rules respectively; the exemption keeps grading seats out of the plan loop.
- PASS — done-condition branch (run): all three state a fixed done condition at step 2 and default it to FAIL at step 3. No sound-loop pointer floor is present on any of the three, and criterion 6 forbids demanding one there.

VERDICT: PASS

outcome: `audit: brainstorm/setup/specify command pairs · gate-grader-w2 · opus · 7 files · 1 rounds · 0 blocking`

## Unit 7 — skill pair `review-seat-plan` (new)

VALIDATE: skill pair `review-seat-plan` — new `SKILL.md` plus its seven rendered blocks.
Checklist run: skill-pair items 1–12, judgment limbs, with item 7 read as the born-in-wave form.
Evidence read: `plugins/mochiko/skills/review-seat-plan/SKILL.md` · `mochiko-cli rules review-seat-plan --section {preamble + the six} --plugin-root plugins/mochiko` · `.mochiko/schema-views/skills/review-seat-plan.yaml`.
Pre-pass: `0 rejecting · 106 advisory`; `budget · skill/review-seat-plan · - · 15 rules · 4339 resolved characters of rule text`, advisory. No condition-coverage, labels-inherited or zero-member-label finding names this document.

## Unit 7 — item grades

- PASS — load-first section: `## Rules — delivered by mochiko-cli` heading, seven `!` lines, `allowed-tools: Bash(mochiko-cli *)`, the no-raw-Read clause, and a Procedure that sequences the plan read before the tree reads.
- PASS — section enumeration set-wise: the review six-set `independence · scope · inputs · verdict · output · reserved`, matching the preamble's printed order. No section is empty.
- PASS — floor pin and read-back: pin prints `class: floor · 5 rules` as expected; the `floors:` line lists `author-grader · fresh-peer-grader · plan-verbatim · default-fail · approval-is-the-leads`; the body's read-back sentence cites both and hard-codes no number.
- PASS — floor survival and anchors: a born-in-wave skill retires nothing. Four of the five floors carry D2/D3/D6 anchors; `default-fail` and `author-grader` bind `review-common` blocks and declare `class:` locally.
- PASS — `extends:` conformance: four stubs, all `review-common.*`, same family, inheriting text/labels/pointer only, ids kept citable. The deliberate non-extension of `review-common.evidence-floor` is ruled by D2 (the plan is transient) and is recorded in the allowlist header.
- PASS — `description:` born-in-wave form: 839 characters, under the 1,536 delivery cap, carrying the MUST clause and the five SHOULD trigger phrases D4 obliges.
- PASS — independence: `fresh-peer-grader` bars the author's own context and the lead; `same-grader-regrades` resumes the grader rather than the author.
- PASS — reserved section: `approval-is-the-leads`, `class: floor`, `kind: reservation`, holding both the approval and the second-consumption escalation with the user.
- PASS — budget: payload 12,701 = body 3,534 + render 9,167, matching the birth-seed row to the character.

VERDICT: PASS

outcome: `audit: review-seat-plan pair · gate-grader-w2 · opus · 3 files · 1 rounds · 0 blocking`

## Unit 8 — schema content `0009-plan-qa-leg.yaml`

VALIDATE: schema content — the migration file plus its regenerated derived-view diff.
Checklist run: the AM-2 five — intent stated · anchor present where the exit requires one · ID lifecycle right · floor and fail survival · register.
Evidence read: `plugins/mochiko/migrations/0009-plan-qa-leg.yaml` (all 283 lines) · `git diff HEAD -- .mochiko/schema-views/` across the six modified files and the one new file.
Pre-pass: `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` → `0 rejecting · 106 advisory`; `mochiko-cli migrate status --plugin-root plugins/mochiko` → `sequences 1..9 (9 migrations)`, `75 documents · 1082 rules`.

## Unit 8 — item grades

- PASS — intent stated: the header `intent:` names all five moves — the leg-1 and disclosure rewords, the common-block and six-command sweep, the plan-minimalism re-point, and the new skill document.
- PASS — anchor where the exit requires one: no `supersede-rule` and no `tombstone-rule` op exists, so the log's hard set demands no exit anchor. The header carries `anchor: 2026-09-03 producer-plan-enforcement D8` regardless, and the binary's `protected-exit` and `anchor-format` checks passed. Two `set-rule-field` ops raise protection by adding anchors, which the log's README says needs no authority.
- PASS — ID lifecycle right: 13 `reword-rule`, 2 `set-rule-field`, 1 `import-document`, and nothing else. Every reword keeps its id — confirmed rule by rule against the view diff, which shows text changes under unchanged `- id:` keys. No split, no merge, no tombstone, so no parent-recording or loser-tombstoning is owed.
- PASS — floor and fail survival: the four touched floors (`patterns-sound-loop.leg-1-seat-produces`, `arch.author-grader-separation`, `arch.sound-loop-floor`, `feat.sound-loop-floor`, `impl.sound-loop-floor`) all survive as floors with their ids and pins unchanged — sound-loop 6, architecture 23, feature 14, implement 20. No `kind: fail` rule is touched by any op.
- PASS — register: sequence 9 allocated by the lead at wave open, the `hash:` header stamped and accepted by replay, the document count moving 74 to 75 and the rule count to 1,082, the derived views regenerated and proved identical to replay, and the new member pre-registered in the contract suite (unit 9).

VERDICT: PASS

outcome: `audit: schema content 0009-plan-qa-leg · gate-grader-w2 · opus · 8 files · 1 rounds · 0 blocking`

## Unit 9 — prose: contract-suite pre-registration

VALIDATE: prose primitive — the contract suite's pre-registration edits.
Checklist run: coherence and preserved responsibilities, with the README bar at "When a migration legitimately moves a floor set" as the domain lens.
Evidence read: `git diff HEAD --` over `evals/contract/expected-skills.json`, `evals/contract/run.py`, `evals/contract/README.md`, `scripts/similar-rules-allowlist.yaml`, plus `evals/contract/README.md` lines 254–274 read in full.
Pre-pass: `python3 evals/contract/run.py --list` counts **91** declared cases, including `review-seat-plan-delivery` and `review-seat-plan-absence`. The similarity sweep in `migrate validate --report` reports `clusters: 0 (none)` with `allowlist-suppressed edges: 170`.

- PASS — coherence of the figures: the README's "ninety-one cases" matches the first-hand `--list` count exactly. The 162-session figure reconciles: 2 fixture + 18 command delivery + 6 command absence + 3 mechanism + 96 skill delivery + 32 skill absence + 2 preload + 1 gate-live + 2 reminder-spawn. The floor-ids row moves 252 across 31 to 257 across 32, and the new member contributes exactly 5.
- PASS — preserved responsibilities, `expected-skills.json`: the row is an addition, not a replacement. Its four byte columns are 0 with the reason recorded in `baseline_source`, following the `validation-primitive-edit` post-freeze precedent set at v0.111.0; no pre-freeze row's byte column moved, and the provenance string appends rather than rewrites.
- PASS — preserved responsibilities, `run.py`: a single `PROBE_ARGUMENTS` row, with the free-text argument justified by D2 (the plan is never a file).
- PASS — allowlist honesty: four rows added, each naming its shape and its adjudication. Three cover the edges the new skill forms; the fourth covers `feat.sound-loop-floor`/`impl.sound-loop-floor` after the reword gave the pointer floors a shared opening clause. The bar holds: only two of the three pairs clear the threshold at all, so no 3-plus near-identical family exists and no extraction is owed. The sweep reports no unsuppressed cluster.

VERDICT: PASS

outcome: `audit: contract-suite pre-registration · gate-grader-w2 · opus · 4 files · 1 rounds · 0 blocking`

## Unit 10 — prose: router

VALIDATE: prose primitive — `plugins/mochiko/skills/mochiko/SKILL.md`, one new table.
Checklist run: coherence and preserved responsibilities.
Evidence read: `git diff HEAD -- plugins/mochiko/skills/mochiko/SKILL.md`.
Pre-pass: not a schema-bearing document; the run-wide `0 rejecting · 106 advisory` stands and no budgeted class moves (the router is a prose skill, unbudgeted for payload).

- PASS — coherence: the new heading follows the established form, naming the classification (model-invoked) and the reach site (every producing seat's plan-only dispatch, all six commands). The row's when-to-reach text states the unit graded, the verbatim-plan input fence, the seven items with the 1-to-6 blocking split, the fresh-peer carrier and its persona-less fallback, the return grammar, and the additive-to-leg-2 boundary. It reads consistently with the `validation-primitive-edit` row directly above it.
- PASS — preserved responsibilities: a pure addition. Nothing else in the file moves, and in particular the `validator` row, the mount-doctrine line and the checklist routing are untouched, which is correct — those are wave 3's, not this wave's.

VERDICT: PASS

outcome: `audit: router skills/mochiko/SKILL.md · gate-grader-w2 · opus · 1 files · 1 rounds · 0 blocking`

## Unit 11 — prose: budget ledger

VALIDATE: prose primitive — the `review-seat-plan` rows in `.mochiko/memory/primitive-cost-budgets.md`.
Checklist run: coherence and preserved responsibilities, scoped to the rows this unit adds.
Evidence read: `git diff HEAD -- .mochiko/memory/primitive-cost-budgets.md` · the file's "How to measure" section · the `patterns-sound-loop`, `patterns-plan-minimalism` and `patterns-model-tiering` rows read in full as comparators.
Pre-pass: the char-budget measurement above. The payload row's claimed figures reproduce exactly — 12,701 = body 3,534 + render 9,167 — and its claimed log state matches `migrate status`: `sha256:d5cb369a…`, 75 documents, 1,082 rules, floor pin 5.

- PASS — coherence: both rows are seeded on the fourth path (ruled birth seed) with no headroom, as the ledger's own rule requires, and each carries its ruling citation, its measurement conditions and the re-confirmation clause the release-gate sweep enforces. The seven per-block figures sum to 9,167.
- PASS — preserved responsibilities: additions only. No existing row's figure, note or seeding path moves. The description-norm paragraph follows the `patterns-model-tiering` disclosure precedent, naming the load-bearing clauses rather than trimming, and states the headroom under the delivery cap.
- Note, not a finding against this unit: the fix for unit 1's blocking issue lands in this file. Graded there, counted once, so this unit's blocking count stays 0.

VERDICT: PASS

outcome: `audit: primitive-cost-budgets.md · gate-grader-w2 · opus · 1 files · 1 rounds · 0 blocking`

## Unit 12 — prose: `primitive-edits.md` criterion 6

VALIDATE: prose primitive — criterion 6 of `.claude/rules/mochiko/primitive-edits.md`.
Checklist run: coherence and preserved responsibilities.
Evidence read: `git diff HEAD -- .claude/rules/mochiko/primitive-edits.md` · the whole file read for context, both criteria blocks.
Pre-pass: not a schema document; run-wide pre-pass stands.

- PASS — coherence: the replacement reads "A producing seat's plan graded by a fresh peer per `mochiko:review-seat-plan` and approved by the lead only on PASS before it works", which is exactly the standard D8 item 5-prime names and exactly what migration 0009 now ships in the schemas. The cited skill exists as of this wave, so the pointer resolves. The rest of criterion 6 — independence, reserved content, bindings, the floor, the DM-chartered pointer carve — is untouched and still reads as one sentence-chain.
- PASS — preserved responsibilities: a two-line replacement of one clause. Nothing else in the file moves; the grader-identity prose still describes the gate grader rather than the retiring `validator`, which is correct at wave 2.
- PASS — strip ceremony: none owed. This file is a repo rule under `.claude/rules/`, not a `plugins/mochiko/` primitive, and the ledger records the same disposition for AM-2's full rewrite of it.

VERDICT: PASS

outcome: `audit: primitive-edits.md criterion 6 · gate-grader-w2 · opus · 1 files · 1 rounds · 0 blocking`

## Unit 13 — prose: CHANGELOG and manifests

VALIDATE: prose primitive — the `## [0.112.0]` entry plus both version manifests.
Checklist run: coherence and preserved responsibilities.
Evidence read: `git diff HEAD --` over `CHANGELOG.md`, `plugins/mochiko/.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json`.
Pre-pass: the char-budget measurement above, against which the entry's quoted payload figures were checked.

- PASS — coherence: the entry states the defect the ruling closes, the new leg, the new skill and its seven items, the migration and what it rewords, the criterion-6 move, the router table, and the budget seed. Its quoted figures reproduce first-hand — payload 12,701 (body 3,534 + render 9,167), description 839, contract suite 91. The version is MINOR, correct for one new skill and no protected exit.
- PASS — manifests synced: both move 0.111.0 to 0.112.0, and nothing else in either file changes.
- PASS — preserved responsibilities: an append above the 0.111.0 entry; no prior entry moves.
- Note, not a finding: the entry's gates sentence is an explicit, clearly-marked placeholder to be filled at the bump from the wave plan section 5. That is the wave's own convention rather than a defect here, but the bump must not land with it unfilled, and release gate 4 reads the finished line.

VERDICT: PASS

outcome: `audit: CHANGELOG + manifests · gate-grader-w2 · opus · 3 files · 1 rounds · 0 blocking`

## Unit 14 — prose: crate fixtures

VALIDATE: prose primitive — the five crate test files re-keyed after migration `0009-plan-qa-leg.yaml`.
Checklist run: `primitive-edits.md`'s prose items — coherence and preserved responsibilities — against the crate bar at `.claude/rules/mochiko/rust-cli.md`, this seat standing as the independent non-author code review its quality-gate bullet requires.
Evidence read: `git diff HEAD --` over `crates/mochiko-cli/tests/{fidelity,validate,matrix_similar,views,render}.rs`, all five in full · `crates/mochiko-cli/Cargo.toml` · `git status --short` over `crates/mochiko-cli/src/` and `Cargo.lock` · `.claude/rules/mochiko/rust-cli.md`.
Pre-pass: `cargo test -p mochiko-cli` → **472 passed, 0 failed**, every `test result:` line `ok` across sixteen binaries (fidelity 17 · validate 100 · matrix_similar 48 · render 39 · views 11). Beside it the other three layers `rust-cli.md` names: `cargo fmt --all --check` clean, `cargo clippy -p mochiko-cli --all-targets -- -D warnings` clean, `cargo audit --deny warnings` clean over 31 dependencies.

## Unit 14 — census figures verified against the tree

| figure | moved | my own reading |
|---|---|---|
| documents | 74 → 75 | `migrate status`: `75 documents` |
| sequences | 1..8 → 1..9 | `migrate status`: `sequences 1..9 (9 migrations)` |
| command rules | 329, unchanged | 329 counted in the six command views |
| skill rules | 738 → 753 | 753 counted in the thirty-two skill views |
| total rules | 1,067 → 1,082 | `migrate status`: `1082 rules` |
| command floors | 117, unchanged | 117 counted |
| skill floors | 252 → 257 | 257 counted |
| command fail nodes | 36, unchanged | 36 counted; skills carry 0, as the grammar requires |
| primitives checked | 37 → 38 | 6 command + 32 skill views |
| corpus sweep | 1,067 / 156,764 / 0 / 171 → 1,082 / 161,376 / 0 / 170 | `migrate validate --report`, all four |

## Unit 14 — item grades

- PASS — every moved number matches the tree: all ten figures above reproduce against my own readings, taken from `mochiko-cli` output and the views rather than from the test files that assert them. The rule and floor counts came from the derived views, which this run's pre-pass proved identical to replay, anchoring `- id:` at six spaces and `class: floor` at eight so the prose mentions of that string inside folded rule text cannot be miscounted. `review-seat-plan` independently measures 15 rules and 5 floors, exactly the delta the comments attribute to it (753 − 738 = 15, 257 − 252 = 5).
- PASS — no assertion weakened or deleted: 16 assert lines removed and 16 added, each a same-shape replacement with a moved constant. Nothing was dropped, no threshold loosened, and `checked` moved 37 → 38, which strictly widens coverage by one primitive.
- PASS — `clusters == 0` still asserted, and its suppressed nature stated: both similarity tests keep the zero assertion, and each comment names it a SUPPRESSED zero with the raw-versus-pinned figures behind it — 29 clusters over 56 edges for the command family, 77 over 170 for the corpus. The falling suppressed counts (60 → 56 family, 171 → 170 corpus) are explained by the reword mechanism retiring allowlisted hits, and the corpus figure reproduces at 170 in my pre-pass.
- PASS — no crate source touched: the diff is five test files only, `12/25/5/11/8` insertions against `6/4/3/6/7` deletions. `git status --short` reports nothing under `crates/mochiko-cli/src/`, and `Cargo.lock` is unmoved.
- PASS — `Cargo.toml` unchanged at 0.2.0: confirmed by reading the manifest and by its absence from `git status`. Correct, since a fixture re-key ships no binary change and no `mochiko-cli-v*` tag rides this wave.
- PASS — coherence and preserved responsibilities: every moved constant carries a comment naming migration 0009, its ruling and anchor, and why the command figures and the fail set hold while only the skill side moves. The reasoning is right — every command-side op in 0009 is a `reword-rule`, which keeps id, class and section.

VERDICT: PASS

Issues requiring fix: none.

outcome: `audit: crate fixtures · gate-grader-w2 · opus · 5 files · 1 rounds · 0 blocking`

## Notes of note

The brief's fence was held: no `reports/w2-p*-plan.md` and no `reports/w2-dg-validator.md` was
opened at any point. No graded file was edited, and no git mutation was run — the only writes
this seat made were this report and scratchpad helpers, plus `mochiko-cli views emit`, which is
a named pre-pass command and proved to be a no-op against the tree.

Two observations that gate nothing. First, the `patterns-sound-loop` frontmatter `description:`
still reads "a seat produces on a lead-approved plan (never the lead)", which is no longer the
whole of leg 1 after this wave; it is not false and criterion 7 governs only byte-identity and
the delivery cap, so it is a note rather than a finding. Second, the two floors that gained
anchors in this migration are precisely the two that carried none before, while the three that
already carried `2026-08-13 charter-ritual-balance` kept it — a consistent rule, though the wave
plan's section 3.1 prose reads as though every reworded floor would carry the D8 anchor.
