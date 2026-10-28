# Wave 3 — the feature and architecture desks key their grades on stakes (`stakes-non-product-surfaces`)

**Opened:** 2026-10-28 · **Lead:** the build session lead · **Branch:** `adaptive-rigor-build`
(@ `07123f6`, W2's commit) · **Target version:** 0.124.0, one bump after this wave · **Record:**
`.mochiko/brainstorms/stakes-non-product-surfaces/record.md` — D1-planning-quick-read (statement,
item 11, the revert route), D2-borrow-product-label, D3-lead-proposes-label, DF4, DF5, DF6, all as
review-amended, are the source for every rule text. This plan names carriers, never restates a
ruling; where it paraphrases, the card wins. W1's `patterns-sound-loop` rules (`0105`) and W2's
specify rules (`0106`) are what these mirror — referenced by id, never restated; W2's shapes
(`spec.stress-test-quick-read`, `spec.lowering-trail`, `spec.fail.skip-unstated`, the all-six
`when:` key plus a clause) are the precedent P1 follows unless it argues a difference.

Wave 3 makes `/mochiko:feature` and `/mochiko:architecture` consume the lever on their own
outputs — the feature desk's map writes (the card grade) and the architecture desk's store writes
(tech-lead's grade). Each declares `stakes`, read at the visit's goal line (DF5) from the product
work the write steers (D2); where the user rules the quick read on that line, the grade keeps its
hard checks — joined IDs, sameness of meaning against the visit file, and whatever else P1 argues
guards the record — and drops its open hunt, reporting "open hunt not run"; the visit's blind list
may be skipped, stated; a new fail per command catches an unstated lowering. A visit with nothing
to lighten states no line (DF5). Store writes made inside `/mochiko:implement`'s design phase stay
on implement's stakes (D1 statement); product baselines and epics stay outside the lever (DF2).

## 0. Sound-loop wiring

As `wave2-specify.md` §0, unchanged (plans to the peer by path, disclosed; the render pasted
verbatim; one re-plan on a shared counter; one gate grader for the wave; Write/Edit only; the lead
relays every count change between seats in the order that lands it, full op lists quoted).

## 1. Sequence allocation

- **`0107-desk-planning-stakes.yaml`** — `command/feature`, `command/architecture`, and the
  `template/feature-visit` / `template/architecture-visit` documents only if P1 argues the trail
  needs a template line (`replace-document`). Anchor `2026-10-28 stakes-non-product-surfaces` with
  the card segment per change. Sequence 107; the last of this build.

## 2. Seats and ownership

- **P1 — schema.** Owns `plugins/mochiko/migrations/0107-desk-planning-stakes.yaml`, the
  regenerated `.mochiko/schema-views/` (`mochiko-cli views emit`), and any
  `scripts/similar-rules-allowlist.yaml` row a near-dup cluster forces. Nothing else.
- **P2 — prose.** Owns `plugins/mochiko/commands/feature.md` and
  `plugins/mochiko/commands/architecture.md` (only where Identity, Entry, Goal or Not-done text must
  change to stay true — the Rules block untouched), their rows in
  `.mochiko/memory/primitive-cost-budgets.md` where one exists, their strip files under
  `.mochiko/strips/` only where prose is removed or superseded, and the router rows for the two
  commands in `plugins/mochiko/skills/mochiko/SKILL.md` only where a row now says something false.
  Nothing else. Plans after P1's plan is approved.
- **P3 — crate pins.** Owns `crates/mochiko-cli/tests/fidelity.rs`, `matrix_similar.rs`,
  `validate.rs`, `evals/contract/run.py` (`EXPECTED["feature"]`, `EXPECTED["architecture"]`),
  `evals/contract/expected-skills.json` and `evals/contract/README.md` where `0107` moves a frozen
  set or a pinned figure. Plans after P1's plan is approved; executes after `0107` lands. Nothing
  else.

## 3. Carriers

| id | class · kind | change | source | gist |
|---|---|---|---|---|
| `command/feature` condition `stakes` | — | set (new) | D2, D3, DF5, DF6 | the six classes; moment-resolved at the visit goal line (P1 names the moment: an existing one re-worded, as `0106` did `intent`, or the goal step's own); the lead proposes the class with its cited line from the product work the card steers, the user rules it on the goal line; only what `patterns-sound-loop.stakes-key` does not already say |
| `command/architecture` condition `stakes` | — | set (new) | D2, D3, DF5, DF6 | as feature's, read at the visit goal line; a store write inside implement's design phase is not this desk's and keeps implement's stakes |
| moments (`card-set` / `focus`, or the goal) | — | set-moment, if the grammar needs the stakes line named | DF5 | the point the stakes line is ruled; never a new gate |
| `feat.card-writer` | must · duty (kept) | key / reword | D1 statement, item 11 | the full card grade by `devils-advocate` runs at money, data or security; where the user ruled the quick read on the stakes line, the grade takes the quick read (next row) — P1 picks the shape and argues it |
| new `feat.card-grade-quick-read` (P1 names it) | must | mint, `when:` low three | D1 item 11 | a non-author seat keeps `feat.grader-checks-joined-ids` and the sameness-of-meaning check against the visit file, drops the adversarial open hunt; "open hunt not run"; the raise on a money, data or security surface per D3 |
| `arch.seat-tech-lead-grader` / `arch.rows-from-visit-file` | must (kept) | key / reword | D1 statement, item 11 | as feature's: the full grade at money, data or security; the quick read where the user ruled it |
| new `arch.grade-quick-read` (P1 names it) | must | mint, `when:` low three | D1 item 11 | keeps `arch.grader-checks-joined-ids`, sameness of meaning against the visit file, and each check P1 argues guards the record (the drawing against its `Draws:` targets; the outside-claims re-check) — or argues out; drops the open hunt; "open hunt not run"; the raise per D3 |
| `feat.card-blind-list` | must · duty (kept) | key / reword | DF4 | where the user ruled it on the stakes line, the list may be skipped, stated in the visit file |
| `arch.focus-blind-list` | must · duty (kept) | key / reword | DF4 | as feature's |
| `feat.user-reserved` / `arch.user-reserved-rulings` | (kept) | reword, if the stakes ruling is not already reserved | D3, DF5 | the stakes label ruled on the goal line is the user's; a lowering after it is the user's checkpoint ruling |
| trail home | — | P1 proposes | `patterns-sound-loop.trail-of-lowerings` | where each desk records a lowering line — the visit file is the natural surface (its `Blind list` section already carries a "skipped" line); a header line, a template change, or neither — argued, graded at the audit |
| new `feat.fail.skip-unstated`, `arch.fail.skip-unstated` | floor · fail | mint | `trail-of-lowerings` | a thinned or skipped step with no trail line; enforces the trail rule and the quick-read / blind-list rules |
| plan-check thinning | — | none expected | DF4, `review-seat-plan.once-per-seat-in-band` (0106) | already reaches a planning output through the skill's own `stakes`; P1 confirms no command rule needs keying (as `0106` ruled for `spec.plan-approval`) |

No floor retired. New floors: the two command fails (and `arch.dm-converge-goal` is a floor — P1
states whether it is touched). P1's plan states the exact before/after counts for P3.

## 4. Gate units (one grader, `general-purpose`, `model: opus`, the rendered `mochiko:validation-primitive-edit` contract pasted verbatim)

- **Unit 1 — schema content:** `0107` + the regenerated view diff (+ any allowlist row).
- **Unit 2 — command pair:** `feature.md` + its render.
- **Unit 3 — command pair:** `architecture.md` + its render.
- **Unit 4 — prose primitive:** the router rows, if P2 touches them.
- **Unit 5 — crate code, non-author review:** the crate test diff; `cargo test --all --no-fail-fast`.
- **Unit 6 — contract freeze:** `run.py` (+ `expected-skills.json`, README) if `0107` moves a
  frozen set.

## 5. Close

All units PASS → the lead commits ("W3: the desks key their grades on stakes"), appends the audit
outcome lines, transcribes any ruled-HOLDS verdicts, and opens the ship (0.124.0). A FAIL → fix by
the owning producer, re-audit by the same grader on the touched files only; a second FAIL halts to
the user.
