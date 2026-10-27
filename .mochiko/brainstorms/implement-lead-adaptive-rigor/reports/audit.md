---
report: review
round: 1
grader: w1-gate-grader
tier: opus
wave: 1
units: [1, 2]
contract: mochiko-cli rules validation-primitive-edit (preamble + six sections, binary 0.4.0, plugin 0.122.0) — floor pin 11, read back before the first step
---

## Pre-pass (shared, run first-hand by the grader)

- `mochiko-cli migrate validate --report --plugin-root plugins/mochiko` — tail: `pointer resolution: 89 checked against plugins/mochiko` · `similar-rule clusters (threshold 0.60): none — no pair clears the threshold` · `rules scanned: 1273 · in-kind pairs scored: 222775 · clusters: 0 (none)` · `mochiko-cli migrate validate · 0 rejecting · 118 advisory`. Sound-loop lines: `condition-coverage · skill/patterns-sound-loop · trigger · value "clear" is declared but named by no rule's when:` (present at baseline too) · `budget · skill/patterns-sound-loop · - · 21 rules · 9145 resolved characters of rule text`.
- Baseline `e546d5f` replayed in scratch: same validate shape, `0 rejecting · 118 advisory`, sound-loop `14 rules · 4505`; no new advisory enters with 0102 (no `unused-condition` for `stakes`).
- `mochiko-cli migrate status` — `sequences 1..102 (90 migrations)` · `state sha256:d652467793b5…` · `94 documents · 1273 rules` (baseline: `1..101 (89)` · `sha256:300e26a8…` · `1266 rules`).
- `mochiko-cli rules patterns-sound-loop --section preamble` — `- class: floor · 10 rules`; floors: two-part-trigger · leg-1-seat-produces · leg-2-non-author-review · no-delta-card-exemption · never-thins · review-form-invariants · only-real-seats · trail-of-lowerings · leg-3-user-gate-stays · user-rules-the-plan. Section totals 5 + 1 + 10 + 0 + 3 + 2 = 21.
- `mochiko-cli views emit --out <scratch>` then `diff -rq` against `.mochiko/schema-views` — identical across every document.
- Char-budget, canonical snippet plus the seven render blocks (characters of stdout), current vs baseline:

  | | body | description | preamble | trigger | scope | discipline | inputs | disclosure | reserved | render | payload |
  |---|---|---|---|---|---|---|---|---|---|---|---|
  | now | 3,315 | 746 | 2,436 | 4,184 | 698 | 5,096 | 423 | 1,938 | 929 | 15,704 | 19,019 |
  | e546d5f | 2,812 | 505 | 1,815 | 2,176 | 698 | 2,868 | 423 | 1,223 | 581 | 9,784 | 12,596 |

- Locate routing: one `Explore` subagent spawned `model: haiku` returned no report; the locate reads (disclosure-grammar callers, `stakes` mentions, `expected-skills.json` entry) ran on-seat.

## Unit 1 — schema content

```
VALIDATE: unit 1 — plugins/mochiko/migrations/0102-sound-loop-stakes.yaml + git diff .mochiko/schema-views/skills/patterns-sound-loop.yaml (schema content)
Checklist run: validation-primitive-edit.judgment-items-schema — the AM-2 five
Evidence read: plugins/mochiko/migrations/0102-sound-loop-stakes.yaml (whole); git diff .mochiko/schema-views/skills/patterns-sound-loop.yaml (whole); record.md D1–D8 cards and Defaults DF1–DF11; wave1-sound-loop.md §0–§5; plugins/mochiko/migrations/README.md "The anchor rule" and the op table; scripts/similar-rules-allowlist.yaml untouched (git status)
Pre-pass: migrate validate --report — "0 rejecting · 118 advisory", 0 clusters; views emit ≡ committed view; migrate status 1..102, 1273 rules
PASS intent stated — the header names every change and its card (D2 as amended, D3, D4, D5, D6 row 2, D8, DF1, DF2, DF5), the floor and rule deltas 6→10 and 14→21, both reproduced by the preamble render.
PASS anchor present where the exit requires one — no protected content exits (no supersede, tombstone or protection-lowering op); the header anchor `2026-10-27 implement-lead-adaptive-rigor` is present and carries the three rewords (two of them on anchored rules, one a floor); each of the seven mints carries its card anchor (D3, D5, D5, D4, D2, D1, D2).
PASS ID lifecycle right — seven ids minted once, no collision; the three rewords keep id, class, kind, labels and the charter-ritual-balance anchor (view diff shows text-only hunks); no tombstones.
PASS floor and fail survival — the two-part-trigger floor keeps its text byte for byte with one clause appended (D6 row 2); the disclosure-line reword keeps every prior segment and its clear-case sentence, widened to name the two new segments; no floor retired or lowered; skill grammar carries no `kind: fail`. Classes follow D1 and DF7: never-thins, review-form-invariants floor; the trail floor per D1's family-level five (DF7's "trail segment" is the disclosure-line segment, still must); the reservation floor per D1 question 1; stakes-key must · binding.
PASS register — texts trace the cards without drift: closed harm list and DF3's read-high and DF4's "stakes, never depth" (stakes-key); D5's four skippable non-leg steps and "a review leg thins, never vanishes" (stakes-band); D4's five invariants, consolidation D8's kept conditions and the two fixed gate forms (review-form-invariants); DF2's evidence list including the voiding surface (raise-free-lower-ruled); D8 S1's product-work scope (kind-keyed-library-wide). Library register kept: `·` lists, kinds from the eight-kind set, labels registry-legal per the pre-pass.
VERDICT: PASS
Issues requiring fix: none
```

## Unit 2 — skill pair

```
VALIDATE: unit 2 — plugins/mochiko/skills/patterns-sound-loop/SKILL.md + mochiko-cli rules patterns-sound-loop render (preamble + six sections) (skill pair)
Checklist run: validation-primitive-edit.judgment-items-pair, held to the skill-pair criteria 1–12 of .claude/rules/mochiko/primitive-edits.md
Evidence read: plugins/mochiko/skills/patterns-sound-loop/SKILL.md (whole) and its git diff; all seven render blocks; .mochiko/strips/patterns-sound-loop.md [v0.123.0] entry; .mochiko/memory/primitive-cost-budgets.md patterns-sound-loop row (git diff) and "## How to measure"; DECISIONS.md 2026-10-27 row; evals/contract/expected-skills.json patterns-sound-loop entry; build-log.md residual lines
Pre-pass: migrate validate --report — "0 rejecting · 118 advisory"; canonical snippet — body 3,315 · description 746 · render 15,704 · payload 19,019 (table above)
PASS scaffold headings and order — frontmatter, H1, bold headline lines, `## Overview`, `## Rules — delivered by mochiko-cli`; the Rules block, its seven `!` lines (preamble then the six patterns-family ids in printed order) and the read-back sentence are byte-identical to e546d5f (diff touches only frontmatter description, the headline's second line and the Overview's last sentence).
PASS preserved responsibilities — every prior sentence survives; the one superseded sentence ("this floor governs who produces and who reviews") has its [v0.123.0] supersession-by-ruling entry with verbatim content, D8 and the DECISIONS.md row cited, kept-deliberately and consumers-assessed filled; description is additive, 505 → 746, under the 1,536 cap (criterion 7's byte-identical limb is conversion-scoped and does not bind this post-conversion edit).
PASS floor survival — pin 6 → 10 and floors line read from the render; the read-back sentence cites both, no hard-coded count; no floor exits.
PASS independence, no seat grading its own row — review-form-invariants invariant (1) and never-thins' "no output cleared by its author" carry it; no rule admits self-grading.
PASS reserved-to-user content in the reserved section — user-rules-the-plan (floor · reservation) holds the stakes read, the review form, the plan of thinnings and any later lowering; stakes-key and raise-free-lower-ruled mention the user's ruling and defer to it.
PASS done-condition branch — n/a for a skill pair: no `moments:` and no done count; the skill's only halt is the delivery read-back, unchanged.
PASS argued overage — the overage +6,631 over 12,388 is argued in the budget row and the brief as D8-home-sound-loop's ruled obligation; every row figure is reproduced first-hand (payload 19,019, body 3,315, render 15,704 with all seven per-section counts, baseline 12,596 exactly, delta +6,423 = render +5,920 + body +503); the render growth is the 0102 obligation set alone, the body growth is the D6/D8-ordered headline and remit lines, and no playbook prose returns.
VERDICT: PASS
Issues requiring fix: none
```

## Outcome lines

- `audit: unit 1 — 0102-sound-loop-stakes.yaml + view diff (schema content) · w1-gate-grader · opus · 2 files · 1 rounds · 0 blocking`
- `audit: unit 2 — patterns-sound-loop SKILL.md + render (skill pair) · w1-gate-grader · opus · 3 files · 1 rounds · 0 blocking`

## Notes of note

- Advisory, unit 2: the Overview's last sentence restates the appended clause of `patterns-sound-loop.kind-keyed-library-wide` almost word for word. It is a second home for one rule's text and can drift on the next reword. Not blocking; trimming it to a pointer at that rule would also return about 230 characters of body.
- Owed before the bump, not this wave's: `evals/contract/expected-skills.json` still freezes `floor_pin: 6` and six floor ids for this skill; gate 6 needs the W3 re-freeze the build log names.
- Owed before the bump: `stakes-band` names the smoke procedure as `mochiko:testing-end-user`'s, which W2 lands; the one bump after W3 keeps the ship order safe.
- The `patterns-transport-floor` remit paraphrases and the router's missing stakes trigger are recorded as W3 residuals in the build log; not graded here.
