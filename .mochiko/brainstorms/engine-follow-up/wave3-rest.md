# Wave 3 — the rest of the table, the readers, the residuals, the bump (`engine-follow-up`)

**Opened:** 2026-10-18 · **Lead:** the build session lead · **Branch:** `engine-follow-up-build`
(W1 `fccae34`, W2 `437b654`, both gate audits all PASS) · **Target version:** 0.121.0, the one bump,
at this wave's close · **Record:** `.mochiko/brainstorms/engine-follow-up/record.md` — D4 and D10 as
review-amended, *The re-sort under D10* (its Specify, Setup and Architecture-desk rows), defaults
15, 16, 21, 22 and 23 as changed at review, QR6 (RB13) are the source for every rule text; this plan
names carriers, never restates a decision; where it paraphrases, the card wins.

Wave 3 completes D4's "every place" across the remaining commands: specify, setup and the
architecture desk each extend W1's `common.asking-line` and set each ruling place on its writer
switch, as feature (W1) and implement (W2) did. Specify reads the feature desk's handed-in
`derivation.md` (default 23, the reader's side). The residuals ride where they touch files this
build already touches (default 21), with the two W2 gate advisories. Then the bump, the release
gates and the KM landing. Brainstorm stays out of scope by the frame.

## 0. Sound-loop wiring

The same as W2 (`wave2-implement.md` §0):
- **Producers:** persona-less `general-purpose` seats, `model: opus`, plan-only first, the tree
  diffed.
- **Plan grades:** fresh generic peers (`model: opus`) grade each plan per
  `mochiko:review-seat-plan`, the render handed by file. The verdict line uses the render's
  `PLAN GRADE:` form.
- **Approval:** the lead approves only a PASS, pinned by sha256.
- **Re-plan bound:** one for the wave, shared across producers.
- **Leg 2:** one gate grader (§4).
- **Leg 3:** the user rules any coverage gap, plus the contract suite's `sbx` credentials.
- **Commits:** the lead commits at the wave close and the bump (the user's word, 2026-10-18);
  never push or merge.
- **Practice:**
  - every brief carries the haiku routing line;
  - Write/Edit only;
  - every Python run uses `-B`;
  - every return is read from the seat's own transcript, since the idle notices arrive truncated.

## 1. Sequence allocation

- **`0098-specify-asking-line.yaml`** — `command/specify`: `spec.asking-line`, the ruling places,
  the derivation reader (default 23).
- **`0099-setup-asking-line.yaml`** — `command/setup`: `setup.asking-line`, the ruling places.
- **`0100-desk-asking-line.yaml`** — `command/architecture`: `arch.asking-line`, the ruling places.
- **`0101-implement-residuals.yaml`** — `command/implement`: the two W2 gate advisories (§3.4).
- `0102`–`0103` held for a fix round.
- P1 may merge or split these with the reason.
- Header anchor: `2026-10-15 engine-follow-up`. Rule anchors as W1 and W2.

## 2. Seats and ownership (disjoint)

| Seat | Owns | Depends on |
|---|---|---|
| **P1 schema** | the migration files above · `.mochiko/schema-views/**` (re-emit only) · `scripts/similar-rules-allowlist.yaml` (rows only for edges the sweep reports) | — |
| **P2 prose** | `commands/specify.md`, `commands/setup.md`, `commands/architecture.md`, `commands/implement.md` (lines made false only) · the router · strips · ripple (`README.md`, `ARCHITECTURE.md`, `GLOSSARY.md`) · the default-21 prose residuals named in §3.5 · `.mochiko/memory/primitive-cost-budgets.md` · `CHANGELOG.md` `## [0.121.0]` (the W3 part) | P1 landed |
| **P3 tests and kits** | crate frozen-census tests · `evals/contract/` expectations · `evals/plan/specify/`, `setup/`, `architecture/`, `implement/` (re-keys, plus the default-21 kit residuals in §3.5) · `evals/plan/README.md` | P1 landed |
| **The lead** | the bump (`plugins/mochiko/.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json`), the CHANGELOG's `[lead: …]` slots, the KM landing (`DECISIONS.md`, `BACKLOG.md`, the trail, `ROADMAP.md`, the brainstorms index) — release bookkeeping and operating docs, never a primitive's content | §4 all PASS |

The order:
1. P1 plans, a peer grades, the lead approves, P1 builds, and the lead reads the result against
   §3.6.
2. P2 and P3 then plan, get graded and approved, and build in parallel.
3. The §4 audit.
4. §5, the bump and landing.

## 3. Content

### 3.0 What every rule text must respect

- **The guardrails of W1 and W2 hold.**
  - Guardrails, never a sequence.
  - Short rules.
  - Referenced, never restated: no engine mechanic copied, no engine edit, no new slot
    (default 19).
  - Nothing weakens. Every floor and fail stands, and a `must` leaves only by `supersede-rule`.
  - No grammar or binary change.
- **Mirror the shapes already built.**
  - Each `<cmd>.asking-line` extends `common.asking-line` (`class: must`, in ways-of-working).
  - Each `<cmd>.ruling-places` (`kind: binding`, reserved, anchor D10) names every place by its
    live rule id with its writer setting, as `feat.ruling-places` (0087) and `impl.ruling-places`
    (0093) do.
- **Gates stay gates.** A G row names its place as a yes / no gate, its writer *held*, never asked
  the asking line's way.
- **Defaults 15 and 16 are already carried.** Setup's and specify's intent contract stays off,
  *held* by their own rules (the re-sort's Setup row). No rule turns either contract on.

### 3.1 Specify — the re-sort's Specify row and default 23 (0098)

`spec.asking-line`, and `spec.ruling-places` naming these places:

| Place | Asked | Writer |
|---|---|---|
| framing on an empty argument | an open probe | off |
| clarification answers only the user can settle | a fact question | held — the analyst's pen under the slate |
| filter disagreement, and a rejected `in` row after authoring (`spec.filter-disagreement-escalates`) | the asking line | held — the slate's contract |
| post-stories frame adjust (`spec.confirm-frame-post-stories`) | the asking line | on — the PM's map derivation writes from it, and the map write's grader checks it |
| selection (`spec.gate-selection`) | the asking line | held — the run builds only what is selected |
| acceptance (`spec.gate-acceptance`) | stays a gate | held — it executes the staged map batch as shown |
| prototype clicking | unchanged | a reaction, not a ruling — not listed, or listed as outside the line |

- **Selection moves off "stays a gate".** The re-sort set selection to E · held. P1 checks whether
  `spec.gate-selection`'s own `kind: gate` text makes it read as a yes / no gate, and proposes the
  least change that keeps the selection the user's: a reword, or the ruling-places row alone.
  Anything that lowers a floor goes to the lead.
- **Default 23, as RB13 changed it.** A demand the feature desk routes to `/mochiko:specify` hands in
  the sitting's `derivation.md`, carrying the goal and the user's words so far. Specify reads it as
  inputs in hand, never re-asking (`analysis-iterative.inputs-in-hand`). The carrier is
  `spec.stage-inputs-in-hand` widened (it names a brainstorm record or story-form stories today).
  Its writer side is W1's `feat.carry-forward`.

### 3.2 Setup — the re-sort's Setup row (0099)

`setup.asking-line`, and `setup.ruling-places` naming these places:

| Place | Asked | Writer |
|---|---|---|
| mode when ambiguous (`setup.user-mode-ruling`) | the asking line | off |
| reality-vs-intent conflicts (`setup.user-conflict-rulings`) | the asking line | held — the trace from ratified intent to surfaces |
| coverage-survivor routing (`setup.coverage-survivor-routing`) | the asking line | off |
| a second FAIL at the gate loop (`setup.gate-loop-bound`) | the asking line | off |
| synthesis ratification (`setup.gate-synthesis-ratification`) | stays a gate | held |
| final acceptance (`setup.gate-final-acceptance`) | stays a gate | held |

Setup's interrogation runs on the thinking stage through its existing pointer and W1's
`setup.thinking-stage`, with the contract off (defaults 15 and 17). The interrogation cards are
outside this list.

### 3.3 The architecture desk — the re-sort's Architecture-desk row (0100)

`arch.asking-line`, and `arch.ruling-places` naming these places:

| Place | Asked | Writer |
|---|---|---|
| drift dispositions (`arch.user-reserved-rulings`) | the asking line | on — the rows written from them, tech-lead's sameness grade checking |
| shelf-scope override | the asking line | on |
| a trigger ruled closed (`arch.dm-route-triggers`) | the asking line | off |
| a governance-ledger waiver for a floor drop | stays a gate | held — the ledger records it |

The desk's row stances, baseline confirmation and amendments stay in its focus stage, as built,
and are not listed.

### 3.4 Implement — the W2 gate advisories (0101)

- **`impl.feature-visit-input` reads "any" feature visit file keyed to the FEAT,** though dated
  visits accumulate. Narrow it to the visit file the run's own entry points at: the selected work
  rows' marked pointer, or the delta card's, as 0095 does for the sufficiency read. On an epic,
  each member's. Keep "in every scope, whatever the gap count".
- **`impl.landing-verifier-folds` does not name the `gates.md` fold check** that
  `impl.ruling-places` gives it (D10: "the landing verifier checks the `gates.md` fold"). P1 picks
  one home: a reword of `impl.landing-verifier-folds` naming the check (and `impl.ruling-places`
  then pointing at it), or leaving `impl.ruling-places` as the single home with the reason.
- Both reword rules this build minted or W2 touched. Each sits in a section measured against the
  tools line if it is `impl.sec.tools` (28,443 bytes today).

### 3.5 Default 21 — the residuals that touch this build's files (P2, P3)

These come from `BACKLOG.md` *Engine-desk build — lines incomplete, not false* and *Engine-desk
build — tool and kit residuals*. The rule (default 21): fix only where the file is already touched
by this build; everything else stays in BACKLOG, the item narrowed.
- **P2:**
  - the router's desk row (`:90`, `:95`, `:98`, `:162`, the `<FEAT-ID>` delta visit);
  - `ARCHITECTURE.md:277`, `:285`, `:388`, and the false `:301` (build-time D-/C-/IP- writes "land
    as `baseline-delta.md` entries");
  - `devils-advocate.md:20` and `templates/report-format.md:7`, only if P2 reads them as files this
    build touches.
  - P2 names each residual it takes and each it leaves, with the reason.
- **P3:**
  - the bare `D3` lines this build's predecessor added to `crates/mochiko-cli/tests/fidelity.rs:829`
    and the architecture, brainstorm and specify plan kits — joined "at the next touch" (W3 touches
    `fidelity.rs` and the architecture and specify kits; brainstorm only if touched);
  - the 18 implement-kit whys cut short by YAML at a comma (W2 touched that kit).
- **Out of this build** (crate source, or not touched): the `home` "section budgets" print, the
  Mermaid parse check, the unchecked `pointer:`, the `ADAPTIVE-EXAMPLES.md` refresh,
  `patterns-system-design`'s description and overview, `review-plan-artifacts`' description,
  `ARTIFACT-CHECKLISTS.md:91`, `FEASIBILITY-LENS.md:99–101`, and the architecture kit's g3 watch.

### 3.6 Decision and default coverage

| Decision / default | Carrier |
|---|---|
| D4 / D10 (W3 share) | `spec.`, `setup.` and `arch.asking-line` · `spec.`, `setup.` and `arch.ruling-places` — the 39-row re-sort complete across the three waves |
| Defaults 15, 16 | the *held* settings in `setup.ruling-places` and specify's; no contract turned on |
| Default 21 | §3.5's taken residuals; the BACKLOG items narrowed at landing |
| Default 22 | the count reported at the bump (CHANGELOG): no moment added to implement beyond the card-order question on a real choice; a feature card visit gains at most one confirm screen |
| Default 23 (reader) | `spec.stage-inputs-in-hand` widened |
| W2 gate advisories | 0101 |

## 4. Audit (leg 2)

One gate grader: a plain `general-purpose` seat, `model: opus` explicit, its contract rendered
first-hand. It writes `reports/audit-w3.md`, one verdict block and one outcome line per unit.
1. schema content — 0098 and the specify view diff, every place checked live
2. schema content — 0099 and the setup view diff
3. schema content — 0100 and the architecture view diff
4. schema content — 0101 and the implement view diff (the tools line measured)
5. the `specify`, `setup` and `architecture` command pairs, plus the `implement` pair's 0101 delta
   — each `.md` with its render, the asking line's cited ids live
6. router, strips, ripple, the default-21 prose residuals, the budget ledger, changelog
7. crate tests, contract expectations, the four plan kits, the default-21 kit residuals

A FAIL goes back to its owning seat, and the same grader is resumed on the fix. A second FAIL halts
to the user (`common.gate-loop-bound`). The lead also reads the result against §3.6 row by row;
that read clears nothing.

## 5. Wave close, the bump, the landing

1. **The release gates**, first-hand by the lead:
   - `migrate validate` 0 rejecting;
   - views ≡ replay;
   - `cargo test --all`, fmt, clippy and `cargo audit`;
   - the full similarity sweep;
   - the char budgets as ruled.
2. **The bump**, release bookkeeping by the lead:
   - `plugin.json` to 0.121.0, with `.claude-plugin/marketplace.json` synced — read
     `.claude/rules/mochiko/` for both manifests first;
   - the CHANGELOG's `[lead: …]` slots filled from the three audits: the date, the title, the final
     log figures, the gates paragraph, default 22's count, and the AM-5 paragraph if it still
     applies. Fix the unit-number slip in the testing-gap-finding verdict slot: unit 4 of W2, not
     unit 6.
3. **The contract suite** against the built binary (`evals/contract/`):
   - rebuild `target/release/mochiko-cli` first;
   - the sandbox cases need the user's `sbx login` and `sbx secret set anthropic`;
   - SKIPPED is not green and blocks the bump.
4. **The KM landing** (`.mochiko/memory/knowledge-management.md`):
   - the `DECISIONS.md` row's status to landed;
   - `BACKLOG.md` *Engine follow-up build* moved to the trail;
   - the two default-21 items narrowed or closed;
   - a new item for the improvement-card criteria gap (W2 N1);
   - the `ROADMAP.md` Plan-surface row touched;
   - the brainstorms index entry updated;
   - statuses agreeing across all of these.
5. **Commit.** Never push or merge.

## 6. Stop conditions

Each of these halts the wave and goes to the user:
- plan FAILs from more than half the producers;
- any second re-plan;
- any tree dirtied during planning;
- a rejecting `migrate validate` finding a seat cannot resolve inside its ownership;
- any floor or fail retired or lowered;
- an engine rule edit, a new engine slot, or a grammar or crate-source change found needed;
- `impl.sec.tools` past its 29,000-byte line;
- a ruled text that will not fit without changing its meaning;
- a second FAIL on any gate unit;
- the contract suite SKIPPED or red.
