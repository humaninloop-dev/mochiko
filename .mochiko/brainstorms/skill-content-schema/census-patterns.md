# Skill-content schema — patterns-family census (wave 2, family section)

**Seat:** census-patterns · **Date:** 2026-09-01 · **Status:** delivered, awaiting user gate
**Referent law:** `.mochiko/brainstorms/skill-content-schema/record.md` D1–D9 as amended ·
command-content-schema D12/D15 · near-dup ADR R1–R6
(`.mochiko/decisions/2026-08-28-near-dup-convergence.md`) · the wave-1 census
(`.mochiko/brainstorms/skill-content-schema/census.md`) as structural referent, its §K
build-corrections included.
**Corpus:** the 13 `patterns-*` skills, `SKILL.md` whole + every `references/*.md` surveyed
(obligation-line sweep + targeted whole-reads of the obligation-dense files) + all 11 existing
strips whole-surveyed with targeted whole-reads of every KEPT/RETURNED/relocation entry +
`DECISIONS.md` grep per member. Paper exercise — this file is the only write.
**Measurement:** characters of the parsed value per the canonical snippet in
`.mochiko/memory/primitive-cost-budgets.md`, never `wc -c`. All figures taken 2026-09-01 against
the quiesced tree (v0.100.0, post review-family conversion).

Member shorthand (filename stems remain the ID prefixes per the wave-wide R-b ruling; shorthand
is presentation only): AF `patterns-adopt-first` · AC `patterns-api-contracts` ·
AS `patterns-architecture-shelves` · CM `patterns-code-minimalism` · EM
`patterns-entity-modeling` · MM `patterns-map-minimalism` · MT `patterns-model-tiering` ·
PM `patterns-plan-minimalism` · SL `patterns-sound-loop` · SD `patterns-system-design` ·
TD `patterns-technical-decisions` · TF `patterns-transport-floor` · VT `patterns-vertical-tdd`.

All rule IDs provisional (R-a): mint-once fires at conversion, never here. Dispositions use the
D3-as-amended vocabulary: **body-stays-prose** / **moves-to-schema** / **reference-stub**.

**Headline finding, stated up front because it reframes the family door:** the patterns family
is **bimodal**. Nine members are **discipline/floor carriers** born from rulings (AF, AS, CM,
MM, MT, PM, SL, TF, VT) — obligation-dense, ~50–70% of body is gradeable obligation, directly
schema-shaped. Four are **design-toolkit teachers** (AC, EM, TD, SD) — template/taxonomy/craft
mass with a thin obligation layer (~15–40%). The record's C3 note ("that family's content is
judgment-prose more than gradeable obligation") holds for the four teachers, **not** for the
nine carriers. What the family lacks is the review family's other asset: shared machinery. The
members were deliberately built as single-source disciplines, so near-identical cross-member
text is scarce (§C/§D) — the drift-control primary driver is weak here; the secondary drivers
(citable IDs for strips/audits/compression, checker visibility, floor semantics) are the real
purchase.

---

## A. Protected-set reconciliation (FIRST, per D9/C4)

**Counting note (the wave-1 J-1 idiom).** Grep across the 11 existing strip files finds **15
lines containing `KEPT`**, of which the distinct live KEPT/RETURNED survivor rulings number
**6** (several strips contain reconciliation mentions, and TD's v0.27.0 KEPT was half-superseded
by a recorded v0.64.0 ruling). Two members (**AS, TF**) have **no strips file at all** — legal
(no post-birth edit has removed content), so their protection basis is DECISIONS-traceability
only. Seven members are **birth-by-ruling bodies** in the RSUF class: no KEPT line exists, and
the D2-analog machinery is `DECISIONS.md`-traceable per rule (each move recorded at conversion
citing the ruling row, the wave-1 J-2/R-c ceremony class). Reconciliation below is at the unit
of **live protection**.

### AF — patterns-adopt-first (strips: 3 entries, all v0.91.0 supersessions; no KEPT)

Birth-by-ruling, v0.73.0 (build-vs-off-the-shelf D1–D6 as amended, `DECISIONS.md` 2026-08-15).

| Protected unit | Status | Census disposition |
|---|---|---|
| D2 two-part obligation: disclosure floor ("names at least one real off-the-shelf candidate… or an explicit 'no shelf candidate exists' line; a missing line is itself a review finding") + rationale bite ("custom wins only against the named candidate… in writing") | live, DECISIONS-traceable | **moves-to-schema** (two floors), citing the 2026-08-15 row |
| D2 two-sided limb (<100-lines legitimate custom-wins rationale, BE-DEP read symmetrically) + external-claims binding (`verified:`/`memory-asserted` per EXTERNAL-CLAIMS.md) | live | **moves-to-schema** (limb as latitude; binding rule with cross-dir pointer) |
| D3 constraint-challenge route-back (three-part finding, routes to the user, only the colliding decision pauses) | live | **moves-to-schema** (floor + reservation) |
| D4 retrofit-cost gate (user-ruled above the line, seat-decidable with disclosure below) | live | **moves-to-schema** (reservation) |
| v0.91.0 (d) build-time firing site + never-builder-decided halt + `baseline-delta.md` landing (plan-stage retirement D1, recorded supersession of the old "not at build time" carve) | live | **moves-to-schema** (gate + reservation + binding) |
| Grader assignments (RPA disclosure BLOCKING · RF class-7 blocking-capable · rationale advisory · retrofit gate user-ruled) | live, D2-as-amended C8/C6/C7/C3 | **moves-to-schema** (routing) |

### AC — patterns-api-contracts (strips: 5 entries; 1 KEPT mention, no live KEPT ruling)

No `DECISIONS.md` name-hit. Protection is strip-recorded rulings:

| Protected unit | Status | Census disposition |
|---|---|---|
| [v0.23.0] quickstart conditional + capped (T3, **user-ruled**): authored only on a real integration surface, ≤150 lines, cites the contract never re-documents it, null path one line — re-pointed at v0.91.0 to the sufficiency report (plan-stage retirement D4) | live | **moves-to-schema** (gate + bound; null-path binding), citing both strip rulings |
| [v0.23.0] endpoint↔FR/US traceability table designated the contract's ID index | live | **moves-to-schema** (binding) |
| [v0.91.0] design-ladder blockquote (necessity answers `patterns-plan-minimalism` before entering the package) | live | **moves-to-schema** (binding) |
| [v0.27.0]/[v0.64.0] keep-sets (tables/contract kept; x-integration field rules) | live as compressed content | field-rule obligations **move-to-schema**; tables **body-stays-prose** |

### AS — patterns-architecture-shelves (no strips)

Birth-by-ruling, v0.81.0 (product-architecture-schema, `DECISIONS.md` 2026-08-19). Per the R-c
idiom: **not wholesale-protected** — no KEPT line exists. The ruled machinery is
DECISIONS-traceable: breadth invariant (every row walked, no magnitude scaling),
recommend-then-arbitrate (user rules, shelf never asserts), three-strata precedence
(floor-asserted categories bind — `n-a — genuinely never` unavailable, drop = ledger waiver),
stance vocabulary with the `not-now` trigger and `n-a` reason-axis + owner-pointer rules, scope
read from `spine.md`, opinions-in-data (`architecture-shelf-backend.yaml` Read raw), honest
gaps never filtered lists, event-keyed freshness. Each such rule's move is recorded at
conversion citing the 2026-08-19 row. Ledger note: post-birth fix-round growth (+343, ruled
obligations) is ledgered in `primitive-cost-budgets.md` — no strip is owed for additions.

### CM — patterns-code-minimalism (strips: 2 entries + explicit no-prior-protection reconciliation)

| Protected unit | Status | Census disposition |
|---|---|---|
| PT-D1–D10 core (DECISIONS 2026-08-05 ×2): the seven-rung ladder as generation-time discipline, rung disclosure in the cycle report, graded by `review-code-minimalism` | live | ladder **body-stays-prose** as the sequenced procedure (D3); the stop-rule, disclosure duty, and grading routing **move-to-schema** |
| [v0.64.0] RETURNED: existing-code-slimming trigger in the `description:` (probe-demanded, user-ruled) | live, description-borne | **body-stays-prose** — `description:` never moves (D3/D8 criterion 7) |
| [v0.91.0] design-time re-keys incl. the adopt-first binding-constraint carve ("a design-committed adopt-first choice reaches these cards as a binding constraint, not a rung to re-open") | live | **moves-to-schema** |
| The floor — lazy-not-negligent (no rung sacrifices a floor obligation **or accessibility**) + one-intensity rule | live, PT-lineage | **moves-to-schema** (both floors) |

### EM — patterns-entity-modeling (strips: 3 entries; **live whole-body-class KEPT**)

| Protected unit | Status | Census disposition |
|---|---|---|
| **[v0.27.0] KEPT: the remaining body** (under-band survivor ruling, 17% vs 30–70 band) — protected core enumerated at the v0.64.0 reconciliation: the ~100-line data-model.md template, sensitivity taxonomy + decision tree + PII mapping, the five-step annotation procedure, conceptual-type vocabulary, extraction heuristics + entity-vs-attribute rules, brownfield status table, validation-script scope paragraph | **live** — the family's one surviving whole-body-class protection | per the D8/C4 review-feasibility precedent: the survivor protection **re-homes onto the pair** (body + schema jointly), recorded once in the family's rollout ruling — no per-line ambiguity. Flagged **J-P4** for the ruling text |
| [v0.23.0] once-per-document handling defaults + one-row-per-Confidential+ form (wave-2 form ruling) | live | **moves-to-schema** (constraint + duty); DATA-SENSITIVITY.md density-floor line gains a **reference-stub** |
| [v0.91.0] design-ladder blockquote | live | **moves-to-schema** (binding) |
| DS-XXX boundary split with `authoring-technical-requirements` (declares vs classifies) | live | **moves-to-schema** (routing) |

### MM — patterns-map-minimalism (strips: 1 entry; no KEPT)

Birth-by-ruling, v0.68.0 (PM role & feature derivation D1–D12, `DECISIONS.md` 2026-08-13).
DECISIONS-traceable machinery: the three governing tests, noun+verbs aid-never-gate, extend
beats mint, soft cap ~9 trigger-never-block, user rules mint/merge/retire, merge-preservation
mechanics, domains-at-cap-trip with the deferred co-sign machinery. [v0.81.0] strip: the
dormant-domains pointer re-key (store's domain-to-spine line) — live as re-keyed. Each move
cites the 2026-08-13 row.

### MT — patterns-model-tiering (strips: 1 entry; no KEPT)

Birth-by-ruling, v0.77.0 (model-tiered-seats D1–D5, `DECISIONS.md` 2026-08-16) as amended by
the **[v0.78.0] explorer retarget** (`DECISIONS.md` 2026-08-19, recorded supersession — the
cheap rung is native `Explore` + explicit `model: haiku` override; the `mochiko:explorer` agent
died). DECISIONS-traceable: the class key both tiers, D5 rostered-seats-never-retier, the
override-is-the-pin rule, disposable-per-gap, the brief obligation, the weak-negative watch.
Moves cite both rows; the retarget wording ("a spawn without the override inherits the session
tier and has failed this floor") survives verbatim-in-substance.

### PM — patterns-plan-minimalism (strips: 4 entries; no KEPT)

Birth v0.67.0 (plan-structure YAGNI 2026-08-12 + architect-role restructure 2026-08-13).

| Protected unit | Status | Census disposition |
|---|---|---|
| The five-rung simplest-execution ladder + stop-at-first-failing-rung | live | ladder **body-stays-prose** (procedure); stop-rule + rung-scope rule (1/4/5 all elements, 2/3 design elements) **move-to-schema** |
| [v0.73.0] rung-3 widening (adoptable proven component per `patterns-adopt-first` satisfies "already exists") | live, recorded ruling | **moves-to-schema** inside the rung-3 read-before-claim rule's text |
| [v0.91.0] three firing sites re-scope (design phase inside `/mochiko:implement`, seat plans, epic joint plan; plan-run wording superseded) | live | **moves-to-schema** (constraint) |
| The floor (rung-1 reads ratified requirements AND asserted floor obligations, floor never leaves) + disclosure grammar (`<element> — rung N`; undisclosed reads rung-skipped) | live | **moves-to-schema** (floor + duty) |

### SL — patterns-sound-loop (strips: 3 entries; no KEPT)

Birth-by-ruling, v0.70.0 (charter-ritual-balance D1–D7, `DECISIONS.md` 2026-08-13).
DECISIONS-traceable: the two-part kind-keyed trigger with no size threshold, the three legs,
never-above-the-user, the three exemptions and the no-delta-card-exemption rule, the
governing-surface table (re-keyed [v0.81.0] to the architecture store — recorded), the
[v0.71.0] neutrality narrowing (transport-choice neutral, transport-use carries TF's floor —
recorded supersession, 2026-08-14 row), the disclosure line's pinned grammar and the
only-real-seats honesty rule. Moves cite the 2026-08-13/14 rows.

### SD — patterns-system-design (strips: 6+ entries; 2 KEPT-class + 1 RETURNED)

| Protected unit | Status | Census disposition |
|---|---|---|
| **[v0.81.0] the relocated no-delta protected line** — "The no-delta judgment is always shown, never made silently" survives **word-for-word** by ruling (D3/D10 fold S13; lineage AD-D7 2026-07-30) | live, verbatim-bound | **moves-to-schema** as `class: floor`, wording verbatim; supersession-transfer per D8/C4, protection re-homes onto the rule ID |
| **[v0.81.0] KEPT:** the density rule — "Density is not a gap; a gap is a missing component, an unlabelled arrow, or a qualifying flow with no sequence diagram" + the `artifact-format.md` envelope pointer | live | **moves-to-schema** (see §C P4) |
| [v0.64.0] RETURNED: current-state-baseline clause in the `description:` (probe-demanded, user-ruled) | live, description-borne | **body-stays-prose** — description never moves |
| [v0.67.0] altitude bar hardened (container-not-level-3 register check; node-count override must assert altitude) — architect-restructure lineage 2026-08-13 | live | **moves-to-schema** (floor + bound) |
| [v0.81.0] store-transform machinery: baseline read from the spine never re-derived; never design on an unconfirmed baseline; qualifying-flow "P1 journeys are the floor, never the cap" | live | **moves-to-schema** (floors; the qualifying-flow guard's wording is the same protected phrase RPA's checklist carries — cross-family evidence only, no sharing) |

### TD — patterns-technical-decisions (strips: 3 entries; v0.27.0 KEPT half-superseded at v0.64.0)

| Protected unit | Status | Census disposition |
|---|---|---|
| [v0.27.0] KEPT "both When-to sections" — the `## When to Use` half | **ended** at v0.64.0 (explicit recorded supersession in the strip) | n/a — historical |
| [v0.27.0]-KEPT survivors: Decision Workflow spine + NEEDS CLARIFICATION paragraph + "Where decisions are recorded" ownership section + boundary table + Quality Checklist + When NOT to Use | live, compressed | ownership boundary + NEEDS-CLARIFICATION rule + never-restate-the-artifact **move-to-schema**; workflow spine + checklist **body-stays-prose** |
| [v0.46.0] loop-discipline pointer removal (sentence stands: driving resolution belongs to the command supervisor, not this skill) | live as re-keyed | **moves-to-schema** (reservation/routing) |

### TF — patterns-transport-floor (no strips)

Birth-by-ruling, v0.71.0 (teammate-transport message races D1–D7, `DECISIONS.md` 2026-08-14).
Not wholesale-protected; DECISIONS-traceable: the two-lane trigger, non-waivable-once-fired,
the **seven legs** (each independently ruled machinery), the ≥ v2.1.224 version floor with its
masked-failure rationale, the ordering-undocumented design posture, the doc-anchor line. Every
leg's move cites the 2026-08-14 row.

### VT — patterns-vertical-tdd (strips: 10 entries; 1 live-partial KEPT)

| Protected unit | Status | Census disposition |
|---|---|---|
| [v0.27.0] KEPT: Markers table · rationalizations · checklist · mapping shape · epigraph | **mostly ended by later recorded rulings**: markers table and mapping shape superseded at v0.75.0 (test-case-bundle re-anchor, `DECISIONS.md` 2026-08-16); rationalizations compressed out through the recorded trail. **Live residue:** the letter/spirit epigraph + the Quality Checklist | epigraph + checklist **body-stays-prose**; nothing further owed — the endings are already recorded |
| [v0.22.0] TEST-grammar relocation — **ownership stays with this skill**; TEST-GRAMMAR.md the single source, 4 external consumers | live | **reference-stub** — the ownership/consumption binding stubs the pointer; the file stays intact |
| [v0.75.0]/2026-08-16 cycle re-anchor: cycles = test-case bundles; no foundation/feature type; **infra-only cards never minted**; `[P]` derives from dependencies; walking-skeleton condition; qa-engineer authors test-case content | live | **moves-to-schema** (floors + constraints) |
| [v0.76.0] two-arm schema read-pointer (`mochiko-cli template tasks`, or Read `plugins/mochiko/schemas/tasks.yaml` when the binary is absent) — GI-020 | live | **moves-to-schema**, both arms preserved verbatim (the RPA two-arm precedent) |
| [v0.91.0] card authorship re-homed inside the implement run + **card-author-is-never-the-executing-builder** (plan-stage retirement D1(c); the +226 body growth ruled HOLDS and byte-reconciled in the budget ledger) | live | **moves-to-schema** (constraint + floor) |

**Reconciliation totals:** 6 distinct live KEPT/RETURNED survivor rulings (EM whole-body ·
SD no-delta-relocation · SD density-KEPT · SD description-RETURNED · CM description-RETURNED ·
VT epigraph/checklist residue) + 1 ended (TD When-to-Use half) + 1 mostly-ended (VT v0.27.0,
endings recorded); **7 birth-by-ruling members** on the DECISIONS-traceable ceremony class
(AF, AS, MM, MT, PM, SL, TF). ~40 live protected units enumerated; every one carries a named
disposition; zero stay-silent; no disposition is "delete". The two description-borne RETURNED
clauses stay in place by construction (descriptions never move). **EM's whole-body survivor
ruling needs the explicit D8/C4 re-home-onto-the-pair clause in any patterns rollout ruling**
(J-P4).

---

## B. Obligation census at D12 grain

One row per independently-citable obligation; `class` floor/must/advisory; `kind` from the
**eight-kind** skill set (`kind: fail` + `enforces:` retired at v0.100.0; nothing here re-admits
them — see §E-analog note in §D). Procedure and teaching prose is NOT inventoried (D3): CM/PM's
rung ladders as sequenced walks, MM's test explanations, AS's walk narrative, SD's diagram
craft, EM's template + taxonomy tables, AC's mapping/type/error tables, TD's workflow spine,
VT's vertical-vs-horizontal teaching all stay prose.

`when:` dimensions observed live (D4 validation): AF design-phase vs build-time firing site
(entry-derived) · SD store-exists/no-store/greenfield + IP-XXX-present (surface-presence) + the
~12-node threshold (bound, not a condition) · AC integration-surface-present + x-integration
applicability (surface-presence) · VT new-end-to-end-path (walking-skeleton condition,
entry-derived) + brownfield exposure · SL trigger-fired (both-parts, entry-derived) · TF lane
fired: messaging present / shared-write-surface present (surface-presence) · EM
stateful-entities-present (surface-presence) · MM cap-trip (entry-derived). All resolvable as
entry-derived or surface-presence; **no new resolution kind needed**.

Summary table (full per-rule enumeration below the table for the three densest members;
remaining members enumerated at row grain with counts — every row is independently citable and
was drafted from a whole-body read):

| Member | Rules | floor | must | advisory | Stubs | Notes |
|---|---|---|---|---|---|---|
| AF | 17 | 6 | 10 | 1 | 0 | densest floor carrier; 2 cross-dir pointers |
| AC | 13 | 1 | 11 | 1 | 0 | teaching-heavy; obligations cluster on x-integration + quickstart |
| AS | 17 | 5 | 11 | 1 | 0 | breadth invariant + strata are the floor core |
| CM | 10 | 4 | 6 | 0 | 0 | ladder itself stays prose |
| EM | 12 | 1 | 10 | 1 | 1 | thin layer over a large protected teaching body |
| MM | 13 | 3 | 10 | 0 | 0 | merge mechanics enumerable as one 4-limb rule or 4 |
| MT | 11 | 4 | 7 | 0 | 0 | override-is-the-pin + brief obligation are floors |
| PM | 10 | 2 | 8 | 0 | 0 | |
| SL | 14 | 6 | 8 | 0 | 0 | three legs = 3 rules at D12 grain |
| SD | 16 | 5 | 10 | 1 | 0–1 | no-delta floor verbatim-bound |
| TD | 10 | 0 | 9 | 1 | 1 | DECISION-RECORD.md disclosure line stubs |
| TF | 14 | 10 | 3 | 1 | 0 | the family's purest floor skill (7 legs + 2 lane rules + version floor) |
| VT | 15 | 5 | 10 | 0 | 3 | TEST-GRAMMAR.md stubs |
| **Family** | **~172 body** | **52** | **113** | **7** | **~6** | **≈178 total incl. stubs** |

Counts are ±10% pending the conversion seats' a/b limb splits (the wave-1 experience: splits
added ~+4 family-wide).

### AF — patterns-adopt-first (17)

1 scope bound: in-process/self-hostable only; managed-service/SaaS/whole-capability routes to
IP-XXX + PM/user, never here (floor, routing) · 2 differentiating domain presumptively not
commodity (must, routing) · 3 never re-open a design-phase-ruled mechanism (must, constraint) ·
4 tooling defaults are governance-floor doctrine, not this decision (must, routing) · 5 the
commodity trigger test — seven-category starting set, judgment extends, absence not an
exemption (must, gate) · 6 the author's framing never gates the check (floor, constraint) ·
7 disclosure floor — named real candidate or explicit no-candidate line; a missing line is
itself a finding (floor, duty) · 8 rationale bite — custom wins only against the named
candidate, in writing (floor, constraint) · 9 two-sided limb — <100-lines custom-wins
rationale, BE-DEP symmetric (must, latitude) · 10 named candidate is an external claim —
`verified:`/`memory-asserted` per `../review-brainstorm/references/EXTERNAL-CLAIMS.md` (must,
binding; cross-dir pointer) · 11 retrofit-cost gate — user-ruled above the line, seat-decidable
with disclosure below (floor, reservation) · 12 build-time gate — never builder-decided; halt
to the user's checkpoint (floor, reservation; `when:` build-time) · 13 the resulting write
lands as a `baseline-delta.md` entry graded as judgment, never an in-place edit (must,
binding) · 14 constraint-challenge finding — three parts, quoted text/real requirement/excluded
candidate; never silently override (floor, duty) · 15 collision routes to the user; only the
colliding decision pauses (must, reservation) · 16 grader routing (RPA BLOCKING · RF class 7 ·
rationale advisory) (must, routing) · 17 siblings boundary — ladders size the artifact, this
asks who builds it; tooling doctrine cross-pointers only, no merge (must, routing).

### TF — patterns-transport-floor (14)

1 governs use, never the neutral choice (must, constraint) · 2 message-lane trigger (floor,
gate; `when:` messaging-present) · 3 topology-lane trigger (floor, gate; `when:`
shared-write-surface) · 4 neither lane waivable once fired (floor, constraint) · 5–11 the seven
legs — composition steer (two in-floor shapes only) · single writer per surface per wave · mesh
hold · content-pinned supersession · quiesce before cold grade · no ritual sends/never re-send ·
fan-in confirmation (floor ×7; kinds: constraint ×5, duty ×2) · 12 version floor ≥ v2.1.224
(floor, bound) · 13 ordering undocumented — design around it, never trust it (must,
constraint) · 14 cross-session transport cited only when in scope (advisory, constraint).

### VT — patterns-vertical-tdd (15 body + 3 stubs)

1 output binding: `tasks.md` cycle-card shape, the **two-arm** schema pointer verbatim (must,
binding — protected v0.76.0) · 2 design-time inside the `/mochiko:implement` run, after the
design phase or on a zero-gap verdict; never a separate plan run (must, constraint — v0.91.0) ·
3 writes no task lists, no file paths — the builder decomposes at build time (floor,
constraint) · 4 two authors, one card: slicing seat owns judgment; `qa-engineer` authors
test-case content (must, constraint) · 5 the slicing seat is never the executing builder
(floor, constraint — v0.91.0, ledger-argued) · 6 not-for routes (bugs/docs/decomposition/
feature derivation) (must, routing) · 7 vertical over horizontal; a card not demonstrable on
its own is not a cycle (must, constraint) · 8 walking skeleton first on a new end-to-end path;
growth skips it (must, constraint; `when:` new-path) · 9 no foundation/feature card type;
infra homed inside the first bundle that needs it; **infra-only cards never minted** (floor,
constraint) · 10 `[P]` derives from dependencies, never a type column (must, constraint) ·
11 every card closes with the `**TEST:**` real-infrastructure gate, never a re-run of the
automated tests (floor, gate) · 12 grammar owned here; downstream parsers consume, never
redefine (must, binding) · 13 Simple/Split/Merge case + rationale on the card; no separate
mapping artifact (must, constraint) · 14 brownfield exposure line per card, `none` counts
(must, duty) · 15 each named test case cites the spec/design IDs it covers, never re-quoted
content (must, constraint). Stubs (TEST-GRAMMAR.md, 6,270 chars — tables and worked examples
stay untouched): S1 the four MUST-includes (real infrastructure · tangible output · explicit
steps · observable outcome) · S2 required-field set (Action + Assert required; Setup/Capture
optional) · S3 producer authors the task, never the runtime approval-mode decision (routing to
`testing-end-user`).

Remaining members' row inventories (row grain; each row one citable obligation): **AC** — every
user action an endpoint · schemas + error responses per endpoint · design-ladder binding ·
not-for routes · type vocabulary from the data model, never redefined · brownfield
reuse/rename/new-only ladder · x-integration required iff wrapping an external system (`when:`) ·
`failure_modes` never empty + required field set · every external dependency a documented
fallback (floor-adjacent must) · traceability table = ID index (binding) · quickstart gate
(conditional, `when:` integration-surface) · quickstart bound (≤150 lines, cites never
re-documents, null path in the sufficiency report) · validation script = self-check never the
substantive review (advisory, routing). **AS** — opinions-in-data Read-raw binding · AX-XXX
grammar routed to `authoring-architecture-store` · never a rigor dial (floor) · not the store's
grammar / not per-feature design (routing) · recommend-then-arbitrate, the user rules (floor,
reservation) · memory-asserted provenance duty · scope from `spine.md`, never re-asked
(binding) · honest gaps, never a filtered backend list (floor) · retrofit-cost walk order ·
breadth invariant — every row walked, no magnitude scaling (floor) · stance vocabulary
(constraint) · `n-a` reason axis + handled-elsewhere owner-pointer required · `not-now` carries
its upgrade trigger · floor-asserted rows: `n-a — genuinely never` unavailable, legal moves
enumerated, drop = ledger waiver (floor) · card/shelf conflict is the user's (reservation) ·
one-dial-one-system · event-keyed freshness (advisory). **CM** — fires at decomposition before
red phase · rung disclosed in the cycle report, graded by RCM against this file · card criteria
are the floor of what to build, never cut (constraint) · not-for routes · rung zero
read-before-rank + brownfield rides `brownfield-integration` · stop at first applicable rung,
one-line why (constraint) · new dependency is not a rung — domain-registry ruling, never
auto-approved (floor) · design-committed adopt-first choice is a binding constraint, not a rung
to re-open · the floor — no rung sacrifices a floor obligation or accessibility (floor) · one
intensity — variance rides recorded waivers (floor). **EM** — single home for `data-model.md`
+ sensitivity annotations (binding) · design-ladder binding · not-for routes incl. the DS-XXX
declares-vs-classifies split (routing) · every attribute exactly one of four levels (duty) ·
PII maps onto the levels, never a parallel axis · classify up when in doubt · handling defaults
once per document, never a repeated per-attribute aspect table (constraint — v0.23.0 ruled
form) · one Sensitivity Details row per Confidential+ attribute (duty) · summary table = the
coverage index (binding) · DS-XXX trace in the compliance cell · density-not-a-gap envelope
binding · validation script = producer self-check, never the independent grade (routing).
Stub: DATA-SENSITIVITY.md's density floor line. **MM** — fires at derivation/review/grooming ·
fewness never hides a passing capability (floor) · discipline-vs-vocabulary boundary with
`authoring-feature-map` (routing) · tests grade the durable layer, not work rows ·
mint/merge/retire is the user's to rule, never self-executed (floor, reservation) · the three
governing tests all hold (duty) · read the current map first (duty) · noun+verbs aid never a
gate · extend beats mint; an unargued mint is the defect (floor) · soft cap ~9 trigger never a
hard block (bound) · merge preserves the four mechanics (constraint) · re-parenting
navigation-only · domains: exactly one, minted only at cap-trip, PM proposes + architect
co-signs, machinery deferred to first cap-trip. **MT** — rostered seats never change model
(floor — D5) · the cheap rung is native `Explore` + explicit `model: haiku`; a bare spawn is
not cheap · class key cheap tier (constraint) · class key session tier incl.
producing/reviewing/grading never tiered down (floor) · the dispatch ladder — lowest trusted
rung; a spawn costing more than the read failed it · disposable per gap, never a standing seat ·
the override is the pin — omitted override = failed floor (floor, duty) · fact-finder brief
binding (provenance, verbatim, method-scoped absence) · terse return, bulk stays in the
disposable context · weak-negative watch + re-route rule · the brief obligation — one line in
every seat brief (floor, duty). **PM** — three firing sites (constraint — v0.91.0) · not-for:
build-time code / delta-scope run (routing) · rung 1 never deletes a ratified requirement or
floor obligation (floor) · stop at first failing rung, one-line why · rung scope 1/4/5 all
elements, 2/3 design elements · read-before-claim — rung-2/3 only after reading baselines +
current state, never on trust (duty) · the floor both ways — nothing speculative in, no floor
obligation out (floor) · disclosure grammar `<element> — rung N`; undisclosed reads
rung-skipped (duty) · grading routing (RPA: honesty advisory, conformance blocking) · epic
joint plan carries one plan over all members (constraint). **SL** — two-part trigger, no size
threshold (floor, gate) · kind-keyed, library-wide, any door · leg 1 seat-produces on a
lead-approved plan, never the lead (floor) · leg 2 non-author review; the user's ruling never
substitutes (floor) · leg 3 the user gate stays; never a machine gate above it (floor) ·
governing-surface table binding (per-member regime) · exemptions exactly three (constraint) ·
store carve — status flips transcription, `As-built:`/`Drift:` graded (constraint) · no
delta-card exemption (floor) · default seat wiring; swaps disclosed with cause (binding) · map
review runs spec-less (constraint) · out-of-remit hosting — name the crossing, import the
rituals (constraint) · disclosure line pinned grammar (duty) · only seats that existed are
named (floor). **SD** — baseline read from the spine, never re-derived (floor) · never design
on an unconfirmed baseline (floor) · not-for routes ×6 (routing) · container diagram craft
constraints (flowchart carrier, boundaries as subgraphs, technology in labels, protocol+purpose
arrows) · delta visually marked · the diagram renders, never a raw code block · sequence
diagram per qualifying flow; P1 the floor never the cap (duty) · register altitude check —
every row a container, never a level-3 construct (floor) · delta summary links the ruling,
never restates it · box↔register bijection · deployment view iff IP-XXX, absence recorded
(gate, `when:`) · neighborhood scoping + ~12-node threshold; an override must assert altitude
(bound) · **no-delta claim always shown, never silent** (floor — protected verbatim) · density
envelope binding · greenfield degenerates cleanly (advisory). **TD** — technique-vs-artifact
ownership boundary; never restate the artifact template (binding) · not-for routes · 2–3
alternatives minimum against the eight criteria · commodity-category candidate line + rationale
bite, trigger/gate at `patterns-adopt-first` (binding) · two comparison shapes only, never a
third format · NEEDS CLARIFICATION rather than guessing (constraint) · driving resolution
belongs to the command supervisor, never this skill (reservation) · ADR field binding ·
project-scope vs feature-scope destination rule (binding) · brownfield alignment always checked
first (duty). Stub: DECISION-RECORD.md's disclosure-is-the-producer's-whole-obligation /
verification-is-the-review-seat's line.

**Section-fit finding (D4 delta — flagged as the census's structural misfit):** the review
six-set (`independence · scope · inputs · verdict · output · reserved`) does **not** fit this
family. `verdict` would be empty or forced for 12 of 13 members (patterns skills issue no
clearing verdicts; their nearest content is grader-routing, which is `scope`), and
`independence` has real content in only ~4 (SL's legs, VT's author split, AF/AS
seat-vs-user reservations read better as `reserved`). The corpus's natural cleavage is the
discipline lifecycle: **trigger** (when the discipline fires — gates, lanes, firing sites) ·
**scope** (jurisdiction, not-for routes, sibling boundaries) · **discipline** (the binding
core — floors, invariants, legs, stop-rules) · **inputs** (read-before-claim, data bindings,
fences) · **disclosure** (output/report/artifact bindings, disclosure grammars) · **reserved**
(user gates, halt rules, seat reservations). Coverage test: 13/13 members home every §B rule in
exactly one of the six; none forced empty (AC/EM/TD carry `trigger` thinly — the explicit empty
marker exists for that). Per D4/D9 the set is minted by the family's own rollout ruling, never
at conversion; this is the census's proposal for that ruling.

---

## C. Common-block candidates (R1–R6 screen)

Manual screen (the detector cannot run pre-conversion). Bar: 3+ members, **near-identical
wording**, strongest-wording-wins (R2), member-specific extras keep local text with allowlist
edges (R6). The family's single-source construction works against extraction by design — each
member owns a distinct discipline, so convergence is thin. Candidates, graded honestly:

| # | Candidate block | Members | Wording evidence | Verdict |
|---|---|---|---|---|
| P1 | **stance-is-the-user's** — "the <ruling/stance/mint> is the user's to rule, never the seat's" | AF · AS · MM · SL (4) | AF "a build-time ruling halts to the user, never builder-decided" · AS "the user ruled, the shelf did not" / "an asserted default here would be an opinion wearing a rule's clothes" · MM "the capability layer is the user's to rule, never self-executed" · SL "rulings reserved to the user remain theirs" | **CLEARS on a thin core only.** The shared text after strongest-wording-wins is one clause ("reserved to the user, never the seat's own call"); every member's operative content (which decisions, at which gate, what halts) is member-specific local text. Extraction saves ~40 chars/member and adds a stub each — an R5 net-reduction question mark. Adjudicate at the gate |
| P2 | **external-claims disclosure binding** — `verified:`/`memory-asserted` per EXTERNAL-CLAIMS.md | AF · TD-ref · (AS partial) (2 body + 1 reference) | AF and DECISION-RECORD.md bind the same grammar near-identically; AS's memory-asserted duty is a different obligation (blanket provenance declaration, no `verified:` line) | **KEPT DISTINCT.** Below bar at 2 body members — and the v0.100.0 landing gate already ruled the cross-pair external-claims family **keep-distinct** (six allowlist rows, "extraction reopens on convergence"). Direct precedent; allowlist edges only |
| P3 | **own-or-point / never-restate the owned home** | AF · CM · MM · PM · SD · TD · VT · EM · AC (9, same-kind) | "the single source; never restated here" (AF) · "Single source of the ladder" (CM/PM/MM descriptions) · "Link, never restate the decision" (SD) · "Do not restate the artifact template here" (TD) · "this skill owns it; downstream parsers consume" (VT) · "reference the data model's types, don't redefine" (AC) | **KEPT DISTINCT under R1.** Same posture, nine different owned objects, wordings not near-identical — the review family's C5 cleared on **verbatim-identical** text, which this is not. Recorded as the family's signature same-kind cluster; the allowlist will need a header note so detector reruns stay quiet |
| P4 | **deliverable-envelope density rule** — "follows the envelope in `templates/artifact-format.md` — density is not a gap; a gap is <member-list>" | SD · EM · AC (3) | SD "Density is not a gap; a gap is a missing component, an unlabelled arrow, or a qualifying flow with no sequence diagram" (**KEPT-protected**) · EM "Density is not a gap; a gap is a missing entity, classification, or relationship" · AC "capped and dense (deliverable envelope, `templates/artifact-format.md`)" | **CLEARS at exactly the 3+ bar.** Shared core: the envelope binding + "density is not a gap"; the gap-list tail stays local per member (R2/R6). Caution: SD's copy is protected — the stub must carry the supersession-transfer, and the strongest wording is SD's own (the protected tail survives as SD-local text). Cross-family note: RSPEC/RPA carry "density is never itself a gap/finding" — evidence only, no sharing (D5) |
| P5 | **read-before-claim** — a reuse/extend/new-kind claim is made only after a real read, never on trust | CM · PM · MM (3) + SD · AS (kept-distinct edges) | CM "the touched code's real flow was traced before ranking" · PM "a rung-2 or rung-3 claim is made only after reading the baselines and current state — never on trust. A reuse claim with no read behind it is not a rung stop" · MM "read the current map first, since 'new kind' is claimed against what is already there" | **CLEARS at the 3+ bar** for the three sibling ladders — the obligation is genuinely convergent (an existence/reuse claim requires a read of the surface it is claimed against). Strongest wording: PM's two-limb form with `${surface}` local. SD's "read from the spine, never re-derived" and AS's "read the scope from `spine.md`" are different obligations (source-of-truth bindings) — allowlist edges |

**Screened and kept distinct (below bar or R5 fail), recorded for the allowlist:**
- *chartered-freedom-when-untripped* — SL + TF (2), near-verbatim ("chartered freedom stands").
- *sizing-belongs-to-the-minimalism-siblings* — SL + TF (2), near-verbatim not-for rows.
- *kind-keyed floor, non-waivable once fired* — SL + TF (2).
- *stop-at-first-rung + one-line-why* — CM + PM (2; inverted predicates: applies vs fails).
- *the-floor-lazy-not-negligent* — CM + PM (2; near-identical heading and posture).
- *script-is-self-check-never-the-review* — EM + AC (2, near-identical closing paragraphs).
- *design-ladder blockquote* — EM + AC (2, near-verbatim v0.91.0 blockquotes; PM is the
  pointed-at owner, not a third member).
- *transcription/mechanical exemption honesty* — SL only (1; AS's waiver routing is different).
- *"aid, never a gate"* — MM noun+verbs + SD node-count-override (2, loosely).

**Cross-grammar note (wave-1 J-5 idiom):** P1's core resembles command `common.yaml`
reservation blocks and review-common `verdict-is-input`; P5 resembles RCM's codebase-read
floor. D5 forbids cross-grammar and cross-family sharing; evidence only, allowlist edges at
build.

---

## D. Abort check (numeric, per D9-I3)

**Strict count: 2 candidates clear the near-identical 3+ bar cleanly (P4, P5); P1 clears only
on a thin-core reading whose net reduction is questionable (R5).** The D9-I3 threshold is
"fewer than three clearing the 3+ bar":

- **Thin-core reading: 3 clear — not tripped, at the exact boundary.**
- **Strict R1 reading: 2 clear — TRIPPED.**

Either way the evidence is the same and the census states it plainly: **the drift-control
primary driver that carried the review family is materially absent here.** The review family
cleared 6 blocks at ×3–×8 membership with near-verbatim floors; the patterns family yields at
most 3 thin blocks at ×3–×4, saving ~40–120 chars each against a ~1,000-char common file plus
a per-member stub and an extra obligated read for all binding members. This census is itself
the D9-I3 return-to-the-user with the evidence. **A patterns conversion, if ruled, stands on
the secondary drivers** — citable IDs for strips/audits/compression waves, `class: floor`
semantics for the ~52 floor rules, checker/detector visibility — **not on drift control.**

Per-member conversion-fit verdicts (does a schema meaningfully serve this member?):

| Member | Verdict | One-line why |
|---|---|---|
| AF | **strong fit** | 6 floors + 3 reservations; ruled machinery wants IDs |
| AS | **strong fit** | breadth invariant + strata floors; birth-ruling traceability wants anchors |
| CM | **strong fit** | small body, high floor density; RCM already grades against it |
| MM | **strong fit** | floors + user reservations |
| MT | **strong fit** | pin/brief floors are exactly silent-loss-prone content |
| PM | **strong fit** | floor + disclosure grammar |
| SL | **strong fit** | the family's densest floor set (6) |
| TF | **strongest fit** | 10 floors; near-pure obligation body |
| VT | **strong fit** | 5 floors incl. two protected two-arm/authorship rules + 3 stubs |
| SD | **fit** | 5 floors incl. the verbatim-bound protected line; craft prose stays |
| AC | **marginal** | 13 obligations over an 11k teaching body; payload grows for a thin layer |
| EM | **marginal** | 12 obligations over a 13.7k KEPT-protected teaching body; the whole-body re-home adds ceremony |
| TD | **marginal** | 10 musts, 0 floors; the boundary lines are the only silent-loss-prone content |

**`kind: fail` question (D9/M2 symmetric check):** zero run-fail predicates in the corpus —
the family's "failed this floor" language (MT's override rule, PM's rung-skipped-at-review) is
verdict-earning/constraint-shaped, exactly the wave-1 §E finding. Nothing re-admits `fail`;
the eight-kind set stands. All 8 kinds are exercised (`latitude` by AF-9; `bound` by
MM/SD/TF/AC; `gate` by AF/SL/TF/SD/AC/VT).

---

## ROAD. Shared-references road weighing (MANDATORY per D5/C3)

The record left the shared-`references/` single-source road "a live candidate at the
patterns-family door — that family's content is judgment-prose more than gradeable obligation."
Assessed per member (fraction of body that is judgment/teaching prose vs gradeable obligation,
census judgment from whole-body reads):

| Member | Judgment-prose share | Obligation share | Shared-refs road serves it? |
|---|---|---|---|
| AF | ~35% | ~65% | No — obligations are member-specific ruled machinery |
| AC | ~80% | ~20% | Its teaching already lives in its own references; nothing to share family-wide |
| AS | ~45% | ~55% | No — shelf opinions already live in schema DATA (`architecture-shelf-backend.yaml`) |
| CM | ~40% | ~60% | No |
| EM | ~85% | ~15% | Teaching is member-specific (its own template/taxonomy); nothing shareable |
| MM | ~40% | ~60% | No |
| MT | ~40% | ~60% | No |
| PM | ~35% | ~65% | No |
| SL | ~35% | ~65% | No |
| SD | ~60% | ~40% | Craft guidance already single-homed in DIAGRAM-CONVENTIONS.md |
| TD | ~60% | ~40% | Teaching already in its two references |
| TF | ~30% | ~70% | No |
| VT | ~50% | ~50% | Grammar already single-homed in TEST-GRAMMAR.md (4 external consumers) |

Two facts decide the road. **First, the premise holds for only 4 of 13 members** — the family
is majority obligation-dense (the C3 note generalized from the wrong exemplars; the 2026-08
floor skills changed the family's character). **Second, the road solves a sharing problem the
family does not have:** §C found at most 3 thin shared cores. A shared family `references/`
file would have almost nothing to single-home — the members' shared text is ~40–120 chars per
candidate, while the members' real shared-content mechanism already exists and works
member-locally (TEST-GRAMMAR.md, DIAGRAM-CONVENTIONS.md, EXTERNAL-CLAIMS.md consumed cross-dir
under the sanctioned Single-source convention). Payload arithmetic: a shared references file
(~2–4k, judgment-prose is verbose) read by all 13 members at invoke would ADD ~26–52k
family-wide delivered payload to save under 400 chars of duplication — the F4 arithmetic fails
by an order of magnitude. Conversely, per-member schema stubs (§B: ~6 total) cost ~1.5k
family-wide and buy IDs + checker visibility for reference-borne obligations.

**Recommendation for the user gate: REJECT the shared-references road as the family mechanism**
— not because schemas beat it at drift control (nothing here needs drift control; §D), but
because there is no shared judgment-prose corpus for it to home. Keep the existing per-member
Single-source convention as the sanctioned mechanism for the four teachers' reference content
(it is already working: VT's TEST-GRAMMAR audit trail shows a clean 4-consumer single source).
Not a hybrid: a family-shared references file has no content to hold, and member-local
references already exist. The live question the gate should rule instead is §D's: whether the
secondary drivers alone justify converting the 9 carriers (recommended), the whole 13
(uniformity argument — one family, one ceremony), or none.

---

## F. Per-member read-cost projection (D8/C1 · D9/I5)

Method: wave-1 measured all-in structural cost ≈ **423 chars/rule** of schema (the ×3.24
structural factor over extracted obligation text of ~130 chars/rule — derived from the eight
v0.100.0 pairs: 84,605 schema chars / 200 rules; the two methods coincide). Est. schema =
rules × 423 + stubs. Est. post body = body − (rules × 130) + 450 load-first block. **Budgeted
payload** (C1 quantity) = post body + own schema. Delivered-at-invoke adds a family common file
(~1,000–1,100 for the §C thin set) **only if one is minted** — §C/ROAD recommend against, so
the payload column is the delivered figure under the recommendation. Estimates ±25% (the wave-1
census's own band; its estimates ran ~30% under the measured build — read these as floors, not
ceilings).

| Member | Body now | Desc | Refs (exempt) | Rules(+stubs) | Est. schema | Est. post body | Est. payload | vs body now | Current budget |
|---|---|---|---|---|---|---|---|---|---|
| AF | 7,407 | 610 | — | 17 | ~7,200 | ~5,650 | ~12,850 | ×1.73 | unbudgeted |
| AC | 11,036 | 486 | 45,047 | 13 | ~5,500 | ~9,800 | ~15,300 | ×1.39 | 13,412 |
| AS | 6,927 | 473 | — | 17 | ~7,200 | ~5,170 | ~12,370 | ×1.79 | unbudgeted |
| CM | 3,795 | 564 | — | 10 | ~4,230 | ~2,950 | ~7,180 | ×1.89 | 4,319 |
| EM | 13,726 | 497 | 22,931 | 12(+1) | ~5,500 | ~12,620 | ~18,120 | ×1.32 | 16,835 |
| MM | 4,647 | 499 | — | 13 | ~5,500 | ~3,410 | ~8,910 | ×1.92 | unbudgeted |
| MT | 4,785 | 655 | — | 11 | ~4,650 | ~3,810 | ~8,460 | ×1.77 | unbudgeted |
| PM | 4,276 | 600 | — | 10 | ~4,230 | ~3,430 | ~7,660 | ×1.79 | unbudgeted |
| SL | 6,363 | 505 | — | 14 | ~5,920 | ~4,990 | ~10,910 | ×1.72 | unbudgeted |
| SD | 9,320 | 649 | 4,501 | 16 | ~6,770 | ~7,690 | ~14,460 | ×1.55 | 11,047 |
| TD | 5,118 | 469 | 10,147 | 10(+1) | ~4,650 | ~4,270 | ~8,920 | ×1.74 | 5,783 |
| TF | 5,398 | 450 | — | 14 | ~5,920 | ~4,030 | ~9,950 | ×1.84 | unbudgeted |
| VT | 6,781 | 497 | 12,581 | 15(+3) | ~7,610 | ~5,280 | ~12,890 | ×1.90 | 6,487 (+294 overage HOLDS) |
| **Family** | **89,579** | | 95,207 | **~172(+6)** | **~74,900** | **~73,100** | **~148,000** | **×1.65** | |

Read plainly: **the conversion multiplies the family's delivered-at-invoke payload ×1.65
(~89.6k → ~148k), lower than the review family's measured ×3.24 only because teaching prose
stays put — the absolute added chars (~58k) exceed the review wave's (+46k after correction to
its ×3.24 base of 37k).** If a common file were minted despite §C, add ~1.1k × binding members.
No patterns member obligates a reference read at invoke (no RF-lens analogue), so payload =
real per-invoke read; VT's authoring seat reads TEST-GRAMMAR.md in practice (+6,270 when it
does). Budget mechanics per C1: the 6 budgeted members all land above their current caps —
exactly the re-seed path's case; the 7 unbudgeted members would gain first rows (the RSUF
precedent); VT's standing +294 overage is absorbed/superseded by its re-seed. Descriptions
untouched, all ≤1,536 (max MT 655; six members sit above the ~500 norm, each previously
disclosed/ruled — CM 564 and SD 649 carry protected RETURNED clauses).

---

## I. Labels (fit against `plugins/mochiko/schemas/skill-labels.yaml`)

Existing nine: **boundary** (heavy — every not-for route and sibling split), **user-gate**
(heavy — AF/AS/MM/SL/TD reservations), **binding** (heavy — schema data files, templates,
spine, tasks.yaml two-arm, EXTERNAL-CLAIMS), **evidence** (moderate — read-before-claim,
`verified:`/memory-asserted, MT provenance rules), **reporting** (moderate — disclosure
grammars, cycle/close reports), **floor-pointer** (moderate — pointers at adopt-first,
brownfield-integration, sound-loop/transport cross-floor lines), **independence** (light —
VT's author split, SL's legs), **fence** (light — MT session-tier holds, VT's
never-the-builder), **verdict** (unused — no patterns member issues a clearing grammar; the
registry line stays honest, no edit needed).

New, corpus-demanded (registry-edit-first ceremony):

- **trigger** — when a kind-keyed discipline fires: lane/two-part/commodity/skeleton firing
  conditions and their non-waivability once fired. Carriers: SL, TF, AF, VT, AC. (Distinct
  from `when:` — the label marks rules ABOUT firing semantics.)
- **ladder** — a ranked rung/leg/test structure's binding rules: stop-at-first, descend-with-
  why, walk-order, breadth/every-row obligations. Carriers: CM, PM, MM, AS, TF (legs).

Considered, rejected: `disclosure` (covered by `reporting`) · `stance` (AS-only, one carrier) ·
`altitude` (SD-only).

---

## J. Anomalies (numbered, each with a recommended disposition)

- **J-P1 — local `__pycache__` in the tree.**
  `plugins/mochiko/skills/patterns-api-contracts/scripts/__pycache__/validate-openapi.cpython-313.pyc`
  exists on disk; verified **gitignored and untracked** (`.gitignore:2`), so nothing ships.
  *Recommendation:* delete the local directory at convenience; no ceremony owed.
- **J-P2 — two members have no strips file** (AS, TF). Legal — no post-birth removal has
  occurred — but their protection basis is DECISIONS-traceability alone, so their conversion
  strips would be each file's FIRST entry. *Recommendation:* the conversion seats create the
  strip files with the supersession-transfer entries; no pre-work owed.
- **J-P3 — protected content living in `description:` values.** CM's [v0.64.0] RETURNED
  slimming trigger and SD's [v0.64.0] RETURNED baseline clause are protected content whose home
  is the frontmatter description — which D3/D8-criterion-7 pins byte-identical. No conflict
  (protection satisfied by construction), but the audit brief should name them so a grader does
  not read the pins as unprotected. *Recommendation:* one line each in the two members' audit
  briefs.
- **J-P4 — EM's live whole-body KEPT ruling.** The [v0.27.0] "KEPT: the remaining body"
  survivor ruling is the family's RF-analogue: a body-residency protection a conversion
  necessarily disturbs. *Recommendation:* the patterns rollout ruling carries an explicit
  D8/C4-style clause re-homing the survivor protection onto the pair (body + schema jointly),
  recorded once — the review-feasibility precedent, verbatim ceremony.
- **J-P5 — VT's protected two-arm citation + standing overage.** The v0.76.0 two-arm
  `tasks` pointer (GI-020) must survive verbatim in its schema rule (the RPA two-arm
  precedent), and the ledgered +294 HOLDS overage is superseded by the conversion re-seed.
  *Recommendation:* both named in VT's audit brief; the re-seed row cites the overage history.
- **J-P6 — `scripts/` in two members** (AC `validate-openapi.py`, EM `validate-model.py`).
  Scripts stay budget-exempt; the run-the-script obligations are body rules whose `pointer:`
  names a script, a pointer class the review wave never exercised (all its pointers were .md/
  skill/template targets). *Recommendation:* checker's pointer-resolution check confirms it
  already accepts non-markdown targets; name it in the P4-analog seat's brief.
- **J-P7 — section-set misfit** (§B finding). The review six-set fails this family on
  `verdict` (empty/forced for 12 of 13). *Recommendation:* the family rollout ruling mints the
  six-set proposed in §B (`trigger · scope · discipline · inputs · disclosure · reserved`),
  uniform across the family, explicit empty markers where thin — a per-family mint the D4/D9
  door-open idiom already sanctions; NOT a new grammar (kinds/axes unchanged).
- **J-P8 — cross-directory pointers** (the wave-1 J-7 class): AF →
  `../review-brainstorm/references/EXTERNAL-CLAIMS.md` and
  `../authoring-constitution/references/catalog/backend-service.md` (BE-DEP); TD-ref →
  EXTERNAL-CLAIMS the same way; AS → `plugins/mochiko/schemas/architecture-shelf-backend.yaml`
  (a `schemas/`-home data file — a new pointer target class); VT →
  `plugins/mochiko/schemas/tasks.yaml` (two-arm). *Recommendation:* legal under D3/C2;
  checker's climb-out resolution (built at v0.100.0 for J-7) covers the `../` cases; the
  `schemas/`-home targets resolve via `../../schemas/` — priced here so the build does not
  rediscover it; both ride the first-live-run watch's path probe (M1 class).
- **J-P9 — one advisory-heavy member with zero floors** (TD). Its conversion yields a schema
  with no `class: floor` rule, making its D6 floor-count pin read "the 0 rules of
  `class: floor`". Legal but novel — the wave-1 pins ran 3–16. *Recommendation:* the pin
  grammar already tolerates it; name it in TD's audit brief so the read-back obligation ("state
  the floor count back") is not mistaken for a skipped read.
- **J-P10 — near-dup pairs below bar** (§C screened list: SL/TF ×3 pairs, CM/PM ×2 pairs,
  EM/AC ×2 pairs). *Recommendation:* pre-seed the allowlist with these ~10 edges at build so
  the detector's first family run is quiet (the wave-1 three-pass allowlist lesson — 96 edges
  landed post-hoc; this family can land them up front).
- **J-P11 — classification uniform.** All 13 are model-invoked (MUST-invoke descriptions);
  no user-invoked member, no router entanglement. No disposition needed.

---

*End of patterns-family census. Per D9-I3 this inventory returns to the user at the family
gate with the §D borderline-abort evidence, the §ROAD rejection recommendation, the §F ×1.65
payload projection, and the 9-carriers-vs-13 scope question, before any conversion begins.*

---

## K. Build-corrections appendix (wave 2B landing, 2026-09-01, v0.102.0)

Corrections and final measures recorded at landing per the wave-1 §K idiom. Scope as ruled:
the 9 discipline carriers converted; the four teachers (AC · EM · TD · SD) stay prose.

- **Class-mix cells vs row-grain enumerations** — the row grain won in every conflict
  (the standing table-beats-tally idiom, applied per lead ruling at plan approval):
  **AF 7 floors** (summary row said 6/10/1; enumeration marks 7 floors, no advisory) ·
  **CM 3 floors** (summary cell said 4; three rows carry the floor marker) ·
  **TF 11 floors** (the §B table's "10", §D's "10 floors" cell, AND the build brief all
  under-counted; the enumeration's "(floor, bound)" marker on the version-floor row won by
  lead overrule) · **VT 5 floors** (the detail line's `must` marker on the two-arm row lost
  to the §B table + §D's explicit "5 floors incl. two protected two-arm/authorship rules";
  the promotion gives the GI-020-protected rule floor survival).
- **Structural resolutions, all disclosed in strip maps:** AS census row 4's store-grammar
  half folded into `ax-grammar-routing` · MM merge mechanics as one 4-limb rule
  (census-sanctioned) · TF's sizing not-for route folded into `governs-use-never-choice`
  (no 15th rule; the J-P10 edge's TF-side ID) · AF's two build-time rules and MM's
  cap-trip rule are DECLARE-form `when:` (the command-ontology idiom, confirmed
  transferring) · SL's two tables compressed into rule text with all seven governing-surface
  rows and all four wiring rows preserved · AFM-class conditions divergences: none (SL and
  VT are the family's only `conditions:` carriers besides AF/MM).
- **Audit-mandated repairs (W1/W2/W3 rounds):** AS regained the display-for-override
  obligation the census row grain had dropped (census-inherited loss, restored by ruling) ·
  MT regained the sound-loop cross-floor pointer (census §I carrier) · SL regained two
  governing-surface regime descriptors · VT shed one undisclosed added sentence · one
  VT strip heading overwrite (the wave-1 GI-005 class) caught mid-wave by a sibling seat
  and restored byte-exact.
- **Final measures** — family delivered-at-invoke **95,858** vs 50,379 pre-conversion =
  **×1.90**, +5.1% over the §F 9-carrier estimate (×1.81, ~91.2k), inside the census ±25%
  band. Per member: AF 12,910 · AS 12,361 · CM 8,024 · MM 9,362 · MT 8,793 · PM 8,594 ·
  SL 10,933 · TF 10,556 · VT 14,325. No common file (§ROAD held); 124 rules · 46 floors ·
  3 reference stubs; two re-seeds (CM, VT — the +294 HOLDS overage absorbed and
  superseded) + seven first-seeds via the ledger's third seeding path.
- **Cross-family graduation evidence (D5, not actioned):** the detector surfaced a 4-member
  cross-family read-before-claim convergence (authoring-feature-map · patterns-map-minimalism
  · review-brainstorm · review-specifications) — the strongest signal yet for the
  cross-family `skill-common.yaml` graduation candidate; suppressed on D5 grounds, six
  allowlist edges, evidence recorded here and at the wave's record trail.
- **Standing non-blocking observations:** MT's session-tier clause drops its "(the F2
  guide-line)" internal citation (verbatim-in-substance holds; decision, not oversight) ·
  the checker's `pointers` stat counts file-path pointers only, so skill-ref pointers
  (`mochiko:<skill>`) read as 0 — consistent corpus-wide.
