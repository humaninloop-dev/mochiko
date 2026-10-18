# Wave 2 — implement's later moments (`engine-follow-up`)

**Opened:** 2026-10-18 · **Lead:** the build session lead · **Branch:** `engine-follow-up-build`
(W1 closed in the tree, all eight gate units PASS, uncommitted) · **Target version:** 0.121.0, one
bump after wave 3 · **Record:** `.mochiko/brainstorms/engine-follow-up/record.md` — D1 as amended
(by D6 and D10), D5, D6, D8 and D10 as review-amended, the *Defaults batch* (defaults 1–5, 12 and
24 as changed at review), *The re-sort under D10* (its Implement row) and QR2, QR6, QR7 are the
source for every rule text. This plan names carriers, never restates a decision; where it
paraphrases, the card wins.

Wave 2 moves implement's moments after the design checkpoint onto the shared asking line W1
minted (`common.asking-line`): the card order asked before slicing on a real choice (D5), every
other ruling place set on D10's two switches (the re-sort's Implement row), acceptance shown as a
promises-kept table (D6), the gap-finder reading the user's own lines (D8), and implement reading
the feature desk's visit file at entry in every scope (default 12) and handing an outside need to
the desk with its note (default 24). Implement's design stage, design checkpoint, agenda split and
seat additions stay as built (out of the table by the record).

## 0. Sound-loop wiring

As W1 (`wave1-feature.md` §0), with these settings:
- **Producers.** Persona-less `general-purpose` seats, `model: opus`, plan-only first, the tree
  diffed before and after.
- **Plan grades.** A fresh generic peer (`model: opus`) grades each plan per
  `mochiko:review-seat-plan`, the render handed by file, verbatim. The grader's verdict line is
  the render's own `PLAN GRADE:` form.
- **Approval and resume.** The lead approves only a PASS and resumes the seat with the approved
  plan pinned by sha256.
- **Re-plan bound.** One for the wave, on a counter the producers share; a second consumption goes
  to the user.
- **Leg 2.** §4, one gate grader.
- **Leg 3.** The user gates any coverage gap §3.7 turns up, the commit and the merge.
- **Transport.** Subagents with disjoint ownership; lead-relayed hand-offs, content-pinned. The
  haiku routing line is in every brief.
- **Writes.** Write/Edit only. Every Python run uses `-B` or `PYTHONDONTWRITEBYTECODE=1`.
- **Truncation.** Every idle notice this build has arrived truncated. The lead reads each return
  from the seat's own transcript, unchanged, and grades from that copy.

## 1. Sequence allocation

- **`0092-implement-card-order.yaml`** — `command/implement`: the card order (D5), the card review's
  order and seed checks (default 5), the order fail.
- **`0093-implement-asking-line.yaml`** — `command/implement`: `impl.asking-line`, the ruling places
  and their switches (the re-sort's Implement row), the checkpoint and dispositions forms (defaults
  1–3), the promises-kept table (D6).
- **`0094-implement-reads-desk.yaml`** — `command/implement`: the feature visit file read at entry in
  every scope (default 12), the design and design-stage inputs widened, the outside need handed in
  with the run's note (default 24), the gap-finder's added input (D8).
- **`0095-sufficiency-reads-visit.yaml`** — `skill/review-sufficiency`, only if its "the desk's delta
  card is itself the `[MODIFY]` instrument, carrying the marked delta on the entry" (its view, line
  40) is made false by the card's move into the visit file; otherwise unused.
- `0096`–`0097` held for a fix round. W3 allocates from `0098`.

P1 may merge or split these files with the reason. Header anchor on all: `2026-10-15
engine-follow-up`. Rule anchors as W1: a rule carrying one ruled decision takes it (`… D5`); a
confirmed default takes the bare session anchor; reworded rules keep their own anchors, each earlier
ruling named in the file's `intent` (`reword-rule` carries no disposition field — W1 P1's finding,
`migration.rs:769`).

## 2. Seats and ownership (disjoint)

| Seat | Owns | Depends on |
|---|---|---|
| **P1 schema** | the migration files above · `.mochiko/schema-views/**` (re-emit only) · `scripts/similar-rules-allowlist.yaml` (rows only for edges the sweep reports) | — |
| **P2 prose** | `commands/implement.md` (lines made false only) · `skills/review-sufficiency/SKILL.md` (lines made false only, if 0095 lands) · the router · strip entries (`.mochiko/strips/implement.md`, `review-sufficiency.md`, any other a strip lands in) · ripple lines made false (`README.md`, `ARCHITECTURE.md`, `GLOSSARY.md` — including W1's carried note, §3.6) · W1-touched prose a W2 rule makes false (`feature.md`, the `feature-visit` template's prose is P1's) · `.mochiko/memory/primitive-cost-budgets.md` · `CHANGELOG.md` `## [0.121.0]` (the W2 part, replacing its `[lead: Wave 2 …]` placeholder) | P1 landed |
| **P3 tests and kits** | crate frozen-census tests under `crates/mochiko-cli/tests/` (no crate source, no crate version bump) · `evals/contract/` expectations · `evals/plan/implement/` (and `evals/plan/README.md`'s implement figure) · any fixture a changed template needs | P1 landed |

Order: P1 plan, then peer grade, approve, build, and the lead's §3.7 read. Then P2 and P3 in
parallel, each plan, peer grade, approve, build. Then §4.

## 3. Content

### 3.0 What every rule text must respect

- **W1's guardrails hold** (`wave1-feature.md` §3.0):
  - guardrails, never a sequence;
  - short rules;
  - referenced, never restated — no engine mechanic copied, no engine edit, no new slot;
  - nothing weakens — every floor and fail of implement stands, and a `must` leaves only by
    `supersede-rule`;
  - no grammar or binary change.
- **The tools ceiling.** `impl.sec.tools` renders 27,651 bytes against the 29,000-byte line under
  the 30,000-character `!` ceiling (the 0.120.0 W3 gate's measure). Net growth there must stay
  under the line.
  - New rules go to `impl.sec.reserved`, `impl.sec.ways-of-working` or `impl.sec.roles` wherever
    their kind allows.
  - A reword in tools is measured before and after.
  - If the wave cannot fit, P1 says so in the plan, and the lead raises it to the user. Never
    trimmed by silence.
- **Implement mirrors feature where the meaning is the same.** `impl.asking-line` takes the shape
  of `feat.asking-line`, and the ruling-places rule the shape of `feat.ruling-places` (W1's 0087).
- **Floors stay whole.** `impl.gate-card-confirm` is unchanged (D5: "unchanged").
  `impl.gate-final-acceptance` keeps its force; the lean is that D6's table lands as a new rule
  beside it rather than a reword of the floor, and P1 argues either.
- **One home per mechanic.** Default 3's altitude ("plain consequences with a worked example, never
  a schema") is a domain reading of `analysis-iterative.question-form` and
  `.mock-in-message`. P1 says whether it needs a rule of its own or is carried by the asking line,
  and the gate reads that call.

### 3.1 The card order — D5, default 4, default 5 (0092)

- **Mint `impl.card-order`.** Before the card-authoring seat slices, the lead shows a rough order of
  what works after each step as a mock in the message. A question is put only on a real choice:
  which independent outcome first, or few big steps against many small. Where the walking skeleton
  and the dependencies leave no choice, no question is asked. The ruled order and step size are the
  ruled set, writer *on*; the card-authoring seat is the next writer; the order is saved as one line
  in `tasks.md`'s header.
  - No order question on a delta or lane run, which builds from one desk card (V2).
  - A lane run that slices several cycles raises its order line's home to the user (W1, QR7).
  - P1 picks the section, `when:` (scope) and the moment wording. Lean: a `card-order` moment beside
    `card-confirm`, mirroring W1's `card-set`.
- **Reword `impl.card-review-before-confirm`.** The verification seat's card review also checks
  three things:
  - the cards follow the ruled order;
  - every "I'd know it works if…" line has at least one **TEST:** case keeping its meaning
    (default 5);
  - an `open` line rides open, listed at the card confirm.
  - The cards are shown as "after step n you can…" (D5).
- **Mint `impl.fail.cards-outside-order`** (`kind: fail`, `class: floor`). Cards cut against a ruled
  order without the user's word (D5, RB2). Its `enforces:` names the order rule and the card
  review.
- **Template work, only if the order line needs it.** `impl.cards-template` / the `tasks` template
  (`mochiko-cli template tasks`): P1 reads whether a one-line order header already fits the
  template's header. If not, one template `replace-document` adds it, and the fixtures move (P3).

### 3.2 The asking line and every ruling place — D10's Implement row, defaults 1–3 (0093)

- **Mint `impl.asking-line`** (`extends: common.asking-line`, `class: must`, ways-of-working — W1's
  placement).
- **Mint `impl.ruling-places`** (`kind: binding`, `impl.sec.reserved`, anchor D10). It covers every
  ruling place of the Implement row, each with its writer setting (record lines `~1209–1225`):
  - **on** — the card order (pointing at `impl.card-order`) · the checkpoint rulings: infeasible
    card, ambiguity, adopt-first and IP-XXX calls, constraint challenge, with the cycle's
    verification seat checking that the rework kept the ruling · the final-validation dispositions,
    with the landing verifier checking the `gates.md` fold · the deviation gate, the delta amended
    from the ruling, its existing non-author delta grade checking it;
  - **held** — the governance conflict (the ledger's own amend path) · the run-open rulings
    (written into the sufficiency report, binding the run);
  - **off** — scope escalation · finding kind disputed · gap-rework bound exhausted · epic member
    halt · attempt exemption · disputed sufficiency clause · desk offer · absent-surface offers;
  - **gates (G · held)** — design-first-write and acceptance stay gates, held by the landing
    verifier, and are not asked the asking line's way.
  - P1 names each place by its existing rule id and checks every id is live.
- **Defaults 1–2.** P1 carries each where the meaning is not already in the asking line:
  - **default 1:** the checkpoint — a real choice on its own turn and the rest on one batch screen;
    an ambiguity only the user can settle asked as a fact question; a build-blocking question still
    interrupting at once. Lean: a reword of `impl.escalation-batching` keeping its interrupt clause.
  - **default 2:** the final-validation findings — fix now, book, or accept as designed, with the
    lead's pick and the case against; "accept as designed" checked against the run's ruled set.
    Lean: a reword of `impl.beyond-spec-disposition`, the reservation kept.
- **Default 3:** see §3.0's one-home check.
- **The promises-kept table (D6, RB1).** At acceptance the gate is shown as a table: each "I'd know
  it works if…" line from the slate, the run's `design-stage.md` or the feature visit file, quoted
  verbatim in the user's words, beside the test that proves it and its result.
  - An `open` line reads "open — no target set"; a measured value is shown where there is one; no
    pass / fail column.
  - "Amend" names the disputed rows; each is then put as fix now or book.
  - The lead composes the table in the conversation, in the user-facing register; no new file. The
    final-validation report and the built-vs-signed diff stay the evidence beneath it.
  - Carrier: §3.0's floor lean.

### 3.3 Implement reads the desk; the outside need; the finder — default 12, default 24, D8 (0094)

- **Default 12 (RB3, S10).** At entry, in every scope — selection, delta, lane, an epic's member —
  whatever the gap count, implement reads a feature-desk visit file keyed to its FEAT or lane,
  `.mochiko/features/desk/<date-slug>/visit.md`, its slug carrying the joined FEAT id lower-cased
  or `lane-<slug>` (W1's `feat.visit-file`). The card itself is read there too (D7's change).
  - Carriers: `impl.design-inputs` and `impl.design-stage-inputs` widened (both name only the
    architecture desk's file today); the delta-scope entry's "desk-confirmed delta card" reads the
    card in the visit file.
  - Also the sufficiency check's per-card form on the zero-gap delta path — 0095 if
    `review-sufficiency`'s line is made false.
  - P1 locates every implement rule that says where a delta card sits (e.g. `impl.landing-delta`,
    `impl.card-contents`) and rewords only those made false.
- **Default 24 (QR6).** An outside need the design stage sends to the feature desk
  (`impl.design-stage-agenda`'s "a feature-desk card") arrives with the run's note handed in to the
  desk visit (W1's `feat.carry-forward` reads it). Carrier: one clause on `impl.design-stage-agenda`
  naming the note's contents (the need in the user's words, the run it came from), or a new
  binding beside it if `impl.sec.tools` cannot take the clause.
- **D8.** `impl.gap-finding-blind-dispatch`'s first message gains the user's own lines from the
  run's ruled sets: each direction and "I'd know it works if…" line in the run's `design-stage.md`
  and in any feature-desk visit file keyed to the run.
  - Never the ruled pieces, the **TEST:** cases, the code or any report.
  - The rest of the fence and the two-message dispatch stay unchanged.
  - The reach is selection and epic runs, since delta and lane runs skip gap-finding
    (`impl.gap-finding-scope`); no text needed for that.
  - This rule sits in `impl.sec.tools`: measured.

### 3.4 Migration 0095 — `skill/review-sufficiency` (conditional)

P1 reads `review-sufficiency`'s delta-card line (view line 40) against W1's `feat.delta-cards` and
`feat.visit-file`. If the line is made false, one `reword-rule` points it at the card in the visit
file, the entry's marked work row pointing at it; if not, 0095 is unused and the plan says why.

### 3.5 Prose (P2)

- **`commands/implement.md`.** Only lines this wave makes false, e.g. the Entry step's "a
  desk-confirmed delta card" (`:59-61`) where the card's home is named. The card-confirm step and
  the acceptance step are touched only where they now read false.
- **Router.** Rows naming implement's acceptance or card confirm, where made false.
- **Strips.** One entry per removal or supersession in shipped prose, `[v0.121.0]`, protected lines
  only by recorded supersession. A wave-context comment per touched strip file for schema-only
  rewords (W1's form, `strips/feature.md` top).
- **W1's carried note (gate grader, W1 audit).** Re-read the `feature-visit` template overview
  ("the dispatched run reads the cards and their set here") and `GLOSSARY.md:16` ("read by
  implement") against the built W2 rules. If either is still false, fix it: the overview is
  P1's (a template `replace-document`), the glossary P2's.
- **Budgets.** `review-sufficiency` re-measured if 0095 lands; implement unbudgeted (commands are,
  ledger `:560–566`), its sizes in the CHANGELOG only.
- **`CHANGELOG.md`.** The W2 part replaces the `[lead: Wave 2 — implement's later moments.]`
  placeholder.

### 3.6 Tests and kits (P3)

- **Crate census tests.** Counts read from `migrate status`, the renders and a census probe, never
  by hand. Add sequences 0092 onwards to `fidelity.rs`.
- **Contract suite.** `run.py` `EXPECTED["implement"]` gains any new floor and fail. The README
  figures change only where a frozen count moved. `TEMPLATE_NAMES` and the fixtures change only if
  a template changed.
- **Plan kit.** `evals/plan/implement/`'s `observable.yaml` re-keyed, every live rule id in exactly
  one list, plus `evals/plan/README.md`'s implement figure. The preregistration is recounted only on
  the W1 precedent (never run), else left with the reason.

### 3.7 Decision and default coverage

| Decision / default | Carrier |
|---|---|
| D1 as amended (by D6, D10) | `impl.ruling-places` · the checkpoint and dispositions carriers · D6's table · `impl.gate-card-confirm` unchanged |
| D5-card-order-asked | `impl.card-order` · the card-review reword · `impl.fail.cards-outside-order` · the order line's home |
| D6-promises-kept-table | the table's carrier (§3.2) |
| D8-finder-reads-promises | `impl.gap-finding-blind-dispatch` reword |
| D10-two-switch-test (W2 share) | `impl.asking-line` · `impl.ruling-places` |
| Defaults 1, 2, 3 | §3.2's carriers and §3.0's one-home call |
| Defaults 4, 5 | `impl.card-order` · the card-review reword |
| Default 12 | `impl.design-inputs` / `impl.design-stage-inputs` widened · the delta entry · 0095 if needed |
| Default 24 (implement side) | the `impl.design-stage-agenda` clause or its binding |
| W1's carried note | P2's re-read and any fix |

Not this wave: the specify, setup and architecture-desk rows; specify's reader for default 23;
defaults 15–16; default 21's residuals (all W3). Default 22's count is reported at the bump.

## 4. Audit (leg 2)

One gate grader for the wave: a plain `general-purpose` seat, `model: opus` explicit.
- **The contract** is rendered first-hand by the seat (`mochiko-cli rules validation-primitive-edit`,
  every block).
- **The brief** carries the units, the paths, the pre-pass commands and any named overage.
- **Output.** One verdict block and one outcome line per unit, in `reports/audit-w2.md`.

The units:

1. schema content — 0092 and the implement view diff
2. schema content — 0093 and the implement view diff, every place `impl.ruling-places` names
   checked live
3. schema content — 0094 (and 0095, and any template `replace-document`) and their view diffs
4. the `implement` command pair — the tools ceiling measured, every caller slot read against the
   engine render, the asking line's cited ids live
5. the `review-sufficiency` skill pair, if 0095 lands
6. router, strips, ripple (including W1's carried note), budget ledger, changelog
7. crate tests, contract expectations, the implement plan kit, any fixture

On a FAIL, the owning seat fixes and the same grader is resumed on the delta. A second FAIL on any
unit halts to the user (`common.gate-loop-bound`). Beside the audit, the lead reads the built rules
against §3.7 row by row; that read clears nothing on its own.

## 5. Wave close

No bump at wave close. The wave closes when §4 is all PASS. The gates (`migrate validate` · views ≡
replay · `cargo test` + fmt + clippy + audit · the full similarity sweep · char budgets) run as a
regression read at the close, and again before the bump after W3, with the contract suite
(`sbx login` and `sbx secret set anthropic` are the user's; a SKIPPED suite blocks).

## 6. Stop conditions

Each of these halts the wave and goes to the user:
- plan FAILs from more than half the producers;
- any second re-plan;
- any tree dirtied during planning;
- a rejecting `migrate validate` finding a seat cannot resolve inside its ownership;
- any floor or fail retired or lowered;
- an engine rule edit, a new engine slot, or a grammar or crate-source change found needed;
- `impl.sec.tools` unable to fit under its 29,000-byte line;
- a ruled text that will not fit without changing its meaning;
- a second FAIL on any gate unit.
