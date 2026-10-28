# Wave 2 — specify keys its review on stakes; `review-specifications` carries the quick read (`stakes-non-product-surfaces`)

**Opened:** 2026-10-28 · **Lead:** the build session lead · **Branch:** `adaptive-rigor-build`
(@ `e9a13e2`, W1's commit) · **Target version:** 0.124.0, one bump after wave 3 · **Record:**
`.mochiko/brainstorms/stakes-non-product-surfaces/record.md` — D1-planning-quick-read (items 1, 7,
10), D2-borrow-product-label, D3-lead-proposes-label, DF4, DF5, DF6, OQ3, all as review-amended,
are the source for every rule text. This plan names carriers, never restates a ruling; where it
paraphrases, the card wins. W1's `patterns-sound-loop` rules (`0105`) are what these extend —
referenced by id, never restated.

Wave 2 makes `/mochiko:specify` consume the lever: it declares `stakes`, read at the intent
confirm from the confirmed intent synthesis; at a low label its stress-test takes the quick read
and its blind journey list may be skipped, stated; its fail rules keep their force through the
quick read and a new fail catches an unstated lowering. `review-specifications`, the stress-test's
grading skill, gains the quick-read form — the D8 bend's home. `review-seat-plan`'s `stakes`
note and its once-per-seat latitude learn that a planning run has no smoke (the W1 peer's
coverage note).

## 0. Sound-loop wiring

As `wave1-sound-loop.md` §0, unchanged: persona-less `general-purpose` producers, `model: opus`,
plan-only dispatch with the lead's `git status --porcelain` snapshot/diff; a fresh generic peer
grades each plan on `mochiko:review-seat-plan`, its render pasted verbatim and the plan text
delivered as the seat's own unedited scratchpad file *(amended 2026-10-28: inline pasting of a
400-line plan was judged too costly; the path form is disclosed in the build log as a deviation
from `review-seat-plan.plan-verbatim`'s letter, never a summary)*; one re-plan on a shared counter; one gate grader for the wave on the
`mochiko:validation-primitive-edit` render pasted verbatim; the routing line in every brief;
Write/Edit only. The lead relays every count change between seats in the order that lands it.

## 1. Sequence allocation

- **`0106-specify-planning-stakes.yaml`** — `command/specify`, `skill/review-specifications`,
  `skill/review-seat-plan`. Anchor `2026-10-28 stakes-non-product-surfaces` with the card segment
  per change. Sequence 106; 0107 held for W3.

## 2. Seats and ownership

- **P1 — schema.** Owns `plugins/mochiko/migrations/0106-specify-planning-stakes.yaml`, the
  regenerated `.mochiko/schema-views/` (`mochiko-cli views emit`), and any
  `scripts/similar-rules-allowlist.yaml` row a near-dup cluster forces. Nothing else.
- **P2 — prose.** Owns `plugins/mochiko/commands/specify.md` (only if its Identity, Entry, Goal
  or Not-done text must change to stay true — the Rules block untouched),
  `plugins/mochiko/skills/review-specifications/SKILL.md` and
  `plugins/mochiko/skills/review-seat-plan/SKILL.md` (headline, overview, `description:` trigger
  phrases only where the remit now needs them), their rows in
  `.mochiko/memory/primitive-cost-budgets.md`, their strip files under `.mochiko/strips/` only
  where prose is removed or superseded, and the router rows for the three primitives in
  `plugins/mochiko/skills/mochiko/SKILL.md` only where a row now says something false. Nothing
  else.
- **P3 — crate pins.** Owns `crates/mochiko-cli/tests/fidelity.rs`, `matrix_similar.rs`,
  `validate.rs`, and `evals/contract/expected-skills.json` (+ `evals/contract/README.md` where the
  precedent touches it): the sequence list, the census (command and skill rules, floors, fail
  nodes), the similarity figures and any frozen floor set `0106` moves. Plans after P1's plan is
  approved; executes after `0106` lands. *(Widened 2026-10-28 on P1's pin note:)* also
  `evals/contract/run.py` (`EXPECTED["specify"]` gains `spec.fail.skip-unstated` with its
  provenance comment). Nothing else.

## 3. Carriers

| id | class · kind | change | source | gist |
|---|---|---|---|---|
| `command/specify` condition `stakes` | — | set (new) | D2, D3, DF5, DF6, S6, S7 | the six classes; resolution moment-resolved at the intent confirm; the lead proposes the class with its cited line from the confirmed intent synthesis, the user rules it there; the higher of the spec's and the map delta's labels governs the one-pass stress-test; no cite → money, data or security; the quick reader may raise, only the user lowers — the key itself is `patterns-sound-loop.stakes-key`'s, referenced never restated |
| `command/specify` moment `intent` | — | reword (if the rule grammar needs the stakes line named there) | DF5 | the stage closing in the confirmed intent synthesis and the stakes line |
| `spec.stress-test-one-pass` | must (kept) | key / reword | D1 statement, item 7 | the full one-pass stress-test runs at money, data or security; at a low label the stress-test takes the quick read (next row) — P1 picks the shape (`when:` key, a clause, or both) and argues it in the plan |
| new `spec.stress-test-quick-read` | must | mint, `when: stakes=availability|wrong-answer|cosmetic` | D1 statement + S4, item 7, item 10 | at a low label the stress-test is `patterns-sound-loop.planning-quick-read` run on `mochiko:review-specifications`' quick-read form: a fresh non-author seat reads what the run wrote (a new spec whole), keeps the derivation and map-delta agreement checks and, on a UX-bearing feature, the prototype walk; reports "open hunt not run" |
| `spec.stress-test-prototype-walk` | must (kept) | reword | D1 item 7 | the walk runs inside the full stress-test and inside the quick read alike |
| `spec.blind-journey-list` | must · duty (kept) | key / reword | DF4 | at a low label the list may be skipped, stated in the trail |
| `spec.reserved-to-user` | (kept) | reword, if the stakes ruling is not already reserved by it | D3, DF5 | the stakes label ruled at the intent confirm is the user's; a lowering after it is the user's checkpoint ruling |
| trail home | — | P1 proposes (OQ3) | `patterns-sound-loop.trail-of-lowerings`, OQ3 | where specify records each lowering line — the run's existing written surface that best holds it; argued in the plan, graded at the audit |
| `spec.fail.blocking-gap`, `spec.fail.screens-flows` | floor · fail (kept) | `set-rule-field enforces:` | D1 item 7 | each gains `spec.stress-test-quick-read` beside the stress-test rule it enforces |
| new `spec.fail.skip-unstated` | floor · fail | mint | `trail-of-lowerings` ("A thinning or skip not stated fails the run under the command's own fail rule") | a thinned or skipped step with no trail line; enforces the trail rule and the quick-read / blind-list rules |
| `review-specifications` condition `stakes` | — | set (new) | D1 item 10 | carried in from the command's ruling; absent outside specify's runs |
| new `review-specifications.quick-read` | must | mint, `when:` low labels | D1 item 1 (D8 bend), item 10, S4 | the quick-read form: which of this skill's checks stay (the map-mirror / derivation checks, `joined-ids`, `slate-checks`, the S&F critical checks and `serve-and-click` where a manifest is present) and which open-hunt checks drop (`gap-taxonomy`, `complete-coverage`, the important-tier hunts); the report states "open hunt not run", never "no gaps"; anchored to D1 item 1 as the bend of `author-grader-consolidation` D8-input-job-conditions' completed-hunt rule for this review only |
| any `review-specifications` floor the quick read collides with (e.g. a completed-hunt or `complete-coverage` reading) | floor (kept) | reword with anchor, only if needed | D1 item 1 | carve the low-label quick read out by clause; P1 names each in the plan or states none collides |
| `review-seat-plan` condition `stakes` | — | re-set (note) | D1 item 3, DF4 (W1 peer note) | carried in from the command's ruling — a product run's or a planning output's; absent where neither |
| `review-seat-plan.once-per-seat-in-band` | must · latitude (kept) | reword | DF4 | where the run has no smoke (a planning output), the once-per-seat thinning stands for the run and returns to the full grade on any raise |

No floor retired. New floor: `spec.fail.skip-unstated` (command fail). P1's plan states the exact
before/after counts for P3.

## 4. Gate units (one grader, `general-purpose`, `model: opus`, the rendered `mochiko:validation-primitive-edit` contract pasted verbatim)

- **Unit 1 — schema content:** `0106` + the regenerated view diff (+ any allowlist row).
- **Unit 2 — command pair:** `specify.md` + its render (the canonical-scaffold criteria).
- **Unit 3 — skill pair:** `review-specifications/SKILL.md` + render (+ budget row, strip).
- **Unit 4 — skill pair:** `review-seat-plan/SKILL.md` + render (+ budget row, strip).
- **Unit 5 — prose primitive:** the router rows, if P2 touches them.
- **Unit 6 — crate code, non-author review:** the crate test diff; `cargo test --all --no-fail-fast`.
- **Unit 7 — contract freeze:** `expected-skills.json` (+ README) if `0106` moves a frozen set.

## 5. Close

All units PASS → the lead commits ("W2: specify keys its review on stakes"), appends the audit
outcome lines, transcribes any ruled-HOLDS verdicts, and opens W3. A FAIL → fix by the owning
producer, re-audit by the same grader on the touched files only; a second FAIL halts to the user.
