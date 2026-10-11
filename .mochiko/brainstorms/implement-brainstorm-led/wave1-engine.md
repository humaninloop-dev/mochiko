# Wave 1 — the shared thinking engine and its first two movers (`implement-brainstorm-led` D1–D4)

**Opened:** 2026-10-11 · **Lead:** the build session lead · **Branch:** `engine-desk-build` (from
`main` @ `5162f1a`) · **Target version:** 0.120.0, one bump after wave 3 (the user's ruling, build
log 2026-10-11) · **Record:** `.mochiko/brainstorms/implement-brainstorm-led/record.md` — D1–D4 as
review-amended, the *Defaults batch*, *Supersessions owed at the build* and *Touch set for the
build* are the source for every rule text; this plan names carriers, never restates a decision.

Wave 1 builds the engine (`mochiko:analysis-iterative` converted to a rendered skill), the shared
pointer block, and moves brainstorm's deciding and specify's story stage onto it. Setup's
interrogation and specify's intent stage take the engine through their unchanged pointers — no
setup rule and no intent rule is edited (frame change 3). The desk (wave 2) and implement (wave 3)
extend the block this wave mints.

## 0. Sound-loop wiring

The trigger fires on every item: judgment-authored writes to plugin primitives.

- **Leg 1.** Each producer is a persona-less `general-purpose` seat, `model: opus`, spawned once
  with a plan-only brief. The lead snapshots `git status --porcelain` before the dispatch and diffs
  it after the seat returns; any new or changed path is a FAIL, `dirty`. A fresh generic peer
  (`general-purpose`, `model: opus`) grades the plan default-FAIL per `mochiko:review-seat-plan`,
  its render pasted verbatim. The lead approves only a PASS and resumes the same seat with the plan
  quoted. Re-plan bound: one, on a counter the three producers share; a second consumption goes to
  the user.
- **Leg 2.** §4, one gate grader for the wave.
- **Leg 3.** The user gates the obligation map's mismatches and gains (§3.1), the commit and the
  merge. The lead never commits unasked, never pushes.
- **Transport.** Seats are subagents with disjoint ownership (§2); no cross-seat messaging; every
  hand-off is lead-relayed. Every brief carries the routing line: locate and enumerate reads go to a
  native `Explore` subagent spawned `model: haiku`; interpretive reads stay on the seat.

## 1. Sequence allocation

- **`0058-thinking-engine.yaml`** — `skill/analysis-iterative` (`import-document`, the review
  six-set — `crates/mochiko-cli/tests/validate.rs:909`: an unprefixed stem falls through to the
  review family by ruling; no section set minted).
- **`0059-thinking-stage-block.yaml`** — `command-common/common` (`mint-rule`, one block).
- **`0060-brainstorm-on-engine.yaml`** — `command/brainstorm`.
- **`0061-specify-on-engine.yaml`** — `command/specify`.
- **`0062-spec-review-boundary.yaml`** — `skill/review-specifications`, only if §3.6 finds the line
  false; otherwise unused.
- `0063`–`0064` held for a fix round; a gap is legal if unused.

Header anchor on all: `2026-10-11 implement-brainstorm-led`. A rule carrying one ruled decision
takes it as its rule-level anchor (`… D1`); a confirmed default takes the bare session anchor. A
supersession of a rule traceable to another session's ruling carries this session's anchor and
names the superseded ruling in its `disposition`.

## 2. Seats and ownership (disjoint)

| Seat | Owns | Depends on |
|---|---|---|
| **P1 schema** | the five migration files · `research/obligation-map.md` in this session dir · `.mochiko/schema-views/**` (re-emit only) · `scripts/similar-rules-allowlist.yaml` (rows only for edges the sweep reports, or rows naming an id this wave retires) | — |
| **P2 prose** | `plugins/mochiko/skills/analysis-iterative/**` (SKILL.md rewritten; `SPECIFICATION-INPUT.md` and `ENRICHMENT.md` deleted; `SYNTHESIS.md` and `references/` kept) · the router `skills/mochiko/SKILL.md` · `commands/brainstorm.md` and `commands/specify.md` (lines made false only) · `skills/review-specifications/SKILL.md` (only if 0062 lands) · strip files (`.mochiko/strips/analysis-iterative.md` · `brainstorm.md` · `specify.md` · `mochiko.md` or the router's file · any other a strip lands in) · ripple lines made false (`README.md`, `ARCHITECTURE.md`, by grep) · `.mochiko/memory/primitive-cost-budgets.md` · `CHANGELOG.md` `[0.120.0]` entry (the W1 part; later waves append) | P1 landed |
| **P3 tests and kits** | crate frozen-census tests under `crates/mochiko-cli/tests/` (no crate source, no crate version bump) · `evals/contract/` expectations (`expected-skills.json`, `run.py` `Expected[...]`, README figures and the prose-skill list near line 370) · `evals/plan/brainstorm/` and `evals/plan/specify/` (`observable.yaml` partitions re-keyed, every live schema rule id in exactly one list) | P1 landed |

Order: P1 plan → peer grade → approve → build → the lead puts §3.1's mismatches and gains to the
user · then P2 and P3 in parallel, each plan → peer grade → approve → build · §4 audit.

## 3. Content

### 3.0 What every rule text must respect

- **Guardrails, never a sequence.** A rule says what must be true; moments anchor, never order.
- **Short.** One or two sentences per minted rule, plain words. P1 reports each render's size
  before and after.
- **One home per mechanic.** After this wave no asking-mechanic rule text survives duplicated
  across the brainstorm and specify renders and the engine's (default *"Ships once", measured*):
  the similar-rules sweep quiet on them, every render resolving the pointer.
- **Nothing weakens.** Every floor and fail of `brainstorm` and `specify` stands; a moved
  `must` rule leaves only by `supersede-rule` (each is traceable to a ruling — `brainstorm-target-
  state`, `specify-brainstorm-discovery`), its disposition naming its new carrier.
- **No grammar or binary change.** If one proves needed, it goes back to the user (default *Slots
  declared where*).
- **Texts come from the cards and the confirmed defaults**, every changed-at-review and verify
  note included. The lead's ids below are proposals; P1 may merge or split with the reason; the
  approved plan fixes the set; every row of §3.7 keeps a carrier.

### 3.1 The obligation map (D3-four-move-now as changed at review, S7; gains added at V1)

P1's plan carries it; P1's build writes it to `research/obligation-map.md`. One row per asking
obligation in today's renders of: `brainstorm` (every rule that governs how the user is asked) ·
`specify`'s story stage and intent stage (`spec.story-forks`, `spec.story-stage-stop-rule`,
`spec.intent-probe-discipline`, `spec.lead-holds-story-stage`, `spec.analyst-brief-and-pen`,
`spec.slate-binds-authoring`, `spec.stage-inputs-in-hand`, `spec.slate-provenance`,
`spec.blind-journey-list`, `spec.intent-stage-first`, `spec.own-rules-win`) · `setup`'s
interrogation (`setup.interrogation-inline` and every rule it leans on). Columns: rule id · the
obligation in a few words · its carrier after the wave (an engine id, the command's own domain
id, or *none*) · status — `kept` · `reworded` · `lost` · `gained`. A `lost` row (a dropped word
such as specify's "one-row" mock) and every `gained` row (an obligation a caller takes from the
engine's defaults that it did not carry — setup's blind list on a costly fork, setup's and
intent's provenance marks in the conversation) are listed at its foot. The lead puts that list
to the user before P2 and P3 open; the user's rulings land in the build log.

### 3.2 Migration 0058 — `skill/analysis-iterative` (`import-document`)

**Conditions.** `contract` — values `on|off`, resolution entry-derived, note: named `on` in the
calling command's own rule text; `off` when the caller names none (default *Slots declared
where*, S5). No other condition: every other slot carries one engine default (V1).

**Rules, by section** (`class: must` unless marked):

| Section | Proposed id | Carries |
|---|---|---|
| independence | `lead-holds-conversation` | default *Who holds the conversation* — the lead runs the stage inline in the main pane; no teammate holds a conversation with the user; rulings are transcribed (no sound loop) |
| independence | `blind-second-list` (`kind: duty`) | D1 + V1 default — on a fork costly to undo, a fresh seat, never a review seat, gets the caller's opening and the question, never your options, the conversation or the ruled set; the user sees where the lists differ before ruling; a caller's own rules may set its cadence and inputs |
| scope | `engine-precedence` | default *Precedence* — the engine's asking rules hold in every caller; a caller adds its domain part and fills slots, never loosens a mechanic; a needed difference becomes a slot by ruling |
| scope | `caller-slots` (`kind: binding`) | D1 + D4 + V1 — the slots a caller's own rules fill (opening · unit · where the set is saved · who writes after · the blind list's cadence, inputs and job · the independent leg · its gate and fail); an unfilled slot takes the engine default; no per-caller carve-out |
| inputs | `fact-before-fork` | D1 — the fact a fork hangs on in hand first, source attached; a fact only the user holds asked as a labelled fact question; one not found stated unknown and carried open |
| inputs | `inputs-in-hand` | D1 — inputs already in hand open the stage as facts, provenance `handed in`, never re-asked; a skip skips the forks, never the close |
| verdict | `one-fork-per-turn` | D1 + default *What analysis-iterative keeps* — one fork per turn; the defaults batch on one screen is the one stated exception |
| verdict | `options-rule` | D1 — two real options, one the do-less shape; the rejected road named; the case against and a one-line pick |
| verdict | `question-form` | D1 + S3 — plain words, decisions by name, one sentence and one concrete example per option, one screen; an open probe to an unsure user stays allowed |
| verdict | `mock-in-message` (`kind: duty`) | D1 — a rough mock inside the message for a question about how something looks or behaves; heavier artifacts on request only |
| verdict | `own-turn-or-batch` | D1 — own turn for a real or costly choice; the rest named in one line as they arise, confirmed together on one screen, never by silence; a default others hang on confirmed first |
| verdict | `status-line` | D1 — each turn one status line over the candidates: ruled · askable · in fog |
| verdict | `ratified-streak` | D1 — a yes to your pick stands as ratified; after three in a row, say so and change the next fork's form |
| verdict | `vague-answer` | D1 — "whatever you think" re-asked as two concrete options; a convention answer draws one probe |
| verdict | `stop-rule` (`kind: bound`) | D1 — closed only when every candidate is ruled in or out and nothing is open; the three-turn stall clause, stepping back to the caller's opening |
| output | `provenance-marks` | D1 + default *One provenance vocabulary* + V1 — one vocabulary (`user's words` · `lead's guess, ratified` · `blind list` · `handed in`), shown as each item is ruled, saved where the caller writes its output, else kept in the conversation |
| output | `open-unknowns` (`class: floor`) | the shipped prose floor line of `## Output`, carried verbatim in meaning (protected: guardrails-vs-detail, v0.63.0) |
| output | `deliverable` (`kind: binding`) | the caller's named deliverable; the synthesis (`SYNTHESIS.md`) when it names none |
| reserved | `user-rules` (`kind: reservation`) | D1 — every ruling the user's word; no default applied by silence; any default may be pulled to its own turn |
| reserved | `contract-pen` (`when: contract=on`) | D1 contract half — the lead transcribes the ruled set where the caller saves it, then passes the pen to the next writer with the user's words quoted |
| reserved | `contract-inside` (`when: contract=on`; the lead's lean: `class: floor`) | D1 + S2 — the next writer writes only inside the ruled set; anything off it that changes a ruling or that the user would notice goes back to the user before it is written; a caller with a later sign-off may let a small technical extra no ruled item touches wait for it |

Not carried as rules (they stay prose in the body, default *What analysis-iterative keeps*): the
confidence-marks table, the Common Mistakes rows, When NOT to Use, the adaptive examples pointer.
`brainstorm.dependency-order` stays brainstorm's (D1 does not list it; moving it would add an
obligation to setup and intent); the map records it `kept`.

### 3.3 Migration 0059 — `command-common/common`

One block, `common.thinking-stage`, `pointer: mochiko:analysis-iterative` — the shared line each
user command extends (default *Slots declared where*; the transport-floor shape, F20). Its text
says the command's thinking stage runs on the engine, whose asking rules hold here, and that the
command's own rules carry its domain part — referenced, never restated. It names no contract mode
(a stub inherits text only). Each extending stub is `class: floor` locally (D3, S7).

### 3.4 Migration 0060 — `command/brainstorm`

- **Mint** `brainstorm.thinking-stage` in `brainstorm.sec.boundaries` — `class: floor`,
  `extends: common.thinking-stage`. Contract: not named (off).
- **Supersede** (`supersede-rule`, disposition naming the engine id): `brainstorm.own-rules-win`
  (*Supersessions owed*: `brainstorm-target-state` D19-brainstorm-only-reach and the clause) ·
  `brainstorm.fact-before-decision` · `brainstorm.own-turn-or-batch` · `brainstorm.options-rule` ·
  `brainstorm.vague-answer-reasked` · `brainstorm.question-form` · `brainstorm.show-before-asking`.
- **Reword to the domain part** (id survives): `brainstorm.lead-inline-questioning` (the deciding
  runs on the thinking stage) · `brainstorm.ratified-yes` (a ratified pick is recorded in the
  card's how-it-was-decided part, no new confidence mark — the streak is the engine's) ·
  `brainstorm.stop-rule` (the decision map empty, no open frame line, the blind map landed and
  folded, a user stop recorded; the stall steps back to the frame) · `brainstorm.decision-map`
  (the three-part map stays; the per-turn status line is the engine's) ·
  `brainstorm.blind-second-list` (brainstorm's cadence: the engine's list on standard and too-big
  sessions only).
- `brainstorm.dependency-order`, `brainstorm.accept-screens` (its defaults screen is brainstorm's
  one-screen batch), every floor and fail: untouched.

### 3.5 Migration 0061 — `command/specify`

- **Mint** `spec.thinking-stage` in `spec.sec.boundaries` — `class: floor`,
  `extends: common.thinking-stage`.
- **Reword** `spec.story-stage` — the story stage runs on the thinking stage with its ruled-set
  contract `on`: the slate is the ruled set, the requirements-analyst seat the next writer (default
  *Slots declared where*).
- **Supersede** `spec.own-rules-win` (*Supersessions owed*: `specify-brainstorm-discovery`
  D5-specify-own-rules).
- **Reword to the domain part**: `spec.story-forks` (the do-less shape is fewer stories or none for
  that journey; the mock is one row of the story as it would read — the "one-row" word kept; the
  status line runs over candidate journeys) · `spec.story-stage-stop-rule` (the close counts the
  blind list's differences; the stall steps back to the intent synthesis) ·
  `spec.intent-probe-discipline` (intent runs as probes and closes in a one-screen synthesis; the
  streak and vague-answer limbs are the engine's) · `spec.lead-holds-story-stage` ·
  `spec.analyst-brief-and-pen` · `spec.slate-binds-authoring` · `spec.stage-inputs-in-hand` ·
  `spec.slate-provenance` — each keeps only what is specify's (the frame in hand, the slate's home
  in `spec.md`, the brief's contents, the waiver-pending wait, the slate count, the Story-cell mark
  position, `$ARGUMENTS`' record, no size gate), the engine carrying the rest.
- `spec.intent-stage-first` (its pointer), `spec.blind-journey-list` (specify's once-per-run
  cadence and inputs), every floor and fail, `spec.fail.story-outside-slate` and its `enforces:`:
  untouched.

### 3.6 Migration 0062 — `skill/review-specifications` (conditional)

`review-specifications`' not-for rule names "pre-spec enrichment of a sparse idea
(`mochiko:analysis-iterative` …)" (F27). With the enrichment variant stripped, P1 reads whether
the line is now false; if so, one `reword-rule` points it at specify's intent and story stages on
the engine; if not, 0062 is unused and the plan says why.

### 3.7 Decision and default coverage

| Decision / default | Carrier |
|---|---|
| D1-engine-asking-contract | 0058's asking rules · `contract-pen` · `contract-inside` · `caller-slots` · `common.thinking-stage` |
| D2-iterative-becomes-engine | 0058 · the SKILL.md conversion and strips (P2) · `description:` and router rewritten |
| D3-four-move-now | 0060 · 0061 · the obligation map · each stub `class: floor` (desk and implement: W2, W3) |
| D4-shaped-for-six | `caller-slots` naming the slots; setup and intent untouched, reached through their pointers |
| *Who holds the conversation* | `lead-holds-conversation` |
| *Precedence* | `engine-precedence` · the two own-rules-win supersessions |
| *Change control* | §4's unit 5 brief: the grader re-reads every caller's render — brainstorm, specify, setup, specify's intent (and, at W2/W3, the desk and implement) |
| *Slots declared where* | condition `contract` · `common.thinking-stage` · `spec.story-stage` naming `on` · `blind-second-list` default |
| *"Ships once", measured* | §3.0's sweep + every render resolving the pointer (P3 asserts) |
| *Setup and specify's intent follow the asking half* | no edit to either; the map's `gained` rows |
| *What analysis-iterative keeps* | P2's body: confidence marks, Common Mistakes reworded, adaptive examples; `one-fork-per-turn`, `question-form` |
| *One provenance vocabulary* | `provenance-marks` |
| *Supersessions owed* (W1 share) | 0060/0061 supersessions · P2's strips: the v0.25.0 KEPT entries, the v0.63.0 keep-set, `## Two output shapes`, the Common Mistakes and When NOT to Use rows |

### 3.8 Prose (P2)

- **`skills/analysis-iterative/SKILL.md`** — the rendered-skill shape of its siblings
  (`analysis-codebase/SKILL.md`): `allowed-tools: Bash(mochiko-cli *)`, a short Overview naming the
  engine and who extends it, the `## Rules — delivered by mochiko-cli` section with the seven `!`
  lines and the floor read-back sentence, When NOT to Use (the "Implementation details" line
  rewritten — implement's design stage now runs the skill, F21, V5), Common Mistakes (rows reworded
  per the default: "one question per turn — always" reads one fork per turn with the defaults
  batch the stated exception; "always using structured options" keeps an open probe for an unsure
  user), the confidence-marks table, the Reference line. `## Two output shapes, one engine`
  removed. `description:` rewritten (≤1,536 chars, MUST/SHOULD and trigger phrases: the shared
  thinking stage; brainstorm, specify, the architecture desk, implement's design stage, setup's
  interrogation). `SPECIFICATION-INPUT.md` and `ENRICHMENT.md` deleted.
- **Strips** — supersession-by-ruling entries at `[v0.120.0]` for every removal above, with the
  protected-content reconciliation of the two `[v0.25.0] KEPT:` entries and the v0.63.0 keep-set,
  consumers assessed (brainstorm, specify, setup, review-specifications, the router).
- **Router** — lines 66 and 124 rewritten around the shared stage.
- **Commands** — `brainstorm.md`, `specify.md`: only lines this wave makes false.
- **Budgets** — `analysis-iterative`'s payload re-seeded as a skill pair (body + seven rendered
  blocks, characters of the parsed value), the growth over the prose budget argued as the ruled
  D1 obligations, never restored playbook prose; its description row re-measured.
- **`CHANGELOG.md`** — `## [0.120.0]` opened with the W1 part; `[lead: …]` placeholders for the
  gates paragraph and later waves.

### 3.9 Tests and kits (P3)

- Crate frozen-census tests: rule, floor and skill-schema counts read from `migrate status` and
  the renders, never computed by hand; the family assert at `validate.rs:909` stays true. No crate
  source, no crate version bump.
- `evals/contract/` — `expected-skills.json` gains `analysis-iterative`'s floor set (it is now
  schema-bearing); `run.py` `Expected["brainstorm"]` and `Expected["specify"]` gain the stub floors;
  the README's prose-skill list drops `analysis-iterative`; figures only where a frozen count moved.
- `evals/plan/brainstorm/` and `evals/plan/specify/` — `observable.yaml` partitions re-keyed
  (superseded ids out, minted ids in, each with its why); the kits' static checks pass; no grid.

## 4. Audit (leg 2)

One gate grader for the wave: a plain `general-purpose` seat, `model: opus` explicit, its brief
the verbatim `mochiko-cli rules validation-primitive-edit` render (every block), the units, the
paths, the pre-pass commands and any named overage. Units, one verdict block and one outcome line
each, blocks in `reports/audit.md`:

1. schema content — 0058 and the new skill view
2. schema content — 0059 and the common view diff
3. schema content — 0060 and the brainstorm view diff
4. schema content — 0061 (and 0062) and their view diffs
5. the `analysis-iterative` skill pair — with the obligation map read for preserved
   responsibilities, and every caller's render re-read (*Change control*)
6. the `brainstorm` and `specify` command pairs
7. router, strips, ripple, budget ledger, changelog
8. crate tests, contract expectations, the two plan kits, the allowlist rows

On a FAIL the owning seat fixes and the same grader is resumed on the delta; a second FAIL on any
unit halts to the user (`common.gate-loop-bound`). Beside the audit, the lead reads the built
rules against §3.7 row by row; that read clears nothing on its own.

## 5. Wave close

No bump at wave close. The wave closes when §4 is all PASS and the obligation map's list is ruled;
the gates (`migrate validate` · views ≡ replay · `cargo test` + fmt + clippy + audit · the full
similarity sweep · char budgets) run at the wave close as a regression read and again before the
bump, after wave 3, with the contract suite (`sbx login` is the user's; a SKIPPED suite blocks).

## 6. Stop conditions

Plan FAILs from more than half the producers · any second re-plan · any tree dirtied during
planning · a rejecting `migrate validate` finding a seat cannot resolve inside its ownership · any
floor or fail retired or lowered · a grammar or crate-source change found needed · a `lost` row
the user does not rule · a second FAIL on any gate unit. Each halts the wave and goes to the user.
