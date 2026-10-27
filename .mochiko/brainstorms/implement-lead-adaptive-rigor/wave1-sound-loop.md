# Wave 1 — the host: `patterns-sound-loop` gains the stakes lever (`implement-lead-adaptive-rigor`)

**Opened:** 2026-10-27 · **Lead:** the build session lead · **Branch:** `adaptive-rigor-build`
(from `main` @ `8a90439`, first commit `e546d5f`) · **Target version:** 0.123.0, one bump after
wave 3 (the user's ruling, build log 2026-10-27) · **Record:**
`.mochiko/brainstorms/implement-lead-adaptive-rigor/record.md` — D1-floor-four-questions,
D2-stakes-read-ruling (as amended), D3-harm-band-smoke, D4-review-form-invariants,
D5-band-step-latitude, D6-prior-rulings-fate, D8-home-sound-loop, and the defaults batch DF1–DF7,
all as review-amended, are the source for every rule text. This plan names carriers, never
restates a decision; where it paraphrases, the card wins.

Wave 1 lands the lever's single home. `patterns-sound-loop` gains the `stakes` condition, the
key, the band, the never-thins floor, the five review invariants, the raise-free rule, the trail
duty and the user's reservation over the plan; its trigger floor and library-wide rule take the
one clause D6 row 2 orders; its disclosure line gains the `stakes:` and `thinned:` segments; its
`SKILL.md` headline, overview and description say the remit now covers how deep. Spokes (W2)
and implement's keys (W3) extend what this wave mints.

## 0. Sound-loop wiring

The trigger fires on every item: judgment-authored writes to plugin primitives.

- **Leg 1.** Each producer is a persona-less `general-purpose` seat, `model: opus`, spawned once
  with a plan-only brief. The lead snapshots `git status --porcelain` before the dispatch and
  diffs it after the seat returns; any new or changed repo path is a FAIL, `dirty`. A fresh
  generic peer (`general-purpose`, `model: opus`) grades the plan default-FAIL per
  `mochiko:review-seat-plan`, its render pasted verbatim. The lead approves only a PASS and
  resumes the same seat with the plan quoted. Re-plan bound: one, on a counter the producers
  share; a second consumption goes to the user.
- **Leg 2.** §4, one gate grader for the wave (DF11: the locked gate form).
- **Leg 3.** The user gates any coverage gap the audit turns up and the merge; the lead commits
  at the wave close on the standing authorization for this branch, never pushes.
- **Transport.** Seats are subagents with disjoint ownership (§2); no cross-seat messaging;
  every hand-off is lead-relayed, each order quoting the text it lands. Every brief carries the
  routing line: locate and enumerate reads go to a native `Explore` subagent spawned
  `model: haiku`; interpretive reads stay on the seat. Artifacts are written with Write/Edit,
  never a shell copy.

## 1. Sequence allocation

- **`0102-sound-loop-stakes.yaml`** — `skill/patterns-sound-loop`: one `set-condition`, seven
  `mint-rule`, three `reword-rule`. Anchor `2026-10-27 implement-lead-adaptive-rigor` with the
  card segment per change (D3 for the key, D5 for the band and never-thins, D4 for the
  invariants, DF1/DF2 for raise-free, D1/DF5 for the trail, D2 for the reservation, D6 for the
  two clause rewords and D8 for the disclosure line). Sequence 102; 0103–0104 held for W2 and
  W3.

## 2. Seats and ownership

- **P1 — schema.** Owns `plugins/mochiko/migrations/0102-sound-loop-stakes.yaml` and the
  regenerated `.mochiko/schema-views/` (`mochiko-cli views emit`), plus any
  `scripts/similar-rules-allowlist.yaml` row a near-dup cluster forces. Nothing else.
- **P2 — prose.** Owns `plugins/mochiko/skills/patterns-sound-loop/SKILL.md` (headline, overview,
  `description:`; the Rules block and the read-back sentence untouched),
  `.mochiko/memory/primitive-cost-budgets.md` (the `patterns-sound-loop` row), and
  `.mochiko/strips/patterns-sound-loop.md` (a supersession-by-ruling entry for the remit
  sentence). Nothing else.

## 3. Carriers

| id | class · kind | section | when | labels | source | gist |
|---|---|---|---|---|---|---|
| condition `stakes` | — | preamble | — | — | D3, D2 | values `money|data|security|availability|wrong-answer|cosmetic` · resolution `entry-derived` · note: the harm class of the run's product work, proposed by the entry grading seat where the command has one (else the lead), raised never lowered by the lead, ruled by the user at the command's run-open confirmation; absent where the reviewed object is not product work |
| `patterns-sound-loop.stakes-key` | must · binding | trigger | — | trigger, binding | D3, D2, DF3, DF4 | the closed list of product harms with a cited line; who proposes, raises, rules; never a count (files, cards, rows, gaps, users); the word is "stakes", the governance `depth` dial keys code standards and is never read to size process; an axis with no cited line reads as the fixed band |
| `patterns-sound-loop.stakes-band` | must | trigger | `stakes: [money, data, security, availability, wrong-answer, cosmetic]` *(amended 2026-10-27 at P1's plan round: keyed so the band binds only where a class is resolved — product work — and the `unused-condition` advisory does not enter the baseline)* | trigger, ladder | D3, D5, D6 row 2 | money, data, security: no non-floor step thins, whatever a probe says; availability, wrong-answer, cosmetic: any `class: must` verification step the command keys may thin and four non-leg steps may be skipped stated, on a clean cold smoke per the user-ruled plan (the smoke procedure is `mochiko:testing-end-user`'s, W2); stakes keys the depth of the legs after the trigger fires, never whether they fire; a review leg thins, never vanishes |
| reword `patterns-sound-loop.two-part-trigger` | floor (kept) | trigger | — | kept | D6 row 2 | append one clause: stakes, a harm kind, never gates it either — it keys the depth of the legs after the trigger fires (`patterns-sound-loop.stakes-band`); anchor D6 |
| reword `patterns-sound-loop.kind-keyed-library-wide` | must (kept) | trigger | — | kept | D8, S1 | append: the stakes key and band bind product-run reviews only; where the reviewed object is not product work — a plugin primitive, a governance surface, a brainstorm record — no harm class applies, there is no band, and the review runs in its fixed form |
| `patterns-sound-loop.never-thins` | floor | discipline | — | boundary, evidence | D5, D1 | at any stakes: the user's gates · outcome honesty, no output cleared by its author · the trail · one non-author read of the built thing before acceptance · the machine-checked record layer · every step guarding a shared record (the baseline diff's mechanical legs, the built-vs-signed diff, the landing verifier on a run that wrote a record, the entry sufficiency check) · `**TEST:**` execution on real infrastructure · probe breadth; a code revert undoes code, never a shared record or a ruling taken from the user |
| `patterns-sound-loop.review-form-invariants` | floor | discipline | — | independence | D4 (as review-amended) | for the input-job reviews of a run's product work the form is the lead's per run, named in the run-open line, bound by five invariants — not the author · the surface, never the report · starts at FAIL · tier at or above the producer, named on the spawn · blind where the form is a hunt — plus the kept conditions for the input jobs `author-grader-consolidation` D8 names, and nothing else; the gate jobs it fixed (the primitive-edit audit, setup's governance validation) keep their forms; named forms are examples, never a menu |
| `patterns-sound-loop.raise-free-lower-ruled` | must | discipline | — | ladder, user-gate | DF1, DF2 | raising rigor is always free and needs no line; after run-open the read moves up on in-run evidence — a consumed attempt, a failed gate, a deviation, a fenced surface found in the diff — by the lead's hand; it never moves down except by the user's checkpoint ruling |
| `patterns-sound-loop.trail-of-lowerings` | floor · duty | disclosure | — | reporting, evidence | D1 q3, DF5, S6 | every thinned or skipped step is proposed at run-open and recorded in the command's final report file — the step, the smoke result it rode on, what still covers the failure kind; a skip not stated fails the run under the command's own fail rule; a raise needs no line |
| reword `patterns-sound-loop.disclosure-line` | must · duty (kept) | disclosure | — | kept | DF5, D8 | the pinned grammar gains two segments: `stakes: <class>|n/a` and `thinned: <step>(<cover>)…|none` |
| `patterns-sound-loop.user-rules-the-plan` | floor · reservation | reserved | — | user-gate | D2 (as amended), D1 q1 | the stakes read and the plan of thinnings are the user's ruling at the command's existing run-open confirmation — no new gate; a lowering after it is the user's checkpoint ruling |

Floors: 6 → 10. Rules: 14 → 21. No floor retired; two floor texts gain a clause by anchored
reword (D6). No `kind: fail` (skill grammar). No `${var}`.

## 4. Gate units (one grader, `general-purpose`, `model: opus`, the rendered `mochiko:validation-primitive-edit` contract pasted verbatim)

- **Unit 1 — schema content:** `0102-sound-loop-stakes.yaml` + the regenerated view diff (+ any
  allowlist row). Items: intent stated · anchor present on every protected exit · ID lifecycle
  right · floor and fail survival · register. Pre-pass run by the grader: `mochiko-cli migrate
  validate --report --plugin-root plugins/mochiko`, 0 rejecting.
- **Unit 2 — skill pair:** `patterns-sound-loop/SKILL.md` + its render. Items: the twelve
  skill-pair criteria; `description:` ≤ 1,536 chars; budget = payload against the ledger row, the
  overage argued in the brief as D8's ruled obligation (seven rules and a condition the record
  orders into this one home).

## 5. Close

All units PASS → the lead commits the wave on the branch ("W1: patterns-sound-loop gains the
stakes lever"), appends the audit outcome lines to the build log, and opens W2. A FAIL → fix by
the same producer, re-audit by the same grader on the touched files only; a second FAIL halts to
the user with both fix lists.
