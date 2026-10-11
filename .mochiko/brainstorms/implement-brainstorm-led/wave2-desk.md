# Wave 2 — the desk's focus stage on the engine, the visit file, and the store's diagram set

**Opened:** 2026-10-11, after wave 1 closed (build log) · **Lead:** the build session lead ·
**Branch:** `engine-desk-build` · **Target version:** 0.120.0, one bump after wave 3 ·
**Records:** `.mochiko/brainstorms/architecture-brainstorm-interview/record.md` — D1–D5, D7–D9 as
review-amended (the desk's share; implement's touches are wave 3's), the *Defaults batch*, the
*Repair batch*, *Verify pass*, *Acceptance and landing* — and
`.mochiko/brainstorms/implement-brainstorm-led/record.md` — D1, D3 (the desk built straight onto
the engine), the *Defaults batch* (*Precedence*, *Slots declared where*, *Change control*),
*Supersessions owed at the build*, *Touch set for the build*. These are the source for every
rule text; this plan names carriers, never restates a decision.

Wave 2 is architecture build items A and B, desk side: the desk's focus stage written straight
onto wave 1's engine (the desk keeps only its domain part), the visit-file home, the store's
diagram set with the index's diagrams section, sixth health count, regeneration commit and stale
compare, and D9's sign-off wording at the desk, the store skill and `patterns-system-design`.
Implement's ten touches are wave 3.

## 0. Sound-loop wiring

As wave 1 §0, with a fresh re-plan counter (one, shared by this wave's producers; a second
consumption goes to the user). Plans are written by the seat to one scratch file outside the tree
and graded by path and hash (wave 1's disclosed transport); returns kept short.

## 1. Sequence allocation (0065–0074; gaps legal)

- **`0065-desk-visit-home.yaml`** — `home/architecture-desk` (`import-document`): directory
  `.mochiko/architecture/desk/<date-slug>/`, one deliverable `visit.md` bound to a new template,
  `reports/` under the report envelope (`feature-desk`'s shape, `0005-artifact-homes.yaml`);
  `template/architecture-visit` (`import-document`).
- **`0066-store-diagram-set.yaml`** — `home/product-diagrams` (`import-document`):
  `.mochiko/product/architecture/diagrams/`, one deliverable per diagram file, its own
  `**Lifecycle:**` field exempt as the baselines' is (`0034-store-entry-budgets.yaml`,
  `entry_exempt_fields: [Lifecycle]`); `home/product-architecture` (`replace-document`, `subdirs`
  gains `diagrams`); `template/architecture-diagram` (`import-document`);
  `template/architecture-store` (`replace-document` — layout table, the index's parts, skeleton).
- **`0067-store-diagram-rules.yaml`** — `skill/authoring-architecture-store`.
- **`0068-desk-focus-stage.yaml`** — `command/architecture`.
- `0069`–`0074` held for a fix round or a split P1 proposes with its reason.

Header anchors: `2026-10-11 architecture-brainstorm-interview` on the desk and diagram content;
`2026-10-11 implement-brainstorm-led` on the engine stub. A rule carrying one ruled decision takes
it as its rule-level anchor (`… D3`); a confirmed default takes the bare session anchor. A reword or
supersession of a `class: floor` rule carries its anchor and names the superseded ruling.

## 2. Seats and ownership (disjoint)

| Seat | Owns | Depends on |
|---|---|---|
| **P1 schema** | migrations 0065–0068 (and any held number it uses) · `.mochiko/schema-views/**` (re-emit only) · `scripts/similar-rules-allowlist.yaml` (rows only for edges the sweep reports, each with its why) | wave 1 landed |
| **P2 prose** | `plugins/mochiko/commands/architecture.md` (the `argument-hint`; lines made false) · `skills/authoring-architecture-store/SKILL.md` (lines made false) · `skills/patterns-system-design/SKILL.md` (`:54`, the checklist item `:128`, any other line D9 makes false, e.g. `:68`) and `references/DIAGRAM-CONVENTIONS.md` (the taste section; `:56`) · the router `skills/mochiko/SKILL.md` (lines made false) · strips for every removal or supersession · ripple lines made false (`README.md`, `ARCHITECTURE.md`) · `.mochiko/memory/primitive-cost-budgets.md` · `CHANGELOG.md` (the W2 part of `[0.120.0]`) | P1 landed |
| **P3 tests and kits** | crate frozen-census tests under `crates/mochiko-cli/tests/` (figures only) · `evals/contract/**` · `evals/plan/architecture/**` | P1 landed |

Order: P1 plan → peer grade → approve → build → lead's read · then P2 and P3 in parallel, each plan
→ peer grade → approve → build · §4 audit. P3 starts its build after P2 lands where a check reads
P2's surfaces (wave 1's order).

## 3. Content

### 3.0 What every rule text must respect

As wave 1 §3.0 — guardrails, never a sequence; one or two sentences per minted rule, plain words;
one home per mechanic; nothing weakens (all 23 desk floors and its fail stand); no grammar or
binary change (one found needed goes back to the user). Specific to this wave:

- **The engine wins.** The desk carries no asking mechanic of its own — no fork, options, batch,
  streak, stop or provenance rule; those are `mochiko:analysis-iterative`'s, reached by
  `arch.thinking-stage`. The desk's rules carry only its domain part and the slots it fills. The
  architecture record's default *Where the rules live* ("the desk's rules win") never shipped; its
  flip is recorded at the landing (`DECISIONS.md` row annotation), not as a strip entry.
- **Texts come from the cards as review-amended** — every changed-at-review line, the repair batch
  and the verify repairs included. The ids below are proposals; P1 may merge or split with the
  reason; the approved plan fixes the set; every row of §3.6 keeps a carrier.
- **The desk's existing gates stand.** `arch.user-reserved-rulings`, `arch.truth-user-ruling`,
  `arch.sound-loop-floor` and the tech-lead grade are untouched (S2); the sameness grade is
  additive to the user's word on the rows.

### 3.1 Migration 0065 — the visit-file home and template (D3, S5, V3; the user's `visit.md`)

- **Home** `architecture-desk` — `.mochiko/architecture/desk/<date-slug>/` (outside
  `.mochiko/product/`, S5), deliverable `visit.md` bound to `architecture-visit`, one whole-file
  bound (P1 proposes the number against `feature-desk`'s 300 and the template's size), `reports/`
  under the report envelope. On a feature's delta visit the slug carries the joined FEAT id (D5);
  P1 confirms `<date-slug>` admits it (`mochiko-cli home` on a sample path), else proposes a token.
- **Template** `architecture-visit` — a header (the visit goal and its done condition; the FEAT id
  it is keyed to, when one; any brainstorm record or direction handed in) · the focus set, per item
  the part (store element, AX row, shelf row or spine area, D1), the user's direction in their
  words, the "how I'd know" sentence or `open` (D2), the provenance mark (the engine's vocabulary)
  · the defaults batch as confirmed (no-stance rows, S11; default, cost and case against, S10) · the
  blind list's differences and their rulings (D4) · outside claims checked, each with its source or
  `unknown` · the rows written, each linked (D3) · the closing verdict. Present tense; the skeleton
  constrains the shape, never the voice.

### 3.2 Migration 0066 — the diagram sub-home, its template, the index template (D7, D8, D9)

- **Home** `product-diagrams` — `.mochiko/product/architecture/diagrams/`, one deliverable per
  diagram file (P1 picks the declared name token — `<slug>.md` or the nearest the grammar admits —
  and says why), one entry per file with its own `**Lifecycle:**` field, exempt as the baselines'
  is (S3). `product-architecture`'s `subdirs` gains `diagrams`.
- **Template** `architecture-diagram` — a title · one Mermaid block · a one-line caption · a
  `Draws:` line citing store elements and concern rows by joined ID and data-model entities by file
  and heading (`data-model.md § Vault`, S1) · the `**Lifecycle:**` field (the store's seven values,
  one open key per file, S3). No stale field in the file (V2: the flag's one home is the index).
- **Template** `architecture-store` (`replace-document`) — the layout table gains `diagrams/`; the
  derived index gains a diagrams section (each diagram linked with a one-line summary, never
  embedded — D7); its header carries the commit it was last regenerated at (V2); the health view
  goes from five counts to six — stale diagrams, each named (S4/V2); the skeleton follows. Every
  other line stands.

### 3.3 Migration 0067 — `skill/authoring-architecture-store` (D7, D8's store share, D9)

- **Reword** `authoring-architecture-store.health-view-binding` — six counts, the sixth the stale
  diagrams, each named (S4 as repaired).
- **Reword** `authoring-architecture-store.sign-off-flips-proposed` (`class: floor`) — "on the
  rendered diagram" becomes D9's surface, the Mermaid text that parses, rendered where the session
  can (S9 names this text); the gate's force and every other clause kept. Anchor `… D9`.
- **Mint** (proposed ids; P1 may merge):
  - `diagram-set-grammar` (`kind: binding`) — the set in `diagrams/`, one file per diagram, one
    entry per file with its own `**Lifecycle:**`; the `Draws:` citation grammar (S1); one open key
    per file — a second run finding a diagram keyed to another waits or routes to the user, never a
    second marker (S3); any kind, each earning its place by the paragraph test and the
    plan-minimalism ladder; the spine keeps its container view and key-flow sequences (D7).
  - `stale-compare` (`kind: duty`) — at every regeneration of the index, this skill reads every
    `Draws:` target against the entries and headings changed since the commit the index header
    carries; a cited target changed with no delta-styled change in that diagram under the same key
    or visit, or a citation that no longer resolves, flags the diagram stale in the health view, its
    one home; a model's read, no code (S4, V2; an advisory `mochiko-cli` check possible, never
    required).
  - `diagram-redraw-timing` — a stale diagram is redrawn at the next desk visit or the next design
    phase that touches it, never inside a landing (D8); a reader of a diagram reads the health
    view's stale list with it (V8).
  - The index's diagrams section (pointer and one-line summary, never an embed) and its header
    commit — a new rule or a reword of the rule naming the index's parts, P1's call with the reason.
- `one-home-one-writer`, `index-regenerated-every-write`, `fold-duty`, `lifecycle-statuses`: read
  for lines this wave makes false; reworded only where false.

### 3.4 Migration 0068 — `command/architecture` (D1–D5, D7–D9 desk share, defaults)

- **Condition** — a visit-kind term so the stage's rules bind only on a visit that rules
  something (S13): a baseline, a stance, an amendment, a feature's delta thinking — never a store
  query, a drift visit (dispositions land on the rows as today) or a trigger's routing. P1 proposes
  name, values and resolution (the lead's lean: entry-derived from the converged goal).
- **Moment** `focus` — after the visit goal is agreed, before the principal-architect authors
  anything (D1). Anchors, never orders.
- **Mint** `arch.thinking-stage` in `arch.sec.boundaries` — `class: floor`,
  `extends: common.thinking-stage` (the third extender; implement's at wave 3 meets the bar).
- **Mint the domain part** (proposed ids; `class: must` unless marked):
  - `arch.focus-stage` (`kind: duty`) — the stage runs on the thinking stage with its ruled-set
    contract `on`: the focus set is the ruled set, saved in the visit file; the principal-architect
    seat the next writer; only on a visit that rules something; its close inside the visit's done
    condition (D1 as amended, S13).
  - `arch.focus-unit` (`kind: binding`) — per item the part, the direction in the user's words, the
    "how I'd know" sentence or `open`; where the sentence lands (S19: the visit file always; a
    `not-now` row's upgrade trigger; a `decided` row's first `Rationale` line; a number only when the
    user gives one); an `open` sentence carried as an open question, never invented (D2).
  - `arch.focus-batch` — the engine's defaults batch filled with the in-scope shelf rows carrying no
    stance yet (`open`, or newly in scope); a ruled row changes only as a focus item the user pulls
    in — an amendment demand, a fired trigger, a drift finding (S11); items and rows each in
    retrofit-cost order, each row with its default, cost and case against (S10;
    `arch.breadth-invariant` and `arch.recommend-then-arbitrate` unamended).
  - `arch.focus-blind-list` (`kind: duty`) — the engine's blind list at the desk's cadence: once per
    visit, before the set closes, skipped only on a single-row ask (the goal names one row and the
    user hands in its direction); its inputs per visit (S12) — the goal, the `Scope:` line, the
    derived index, any brainstorm record named, `FEATURES.md` and the capability entries in the
    territory, the codebase analysis where one exists, a feature's spec on a delta visit; never the
    focus set, the visit file or the conversation; on a greenfield baseline the product description
    and the capability map carry the context; a candidate the shelf lacks is how an off-shelf
    concern enters (D4).
  - `arch.visit-file` (`kind: binding`) — the visit file at the `architecture-desk` home, written by
    the lead as the visit goes (transcription, no sound loop); the lead holds its pen, the
    architect the store's (D3).
  - `arch.rows-from-visit-file` (`kind: duty`) — at the set's close the principal-architect authors
    the rows and spine elements from the visit file in the same visit, each row linking the file as
    its rationale home; tech-lead grades them against the file for sameness of meaning, additive to
    the user's gate on the row text (S2); the visit does not close until the rows are in (D3).
  - `arch.first-visit-stage` — greenfield: the elicitation is the focus stage; brownfield: the
    reconstructed derivation is the fact base, the stage picks which derived elements get a real
    fork, its close the confirm step (`arch.dm-author-baseline` stands; default *First visits*).
  - `arch.feature-delta-visit` — `/mochiko:architecture <FEAT-ID>` closes in a visit file keyed to
    the feature; the desk writes nothing to the store for a feature delta and no link on the
    feature's capability entry (D5, S17; `arch.no-delivery-harness` stands).
  - `arch.outside-claims` — a load-bearing outside claim a focus item hangs on is live-checked
    before the item is ruled, its source in the visit file; one that cannot be checked is stated
    unknown and the item carries it `open` (default *Outside claims*).
  - `arch.desk-redraw` — a change made at the desk with no feature redraws the diagrams it touches
    during the visit, by the architect seat inside the ruled focus set, graded by tech-lead against
    the targets the `Draws:` line cites (D8, V4).
  - `arch.diagram-signoff` — a diagram's sign-off surface is its Mermaid text, which must parse —
    checked by the render surface when one exists, else by the author's own read, the degrade note
    saying which; rendered beside the text where the session can, else the one-line note; the user
    asked before any publish to a hosted service, a local preview preferred (D9, S9).
  - `arch.fail.write-outside-focus` (`class: floor` · `kind: fail`, `arch.sec.fail-conditions`) — a
    desk visit's store write (baseline, stance batch, amendment, diagram) with no ruled focus set
    behind it in a visit file; an element the set lacks is proposed as a candidate, never authored
    before the user rules it in; scoped to desk visits, never an implement run (S18). `enforces:`
    the stage and rows rules above.
- **Reword** (id survives): `arch.seat-architect-producer` (it also draws the store's diagrams and
  authors rows from the visit file) · `arch.seat-tech-lead-grader` (its grade re-checks the outside
  claims the rows rest on through `review-brainstorm`'s `EXTERNAL-CLAIMS.md` review-seat duty, named
  in the grading brief, S21 — P1 reads whether a `pointer:` can carry that reference from a command,
  else the text names it; it grades a drawing against its `Draws:` targets) · `arch.tools-store`
  (the store gains `diagrams/`; the index's parts per §3.2) · `arch.artifact-home` (the visit home
  and the diagram sub-home bound like the others) · any line §3.2–§3.3 makes false (e.g.
  `arch.dm-store-integrity-close`'s index list). `arch.dm-converge-goal`, `arch.dm-close-verdict`
  and `arch.fail.no-verdict` stand.
- **Not desk rules** (the engine carries them): who holds the conversation, one fork per turn,
  options, the in-message mock, the status line, own-turn-or-batch, the streak over forks, vague
  answers, the stall and stop rules, the provenance vocabulary, inputs in hand opening as facts (a
  brainstorm record in `$ARGUMENTS` is the engine's `inputs-in-hand`; the `argument-hint` gains it).

### 3.5 Prose (P2)

- **`commands/architecture.md`** — `argument-hint` gains a brainstorm record and a `<FEAT-ID>`;
  otherwise only lines this wave makes false (no rule text in the `.md`).
- **`skills/authoring-architecture-store/SKILL.md`** — lines made false by §3.2–§3.3 only.
- **`skills/patterns-system-design/`** — `SKILL.md:54` ("it must render, never ship as a raw code
  block masquerading as a picture") and the checklist item `:128` ("The diagram renders … not read
  as source") superseded by D9 (S9), strip entries each; any other line D9 makes false (`:68` —
  check). `references/DIAGRAM-CONVENTIONS.md` gains the taste section (default *Taste*: one accent
  colour for the one or two things to look at first, the delta keeping its three classes; hairline
  strokes, no fills beyond the accent; one font family by the init directive; delete first and the
  paragraph test before drawing; the ~12-node budget; a title and a one-line caption; a delta read
  as Before · Changes · After; SVG-only rules do not carry), the queued trust-boundary convention to
  extend it later (S23); its `:56` made true under D9; the init-directive line states GitHub's
  behaviour only as §5's check finds it.
- **Router, ripple, strips, budgets, `CHANGELOG.md`** — as wave 1 §3.8: lines made false only;
  `[v0.120.0]` strip entries for every removal and supersession, consumers assessed (the desk,
  implement's design phase, `review-plan-artifacts`, the store skill); the store skill's payload
  re-measured as a skill pair, growth argued as the ruled D7/D8/D9 obligations; the W2 part.

### 3.6 Decision and default coverage

| Decision / default | Carrier |
|---|---|
| D1-focus-stage-first (S10, S11, S13) | `arch.focus-stage` · moment `focus` · the visit-kind condition · `arch.focus-batch` |
| D2-part-direction-know (S19) | `arch.focus-unit` · the visit template's item shape |
| D3-visit-file-home (S2, S5, V3, V6) | 0065 · `arch.visit-file` · `arch.rows-from-visit-file` |
| D4-blind-list-visits (S12, S22) | `arch.focus-blind-list` (the engine's `blind-second-list` at the desk's cadence) |
| D5-visit-file-input, desk share (S17; S20 is implement's) | `arch.feature-delta-visit` · the FEAT-keyed slug |
| D6-desk-stays-engine | wave 1's engine and `arch.thinking-stage`; BACKLOG at landing |
| D7-diagram-set-index (S1) | 0066 · `diagram-set-grammar` · the index's diagrams section |
| D8-draw-before-code, store and desk share (S3, S4, V2, V8) | `diagram-set-grammar` · `stale-compare` · `diagram-redraw-timing` · `health-view-binding` · `arch.desk-redraw` |
| D9-render-where-viewed (S9), desk, store, system-design share | `arch.diagram-signoff` · `sign-off-flips-proposed` reworded · `patterns-system-design`'s lines |
| *Who holds the conversation* · *Mechanisms carried* · *Inputs already in hand* · *Provenance* · *No size gate* · *Where the rules live*, flipped | the engine (`arch.thinking-stage`); no desk asking rule; the visit file holds the provenance; the flip recorded at landing |
| *A `kind: fail`* (S18) · *Context inputs* (S12) · *First visits* · *Outside claims* (S21) | `arch.fail.write-outside-focus` · `arch.focus-blind-list` · `arch.first-visit-stage` · `arch.outside-claims` · `arch.seat-tech-lead-grader` |
| *Taste* (S23) · OQ3 · *Who draws* (V4) · *Readers* (V8) | the taste section · §5 · the two seat rewords · `diagram-redraw-timing` |
| *Two notations* · entity IDs · the shared-stage item · dogfood criteria | none in the build — open threads and BACKLOG at landing |

### 3.7 Tests and kits (P3)

Crate frozen-census figures from `migrate status`, the views and the renders, never by hand (no
crate source, no crate version) · `evals/contract/`: `Expected["architecture"]` gains
`arch.thinking-stage` and `arch.fail.write-outside-focus` (and any floor P1 mints), the store
skill's frozen floor set only if its floors move, README figures where a count moved ·
`evals/plan/architecture/`: `observable.yaml` re-keyed, each minted id with its why, every live
schema rule id in exactly one list; the kit's static checks pass; no grid.

## 4. Audit (leg 2)

One gate grader for the wave: a fresh plain `general-purpose` seat, `model: opus` explicit (a new
seat that authored nothing here), its brief the verbatim `mochiko-cli rules
validation-primitive-edit` render (every block), the units, the paths, the pre-pass commands and
any named overage. Units, one verdict block and one outcome line each, in `reports/audit-w2.md`:

1. schema content — 0065 and the new home and template views
2. schema content — 0066 and its home and template view diffs
3. schema content — 0067 and the store skill's view diff
4. schema content — 0068 and the architecture view diff
5. the `authoring-architecture-store` skill pair
6. the `architecture` command pair — with *Change control*: the engine's render re-read against
   the desk's (no desk rule loosens an engine mechanic), and the delivery check that
   `arch.thinking-stage` resolves to the engine's seven blocks, first-hand, as wave 1's unit 5 did
7. `patterns-system-design` (prose skill and reference), router, strips, ripple, ledger, changelog
8. crate tests, contract expectations, the architecture plan kit, any allowlist rows

On a FAIL the owning seat fixes and the same grader is resumed on the delta; a second FAIL on any
unit halts to the user (`common.gate-loop-bound`). Beside the audit, the lead reads the built rules
against §3.6 row by row; that read clears nothing on its own.

## 5. Facts the build checks

- **OQ3 — does GitHub honour a Mermaid init directive?** A real GitHub render needs the file on
  GitHub, an outward-facing step that is the user's (a push or a gist). Put to the user at the
  wave's open; until it is checked, the taste section's init-directive line holds only where the
  renderer honours it.
- **Size.** `architecture`'s render (20,918 bytes at the build open) gains a stage; P1 reports each
  render's size per block, before and after.

## 6. Wave close and stop conditions

No bump at wave close. The wave closes when §4 is all PASS; the gates (`migrate validate` · views
≡ replay · `cargo test` + fmt + clippy + audit · the full similarity sweep · char budgets) run as a
regression read, and again before the bump after wave 3, with the contract suite.

Stops, each halting the wave to the user: plan FAILs from more than half the producers · any second
re-plan · a tree dirtied during planning · a rejecting `migrate validate` finding a seat cannot
resolve inside its ownership · any floor or fail retired or lowered · a grammar or crate-source
change found needed · an asking mechanic the desk needs and the engine lacks (a slot by ruling) · a
second FAIL on any gate unit.
