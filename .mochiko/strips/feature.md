# Strip notes — `commands/feature.md`

Entry formats: `strips/README.md`. Ruling for every [v0.68.0] entry below:
`DECISIONS.md` 2026-08-13 "PM role & feature derivation" row →
`.mochiko/brainstorms/pm-role-and-feature-derivation/record.md` (D6/D7/D8/D9/D10).

## [v0.74.0] Static tasks-template read-pointer re-pointed to the CLI-render / raw-schema two-arm home
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
