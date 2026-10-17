# Wave 1 — the shared asking line, feature on the engine, setup's pointer (`engine-follow-up`)

**Opened:** 2026-10-16 · **Lead:** the build session lead · **Branch:** `engine-follow-up-build`
(from `main` @ `cbe3f5f`) · **Target version:** 0.121.0, one bump after wave 3 (the user's ruling,
build log 2026-10-15) · **Record:** `.mochiko/brainstorms/engine-follow-up/record.md` — D2, D3, D7
and D10 as review-amended, the *Defaults batch* (defaults 6–14, 17, 18, 19, 20, 23, 24 as changed
at review), *The re-sort under D10* (its Feature row) and QR3, QR5, QR6 are the source for every
rule text. This plan names carriers, never restates a decision; where it paraphrases, the card
wins.

Wave 1 mints the shared asking line every command extends for its questions outside a thinking
stage (D10, QR3; default 18), moves `/mochiko:feature` onto the engine — its card visits run the
full stage with the contract on (D2, D3), its ruled set and card live in a typed visit file at the
feature desk (D7), its other rulings take the asking line (the re-sort's Feature row) — and gives
setup its thinking-stage floor pointer (default 17). Implement's side (W2) and the specify, setup
and architecture-desk rows (W3) extend what this wave mints.

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
- **Leg 3.** The user gates any coverage gap §3.8 turns up, the commit and the merge. The lead
  never commits unasked, never pushes.
- **Transport.** Seats are subagents with disjoint ownership (§2); no cross-seat messaging; every
  hand-off is lead-relayed, each order quoting the text it lands. Every brief carries the routing
  line: locate and enumerate reads go to a native `Explore` subagent spawned `model: haiku`;
  interpretive reads stay on the seat. Artifacts are written with Write/Edit, never a shell copy
  (the artifact hook refuses it — the session's own lesson).

## 1. Sequence allocation

- **`0085-asking-line.yaml`** — `command-common/common` (`mint-rule`, one block).
- **`0086-feature-visit-home.yaml`** — `template/feature-visit` (`import-document`) and the
  `feature-desk` home (`replace-document`: the title, the `visit.md` deliverable).
- **`0087-feature-on-engine.yaml`** — `command/feature`.
- **`0088-setup-thinking-stage.yaml`** — `command/setup` (one `mint-rule`).
- **`0089-entry-points-at-visit.yaml`** — `skill/authoring-feature-map`, only if §3.4's check finds
  the delta grammar has no place for the pointer to the visit file; otherwise unused.
- `0090`–`0091` held for a fix round; a gap is legal if unused. W2 allocates from `0092`.

Header anchor on all: `2026-10-15 engine-follow-up`. A rule carrying one ruled decision takes it as
its rule-level anchor (`… D7`); a confirmed default takes the bare session anchor. A reword or
supersession of a rule traceable to another session's ruling carries this session's anchor and
names the earlier ruling in its `disposition` (default 20).

## 2. Seats and ownership (disjoint)

| Seat | Owns | Depends on |
|---|---|---|
| **P1 schema** | the migration files above · `.mochiko/schema-views/**` (re-emit only) · `scripts/similar-rules-allowlist.yaml` (rows only for edges the sweep reports, or rows naming an id this wave retires) | — |
| **P2 prose** | `commands/feature.md` and `commands/setup.md` (lines made false only, plus default 14's reword) · `skills/analysis-iterative/SKILL.md` (`description:` and Overview only — no rule, no body mechanic) · `skills/authoring-feature-map/SKILL.md` (lines made false only) · the router `skills/mochiko/SKILL.md` · strip files (`.mochiko/strips/feature.md` · `setup.md` · `analysis-iterative.md` · the router's · any other a strip lands in) · ripple lines made false (`README.md`; `ARCHITECTURE.md` only if it is still hand-maintained legacy — `.claude/rules/mochiko/operating-docs.md`) · `.mochiko/memory/primitive-cost-budgets.md` · `CHANGELOG.md` `## [0.121.0]` (opened with the W1 part; later waves append) | P1 landed |
| **P3 tests and kits** | crate frozen-census tests under `crates/mochiko-cli/tests/` (no crate source, no crate version bump) · `crates/mochiko-cli/tests/fixtures/template/feature-visit.check.txt` and `.producer.txt` · `evals/contract/` expectations (`expected-skills.json`, `run.py` `Expected[...]`, README figures) · `evals/plan/feature/` and `evals/plan/setup/` (`observable.yaml` partitions re-keyed, every live schema rule id in exactly one list) | P1 landed |

Order: P1 plan → peer grade → approve → build → the lead reads the built rules against §3.8 ·
then P2 and P3 in parallel, each plan → peer grade → approve → build · §4 audit.

## 3. Content

### 3.0 What every rule text must respect

- **Guardrails, never a sequence.** A rule says what must be true; moments anchor, never order.
- **Short.** One or two sentences per minted rule, plain words. P1 reports each render's size
  before and after; `feat.sec.tools` (5,264 bytes today) stays well under the ~30k inline ceiling.
- **Referenced, never restated.** No asking mechanic of `mochiko:analysis-iterative` is copied
  into feature's or setup's text; feature carries only its domain part and the slots it fills
  (`analysis-iterative.caller-slots`). No engine rule changes and no engine slot is added
  (default 19); a needed one goes back to the user as a frame change.
- **Nothing weakens.** Every floor and fail of `feature` and `setup` stands. A `must` rule leaves
  only by `supersede-rule`, its disposition naming its new carrier.
- **No grammar or binary change.** If one proves needed, it goes back to the user.
- **The desk's shape is the model.** Where feature mirrors the architecture desk (`visit_kind`,
  the stage moment, the visit file, the blind-list skip, the fail — F22, defaults 8–10), P1 reads
  `0065-desk-visit-home.yaml`, `0068-desk-focus-stage.yaml` and `0069-desk-on-engine.yaml` and
  keeps the two desks' wording parallel where the meaning is the same.
- The lead's ids below are proposals; P1 may merge or split with the reason; the approved plan
  fixes the set; every row of §3.8 keeps a carrier.

### 3.1 Migration 0085 — `command-common/common`

One block, proposed `common.asking-line`, `pointer: mochiko:analysis-iterative`, the shared line
each user command extends for the questions it puts outside a thinking stage (D10 as changed at
review, QR3; default 18). Its text names the subset that holds for those questions —
`analysis-iterative.one-fork-per-turn`, `.options-rule`, `.question-form`, `.mock-in-message`,
`.own-turn-or-batch`, `.ratified-streak`, `.vague-answer`, `.fact-before-fork`, `.user-rules` —
and, where the command's own rule sets a ruling's next writer *on*, `.contract-pen` and
`.contract-inside`; it says the blind list, the status line, the stop rule and provenance marks
stay the stage's, and that a yes / no gate is not asked this way. Class: the record rules none;
lead's lean `must` (a floor would add an obligation the user did not rule). It names no command
and no place; each command's own rules set each place's switches. Only `feature` extends it this
wave; the other commands extend it in W2 and W3.

### 3.2 Migration 0086 — the feature visit file's template and home (D7; defaults 11, 13)

- **`import-document` `template/feature-visit`** — the sibling of `architecture-visit`. Sections
  (P1 proposes the exact set and order): a header — the visit goal and done condition, the FEAT id
  or `lane-<slug>`, the visit kind; the ruled set — per card, the user's change in their own words
  and the "I'd know it works if…" line or `open`, each with its provenance mark, and the defaults
  as confirmed; the blind list (when it ran); the card itself, in its own part below the set, in
  the tasks template's card shape (`mochiko-cli template tasks`) — never a second card grammar;
  the verdict.
- **`replace-document` `home/feature-desk`** — title corrected from "One specify-desk sitting"
  (default 13; P1 proposes the words); `visit.md` added as a deliverable bound to `feature-visit`
  under a whole-file bound (the architecture desk's 300 the lean, P1 argues any other);
  `derivation.md` and `reports/` unchanged. The folder slug carries the lower-cased FEAT id or
  `lane-<slug>` (D7); a visit that cuts cards on two capabilities writes one visit file per
  capability (default 11) — P1 reads whether the home's `<date-slug>` grammar holds two folders
  for one sitting or needs a rule, and says which.

### 3.3 Migration 0087 — `command/feature`

- **Condition** `visit_kind` — values `card|ruling|other`, entry-derived, fixed by the converged
  visit goal (default 9): `card` — the visit cuts a card (a bug, an improvement, a growth row, a
  lane card); `ruling` — grooming (merge, retire, status) or an epic ruling (mint, membership,
  close); `other` — a map query, a route to `/mochiko:specify`, a route override, an architecture
  trigger's routing. Re-derived when the goal is re-converged, so the stage runs before any card is
  written (the desk's note, mirrored).
- **Moment** for the card set (the desk's `focus`, mirrored): after the goal is agreed, before the
  card's writer writes anything.
- **Mint, `feat.sec.boundaries`:** `feat.thinking-stage` — `class: floor`, `extends:
  common.thinking-stage` (default 8). `feat.asking-line` — `extends: common.asking-line`, class as
  §3.1.
- **Mint, the stage (`when: visit_kind=card`):**
  - `feat.card-stage` (`kind: duty`) — D2 with QR5: the card set is worked on the thinking stage
    with the ruled-set contract *on*; the visit goal is the opening, the set saved in the visit
    file, the card's writer the next writer; a single-card clear ask whose set the demand already
    states closes with the goal nod; the set takes its own screen whenever there is more than one
    card or any fork was put; the set's close sits inside the visit's done condition.
  - `feat.card-unit` (`kind: binding`) — D3: per card, the change in the user's words and one
    "I'd know it works if…" line or `open`; a bug's reproduction a fact in hand, never re-asked;
    the card's writer writes the 1–3 acceptance criteria or the reproduction-failing test inside
    it; a target number only from the user; an `open` line stays open, never filled in.
  - `feat.visit-file` (`kind: binding`) — D7: one `visit.md` per capability in the feature-desk
    home to the shape `mochiko-cli template feature-visit`; you hold its pen and transcribe the
    set, then pass the pen to the card's writer for the card part; the capability entry's marked
    delta points at the file.
  - The card's writer and its meaning grade — defaults 6 and 7 (RB12, RB2): the writer follows the
    desk's wiring — desk-authored by default (`patterns-sound-loop.default-seat-wiring`), a
    producing seat where the desk staffs one (`feat.author-grader`), the user's words quoted in its
    brief; the card's existing review leg, `devils-advocate` by the sound loop's default wiring,
    grades the card against the visit file for sameness of meaning; no new skill. P1 picks the
    carrier — a new rule, or rewords of `feat.author-grader` and `feat.delta-cards`.
  - `feat.card-blind-list` (`kind: duty`) — default 10: feature's cadence for the engine's blind
    list — once per visit that cuts more than one card or a growth row, skipped on a single bug or
    improvement card; its seat gets the visit goal as the opening (the engine's default inputs
    otherwise).
- **Mint, the fail:** `feat.fail.card-outside-set` (`kind: fail`, `class: floor`) — default 8: a
  card written with no ruled set behind it, or one that changes a ruling without the user's word;
  `enforces:` the stage and visit-file rules (P1 checks the enforces grammar against
  `arch.fail.write-outside-focus`). The feature `.md`'s Not-done line follows the new count.
- **The re-sort's Feature row — the other rulings** (the asking line holds for each): grooming —
  merge, retire, status — and epic mint, membership and close: writer *on* (the map write keeps
  the ruling; its existing non-author grade checks the meaning) · route overrides and an
  architecture trigger at the growth door: writer *off* · selection: writer *held* (the run builds
  only what is selected). P1 picks the carrier: the lean is one `kind: binding` rule beside
  `feat.user-reserved`, which keeps its reservation unchanged.
- **Carry-forward, the desk's side** — default 23 (RB13): a demand the desk routes to
  `/mochiko:specify` writes no visit file and hands in the sitting's existing `derivation.md`,
  carrying the goal and the user's words so far (specify's reading lands in W3) · default 24: an
  outside need `/mochiko:implement` sends to the desk arrives with the run's note, handed in to
  the visit and read as inputs in hand (`analysis-iterative.inputs-in-hand`); the stage asks only
  what the note leaves open (implement's sending lands in W2).
- **Reword (ids survive):** `feat.artifact-home` (the desk's output adds `visit.md`) ·
  `feat.delta-cards` (the card lives in the visit file's card part; its shape, the product-baseline
  touch and the review leg unchanged) · `feat.stable-ground-triage` ("card on the entry" reads as
  the entry pointing at the card in the visit file — D7's changed-at-review line) ·
  `feat.dispatch-scope-split` only if its "desk-confirmed card is implement's delta entry" is made
  false (implement's own entry rule moves in W2) · `feat.dispatch-specify` only if default 23 lands
  there. Each reword's disposition names the earlier ruling where the rule traces to one.
- Every other floor and fail, `feat.user-reserved`'s reservation, the grooming-door ceiling and
  the growth-door routing: untouched.

### 3.4 Migration 0089 — `skill/authoring-feature-map` (conditional)

D7 says the capability entry carries the marked delta, pointing at the visit file. P1 reads
`authoring-feature-map`'s delta grammar and the `feature-entry` template: if a marked delta can
already carry the pointer, 0089 is unused and the plan quotes the line that allows it; if not, one
`reword-rule` (or one template `replace-document`) adds it, nothing more.

### 3.5 Migration 0088 — `command/setup`

One `mint-rule`, `setup.thinking-stage`, in `setup.sec.boundaries` — `class: floor`, `extends:
common.thinking-stage` (default 17). Contract: not named (off; defaults 15, 16 read as *held* at
the re-sort). `setup.interrogation-inline` keeps its pointer; no other setup rule changes this wave
(setup's asking rows are W3).

### 3.6 Prose (P2)

- **`commands/feature.md`** — the "never imposes brainstorm ceremony on a clear ask" line kept,
  reworded to point at the stage's one-confirm close (default 14); the Entry or visit steps name
  the visit file where today's text says the card sits on the entry; the Not-done line's count;
  nothing else.
- **`commands/setup.md`** — only lines this wave makes false (expected none).
- **`skills/analysis-iterative/SKILL.md`** — `description:` (≤1,536 chars, measured) and Overview
  add `/mochiko:feature`'s card visits to the callers (D2), and note that commands opt their other
  questions into the named subset by the shared asking line (QR3); no rule text, no body mechanic,
  no slot.
- **`skills/authoring-feature-map/SKILL.md`** — lines made false only, if 0089 lands.
- **Router** — rows naming feature or the engine's callers, where made false.
- **Strips** — one entry per removal or supersession this wave makes in shipped prose (the
  feature.md ceremony line's wording, any made-false line), each version-stamped `[v0.121.0]`, a
  protected line leaving only by recorded supersession; consumers assessed.
- **Budgets** — `feature`'s and `setup`'s payload rows re-measured (characters of the parsed
  value); `analysis-iterative`'s description row re-measured; any growth argued as the ruled
  D2/D3/D7/D10 obligations.
- **`CHANGELOG.md`** — `## [0.121.0]` opened with the W1 part; `[lead: …]` placeholders for the
  gates paragraph and the later waves.

### 3.7 Tests and kits (P3)

- Crate frozen-census tests: rule, floor, fail, condition, template and home counts read from
  `migrate status` and the renders, never computed by hand; the `feature-visit` template joins
  the fixture set (`.check.txt` and `.producer.txt`, the `architecture-visit` pair the model) and
  any test enumerating templates or homes. No crate source, no crate version bump.
- `evals/contract/` — `run.py` `Expected["feature"]` and `Expected["setup"]` gain the new floors
  and fail; `expected-skills.json` only if a skill's floor set moved; README figures only where a
  frozen count moved.
- `evals/plan/feature/` and `evals/plan/setup/` — `observable.yaml` partitions re-keyed (minted ids
  in, each with its why; reworded ids kept); the kits' static checks pass; no grid.

### 3.8 Decision and default coverage

| Decision / default | Carrier |
|---|---|
| D2-card-visits-stage (with QR5) | `visit_kind` · the stage moment · `feat.card-stage` · `feat.thinking-stage` · the Feature row's carrier · `feature.md` |
| D3-card-unit-reused | `feat.card-unit` |
| D7-desk-visit-file (with QR4) | 0086 · `feat.visit-file` · the writer and meaning-grade carrier · `feat.delta-cards` / `feat.stable-ground-triage` rewords · 0089 if needed |
| D10-two-switch-test (W1 share) | `common.asking-line` · `feat.asking-line` · the Feature row's switches |
| Defaults 6, 7 (RB12, RB2) | the writer and meaning-grade carrier |
| Default 8 | `feat.thinking-stage` · `feat.fail.card-outside-set` |
| Defaults 9, 10, 11 | `visit_kind` · `feat.card-blind-list` · 0086's per-capability file |
| Defaults 13, 14 | 0086's title · `feature.md`'s reworded line |
| Default 17 | `setup.thinking-stage` |
| Defaults 18, 19 | `common.asking-line`, one pointer line; no engine edit |
| Default 20 (W1 share) | dispositions on 0087's rewords · P2's strips |
| Defaults 23, 24 (desk side) | the carry-forward carrier in 0087 |

Not this wave: D1, D5, D6, D8 and defaults 1–5, 12 (W2, implement) · defaults 15, 16 and the
specify, setup and architecture-desk rows of the re-sort, specify's reader for default 23 (W3) ·
default 21's residuals (W3) · default 22 is a count, reported at the bump.

## 4. Audit (leg 2)

One gate grader for the wave: a plain `general-purpose` seat, `model: opus` explicit, its brief
the verbatim `mochiko-cli rules validation-primitive-edit` render (every block), the units, the
paths, the pre-pass commands (`mochiko-cli migrate validate --report --plugin-root
plugins/mochiko`) and any named overage. Units, one verdict block and one outcome line each, blocks
in `reports/audit.md`:

1. schema content — 0085 and the common view diff, **with every engine id it cites checked live
   against the `analysis-iterative` render** (D10's accepted risk: nothing else checks a cited id)
2. schema content — 0086, the new template, the home diff, the `mochiko-cli home` and `template`
   renders
3. schema content — 0087 and the feature view diff (0089 with it, if it lands)
4. schema content — 0088 and the setup view diff
5. the `feature` command pair — every caller slot of `analysis-iterative.caller-slots` filled or
   defaulted, read against the engine render
6. the `setup` command pair and the `analysis-iterative` skill pair (description and Overview only;
   every caller's render re-read for a line now false)
7. router, strips, ripple, budget ledger, changelog
8. crate tests, template fixtures, contract expectations, the two plan kits, the allowlist rows

On a FAIL the owning seat fixes and the same grader is resumed on the delta; a second FAIL on any
unit halts to the user (`common.gate-loop-bound`). Beside the audit, the lead reads the built
rules against §3.8 row by row; that read clears nothing on its own.

## 5. Wave close

No bump at wave close. The wave closes when §4 is all PASS; the gates (`migrate validate` · views ≡
replay · `cargo test` + fmt + clippy + audit · the full similarity sweep · char budgets) run at the
wave close as a regression read and again before the bump, after wave 3, with the contract suite
(`sbx login` and `sbx secret set anthropic` are the user's; a SKIPPED suite blocks).

## 6. Stop conditions

Plan FAILs from more than half the producers · any second re-plan · any tree dirtied during
planning · a rejecting `migrate validate` finding a seat cannot resolve inside its ownership · any
floor or fail retired or lowered · an engine rule edit, a new engine slot, or a grammar or
crate-source change found needed · a ruled text that will not fit without changing its meaning ·
a second FAIL on any gate unit. Each halts the wave and goes to the user.
