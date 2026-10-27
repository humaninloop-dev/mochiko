---
report: review
round: 1
grader: w2-gate-grader
tier: opus
wave: 2
units: [1, 2, 3, 4, 5]
contract: mochiko-cli rules validation-primitive-edit (preamble + six sections, binary 0.4.0, plugin 0.122.0) — floor pin 11, read back before the first step
---

## Pre-pass (shared, run first-hand by the grader)

- Contract read-back before the first step: `- class: floor · 11 rules`; floors author-grader · plain-seat-explicit-tier · rendered-contract-only · from-file-floor · pre-pass-first-hand · binary-verdict · default-fail · tamper-proof-clause · gate-loop-bound · evidence-floor · second-fail-user. Section totals 4 + 3 + 3 + 7 + 3 + 1 = 21.
- `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` (exit 0) — tail: `pointer resolution: 90 checked against plugins/mochiko` · `similar-rule clusters (threshold 0.60): none — no pair clears the threshold` · `rules scanned: 1281 · in-kind pairs scored: 224283 · clusters: 0 (none)` · `allowlist-suppressed edges: 186` · `mochiko-cli migrate validate · 0 rejecting · 118 advisory`.
- The four budget advisory lines: `budget · skill/review-seat-plan · - · 16 rules · 4967 resolved characters of rule text` · `budget · skill/review-sufficiency · - · 30 rules · 8684 …` · `budget · skill/testing-end-user · - · 30 rules · 8891 …` · `budget · skill/testing-gap-finding · - · 37 rules · 9186 …`.
- Baseline `5c4ba7c` (`git archive` into scratch, replayed with `--plugin-root`): `0 rejecting · 118 advisory`, `pointer resolution: 89`, `rules scanned: 1273`, `allowlist-suppressed edges: 185`; budget lines 15 · 29 · 25 · 36 rules. A line-by-line diff of the two reports, budget lines aside, differs only in pointer count (+1, the `references/REPORT-TEMPLATES.md` pointer of `testing-end-user.smoke-report`), rules scanned and suppressed edges. No `condition-coverage` line names `stakes`.
- Allowlist check: the current tree validated against the baseline allowlist yields `[1] CROSS-PAIR · kind: binding · best 0.64` between `testing-end-user.smoke-seat` and `testing-gap-finding.seat-devils-advocate`, `clusters: 1`; with the appended row, 0. The row suppresses exactly that edge.
- `mochiko-cli migrate status` — `sequences 1..103 (91 migrations)` · `state sha256:6cf0a382ed28…` · `94 documents · 1281 rules`.
- Spoke preambles: `testing-gap-finding` `- class: floor · 9 rules` · `testing-end-user` `7` · `review-seat-plan` `5` · `review-sufficiency` `8`; floors lines name the same ids as the baseline render. Each preamble now prints the `stakes` condition except `review-sufficiency`, which declares none and keys no rule on it.
- `mochiko-cli views emit --out <scratch>` then `diff -rq` against `.mochiko/schema-views` — identical across every document.
- Note (c) prefix replays: `review-seat-plan.sec.scope` renders 1,034 characters at sequences 1..18 (`A floor-, compliance-module-, or NFR-derived obligation …`) and 1,013 at 1..19 (`A floor- or NFR-derived obligation …`). The −21 traces to `0019-setup-agnostic-modules-out.yaml`, as the row claims.
- Char-budget, canonical snippet (body, description) plus the seven render blocks as characters of stdout, now vs `5c4ba7c`:

  | skill | body | desc | preamble | indep. | scope | inputs | verdict | output | reserved | render | payload |
  |---|---|---|---|---|---|---|---|---|---|---|---|
  | testing-end-user now | 9,763 | 521 | 1,826 | 760 | 4,467 | 820 | 2,989 | 4,361 | 739 | 15,962 | 25,725 |
  | testing-end-user base | 9,593 | 500 | 1,551 | 448 | 3,124 | 820 | 2,573 | 3,892 | 739 | 13,147 | 22,740 |
  | testing-gap-finding now | 7,005 | 770 | 2,743 | 1,200 | 4,399 | 3,518 | 2,251 | 2,237 | 1,506 | 17,854 | 24,859 |
  | testing-gap-finding base | 7,005 | 770 | 2,484 | 1,200 | 3,428 | 3,518 | 2,251 | 2,237 | 1,506 | 16,624 | 23,629 |
  | review-seat-plan now | 3,534 | 839 | 1,894 | 1,531 | 1,875 | 1,080 | 2,309 | 894 | 696 | 10,279 | 13,813 |
  | review-seat-plan base | 3,534 | 839 | 1,623 | 1,531 | 1,013 | 1,080 | 2,309 | 894 | 696 | 9,146 | 12,680 |
  | review-sufficiency now | 3,262 | 686 | 1,827 | 514 | 1,839 | 2,225 | 6,440 | 1,297 | 1,021 | 15,163 | 18,425 |
  | review-sufficiency base | 3,262 | 686 | 1,827 | 514 | 1,839 | 2,225 | 5,726 | 1,200 | 1,021 | 14,352 | 17,614 |

- Locate routing: one `Explore` subagent spawned `model: haiku` returned no report; the locate reads (record card and default line ranges, wave-plan sections) ran on-seat.

## Unit 1 — schema content

```
VALIDATE: unit 1 — plugins/mochiko/migrations/0103-spokes-stakes.yaml + git diff .mochiko/schema-views/skills/{testing-gap-finding,testing-end-user,review-seat-plan,review-sufficiency}.yaml + git diff scripts/similar-rules-allowlist.yaml (schema content)
Checklist run: validation-primitive-edit.judgment-items-schema — the AM-2 five
Evidence read: plugins/mochiko/migrations/0103-spokes-stakes.yaml (whole); the four view diffs (whole); the allowlist diff; record.md D2-stakes-read-ruling (as amended), D3-harm-band-smoke, D5-band-step-latitude, D8-home-sound-loop and Defaults DF1–DF11; wave2-spokes.md §0–§5 including the 2026-10-28 carriers amendment; plugins/mochiko/migrations/README.md op table and "The anchor rule"; the patterns-sound-loop render's stakes-key, stakes-band, never-thins, raise-free-lower-ruled, trail-of-lowerings and user-rules-the-plan; the report envelope's extra-field clause
Pre-pass: migrate validate --report — "0 rejecting · 118 advisory", 0 clusters, 186 suppressed; views emit ≡ committed views; migrate status 1..103, 1281 rules
PASS intent stated — the header names every change, its skill and its cards (D2 as amended, D3, D5, D8, DF2, DF3, DF8), the reword and the cleared note, what stays sound-loop's, and "floors 9 · 7 · 5 · 8 unchanged", which the four preambles reproduce.
PASS anchor present where the exit requires one — nothing protected exits: no supersede, tombstone or set-rule-field op; the one rule reword (`review-sufficiency.report-binding`, anchored) keeps its id and anchor, and the header anchor `2026-10-27 implement-lead-adaptive-rigor` is present anyway; every one of the eight mints carries its card anchor (D5, D3, D3, D5, D3, D3, D5, D2), matching the carriers table.
PASS ID lifecycle right — eight ids minted once; the rule counts 36→37, 25→30, 15→16, 29→30 agree with the validate budget lines; `report-binding` keeps id, class must, kind binding and its plan-stage-utility anchor, its prior text a verbatim prefix with one clause appended; `testing-end-user.sec.independence` keeps its id, its `note: ~` a legal clear per the README op table; no tombstone.
PASS floor and fail survival — no floor rule is touched in any of the four schemas and every floors line is unchanged; skill grammar carries no `kind: fail`; the new latitudes stay outside the never-thins floor (plan-QA and the gap-finding pass are in D5's thinnable and skippable sets) and leave the TGF `delta-lane-skip-stated` floor and the review-seat-plan floors (plan-verbatim, approval-is-the-leads, default-fail) intact — each later plan still reaches the lead verbatim.
PASS register — texts trace the cards: the five smoke legs, the diff-hold voiding and any red going full depth (D3, S5, DF2); the QA seat's no-product-code wording (D3, N3); the method-scoped weak negative (D3); `smoke: true` on a `verification` report (D3, S8), the envelope's "Unknown extra fields are permitted" confirming the claim; the cold run always on the accepted head, repeating after rework within the gap-rework bound, no CI stand-in (D5, S7, DF9); the stated skip with what still covers the failure kind (D5, S4/N4); once per seat per run, re-staffed or post-FAIL graded again, provisional until the smoke (D5 worked plan, carriers Q2); the proposal from one cited line, per epic member, uncited reading money, data or security, a proposal never a ruling (D2 as amended, DF3, DF8). The band and raise and ruling authority point at sound-loop, never restated. The whole-list `when:` follows the carriers amendment and `0102`'s `stakes-band`. The allowlist row covers a two-member edge, below the 3+ near-dup bar, with a reason that holds (census-small-families §C: no common block).
VERDICT: PASS
Issues requiring fix: none
```

## Unit 2 — skill pair `testing-end-user`

```
VALIDATE: unit 2 — plugins/mochiko/skills/testing-end-user/SKILL.md + mochiko-cli rules testing-end-user render (preamble + six sections) (skill pair)
Checklist run: validation-primitive-edit.judgment-items-pair, held to the skill-pair criteria 1–12 of .claude/rules/mochiko/primitive-edits.md
Evidence read: plugins/mochiko/skills/testing-end-user/SKILL.md (whole) and its git diff; the preamble, independence, scope, verdict and output render blocks (whole), inputs and reserved by measured size; .mochiko/memory/primitive-cost-budgets.md line 69 and line 315 (git diff) and "## How to measure"; evals/contract/expected-skills.json (floor ids and pin only); references/REPORT-TEMPLATES.md and the report-envelope frontmatter clause
Pre-pass: migrate validate --report — "0 rejecting · 118 advisory", `budget · skill/testing-end-user · - · 30 rules · 8891`; canonical snippet — body 9,763 · description 521 · render 15,962 · payload 25,725 (shared table)
PASS scaffold headings and order — frontmatter, H1, `## Overview`, the bold spirit line, `## Rules — delivered by mochiko-cli` with its seven `!` lines in the preamble's printed order and the read-back sentence, then the unchanged process sections; the diff touches only the `description:` value and the Overview's first paragraph.
PASS preserved responsibilities — every prior sentence survives; the description is additive, 500 → 521, inside its 625 budget and the 1,536 cap (lead note a; criterion 7's byte-identical limb is conversion-scoped); the independence section's census note was schema content cleared by `0103`, nothing superseded, so no strip entry is owed (lead note d).
PASS floor survival — pin `- class: floor · 7 rules` and the floors line read from the render, unchanged from baseline and from expected-skills.json; the read-back sentence cites both, no hard-coded count.
PASS independence, no seat grading its own row — `testing-end-user.smoke-seat` binds the smoke to the QA seat that wrote no product code in the run; no rule lets a seat clear its own output.
PASS reserved-to-user content in the reserved section — the reserved section is unchanged (739); the smoke's thinning rides the user's run-open ruling, stated as such in `smoke-clean-weak-negative`, and the ruling itself stays `patterns-sound-loop.user-rules-the-plan`'s.
PASS done-condition branch — n/a for a skill pair: no `moments:`, no done count; the only halt is the delivery read-back, unchanged.
PASS argued overage — +4,015 over 21,710 is argued in the row as D8-home-sound-loop's ruled obligation (the smoke procedure and cold-run clause land in the skill that owns them, with D3 S5/S8, D5, D8's verify-pass residual); every figure reproduced first-hand: payload 25,725, body 9,763, render 15,962 with all seven per-section counts, baseline 22,740 exactly, delta +2,985 = render +2,815 (preamble +275 · independence +312 · scope +1,343 · verdict +416 · output +469) + body +170; description row 521 reproduced. No playbook prose returns: the body gains one pointer sentence.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 3 — skill pair `testing-gap-finding`

```
VALIDATE: unit 3 — plugins/mochiko/skills/testing-gap-finding/SKILL.md (unchanged) + mochiko-cli rules testing-gap-finding render (preamble + six sections) (skill pair)
Checklist run: validation-primitive-edit.judgment-items-pair, held to the skill-pair criteria 1–12 of .claude/rules/mochiko/primitive-edits.md
Evidence read: git diff of plugins/mochiko/skills/testing-gap-finding/ (empty); its seven `!` lines and read-back sentence against the preamble's section list; the preamble and scope render blocks (whole) and the independence block's seat rule; .mochiko/memory/primitive-cost-budgets.md line 92 (git diff)
Pre-pass: migrate validate --report — "0 rejecting · 118 advisory", `budget · skill/testing-gap-finding · - · 37 rules · 9186`; canonical snippet — body 7,005 · description 770 · render 17,854 · payload 24,859 (shared table)
PASS scaffold headings and order — `SKILL.md` byte-identical to `5c4ba7c`; its `!` lines enumerate preamble then the six section ids in the preamble's printed order.
PASS preserved responsibilities — the render only gains the `stakes` condition and `testing-gap-finding.band-skip-stated`; `one-pass-at-final-validation` and the floor `delta-lane-skip-stated` render unchanged, and the new rule cites the former by id rather than rewording it.
PASS floor survival — pin `- class: floor · 9 rules` and the floors line unchanged; read-back sentence present.
PASS independence, no seat grading its own row — the skip can only be ruled by the user, never on the finder's or the lead's own mid-run call; the blind seat binding is untouched.
PASS reserved-to-user content in the reserved section — reserved block unchanged (1,506); the skip's authority is the user's run-open or checkpoint ruling, owned by sound-loop's reservation floor and referenced.
PASS done-condition branch — n/a for a skill pair.
PASS argued overage — +4,882 over 19,977 is argued in the row as D8's ruled obligation (gap finding's stated skip lands in the skill that owns the pass; D5, D3); figures reproduced: payload 24,859, render 17,854 with all seven counts, baseline 23,629 exactly, delta +1,230 render only (preamble +259 · scope +971); body and description unchanged. No playbook prose.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 4 — skill pair `review-seat-plan`

```
VALIDATE: unit 4 — plugins/mochiko/skills/review-seat-plan/SKILL.md (unchanged) + mochiko-cli rules review-seat-plan render (preamble + six sections) (skill pair)
Checklist run: validation-primitive-edit.judgment-items-pair, held to the skill-pair criteria 1–12 of .claude/rules/mochiko/primitive-edits.md
Evidence read: git diff of plugins/mochiko/skills/review-seat-plan/ (empty); its seven `!` lines and read-back sentence against the preamble's section list; the preamble, independence, scope and verdict render blocks; .mochiko/memory/primitive-cost-budgets.md line 94 (git diff); prefix replays 1..18 and 1..19 of the baseline log
Pre-pass: migrate validate --report — "0 rejecting · 118 advisory", `budget · skill/review-seat-plan · - · 16 rules · 4967`; canonical snippet — body 3,534 · description 839 · render 10,279 · payload 13,813 (shared table)
PASS scaffold headings and order — `SKILL.md` byte-identical to `5c4ba7c`; `!` lines match the preamble's printed order.
PASS preserved responsibilities — the render only gains the `stakes` condition and `review-seat-plan.once-per-seat-in-band`; the seven items, plan-not-artifact, same-grader-regrades and its-command-states-them render unchanged, and the new rule restates no loop or round-cap mechanics.
PASS floor survival — pin `- class: floor · 5 rules` and the floors line unchanged; plan-verbatim and approval-is-the-leads hold because each later plan still reaches the lead verbatim.
PASS independence, no seat grading its own row — fresh-peer-grader is untouched; the latitude reduces how often a peer grades, never who grades.
PASS reserved-to-user content in the reserved section — reserved block unchanged (696); the thinning stands only per the user's run-open plan, with the raise rule pointed at sound-loop.
PASS done-condition branch — n/a for a skill pair.
PASS argued overage — +1,112 over the 12,701 seed is argued in the row as D8's ruled obligation (the plan leg's once-per-seat form, D5's worked thin plan); figures reproduced: payload 13,813, render 10,279 with all seven counts, baseline 12,680, delta +1,133 from 0103 (preamble +271 · scope +862) and the pre-wave −21 confirmed by the 1..18 and 1..19 replays (scope 1,034 → 1,013 at `0019`); body and description unchanged. No playbook prose.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 5 — skill pair `review-sufficiency`

```
VALIDATE: unit 5 — plugins/mochiko/skills/review-sufficiency/SKILL.md (unchanged) + mochiko-cli rules review-sufficiency render (preamble + six sections) (skill pair)
Checklist run: validation-primitive-edit.judgment-items-pair, held to the skill-pair criteria 1–12 of .claude/rules/mochiko/primitive-edits.md
Evidence read: git diff of plugins/mochiko/skills/review-sufficiency/ (empty); its seven `!` lines and read-back sentence against the preamble's section list; the inputs render block's fence rules; the verdict and output view hunks for the minted and reworded rules; .mochiko/memory/primitive-cost-budgets.md line 80 (git diff)
Pre-pass: migrate validate --report — "0 rejecting · 118 advisory", `budget · skill/review-sufficiency · - · 30 rules · 8684`; canonical snippet — body 3,262 · description 686 · render 15,163 · payload 18,425 (shared table)
PASS scaffold headings and order — `SKILL.md` byte-identical to `5c4ba7c`; `!` lines match the preamble's printed order.
PASS preserved responsibilities — `report-binding`'s prior carried set survives verbatim with the stakes field appended; the minted `stakes-proposal` reads only inside the fence, so `fence-never-reads` stands.
PASS floor survival — pin `- class: floor · 8 rules` and the floors line unchanged; verdict-is-input agrees with "a proposal, never a ruling".
PASS independence, no seat grading its own row — the sufficiency seat proposes and the user rules, per D2 as amended; no seat rules its own proposal.
PASS reserved-to-user content in the reserved section — reserved block unchanged (1,021); who may raise the class and who rules it is pointed at sound-loop's stakes key, which carries both.
PASS done-condition branch — n/a for a skill pair.
PASS argued overage — +3,002 over 15,423 is argued in the row as D8's touch-set sentence as changed at review (S8: D2's harm-class field and duty; D3, DF3, DF8); figures reproduced: payload 18,425, render 15,163 with all seven counts, baseline 17,614 exactly, delta +811 render only (verdict +714 · output +97); body and description unchanged. No playbook prose.
VERDICT: PASS
Issues requiring fix: none
```

## Outcome lines

- `audit: unit 1 — 0103-spokes-stakes.yaml + four view diffs + allowlist row (schema content) · w2-gate-grader · opus · 6 files · 1 rounds · 0 blocking`
- `audit: unit 2 — testing-end-user SKILL.md + render (skill pair) · w2-gate-grader · opus · 2 files · 1 rounds · 0 blocking`
- `audit: unit 3 — testing-gap-finding SKILL.md + render (skill pair) · w2-gate-grader · opus · 2 files · 1 rounds · 0 blocking`
- `audit: unit 4 — review-seat-plan SKILL.md + render (skill pair) · w2-gate-grader · opus · 2 files · 1 rounds · 0 blocking`
- `audit: unit 5 — review-sufficiency SKILL.md + render (skill pair) · w2-gate-grader · opus · 2 files · 1 rounds · 0 blocking`

## Notes of note

- Advisory, unit 5: `review-sufficiency.stakes-proposal` names its cite sources as "the spec, a baseline or the store". The fence also admits the capability map entries and, under delta scope, the delta card. On a delta-scope run whose harm shows only in the card, the proposal reads uncited and so reads high. That is the safe direction, so it does not block. W3 may want to widen the parenthetical or cite the fence rule by id.
- Advisory, unit 2: the Overview's new sentence names the QA seat, which is `testing-end-user.smoke-seat`'s content. It is a short second home for one binding. Trimming it to "the stakes smoke probe lives here too; its seat, legs and report are rules below" would keep it a pure pointer.
- Unit 4's "provisional until the smoke" reading comes from the lead's 2026-10-28 carriers amendment, not from a record line. It is the only coherent reading of D5, because plan grades run before the smoke. The ruling is recorded in the wave plan.
- Owed before the bump, not this wave's: `patterns-sound-loop.stakes-band` names the smoke procedure as `mochiko:testing-end-user`'s, which this wave lands. Implement's keys and the expected-skills re-freeze stay W3's.
