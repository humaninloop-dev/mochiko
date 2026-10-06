# Wave 1 — the brainstorm target state (`brainstorm-target-state` D1–D23)

**Opened:** 2026-10-06 · **Lead:** the brainstorm session lead · **Branch:** `brainstorm-target-state`
(from `main` @ `2e4c57c`) · **Target version:** 0.117.0 (MINOR — new rules, rewords, no floor or
fail minted or retired) · **Ruling:** `DECISIONS.md` row 2026-10-06 ·
**Record:** `.mochiko/brainstorms/brainstorm-target-state/record.md` (the decision cards are the
source for every rule text; this plan names carriers, never restates a decision).

One wave. Brainstorm only (D19): no rule of `setup` or any other command moves, and
`review-governance-intent` is untouched.

## 0. Sound-loop wiring

The trigger fires on every item: judgment-authored writes to plugin primitives.

- **Leg 1.** Each producer is a persona-less `general-purpose` seat, `model: opus`, spawned once
  with a plan-only brief. The lead snapshots `git status --porcelain` before the dispatch and
  diffs it after the seat returns; any new or changed path is a FAIL, `dirty`. The plan comes
  back verbatim. A fresh generic peer (`general-purpose`, `model: opus`) grades it default-FAIL
  per `mochiko:review-seat-plan`, the render pasted into its brief. The lead approves only a
  PASS and resumes the same seat with the plan quoted. Re-plan bound: one, on a counter the
  three producers share; a second consumption goes to the user.
- **Leg 2.** §4, one gate grader for the wave.
- **Leg 3.** The user gates the commit and the merge. The lead never commits unasked and never
  pushes.
- **Transport.** Subagents, disjoint ownership (§2), no cross-seat messaging; every hand-off is
  lead-relayed. Every seat brief carries the routing line: locate and enumerate reads go to a
  native `Explore` subagent spawned `model: haiku`.

## 1. Sequence allocation

- **`0044-brainstorm-target-state.yaml`** — `command/brainstorm`.
- **`0045-review-brainstorm-target-state.yaml`** — `skill/review-brainstorm`.
- `0046` held for a fix round; a gap is legal if unused.

Header anchor on both: `2026-10-06 brainstorm-target-state`. A minted rule that carries one
ruled decision takes that decision as its rule-level anchor (`… D10`).

## 2. Seats and ownership (disjoint)

| Seat | Owns | Depends on |
|---|---|---|
| **P1 schema** | the two migration files · `.mochiko/schema-views/**` (re-emit only) · `scripts/similar-rules-allowlist.yaml` (rows only for edges the sweep reports) | — |
| **P2 prose** | `plugins/mochiko/commands/brainstorm.md` · `plugins/mochiko/skills/analysis-iterative/SKILL.md` · `plugins/mochiko/skills/review-brainstorm/SKILL.md` + `references/RECORD-FITNESS.md` · the three strip files (`.mochiko/strips/brainstorm.md` · `analysis-iterative.md` · `review-brainstorm.md`) · ripple lines the build makes false (`README.md`, the router `skills/mochiko/SKILL.md`, `docs/`, `ARCHITECTURE.md` — found by grep, reworded only where false) · `.mochiko/memory/primitive-cost-budgets.md` · `CHANGELOG.md` 0.117.0 entry | P1 landed (the budget is measured against the render) |
| **P3 tests and kits** | the crate's frozen-census tests under `crates/mochiko-cli/tests/` (no crate source, no crate version bump) · `evals/contract/` expectations and README figures, only where a count or pin they freeze moved · `evals/plan/brainstorm/` (the kit re-keyed to the new rule set) · `evals/review-brainstorm/` if it freezes rule ids | P1 landed |

The manifests (`plugin.json`, `marketplace.json`) are the lead's last mechanical step, after
the audit passes and before the gates run (GI-004: no bump before the audit).

Order: P1 plan → peer grade → approve → build · then P2 and P3 in parallel, each plan → peer
grade → approve → build · §4 audit · manifests · §5 gates · §6 landing.

## 3. Content

### 3.0 What every rule text must respect (D22)

- **Guardrails, never a sequence.** A rule says what must be true before something else
  happens. No rule says "first … then …", and no rule numbers steps.
- **Three counted limits and no others:** three ratified answers in a row (D8) · three
  questions without the map shrinking (D15) · a second failed verify goes to the user (D17).
- **Short.** One or two sentences per minted rule, in plain words. P1 reports the command's
  and the skill's render size before and after.
- **No floor and no fail is minted, retired, or lowered.** Pins stay: `brainstorm` 8 floors and
  4 fails, `review-brainstorm` 9 floors. One floor is reworded and keeps its id (§3.2).
- **Texts come from the cards.** Verbatim where the card phrases a rule; every
  changed-at-review and changed-at-verify note on a card is part of that decision.

### 3.1 Migration 0044 — `command/brainstorm`

The ids below are the lead's proposal. P1 may merge or split in its plan, with the reason;
the approved plan fixes the set. Every decision in the coverage table must keep a carrier.

**Declarations.** `set-condition size` (values `small|standard|too-big`, ruled by the user on
the frame card; P1 picks the resolution word from the grammar's legal set) · `set-moment
frame-hardened` (where the user's first reply confirms the frame card and rules the size) ·
`set-moment cold-review` reworded so it no longer names the two-message dispatch as the only
form.

**Rewords (ids survive).**

| Rule | Change | Decision |
|---|---|---|
| `brainstorm.lead-inline-questioning` | drop "one question per turn, format adapted to the user's state"; the lead, inline questioning and the pointer stay | D6 |
| `brainstorm.blind-map-dispatch` | standard and too-big sessions: the review seat is spawned at frame hardening with the frame's problem, destination and out-of-scope lines only, never the record; questioning does not wait for the map; its angles are folded into the decision map marked as from the blind map, dropped ones listed; the same seat is resumed with the record for the end read; a fresh seat with the saved map only if the first is gone. The fence stays structural | D10 |
| `brainstorm.pair-maps-independent` | each seat of a pair draws its own front map | D10 (S16) |
| `brainstorm.non-coverage-survivors` | findings reach the user sorted: own turn · one named batch of lead repairs · listed with the reason when nothing changes | D17 |
| `brainstorm.acceptance-plain-text` | adds: acceptance is the user's explicit word; anything vaguer is asked once more | D18 |

**Mints.** All `class: must`.

| Section | Proposed id | Carries |
|---|---|---|
| roles | `brainstorm.own-rules-win` | D19 — where these rules differ from `mochiko:analysis-iterative`, these rules win |
| roles | `brainstorm.small-session-review` (`when: size=small`) | D11 — no front map, no blind second lists; the review is drawn at the end in two messages, or waived by the user |
| roles | `brainstorm.blind-second-list` | D7, D20 — on a costly fork a fresh seat, never the review seat, gets the frame and the question and not the lead's options; the lead shows where the lists differ |
| reserved | `brainstorm.frame-changes-users-word` | D3 — after hardening a frame line changes only by the user's word, one line at a time, stamped; a changed destination re-checks the decisions made under it |
| reserved | `brainstorm.size-users-ruling` | D11 — proposed on the card, ruled in the confirming reply; raised freely, never lowered without the user's word |
| reserved | `brainstorm.user-steers` | D4, D6, D12, D15 — the user may stop at any time, pick the next question, promote a default to its own turn; moving something out of scope takes the user's word |
| tools | `brainstorm.decision-cards` | D9 — one named card per decision with its fixed parts; everything else points at a card by name with at most a one-line gist |
| tools | `brainstorm.facts-quoted` | D5 (S6) — a fact enters the record as a quoted source line with path and line; the lead's reading is written apart from it |
| ways-of-working | `brainstorm.light-orient` | D1 |
| ways-of-working | `brainstorm.frame-card` | D2 — six lines, each marked as the user's words or the lead's guess; no confidence percentage |
| ways-of-working | `brainstorm.frame-hardens` | D3 — at the user's first reply; a line the user cannot answer is marked open and becomes an early decision |
| ways-of-working | `brainstorm.decision-map` | D4 — three parts, shown with the first question, never a gate, a status line each turn |
| ways-of-working | `brainstorm.fact-before-decision` | D5 |
| ways-of-working | `brainstorm.own-turn-or-batch` | D6 |
| ways-of-working | `brainstorm.options-rule` | D7 — at least two real options, the cheaper shape among them, the rejected road named |
| ways-of-working | `brainstorm.ratified-yes` | D8 — counted limit one |
| ways-of-working | `brainstorm.dependency-order` | D12 as changed by D22 — the dependency clause only; which askable question goes first stays the lead's judgment |
| ways-of-working | `brainstorm.question-form` | D13 (S5: an open question to an unsure user stays allowed) |
| ways-of-working | `brainstorm.show-before-asking` | D14 |
| ways-of-working | `brainstorm.stop-rule` | D15 as amended at verify (V3) — counted limit two |
| ways-of-working | `brainstorm.fix-one-card-verify-once` (`kind: bound`) | D17 — counted limit three |
| ways-of-working | `brainstorm.accept-screens` | D18 |
| ways-of-working | `brainstorm.size-shapes` | D11 — what too big obliges: rule the askable forks, hand the fog to named later sessions |

Untouched: every `extends: common.*` stub, every floor, the four `kind: fail` rules,
`brainstorm.no-git-mutations`, `brainstorm.coverage-survivor-routing` (D17 keeps today's routing
for end coverage findings), `brainstorm.reopen-born-verify`.

### 3.2 Migration 0045 — `skill/review-brainstorm`

**Declaration.** A condition that tells the seat whether its first brief asks for the map at
frame hardening or at the end (proposed `map_timing`, values `front|end`, entry-derived).

| Op | Rule | Change | Decision |
|---|---|---|---|
| reword (floor, id survives) | `review-brainstorm.blind-map-before-record-contact` | the map is drawn from what the first brief carries — the topic and goal, or the frame's problem, destination and out-of-scope lines — and is still its own deliverable before any record contact | D10 |
| reword | `review-brainstorm.verify-pass-grade` | the verify pass reads the changed cards and their gists; the rest of the rule stands | D17 |
| mint | `review-brainstorm.resumed-end-read` (`when: map_timing=front`) | resumed with the record: today's hunt, plus whether each angle the session took was carried deep enough, plus the dropped angles; no second map; the seat says plainly that it has a stake in its own angles (S7) | D10 |
| mint | `review-brainstorm.card-parts-first` | the seat's first step with the record is a checklist that every card has its fixed parts, reported before it reads for judgment; pointer `references/RECORD-FITNESS.md` | D16 (S4) |
| mint | `review-brainstorm.ratified-first` | the hunt aims first at decisions recorded as ratified | D8, D16 |
| mint | `review-brainstorm.builder-test` | reading only the record, what would a builder still have to ask the user, held against the frame's destination | D16 |

P1 places each mint in the family section it fits and names the section in its plan.

### 3.3 Prose (P2)

- **`commands/brainstorm.md`** — reword only what the new rules make false: "one question at a
  time" in Identity & Mission; the Goal step's "statement + rationale + confidence mark" where
  D9's card replaces it (the confidence marks stay). Scaffold, Entry and Not-done untouched.
  Strip entry, supersession by ruling.
- **`skills/analysis-iterative/SKILL.md`** — D21's two repairs and nothing else: the sentence
  that points at "the adaptive flow above", and the Output line, reworded to defer to the calling
  command's deliverable (setup's output unchanged). The Common Mistakes rows stay as written:
  D19 supersedes them for brainstorm only, through `brainstorm.own-rules-win`. The strip file
  records the two repairs and a scoped supersession note beside the v0.63.0 kept-deliberately
  line.
- **`skills/review-brainstorm/SKILL.md` + `references/RECORD-FITNESS.md`** — the Protocol prose
  follows the rules: the map at the front or at the end, the card checklist first, ratified
  decisions first, the builder test, the end read's two additions. `RECORD-FITNESS.md` gains the
  card's fixed parts as the checklist. The `description:` value stays byte-identical. Strip
  entries wherever content leaves or is superseded.
- **Ripple** — `README.md` (lines 61 and 130 name the old flow) and any other line grep finds
  restating a changed behaviour.
- **Budgets** — `review-brainstorm`'s delivered payload re-measured with the ledger's canonical
  snippet. Growth is expected and argued: D10, D16 and D17 are new obligations, not restored
  playbook prose. `analysis-iterative`'s body re-measured (budget 4,928).
- **`CHANGELOG.md`** — `## [0.117.0] — 2026-10-06`.

### 3.4 Tests and kits (P3)

- Crate frozen-census tests re-keyed to the post-0045 state; figures read from `migrate status`
  and the render, never computed by hand.
- `evals/contract/` — expectations and README figures only where a frozen count moved. No floor
  set moves, so `expected-skills.json`'s floor fields should stand; P3 confirms from the render.
- `evals/plan/brainstorm/` — `observable.yaml`, `evals.json` and the preregistration re-keyed to
  the new rule set; the kit's static check passes. No grid is run.

### 3.5 Decision coverage

| Decision | Carrier |
|---|---|
| D1 | `brainstorm.light-orient` |
| D2 | `brainstorm.frame-card` |
| D3 | `brainstorm.frame-hardens` · `brainstorm.frame-changes-users-word` · moment `frame-hardened` |
| D4 | `brainstorm.decision-map` · `brainstorm.user-steers` |
| D5 | `brainstorm.fact-before-decision` · `brainstorm.facts-quoted` |
| D6 | `brainstorm.own-turn-or-batch` · `brainstorm.lead-inline-questioning` reworded |
| D7 | `brainstorm.options-rule` · `brainstorm.blind-second-list` |
| D8 | `brainstorm.ratified-yes` · `review-brainstorm.ratified-first` |
| D9 | `brainstorm.decision-cards` · the command's Goal step · `RECORD-FITNESS.md` (the long index is kept: no change) |
| D10 | `brainstorm.blind-map-dispatch` · `brainstorm.pair-maps-independent` · `review-brainstorm.blind-map-before-record-contact` · `review-brainstorm.resumed-end-read` · the skill's Protocol prose |
| D11 | condition `size` · `brainstorm.size-users-ruling` · `brainstorm.size-shapes` · `brainstorm.small-session-review` |
| D12 | `brainstorm.dependency-order` · `brainstorm.user-steers` |
| D13 | `brainstorm.question-form` |
| D14 | `brainstorm.show-before-asking` |
| D15 | `brainstorm.stop-rule` |
| D16 | `review-brainstorm.card-parts-first` · `.ratified-first` · `.builder-test` |
| D17 | `brainstorm.non-coverage-survivors` · `brainstorm.fix-one-card-verify-once` · `review-brainstorm.verify-pass-grade` |
| D18 | `brainstorm.accept-screens` · `brainstorm.acceptance-plain-text` |
| D19 | `brainstorm.own-rules-win` · the scoped supersession note in the strip file |
| D20 | no carrier of its own: a build-and-watch ruling (the `BACKLOG.md` dogfood watch) |
| D21 | the two repairs in `analysis-iterative/SKILL.md` |
| D22 | §3.0, binding every rule text; the three counts in `ratified-yes`, `stop-rule`, `fix-one-card-verify-once` |
| D23 | no carrier: the `BACKLOG.md` dogfood watch |

## 4. Audit (leg 2)

One gate grader for the wave: a plain `general-purpose` seat, `model: opus` explicit. Its brief
carries the verbatim render of `mochiko-cli rules validation-primitive-edit` (every block), the
unit list with file paths, the pre-pass commands, and the named budget overage. It runs the
pre-pass itself and quotes it. Units, one verdict block and one outcome line each:

1. schema content — 0044 and its view diff
2. schema content — 0045 and its view diff
3. the `brainstorm` command pair
4. the `review-brainstorm` skill pair
5. `analysis-iterative` (prose primitive) with its strip entries
6. ripple lines, the budget ledger, the changelog entry
7. crate fixtures and contract expectations
8. the `evals/plan/brainstorm` kit

On a FAIL the owning seat fixes and the same grader is resumed to read only the delta. A second
FAIL on any unit halts to the user (`common.gate-loop-bound`).

Beside the audit, the lead reads the built rules against §3.5 decision by decision. That read
is the lead checking seat output; it clears nothing on its own.

## 5. Gates before the bump lands

`migrate validate --report` 0 rejecting, clusters 0 · views ≡ replay · `cargo test -p
mochiko-cli` + fmt + clippy + audit · the full similarity sweep · the contract suite, full run
in Docker (a SKIPPED suite blocks) · char budgets on every touched budgeted primitive ·
`CHANGELOG.md` entry · both manifests at 0.117.0 · strips recorded.

## 6. Landing

`DECISIONS.md` 2026-10-06 row and its five annotated rows → built at v0.117.0 · `BACKLOG.md`:
the build item → the trail, the dogfood watch stays open, the cold-review watch annotation
confirmed · `ROADMAP.md` Now row · the record's and the index's Landed lines · the five prior
records' "takes effect at that build" notes → built · `build-log.md` closed with the `floor:`
line and every `audit:` line · a commit suggested to the user. The installed plugin copy and
the dogfood repo's copy are upgraded by the user after the merge; the dogfood runs follow.

## 7. Stop conditions

Plan FAILs from more than half the producers · any second re-plan · any tree dirtied during
planning · a rejecting `migrate validate` finding a seat cannot resolve inside its ownership ·
any floor or fail minted, retired or lowered · any crate source file touched · a second FAIL on
any gate unit · the grader not holding the `review-brainstorm` overage argument · the contract
suite not green after one fix round. Each halts the wave and goes to the user.
