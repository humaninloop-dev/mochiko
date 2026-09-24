# Setup goes product-agnostic — decision record

**Status:** **accepted 2026-09-24** (user: "accepted") — review round 1 folded (16/16 survivors
dispositioned: S2–S5, S7, S9, S10, S13 user-ruled "as recommended", S1 opened and user-ruled "as
recommended" → D6, S6, S8, S11, S14–S16 lead-repaired, S12 no change); verify round 1 NOT CLEAN
(12/16; V1–V9 all dispositioned — V3 user-ruled "as recommended" then reversed at Q8, forward-only,
no legacy accommodation); delta-check 7/9 CLEAN, X1/X2 moot under Q8, X3 lead-repaired
seat-unverified, disclosed; reviewer's regrade after the Q8 supersession 8/9, status `ready`, Y1
(D6 rationale) lead-repaired post-acceptance as fix-on-sight · **Landed:** `DECISIONS.md` row 2026-09-24 (PO-D2 · PO-D3 · PO-D4 ·
PO-D5 · adaptive-depth D7 · Impeccable D6/D11 · OO-D3 annotated superseded in part) · `BACKLOG.md`
*Setup product-agnostic build* section (the build item + the rehoming brainstorm as OQ1's owner);
*Setup baseline-bootstrap hardening* → trail; *Module-elicitation scaling* annotated · `ROADMAP.md`
folded into the Production-only narrowing Next row (cap held at 7) · both indexes updated (this
entry; four prior entries annotated) · **build pending** ·
**Opened:** 2026-09-24 · **Lead:** session lead (inline questioning via
`mochiko:analysis-iterative`) · **Review:** solo cold review — `mochiko:devils-advocate` seat on
`mochiko:review-brainstorm` at its persona default tier, blind two-message dispatch (message 1:
topic + goal line only; the index and the session directory fenced; `record_contact: none` to be
attested) · **Transport (patterns-transport-floor):** one reviewer seat, lead-relayed messaging,
no cross-seat mesh; single writer per file — the record is the lead's, `reports/angle-map.md`
and `reports/review.md` the reviewer's; the record holds frozen until the verdict

## Topic

Driver ask (user, 2026-09-24): "i want to brainstorm the path forward on stripping setup workflow
of mochiko to not be about product related guidelines, but be just about coding, deployment and
other product agnostic standards, the reason for this is, product changes far too quickly and
having out of sync governance causes a lot of issues."

**Goal line (as narrowed at Q1, user 2026-09-24: "lets focus only on what stays in setup for
now. rehoming not a issue"):** decide what a product-agnostic `/mochiko:setup` carries — which
obligations stay, what the governance surface set is allowed to hold once the cut lands, and how a
product change stops being a governance event — leaving one hardened decision record. Where the
product-shaped obligations live instead is **out of scope** by the user's ruling (OQ1).

**Prior-session relations.** `production-only-focus` PO-D3 (2026-07-30): mochiko owns the
production standard, setup elicits facts, not standards; PO-D2: fact-triggered compliance modules.
`production-floor-adaptive-depth` D1–D8 (2026-08-11): the single depth dial. `product-architecture-
schema` D1–D16 (2026-08-19): the architecture store at `.mochiko/product/architecture/`, setup's
unconditional scaffold + `Scope:` handoff. `impeccable-design-integration` (2026-09-19): the
product-truth leg and the design baseline write inside setup. `delta-files-vs-direct-baseline-
edits` D1–D7 (2026-09-24): product baselines edited in place under git. `constitution-native-
surfaces` and `setup-constitution-flexibility` (2026-07): governance on native surfaces, the
ten-dimension interrogation. `ops-observability-hardening` OO-D3–D5 (2026-07-31, build not
started): app-level SLOs elicited at dimension 8 into a region one-liner plus a `paths`-scoped
rules file, RUNBOOK as a floor obligation with a KM scaffold, a fifth floor category Operations —
reconciled at review fold S10 (D3, D4).

## Ground facts (F)

*Sources: `mochiko-cli rules setup` (all seven sections, plugin 0.114.0); `authoring-constitution/
references/INTERROGATION-AGENDA.md`, `ESSENTIAL-FLOOR.md`, `COMPLIANCE-MODULES.md`, `catalog/
README.md`; `mochiko-cli template governance-intent` and `design-baseline`; the kinako tree
(`CLAUDE.md` governance region v1.2.1, `.claude/rules/mochiko/*.md`, `.mochiko/memory/governance-
intent.md`, `governance-ledger.md`, `DECISIONS.md`, `.mochiko/product/architecture/spine.md`).*

- **F1 — The interrogation agenda's ten dimensions split roughly six engineering, four product.**
  Product-shaped: 1 identity & intent · 2 fact profile (industry, data classes, jurisdictions,
  contracts → compliance modules, mechanical) · 3 type & shape (→ shelves) · 4 risk surface.
  Engineering-shaped: 5 team reality · 6 existing practices & tools · 7 knowledge management · 8
  deployment & release · 9 values & non-negotiables · 10 deliberate exclusions. Plus the one depth
  level declaration (`low` | `high`). PO-D3's agenda test: every question elicits a fact.
- **F2 — Setup's rules carry product-desk obligations beyond the agenda.** `setup.product-truth-
  leg` (platform · brand and voice · design principles · accessibility pointer · evidence fence)
  and `setup.design-truth-write` (the `product-designer` seat writes `.mochiko/product/design/
  design.md`, graded) · `setup.feature-map-brownfield` / `-greenfield` with `setup.fail.no-
  feature-map` · `setup.baselines-bootstrap` (`Assumed`: data-model · contracts · constraints ·
  quickstart from code, brownfield) · `setup.store-scaffold-unconditional` + `setup.architecture-
  scope-handoff` (the `Scope:` line) · `setup.design-scaffold-unconditional` · `setup.user-card-
  rulings` names the product-truth answers and the design baseline's ratification. Section
  counts: roles 7 · reserved 7 · tools 13 · ways-of-working 6 · boundaries 7 · fail 6.
- **F3 — Kinako's governance carries product behaviour and product architecture.** The region
  (v1.2.1) lists GI-037 "Kinako proposes; the leader rules", GI-036 "the IPC envelope carries no
  sentences; the console composes", GI-006/011/012/013/031 corpus integrity, erasure, export,
  transparency, backup, GI-028 shadcn-sourced UI. Rule files total 476 lines; `engine-port.md`
  (154) carries GI-004's five security boundaries and GI-016's project-structure tree. The
  architecture store's boundary rows SPN-015…SPN-019 cite "GI-004 boundary 1", "GI-008", "GI-036
  `prose.md`", "GI-032" as their sources — the same boundary held in two homes.
- **F4 — Product changes have become owed governance events, booked and not taken.** 2026-09-19:
  SPN-037 accepted as a proposed sixth GI-004 boundary, "the five→six ledger amendment an owed
  governance event through `/mochiko:setup`, not taken". 2026-09-22: boundary 5's "the CLI writes
  the spool only" wording at `engine-port.md:115` and `spine.md:207` "booked through
  `/mochiko:setup`, not taken" (both quoted from kinako `DECISIONS.md` lines 77 and 80; the
  `spine.md:207` pointer is the row's own, historical — the text sits at `spine.md:173` and
  `:233` at review time). Amend history: v1.0.0 (2026-08-12) → v1.1.0 (2026-08-16, GI-035
  minted out of remit, folded at 1.2.0) → v1.2.0 (2026-09-03, the B30 amend, eleven cards, 30
  raised → 15 survivors, verify PASS) → v1.2.1 (2026-09-05, direct stamp move, disclosed).
  *(S16 repair.)*
- **F5 — Kinako's intent synthesis is welded to product records.** It cites the vision and
  integration records 39 times by phrase count (23 "vision record" + 16 "integration record",
  S16 repair); GI-001's `gdpr` attachment rests on the vision record's D12
  (transcripts retained); the cold review's C1 found dimensions 1 and 4 skipped on the authority
  of an unaccepted vision record — user-ruled to stand with a revisit trigger.
- **F6 — Compliance modules attach mechanically from product facts.** `COMPLIANCE-MODULES.md`:
  `hipaa` · `pci-dss` · `gdpr` · `a11y` (legal-mandate, unwaivable, level-blind) · `attestation`
  (contractual). The fact-validation fail-safe demands named elicitation and consequence-stated
  confirmation of every negative. A fact-profile change is a governance event by the amendment
  policy.
- **F7 — The design baseline's truth part is setup-sourced by design.** `mochiko-cli template
  design-baseline`: platform, brand and voice, design principles, evidence fence, accessibility
  pointer — "one writer, `product-designer`, on exactly three graded paths: the setup leg, a
  build-time `baseline-delta.md` entry, and the landing fold" (the middle path retired by
  `delta-files-vs-direct-baseline-edits` D1).
- **F8 — Mochiko's own governance is product-light by accident of subject.** GI-001 facts: "none
  (developer tool)"; the "What this is" target paragraph sits outside the governance region;
  GI-017 rules that governance surfaces point at constraint homes and never restate them.
- **F9 — Setup's done condition names product artifacts.** The goal in `setup.md`: the feature
  map exists at close (fail condition), brownfield close carries the bootstrapped product
  baselines (`Assumed`), the store's `spine.md` stub with `Scope:` and the design home's scaffold
  are written on every path.

## Constraints carried in

- PO-D1's target (customer-facing product applications) stands unless ruled here; this session is
  about where product facts live, not what mochiko targets.
- GI-017: governance surfaces point at existing constraint homes and MUST NOT restate them.
- Editing shipped primitives is a landing: strips + author≠grader audit + `plugin.json` bump;
  setup's rules live in the migration log (`plugins/mochiko/migrations/`), so a cut is a migration.
- The KM landing ritual: `DECISIONS.md` row · `BACKLOG.md` move · `ROADMAP.md` touch · both
  indexes agreeing.

## Decisions (D)

### D1 — The fact profile and the compliance-module layer leave setup — `Confident`

**Statement.** A product-agnostic `/mochiko:setup` asks nothing about industry, data classes,
jurisdictions or contractual commitments; interrogation dimension 2 is struck, no compliance
module (`hipaa` · `pci-dss` · `gdpr` · `a11y` · `attestation`) attaches in a setup run, and the
governance surface set carries no module and no legal-mandate stratum. The fact-validation
fail-safe (named elicitation, consequence-stated negatives) goes with the dimension it guarded.

**Rationale.** Dimension 2 is the one agenda dimension whose answers are product facts, and a
setup that cannot ask them cannot select modules from them. The facts move with the product —
kinako's `pci-dss` negative is a "today" fact with a revisit trigger on any billing work, its
`gdpr` attachment rests on a product-vision decision (F5) — and every move is a governance event
under the amendment policy (F6). The floor keeps the four engineering rules the modules build on
(secrets out of the repo, input validation, no PII in logs, no silent data loss); the modules'
**additive** obligations — audit logging of auth events, key rotation, coverage at 90/80, log
retention, access-controlled log storage, license compliance, medium-plus vulnerability blocking
(`COMPLIANCE-MODULES.md` seed pool) — leave with the modules and are OQ1's to rehome. *(S2
repair: the earlier sentence "already floor content" was false — a module only ever adds to the
floor.)*

**Consequence stated to the user before the ruling.** Governance holds no compliance obligations
after the cut; where the product's legal obligations are held is OQ1's rehoming question, out of
scope here. User ruled "out" (2026-09-24) with the consequence in view.

**Alternatives considered.** Keep dimension 2 as the one product-fact question because the
obligations it attaches are code standards — rejected by the user: the out-of-sync pain would
stay for exactly the dimension that moves most. **Modules as user-declared engineering modules**
(S2's road: attach and detach by declaration only, D4 event (5)'s shape, no fact re-elicitation)
— put at review, rejected by the user as recommended: a declared module is the same product fact
under another name ("we have EU users"), and it moves with the product exactly as the elicited
fact did.

**Touches.** `INTERROGATION-AGENDA.md` dimension 2 and steps 1/5 (module attachment) ·
`COMPLIANCE-MODULES.md` (its setup binding) · the governance-intent template's Fact profile
section and modules line · the CLAUDE.md governance region's `modules:` stamp · the design
baseline's Accessibility pointer (F7, rehoming out of scope) · setup's amendment-policy trigger
"fact-profile changes (module attach/detach)".

### D2 — Governance holds the rule, never the product's instance of it — `Confident`

**Statement.** A minted principle (interrogation dimension 9) enters the governance surface set
only if it can be checked from the code, the pipeline or the stack without knowing what the
product does for its users. A product's instance of a rule — its boundary list, its data's name,
its behaviour toward its users — never enters; where governance needs the instance it points at
its home and never restates it (GI-017 extended from restating constraint homes to restating
product instances). Elicited intent that fails the test is not authored as a principle and not
flagged as a proposal; it is handed off (OQ1) — setup records the hand-off, not the content: the
governance-intent template's Deliberate exclusions section gains a **Handed off** list, one line
per failed intent with the pointer to OQ1's owner *(S14 repair)*.

**Rationale.** The rows of kinako's region (F3) that fail the test are the rows that generated
the owed amends (F4): the five-boundary list and the prose boundary are product architecture the
store already holds. Every engineering standard survives it. The claim that the test is
mechanical enough for the validate seat to apply is **`Assumed`** *(S3)* — the build owes the
validate seat worked cases, seeded from the sort below.

**The sort (illustrative, kinako v1.2.1 — the full sort over every region row is kinako's at
its next amend, OQ2; user-confirmed at Q3, GI-008 and GI-030 re-sorted at review fold S3 as
recommended):** GI-009 dependencies pinned and justified — stays · GI-008 "the Claude engine MUST
be reached only through a port" — the **rule** stays (external systems through ports, at
whatever scope the project ratified — kinako ratified the engine seam only, V9), the **engine
instance** leaves — it is the store's SPN-016, and the engine choice was itself a product ruling · GI-030 UI never touches the corpus or the engine —
the layer rule stays (the UI layer reaches no persistence or external system directly), the
corpus and engine names leave · GI-028 UI components from shadcn — stays (stack choice) · GI-004
"validate input at every boundary" — rule stays, the five-boundary list leaves (the store's) ·
GI-006 corpus data never silently lost — the floor's "no silent data corruption" stays, the
corpus wording leaves · GI-036 IPC envelope carries no sentences — leaves (product architecture
boundary) · GI-037 Kinako proposes, the leader rules — leaves (product behaviour).

**Alternatives considered.** Keep product-behaviour principles when they are test-enforceable
(GI-036 has a test) — rejected: enforceability is not the axis; the axis is whether the principle
changes when the product changes. Keep GI-008 as a stack fact like GI-028 — rejected at review
fold S3: a named port is a boundary row, and a boundary held in two homes is the pattern this
decision ends.

**Touches.** `INTERROGATION-AGENDA.md` dimension 9 phrasing and step 3 (mint) · the
governance-intent template's Minted principle intents section · `authoring-constitution`'s minted-
principle path · `validation-constitution` (a new check: no product instance in the set) · the
region's Principles list shape.

### D3 — The interrogation agenda after the cut: seven dimensions and the depth level — `Confident`

**Statement.** The agenda keeps, in order and still adaptively: **3** type and shape — read from
the code (brownfield) or from the architecture store's `Scope:` line when the store carries one,
asked only when neither answers; picks the shelves · **5** team reality · **6** existing
practices, tools and real commands (brownfield analysis pre-fills) · **7** the knowledge-management
module · **8** deployment and release reality with the release-gates module — **and, folded at
review (S4), the trust vectors of what ships and where**: a binary, hooks, a public package, a
deploy target's exposure, and the supply-chain controls they call for; deck presets are tuned by
dimensions 8 and 9 · **9** values, minted under D2's test · **10** deliberate exclusions · and the
single depth-level declaration (`low` | `high`). After the dimensions, unchanged: the floor
asserted at the declared row with expression by type, the arbitrated deck dealt by type, minting
under D2, the layered-architecture beat with its domain-dependency seeds, waiver rulings, then the
synthesis-confirmation checkpoint. Struck: **1** project identity and intent (the synthesis keeps
a project name and nothing else of it), **2** the fact profile (D1), **4** the risk surface — its
product-harm half (what failure costs users, money, reputation) struck, its engineering half
carried into dimension 8 as above.

**Dimension 8 under D2 (review fold S10, as recommended).** The kept dimension carries rules, not
product values: an SLO is declared and measured (the rule stays), a runbook exists under its
contract (OO-D4, untouched), Operations is a floor category (OO-D5, untouched) — but the
**app-level SLO number set** is a product value and lives in the architecture store as a concern
row, never as a governance element; OO-D3's "region one-liner + `paths`-scoped rules file" is
**superseded in part** — the rules file keeps the obligation, the numbers leave it.

**Rationale.** Dimensions 1, 2 and 4 are the three whose answers are about the product — what it
is, for whom, what its failure costs — and they are the three whose answers kinako's synthesis
took from product records (F5). The seven that stay are answered by the codebase, the team and
the pipeline. The depth recommendation loses its risk-surface input and falls back to the agenda's
own default: `low` for greenfield, `high` where brownfield reality warrants it. Dimension 4's
engineering half is not product-shaped — mochiko's own risk entries AM-1 to AM-3 produced the four
supply-chain controls that gate the hook ship (GI-012) — so it moves rather than dies (S4).

**Alternatives considered.** Keep dimension 4 whole as the one product-flavoured input to the
depth recommendation — the lead flagged it as the least certain strike; the user struck it: the
depth level is the user's declaration, and setup's recommendation can rest on code reality alone.
Re-put dimension 4 whole at review (S4's first road) — the user took the split instead.

**Touches.** `INTERROGATION-AGENDA.md` (the ten-dimension table, depth-per-mode, the "no pruning
license" note — a struck dimension is a scope ruling, so the note gains a clause) · the
governance-intent template's Project identity & type section (identity and risk lines struck;
type kept) · `setup.interrogation-inputs` · dimension 3's new read-before-ask order.

### D4 — Governance events after the cut: a closed set; a product change opens no amend except through event (3) — `Confident`

**Statement.** The amendment policy's governance-event set is closed at six: **(1)** the depth
flip `low`→`high` (the D6 ceremony, unchanged) · **(2)** a waiver added, lifted or re-grounded ·
**(3)** a stack, toolchain or layout change — a real command changes, a surface type is added and
a new shelf deals, or the repo layout the `paths`-scoped rules files bind to moves *(S13 fold)* ·
**(4)** an engineering principle minted, dropped or redefined under D2 · **(5)** an engineering
module attached or detached — knowledge-management, release-gates, layer-rules · **(6)** the
deliberate exclusions change. **A product change opens no amend except through event (3)** *(S5
reword)*: a new boundary, a new data class, a new market, a feature-map change, an
architecture-store write, a design-baseline write, an SLO number moved (S10) — none opens an amend
and none leaves governance stale, because after D2 governance cites no product instance. The one
crossing is a product change that changes the surface types — a web or mobile surface added — and
that is event (3) by its engineering face. **Detector (S5, as recommended):** the architecture
desk's write to the `Scope:` line surfaces "shelf set changed — a setup amend is owed" at its
landing; no watcher, no automatic amend — the same no-watcher posture as the depth dial. OQ3
closes into this. **Amend meeting old content (forward-only — user-ruled at Q8, 2026-09-24: "are we fitting for
legacy projects, we dont need to. it needs to align with going forward"):** an amend run meeting
a synthesis that still carries a fact profile, attached modules, or product-instance principles
supersedes them in place, once, never renumbered — the same shape as today's tier-ladder
migration clause in the governance-intent template. No carry-forward, no interim marker, no
waiting on the rehome: the new agenda is the only agenda from the cut on. *(The V3 split —
modules and product principles carried `carried pending rehome` until the rehome landed — was
folded as recommended and then reversed by the user at Q8; existing consumers are not a design
input for this cut.)*

**Rationale.** The goal line's last limb. Kinako's owed amends (F4) were all product changes
booked against governance because governance restated the instance; under D2 the sixth boundary
SPN-037 is a store write and nothing else. The six that remain are the events an engineering
standard genuinely has.

**Alternatives considered.** Leave the event set open ("any change to a governance surface") —
rejected: an open set is how product changes crept in. **A cheaper amend path** as the fix for
amends booked but not taken (S15's road — a PATCH-class direct stamp, which kinako already used at
v1.2.1) — rejected as the fix: a cheaper ceremony still leaves two homes for one fact, so the drift
is structural, not procedural; cheaper ceremonies for events (2)–(4) stay open to a later ruling.

**Touches.** The governance-intent template's amend preamble and legacy-migration clause · the
ledger's Amendment policy section (the "fact-profile changes (module attach/detach)" trigger
struck) · the CLAUDE.md region's Governance operations "Amend via" line · `INTERROGATION-AGENDA.md`
depth-per-mode Amend paragraph · `authoring-constitution`'s amend path.

### D5 — Setup's rule set and done condition after the cut — `Confident`

**Statement (as amended at review folds S6 and S7, user-ruled as recommended).** Of setup's 46
delivered rules (plugin 0.114.0: roles 7 · reserved 7 · tools 13 · ways-of-working 6 · boundaries
7 · fail 6), **five ids are struck and seven amended**; no floor leaves. **Elicited product
content goes; empty scaffolds stay** — a scaffold carries no product content and cannot drift.

*Struck (5):*
- roles: `setup.product-truth-leg` · `setup.design-truth-write`
- reserved: `setup.user-map-confirmation`
- tools: `setup.feature-map-brownfield` (the reconstruction) · `setup.baselines-bootstrap`

*Amended (7):*
- `setup.user-card-rulings` keeps the card, module and waiver clauses; loses the
  product-truth-answer and design-baseline-ratification clauses
- `setup.interrogation-inputs` drops `COMPLIANCE-MODULES.md` (D1)
- `setup.feature-map-greenfield` becomes the scaffold of the empty `FEATURES.md` index,
  write-if-absent on **every** path (its `when: mode=greenfield` gate dropped); on brownfield the
  empty index carries one line naming that reconstruction is pending and whose it is (OQ1)
- `setup.design-scaffold-unconditional` — the required truth headings written by the lead as an
  empty scaffold, no `product-designer` seat; the truth content is its owner's (OQ1)
- `setup.fail.no-feature-map` narrows to "no feature-map index at close — absent and not
  scaffolded"; enforces `setup.feature-map-greenfield` alone (still a floor)
- `setup.map-never-overwrite` — its instance is now the scaffold alone
- `setup.blind-map-dispatch` sends "the setup topic / project name and goal" (S11)

*Unchanged of the product-desk set:* `setup.store-scaffold-unconditional` ·
`setup.architecture-scope-handoff` (the `Scope:` line from dimension 3, write-if-absent — the
desk's `arch.shelf-scope-source` keeps its source) · `setup.store-ruled-content-never-here`.

**Forty-one stay** (5 struck of 46): section counts after — roles 5 · reserved 6 · tools 11 ·
ways-of-working 6 · boundaries 7 · fail 6; floors 20 → 20; fails 6 → 6. *(S6 repair: the Q6
shape's counts were wrong — 12 ids not 13, and the fail rule is a floor; the S7 shape makes them
moot.)*

**The done condition:** the governance surface set — the intent synthesis ratified before any
surface is authored, the trace closed and independently graded from the files, the region's
semver bumped, the set accepted flagged proposal by flagged proposal — **plus the scaffolds**: the
feature-map index exists at close (scaffolded if absent, never reconstructed), the store's
`spine.md` stub with its `Scope:` line and the design home's empty truth headings exist, each
created only where missing. No product baselines bootstrap; the `Assumed` baseline sentences in
`setup.md`'s goal go.

**Inputs:** `COMPLIANCE-MODULES.md` leaves `setup.interrogation-inputs` (D1). `analysis-codebase`
keeps stack detection (`detect-stack.sh`) and the floor's present/partial/absent read; its
feature-map reconstruction and baselines feed leave setup's binding (their fate is OQ1's).

**Rationale.** The rule-level form of Q1's leaves-list, now ruled: every struck rule elicits or
writes product **content** — a design truth, a reconstructed map, bootstrapped baselines. Every
kept scaffold writes layout that carries no product content, so the out-of-sync rationale does not
reach it; keeping them costs no drift and keeps `/mochiko:specify`'s missing-map routing and the
desk's `Scope:` source intact (S7).

**Alternatives considered.** Strike the scaffolds too (the Q6 shape, user-confirmed) — reversed
at review fold S7 as recommended: it dead-ended specify (`spec.missing-map-surfaced` routes a
missing map to setup) and orphaned `arch.shelf-scope-source`, for zero drift gain. Keep the two
scaffold floors as guards — moot, they stay with their instances.

**Touches.** One migration in `plugins/mochiko/migrations/` striking five ids and amending seven
(tombstones, never renames — `setup.gate-loop-bound` untouched) · `setup.md`'s goal paragraph
(Not-done count unchanged at 6) · `.mochiko/strips/setup.md` entries · the contract suite's setup
expectations (`evals/contract/`) · `evals/plan/setup/observable.yaml` re-keyed to the struck and
amended set (S9) · `CHANGELOG.md`.

### D6 — Shipping order: the cut ships now; the rehome is a named, queued session — `Confident`

**Statement (born at review fold S1, user-ruled "as recommended" 2026-09-24).** The cut (D1–D5)
ships as one release without waiting for the rehome. In that same release every consumer that
cites a compliance module — the closed list in build steps 3 and 4 (V2) — is reworded so nothing
points at a module. OQ1 gets an owner: a `BACKLOG.md` item and a `ROADMAP.md` Next row for a
brainstorm on rehoming the product-shaped content — the compliance-module content, the design
truth, the feature-map reconstruction, the baselines bootstrap, the minted product-behaviour
principles. **Exposure accepted eyes-open:** between the cut and the rehome, a brand-new project
with legal-mandate facts set up in the window attaches no module and nothing says so. **Existing
consumers are not a design input (Q8):** kinako's and mochiko's governance take the new agenda at
their next amend, their product content superseded there under D4; where that content goes is
OQ1's session, and nothing in this cut carries it for them.
**The BACKLOG trigger (V4):** setup no longer elicits legal-mandate facts, so no fact can fire it;
the item rides the ROADMAP Next queue and is re-put at every groom until taken.

**Rationale.** With S7 folded the specify dead-end is gone — setup still scaffolds the empty
index and the `Scope:` line — so the only cost of shipping first is the window, and existing
consumers are not a design input (Q8). Gating the strip on a second session delays the pain the
user came to end. *(Y1 repair, post-acceptance fix-on-sight: the earlier "touches no live
consumer" / "no live project is in" phrasing was false under Q8 — kinako's owed amend supersedes
its module and product-instance content with nothing carrying it; the ruling makes that
acceptable, not absent.)*

**Alternatives considered.** Gate the cut on the rehome (build both together) — rejected: no
window, but the strip waits on a session with no date. An interim shelf (module content dealt as
concern rows by the architecture desk now) — rejected: a rehoming design in miniature, out of
scope by the Q1 ruling. *(V7 distinction, disclosed: D3's placement of the app-level SLO number set
in the store is not the same move — the set already had a ruled home in governance (OO-D3), and
S10 superseded that ruling by naming the store, the home every other product value already has;
the module content has no non-governance home yet, so naming one is design.)*

**Touches.** `BACKLOG.md` (the rehoming item — no fact-based trigger, V4: it rides the
`ROADMAP.md` Next queue and is re-put at every groom until taken; X3 repair) · `ROADMAP.md` Next
· the landing.

## Review (cold, round 1 — `reports/review.md`; blind map `reports/angle-map.md`, 23 angles / 17 load-bearing / 7 classes)

Solo `mochiko:devils-advocate` seat on `mochiko:review-brainstorm`, both lenses, persona default
tier, no haiku explorer spawned (every read interpretive — disclosed by the seat). Fence held,
`record_contact: none`. Facts: F1–F3, F6–F9 verified; F4/F5 partly (S16); D5's counts broken
(S6). Recommended status **`critical-gaps`** on the unowned-decision criterion (S1). Tally: 21
raised → 16 survivors (1 Critical · 9 Important · 6 Minor; coverage: S9, S10, S13). Nine angles
covered clean. Lead verified S1's, S7's, S9's and S10's load-bearing claims against the files
before proposing dispositions (`spec.missing-map-surfaced`, `arch.shelf-scope-source`, OO-D3–D5,
`evals/plan/setup/observable.yaml` — nine struck ids cited).

| # | Sev | Touches | Finding (short) | Proposed disposition | Ruling |
|---|---|---|---|---|---|
| S1 | Critical | D5, D1, build, OQ1 | Strip ships as one wave; OQ1 unowned; specify's missing-map routing dead-ends greenfield; legal-mandate stratum held nowhere; TEST-GRAMMAR's a11y assert falls back silently | Accept: OQ1 becomes a BACKLOG item with a trigger; S7's scaffold-keep is the map/store interim in the same release; every module-citing consumer (S9) reworded in the same release; no sequencing gate on a rehome session | opened with the user (user: "open critical") — the specify limb dissolved by S7; the shipping order and OQ1's owner re-put as Q7 → **user: "as recommended" → D6** |
| S2 | Important | D1 | Rationale "already floor content" false — modules ADD (audit logging, key rotation, 90/80, retention); the declared-module road never put | Repair the sentence; put the declared-module road: **recommend reject** — a declared module is the same product fact under another name, same drift class | user: as recommended — folded 2026-09-24 |
| S3 | Important | D2 | GI-008 kept but SPN-016 is the same boundary (two homes); only 7 of ~17 rows sorted; "mechanical" untested | Re-sort GI-008: the ports rule stays (BE-HEX), the engine instance leaves; GI-030 likewise (layer rule stays, corpus/engine names leave); the sort declared illustrative, the full sort kinako's at its amend (OQ2); D2's "mechanical" claim re-marked `Assumed`, worked cases owed to the build | user: as recommended — folded 2026-09-24 |
| S4 | Important | D3 | Dimension 4 also feeds deck presets and floor expression; mochiko's own risk entries produced the supply-chain controls (an engineering trust vector) | Split: the product-harm half stays struck; the engineering trust-vector half (what ships where — binaries, hooks, supply chain, deploy exposure) folds into dimension 8 explicitly | user: as recommended — folded 2026-09-24 |
| S5 | Important | D4, OQ3 | "Never" contradicts event (3); a Scope change at the desk deals no new shelf | Reword D4: a product change opens no amend except through event (3); detector: the desk's `Scope:` write surfaces "shelf set changed — setup amend owed", no watcher | user: as recommended — folded 2026-09-24 |
| S6 | Important | D5, build 1 | Counts wrong: 12 ids, 3 floors leave (the fail rule is a floor), 20 → 17; "other three" lists five | Lead repair | lead-repaired 2026-09-24 |
| S7 | Important | D5 | Empty scaffolds carry no product content, cannot drift; keep-scaffolds road never dealt; striking orphans `arch.shelf-scope-source` | Put the road: **recommend accept** — scaffolds stay (empty index write-if-absent on every path, spine stub + `Scope:`, design headings by the lead, no seat), elicited content goes; D5 narrows | user: as recommended — folded 2026-09-24 |
| S8 | Important | build 5 | Supersession chain misses PO-D3, PO-D4.2, the PO S4 fold, impeccable D11 | Lead repair: rows added | lead |
| S9 | Important (coverage) | build 2/4 | Consumers unlisted: specify routing, desk Scope rule, TEST-GRAMMAR a11y, design-direction + migration 0011, release-gates fragment, `observable.yaml` | Candidate topic; **recommend rule inline**: fold into the build surface | user: as recommended — folded 2026-09-24 |
| S10 | Important (coverage) | D3, D4 | Queued ops-observability build: OO-D3 SLO numbers via dimension 8, OO-D4 RUNBOOK, OO-D5 Operations floor — product values through a kept dimension | Candidate topic; **recommend rule inline**: D2's test governs dimension 8 — the rules stay (SLOs declared and measured, runbook contract, Operations category); the app-level SLO set is a store concern, not a governance element; OO-D3 superseded in part | user: as recommended — folded 2026-09-24 |
| S11 | Minor | D3, D5 | `setup.blind-map-dispatch` still sends "project identity" | Lead repair: reword in the migration | lead |
| S12 | Minor | D5 | KM scaffold writes ARCHITECTURE.md and GLOSSARY.md | No change under S7: scaffolds carry no product content; the KM module is a declared engineering module | no change — reason recorded |
| S13 | Minor (coverage) | D2, D4 | Path-scoped globs go stale; no event covers | Candidate topic; **recommend rule inline**: event (3) widened to the repo layout the path-scoped rules bind to | user: as recommended — folded 2026-09-24 |
| S14 | Minor | D2 | Hand-off record has no slot | Lead repair: the governance-intent template's Deliberate exclusions gains a "Handed off" list | lead |
| S15 | Minor | D4 | Cheaper-amend road never weighed | Lead repair: recorded as rejected — a cheaper amend leaves two homes for one fact | lead |
| S16 | Minor | F4, F5 | v1.1.0 omitted; `spine.md:207` stale (now :173/:233); "37" method unstated (phrase count 39) | Lead repair | lead-repaired 2026-09-24 |

### Verify round 1 (`reports/review.md` § Verify round 1) — 12/16 folds CLEAN, `needs-revision`

NOT CLEAN: S1 (V2, V3) · S5 (V6) · S7 (V1, V5) · S10 (V7). New findings and dispositions:

| # | Sev | Touches | Finding (short) | Disposition |
|---|---|---|---|---|
| V1 | Important | OQ1, D6 | OQ1 still listed the scaffolds as rehoming items — false under D5 as amended, and OQ1 is D6's brief | Lead repair: OQ1 rewritten to the five content items |
| V2 | Important | D6, build 3/4 | Four module-citing consumers missing: the design-baseline Accessibility section (0018), QUALITY-CHECKLIST module checks, `universal-floor.md` line 16, OQ1's pointer item | Lead repair: added; D6's "every" bound to the closed list |
| V3 | Important | D4 legacy clause, D6 | Kinako owes an amend now; the legacy clause would strip its `gdpr` module and GI-011/012/013 before any rehome exists — a live product loses obligations | Put to the user — recommended the split: the module and product-principle limb activates only once the rehome lands, carried `carried pending rehome` before that; the fact-profile limb at once → user: "as recommended" — folded into D4 and D6 → **reversed by the user at Q8**: no legacy accommodation, the clause forward-only, existing consumers not a design input |
| V4 | Minor | D6 | The BACKLOG trigger cannot fire — setup no longer elicits the fact | Lead repair: rides the Next queue, re-put at every groom |
| V5 | Minor | build 3 | Design-baseline template's writer sentence not in the build | Lead repair: added to step 3 |
| V6 | Minor | D4 | Heading still said "never" | Lead repair: retitled |
| V7 | Minor | D3, D6 | SLO numbers to the store vs the rejected interim shelf | Lead repair: the distinction recorded in D6's alternatives |
| V8 | Minor | D3 | Trust-vector answers had no synthesis slot | Lead repair: a Trust vectors line in the template, step 3 |
| V9 | Minor | D2 | GI-008 re-sort wording wider than kinako's ratified scope | Lead repair: "at whatever scope the project ratified" |

### Delta-check (`reports/review.md` § Delta-check) — 7/9 CLEAN, reviewer's status `needs-revision`; closed by the lead

The reviewer graded V3 against the split before the Q8 supersession reached it (message ordering
is undocumented — designed around, not trusted). Its findings: **X1** (Important) — the D2
validator check has no exemption for `carried pending rehome` rows; **X2** (Minor) — OQ2 and D1
not realigned to the carried limb; **X3** (Minor) — D6's Touches line kept the stale fact-based
trigger. **Dispositions:** X1 and X2 **moot under Q8** — the marker and the carried limb no longer
exist, D1 "carries no module" and OQ2 "superseded at its next amend" are the forward-only text;
X3 **lead-repaired**, closed seat-unverified, disclosed. The reviewer's builder note
(`catalog/backend-service.md` line 110) added to build step 4's closed list. **Regrade after the
Q8 supersession reached the seat (message ordering):** 8/9 CLEAN, X3's repair verified, X1/X2
void, reviewer's final status **`ready`**; one Minor, **Y1** — D6's rationale still claimed the
window "touches no live consumer", false under Q8 — lead-repaired post-acceptance as a
fix-on-sight, disclosed in D6.

## Build surface (cold-buildable, single wave)

1. **Migration** — strike the five ids and amend the seven of D5 (pins unchanged: floors 20 ·
   fail 6); tombstones cite this record. Update `setup.md`'s goal paragraph (scaffolds stay,
   baselines sentences go).
2. **Interrogation surfaces** — `INTERROGATION-AGENDA.md` to seven dimensions + depth level (D3)
   with dimension 8's trust-vector input (S4) and its D2 clause on SLO numbers (S10), the D2
   phrasing on dimension 9, the amend paragraph and legacy clause of D4, and the
   struck-dimension clause on the no-pruning note; `catalog/README.md` and `ESSENTIAL-FLOOR.md`
   lose their module sentences; `COMPLIANCE-MODULES.md` is retired from setup's inputs (its
   content's fate is OQ1's).
3. **Templates** — `governance-intent`: Fact profile struck, Project identity & type reduced to
   type, a **Trust vectors** line in dimension 8's slot for the S4 input (V8), the Minted section
   under D2, a **Handed off** list under Deliberate exclusions (S14), the amend preamble under D4;
   the CLAUDE.md region's `modules:` stamp struck; the ledger's Amendment policy trigger list
   under D4 (event (3) incl. layout, S13); **`design-baseline`** (migration `0018`): the writer
   sentence amended — the setup path is the lead's empty scaffold, the `product-designer`'s
   graded paths are the landing fold and whatever OQ1's owner rules — and the Accessibility
   section's pointer text no longer names a module the project's governance attaches (V2, V5).
4. **Skills and consumers** — `authoring-constitution` (module attachment path, minted-principle
   test, amend legacy clause) · `validation-constitution` (new check: no product instance in the
   set, worked cases from D2's sort; floor-category check unchanged) · `analysis-codebase` (setup
   binding narrowed) · `review-governance-intent` (module-coverage angles retired) · **consumers
   named at review (S9):** `spec.missing-map-surfaced` reworded (a missing map → setup scaffolds
   the empty index; content reconstruction → OQ1's owner) · `patterns-vertical-tdd`
   TEST-GRAMMAR's accessibility assert (cites "the attached a11y compliance module") ·
   `patterns-design-direction` and migration `0011`'s no-accessibility-content rule (route to the
   a11y module) · the `release-gates.md` validator fragment ("Gates consistent with the attached
   compliance modules") · `authoring-architecture-store`'s `Scope:` write gains the "shelf set
   changed — setup amend owed" landing line (D4 detector) · `arch.shelf-scope-source` unchanged ·
   **added at verify (V2):** `validation-constitution`'s QUALITY-CHECKLIST module checks (lines
   26, 35, 39, 40, 47 at review time) retired · `catalog/universal-floor.md` line 16's module
   sentence · `catalog/backend-service.md` line 110 ("attached via the fact profile", flagged at
   the delta-check). This list is the closed set D6's "every module-citing consumer" means; a
   consumer found later is a build defect, not a new ruling.
5. **Landing supersessions** — `DECISIONS.md` rows: PO-D2 superseded in part (modules no longer
   setup-attached) · **PO-D3 in part** (the facts setup elicits are engineering facts) · **PO-D4's
   D4.2** moot (no legal-mandate stratum attaches; the unwaivable clause leaves with the modules)
   · **the PO record's S4 fact-validation fail-safe** struck with dimension 2 · PO-D5+S8 in part
   (a11y-to-modules routing) · adaptive-depth D7 moot (module level-blindness) ·
   **impeccable-design-integration D11** in part (the accessibility standard of record no longer
   routes to a module) and its setup leg in part · `product-architecture-schema`'s setup
   scaffold/`Scope:` handoff **stands** (S7) · **OO-D3 in part** (the SLO number set is a store
   concern, S10); the brainstorms index annotated on each. *(S8 repair.)*
6. **Evals** — `evals/plan/setup/observable.yaml` re-keyed (nine of its cited ids are struck or
   amended); the contract suite's setup expectations updated; both green before the bump.
7. **Self-application** — mochiko's own governance (v3.2.0) and kinako's (v1.2.1) take the new
   agenda at their next amend, product content superseded on sight under D4 (Q8); neither is
   stripped in this build.

Every edit is a shipped-primitive landing: strips recorded, author≠grader audit, `plugin.json`
bump, contract suite green.

## Open questions

- **OQ1 — Rehoming of the product-shaped content.** *(Rewritten at verify fold V1 to match D5 as
  amended.)* Out of this session's scope by the user's Q1 ruling ("rehoming not a issue"):
  where these five live after the cut — the **compliance-module content** (`COMPLIANCE-MODULES.md`'s
  trigger table and seed obligations) · the **design truth content** (platform, brand and voice,
  design principles, evidence fence, the accessibility pointer's target) · the **brownfield
  feature-map reconstruction** · the **product baselines bootstrap** (data-model, contracts,
  constraints, quickstart from code) · the **minted product-behaviour principles** D2 hands off.
  The scaffolds are not rehoming items — setup keeps writing the empty index, the spine stub with
  `Scope:`, and the design home's empty truth headings (D5). **Owner (D6):** a queued brainstorm on
  rehoming — `BACKLOG.md` item + `ROADMAP.md` Next row at this session's landing. What it must
  answer: what the design baseline's empty Accessibility heading points at once no module attaches
  (F7) · who fills a brownfield project's empty index and when · where a module's obligations are
  asserted and graded once they are not governance.
- **OQ2 — Kinako's strip.** Kinako's region carries the rows D2 sorted out and a `gdpr` module;
  under D4 they are superseded at its next amend, which is a governance event kinako owes anyway
  (F4). Whether the store rows that cite "GI-004 boundary 1" and kin re-point before or at that
  amend is kinako's call. *Not a design input for this cut (Q8).*
- **OQ3 — `Scope:` and type drift.** *Closed at review fold S5 into D4:* a `Scope:` change is
  event (3) by its engineering face, and the desk's `Scope:` write surfaces the owed amend at its
  landing; no watcher.

## Question trail (Q)

- **Q1** — the cut: which setup obligations are product-shaped and leave; the fact-profile →
  compliance-module gray zone put to the user. **Answer:** the user narrowed the session to what
  stays in setup; rehoming declared not an issue. The proposed leaves-list (product-truth leg +
  design write · feature map at close · baselines bootstrap · store scaffold + `Scope:` · minted
  product-behaviour principles · dimensions 1 and 4) stands unchallenged — carried `Assumed`
  until a decision names it. The gray item is re-put as Q2.
- **Q2** — the fact profile and its compliance modules: in or out of a product-agnostic setup.
  Lead recommended out, consequence stated (no modules, no legal-mandate stratum in governance).
  **Answer:** "out" → D1.
- **Q3** — the test that keeps dimension 9's minted values engineering-only: the rule stays, the
  product's instance of it leaves. Kinako's region sorted under the test for the user to correct.
  **Answer:** "yes" → D2, sort confirmed as put.
- **Q4** — the interrogation agenda after the cut: seven dimensions plus the depth level stay,
  dimensions 1, 2 and 4 leave; dimension 4 (risk surface) flagged as the least certain strike.
  **Answer:** "yes" → D3.
- **Q5** — what is still a governance event after the cut: a closed six-item set, product changes
  never, plus a legacy clause for an amend run meeting a product-carrying synthesis.
  **Answer:** "confiremd" → D4.
- **Q6** — setup's own rules and done condition after the cut: the rule-level strike list (twelve
  rules and one fail, two of them floors) and the goal shrunk to the governance surface set.
  **Answer:** "yes" → D5 (as first ruled; reshaped at review folds S6/S7).
- **Review round 1 dispositions** — user: "open citical, rest recommeded" → S2–S5, S7, S9, S10,
  S13 folded as recommended; S1 opened.
- **Q7** — S1's remaining limbs after S7: the shipping order (cut before the rehome, or gated on
  it) and OQ1's owner; the exposure window stated. **Answer:** "as recommded" → D6.
- **Verify round 1** — user: "as recommded" on V3 (the legacy-clause split).
- **Q8** — user, unprompted, during the delta-check: "are we fitting for legacy projects, we dont
  need to. it needs to align with going forward." **Ruling:** the V3 split reversed; D4's clause
  forward-only (supersede on sight, no carry, no marker); D6, OQ2 and build step 7 aligned;
  existing consumers are not a design input. Applied by the lead, the reviewer's V3 leg
  superseded.
