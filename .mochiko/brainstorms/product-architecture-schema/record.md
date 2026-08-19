# Product-Level Architecture as Schema — Decision Record

**Status:** accepted (2026-08-19 — user word "accept", following verify round 2 CLEAN)
**Opened:** 2026-08-19
**Session:** collaborative brainstorm via `mochiko:analysis-iterative` (one question per turn)

## Topic

Define what product-level architecture comprises for mochiko's target segment (SaaS,
product-led-growth startups) and express it as a schema. Today's architecture surfaces are
user-story / feature driven — not wrong, but missing a product-level lens. Threads:

1. What product-level architecture comprises — and whether/how it can be defined as a schema.
2. An opinionated architecture posture: common patterns for SaaS/PLG products.
3. How the product lens and the feature lens interact — agents creating the necessary
   push and pull between them.
4. Adaptive depth: architecture depth must scale with project and repo scope.

## Ground facts

*(fact-finder sweep 2026-08-19, Explore seat, read-only; paths verified by the seat)*

- **F1 — per-feature architecture is altitude-governed, plan-time, delta-shaped.**
  `patterns-system-design` produces `architecture.md`: container-level mermaid flowchart delta
  (~12-node scope bound), sequence diagrams for qualifying flows, component register
  (name — responsibility — boundary — status) with D-XXX-linked delta summary, deployment view
  conditional on IP-XXX rows. Strict C4-container altitude; governs *form*, explicitly not
  *amount* (amount belongs to `patterns-plan-minimalism`).
- **F2 — repo `ARCHITECTURE.md` is current-state-only prose.** `authoring-architecture`: present
  tense, no rationale; sections named in prose (Components · Data flow · External integrations ·
  In-flight pointers), not schema-prescribed. Updates only on structural change (landing diff +
  fold).
- **F3 — the architect persona is explicitly anti-default.** `principal-architect` carries
  judgment axes (boundary cuts, sync-vs-async as decision, single-home responsibility,
  refuse-unpaid-structure) but names no SaaS pattern, tenancy model, or deployment shape.
- **F4 — no architecture schema exists.** Eight schema files ship at `plugins/mochiko/schemas/`
  (spec, plan, tasks, feature-entry, features-index, codebase-analysis, governance-intent,
  governance-surfaces); both architecture artifacts are skill-prose-structured only.
- **F5 — architecture enters the pipeline only through `plan.md`, conditionally.** Included
  when the plan-the-plan proposal includes it ("An omit-architecture proposal → no sign-off, no
  pointer, no close-diff owed"); user signs off on a rendered diagram before detailed design;
  `implement.md` stops on diagram deviation. `specify.md`/`setup.md` do no architecture work.
  *Path inconsistency flagged:* `plan.md` homes package artifacts at
  `.mochiko/features/FEAT-XXX/` while `patterns-system-design` says the feature's spec dir and
  `templates/architect-report-template.md:58` says `.mochiko/specs/<feature>/architect-report.md`.
- **F6 — no adaptive depth on architecture.** The low/high depth dial lives entirely in the
  governance/constitution cluster; neither architecture skill carries any depth-dial machinery
  *(precision repair at review, Y6: the literal word "depth" does occur twice in unrelated
  senses — `patterns-system-design:22` "ADR depth", `authoring-architecture:51` "depth lives in
  the feature artifacts"; the latter is itself a shipped depth-allocation rule that D3/D4
  inverts, noted for the supersession list)*. Nearest scaling machinery: plan-minimalism
  ladder + the ~12-node diagram bound.
- **F7 (corrected at review — original was false on auth/observability) — opinionated
  defaults today: three backend cards arbitrated, PLUS two floor-asserted category
  obligations.** BE-HEX / BE-SRP / BE-DEP in the constitution catalog, dealt
  recommend-then-arbitrate, never asserted (PO-D3 S7 arch-opinion carve-out);
  frontend/mobile/desktop shelves are declared Tier-I gaps. **However** the universal floor
  asserts two architecture-adjacent categories at both depth levels: FLOOR-SEC
  ("auth enforced at all boundaries", waiver posture: narrowing over dropping) and FLOOR-OBS
  (critical-path logs, no PII) — floor-asserted, dial-keyed, loosenable only by recorded
  ledger waiver; `patterns-adopt-first` additionally names auth a commodity category. No
  multi-tenancy, billing, analytics, or feature-flag default exists; Stripe/Auth0 appear only
  as illustrative examples. *(Original F7 text superseded by review finding S2 →
  D14.)*

- **F8 — blast radius: 26 load-bearing files** (fact-finder sweep 2, exhaustive, line-keyed
  inventory held in session; summary here — *re-ordered after F7 at verify, N6*). Owners:
  `patterns-system-design` (+ conventions ref), `authoring-architecture`,
  `principal-architect`, `architect-report-template`, plugin manifest. Commands: `plan.md`
  (heaviest — proposal conditionality, sign-off gate, In-flight pointer, Boundaries floor),
  `implement.md` (deviation gate, dual-target fold, In-flight clear), `setup.md:103`,
  `feature.md` (dormant co-sign at L70; the `.mochiko/product/` Product-surface block at
  L106-109 — *key corrected at review, Y6: the original sweep put the co-sign at L107, which
  is actually the baseline-layer line; that mislabel is the mechanical origin of the review's
  C1*). Schemas: `plan.yaml` (conditional Architecture section + check string — its check
  string is at `plan.yaml:40`), `feature-entry.yaml` (component pointers),
  `features-index.yaml` (peer-of lines). Review: `review-plan-artifacts`
  (ARTIFACT-CHECKLISTS.md L100-133 full graded Architecture checklist + conformance rows),
  `review-feasibility` (architecture pass named in its `description:` — affects
  model-invocation triggering). Cross-refs: router ×6 rows, `patterns-sound-loop` (governing
  surface + seat-wiring row), `authoring-feature-map`, `patterns-map-minimalism` (cap-trip
  co-sign), `authoring-technical-requirements` (D-XXX `Origin: structural` cites
  `patterns-system-design`), `patterns-plan/code-minimalism`, `authoring-epic:57` (joint
  architecture + seam design), KM constitution module. Pinned: `.claude/rules/mochiko/
  operating-docs.md` paths glob · `.mochiko/memory/knowledge-management.md` **In-flight
  agreement invariant (AT-D6-C, project-pinned — touching it is a landing)** · cost-budget
  rows (authoring-architecture 5,250 · patterns-system-design 8,837 · principal-architect 756)
  · governance-ledger GI-018 version-lag acceptance.

## Decisions

- **D1 — Product architecture becomes a first-class workflow: its own living-desk command.**
  Baseline authored once (greenfield elicits; brownfield derives from codebase analysis),
  revisited any time; features contest against it and fold proven deltas back at landings.
  *Rationale:* cross-cutting product concerns (auth, logging, billing) have no home in the
  story/feature-driven flow — felt directly in the kinako dogfood; a plan-time-only,
  per-feature entry point can never answer "where do I say something about billing."
  **Confident**

- **D2 — Schema shape: two layers.** A topology spine (containers, boundaries, communication
  styles) plus a concern catalog (AX-XXX rows); a concern row may pin to a structural element
  or apply product-wide. *Rationale:* "say something about auth/billing" is catalog language,
  but concerns without structural homes float — the layers need each other. **Confident**

- **D3 — One schema-backed architecture store; the per-feature architecture artifact dies.**
  Blank-slate ruling, breaking change accepted. Every element (container, arrow, concern row)
  carries lifecycle `ruled` → `in-flight (FEAT-XXX)` → `built`; intent and built-state live on
  one surface. Pipeline re-keys: plan **always** consults the store (the omit-architecture
  escape dies); a feature touching an `open`/`not-now` row trips a user ruling; implement's
  deviation gate points at the store; landings flip elements `built`. *Rationale (corrected
  at verify, N1 — original said "three-surface split" and overclaimed the diff kill):* the
  current **five-surface** split (per-feature deltas · prose ARCHITECTURE.md · conditional
  plan inclusion · `nfrs.md` · `constraints-and-decisions.md`, the last two product-scoped —
  see D12) is exactly where product-level concerns fall through; one store with statuses
  kills the **artifact** diff (intent and built-state on one surface) — the code-vs-claim
  diff remains and is the scoped drift probe's job (D7 fold). **Confident**

- **D4 — Store file structure: root index over two levels.** *(Paths restated under D12 at
  verify — B1: the store homes at `.mochiko/product/architecture/`, not the originally
  sketched `.mochiko/architecture/`.)* Repo-root `ARCHITECTURE.md` **stays at the repo root**
  as a pure index/TOC (spine thumbnail + AX summary table + links) — a **derived projection
  of the store, not a second store**: the store is the single source, the index its rendered
  view (KM home and the top-level operating-doc reservation untouched);
  `.mochiko/product/architecture/spine.md` holds the topology deep view (the existing
  container/sequence diagram craft is kept); concern rows start compact in a `concerns.md`
  ledger and graduate to per-concern files only on real depth — extend-beats-mint.
  *Rationale:* root stays scannable (user requirement: index/table-of-contents), depth one hop
  down; exhaustive shelves make per-row files at birth too heavy. **Confident**
  *Review fold (S12, batch):* the root index is **derived, never hand-maintained** — the
  store skill regenerates it on every store write (single writer); index-vs-ledger
  disagreement is a defect fixed on sight. This closes the stale-index-misses-a-trip failure
  mode.

- **D5 — Opinionated shelves, exhaustive, per surface, dealt never asserted.** Shelf per
  surface type (backend-service · frontend-web · mobile · desktop), each native-deep and
  deliberately exhaustive (well past 12 rows is fine); rows carry suggested SaaS/PLG defaults
  and upgrade-trigger patterns; dealt recommend-then-arbitrate (PO-D3 S7 arch-opinion
  precedent) — the user forms a stance per row. Scope comes from setup, **overridable at the
  desk**; full-stack/monorepo composes shelves. **Breadth invariant:** every shelf row is
  walked; a row may close in two seconds but is never silently skipped. The 13-dimension
  backend shelf + 3 topology-spine opinions (modular monolith first · one tenant-scoped
  Postgres · queue before event bus) accepted as the backend baseline; frontend sample
  indicative. *Rationale:* the value is breadth — cheap to say "not a concern", expensive to
  never have asked. **Assumed** *(re-marked at review, S4: approved "Looks right" at the
  dimension level, content untested; was Confident)*

- **D6 — Row stance vocabulary.** `decided` · `not-now` (real concern, consciously deferred,
  optional revisit trigger) · `n-a` (with reason axis: *genuinely never* **or**
  *handled elsewhere* — another repo/system owns it, pointer optional) · `open` (walked past,
  no stance; health view counts these). The Q8 row sketch (Ruling / Rationale / Upgrade
  trigger / As-built / Drift / Work pointers) is illustrative, not the final schema.
  *Rationale:* `not-now` rows are the PLG time bombs and deserve triggers; `n-a` is permanent
  dismissal and deserves a why. **Confident**
  *Review folds (batch):* (Y1) `n-a — handled elsewhere` **requires** its pointer;
  optional only for *genuinely never*. (D14 interaction) where a floor card asserts the
  category, `n-a — genuinely never` is unavailable (see D14).

- **D7 — Crew.** Skills: new `authoring-architecture-store` (store owner: AX/spine grammar,
  lifecycle, graduation, fold-at-landing, health view; retires `authoring-architecture`) ·
  new `patterns-architecture-shelves` (opinion carrier; shelves as data files, constitution-
  catalog precedent) · `patterns-system-design` survives transformed (altitude + diagram
  discipline serving store deltas). Agents: `principal-architect` recharters as desk lead /
  store steward (shelf walks, plan-time contest from the store baseline, delta authoring);
  **drift becomes an empirical duty** — desk visits spawn a codebase probe grading `As-built:`
  claims against actual code; `tech-lead` grades store writes (author≠grader holds); no new
  persona. *Rationale:* opinions live in shelf data, judgment stays in the persona — resolves
  the F3 anti-default tension without corrupting the architect's contest posture. **Assumed**
  *(re-marked at review, S4: adopted in one word ("Stand."), includes a shipped-skill
  retirement; was Confident)*
  *Review folds (S10 + S7, batch):* the **drift probe is scoped** — rows touched
  since the last desk visit plus a sample of the retrofit-expensive rows, never all rows every
  visit; findings land in the row's `Drift:` field and take a user disposition at the desk.
  D3's "kills the intent-vs-built diff problem" is restated precisely: one store kills the
  *artifact* diff; the *code-vs-claim* diff remains and is exactly what the scoped probe
  exists for. The **shelf walk is ordered by retrofit cost** (tenancy / auth / data
  partitioning first; flags / experimentation late), and the no-magnitude-scaling choice is
  deliberate: a two-person MVP and a mature platform walk the same shelf — per-row stances
  are the magnitude valve, with the steelman (fact-triggered rows, visible unwalked list)
  recorded and declined for silently narrowing breadth.

- **D8 — Schema carrier.** Store shape ships as schema data file(s) at
  `plugins/mochiko/schemas/` (ninth schema-backed artifact family), shelves as data beside
  them; `mochiko-cli` renders guidance; raw Read stays the degraded path (GI-020; the
  kernel-class bright line of the `schema-based-template-guidance` record's D11 — *not this
  record's D11* — untouched; pointer disambiguated at review, Y7). Shape principle:
  **small required core + broad flexibility** — the schema constrains the skeleton, never the
  voice. **Confident**
  *Review fold (Y8):* the store schema's `--check` validator view grades the skeleton
  only — required core fields present, status values legal, id uniqueness, dangling
  `Work:`/NFR pointers — never the free-form body; advisory exit-code signal, non-gating,
  per the standing bright line.

- **D9 — No coupling to the governance depth dial.** The architecture desk never reads the
  low/high production-floor level; adaptiveness comes from scope-selected shelves + per-row
  stances (`not-now` is the per-dimension depth valve); a low-depth project still walks the
  full shelf. *Rationale:* one dial must not silently drive two systems. **Confident**

- **D10 — Plan-time contract: consult always, author on structural change; six-step delta
  lifecycle.** Every plan run reads the store (root index + full AX summary table — the trip
  check runs there — + touched concern files only; spine deep view **only on the
  structural-change trigger**, per the S8 fold below — statement amended at verify, B2) and
  the full feature spec;
  never other features' plan packages. A delta is authored only when the feature changes
  structure: drafted in the plan package (store untouched) → user sign-off on rendered diagram
  + named AX-row changes is the **write gate** → signed delta lands as `in-flight` /
  `modifying (FEAT-XXX)` / `removing (FEAT-XXX)` elements → implement's deviation gate points
  at the store (differ = stop; build as approved or amend by user ruling) → landing flips
  `built`, updates `As-built:` + drift on touched rows, clears keys → **orphan rule**: every
  in-flight-class element keys an open feature; orphans flagged by the health view, cleaned at
  desk visits. Supersedes the pinned ARCHITECTURE.md In-flight-pointer invariant (AT-D6-C) —
  recorded supersession owed at build. Epic: one joint sign-off survives. *Rationale:* consult
  must be free or it gets skipped; authoring must be gated or every typo fix pays architecture
  tax; sign-off-as-write-gate keeps the store ruled-truth-only. **Assumed** *(re-marked at
  review, S4: designed live, never walked against a real feature; was Confident)*
  *Review folds (batch):* **(S8) consult metered** — every plan run reads root index + AX
  summary table (the trip check lives there); the spine deep view is read only on the
  structural-change trigger that gates delta authoring — delta-scope runs stop at the index.
  **(S9) trip semantics** — trips batch at the run's front; disposition is
  warn-and-record with a recorded-deferral escape, never a silent skip; a fired upgrade
  trigger outranks a feature-touch trip; a trip fires once per feature, not per touch.
  **(S13) the no-delta claim survives** — a plan run judging the feature non-structural
  records that judgment as one line in the plan package, shown at gates, never made silently
  (carries `patterns-system-design`'s protected line forward). **(S6) the landing diff
  survives** — the store skill inherits Duty 1's approved-delta-existed trigger: at landing,
  built-vs-signed is diffed even when nothing was built (descoped features caught at the
  landing, not weeks later by the orphan sweep); `As-built:`/`Drift:` writes are judgment and
  graded, narrowing D11. BACKLOG's AT-D5 audit rider re-keys its input to the store.

- **D11 — Store-write review cadence: judgment writes graded, mechanical writes ride the
  landing audit.** Desk judgment writes (baseline authoring, shelf-walk stance batches,
  amendments) take an independent tech-lead review before user ratification — the sound-loop
  seat pairing (architect produces, tech-lead reviews) applied to the new desk surface.
  Plan-time deltas need no new seat: `review-feasibility` (tech-lead) + `review-plan-artifacts`
  + user sign-off already grade them. Mechanical writes (landing status flips, orphan cleanup)
  are transcription — no separate seat. *(Narrowed at review/verify, S6 + N1: `As-built:` and
  `Drift:` writes are **judgment, graded** — struck from the mechanical list; only status
  flips and orphan cleanup remain transcription.)*
  *Rationale:* mirrors sound-loop's own judgment/transcription carve; grading mechanical flips
  is ceremony without information. **Assumed** *(re-marked at review, S4: two-word adoption
  in a flagged ratification streak; was Confident)*

- **D12 — Absorb: the store swallows `nfrs.md` and the structural half of
  `constraints-and-decisions.md`; store homed at `.mochiko/product/architecture/`.**
  *(Reopen-born from review survivor S1 — the record had never counted the `.mochiko/product/`
  baseline layer; D3's "three-surface" premise corrected here.)* `nfrs.md` dies whole — NFR
  targets become fields on concern rows (one home per concern: stance + pattern + targets +
  as-built + drift). **NFR-XXX ids survive, homed inside store rows** — trace chains
  (TR-XXX → NFR-XXX) unchanged, only the path moves. Structural-origin D-XXX die into store
  deltas (the store ruling is the decision record); `constraints-and-decisions.md` survives
  reduced: feature-run ADR trail (analysis-origin D-XXX), **C-XXX hard constraints (stay)**,
  IP-XXX rows. `quickstart.md` **kept** — usage documentation, not architecture; already
  conditional + capped by recorded ruling; killing it would dilute the store's ruled-truth
  character for near-zero saving (user probed, ruled keep). Baseline set becomes:
  `data-model.md` · `contracts/` · `quickstart.md` · `constraints-and-decisions.md` (reduced) ·
  `architecture/` (the store). Added consumer rewires on top of the 26: 
  `authoring-technical-requirements` (NFR grammar + constraints-file structure),
  `review-feasibility` NFR↔topology lens (simpler after — both sides one artifact),
  `testing-gap-finding`/gates runtime-NFR re-points, plan/implement baseline lists + folds,
  setup bootstrap seeds the store. The `plan.md` "baselines never edited in place" floor gains
  a recorded amendment: plan-time store writes legal only as `in-flight`-status deltas at user
  sign-off; ruled truth never edited in place by a plan run. *Rationale:* user's standing
  principle — architecture becomes a first-class citizen with **one home**, not split across
  feature builds; two files both speaking "observability" invites drift even with cross-cites;
  user explicitly accepted the enlarged blast radius. **Contested** *(user ruled absorb against
  the lead's coexist recommendation — deliberate choice, blast radius priced; per the
  confidence vocabulary this is the Contested mark's exact definition)*

- **D13 — Pull origination: a fired upgrade trigger routes through `/mochiko:feature`'s
  growth door.** *(Reopen-born from review survivor S14 — the pull half of the topic was
  unvisited; the sketched `Work:` field presumed a path no decision created.)* Desk flags the
  fired trigger in the health view → lands at the feature desk as a candidate capability/work
  row under the capability-write test → extend-beats-mint applies → the user rules the mint.
  No new door, no auto-mint; the architecture lens proposes, the map machinery disposes.
  *Rationale:* the growth door exists precisely for non-story capability entry; every
  alternative either bypasses the user's mint ruling or lets the trigger rot. **Confident**
  *(user: "as recommended")*

- **D14 — Opinion precedence across three strata; F7 corrected.** *(Born from review survivor
  S2, which falsified F7: FLOOR-SEC asserts "auth enforced at all boundaries" at both depth
  levels, FLOOR-OBS same shape — floor-asserted, waiver-only.)* (1) **Floor-asserted
  obligations bind shelf rows**: where a floor card asserts a category, `n-a — genuinely
  never` is unavailable at the desk; legal moves are a stance within the obligation,
  `n-a — handled elsewhere` (pointer required), or narrowing; a true drop routes to the
  governance-ledger waiver, never the shelf. (2) **Arbitrated cards vs shelf rows**: cards
  bind code-layer structure, shelf rows bind product topology/concerns; genuine conflicts
  route to the user. (3) **D9 stands with the fork named**: the governance dial governs floor
  rows' rigor; the desk governs stance — two instruments, two axes, stated openly.
  *Rationale:* leg 1 is the only ordering consistent with floor-asserted being NON-NEGOTIABLE;
  leg 3 is honest labeling of shipped machinery. **Confident** *(user: "as recommended")*

- **D15 — Delivery shape: two stages.** *(Born from review survivor S5's staging limb.)*
  **Stage 1, one wave:** store + schema + backend shelf + the full pipeline re-key
  (plan/implement/setup/feature, both review skills, crew recharter, D12 absorb migration,
  all strips/supersessions) — everything structural lands together; a half-re-keyed pipeline
  is worse than either whole state. **Stage 2, later:** frontend/mobile/desktop shelf content
  — pure data authoring, no pipeline coupling, each shelf shippable alone; store ships
  backend-first with the other surfaces as named gaps (constitution-catalog precedent).
  Stage 1 is not split further: a store without the re-key is dead weight, a re-key without
  the store points at nothing. *Rationale:* the one-wave precedent (template-schema D10,
  `Contested`) was 14 surfaces; this is roughly triple. **Confident** *(user: "as
  recommended")*

- **D16 — Migration posture: reconstruct-and-confirm at the first desk visit.** *(Batch-ruled
  "as recommended", S11.)* Existing repos: the first `/mochiko:architecture` visit
  reconstructs the store from what exists — repo `ARCHITECTURE.md` prose, any
  `.mochiko/features/FEAT-XXX/architecture.md` files, `nfrs.md`, structural D-XXX — presents
  it for confirmation, then archives the absorbed sources to
  `.mochiko/archive/product-baselines/<date>/` (the designated append-only archive home —
  destination fixed at verify, N7); nothing silently discarded.
  Brownfield derive explicitly inherits the setup-bootstrap `Assumed` caveat and its named
  partial-baseline poisoning risk. Pre-store pipeline runs against a store-less repo: the run
  offers the desk bootstrap rather than failing (the v0.66.0 seeding-defect class named and
  fenced). **Assumed** *(batch adoption; n=0)*

## Evidence honesty

n=0 — no live run of any of this. Driver is directional: the kinako dogfood *felt* gap
("nowhere to say anything about auth, logging, billing"), no cited incident where the missing
product lens produced a wrong build. First-live-run watch owed at build.

**Precision on the driver (review fold, S1):** cross-cutting *content* was not fully
homeless — `nfrs.md` (NFR-XXX) and `constraints-and-decisions.md` (C-XXX/D-XXX) already carry
product-scope cross-cutting content. What did not exist: a topology home, per-concern stances,
and any way to write product-level architecture **outside a feature run**. D1's premise holds
in that narrower, corrected form; D12 (absorb) is the reconciliation.

**Falsifier (added at review, S4):** at the first live run, either of these reopens the
store design — (a) the baseline shelf walk proves unbearably heavy for a greenfield project,
or (b) a real plan run proceeds without consulting the store (the consult contract failed in
practice). **First-live-run watch additionally carries** (verify round-2 observation): the
landing-time grading cost — with `As-built:`/`Drift:` writes ruled judgment-and-graded (S6),
every row-touching landing carries a grading obligation, priced nowhere yet; watch whether it
bites. Confidence roster (complete, per verify N5): D5, D7, D10, D11 → `Assumed`
(re-marked at review); D1–D4, D6, D8, D9 `Confident` (each actively shaped or redirected by
the user in session); D12 `Contested`; D13, D14, D15 `Confident` (explored individually with
stated lead confidence, user-ruled); D16 `Assumed` (batch adoption, n=0).

## Session trail

**Q1 — driver.** What has feature-driven architecture missed?
**A:** Two-part. (1) Architecture should become its own first-class workflow. (2) Felt gap in
the kinako dogfood (current version): no place to say anything about auth, logging, billing —
cross-cutting product concerns have no home in the story/feature-driven flow. Wants a
*suggestive, opinionated* schema; repo scope conditions it (example given: backend repo →
schema scoped to backend concerns).

**Q2 — schema shape.** Concern catalog (a), topology schema (b), or two layers (c)?
**A:** Both (a) and (b), layered — i.e. (c): a topology spine (containers, boundaries,
communication) plus a concern catalog (auth · tenancy · billing · observability · …) riding it;
concern rows may pin to structure or apply product-wide.

**Q3 — workflow home.** New living-desk command vs setup module vs plan-time expansion?
**A:** (i) — own command, living desk (`/mochiko:architecture`-shaped): baseline authored once
(greenfield elicits, brownfield derives), revisited any time; feature plans contest against the
standing product architecture and fold proven deltas back at landings. *Why the losers lost
(recorded at review, Y10):* (iii) plan-time expansion never answers "where do I say something
about billing" outside a feature run — the driver itself; (ii) setup module buries a living,
revisitable surface inside a one-time ceremony — the user's "first-class" requirement is a
standing desk, not an interrogation section.

**Q4 — catalog content.** 13-dimension backend SaaS/PLG shelf + 3 topology-spine opinions
(modular monolith first · one tenant-scoped Postgres · queue before event bus) proposed.
**A:** "Looks right" — accepted as the backend-shelf baseline. Rider: catalog must be
*adaptive to repo scope* — setup should scope it; a frontend repo changes the shelf and goes
deeper on frontend concerns rather than receiving a filtered backend list.

**Q5 — shelf mechanism.** Scope from setup, shelf per surface type (native-deep), composition
for full-stack/monorepo?
**A:** Yes — scope from setup, **overridable at the desk**. Rider (emphatic): shelves should be
*exhaustive*, bigger than 12 rows is fine. The crucial value is breadth — walk every dimension
and have the user form a stance per row given the project's state: "important", "not a concern
at the moment", or "not a concern at all".

**Q6 — row stance vocabulary.** `decided` / `not-now` (+ revisit trigger) / `n-a` (+ why) /
`open`, with a breadth invariant (every shelf row walked, never silently skipped)?
**A:** Keep the `not-now` vs `n-a` split. `n-a` gains a reason axis: *genuinely not a concern*
**or** *handled elsewhere* — the concern is real but another repo/system owns it (pointer
optional). Breadth invariant accepted.

**Q7 — push/pull wiring.** First proposed as four legs against existing surfaces; user redirected:
current docs are no constraint — blank slate, bold, breaking change fine, other workflows may
change. Re-proposed as single-store model.
**A:** Accepted: **one schema-backed architecture store** — topology spine + concern shelves +
element lifecycle (`ruled` → `in-flight (FEAT-XXX)` → `built`); per-feature `architecture.md`
**dies**; `ARCHITECTURE.md` becomes a rendered view or dies; plan always consults the store (no
omit-architecture escape); feature touching `open`/`not-now` rows trips a ruling; implement's
deviation gate points at the store; desk command owns baseline, walks, amendments, health view.

**Q8 — row form.** User sketched the canonical concern row; confirmed as the schema's row shape:

```
## AX-001 Tenancy
Ruling:    pooled multi-tenant (ruled 2026-08-19)   | or: default — unruled
Rationale: ...
Upgrade trigger: first enterprise deal requiring silo OR >N tenants
As-built:  pooled, tenant_id on 14 tables           | updated at landings
Drift:     none                                     | or named divergence
Work:      FEAT-012 (row r3), EPIC-004              | pointers, work lives on feature map
```

One store, per-feature architecture dies (re-confirmed). Rider: the sketch is illustrative,
not the final schema.

**Q8b — store file structure.** Root doc as index/TOC over the two Q2 layers, depth one hop
down, diagrams kept (user likes the existing container/sequence craft). *(Paths in this entry
superseded — see D4 as restated under D12.)*
**A:** Accepted:
- `ARCHITECTURE.md` — root index/TOC only: spine thumbnail + concern summary table
  (AX-XXX · status · one-liner · link).
- `.mochiko/architecture/spine.md` — topology deep view (container diagram, boundaries,
  sequence flows).
- `.mochiko/architecture/concerns/` — concern deep views. Rows start compact in a
  `concerns.md` ledger; a row graduates to its own file only when it carries real depth
  (long rationale, local diagrams) — extend-beats-mint.

**Q9 — agent/skill crew.** Proposed bold cut; user deferred once (structure first), re-presented.
**A:** "Stand." Ruled: skills = `authoring-architecture-store` (store owner; retires
`authoring-architecture`) · `patterns-architecture-shelves` (opinion carrier, shelves as data,
recommend-then-arbitrate) · `patterns-system-design` transformed (altitude + diagram craft
serving store deltas). Agents: `principal-architect` recharters as desk lead / store steward;
drift checks become an empirical duty (codebase probe grades `As-built:` claims, evidence not
memory); `tech-lead` grades store writes (author≠grader); no new persona.

**Q10 — schema carrier.** Store shape as schema data file(s) at `plugins/mochiko/schemas/`
(ninth schema-backed artifact family), shelves as data beside them, `mochiko-cli` renders,
raw Read degraded path (GI-020)?
**A:** Yes — with a shape principle: a small set of **required core fields** plus **broad
flexibility** beyond them (free-form extra fields/sections legal; the schema constrains the
skeleton, never the voice).

**Q11 — depth-dial coupling.** Architecture desk reads the governance low/high dial?
**A:** Confirmed: no coupling (→ D9).

**Q12 — consult vs author.** Every plan run reads the store + runs the trip check; a delta is
authored + signed off only on structural change (bugfix/delta-scope batch: consult yes, delta
no, no ceremony).
**A:** Confirmed — split works (→ D10).

**Q13 — delta lifecycle.** Six steps proposed: draft in plan package (store untouched) →
user sign-off as the write gate → store write with `in-flight`/`modifying`/`removing
(FEAT-XXX)` statuses → implement deviation gate points at store → landing flips `built`,
updates `As-built:`/drift, clears keys → orphan rule replacing the pinned In-flight invariant.
**A:** Hold (accepted), after one probe:

**Q13b — architect's reading contract at draft.** Whole plan or delta only? *(The "spine deep
view in full" below superseded — see D10's S8 fold: spine read only on the structural-change
trigger.)*
**A (ruled as proposed):** full feature spec (stories/FRs/TRs/NFRs/IP) + store root index
(full AX summary table — the trip check runs there) + spine deep view in full + only touched
concern deep-files; never other features' plan packages — cross-feature awareness flows
through the store (→ D10).

**Q14 — store-write review cadence.** Every write vs judgment-only vs deltas-only; user asked
for tech-lead role clarification (answered plainly: independent grader of architect-authored
store content, author≠grader; question wider than plan — desk visits are the uncovered
surface).
**A:** "As recommended" — (b), judgment writes graded, mechanical rides landing audit (→ D11).

**Q15 — (post-review, S1 explored) absorb / coexist / write-through.** Lead recommended
coexist (moderate confidence, `Assumed`-grade); user ruled **absorb** — "i am fine with the
large blast radius… architecture becomes first class citizen… one home" (→ D12, `Contested`).
Edges: NFR-XXX ids kept + re-homed, C-XXX stays — both "as recommended". Quickstart probe:
user asked can it die too; lead recommended keep (usage doc, not architecture; already
conditional + capped); user ruled **keep**.

**Q16 — (post-review, S14) pull origination.** Recommended growth-door routing, high
confidence. **A:** "as recommended" (→ D13).

**Q17 — (post-review, S2) floor precedence.** Recommended three-strata rule + F7 correction,
high confidence. **A:** "as recommended" (→ D14).

**Q18 — (post-review, S4) confidence re-marks.** Recommended D5/D7/D10/D11 → `Assumed` +
falsifier, high confidence; re-affirmation offered as the alternative. **A:** "as
recommended" — re-marks applied.

**Q19 — (post-review, S5 staging) delivery shape.** Recommended two stages, moderate-high
confidence. **A:** "as recommended" (→ D15).

## Appendix A — the accepted backend shelf (D5's content, landed verbatim at review, S3)

**Provenance disclosure (external-claims floor):** every default below is **memory-asserted**
— common SaaS/PLG practice as the lead model knows it, not verified against a named external
source. Each is a falsifiable judgment posture, dealt recommend-then-arbitrate, never
asserted. **Revisit condition (whole shelf):** shelf content is re-reviewed when the desk
skill's first live run completes, and thereafter whenever a desk visit finds a default argued
against twice in one project (the arguing-twice signal means the default is stale or the
segment moved); the store steward (`principal-architect` seat) owns freshness — no calendar
cadence, event-keyed only.

**Topology-spine opinions (3):**
1. Modular monolith first; extract a service only when change-rate or scale pays for it.
2. One Postgres, tenant-scoped, before polyglot storage.
3. Background jobs via queue before an event bus.

**Concern dimensions (13, backend SaaS/PLG):**

| # | Dimension | Suggested PLG default (memory-asserted) |
|---|-----------|------------------------------------------|
| 1 | Identity & auth | Managed IdP or framework auth; SSO a deferred-not-forgotten row |
| 2 | Multi-tenancy | Pooled, `tenant_id` + row-level scoping; silo only when compliance pays |
| 3 | Authorization | RBAC per tenant, coarse roles first |
| 4 | Billing & entitlements | Stripe-class provider; entitlements decoupled from billing events; self-serve trial→upgrade |
| 5 | Observability | Structured logs + error tracking + traces from day one |
| 6 | Product analytics | Event taxonomy `{object}_{action}`; activation funnel instrumented (PLG core) |
| 7 | Feature flags & rollout | Flags before branches for risky surfaces |
| 8 | Data lifecycle | Migrations discipline · backups · PII classification · delete/export path |
| 9 | Background work | Job queue + scheduler, idempotent jobs |
| 10 | API surface | Internal vs public split; versioning + webhooks when public |
| 11 | Notifications | Transactional email provider; in-app later |
| 12 | Deployment & environments | PaaS-first, staging + prod, IaC when the team pays for it |
| 13 | Security baseline | Secrets handling · rate limiting · audit log — **subordinate to floor-asserted FLOOR-SEC per D14** |

Rows 1 (auth), 5 (observability), and 13 (security baseline) carry floor-asserted obligations
(FLOOR-SEC / FLOOR-OBS): `n-a — genuinely never` unavailable on all three per D14 *(row 13
added to this list at verify, N4)*. The frontend shelf sample shown in session
(rendering strategy · state management · data fetching · design system · auth/session UX ·
client analytics · client flags · performance budgets · error handling · accessibility · i18n
· build & deploy) is **indicative only** — its authoritative form is Stage-2 authoring (D15).

## Build surface (landed at review, S5; cold-buildable target)

**Stage 1 — one wave (D15):**
- **New:** `/mochiko:architecture` desk command (name assumed, open thread) ·
  `authoring-architecture-store` skill · `patterns-architecture-shelves` skill + backend shelf
  data file · store schema data file(s) at `plugins/mochiko/schemas/` (`--check` skeleton-only
  per D8 fold) · store instance layout `.mochiko/product/architecture/` (`spine.md` +
  `concerns.md` ledger + graduation dirs), with repo-root `ARCHITECTURE.md` as the derived
  index — at the repo root, never inside the store directory, per D4 · health
  view = a section of the derived root index (open counts · stale `not-now` triggers · fired
  triggers awaiting D13 routing · orphan elements · drift register) — **that is the health
  view's definition; no separate artifact**.
- **Transformed:** `patterns-system-design` (altitude + diagram craft serving store deltas;
  no-delta claim carried forward per D10 fold) · `principal-architect` recharter (desk lead /
  store steward; manifest + cost-budget rows re-assert) · `tech-lead` (+store-write grading
  per D11-as-narrowed).
- **Retired/dying:** `authoring-architecture` skill (Duty 1 trigger inherited by the store
  skill per D10 fold) · per-feature `architecture.md` (D3) · `architect-report-template.md`
  (dies or reshapes) · `nfrs.md` as a file (D12 absorb; NFR-XXX ids survive in-store) ·
  structural-origin D-XXX section of `constraints-and-decisions.md` (D12).
- **Re-keyed (from the F8 26-file inventory + D12 additions):** `plan.md` (omit-escape dies ·
  sign-off re-targets store delta · consult contract per D10 fold · Boundaries floor amendment
  per D12) · `plan.yaml` (Architecture section mandatory-consult form; check string at :40
  superseded) · `implement.md` (deviation gate → store; landing = status flips + graded
  As-built/Drift + landing diff; In-flight-pointer machinery dies) · `setup.md` (scope handoff;
  bootstrap seeds store) · `feature.md` (desk cross-link; architect dormancy line revisited) ·
  `review-plan-artifacts` (ARTIFACT-CHECKLISTS Architecture section L100-133 re-written to
  store-delta grammar) · `review-feasibility` (architecture pass re-keyed incl. its
  `description:`; NFR↔topology lens reads the store) · `testing-gap-finding`/gates runtime-NFR
  re-points · `authoring-technical-requirements` (NFR grammar home + constraints-file
  structure, D12) · router rows ×6 · `patterns-sound-loop` (governing-surface row: store
  replaces `ARCHITECTURE.md` + `nfrs.md` entries; seat-wiring row already matches) ·
  `authoring-feature-map` + `patterns-map-minimalism` (peer-of lines; cap-trip co-sign
  re-point; D13 growth-door routing) · `authoring-epic` (joint sign-off re-target) ·
  `patterns-plan-minimalism`/`-code-minimalism` pointer lines · KM constitution module doc-role
  rows · `.claude/rules/mochiko/operating-docs.md` paths glob · cost-budget ledger rows.
- **Supersessions owed (recorded rulings, strips at build):** AT-D6-C In-flight-pointer
  invariant (project-pinned — replaced by the D10 orphan rule; touching it is a landing) ·
  `plan.md:75,211` "baselines never edited in place" floor clause (amended per D12: in-flight
  deltas at sign-off legal for the store) · `patterns-system-design:86` no-delta protected
  line (carried forward, home moves — supersession-by-relocation) ·
  `authoring-architecture:51` "depth lives in the feature artifacts" (inverted by D3/D4) ·
  `feature-sizing` D9/D15 baseline-set clauses touching `nfrs.md` (D12) · BACKLOG AT-D5 audit
  rider input re-key · governance: GI-018's version-lag acceptance revisited (derived index
  regenerates every write — the lag rationale dies with the prose doc); GLOSSARY standing
  trigger plausibly fires on the new vocabulary (store · spine · shelf · stance · AX-XXX ·
  desk · trip · health view · orphan) — governance amend via `/mochiko:setup` if so.
- **Recorded deferrals:** multi-repo / multi-product spine composition → rides the PO-D5
  Tier-III multi-stack/monorepo deferral, not an open question here · self-application note:
  mochiko itself has no `.mochiko/product/` and is not a SaaS product — kinako is the test
  bed; which shelf (if any) fits a plugin/CLI repo is Stage-2-adjacent, unblocked, unpromised.
- **Fold-failure datapoint (review minor, B-set I10 self-downgraded):** this repo's own `ARCHITECTURE.md` carries
  unaccepted content drift (pipeline line pre-v0.49.0). What makes the store's fold survive
  where prose fold failed: the index is derived not hand-maintained (D4 fold), statuses flip
  mechanically at landings (D10), and the scoped drift probe (D7 fold) checks claims against
  code empirically.

## Review trail

**Cold review (2026-08-19):** pair, lens-split (decision-quality + record-integrity), both via
blind-map two-message dispatch (38-angle maps each; seat B disclosed a negligible index-entry
fence leak, accepted). Cold reads independent; one-shot cross-examination both directions.
Both verdicts **critical-gaps**. Union after merge: **5 Critical · 13 Important · 10 Minor —
zero kills, reduction by merge only.** Lead adjudication: "7th command unargued" demoted
Important → Minor (seat A's fact-check: 6 commands, last movement additive; residue = Q3
option-(ii) reason unrecorded).

**Disposition (user-ruled, 28/28 dispositioned):** Bucket 3 (S3 shelf-enumeration +
provenance · S5-remainder build surface + supersession list + health-view definition · S6
landing-diff inheritance · S7 retrofit-cost walk ordering · S8 consult metering · S9 trip
batching/precedence · S10 drift probe scoping · S11 migration posture · S12 index
single-writer · S13 no-delta claim kept · S15 + ten Minors incl. two recorded deferrals) —
**"as recommended", one batch**; folds applied as decision annotations, Appendix A, and the
Build surface. Buckets 1–2 explored individually with recommendation + stated confidence each:
S1 → **D12 absorb** (`Contested` — user ruled against the lead's coexist recommendation; two
edges + quickstart probe ruled in-session) · S14 → **D13** growth-door routing · S2 → **D14**
three-strata precedence + F7 correction · S4 → re-marks applied (D5/D7/D10/D11 → `Assumed`)
+ falsifier landed · S5-staging → **D15** two stages · S11's standalone half → **D16**
migration posture. Severity splits (shelf-enumeration, confidence-marks) mooted — both repaired at Critical grade.
Lead adjudication of Y10 (7th-command) upheld: Minor; Q3 loser-reasons recorded.
*Tally clarification (verify N3):* S15 **is** Y10 — the demoted 7th-command finding; the
post-adjudication union reads **5 Critical · 12 Important · 11 Minor** (the "5/13/10" figure
earlier in this trail is the pre-adjudication union, kept as the historical number).

**Produced by:** session lead (this record, as-you-go) · fact seats: one Explore agent (two
sweeps) · review seats: two cold reviewers, lens-split, blind-map two-message dispatch ·
all rulings the user's.

## Open threads

- Desk command name — `/mochiko:architecture` assumed, unruled; settled at build.
- Exact required-core field set of the store schema — build-time design under D8's
  core+flexibility principle (the Q8 sketch is illustrative; `--check` scope ruled in D8's
  fold).
- ~~Shelf sequencing~~ — resolved: D15 Stage 2.
- ~~Constitution-card boundary~~ — resolved: D14 strata precedence.
- F5 path inconsistency (`plan.md` vs `patterns-system-design` vs architect report template
  artifact homes) — pre-existing defect; D3 kills most of it, repair rides the build.
- `architect-report-template.md` "dies or reshapes" — unruled either way; settled at build
  (added at verify — the Build surface carried it but Open threads did not).
