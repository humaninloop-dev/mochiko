# Strip notes — `commands/feature.md`

Entry formats: `strips/README.md`. Ruling for every [v0.68.0] entry below:
`DECISIONS.md` 2026-08-13 "PM role & feature derivation" row →
`.mochiko/brainstorms/pm-role-and-feature-derivation/record.md` (D6/D7/D8/D9/D10).

<!-- Wave context: the product-architecture-schema Stage-1 build wave (v0.81.0). Ruling for every
[v0.81.0] entry below: `.mochiko/brainstorms/product-architecture-schema/record.md` (D7 · D12 ·
D13) → `DECISIONS.md` 2026-08-19 product-architecture row. -->

<!-- Ruling for every [v0.95.0] entry below: the command-content-schema D10 per-command rollout
ruling, 2026-08-26 (`.mochiko/brainstorms/command-content-schema/record.md`, Session trail —
"D10 rollout ruling (2026-08-26, user-directed)"; `DECISIONS.md` 2026-08-26 command-content-schema
row, D2/D7/D10/D14/D15). Step-0 referent, frozen:
`.mochiko/brainstorms/command-content-schema/referents/feature-shipped-v0.94.0.md`. Scope:
structure-only extraction — meaning survives whole; rewording only where D15 referential closure
required it, each recast named in its entry. -->

## [v0.95.0] Charter converts to the `.md` + schema pair — Roles & Responsibilities moves to `feature.yaml` `feat.sec.roles`

- **Disposition:** superseded → `plugins/mochiko/schemas/feature.yaml` section `feat.sec.roles`,
  rules `feat.staffing-latitude` · `feat.dm-health-first` · `feat.dm-converge-goal` ·
  `feat.dm-map-integrity` · `feat.dm-route-honestly` · `feat.dm-complete-card` ·
  `feat.dm-epic-stewardship` · `feat.dm-km-landing` · `feat.dm-close-verdict` · `feat.pm-seat` ·
  `feat.architect-dormancy` · `feat.dispatched-runs-own-delivery` · `feat.user-reserved`.
- **Tier failed:** n/a — supersession by ruling (command-content-schema D2/D7 + the D10 rollout
  ruling 2026-08-26; section grammar D14; `DECISIONS.md` 2026-08-26 command-content-schema row).
- **Content:** the shipped v0.94.0 section, verbatim:

  ```
  ## Roles & Responsibilities

  There is **no Bindings section**. The bare minimum that must always happen is carried here as the
  Delivery Manager's owned responsibilities; everything beyond it is your per-visit judgment — below
  the sound-loop floor (`mochiko:patterns-sound-loop`, Boundaries), how you staff, sequence, and run
  the visit is yours to shape (the lead-owned-process-flexibility posture, applied to a standing
  desk).

  **You, the Delivery Manager — the always-happens floor:**

  - Surface health before the ask.
  - Converge every visit to a one-line goal and its done condition.
  - Keep map integrity intact at close — no dangling entries, no orphaned deltas, statuses agreeing.
  - Route honestly by the capability-write test; keying a raw report to its surface is triage
    judgment, audited from the resulting map delta, never claimed mechanical.
  - Hand every dispatched run a complete card.
  - Steward open epics across visits — mint (mint-once, overlap-guarded), membership change,
    status, and close.
  - Execute the KM landing for desk-side writes where knowledge-management exists.
  - Close the visit with a verdict against its done condition.

  **Other seats:**

  - **PM seat** — the extend-vs-mint verdict, grooming proposals when the soft cap trips (merge
    lookalikes, retire dead entries), and the what-next line. Recommends with reasons; never rules.
  - **Principal-architect** — domain co-sign, **dormant until the first cap-trip**; no live duty on
    today's maps. The seat is not idle elsewhere: its standing home is `/mochiko:architecture`,
    where it stewards the architecture store — the dormancy is this desk's, not the seat's.
  - **Dispatched runs** — all delivery. Implement owns its bounds, verification seats, and
    evidence rules; the desk runs none of it.
  - **The user** — retire and merge rulings, route overrides, and every selection.
  ```

- **Kept deliberately:** every obligation survives at D12 grain in `feat.sec.roles`; the v0.70.0
  "below the sound-loop floor" narrowing (charter-ritual-balance D1/D3) survives inside
  `feat.staffing-latitude` with a `ruling:` anchor; the v0.81.0 principal-architect standing-home
  sentence (product-architecture-schema D7) survives verbatim inside `feat.architect-dormancy`
  with its anchor. D15 recasts, meaning unchanged: the document-shape opener "There is **no
  Bindings section**. The bare minimum … carried here" becomes "The bare minimum that must always
  happen is carried as the Delivery Manager rules in feat.sec.roles" (the checker's own deixis
  lint names "there is no X section" a defect); the `(mochiko:patterns-sound-loop, Boundaries)`
  cross-section parenthetical becomes the rule ID `feat.sound-loop-floor`; "where
  knowledge-management exists" names the file `.mochiko/memory/knowledge-management.md`;
  "Implement owns its bounds" names `/mochiko:implement`.
- **Consumers assessed:** commands are entry points, nothing mounts them. The pair audit re-key
  (`.claude/rules/mochiko/primitive-edits.md` pair-form block, v0.95.0) is lead-owned this wave.

## [v0.95.0] Tools moves to `feature.yaml` `feat.sec.tools`

- **Disposition:** superseded → `plugins/mochiko/schemas/feature.yaml` section `feat.sec.tools`,
  rules `feat.tools-referenced-never-restated` · `feat.map-files` ·
  `feat.map-minimalism-binding` · `feat.feature-map-binding` · `feat.epic-binding` ·
  `feat.epic-dispatch` · `feat.capability-write-test` · `feat.stable-ground-triage` ·
  `feat.delta-cards` · `feat.product-surface` · `feat.architecture-intake` ·
  `feat.dispatch-scope-split` · `feat.dispatch-specify` · `feat.km-relation` · `feat.register`.
- **Tier failed:** n/a — supersession by ruling (command-content-schema D2/D7 + the D10 rollout
  ruling 2026-08-26; D14; `DECISIONS.md` 2026-08-26 command-content-schema row).
- **Content:** the shipped v0.94.0 section, verbatim:

  ```
  ## Tools

  Each tool below is referenced, never restated — its procedure lives in its home.

  - **Map files** — the repo-root `FEATURES.md` index and the per-capability entry files at
    `.mochiko/features/FEAT-XXX-<slug>.md`; per-capability run artifacts at
    `.mochiko/features/FEAT-XXX/`. A map query is answered from the actual files in the territory
    asked about, never from memory of them.
  - **`mochiko:patterns-map-minimalism`** — the capability tests, extend-vs-mint, the soft cap and its
    grooming trigger, merge mechanics, and the `unrefined` stub mark.
  - **`mochiko:authoring-feature-map`** — the entry shape, delta grammar, the lane-liveness invariant
    (its home), and the fold-at-landing rules.
  - **`mochiko:authoring-epic`** — the epic's manifest + spine shape, the mint / membership-overlap
    guard, and close semantics; the desk stewards the epic (mint, membership change, status view,
    close) through it. A multi-feature epic dispatches to `/mochiko:implement` as one run over its
    members; delivery and its bounds stay the run's, never the desk's.
  - **Capability-write test** — the routing instrument: does the work touch a capability (mint, merge,
    retire, capability-status) or only its work rows? Capabilities route out; rows the desk may cut.
  - **Stable-ground triage** — key a report to its surface, then check the ground from the files: a
    single owning capability `delivered` → the delta lane, card on the entry · a single owning
    capability `in-flight` → not lane work, the finding files to the owning run · no single owner →
    the product lane, single-flight.
  - **Delta cards** — one card per the tasks template's card shape (rendered by
    `mochiko-cli template tasks`, or its schema `plugins/mochiko/schemas/tasks.yaml` Read raw when
    the binary is absent — the shipped schema is the first-class source of truth): a bug's
    acceptance is
    its reproduction-failing test, an improvement carries 1–3 acceptance criteria; plus the minimal
    enumerated `baseline-delta.md` in appliable before/after form when a product-baseline touch is
    known at intake. Each card — bug and improvement alike — takes the sound-loop review leg
    before dispatch (Boundaries).
  - **Product surface** — baselines at `.mochiko/product/` (`data-model.md`, `contracts/`,
    `constraints-and-decisions.md`, `quickstart.md`, and `architecture/` — the architecture store,
    whose derived index is repo-root `ARCHITECTURE.md`); product-lane
    runs at `.mochiko/product/lane-<slug>/`. Across repeat runs, cards and reports append (dated);
    delta files overwrite only via the graded fold.
  - **Architecture-desk intake** — a **fired upgrade trigger** arrives here from
    `/mochiko:architecture` as a candidate capability or work row and is routed like any other
    demand: the capability-write test decides the door, extend-beats-mint applies, and **the user
    rules the mint**. No new door and no auto-mint — the architecture lens proposes, the map
    machinery disposes.
  - **Dispatch targets** — `/mochiko:implement` for a dispatched capability-batch that scales itself,
    in the scope its door sets: **growth-door rows enter selection scope** (the batch covers the cut
    rows; its landing folds them into the capability's extent), a **bug/improvement delta card stays
    delta scope** (the desk-confirmed card is implement's delta entry; its landing is the delta fold)
    — the same split `/mochiko:implement` names at its Entry. `/mochiko:specify` for anything the
    capability-write test routes out. Where KM exists, `BACKLOG.md` is the defect queue and lane acceptance is a landing
    event; without KM, lane runs accept direct requests — the stated degrade path, never silently
    assumed.
  - **Register** — user-facing prose per `templates/output-style.md`.
  ```

- **Kept deliberately:** every binding survives; paths move to `vars:`
  (`${features_dir}` · `${product_dir}` · `${tasks_schema}`); skill bindings gain `pointer:`
  fields; the v0.81.0 product-surface list (product-architecture-schema D12) and the v0.91.0
  dispatch re-key with the D6 desk-confirmed-card clause (plan-stage-utility) survive with
  `ruling:` anchors. The epic bullet splits at D12 grain into `feat.epic-binding` +
  `feat.epic-dispatch`; the dispatch-targets bullet splits into `feat.dispatch-scope-split` +
  `feat.dispatch-specify` + `feat.km-relation` (splits, not drops). D15 recasts, meaning
  unchanged: "Each tool below" becomes "Every tool binding"; "takes the sound-loop review leg
  before dispatch (Boundaries)" names the rule ID `feat.sound-loop-floor`; "arrives here"
  becomes "arrives at the desk"; "Where KM exists" names
  `.mochiko/memory/knowledge-management.md`.
- **Consumers assessed:** commands are entry points, nothing mounts them; the named
  `plugins/mochiko/schemas/tasks.yaml` pointer is unchanged in meaning.

## [v0.95.0] Ways of Working moves to `feature.yaml` `feat.sec.ways-of-working`

- **Disposition:** superseded → `plugins/mochiko/schemas/feature.yaml` section
  `feat.sec.ways-of-working`, rules `feat.proactive-report` · `feat.reference-never-restate` ·
  `feat.author-grader` · `feat.advisory-front-door` · `feat.model-tiering` ·
  `feat.single-flight-lane` · `feat.no-git-mutations` · `feat.rulings-plain-text`.
- **Tier failed:** n/a — supersession by ruling (command-content-schema D2/D7 + the D10 rollout
  ruling 2026-08-26; D14; `DECISIONS.md` 2026-08-26 command-content-schema row).
- **Content:** the shipped v0.94.0 section, verbatim:

  ```
  ## Ways of Working

  - **Proactive report first** — health before the ask, every visit.
  - **Reference, never restate** — the dispatched run's bounds, verification seats, and evidence rules
    live in `/mochiko:implement` and the skills it binds; the desk points at them and adds nothing.
  - **Author ≠ grader** — wherever a seat produces (delta card, `baseline-delta.md`, grooming
    proposal), no output is cleared by its author; a producing seat plans first and works only on a
    plan you approved.
  - **Advisory front door** — `/mochiko:specify` stays directly invocable; the desk is the **default
    entry when the user is unsure** and the **only door for growth, bug, and improvement intake**. It
    is a routing service you can always use, never a gate you must pass.
  - **Model tiering** — exploration and fact-finding dispatches (map sweeps, territory reads)
    ride the class-keyed tiering floor: locate/enumerate reads go to a native `Explore`
    subagent spawned `model: haiku`, interpretive or absence-driven reads stay session tier, and every
    seat brief carries the routing rule. Class key, dispatch ladder, and brief obligation:
    `mochiko:patterns-model-tiering`, referenced never restated.
  - **Single-flight product lane** — one live product-lane run at a time.
  - **Commits and rulings** — suggest commits; never run git mutations, never push. User rulings are
    plain blocking text, never a timed prompt.
  ```

- **Kept deliberately:** all eight obligations survive; the model-tiering floor (v0.78.0 native
  `Explore` retarget wording) survives whole in `feat.model-tiering` with its `ruling:` anchor
  and `pointer:`; `model: haiku` moves to `vars:` as `${explore_model}`. The commits-and-rulings
  bullet splits at D12 grain into `feat.no-git-mutations` + `feat.rulings-plain-text` (a split,
  not a drop). No D15 recast was needed in this section.
- **Consumers assessed:** commands are entry points, nothing mounts them.

## [v0.95.0] Boundaries floor moves to `feature.yaml` `feat.sec.boundaries`

- **Disposition:** superseded → `plugins/mochiko/schemas/feature.yaml` section
  `feat.sec.boundaries`, rules `feat.capability-writes-sacred` · `feat.grooming-door-ceiling` ·
  `feat.out-of-remit-hosting` · `feat.growth-door` · `feat.growth-routes-to-specify` ·
  `feat.lane-never-widens` · `feat.no-delivery-harness` · `feat.no-self-graded-writes` ·
  `feat.no-silent-map-mutations` · `feat.sound-loop-floor` · `feat.transport-floor` ·
  `feat.stub-parking` — all `class: floor`.
- **Tier failed:** n/a — supersession by ruling (command-content-schema D2/D7 + the D10 rollout
  ruling 2026-08-26; D14; `DECISIONS.md` 2026-08-26 command-content-schema row).
- **Content:** the shipped v0.94.0 section, verbatim:

  ```
  ## Boundaries — the non-waivable floor

  - **Capability writes are sacred.** Minting, merging, retiring, or changing a capability's status
    happens only through `/mochiko:specify` or a user grooming ruling — never at the desk. The
    grooming-ruling door covers merge, retire, status change, and extent-tidying of **existing**
    entries; wholesale or from-scratch re-derivation is specify's derivation work — route to
    `/mochiko:specify`. When the user explicitly asks the desk to host work outside that remit,
    name the boundary crossing and serve with the home command's rituals imported — the door
    moves, the ritual never drops (adaptation rule: `mochiko:patterns-sound-loop`).
  - **Work rows are delivery bookkeeping the desk may cut** through the growth door: an extend verdict
    only, with acceptance criteria on the card. A fired architecture upgrade trigger enters by this
    same door and takes the same tests (Tools). Mint-or-uncertain routes to `/mochiko:specify`;
    several rows, a new UX surface, or cross-capability reach routes to specify regardless. The lane
    never widens in place — a mid-run outgrowth aborts and re-routes, and a report that lands on an
    `in-flight` capability's territory files to that run instead.
  - **No delivery harness at the desk — dispatch only.** Every admitted demand leaves as a
    `/mochiko:implement` capability-batch — growth-door rows in selection scope, a bug/improvement
    delta card in delta scope; the run owns the delivery, and the boundary is audited from the map
    delta the work leaves behind.
  - **No self-graded writes.** **No silent map mutations** — an integrity defect is fixed on sight, and
    every write is visible on the map.
  - **The sound-loop floor.** A judgment-authored write to a governing surface obliges the loop:
    a seat produces on a plan you approved, an independent non-author seat reviews before the
    user's gate — the user's ruling alone never substitutes for the review leg — and every desk
    delta card, bug and improvement alike, takes that review leg before dispatch. Trigger test,
    exemptions, seat wiring, and disclosure: `mochiko:patterns-sound-loop`, referenced never
    restated.
  - **The transport floor.** A visit that composes more than one seat gains a floor on its
    composition and messaging: a split trigger — message legs on any multi-seat messaging,
    topology legs on shared writes — non-waivable once triggered. Trigger test, floor legs,
    composition-safe shapes, and disclosure: `mochiko:patterns-transport-floor`, referenced
    never restated.
  - **Stub parking is parking, not a spec-bypass.** A parked capability hypothesis is a name plus a
    one-breath hook, marked `unrefined`; it earns selectability only through `/mochiko:specify`'s
    derivation, never here.
  ```

- **Kept deliberately:** every floor survives as a `class: floor` rule (must-survive under the
  pair audit, M3); the v0.70.0 grooming-door ceiling and out-of-remit adaptation
  (charter-ritual-balance D6), the sound-loop floor (charter-ritual-balance D1/D3), the growth
  door (pm-role-and-feature-derivation D8), and the v0.91.0 `/mochiko:implement` re-key
  (plan-stage-utility D1) survive with `ruling:` anchors. The first bullet splits at D12 grain
  into `feat.capability-writes-sacred` + `feat.grooming-door-ceiling` +
  `feat.out-of-remit-hosting`; the second into `feat.growth-door` +
  `feat.growth-routes-to-specify` + `feat.lane-never-widens`; the fourth into
  `feat.no-self-graded-writes` + `feat.no-silent-map-mutations` (splits, not drops). D15
  recasts, meaning unchanged: "outside that remit" names `feat.grooming-door-ceiling`; "takes
  the same tests (Tools)" names `feat.architecture-intake` + `feat.capability-write-test`;
  "never here" becomes "never at the desk".
- **Consumers assessed:** commands are entry points, nothing mounts them; the pointed-at skills
  (`mochiko:patterns-sound-loop`, `mochiko:patterns-transport-floor`) are untouched.

## [v0.95.0] Protocol defect line becomes `feat.fail.no-verdict` + the checker-guarded Not-done line

- **Disposition:** superseded → `plugins/mochiko/schemas/feature.yaml` rule
  `feat.fail.no-verdict` (section `feat.sec.fail-conditions`, labels `fail-condition`,
  `class: floor`), plus the `.md`'s new Not-done line in the checker-grepped phrase form —
  "the 1 rules labeled `fail-condition`" — with the out-of-sync halt clause (D7 C2 guard).
- **Tier failed:** n/a — supersession by ruling (command-content-schema D7 + the D10 rollout
  ruling 2026-08-26; `DECISIONS.md` 2026-08-26 command-content-schema row).
- **Content:** the shipped v0.94.0 line, verbatim: "**A visit that ends with no stated
  done-condition verdict is a defect.**" (the Adaptive Goal Protocol's closing line, after the
  `$ARGUMENTS` sentence).
- **Kept deliberately:** the rest of the Adaptive Goal Protocol — the three steps, the
  convergence contract, and the `$ARGUMENTS` line — stays in the `.md` verbatim (the per-visit
  charter contract, never recast as a per-run pipeline contract); Identity & Mission and the
  frontmatter stay verbatim. The `.md` gains the "## Rules — load the schema first" section
  naming the five `feat.sec.*` IDs — an addition riding the same ruling, no content displaced.
- **Consumers assessed:** commands are entry points, nothing mounts them; the pair audit's FAIL
  survival now keys to the `fail-condition` label set per
  `.claude/rules/mochiko/primitive-edits.md` (lead-owned this wave).

## [v0.91.0] Every `/mochiko:plan` dispatch site re-keyed to `/mochiko:implement` — plan-stage retirement D1/D6

- **Disposition:** superseded → the same six lines naming `/mochiko:implement`, the single
  downstream run after plan's retirement.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1 (plan retires; implement is the
  single downstream run) and D6 (implement's delta-scope entry gates on the desk-confirmed card
  directly; run-open absorbs the card-vs-entry confirmation the delta-scope plan run performed)).
- **Content (superseded fragments, verbatim — six sites):**

  1. Identity & Mission, the symmetry parenthetical:
     `` `/mochiko:plan`'s lead is already delivery manager of the goal ``
  2. Roles & Responsibilities, Other seats:
     `- **Dispatched runs** — all delivery. Plan and implement own their bounds, verification seats, and`
  3. Tools, the `mochiko:authoring-epic` entry:
     `` close) through it. A multi-feature epic dispatches to `/mochiko:plan` as one run over its ``
  4. Tools, Dispatch targets (two sites in one block):

     ```
     - **Dispatch targets** — `/mochiko:plan` for a dispatched capability-batch that scales itself, in
       the scope its door sets: **growth-door rows enter selection scope** (the batch covers the cut
       rows; its landing folds them into the capability's extent), a **bug/improvement delta card stays
       delta scope** (its landing is the delta fold) — the same split `/mochiko:plan` and
       `/mochiko:implement` name at their Entry.
     ```
  5. Ways of Working, Reference-never-restate:
     `` live in `/mochiko:plan` and `/mochiko:implement` and the skills they bind; the desk points at them ``
  6. Boundaries, no-delivery-harness:
     `` `/mochiko:plan` capability-batch — growth-door rows in selection scope, a bug/improvement delta ``

- **Kept deliberately:** the whole growth-vs-delta scope split (growth-door rows enter selection
  scope, bug/improvement delta cards stay delta scope), the landing semantics of each, the
  capability-write test, and the audited-from-the-map-delta boundary — D1 changes the dispatch
  target only. Site 4 gained the D6 clause naming the desk-confirmed card as implement's delta
  entry; site 4's "the same split … name at their Entry" narrowed to the singular "names at its
  Entry" because only one command now has an Entry.
- **Consumers assessed:** `implement.md` Entry (P1's rewrite carries both scope branches),
  `specify.md`'s next-step line and the router's entry-point + pipeline-consumption surfaces
  (all re-keyed same wave).

## [v0.81.0] Product-surface baseline list re-keyed — `nfrs.md` out, the store in (D12)

- **Disposition:** superseded → the list drops `nfrs.md` (dies whole under D12) and gains
  `architecture/`, the architecture store, with repo-root `ARCHITECTURE.md` named as its derived
  index rather than a standalone peer.
- **Tier failed:** n/a — supersession by ruling (record D12/D4; `DECISIONS.md` 2026-08-19 row).
- **Content:** verbatim — "**Product surface** — baselines at `.mochiko/product/`
  (`data-model.md`, `contracts/`, `nfrs.md`, `constraints-and-decisions.md`, `quickstart.md`)
  beside repo-root `ARCHITECTURE.md`".
- **Kept deliberately:** the product-lane home, the dated-append rule for cards and reports, and
  the overwrite-only-via-the-graded-fold rule for delta files all survive byte-for-byte.
- **Consumers assessed:** the same list re-keyed in `plan.md` (own strip entry, same wave); the
  desk's `baseline-delta.md` intake is unchanged — a store touch known at intake now names a
  store element, which the delta card's existing appliable before/after form already carries.

## [v0.81.0] Principal-architect dormancy line — cross-link added (D7); nothing removed

- **Disposition:** n/a — this entry records an **addition**, logged here only because the line
  it extends is a seat-duty claim an auditor would expect to find ruled. One sentence was
  appended naming the seat's standing home: "The seat is not idle elsewhere: its standing home
  is `/mochiko:architecture`, where it stewards the architecture store — the dormancy is this
  desk's, not the seat's."
- **Tier failed:** n/a — supersession by ruling (record D7 — the recharter of
  `principal-architect` as desk lead and store steward; `DECISIONS.md` 2026-08-19 row).
- **Content:** nothing removed. The original bullet survives byte-for-byte — "-
  **Principal-architect** — domain co-sign, **dormant until the first cap-trip**; no live duty
  on today's maps." — and the new sentence follows it.
- **Kept deliberately:** the dormancy itself, the cap-trip trigger, and the cap-trip co-sign
  duty — all unchanged, and the "no live duty on today's maps" absolute is untouched because it
  remains true *of this desk*. The addition exists so the sentence is not misread after this
  wave as a claim about a seat that now runs a desk of its own.
- **Consumers assessed:** `mochiko:patterns-map-minimalism`'s cap-trip co-sign (P4's file, same
  wave) — the co-sign duty is unmoved; the `principal-architect` recharter is P1's cluster.

## [v0.81.0] D13 growth-door intake — pure addition (no content left)

- **Disposition:** n/a — this entry records an **addition**, logged here only because auditors
  reading the desk's routing surface should find the ruling that put it there. A fired upgrade
  trigger from `/mochiko:architecture` arrives as a candidate capability or work row, is routed by
  the capability-write test, takes extend-beats-mint, and **the user rules the mint** — no new
  door, no auto-mint.
- **Tier failed:** n/a — supersession by ruling (record D13; `DECISIONS.md` 2026-08-19 row).
- **Content:** nothing removed. Two sites added: the **Architecture-desk intake** tool bullet, and
  one sentence in the growth-door Boundaries bullet cross-linking it.
- **Kept deliberately:** the growth door's existing tests are untouched and explicitly govern the
  new intake — extend verdict only, acceptance criteria on the card, mint-or-uncertain routing to
  `/mochiko:specify`, and the several-rows / new-UX-surface / cross-capability-reach routing.
  Capability writes stay sacred: D13 routes a *proposal* here, never a write.
- **Consumers assessed:** `commands/architecture.md` (new this wave) carries the dispatch half;
  `mochiko:patterns-map-minimalism` and `authoring-feature-map` are P4's this wave.

## [v0.78.0] Model-tiering floor line retargeted — `mochiko:explorer` superseded by native `Explore` + `model: haiku` override

- **Disposition:** superseded → the reworded floor line: locate/enumerate reads go to "a
  native `Explore` subagent spawned `model: haiku`".
- **Tier failed:** n/a — supersession by ruling (ADR
  `.mochiko/decisions/2026-08-19-explorer-retarget-native.md`; `DECISIONS.md` 2026-08-19
  row). Dogfood failure: agent-team teammates cannot spawn plugin-scoped agents.
- **Content:** verbatim superseded phrase: "the cheap explorer seat (`mochiko:explorer`)"
  (line wrap varies per command; only this phrase changed).
- **Kept deliberately:** the rest of the floor line byte-for-byte — the class-key summary,
  session-tier carve-outs, the every-seat-brief obligation, and the closing
  `mochiko:patterns-model-tiering` referenced-never-restated pointer.
- **Consumers assessed:** the same phrase edited in all six commands in the same v0.78.0
  wave (entry mirrored per command strip file); the pointed-at skill reworded in the same
  wave (`strips/patterns-model-tiering.md`).

## [v0.76.0] Static tasks-template read-pointer re-pointed to the CLI-render / raw-schema two-arm home
- **Disposition:** superseded → the Tools/Delta-cards `templates/tasks-template.md` read-pointer now
  names the two-arm guidance home: `mochiko-cli template tasks` when the binary is present, else the
  shipped schema `plugins/mochiko/schemas/tasks.yaml` Read raw — the raw Read is the D8-first-class
  path, not an error state. One template re-pointed: **tasks** (the delta card's card shape).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-16 "Template-schema CLI
  ruled (D1–D11 as amended at review)" row; record
  `.mochiko/brainstorms/schema-based-template-guidance/record.md`, **D1** (a single plugin CLI is
  the guidance authority, static `.md` exemplars retired) + **D8** (schemas ship as structured data
  files, the binary renders over them, raw Read is the first-class fallback); build plan §5 re-point
  inventory)
- **Content (superseded, verbatim — the read-pointer that left):**
  - Delta cards: "one card per `templates/tasks-template.md`'s card shape"
- **Kept deliberately:** the whole Delta-cards responsibility — a bug's reproduction-failing-test
  acceptance, an improvement's 1–3 acceptance criteria, the minimal enumerated `baseline-delta.md`
  in appliable before/after form, and the per-card sound-loop review leg — all untouched. Only the
  template-source token changed. `templates/output-style.md` (Register) is out of the 8-template set
  and unchanged; the `templates/tasks-template.md` / `templates/feature-entry-template.md` references
  inside this file's frozen v0.68.0 verbatim-superseded-file archive are history, not live pointers,
  and are not touched.
- **Consumers assessed:** none — commands are entry points, nothing mounts them. Co-edited this wave
  under the same ruling: the 8 `plugins/mochiko/templates/<t>.md` deletions + their supersession
  strips (P3); the sibling command re-points `specify.md` / `plan.md` / `setup.md` (own strips); the
  skill/reference re-points + D7 re-key (P5, own strips). The named `plugins/mochiko/schemas/tasks.yaml`
  file is authored by the schemas seat (P1) this wave — the path is fixed contract per the approved
  build plan, not created here.

## [v0.70.0] Unbounded "user grooming ruling" door → D6 ceiling (Boundaries, capability writes)
- **Disposition:** superseded → the same bullet, extended in place: the grooming-ruling door now
  covers merge, retire, status change, and extent-tidying of **existing** entries; wholesale or
  from-scratch re-derivation routes to `/mochiko:specify`; explicit-user out-of-remit hosting
  names the boundary crossing and imports the home command's rituals (adaptation rule carried by
  `mochiko:patterns-sound-loop`).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-13 "Charter ritual
  balance ruled" row, D6; record `.mochiko/brainstorms/charter-ritual-balance/record.md` D6 —
  driver F4: the kinako whole-map re-derivation passed through this door legally, the sole check
  one user "adopt", no review leg beneath it).
- **Content:** the v0.68.0 bullet, verbatim:

  ```
  - **Capability writes are sacred.** Minting, merging, retiring, or changing a capability's status
    happens only through `/mochiko:specify` or a user grooming ruling — never at the desk.
  ```

  What dies is the unbounded reading — any capability write, at any scale, legal through a single
  user grooming ruling. Mint never enters the enumerated door (the ceiling's list is D6.1's,
  existing entries only).
- **Kept deliberately:** the first sentence survives verbatim as the bullet's opening —
  capability writes stay sacred, the two doors (specify · user grooming ruling) stay the only
  doors; the kinako route is recorded out-of-bounds under the ceiling but its adopted outcome
  stands ratified — nothing rolls back (D6.2).
- **Consumers assessed:** grep of `plugins/mochiko/` for the door's phrasing — the router's
  `/mochiko:feature` row (stewardship: view/query, park, retire, integrity grooming, cap-trip
  merge/retire proposals) stays true under the ceiling, no re-derivation claim to repair;
  `specify.md`'s reciprocal front-door wording gains scope (re-derivation routes to it) with no
  text change owed; `plan.md`/`implement.md` never reference the grooming door.

## [v0.70.0] Unscoped staffing freedom → "below the sound-loop floor" narrowing (Roles & Responsibilities)
- **Disposition:** superseded → the same sentence, narrowed in place: per-visit
  staffing/sequencing/run freedom now scoped "below the sound-loop floor"
  (`mochiko:patterns-sound-loop`, pointed at from Boundaries); when the floor's trigger fires,
  the lead-absorbs-the-seat reading dies — a seat produces on an approved plan, an independent
  non-author seat reviews, the user rules.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-13 "Charter ritual
  balance ruled" row, D1/D3; record `.mochiko/brainstorms/charter-ritual-balance/record.md` D1 —
  "the *lead-absorbs-the-seat* reading of 'how you staff … is yours to shape' dies when the D2
  trigger fires; that clause takes a recorded narrowing at build").
- **Content:** the v0.68.0 paragraph, verbatim:

  ```
  There is **no Bindings section**. The bare minimum that must always happen is carried here as the
  Delivery Manager's owned responsibilities; everything beyond it is your per-visit judgment — how you
  staff, sequence, and run the visit is yours to shape (the lead-owned-process-flexibility posture,
  applied to a standing desk).
  ```

- **Kept deliberately:** everything but the scoping — the no-Bindings rule stands (D4: no
  Bindings section returns), the DM floor stands, and the lead-owned-process-flexibility posture
  itself stays chartered below the floor (D1: "when the trigger does not fire, the lead's inline
  freedom stands as chartered"); transport neutrality (teammate vs subagent per seat) untouched
  at the transport level (F6, `command-architecture-realignment` D5).
- **Consumers assessed:** the sibling staffing clauses in `plan.md` ("how you staff, sequence,
  and run the seats is yours to shape") and `implement.md` ("…run the cycles…") are deliberately
  NOT narrowed this wave — the record's build surface gives those charters pointer lines only
  ("their run shapes already satisfy the floor; the pointer makes it doctrine, not habit"); the
  router's lead-orchestration line stays true at the transport level.

## [v0.68.0] v8 Goal · Harness · Bindings anatomy → six-section charter (the product desk)
- **Disposition:** superseded → the six-section charter that now IS `commands/feature.md`
  (Identity & Mission · Adaptive Goal Protocol · Roles & Responsibilities · Tools · Ways of Working
  · Boundaries). The v8 default-FAIL-goal anatomy is replaced whole; the audit re-keys to grade
  *floor present + per-visit-goal contract present* in place of *default-FAIL goal present*.
- **Tier failed:** n/a — supersession by ruling (record D10; `DECISIONS.md` 2026-08-13 — v8 shape
  superseded **this command only**, the other five commands stay v8; uniformity + churn costs on
  record at the record's I7 fold).
- **Content:** the entire pre-charter `commands/feature.md`, verbatim:

  ```
  ---
  description: Front door to the feature map — steward entries, triage bugs and improvements by the stable-ground test, author the delta card, and dispatch delivery to the re-keyed pipeline.
  disable-model-invocation: true
  ---

  # Feature — Map Stewardship & Delivery Lane

  **Goal:** resolve `$ARGUMENTS` (a map query, a capability idea to park, a promotion or
  retire ask, or a bug/improvement report) through exactly one of the command's remits —
  map stewardship, or lane intake ending in a dispatch. Empty → ask the user what they need.

  ## Goal

  The request landed in its remit. **Stewardship:** a map query answered from the actual
  files — `FEATURES.md` plus the entries in the territory asked about, never memory of
  them · a capability idea parked as a `proposed` stub — name + one-breath hook only,
  marked `unrefined`; a stub is parking, never a spec-bypass — selectability stays behind
  `/mochiko:specify`'s derivation · a flat entry retroactively promoted to parent — the
  delivered extent becomes the first child, new work lands as sibling children, status
  never regresses · a retire executed on the user's ruling, entry kept and dated · any
  stewardship touch on a parent (query, promotion, grooming) re-surfaces that parent's
  parked stubs and undelivered leaves · an integrity defect fixed on sight. **Lane intake:** the report triaged by the
  stable-ground test — the lane writes only surfaces no live run owns — into the feature
  lane, the product lane, or filed to the owning run; lane work captured as **one delta
  card** — a bug's acceptance is its reproduction-failing test, an improvement carries 1–3
  acceptance criteria — plus the minimal enumerated `baseline-delta.md` (appliable
  before/after form) when a product-baseline touch is known at intake; the card handed to
  the re-keyed pipeline as **delta scope**, where it executes under plan/implement's own
  bounds, verification seats, and evidence rules — this command runs no delivery harness.
  The map delta the work leaves behind is what the boundary is audited from.

  **Not done — default FAIL:** a stub minted with extent or relations filled, or missing
  the `unrefined` mark · lane work that mints an entry, promotes to parent, or flips
  status — the map-write test failed; it routes to `/mochiko:specify` · a mid-run
  outgrowth widened in place instead of aborted and re-routed · a report keyed to an
  in-flight feature's surface run as lane work instead of filed to the owning run · a
  second live product-lane run · a known baseline touch with no `baseline-delta.md`
  authored at intake · any bounds, verification, or evidence discipline restated here
  instead of referenced · a retire, or a promotion on an ambiguous case, executed without
  the user's ruling.

  ## Harness

  - **You are the lead.** Plan the run and orchestrate it toward the Goal; teammates or
    subagents per seat is your call. Stewardship writes are bookkeeping edits on the live
    map; lane delivery is never yours — it belongs to the dispatched run.
  - **Triage — the stable-ground test.** Key the report to its surface, then check the
    ground. The check inputs are **files**: entry status at the feature level; the
    in-flight feature dirs' enumerated baseline deltas at the product level. Keying a raw
    report to its surface is triage judgment, audited from the resulting delta — never
    claimed mechanical. Three branches: single owning feature `delivered` → feature lane,
    delta card on the entry · single owning feature `in-flight` → not lane work; the
    finding files to the owning run, whose verification and regression gates own that
    territory · no single owner → product lane, keyed to the `.mochiko/product/` baselines
    and `ARCHITECTURE.md`, under the same test at the product surface — a baseline surface
    under active delta by an in-flight run files to that run instead. The product lane is
    **single-flight**: one live product-lane run at a time.
  - **Lane boundary — the map-write test.** The lane is allowed only when the work needs
    **no new map entry and no status change** — a pure marked delta on an existing
    feature. Anything that would mint, promote, or flip routes to `/mochiko:specify`.
    Mid-run discovery that the work outgrew the lane **aborts and re-routes** — the lane
    never widens in place; the product lane the same — mid-fix discovery that the run
    stands on an in-flight feature's territory files the finding to that run and aborts.
    The boundary is graded from the map delta by the dispatched run's verification seat —
    no new seat here, never a self-declared "small".
  - **Dispatch — reference, never restate.** Hand the delta card to `/mochiko:plan` /
    `/mochiko:implement`: the run gates on a feature entry carrying ratified scope — the
    scope source is a spec's accepted Feature Selection or a feature-command delta card.
    That gate, the bounds, the verification seats, and the evidence rules live in those
    commands and the craft skills they bind; this command points at them and adds nothing.
  - **Independence:** where a producing seat exists — delta-card or `baseline-delta.md`
    authoring — no output is cleared by its author; any grading reads the files
    themselves, default FAIL. Plan approval: a seat that writes artifacts plans first and
    works only on a plan you approved.
  - **Reserved to the user:** retire rulings · promotion on ambiguous cases ·
    lane-vs-specify routing when triage is genuinely borderline · parent selection
    semantics — unruled; surfaced when it bites, never defaulted here.
  - Suggest commits; never run git mutations, never push. User rulings are plain blocking
    text, never a timed prompt.

  ## Bindings

  - **Map machinery:** entry shape, parent/leaf nesting, delta grammar, integrity
    invariants, and the `unrefined` mark per `mochiko:authoring-feature-map` and
    `templates/feature-entry-template.md`, never restated. Entry files at
    `.mochiko/features/FEAT-XXX-<slug>.md`; per-feature run artifacts at
    `.mochiko/features/FEAT-XXX/`.
  - **Product surface:** baselines at `.mochiko/product/` — `data-model.md` ·
    `contracts/` · `nfrs.md` · `constraints-and-decisions.md` · `quickstart.md` — with
    `ARCHITECTURE.md` at repo root. Product-lane runs at `.mochiko/product/lane-<slug>/`
    (card + reports + `baseline-delta.md`). Across repeat lane runs, cards and reports
    append (dated); delta files overwrite only via the graded fold.
  - **Delta card:** one cycle-card-shaped unit per `templates/tasks-template.md`'s card
    shape. `baseline-delta.md` in appliable before/after form; a touch discovered mid-fix
    is authored by the dispatched run, not retro-authored here.
  - **Scope types:** `delta scope` — landing is the feature-map delta fold · `selection
    scope` — landing is the graduation batch. The lane dispatches delta scope only;
    landings belong to the dispatched run.
  - **Lane liveness:** every `in-flight` status or delta points at an open spec or a live
    lane run — live from dispatch until its acceptance landing; a delta whose lane run
    ended without folding is a defect, fix-on-sight (invariant home:
    `mochiko:authoring-feature-map`).
  - **KM relation:** where `.mochiko/memory/knowledge-management.md` exists, `BACKLOG.md`
    is the defect queue — a reported bug is a BACKLOG item until a lane run picks it up —
    and lane acceptance is a landing event, same ritual home as spec and implement
    acceptance. Without KM: no queue — lane runs accept direct requests; that is the
    stated degrade path, never silently assumed.
  - **Register:** user-facing prose per `templates/output-style.md`.
  - **Next step:** `/mochiko:plan` for a dispatched delta scope (the pipeline scales
    itself); `/mochiko:specify` for anything the map-write test routes out.
  ```

- **Kept deliberately:** everything the v8 body carried that the ruling did not kill survives,
  re-homed into the charter and re-typed feature→capability / leaf→work-row where D2/D6 re-typed
  the map. The exhaustive re-home map (so no survivor reads as a silent drop):
  - *stable-ground triage* (the three branches + "keying a raw report to its surface is triage
    judgment, audited from the resulting delta, never claimed mechanical") → **Tools** (branches,
    re-typed to capability) + **Roles & Responsibilities** ("route honestly … audited from the
    resulting map delta, never claimed mechanical"). Desk craft, no skill home — carried, not
    referenced away.
  - *lane intake / delta card / `baseline-delta.md` appliable form* → **Tools** (Delta cards) +
    **Roles & Responsibilities** ("hand every dispatched run a complete card").
  - *a baseline touch discovered mid-fix is authored by the dispatched run, not retro-authored at
    the desk* → survives implicitly at **Tools** (Delta cards — `baseline-delta.md` authored only
    "when a product-baseline touch is **known at intake**") + **Boundaries** (dispatch only): the
    desk authors only the intake-known delta, so any touch surfaced mid-fix falls to the dispatched
    run by the intake-scoping + no-delivery-harness lines — preserved, not restated.
  - *stub parking* ("`proposed` stub, name + one-breath hook, `unrefined`, never a spec-bypass,
    selectability behind `/mochiko:specify`") → **Boundaries** (re-typed to *parked capability
    hypothesis*).
  - *retire-by-ruling* (entry kept and dated) → **Boundaries** (capability writes sacred: retire
    via user ruling) + **Roles & Responsibilities** (the user: retire and merge rulings).
  - *re-surfacing on a stewardship touch* ("re-surfaces that parent's parked stubs and undelivered
    leaves") → **Adaptive Goal Protocol** (the health report, opening every visit) — re-typed:
    parent→capability, undelivered leaves→undelivered pending rows.
  - *integrity fix-on-sight* → **Roles & Responsibilities** (keep map integrity intact at close) +
    **Boundaries** (no silent map mutations).
  - *single-flight product lane* → **Ways of Working** + **Tools** (product surface).
  - *author ≠ grader + plan-approval for producing seats* → **Ways of Working** + **Boundaries**
    (no self-graded writes).
  - *decisions reserved to the user* → **Roles & Responsibilities** (the user: retire/merge
    rulings, route overrides, selections).
  - *product baselines / `.mochiko/product/` machinery* → **Tools** (Product surface) — D7 scopes
    baselines in untouched; folds fire at the same acceptance landings.
  - *KM relation* (BACKLOG defect queue, lane acceptance a landing event, no-KM degrade path) →
    **Roles & Responsibilities** (execute the KM landing where KM exists) + **Tools** (Dispatch
    targets).
  - *"reference, never restate"* + *register* + *commits-not-push* + *rulings-are-plain-text* →
    **Ways of Working** + **Tools** (Register).
  - *dispatch; no delivery harness; boundary audited from the map delta* → **Tools** (Dispatch
    targets) + **Boundaries** (dispatch only). The v8 "the lane dispatches delta scope only" clause
    is superseded further by record D8's **Build-time rider** (user-ratified 2026-08-13 at the
    v0.68.0 build — `.mochiko/brainstorms/pm-role-and-feature-derivation/record.md` D8, and the
    `DECISIONS.md` 2026-08-13 pm-role row's Build-rider annotation): the desk now
    dispatches **both** scopes — the **growth door → selection scope** (the capability-batch covers
    the cut rows; its landing folds them into the capability's extent), a **bug/improvement card →
    delta scope** (its landing is the delta fold) — both as `/mochiko:plan` capability-batches, the
    split matching plan/implement's Entry verbatim. *selection scope* is no longer specify's alone.
  - *lead-owned process flexibility* ("teammates or subagents per seat is your call") → **Roles &
    Responsibilities** ("everything beyond the minimum is your per-visit judgment").
  The two protected clauses this ruling KILLS or RE-KEYS are recorded discretely below (parent/leaf
  nesting → the v0.68.0 nesting-death entry; the map-write test → the v0.68.0 test-re-key entry) so
  neither reads as a silent drop.
- **Consumers assessed:** grep of `plugins/mochiko/` for `feature.md` references and the routing
  surface — the `mochiko` router (`skills/mochiko/SKILL.md`, which indexes `/mochiko:feature` and
  names the stable-ground test), `commands/specify.md` (the capability-write boundary's other
  door), `commands/plan.md` and `commands/implement.md` (the dispatch targets whose entry gates on
  a delta card). Router index text and the plan/implement entry-condition wording are downstream
  ripple owned by the wave's router + pipeline seats, not this strip.

## [v0.68.0] Parent/leaf nesting + retroactive-promotion remit — superseded, dies
- **Disposition:** superseded → nothing; the two-level parent/leaf nesting shipped by
  `feature-sizing-and-entry-points` D2–D4 dies (record D6). What survives of the two-tier idea is
  re-typed as capability + transient work rows (owned by `authoring-feature-map` + the templates,
  other seats' strips). At the desk, the *retroactive-promotion* remit and *parent-selection*
  semantics have no successor — the charter carries no promotion door.
- **Tier failed:** n/a — supersession by ruling (record D6, firmed `Confident` by user word at the
  I9 fold; `DECISIONS.md` 2026-08-13). Full anatomy context: the v0.68.0 charter-reshape entry
  above.
- **Content:** the v8 clauses that named parent/leaf promotion, verbatim:
  - Goal: "a flat entry retroactively promoted to parent — the delivered extent becomes the first
    child, new work lands as sibling children, status never regresses".
  - Goal / re-surfacing: "any stewardship touch on a parent (query, promotion, grooming)
    re-surfaces that parent's parked stubs and undelivered leaves" — the *promotion* trigger and
    *parent* framing die; the re-surfacing obligation itself survives, re-typed onto the capability
    health report (see the charter-reshape entry's Kept-deliberately).
  - Not done: "lane work that mints an entry, **promotes to parent**, or flips status" — the
    promote branch dies; mint / status-flip survive re-keyed onto the capability-write test.
  - Harness / Reserved to the user: "promotion on ambiguous cases · … · parent selection
    semantics — unruled; surfaced when it bites, never defaulted here".
  - Bindings / Map machinery: "parent/leaf nesting" (in "entry shape, parent/leaf nesting, delta
    grammar, integrity invariants").
- **Kept deliberately:** retire-by-ruling survives (Boundaries + the user's rulings); mint and
  status-flip survive as capability-write-test routes to specify; the re-surfacing obligation
  survives as the health report. Only the *promotion / parent-leaf* machinery leaves.
- **Consumers assessed:** as above — `authoring-feature-map`, `feature-entry-template.md`,
  `features-index-template.md`, `plan.md`, `implement.md` carry the map's own parent/leaf → work-row
  re-type under the same D6 inventory; those are their seats' strips, not this command's.

## [v0.68.0] "Map-write test" lane boundary → "capability-write test" re-key
- **Disposition:** superseded → the **capability-write test** (record D8): the lane boundary
  re-keys from "no map write" to "no *capability* write" — capabilities (mint, merge, retire,
  capability-status) are the sacred layer routed to specify or a user grooming ruling; work rows are
  delivery bookkeeping the desk may cut through the growth door. The instrument's name and phrasing
  change; its job (route capability truth out, keep row bookkeeping in) is now in **Tools**
  (Capability-write test) and **Boundaries**.
- **Tier failed:** n/a — supersession by ruling (record D8; `DECISIONS.md` 2026-08-13). Full
  anatomy context: the v0.68.0 charter-reshape entry above.
- **Content:** the v8 "map-write test" phrasings, verbatim:
  - Harness / Lane boundary: "**Lane boundary — the map-write test.** The lane is allowed only when
    the work needs **no new map entry and no status change** — a pure marked delta on an existing
    feature. Anything that would mint, promote, or flip routes to `/mochiko:specify`."
  - Not done: "lane work that mints an entry, promotes to parent, or flips status — **the map-write
    test failed**; it routes to `/mochiko:specify`".
  - Bindings / Next step: "`/mochiko:specify` for anything **the map-write test** routes out".
- **Kept deliberately:** abort-and-reroute ("the lane never widens in place — a mid-run outgrowth
  aborts and re-routes") and "the boundary is audited from the map delta" survive, re-homed to
  **Boundaries**; the growth door (D8) is the new affordance the re-key opens — an extent-growth
  verdict cuts work rows and dispatches a capability-batch rather than routing out.
- **Consumers assessed:** as above — the router's boundary phrasing and specify's reciprocal
  "front door" wording are the router + specify seats' ripple.
