# Conversion inventory — command-schema ontology wave (I8)

**Status:** authored 2026-08-27 · the audit referent for the D1–D11 conversion wave
**Ruling:** [`record.md`](record.md) D1–D11 as amended (build-surface item 3: "preceded by the
I8 exhaustive clause inventory … the inventory is the audit referent")
**Corpus:** the six command schemas at working-tree state — `plugins/mochiko/schemas/`
`{implement,feature,specify,architecture,setup,brainstorm}.yaml`, 320 rules
(implement 104 · specify 51 · feature 49 · architecture 47 · setup 40 · brainstorm 29),
plus `common.yaml` (the uncommitted six-block `extends:` prototype) and the six `.md` halves.

Every count in this file was produced by parsing the schemas, not by reading them. Where a
figure disagrees with the record, both are stated and the disagreement is explained in
[section J](#j--anomalies).

**Scope discipline.** This file applies the record's policy. Where the record is silent, the
question is flagged in section J rather than resolved here. No schema, `.md`, script, or shipped
file was edited to produce it.

---

## Contents

- [A — Kind classification](#a--kind-classification)
- [B — `when:` extraction inventory](#b--when-extraction-inventory)
- [C — Per-schema `conditions:` blocks](#c--per-schema-conditions-blocks)
- [D — Per-schema `moments:` blocks](#d--per-schema-moments-blocks)
- [E — `enforces:` mapping](#e--enforces-mapping)
- [F — `extends:` bindings](#f--extends-bindings)
- [G — Canonical schema header comment](#g--canonical-schema-header-comment)
- [H — Not-done re-key lines](#h--not-done-re-key-lines)
- [I — F8 move](#i--f8-move)
- [J — Anomalies](#j--anomalies)

---

## A — Kind classification

### A.0 — The discriminators applied

D1's nine kinds, with the test each rule was put to. `constraint` is the omitted default: an
absent `kind:` reads `constraint`, so no constraint rule carries the field — but every one is
recorded below, because the census is only trustworthy if it is total.

| Kind | Test applied |
|---|---|
| `constraint` | A standing obligation or prohibition. The default; anything not positively matching another kind lands here. |
| `duty` | An action **the lead** always performs. A seat's obligation is a constraint, not a duty — the duty kind names the always-happens floor of the run's or desk's own lead. |
| `gate` | A **blocking checkpoint**: the run halts and something is presented before it proceeds. |
| `reservation` | **Standing decision ownership** with no checkpoint of its own — who owns a call, whenever it arises. |
| `binding` | What lives where, shaped by what — a path, template, schema, workspace, or artifact home. |
| `bound` | A countable allowance whose exhaustion has a **stated consequence**. |
| `routing` | An event or demand mapped to a **destination**, with defaults. |
| `fail` | An end-state predicate — the `<cmd>.fail.*` segment. |
| `latitude` | A **grant of judgment** — neither obligation nor prohibition. What the holder may decide for themselves. |

**Gate-vs-reservation split — validated corpus-wide (D1 as amended asked for this).** The split
holds, and the corpus sharpens it in three ways worth carrying into the checker and the audit:

1. **A gate need not be the user's.** `impl.sufficiency-binding-verdict` is a scheduled blocking
   checkpoint held by a *grader* ("The sufficiency check runs at entry … The verdict is
   binding"). Ten gates corpus-wide; nine are the user's, one is not. An audit criterion keyed to
   "gate = user gate" would misgrade it.
2. **Gates come in two schedule forms, and both block.** *Moment-scheduled* — the design
   checkpoint, the card confirm, final acceptance, synthesis ratification, spec selection and
   acceptance. *Event-scheduled* — `impl.deviation-gate` ("A cycle that adds or removes a box …
   stops and is presented"). The discriminator that survives is **blocking**, not *calendared*.
3. **The `gate-` ID prefix is not the selector.** `brainstorm.user-record-acceptance` carries no
   `gate-` prefix and is a gate; `impl.user-runopen-rulings` names a moment (`run-open`) and is a
   reservation, because the blocking is done by the run-open confirmation, not by the rule. The
   `kind:` field is therefore load-bearing here — it is not derivable from the ID.

A reservation never blocks on its own account. `impl.beyond-spec-disposition`, `feat.user-reserved`,
`arch.truth-user-ruling`, `setup.user-mode-ruling` all say *who decides*, never *when the run stops*.

**Latitude membership — finalized.** Eight rules, matching F3-as-corrected's "~8". Every candidate
the record named is ruled in or out below:

| Candidate (from F3/D1) | Verdict | Reason |
|---|---|---|
| `impl.staffing-latitude` | **IN** | "everything is your per-run judgment: how you staff, sequence, and run the cycles" — a pure grant. |
| `feat.staffing-latitude` | **IN** | "everything beyond it is your per-visit judgment … yours to shape". |
| `arch.staffing-latitude` | **IN** | Same form, per-visit. |
| `setup.staffing-latitude` | **IN** | "Teammates or subagents per seat is your call." |
| `brainstorm.staffing-latitude` | **IN** | Same text as setup's. |
| `spec.lead-latitude` | **IN** | "teammates or subagents per seat is your call" is the operative new information; "plan the run and orchestrate it" states the standing role, not a discrete always-performed action. |
| `impl.design-seats-staffing` | **IN** | "Design-seat staffing is your call: typically …" — `class: advisory`, a recommendation inside a grant. |
| `brainstorm.record-review-independence` | **OUT** → `constraint` | "The record … is yours, **so** its review seat is always someone else" — the ownership clause is a premise; the operative content is an independence obligation. Nothing is left to the lead's judgment. |
| `spec.epic-proposal-optional` (**not** on the record's list) | **IN** | "the `${pm_seat}` seat **may** propose an epic … a proposal only, never a mint" — grants a discretionary move and bounds it. Beyond the record's candidate set; see [J-4](#j-4--one-latitude-member-beyond-the-records-candidate-list). |

Latitude is a grant of judgment **to whoever holds it** — seven to the lead, one
(`spec.epic-proposal-optional`) to a seat. The record's framing ("per-run/per-visit judgment")
does not settle whether a seat-held grant qualifies; this inventory rules it in and flags it.

---

### A.1 — implement (104 rules)

**`impl.sec.roles` (16)**

| Rule | Kind | Reason (only where non-obvious) |
|---|---|---|
| `impl.staffing-latitude` | `latitude` | |
| `impl.dm-entry-gate` | `duty` | The `dm-*` set — an action the lead performs every run. The blocking it names belongs to the run-open confirmation; the user-held calls there are `impl.user-runopen-rulings`. |
| `impl.design-phase-fires-on-gap` | `duty` | Lead-performed, always, on the trigger — not a demand routed to a destination. |
| `impl.dm-surface-rounds` | `duty` | |
| `impl.dm-landing-whole` | `duty` | |
| `impl.dm-close-verdict` | `duty` | |
| `impl.seat-sufficiency-independence` | `constraint` | |
| `impl.design-seats-staffing` | `latitude` | |
| `impl.design-gaps-only` | `constraint` | |
| `impl.builder-never-designs` | `constraint` | |
| `impl.seat-card-author-independence` | `constraint` | |
| `impl.builder-decompose-disclose` | `constraint` | A seat's obligation, not the lead's — so not a duty. |
| `impl.seat-verification-independence` | `constraint` | |
| `impl.verification-design-time-grades` | `constraint` | |
| `impl.landing-verifier-folds` | `constraint` | |
| `impl.seat-gap-finder-blind` | `constraint` | |

**`impl.sec.reserved` (8)**

| Rule | Kind | Reason |
|---|---|---|
| `impl.gate-design-checkpoint` | `gate` | |
| `impl.gate-card-confirm` | `gate` | |
| `impl.gate-final-acceptance` | `gate` | |
| `impl.user-runopen-rulings` | `reservation` | Enumerates ownership at a moment; the blocking is the run-open confirmation's, not this rule's. |
| `impl.infeasible-card-escalation` | `routing` | Escalation route: infeasible card → the user as a business-level scope decision. |
| `impl.adopt-first-user-call` | `reservation` | "Two calls are never the builder's" — ownership first, halt second. |
| `impl.ambiguity-escalation` | `routing` | |
| `impl.scope-escalation-fail` | `reservation` | "Scope escalation is the user's" — ownership with a FAIL default. |

**`impl.sec.tools` (43)**

| Rule | Kind | Reason |
|---|---|---|
| `impl.tools-referenced-never-restated` | `constraint` | |
| `impl.sufficiency-binding-verdict` | `gate` | Scheduled ("runs at entry") and blocking ("The verdict is binding") — a grader-held gate, not a user gate. |
| `impl.sufficiency-disputed-clause` | `routing` | Disputed clause → default gap → the user. |
| `impl.sufficiency-report` | `binding` | |
| `impl.absent-surfaces` | `routing` | A per-surface branch table with a standing default (surface, never auto-resolve, never fail). |
| `impl.briefs-name-rules-files` | `constraint` | |
| `impl.design-outputs-home` | `binding` | |
| `impl.design-review-pair` | `constraint` | |
| `impl.design-absent-baseline-seed` | `constraint` | |
| `impl.design-map-assertion` | `constraint` | |
| `impl.midrun-refire` | `routing` | Discovery → halt that cycle → design phase re-fires scoped to it. |
| `impl.epic-design-always-fires` | `constraint` | |
| `impl.epic-seam-owners` | `constraint` | |
| `impl.epic-shared-baseline-single-pen` | `constraint` | |
| `impl.cards-template` | `binding` | |
| `impl.card-contents` | `binding` | What a card carries — content shape, "shaped by what". |
| `impl.zero-gap-map-assertion` | `constraint` | |
| `impl.card-review-before-confirm` | `constraint` | |
| `impl.epic-card-sequence` | `constraint` | |
| `impl.craft-build-bindings` | `binding` | |
| `impl.craft-verify-bindings` | `binding` | |
| `impl.design-inputs` | `binding` | |
| `impl.progress-surface` | `binding` | |
| `impl.reports-envelope` | `binding` | |
| `impl.gates-full-suite` | `constraint` | |
| `impl.regression-sweep` | `constraint` | |
| `impl.cold-verification` | `constraint` | |
| `impl.gap-finding-scope` | `constraint` | An applicability statement over all four scope values, not a demand routed anywhere. |
| `impl.gap-finding-blind-dispatch` | `constraint` | |
| `impl.mutation-lens` | `constraint` | |
| `impl.finding-kinds` | `routing` | Finding class → consequence, with defaults. |
| `impl.finding-kind-disputed` | `routing` | Disputed kind → default advisory → the user. |
| `impl.delivered-territory-routing` | `routing` | |
| `impl.beyond-spec-disposition` | `reservation` | The disposition is the user's; the three options are their choice, not a route the run takes. |
| `impl.store-landing` | `constraint` | |
| `impl.km-landing` | `constraint` | |
| `impl.baseline-delta-grammar` | `binding` | |
| `impl.landing-selection` | `constraint` | |
| `impl.landing-epic` | `constraint` | |
| `impl.landing-delta` | `constraint` | |
| `impl.graded-fold` | `constraint` | |
| `impl.gates-fold` | `constraint` | |
| `impl.register` | `binding` | What shapes user-facing prose. Bound to `common.register`; `kind:` is local (C3). |

**`impl.sec.ways-of-working` (8)**

| Rule | Kind | Reason |
|---|---|---|
| `impl.author-grader-default-fail` | `constraint` | |
| `impl.plan-approval-producers` | `constraint` | |
| `impl.escalation-batching` | `routing` | Reserved question → the cycle-checkpoint batch, with a build-blocking exception route. |
| `impl.finding-severity-routing` | `routing` | |
| `impl.model-tiering` | `routing` | Read class → dispatch tier, with defaults. The cleanest routing instance in the corpus. |
| `impl.delta-reverification` | `constraint` | |
| `impl.no-git-mutations` | `constraint` | |
| `impl.acceptance-plain-text` | `constraint` | |

**`impl.sec.boundaries` (14)**

| Rule | Kind | Reason |
|---|---|---|
| `impl.baselines-never-in-place` | `constraint` | |
| `impl.deviation-gate` | `gate` | Event-scheduled blocking checkpoint — "stops and is presented". |
| `impl.constitution-supremacy` | `constraint` | |
| `impl.constraint-challenge` | `constraint` | "A ratified constraint is never silently overridden" — prohibition first; the filing route is its remedy. |
| `impl.attempt-per-grade` | `bound` | |
| `impl.attempt-exemption-user-only` | `reservation` | |
| `impl.no-progress-stop` | `bound` | Allowance (two consecutive unchanged rounds) with a stated consequence (halt, present). |
| `impl.epic-member-halt` | `bound` | Declares the **exhaustion route** limb for epic scope — the third limb of the bound triple. See [J-5](#j-5--a-fourth-bound-the-record-named-three). |
| `impl.gap-rework-bound` | `bound` | |
| `impl.gates-never-triaged` | `constraint` | |
| `impl.minimalism-advisory` | `constraint` | |
| `impl.lane-never-widens` | `constraint` | |
| `impl.sound-loop-floor` | `constraint` | |
| `impl.transport-floor` | `constraint` | |

**`impl.sec.fail-conditions` (15)** — all `fail`:
`impl.fail.sufficiency-unrecorded` · `design-skipped` · `card-independence` · `card-unchecked` ·
`quality-gate` · `no-evidence` · `regression` · `baseline-in-place` · `deviation-unresolved` ·
`store-landing-incomplete` · `ungraded-fold` · `gap-finding-missing` · `skip-unstated` ·
`spec-gap-unresolved` · `no-acceptance`.

**implement totals:** `constraint` 46 · `binding` 11 · `routing` 11 · `fail` 15 · `duty` 5 ·
`gate` 5 · `reservation` 5 · `bound` 4 · `latitude` 2 = **104** ✓

---

### A.2 — feature (49 rules)

**`feat.sec.roles` (12)**

| Rule | Kind | Reason |
|---|---|---|
| `feat.staffing-latitude` | `latitude` | |
| `feat.dm-health-first` | `duty` | |
| `feat.dm-converge-goal` | `duty` | |
| `feat.dm-map-integrity` | `duty` | |
| `feat.dm-route-honestly` | `duty` | |
| `feat.dm-complete-card` | `duty` | |
| `feat.dm-epic-stewardship` | `duty` | |
| `feat.dm-km-landing` | `duty` | |
| `feat.dm-close-verdict` | `duty` | |
| `feat.pm-seat` | `constraint` | A seat's responsibility set, not the lead's action. |
| `feat.architect-dormancy` | `constraint` | |
| `feat.dispatched-runs-own-delivery` | `constraint` | |

**`feat.sec.reserved` (1)** — `feat.user-reserved` → `reservation` (the desk aggregate row; D12
grain review has its name now).

**`feat.sec.tools` (15)**

| Rule | Kind | Reason |
|---|---|---|
| `feat.tools-referenced-never-restated` | `constraint` | |
| `feat.map-files` | `binding` | |
| `feat.map-minimalism-binding` | `binding` | |
| `feat.feature-map-binding` | `binding` | |
| `feat.epic-binding` | `binding` | |
| `feat.epic-dispatch` | `routing` | |
| `feat.capability-write-test` | `routing` | Self-described: "the routing instrument". |
| `feat.stable-ground-triage` | `routing` | The literal three-branch decision table (F3's exemplar). |
| `feat.delta-cards` | `binding` | |
| `feat.product-surface` | `binding` | |
| `feat.architecture-intake` | `routing` | |
| `feat.dispatch-scope-split` | `routing` | |
| `feat.dispatch-specify` | `routing` | |
| `feat.km-relation` | `binding` | Where the defect queue lives, plus its stated degrade path. |
| `feat.register` | `binding` | |

**`feat.sec.ways-of-working` (8)**

| Rule | Kind | Reason |
|---|---|---|
| `feat.proactive-report` | `duty` | "every visit" — a lead action; kind crosscuts sections (D1), so a duty outside `roles` is expected. |
| `feat.reference-never-restate` | `constraint` | |
| `feat.author-grader` | `constraint` | |
| `feat.advisory-front-door` | `routing` | Door topology — which demands enter where, with a default. |
| `feat.model-tiering` | `routing` | |
| `feat.single-flight-lane` | `constraint` | An allowance of one with **no stated exhaustion consequence** — fails the bound test. See [J-5](#j-5--a-fourth-bound-the-record-named-three). |
| `feat.no-git-mutations` | `constraint` | |
| `feat.rulings-plain-text` | `constraint` | |

**`feat.sec.boundaries` (12)**

| Rule | Kind | Reason |
|---|---|---|
| `feat.capability-writes-sacred` | `reservation` | Who may write a capability — ownership, no checkpoint. |
| `feat.grooming-door-ceiling` | `routing` | Door remit plus the overflow route to specify. |
| `feat.out-of-remit-hosting` | `constraint` | |
| `feat.growth-door` | `routing` | |
| `feat.growth-routes-to-specify` | `routing` | |
| `feat.lane-never-widens` | `constraint` | |
| `feat.no-delivery-harness` | `constraint` | |
| `feat.no-self-graded-writes` | `constraint` | |
| `feat.no-silent-map-mutations` | `constraint` | |
| `feat.sound-loop-floor` | `constraint` | |
| `feat.transport-floor` | `constraint` | |
| `feat.stub-parking` | `constraint` | |

**`feat.sec.fail-conditions` (1)** — `feat.fail.no-verdict` → `fail`.

**feature totals:** `constraint` 17 · `routing` 11 · `duty` 9 · `binding` 8 · `reservation` 2 ·
`fail` 1 · `latitude` 1 · `gate` 0 · `bound` 0 = **49** ✓

---

### A.3 — specify (51 rules)

**`spec.sec.roles` (9)**

| Rule | Kind | Reason |
|---|---|---|
| `spec.lead-latitude` | `latitude` | |
| `spec.capability-frame-at-intent` | `constraint` | A producing seat's obligation at a moment — not the lead's action. |
| `spec.confirm-frame-post-stories` | `constraint` | |
| `spec.cut-work-rows` | `constraint` | |
| `spec.filter-rejections-recorded` | `constraint` | |
| `spec.epic-proposal-optional` | `latitude` | A permission ("may propose") bounded by a prohibition ("never a mint"); the grant is the new information. |
| `spec.pm-recommends-never-selects` | `constraint` | The seat-side prohibition. Its user-side twin `spec.gate-selection` carries the ownership. |
| `spec.stress-test-one-pass` | `constraint` | |
| `spec.stress-test-prototype-walk` | `constraint` | |

**`spec.sec.reserved` (5)**

| Rule | Kind | Reason |
|---|---|---|
| `spec.reserved-to-user` | `reservation` | The aggregate row. |
| `spec.selection-card` | `binding` | Its non-duplicative content is the card's required contents; the ownership is `spec.gate-selection`'s. |
| `spec.gate-selection` | `gate` | |
| `spec.gate-acceptance` | `gate` | |
| `spec.filter-disagreement-escalates` | `routing` | |

**`spec.sec.tools` (17)**

| Rule | Kind | Reason |
|---|---|---|
| `spec.deliverable` | `binding` | |
| `spec.intent-stage-first` | `constraint` | Stage ordering, not a blocking checkpoint with a decision. |
| `spec.map-obligated-read` | `constraint` | |
| `spec.missing-map-surfaced` | `routing` | Absent surface → surfaced, with the `/mochiko:setup` offer. |
| `spec.unrefined-stubs` | `constraint` | |
| `spec.intent-synthesis-governs` | `binding` | |
| `spec.frame-greenfield-inputs` | `constraint` | Names no artifact home — guidance on inputs to a judgment, so not a binding. |
| `spec.whole-feature-prototype` | `constraint` | |
| `spec.feature-map-craft` | `binding` | |
| `spec.map-minimalism-binding` | `binding` | |
| `spec.prototype-craft` | `binding` | |
| `spec.governance-briefs` | `constraint` | |
| `spec.governance-region-absent` | `routing` | |
| `spec.km-landing` | `constraint` | |
| `spec.migration-frozen-specs` | `binding` | Which form new runs take, and where frozen history stands. |
| `spec.register` | `binding` | |
| `spec.next-step` | `routing` | |

**`spec.sec.ways-of-working` (8)**

| Rule | Kind | Reason |
|---|---|---|
| `spec.model-tiering` | `routing` | |
| `spec.intent-probe-discipline` | `constraint` | |
| `spec.frame-hypothesis-not-anchor` | `constraint` | |
| `spec.lockstep-prototyping` | `constraint` | |
| `spec.plan-approval` | `constraint` | |
| `spec.author-grader-default-fail` | `constraint` | |
| `spec.no-git-mutations` | `constraint` | |
| `spec.acceptance-plain-text` | `constraint` | |

**`spec.sec.boundaries` (3)** — `spec.transport-floor` → `constraint` · `spec.staged-derivation`
→ `constraint` · `spec.epic-mint-desk-only` → `routing` (an epic proposal → the desk, "the only
door that mints").

**`spec.sec.fail-conditions` (9)** — all `fail`: `blocking-gap` · `intent-unconfirmed` ·
`map-unread` · `story-unhomed` · `screens-flows` · `selection-unruled` · `premature-map-write` ·
`self-graded` · `no-acceptance`.

**specify totals:** `constraint` 23 · `fail` 9 · `binding` 8 · `routing` 6 · `gate` 2 ·
`latitude` 2 · `reservation` 1 · `duty` 0 · `bound` 0 = **51** ✓

---

### A.4 — architecture (47 rules)

**`arch.sec.roles` (13)** — `arch.staffing-latitude` → `latitude`; the nine `dm-*` rules
(`dm-health-first` · `dm-converge-goal` · `dm-author-baseline` · `dm-shelf-walk` ·
`dm-drift-dispatch` · `dm-route-triggers` · `dm-store-integrity-close` · `dm-km-landing` ·
`dm-close-verdict`) → `duty`, all `class: floor` — F3's clean case, confirmed exactly;
`arch.seat-architect-producer` · `arch.seat-tech-lead-grader` · `arch.seat-drift-probe-empirical`
→ `constraint` (seat obligations, not the lead's actions).

**`arch.sec.reserved` (1)** — `arch.user-reserved-rulings` → `reservation` (the desk aggregate row).

**`arch.sec.tools` (13)**

| Rule | Kind | Reason |
|---|---|---|
| `arch.tools-referenced-never-restated` | `constraint` | |
| `arch.tools-store` | `binding` | |
| `arch.tools-store-skill` | `binding` | |
| `arch.tools-shelves-skill` | `binding` | |
| `arch.shelf-scope-source` | `binding` | Where shelf scope is read from and how it is overridden. |
| `arch.tools-system-design` | `binding` | |
| `arch.tools-store-schema` | `binding` | |
| `arch.tools-brownfield-reconstruction` | `constraint` | An obligation on the first visit; the archive path is incidental to it. |
| `arch.tools-drift-probe-scope` | `constraint` | |
| `arch.dispatch-feature-growth-door` | `routing` | |
| `arch.dispatch-setup-waiver` | `routing` | |
| `arch.dispatch-implement-delivery` | `routing` | |
| `arch.register` | `binding` | |

**`arch.sec.ways-of-working` (8)** — `arch.proactive-report-first` → `duty` ("every visit");
`arch.model-tiering` → `routing`; `arch.reference-never-restate` · `arch.author-grader-separation`
· `arch.recommend-then-arbitrate` · `arch.single-writer-store` · `arch.no-git-mutations` ·
`arch.rulings-plain-text` → `constraint`. (`recommend-then-arbitrate` is the *deal protocol* for a
row, not a checkpoint the visit halts at — hence constraint, not gate.)

**`arch.sec.boundaries` (11)** — `arch.truth-user-ruling` → `reservation` (standing ownership,
no checkpoint); the other ten → `constraint`: `breadth-invariant` · `floor-precedence` (its core
is *which moves are legal on a row*; the waiver route is its remedy) · `na-handled-elsewhere-pointer`
· `derived-index-never-hand-maintained` · `drift-empirical` · `no-depth-dial-coupling` ·
`no-delivery-harness` · `no-silent-store-mutations` · `sound-loop-floor` · `transport-floor`.

**`arch.sec.fail-conditions` (1)** — `arch.fail.no-verdict` → `fail`.

**architecture totals:** `constraint` 22 · `duty` 10 · `binding` 7 · `routing` 4 · `reservation` 2 ·
`fail` 1 · `latitude` 1 · `gate` 0 · `bound` 0 = **47** ✓

---

### A.5 — setup (40 rules)

**`setup.sec.roles` (4)** — `setup.staffing-latitude` → `latitude`;
`setup.interrogation-inline` → `duty` ("Run the interrogation yourself, inline");
`setup.stress-test-cold-seat` · `setup.blind-map-dispatch` → `constraint`.

**`setup.sec.reserved` (7)**

| Rule | Kind | Reason |
|---|---|---|
| `setup.coverage-survivor-routing` | `routing` | Survivor → candidate topic → one of three user-ruled paths, with an overlap default. |
| `setup.user-mode-ruling` | `reservation` | Also the **resolution point** for the `mode` dimension (F4's user-ruled exemplar). |
| `setup.user-card-rulings` | `reservation` | |
| `setup.gate-synthesis-ratification` | `gate` | |
| `setup.user-conflict-rulings` | `reservation` | |
| `setup.user-map-confirmation` | `reservation` | |
| `setup.gate-final-acceptance` | `gate` | |

**`setup.sec.tools` (12)** — `binding`: `surface-set` · `synthesis-artifact` ·
`interrogation-inputs` · `km-module-scaffold` · `register`. `routing`: `next-step`.
`constraint`: `constitution-superseded` · `feature-map-brownfield` · `feature-map-greenfield` ·
`baselines-bootstrap` · `store-scaffold-unconditional` · `architecture-scope-handoff` — each is
an obligation on a path; the artifact homes they name are incidental to the obligation.

**`setup.sec.ways-of-working` (5)** — `setup.model-tiering` → `routing`; the other four
(`plan-approval-producers` · `author-grader-default-fail` · `no-git-mutations` ·
`acceptance-plain-text`) → `constraint`.

**`setup.sec.boundaries` (6)** — all `constraint`: `transport-floor` · `durables-never-deleted` ·
`governance-region-ownership` · `carve-outs-preserved` · `map-never-overwrite` ·
`store-ruled-content-never-here`.

**`setup.sec.fail-conditions` (6)** — all `fail`: `pre-ratification-authoring` ·
`unclosed-trace` · `author-graded` · `floor-category-uncovered` · `no-acceptance` ·
`no-feature-map`.

**setup totals:** `constraint` 18 · `fail` 6 · `binding` 5 · `reservation` 4 · `routing` 3 ·
`gate` 2 · `duty` 1 · `latitude` 1 · `bound` 0 = **40** ✓

---

### A.6 — brainstorm (29 rules)

**`brainstorm.sec.roles` (5)** — `lead-inline-questioning` → `duty`;
`staffing-latitude` → `latitude`; `record-review-independence` → `constraint`
(candidate ruled OUT, see A.0); `blind-map-dispatch` · `pair-maps-independent` → `constraint`.

**`brainstorm.sec.reserved` (5)**

| Rule | Kind | Reason |
|---|---|---|
| `brainstorm.coverage-survivor-routing` | `routing` | |
| `brainstorm.user-record-acceptance` | `gate` | A blocking checkpoint at the run's end — the same species as `spec.gate-acceptance` and `impl.gate-final-acceptance`, without the `gate-` prefix. |
| `brainstorm.user-survivor-challenge` | `reservation` | |
| `brainstorm.user-review-waiver` | `reservation` | |
| `brainstorm.user-pen-boundary` | `reservation` | |

**`brainstorm.sec.tools` (7)** — `binding`: `deliverable-record` · `synthesis-on-request` ·
`register`. `routing`: `next-step-offer` (pipeline entry is an offer, "never a default").
`constraint`: `record-as-you-go` · `index-bookkeeping` · `km-close-ritual`.

**`brainstorm.sec.ways-of-working` (7)** — `routing`: `model-tiering` · `non-coverage-survivors`.
`bound`: `reopen-born-verify` (one round, with a stated consequence: "no second reopen off it").
`constraint`: `plan-approval-producers` · `author-grader-default-fail` · `no-git-mutations` ·
`acceptance-plain-text`.

**`brainstorm.sec.boundaries` (1)** — `brainstorm.transport-floor` → `constraint`.

**`brainstorm.sec.fail-conditions` (4)** — all `fail`: `record-unaccepted` ·
`unreviewed-no-waiver` · `survivor-undispositioned` · `index-mismatch`.

**brainstorm totals:** `constraint` 11 · `routing` 4 · `fail` 4 · `reservation` 3 · `binding` 3 ·
`duty` 1 · `gate` 1 · `bound` 1 · `latitude` 1 = **29** ✓

---

### A.7 — Corpus kind totals

| Kind | impl | feat | spec | arch | setup | brainstorm | **Total** | Field written? |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `constraint` | 46 | 17 | 23 | 22 | 18 | 11 | **137** | no — the omitted default |
| `binding` | 11 | 8 | 8 | 7 | 5 | 3 | **42** | yes |
| `routing` | 11 | 11 | 6 | 4 | 3 | 4 | **39** | yes |
| `fail` | 15 | 1 | 9 | 1 | 6 | 4 | **36** | yes |
| `duty` | 5 | 9 | 0 | 10 | 1 | 1 | **26** | yes |
| `reservation` | 5 | 2 | 1 | 2 | 4 | 3 | **17** | yes |
| `gate` | 5 | 0 | 2 | 0 | 2 | 1 | **10** | yes |
| `latitude` | 2 | 1 | 2 | 1 | 1 | 1 | **8** | yes |
| `bound` | 4 | 0 | 0 | 0 | 0 | 1 | **5** | yes |
| **Total** | **104** | **49** | **51** | **47** | **40** | **29** | **320** | |

**183 rules gain an explicit `kind:` line; 137 stay bare.** Cross-checks against the record: the
`fail` total (36) matches D6's "36 fail nodes"; `latitude` (8) matches F3-as-corrected's "~8";
arch's nine `dm-*` duties and feat's eight match F3 exactly; implement's `duty` count (5) sits
inside F3's "impl 4–6 run-side analogues"; the three bounds F3 named are all present, with one
addition flagged in [J-5](#j-5--a-fourth-bound-the-record-named-three).

**Derived eval partition (build item 10's watch).** Under the record's proposed derivation
(fail + routing + exhaustion routes = contingency), the kind-derived contingency set is
`fail` 36 + `routing` 39 + `bound` 5 = **80 of 320 (25%)**. This is the figure to diff against
the hand-authored D8 partition when the post arm runs.

---

## B — `when:` extraction inventory

### B.0 — The movable-population test

A clause moves to `when:` **only** if it is a rule-**level** activation guard: the whole rule
either binds or does not, decided by the run's declared shape. Mid-sentence exceptions,
carve-outs, and multi-arm obligations stay prose (D3's single-homing clause).

Two dispositions are recorded:

- **MOVE** — the guard clause leaves `text` and lands in `when:`. A reword that keeps the ID; a
  strip entry with the verbatim removed clause is owed (build item 8).
- **DECLARE** — the rule is genuinely shape-gated but its condition is carried by the **subject
  noun**, not by a detachable guard clause. Stripping it would leave deixis or a dangling
  referent, which D15 forbids. `when:` is added; `text` is unchanged; no strip is owed (a pure
  addition rides the decision row). See [J-1](#j-1--single-homing-cannot-be-absolute-subject-carried-conditions)
  — the record states single-homing absolutely and did not anticipate this class.

**Population: 37 rules gain `when:` — 31 MOVE + 6 DECLARE.** Inside the record's 16–74 bracket.

Texts below are quoted **as they resolve** — YAML `>-` block scalars fold their newlines to
single spaces, so the folded single-line form is the semantic text a run sees.

### B.1 — implement (16: 14 MOVE + 2 DECLARE)

| # | Rule | `when:` | Disp. |
|---|---|---|---|
| 1 | `impl.epic-design-always-fires` | `{scope: [epic]}` | MOVE |
| 2 | `impl.epic-seam-owners` | `{scope: [epic]}` | MOVE |
| 3 | `impl.epic-card-sequence` | `{scope: [epic]}` | MOVE |
| 4 | `impl.epic-member-halt` | `{scope: [epic]}` | MOVE · **floor** |
| 5 | `impl.epic-shared-baseline-single-pen` | `{scope: [epic]}` | DECLARE |
| 6 | `impl.landing-selection` | `{scope: [selection]}` | MOVE |
| 7 | `impl.landing-epic` | `{scope: [epic]}` | MOVE |
| 8 | `impl.landing-delta` | `{scope: [delta]}` | MOVE |
| 9 | `impl.lane-never-widens` | `{scope: [lane]}` | DECLARE · **floor** |
| 10 | `impl.fail.gap-finding-missing` | `{scope: [selection, epic]}` | MOVE · **floor** · **D9** |
| 11 | `impl.fail.skip-unstated` | `{scope: [delta, lane]}` | MOVE · **floor** · **D9** |
| 12 | `impl.mutation-lens` | `{depth: [high]}` | MOVE |
| 13 | `impl.km-landing` | `{km_file: present}` | MOVE |
| 14 | `impl.briefs-name-rules-files` | `{governance_region: present}` | MOVE |
| 15 | `impl.design-absent-baseline-seed` | `{baseline: absent}` | MOVE |
| 16 | `impl.transport-floor` | `{seats: multi}` | MOVE · **floor** |

**1 · `impl.epic-design-always-fires`**
Current: `An epic's design phase always fires, for the joint spine at ${epics_dir}/EPIC-XXX/: the joint design plan, the joint architecture and seam design, and batch ordering. Spine artifacts follow templates/artifact-format.md.`
Reworded: `The design phase always fires, for the joint spine at ${epics_dir}/EPIC-XXX/: the joint design plan, the joint architecture and seam design, and batch ordering. Spine artifacts follow templates/artifact-format.md.`

**2 · `impl.epic-seam-owners`**
Current: `Every cross-member seam owner is named at design time (no later-lander default inside an epic).`
Reworded: `Every cross-member seam owner is named at design time — no later-lander default.`

**3 · `impl.epic-card-sequence`**
Current: `An epic builds one merged sequential card sequence from the joint design — shared foundation cycles first, then in-epic dependency order — with feature-tagged cards whose reports land in each member's ${features_dir}/FEAT-XXX/.`
Reworded: `The run builds one merged sequential card sequence from the joint design — shared foundation cycles first, then in-epic dependency order — with feature-tagged cards whose reports land in each member's ${features_dir}/FEAT-XXX/.`

**4 · `impl.epic-member-halt`** — `class: floor`
Current: `In an epic, exhaustion or no-progress halts member-scoped; the disposition — carve the member out (its rows return to pending, the epic continues) or hold the whole run — is the user's.`
Reworded: `Exhaustion or no-progress halts member-scoped; the disposition — carve the member out (its rows return to pending, the run continues) or hold the whole run — is the user's.`

**5 · `impl.epic-shared-baseline-single-pen`** — DECLARE, text unchanged. A shared-baseline delta
arises only inside an epic; the condition rides the subject noun ("A shared-baseline delta"), and
there is no guard clause to remove.

**6 · `impl.landing-selection`**
Current: `Selection scope lands the store landing plus the map's graduation batch: this run's delivered work rows fold into the capability's extent lines and vanish (pending rows persist) · status set delivered, dated, never regressing · the FEATURES.md index line updates · the specs-index row is touched — a spec reads closed exactly when all its selected rows have folded (derived, never asserted). No separate feature-close stage exists.`
Reworded: `The landing executes the store landing plus the map's graduation batch: this run's delivered work rows fold into the capability's extent lines and vanish (pending rows persist) · status set delivered, dated, never regressing · the FEATURES.md index line updates · the specs-index row is touched — a spec reads closed exactly when all its selected rows have folded (derived, never asserted). No separate feature-close stage exists.`

**7 · `impl.landing-epic`**
Current: `An epic lands each member's graduation batch, plus the epic close: the [EPIC-XXX] row markers vanish, the manifest is stamped delivered (dated), the spine directory persists as record. Multi-spec closure is compositional: each spec closes on its own rows, however many specs one landing touches.`
Reworded: `The landing executes each member's graduation batch, plus the epic close: the [EPIC-XXX] row markers vanish, the manifest is stamped delivered (dated), the spine directory persists as record. Multi-spec closure is compositional: each spec closes on its own rows, however many specs one landing touches.`

**8 · `impl.landing-delta`**
Current: `Delta scope lands the entry's marked delta fold.`
Reworded: `The landing executes the entry's marked delta fold.`
**Coverage note (F6-1):** with 6/7/8 declared, the per-dimension coverage report shows `scope:
lane` with **no landing rule** — the first F6 hole, now a deterministic finding instead of a hand
audit. Content is D9 user-gate work (build item 5), not this inventory's.

**9 · `impl.lane-never-widens`** — DECLARE, `class: floor`, text unchanged. Condition rides the
subject ("The lane … A product-lane run"); removing it would leave "It never widens in place".

**10 · `impl.fail.gap-finding-missing`** — `class: floor`, **the first D9 declaration**
Current: `A selection-scope or epic run without its gap-finding pass.`
Reworded: `A run without its gap-finding pass.`

**11 · `impl.fail.skip-unstated`** — `class: floor`, **the second D9 declaration**
Current: `A delta-scope or lane run whose final-validation report does not state that skip.`
Reworded: `A run whose final-validation report does not state its skipped gap-finding pass.`
The reword must name the skip: once the guard leaves, "that skip" has no antecedent (D15).

**12 · `impl.mutation-lens`**
Current: `The mutation lens runs on the verification seat, which already holds code sight — at high depth only, skips disclosed: a high-depth run owes mutation results or a stated skip.`
Reworded: `The mutation lens runs on the verification seat, which already holds code sight; skips are disclosed — the run owes mutation results or a stated skip.`

**13 · `impl.km-landing`**
Current: `Where .mochiko/memory/knowledge-management.md exists, the same landing carries its KM obligations.`
Reworded: `The same landing carries the knowledge-management obligations of .mochiko/memory/knowledge-management.md.`

**14 · `impl.briefs-name-rules-files`**
Current: `When a governance region is present, every code-touching brief names the relevant ${rules_dir}/ files as an obligated read.`
Reworded: `Every code-touching brief names the relevant ${rules_dir}/ files as an obligated read.`

**15 · `impl.design-absent-baseline-seed`**
Current: `On an absent baseline the phase's first duty is the seed — an empty scaffold stating so where no code is delivered; reconstructed from delivered code, and confirmed with the user at the checkpoint, where it exists. The seed is the baseline write; this feature's design still lands as deltas, never merged into the seed.`
Reworded: `The phase's first duty is the seed — an empty scaffold stating so where no code is delivered; reconstructed from delivered code, and confirmed with the user at the checkpoint, where it exists. The seed is the baseline write; this feature's design still lands as deltas, never merged into the seed.`
The two surviving "where" clauses are mid-sentence carve-outs on *how* the seed is written, not
rule-level guards — they stay.

**16 · `impl.transport-floor`** — `class: floor`. Guard moves; the residue is not an exact
duplicate of the guard-stripped common block, so implement does **not** bind it — see
[F.4](#f4--commontransport-floor--3-binds-guard-stripped).
Current: `A run composing more than one seat gains the transport floor: message legs on any multi-seat messaging, topology legs on shared writes — non-waivable once triggered.`
Reworded: `The transport floor governs the run's composition and messaging under a split trigger — message legs on any multi-seat messaging, topology legs on shared writes — non-waivable once triggered; referenced, never restated.`

#### Stays prose — implement (the near misses an auditor will ask about)

| Rule | Why it stays |
|---|---|
| `impl.gap-finding-scope` | Carries arms for all four scope values in one obligation (runs on selection/epic, skips and states the skip on delta/lane) — an intra-rule branch. |
| `impl.reports-envelope` | Parenthetical alternative home for a lane run, inside one obligation. |
| `impl.landing-verifier-folds` | Binds in every scope; only its second limb is lane-conditional — a carve-out, not an activation guard. |
| `impl.regression-sweep`, `impl.cold-verification` | One-sentence epic carves inside a rule that binds in every scope. |
| `impl.sufficiency-binding-verdict` | "per row, per card under delta scope" — both arms in one obligation. |
| `impl.absent-surfaces` | A per-surface branch table; also the only implement rule naming `brownfield`, which is *not* a declared implement dimension. |
| `impl.cards-template`, `impl.gates-fold`, `impl.graded-fold` | Render-degrade / absent-file carves inside a rule that always binds. |
| `impl.delta-reverification`, `impl.design-outputs-home`, `impl.design-map-assertion` | Mid-sentence carve-outs. |
| `impl.zero-gap-map-assertion`, `impl.design-phase-fires-on-gap` | Gated on whether the sufficiency check found gaps — a real branch, but **no dimension for it is declared**; see [J-3](#j-3--an-undeclared-branch-dimension-the-zero-gap-path). |

### B.2 — feature (2 MOVE)

**1 · `feat.dm-km-landing`** — `when: {km_file: present}`
Current: `Execute the KM landing for desk-side writes where .mochiko/memory/knowledge-management.md exists.`
Reworded: `Execute the KM landing for desk-side writes, per .mochiko/memory/knowledge-management.md.`

**2 · `feat.transport-floor`** — `when: {seats: multi}`, `class: floor`. Not an exact duplicate of
the common block (desk-voiced, longer) → no bind.
Current: `A visit that composes more than one seat gains a floor on its composition and messaging: a split trigger — message legs on any multi-seat messaging, topology legs on shared writes — non-waivable once triggered. Trigger test, floor legs, composition-safe shapes, and disclosure: mochiko:patterns-transport-floor, referenced never restated.`
Reworded: `The visit gains a floor on its composition and messaging: a split trigger — message legs on any multi-seat messaging, topology legs on shared writes — non-waivable once triggered. Trigger test, floor legs, composition-safe shapes, and disclosure: mochiko:patterns-transport-floor, referenced never restated.`

**Stays prose:** `feat.km-relation` (states **both** arms — with KM and the degrade path without
it — in one obligation; the exact contrast case against `feat.dm-km-landing`) ·
`feat.stable-ground-triage` (D3 names it explicitly) · `feat.dispatch-scope-split` (describes both
scopes) · `feat.architect-dormancy` ("dormant until the first cap-trip" is a temporal trigger, not
a shape dial) · `feat.out-of-remit-hosting` (an event trigger) · `feat.delta-cards`
(render-degrade) · `feat.single-flight-lane`, `feat.lane-never-widens` (desk-standing, always bind).

### B.3 — specify (5 MOVE + 4 DECLARE)

**1 · `spec.transport-floor`** — `when: {seats: multi}`, `class: floor`. Exact duplicate → binds
`common.transport-floor` (guard-stripped); the guard becomes the stub's local `when:`.
Current: `When the run composes more than one seat, mochiko:patterns-transport-floor governs its composition and messaging under a split trigger — message legs on any multi-seat messaging, topology legs on shared writes — non-waivable once triggered; referenced, never restated.`
Resolved text after binding: the guard-stripped block in [F.4](#f4--commontransport-floor--3-binds-guard-stripped).

**2 · `spec.km-landing`** — `when: {km_file: present}`
Current: `Where .mochiko/memory/knowledge-management.md exists, spec acceptance is a landing — run its landing ritual (close/move any BACKLOG item the spec discharges, touch ROADMAP.md) and its command-boundary invariants fix-on-sight; the acceptance-time map write batch lands in the same moment.`
Reworded: `Spec acceptance is a landing — run the .mochiko/memory/knowledge-management.md landing ritual (close/move any BACKLOG item the spec discharges, touch ROADMAP.md) and its command-boundary invariants fix-on-sight; the acceptance-time map write batch lands in the same moment.`

**3 · `spec.governance-briefs`** — `when: {governance_region: present}`
Current: `Where the CLAUDE.md governance region is present, name the relevant ${rules_dir}/ files as an obligated read in each author's brief — \`paths\`-scoped rules do not fire for from-scratch authoring.`
Reworded: `Name the relevant ${rules_dir}/ files as an obligated read in each author's brief — \`paths\`-scoped rules do not fire for from-scratch authoring.`

**4 · `spec.lockstep-prototyping`** — `when: {ux_bearing: yes}`
Current: `UX-bearing only: stories and their screens co-evolve as one unit — skeleton nav frame first, then each story's screens and flows land while that story is under discussion; the user clicks through while the story is wet, never a batch render after the text settles.`
Reworded: `Stories and their screens co-evolve as one unit — skeleton nav frame first, then each story's screens and flows land while that story is under discussion; the user clicks through while the story is wet, never a batch render after the text settles.`

**5 · `spec.stress-test-prototype-walk`** — `when: {ux_bearing: yes}`
Current: `On a UX-bearing spec the stress-test also walks the prototype (served, or opened directly via the no-server degrade path): every FLOW-XXX clickable end-to-end, every SCR-XXX reachable, every P1 acceptance scenario carrying a click path; manifest↔prototype drift is a blocking gap.`
Reworded: `The stress-test also walks the prototype (served, or opened directly via the no-server degrade path): every FLOW-XXX clickable end-to-end, every SCR-XXX reachable, every P1 acceptance scenario carrying a click path; manifest↔prototype drift is a blocking gap.`

**DECLARE (text unchanged) — the four subject-carried conditions.** These four give the coverage
report both arms of `feature_map`, `governance_region`, and `ux_bearing`:

| Rule | `when:` | Subject that carries it |
|---|---|---|
| `spec.map-obligated-read` | `{feature_map: present}` | "The **existing** feature map is an obligated read…" |
| `spec.missing-map-surfaced` | `{feature_map: absent}` | "A **missing** feature map is surfaced…" |
| `spec.governance-region-absent` | `{governance_region: absent}` | "A **missing** CLAUDE.md governance region is surfaced…" |
| `spec.whole-feature-prototype` | `{ux_bearing: yes}` | "The whole feature **is prototyped**…" — inapplicable where the prototype is waived. |

**Stays prose:** `spec.prototype-craft` (states both the UX-bearing craft and the waiver line — one
obligation, two arms) · `spec.fail.screens-flows` (three arms, one conditional) ·
`spec.frame-greenfield-inputs` ("a thin greenfield intent" — specify declares no `mode` dimension;
see [J-6](#j-6--greenfieldbrownfield-language-outside-setup)) · `spec.deliverable`,
`spec.feature-map-craft` (render-degrade) · `spec.epic-proposal-optional` (a judgment trigger) ·
`spec.reserved-to-user` (one bullet of an aggregate).

### B.4 — architecture (3 MOVE)

**1 · `arch.transport-floor`** — `when: {seats: multi}`, `class: floor`. Not an exact duplicate →
no bind.
Current: `A visit that composes more than one seat gains a floor on its composition and messaging: a split trigger — message legs on any multi-seat messaging, topology legs on shared writes — non-waivable once triggered. Trigger test, floor legs, composition-safe shapes, and disclosure: mochiko:patterns-transport-floor, referenced never restated.`
Reworded: `The visit gains a floor on its composition and messaging: a split trigger — message legs on any multi-seat messaging, topology legs on shared writes — non-waivable once triggered. Trigger test, floor legs, composition-safe shapes, and disclosure: mochiko:patterns-transport-floor, referenced never restated.`

**2 · `arch.dm-km-landing`** — `when: {km_file: present}`, `class: floor`
Current: `Execute the KM landing for desk-side writes where knowledge-management exists.`
Reworded: `Execute the KM landing for desk-side writes, per .mochiko/memory/knowledge-management.md.`
The reword also closes a D15 gap: the current text names "knowledge-management" with no path.

**3 · `arch.dm-author-baseline`** — `when: {store_ruled_content: absent}`, `class: floor`
Current: `Author the baseline wherever the store carries no ruled content — scaffold-only (a \`spine.md\` holding just its \`Scope:\` header) or absent: greenfield elicits it; brownfield reconstructs and confirms it per arch.tools-brownfield-reconstruction — nothing absorbed is ever silently discarded.`
Reworded: `Author the baseline — greenfield elicits it; brownfield reconstructs and confirms it per arch.tools-brownfield-reconstruction — nothing absorbed is ever silently discarded.`
The parenthetical **defines** the condition's two forms (scaffold-only or absent). Under
single-homing that definition belongs on the `conditions:` declaration, not in the rule text —
see [C.4](#c4--architecture). The greenfield/brownfield arms stay prose: both bind, and
architecture declares no `mode` dimension.

**Stays prose:** `arch.tools-brownfield-reconstruction` ("The first visit to an existing repo" —
a first-visit trigger plus an existing-repo condition, neither a declared arch dimension;
see [J-6](#j-6--greenfieldbrownfield-language-outside-setup)) · `arch.floor-precedence` ("Where a
governance floor card asserts the category" gates **a row**, not the visit — the clearest
row-level-vs-run-level case in the corpus) · `arch.tools-store-schema` (render-degrade).

**Architecture must not gain a `depth` dimension.** `arch.no-depth-dial-coupling` is a `class:
floor` prohibition on reading the governance depth level. Declaring `depth` in this schema's
`conditions:` block would contradict a floor.

### B.5 — setup (5 MOVE)

**1 · `setup.transport-floor`** — `when: {seats: multi}`, `class: floor`. Exact duplicate → binds
`common.transport-floor` (guard-stripped).
Current: `When the run composes more than one seat, mochiko:patterns-transport-floor governs its composition and messaging under a split trigger — message legs on any multi-seat messaging, topology legs on shared writes — non-waivable once triggered; referenced, never restated.`

**2 · `setup.feature-map-brownfield`** — `when: {mode: [brownfield]}`
Current: `Brownfield → the analysis (setup.interrogation-inputs) extends into a feature-map reconstruction — delivered capabilities derived from the code (routes, UI surfaces, services), confirmed with the user, landing as the initial FEATURES.md + .mochiko/features/ entries with \`delivered\` status and the reconstructed-from-code mark (shape: the features-index and feature-entry templates — mochiko-cli template features-index and mochiko-cli template feature-entry, or their schemas plugins/mochiko/schemas/features-index.yaml and plugins/mochiko/schemas/feature-entry.yaml Read raw when the binary is absent, the shipped schemas being the first-class source of truth; map machinery and the first-touch re-verify obligation: mochiko:authoring-feature-map).`
Reworded: `The analysis (setup.interrogation-inputs) extends into a feature-map reconstruction — delivered capabilities derived from the code (routes, UI surfaces, services), confirmed with the user, landing as the initial FEATURES.md + .mochiko/features/ entries with \`delivered\` status and the reconstructed-from-code mark (shape: the features-index and feature-entry templates — mochiko-cli template features-index and mochiko-cli template feature-entry, or their schemas plugins/mochiko/schemas/features-index.yaml and plugins/mochiko/schemas/feature-entry.yaml Read raw when the binary is absent, the shipped schemas being the first-class source of truth; map machinery and the first-touch re-verify obligation: mochiko:authoring-feature-map).`

**3 · `setup.feature-map-greenfield`** — `when: {mode: [greenfield]}`
Current: `Greenfield → scaffold the empty FEATURES.md index.`
Reworded: `Scaffold the empty FEATURES.md index.`
**Coverage note (F6-3):** with 2/3 declared, the coverage report shows `mode: amend` reaching
**no feature-map rule** — the third F6 hole, made deterministic. Content is D9 user-gate work.

**4 · `setup.user-conflict-rulings`** — `when: {mode: [brownfield]}`
Current: `Detected reality vs declared intent conflicts (brownfield) are the user's — confronted in the open, never silently resolved.`
Reworded: `Detected reality vs declared intent conflicts are the user's — confronted in the open, never silently resolved.`

**5 · `setup.user-map-confirmation`** — `when: {mode: [brownfield]}`
Current: `Confirmation of the reconstructed feature map, entry by entry (brownfield), is the user's.`
Reworded: `Confirmation of the reconstructed feature map, entry by entry, is the user's.`

**Stays prose:** `setup.baselines-bootstrap` (states **both** mode arms in one obligation — and is
separately reworded by [section I](#i--f8-move)) · `setup.store-scaffold-unconditional`,
`setup.architecture-scope-handoff`, `setup.map-never-overwrite`,
`setup.store-ruled-content-never-here`, `setup.fail.no-feature-map` (each names greenfield and
brownfield together; all also leave `amend` unnamed — see [J-7](#j-7--amend-mode-is-unnamed-in-six-rules-not-two)) ·
`setup.user-mode-ruling` (the dimension's **resolution point** — it cannot be gated on the
dimension it resolves) · `setup.interrogation-inputs` (one limb of a binding list) ·
`setup.synthesis-artifact` (render-degrade) · `setup.next-step` (a module carve).

### B.6 — brainstorm (2 MOVE)

**1 · `brainstorm.transport-floor`** — `when: {seats: multi}`, `class: floor`. Exact duplicate →
binds `common.transport-floor` (guard-stripped).
Current: `When the run composes more than one seat, mochiko:patterns-transport-floor governs its composition and messaging under a split trigger — message legs on any multi-seat messaging, topology legs on shared writes — non-waivable once triggered; referenced, never restated.`

**2 · `brainstorm.km-close-ritual`** — `when: {km_file: present}`
Current: `Where ${km_path} exists, run its close ritual.`
Reworded: `Run the close ritual of ${km_path}.`

**Stays prose:** `brainstorm.pair-maps-independent` ("In a review pair" — a genuine composition
branch with no declared dimension; see [J-2](#j-2--two-genuine-branches-with-no-declared-dimension)) ·
`brainstorm.next-step-offer`, `brainstorm.user-review-waiver`, `brainstorm.synthesis-on-request`
(mid-sentence carves) · `brainstorm.reopen-born-verify` (a subject-carried trigger, not a run
shape) · `brainstorm.index-bookkeeping` (its "where the outcome landed" is a noun, not a guard).

### B.7 — Floors carrying `when:` — the semantics comment

**Twelve `class: floor` rules gain a `when:`.** Under C4 a floor is **always read and always
delivered**, whatever its `when:`; the condition gates the obligation's *application*, never its
delivery. The checker makes no coverage claim over floors, and any shed set is re-evaluated when
the run's shape changes.

| Schema | Floors with `when:` |
|---|---|
| implement | `epic-member-halt` · `lane-never-widens` · `fail.gap-finding-missing` · `fail.skip-unstated` · `transport-floor` |
| feature | `transport-floor` |
| specify | `transport-floor` |
| architecture | `transport-floor` · `dm-km-landing` · `dm-author-baseline` |
| setup | `transport-floor` |
| brainstorm | `transport-floor` |

Every producer copies this comment verbatim onto each floor-with-`when:` block:

```yaml
        # C4: floor — always read, always delivered. `when:` gates when the
        # obligation APPLIES, never whether it is delivered. Re-evaluated
        # whenever the run's shape changes (a seat added mid-run re-activates it).
```

The six `transport-floor` rules are C4's named live case: their guard is seat count, so adding a
seat mid-run re-activates all six.

---

## C — Per-schema `conditions:` blocks

Only dimensions this schema's own rules use. Resolution points from D3's closed set:
`entry-derived` · `surface-presence` · `moment-resolved(<moment>)` · `user-ruled` ·
`standing-trigger`. Per D3-as-amended (I6), `moment-resolved` means "resolved at the named moment;
rules gated on it are inapplicable until it resolves" — no ordering is claimed.

Every dimension below is used by at least one rule (the unused-dimension check passes for all six).

### C.1 — implement

```yaml
conditions:
  scope:
    values: [selection, delta, epic, lane]
    resolution: entry-derived
    note: the batch's scope type, fixed at the run-open confirmation.
  depth:
    values: [low, high]
    resolution: entry-derived
    note: the governance depth level, read from the governance region at entry.
  km_file:
    values: presence
    resolution: surface-presence
    note: .mochiko/memory/knowledge-management.md.
  governance_region:
    values: presence
    resolution: surface-presence
    note: the marked governance region in CLAUDE.md.
  baseline:
    values: presence
    resolution: surface-presence
    note: the product baseline a design gap targets; checked at the entry sufficiency check.
  seats:
    values: [single, multi]
    resolution: standing-trigger
    note: fires the moment the run composes more than one seat, and re-fires on a seat added mid-run.
```

Used by: `scope` 7 rules · `seats` 1 · `km_file` 1 · `governance_region` 1 · `depth` 1 ·
`baseline` 1.

### C.2 — feature

```yaml
conditions:
  km_file:
    values: presence
    resolution: surface-presence
    note: .mochiko/memory/knowledge-management.md.
  seats:
    values: [single, multi]
    resolution: standing-trigger
    note: fires the moment the visit composes more than one seat, and re-fires on a seat added mid-visit.
```

The desk has no scope or mode dial: `feat.dispatch-scope-split` names implement's scopes but sets
none of its own. Two dimensions is the honest declaration.

### C.3 — specify

```yaml
conditions:
  ux_bearing:
    values: [yes, no]
    resolution: moment-resolved(intent)
    note: ruled in the intent stage; rules gated on it are inapplicable until it resolves.
  feature_map:
    values: presence
    resolution: surface-presence
    note: FEATURES.md plus the entries in the intent's territory.
  governance_region:
    values: presence
    resolution: surface-presence
    note: the marked governance region in CLAUDE.md.
  km_file:
    values: presence
    resolution: surface-presence
    note: .mochiko/memory/knowledge-management.md.
  seats:
    values: [single, multi]
    resolution: standing-trigger
    note: fires the moment the run composes more than one seat, and re-fires on a seat added mid-run.
```

`ux_bearing` is the corpus's **only** `moment-resolved` dimension — D3's demonstrated consumer for
the `moments:` block, and the reason specify's `moments:` must declare `intent`.

### C.4 — architecture

```yaml
conditions:
  store_ruled_content:
    values: presence
    resolution: surface-presence
    note: >-
      absent covers both forms — no store, and scaffold-only (a spine.md holding
      just its Scope: header).
  km_file:
    values: presence
    resolution: surface-presence
    note: .mochiko/memory/knowledge-management.md.
  seats:
    values: [single, multi]
    resolution: standing-trigger
    note: fires the moment the visit composes more than one seat, and re-fires on a seat added mid-visit.
```

The `store_ruled_content` note carries the two-form definition single-homed out of
`arch.dm-author-baseline`'s text (B.4-3). **No `depth` dimension** — `arch.no-depth-dial-coupling`
is a floor forbidding the desk to read it.

### C.5 — setup

```yaml
conditions:
  mode:
    values: [greenfield, brownfield, amend]
    resolution: user-ruled
    note: >-
      proposed from what the workspace shows; the user rules it whenever it is
      ambiguous (setup.user-mode-ruling).
  seats:
    values: [single, multi]
    resolution: standing-trigger
    note: fires the moment the run composes more than one seat, and re-fires on a seat added mid-run.
```

`amend` is declared because it is a real mode value the run resolves — and declaring it is exactly
what makes the F6-3 hole a deterministic coverage finding.

### C.6 — brainstorm

```yaml
conditions:
  km_file:
    values: presence
    resolution: surface-presence
    note: the knowledge-management file at ${km_path}.
  seats:
    values: [single, multi]
    resolution: standing-trigger
    note: fires the moment the run composes more than one seat, and re-fires on a seat added mid-run.
```

### C.7 — Coverage-report expectations

What the advisory per-dimension report should show once the wave lands (floors excluded from any
coverage claim, C4). *(Corrected at audit — V2 M2: this table is not exhaustive; the live
checker also emits correct-absence warnings it never pre-declared — `km_file: absent` on both
desks (the absent arm rides `feat.km-relation`'s always-binding text) and
`store_ruled_content: present` on architecture (the ordinary desk path, covered by every
unconditional rule) — the same expected class as the rows below, discovered at the first live
run.)*:

| Schema · dimension | Value with no rule | Disposition |
|---|---|---|
| implement · `scope` | **`lane`** — no landing rule | **F6-1 hole** → D9 user gate |
| setup · `mode` | **`amend`** — no feature-map rule | **F6-3 hole** → D9 user gate |
| implement · `depth` | `low` — no rule activates | Expected and correct: `low` is the absence of the high-depth mutation-lens obligation, not a hole. |
| specify · `ux_bearing` | `no` — no rule activates | Expected: the not-UX-bearing path is carried inside `spec.prototype-craft`'s waiver arm, which always binds. |
| all six · `seats: single` | no rule activates | Expected: the floor is a trigger, not a per-value table. |

---

## D — Per-schema `moments:` blocks

Declared, **unordered** anchor vocabulary — name plus one navigation line. **`at:` does not ship**
(D4 as amended, I5): moments exist for D3's resolution points and for prose to reference. Nothing
in this section schedules moments against each other.

~~Every moment below is referenced by at least one rule's prose (the unused-moment check
passes).~~ *(Corrected at audit — V1 M1 / V2 M1 / V3 M2: under the checker's literal-token
semantics, compound moment names appearing in no rule text warn as unused — `visit-open`
(feature, architecture) · `stories-confirm` (specify) · `session-open`, `cold-review`
(brainstorm). The blanket claim was wrong; the warnings are D4's own prescribed advisory and
carry into the D4 graduation review per the J-12 ruling.)*

### D.1 — implement

```yaml
moments:
  run-open: The confirmation that closes entry — batch, scope type, attempt bounds, done condition.
  entry: The gating and the sufficiency check, before the run-open confirmation.
  design-checkpoint: The user's sign-off on the design phase's output and the store delta.
  card-confirm: The user's ruling on the cycle slicing, before any card is built.
  cycle-checkpoint: The per-cycle grading point where escalations and findings land as one batch.
  final-validation: The whole-build verification pass — regression sweep, cold verification, gap finding.
  landing: The acceptance-time execution of folds, graduations, and the store landing.
  acceptance: The user's accept / amend / reject that closes the run.
```

`run-open` and `entry` are distinct on purpose: the sufficiency check runs at `entry`, and
`run-open` is the **only** redeclaration point for both attempt bounds
(`impl.attempt-per-grade`, `impl.gap-rework-bound`).

### D.2 — feature

```yaml
moments:
  visit-open: Map health surfaced before the ask, every visit.
  close: The verdict against the visit's done condition, with map integrity intact.
```

### D.3 — specify

```yaml
moments:
  intent: The adaptive-probe stage that closes in the user-confirmed intent synthesis.
  stories-confirm: The post-stories step where the capability frame is confirmed or adjusted.
  derivation: Where work rows are cut under their capabilities and the map delta is staged.
  selection: Where the user picks which work rows build now, from the selection card.
  acceptance: The user's accept / amend / reject, which executes the staged map write batch.
```

`intent` is the resolution moment named by `ux_bearing` in [C.3](#c3--specify) — the one
moment in the corpus with a D3 consumer.

### D.4 — architecture

```yaml
moments:
  visit-open: Store health surfaced from the derived index before the ask, every visit.
  close: The verdict against the visit's done condition, with store integrity intact.
```

### D.5 — setup

```yaml
moments:
  interrogation: The inline agenda-then-catalog stage, worked card by card with the user.
  ratification: The user's ratification of the synthesis — before any surface is authored.
  acceptance: The user's final acceptance of the surface set, flagged proposal by flagged proposal.
  close: Where the feature map and the bootstrapped surfaces must exist.
```

### D.6 — brainstorm

```yaml
moments:
  session-open: Where the index is read and the session is entered as open.
  cold-review: The blind two-message dispatch and the cold read of the frozen record.
  acceptance: The user's acceptance of the record.
  close: Where the index is updated with where the outcome landed and the KM close ritual runs.
```

---

## E — `enforces:` mapping

All **36** `kind: fail` nodes, mapped to the local rules they are the end-state contrapositive of.
**51 target links; every listed ID was verified to resolve against a live node in its own schema.**
Two nodes carry `enforces: []` with a reason (the weak-mirror case, D6).

### E.1 — implement (15)

| Fail node | `enforces:` |
|---|---|
| `impl.fail.sufficiency-unrecorded` | `[impl.sufficiency-report]` |
| `impl.fail.design-skipped` | `[impl.design-phase-fires-on-gap, impl.gate-design-checkpoint]` |
| `impl.fail.card-independence` | `[impl.seat-card-author-independence, impl.gate-card-confirm]` |
| `impl.fail.card-unchecked` | `[impl.progress-surface]` |
| `impl.fail.quality-gate` | `[impl.gates-full-suite, impl.gates-never-triaged]` |
| `impl.fail.no-evidence` | `[impl.craft-verify-bindings]` |
| `impl.fail.regression` | `[impl.regression-sweep]` |
| `impl.fail.baseline-in-place` | `[impl.baselines-never-in-place, impl.baseline-delta-grammar]` |
| `impl.fail.deviation-unresolved` | `[impl.deviation-gate]` |
| `impl.fail.store-landing-incomplete` | `[impl.store-landing]` |
| `impl.fail.ungraded-fold` | `[impl.graded-fold]` |
| `impl.fail.gap-finding-missing` | `[impl.gap-finding-scope]` |
| `impl.fail.skip-unstated` | `[impl.gap-finding-scope]` |
| `impl.fail.spec-gap-unresolved` | `[impl.finding-kinds]` |
| `impl.fail.no-acceptance` | `[impl.gate-final-acceptance]` |

`impl.gap-finding-scope` is enforced by two nodes — the two D9 conditional members, each mirroring
one arm of the same rule. That is the shape D9 made data: one obligation, two scope-conditional
contrapositives.

### E.2 — feature (1)

| Fail node | `enforces:` |
|---|---|
| `feat.fail.no-verdict` | `[feat.dm-converge-goal, feat.dm-close-verdict]` |

### E.3 — specify (9)

| Fail node | `enforces:` |
|---|---|
| `spec.fail.blocking-gap` | `[spec.stress-test-one-pass]` |
| `spec.fail.intent-unconfirmed` | `[spec.intent-stage-first, spec.reserved-to-user]` |
| `spec.fail.map-unread` | `[spec.map-obligated-read, spec.missing-map-surfaced]` |
| `spec.fail.story-unhomed` | `[spec.cut-work-rows, spec.filter-rejections-recorded]` |
| `spec.fail.screens-flows` | `[spec.prototype-craft, spec.stress-test-prototype-walk]` |
| `spec.fail.selection-unruled` | `[spec.gate-selection, spec.selection-card]` |
| `spec.fail.premature-map-write` | `[spec.staged-derivation, spec.gate-acceptance]` |
| `spec.fail.self-graded` | `[spec.author-grader-default-fail]` |
| `spec.fail.no-acceptance` | `[spec.gate-acceptance]` |

`spec.fail.blocking-gap` mirrors the rule that mandates the grading pass its gaps come from. The
*closure* obligation ("with no blocking gap left open") lives in specify's `.md` Goal step, not in
any schema rule — a partial mirror, recorded here so an auditor does not read it as a mismapping.
See [J-8](#j-8--one-fail-node-mirrors-an-obligation-that-lives-in-the-md).

### E.4 — architecture (1)

| Fail node | `enforces:` |
|---|---|
| `arch.fail.no-verdict` | `[arch.dm-converge-goal, arch.dm-close-verdict]` |

### E.5 — setup (6)

| Fail node | `enforces:` |
|---|---|
| `setup.fail.pre-ratification-authoring` | `[setup.gate-synthesis-ratification]` |
| `setup.fail.unclosed-trace` | `[]` — the intent→surface trace obligation is owned by `mochiko:authoring-constitution`, bound at `setup.surface-set`; no local rule states it. (D6's named exemplar.) |
| `setup.fail.author-graded` | `[setup.author-grader-default-fail, setup.stress-test-cold-seat]` |
| `setup.fail.floor-category-uncovered` | `[]` — the Essential Floor category set is owned by `mochiko:authoring-constitution`, carried on this node's own `pointer:`; no local rule enumerates it. |
| `setup.fail.no-acceptance` | `[setup.gate-final-acceptance]` |
| `setup.fail.no-feature-map` | `[setup.feature-map-brownfield, setup.feature-map-greenfield, setup.user-map-confirmation]` |

### E.6 — brainstorm (4)

| Fail node | `enforces:` |
|---|---|
| `brainstorm.fail.record-unaccepted` | `[brainstorm.user-record-acceptance]` |
| `brainstorm.fail.unreviewed-no-waiver` | `[brainstorm.record-review-independence, brainstorm.user-review-waiver]` |
| `brainstorm.fail.survivor-undispositioned` | `[brainstorm.coverage-survivor-routing, brainstorm.non-coverage-survivors]` |
| `brainstorm.fail.index-mismatch` | `[brainstorm.index-bookkeeping]` |

### E.7 — Reverse coverage (advisory; I7's deferred pass, not this wave's work)

Non-fail `class: floor` rules corpus-wide: **74** — architecture 21 · implement 19 · feature 12 ·
setup 12 · specify 7 · brainstorm 3. **This reproduces I7's figures exactly**, independently
computed.

After this wave's `enforces:` mapping, **57** of those 74 floors still have no fail mirror —
architecture 19 · implement 12 · feature 12 · setup 9 · specify 3 · brainstorm 2. Per I7 this is
**input to the user-deferred Desk FAIL-set widening pass and is never audit pressure**; the pair
audit's FAIL-survival handle covers the existing fail sets only.

---

## F — `extends:` bindings

### F.0 — Method and the C2 confirmation

Every command rule was compared, whitespace-normalized, against every common block. For the two
implement rules whose working-tree stubs carry no local `text:`, the comparison used their `git
HEAD` text (their pre-prototype form).

**C2's re-screen counts are confirmed exactly:**

| Block | Exact-duplicate count | Members | C2 said | Verdict |
|---|---:|---|---:|---|
| `common.register` | **5** | impl · feat · spec · arch · brainstorm | 5 | ✓ pass |
| `common.no-git-mutations` | **5** | feat · spec · arch · setup · brainstorm | 5 | ✓ pass |
| `common.acceptance-plain-text` | **3** | spec · setup · brainstorm | 3 | ✓ pass |
| `common.transport-floor` | **3** | spec · setup · brainstorm | 3 | ✓ pass |
| `common.author-grader-default-fail` | **2** | impl · brainstorm | 2 | ✗ fails the 3+ bar → **revert** |
| `common.model-tiering` | **2** | spec · setup | 2 | ✗ fails the 3+ bar → **revert** |

The counts match, but **the member sets are not what a reader would guess**, and two of them
exclude the very command the prototype was built on:

- `register`'s five **exclude setup** — `setup.register` reads "User-facing prose **per**
  templates/output-style.md" against the block's "**follows**".
- `no-git-mutations`'s five **exclude implement** — `impl.no-git-mutations` carries the
  ephemeral-snapshot carve.
- `acceptance-plain-text`'s three and `transport-floor`'s three **exclude implement, feature and
  architecture** — implement overrides both, and the two desks say "rulings"/"visit", not
  "acceptance"/"run".

### F.1 — Bind / no-bind, per command

A command binds a block **only** where its current text is an exact duplicate. A stub carrying a
full local text override buys nothing and is not used. Per C3, `class:` is declared locally on
every stub, and `kind:` / `when:` are local always.

| Block | impl | feat | spec | arch | setup | brainstorm |
|---|---|---|---|---|---|---|
| `common.register` | **BIND** | **BIND** | **BIND** | **BIND** | no-bind | **BIND** |
| `common.no-git-mutations` | no-bind | **BIND** | **BIND** | **BIND** | **BIND** | **BIND** |
| `common.acceptance-plain-text` | no-bind | n/a | **BIND** | n/a | **BIND** | **BIND** |
| `common.transport-floor` | no-bind | no-bind | **BIND** | no-bind | **BIND** | **BIND** |
| **Stubs per command** | **1** | **2** | **4** | **2** | **3** | **4** |

**16 binding stubs total** — and the per-block totals check out: register 5 + no-git-mutations 5 +
acceptance-plain-text 3 + transport-floor 3 = 16. ✓

**The headline for the build:** implement — the command the six-block prototype was written
against — ends the wave with **exactly one** binding stub (`impl.register`). Four of its six
prototype stubs carry local text overrides and one of its two clean stubs
(`author-grader-default-fail`) belongs to a reverting block. The prototype's read-path cost
lands almost entirely on commands that were never its subject.

### F.2 — Every no-bind, with its diff

**`common.register` vs `setup.register`**
```
block: User-facing prose follows templates/output-style.md.
setup: User-facing prose per     templates/output-style.md.
                        ^^^^^^^^^^^^^
```
One word. Recommend **no-bind**: rewording a shipped rule to make it bindable is content churn the
record did not authorize, and 5 already clears the bar. Flagged in [J-9](#j-9--two-one-token-misses).

**`common.no-git-mutations` vs `impl.no-git-mutations`**
```
block: Suggest commits; never run git mutations, never push.
impl:  Suggest commits; never run git mutations, never push (an ephemeral, self-removed
       verification snapshot is not a mutation of refs, index, tracked content, or history).
```
Recommend **no-bind** → a full per-command rule. The carve is implement-specific (it exists for
`impl.cold-verification`'s snapshot) and belongs nowhere else.

**`common.acceptance-plain-text` vs `impl.acceptance-plain-text`**
```
block: User acceptance is plain blocking text, never a timed prompt.
impl:  The user's ruling at each blocking gate — the design checkpoint, the card confirm,
       final acceptance — is plain blocking text, never a timed prompt.
```
Recommend **no-bind** → a full per-command rule. Implement's version enumerates its three gates.

**`common.acceptance-plain-text` vs `feat.rulings-plain-text` / `arch.rulings-plain-text`**
```
block: User acceptance is plain blocking text, never a timed prompt.
desks: User rulings    are plain blocking text, never a timed prompt.
```
Recommend **no-bind**, and note these are differently-named rules (`rulings-plain-text`), not
overrides of the same ID. A desk takes rulings every visit; it has no single acceptance. Merging
them would be a content change across two shipped desks.

**`common.transport-floor` vs `impl.transport-floor`**
```
block (guard-stripped): mochiko:patterns-transport-floor governs composition and messaging
       under a split trigger — message legs on any multi-seat messaging, topology legs on
       shared writes — non-waivable once triggered; referenced, never restated.
impl (guard-stripped):  The transport floor governs the run's composition and messaging under a
       split trigger — message legs on any multi-seat messaging, topology legs on shared writes
       — non-waivable once triggered; referenced, never restated.
```
Even after guard-stripping, implement's residue is not the block. Recommend **no-bind**.

**`common.transport-floor` vs `feat.transport-floor` / `arch.transport-floor`**
The desk texts are materially longer and desk-voiced ("A visit that composes…", plus a "Trigger
test, floor legs, composition-safe shapes, and disclosure" tail the block does not carry).
Recommend **no-bind**.

### F.3 — The two reverting blocks

`common.author-grader-default-fail` and `common.model-tiering` are **deleted from `common.yaml`**
(C2). The two implement stubs revert to full per-command rules, restored verbatim from `git HEAD`:

```yaml
      - id: impl.author-grader-default-fail
        labels: [independence]
        class: floor
        text: >-
          No output is cleared by its author — default FAIL.

      - id: impl.model-tiering
        labels: [seats, floor-pointer]
        kind: routing
        class: must
        text: >-
          Locate/enumerate reads go to a native Explore subagent spawned
          model: ${explore_model}; interpretive or absence-driven reads stay on the session
          tier; every seat brief carries the routing rule.
        pointer: "mochiko:patterns-model-tiering"
```

Both are pure restorations — no ID moves, no text changes against HEAD, so the wave's net effect
on these two rules is nil.

### F.4 — `common.transport-floor` — 3 binds, guard-stripped

After single-homing the seat-count guard out of the text (B.7), the shared block carries less than
it appears to. Drafted block:

```yaml
  - id: common.transport-floor
    labels: [floor-pointer, seats]
    text: >-
      mochiko:patterns-transport-floor governs composition and messaging under a
      split trigger — message legs on any multi-seat messaging, topology legs on
      shared writes — non-waivable once triggered; referenced, never restated.
    pointer: "mochiko:patterns-transport-floor"
```

Note the block carries **no `class:`**. Under C3, `class:` is always local and the checker asserts
a local `class:` on every stub — a `class:` in the block would be inherited-but-always-overridden,
which is dead weight and a trap for a reader who assumes it applies. **Recommend stripping
`class:` from all four surviving blocks**, not just this one. This follows from C3 rather than
extending it.

The three binding stubs, each re-declaring everything C3 makes local:

```yaml
      - id: spec.transport-floor          # identically for setup.* and brainstorm.*
        extends: common.transport-floor
        class: floor
        when: {seats: multi}
        # C4: floor — always read, always delivered. `when:` gates when the
        # obligation APPLIES, never whether it is delivered. Re-evaluated
        # whenever the run's shape changes (a seat added mid-run re-activates it).
```

### F.5 — The other three blocks, with per-stub locals

```yaml
  - id: common.register
    labels: [reporting]
    text: >-
      User-facing prose follows templates/output-style.md.

  - id: common.no-git-mutations
    labels: [user-gate]
    text: >-
      Suggest commits; never run git mutations, never push.

  - id: common.acceptance-plain-text
    labels: [user-gate]
    text: >-
      User acceptance is plain blocking text, never a timed prompt.
```

Per-stub locals — `labels` **are** inherited (C3 inherits text, labels, pointer), so a stub whose
current labels differ from the block must declare them locally or its labels silently change:

| Stub | `class:` (local) | `kind:` (local) | `labels:` local? |
|---|---|---|---|
| `impl.register` | `must` | `binding` | no — block's `[reporting]` matches HEAD |
| `feat.register` | `must` | `binding` | **yes** — `[binding, reporting]` |
| `spec.register` | `must` | `binding` | no — matches |
| `arch.register` | `must` | `binding` | **yes** — `[reporting, binding]` |
| `brainstorm.register` | `must` | `binding` | **yes** — `[binding, reporting]` |
| `feat.no-git-mutations` | `must` | — (constraint) | no |
| `spec.no-git-mutations` | `must` | — | no |
| `arch.no-git-mutations` | `must` | — | no |
| `setup.no-git-mutations` | **`floor`** | — | no |
| `brainstorm.no-git-mutations` | `must` | — | no |
| `spec.acceptance-plain-text` | `must` | — | no |
| `setup.acceptance-plain-text` | **`floor`** | — | no |
| `brainstorm.acceptance-plain-text` | `must` | — | no |
| `spec.transport-floor` | `floor` | — | no |
| `setup.transport-floor` | `floor` | — | no |
| `brainstorm.transport-floor` | `floor` | — | no |

Setup's two `class: floor` locals against block-level `must` are exactly the case C3's precedence
clause exists for: a floor's class stays readable from its own file. **Three register stubs must
declare `labels:` locally** — silently narrowing `feat.register` from `[binding, reporting]` to
`[reporting]` would be an unrecorded label change on a shipped rule.

### F.6 — `common.yaml` header discharge

The prototype header's "not yet ruled — a D6 grammar amendment is required before this ships" note
is discharged citing this session's D8. Also required: the resolution comment must state the C3
precedence clause (inherits `text`, `labels`, `pointer` **only**; `class`, `kind`, `when`,
`enforces` always local), replacing the prototype's "inherits every field".

---

## G — Canonical schema header comment

One block, per schema, `<cmd>`/`<prefix>` substituted. It keeps every surviving D6/D11/D12/D14/D15/D16
grammar line from the current headers and adds the D1–D8 grammar. `at:` is deliberately absent
(D4 as amended, I5).

```yaml
# Command content schema for /mochiko:<cmd> — SOURCE OF TRUTH for the command's
# rule-shaped content (command-content-schema D1/D7, DECISIONS.md 2026-08-26; grammar
# amended by command-schema-ontology D1–D8, DECISIONS.md 2026-08-27). The command .md
# instructs a raw, full Read of this file at command fire; the model interprets it live —
# no build step, no binary on the read path (GI-020). Narrative (Identity & Mission,
# Adaptive Goal Protocol prose) stays in plugins/mochiko/commands/<cmd>.md.
#
# Grammar (D6, as amended by D14 and by command-schema-ontology D1–D8):
# sections: list, each {id, title, intent, rules}; rule blocks
# {id, labels, class, kind?, when?, enforces?, extends?, text, pointer?} nest under their section.
#   section id  <prefix>.sec.<slug> — minted once like rule IDs (D11); title verbatim from
#           the group it carries; intent one line, navigation only — sections never grow a
#           second prose surface (narrative stays in the .md).
#   id      dotted slug — a NAME, never a summary; minted once, frozen (D11). A reword
#           keeps its ID; a split mints children recording the parent; a merge retires
#           the losers under a top-level tombstones: key.
#           FAIL clauses live under the <prefix>.fail.* segment.
#   labels  values from plugins/mochiko/schemas/command-labels.yaml (D8).
#   class   floor = non-waivable, must-survive under the pair audit (M3) ·
#           must = binding obligation · advisory = may change without supersession ceremony.
#   kind    one of constraint · duty · gate · reservation · binding · bound · routing ·
#           fail · latitude. CONSTRAINT IS THE DEFAULT — an absent kind: reads constraint
#           and is never written. kind crosscuts sections; it does not re-group them.
#           kind: fail is the operative selector for the Not-done set (never defaulted on
#           a <prefix>.fail.* ID), and the .md Not-done line counts exactly that set.
#   when    a conjunction of dimension: value terms from this schema's conditions: block —
#           declared vocabulary only, no boolean algebra, no negation beyond a declared
#           value, no free-form strings. It carries WHETHER the rule binds, never the
#           rule's internal logic: a rule-level activation guard lives in when: alone and
#           leaves the text (single-homing), while intra-rule branches, exceptions, and
#           carve-outs stay prose. On a class: floor rule, when: gates when the obligation
#           APPLIES — a floor is always read and always delivered, whatever its when:.
#   enforces  on kind: fail nodes only — the local rule IDs this end-state predicate is the
#           contrapositive of. An empty list is legal ONLY with a one-line reason (the
#           obligation lives in a pointer skill), so absence is a statement, never an omission.
#   extends  common.<slug> — inherits text, labels, and pointer ONLY from that block in
#           plugins/mochiko/schemas/common.yaml, which the .md instructs Read raw in the
#           same first action. class, kind, when, and enforces are ALWAYS local; a locally
#           declared text/labels/pointer replaces the inherited one; ${var} placeholders in
#           inherited text substitute from THIS schema's vars:; the stub's <prefix>.* ID
#           stays the citable ID.
#   text    carries ${var} placeholders substituted from vars: at read (D5). Verbatim
#           command strings carry no placeholders.
#   provenance  decision anchors live in .mochiko/provenance.yaml (D16, repo-side —
#           not shipped), keyed by rule ID; never read by the run; an anchored
#           rule still leaves only by recorded supersession-by-ruling.
#   pointer the skill that owns the floor or procedure — the rule holds the pointer,
#           never the procedure. Multi-skill bindings name every skill in text.
# Top-level conditions: declares every run-shape dimension this schema's rules use — the
#   name, its closed value set (or presence), and its resolution point: entry-derived ·
#   surface-presence · moment-resolved(<moment>) · user-ruled · standing-trigger. A
#   moment-resolved dimension is resolved at the named moment; rules gated on it are
#   inapplicable until it resolves.
# Top-level moments: declares the named anchor points this schema's rules reference, each
#   with one navigation line. The list is UNORDERED — relative sequence stays the .md
#   protocol's narrative and the lead's latitude, and no precedence between moments is
#   claimed or checkable.
# Referential closure (D15): a rule's text is self-contained — every reference resolves
#   within the block or the schema's addressable namespace (${var} · <prefix>.* IDs ·
#   <prefix>.sec.* · common.* blocks · class:/kind: values · conditions:/moments: names ·
#   labels · pointer: skills · file paths). A parenthetical <prefix>.<slug> citation is the
#   dependency idiom and must resolve to a live node or a tombstone (D5). Deixis ("these
#   rules", "this section", "above"/"below", document-shape remarks) is a defect — quoted
#   alone, the reference dangles. The checker lints a curated marker list (warning-class).
#   "this schema" and "the run" are legal self-reference.
#
# Rule grain (D12): one block per independently-citable obligation.
# Advisory checker: scripts/check-command-schema.py (D13) — exit-code signal only.
```

Per-schema variance to preserve from the current headers: the desks (feature, architecture)
declare `"this schema", "the desk", and "the visit"` as legal self-reference; implement, specify,
setup and brainstorm keep `"this schema" and "the run"`. Implement's and architecture's headers say
"charter audit" where specify's, setup's and brainstorm's say "pair audit" — the scaffold
standardization made all six pair-form commands, so **"pair audit" is correct everywhere**; this
wave should normalize the two stragglers (a comment-only change).

---

## H — Not-done re-key lines

`kind: fail` replaces the `fail-condition` label as the operative selector (build item 4). Counts
are unchanged by this wave — **N moves only if a later ruling adds a fail node**, which the D9 hole
rulings (build item 5) may do for `implement` (lane landing) and `setup` (amend mode).

| Command | N (current) | Line |
|---|---:|---|
| implement | **15** | `.md` line 84 |
| feature | **1** | `.md` line 57 |
| specify | **9** | `.md` line 61 |
| architecture | **1** | `.md` line 55 |
| setup | **6** | `.md` line 53 |
| brainstorm | **4** | `.md` line 48 |

**implement** — current:
> 3. **Not done — default FAIL:** the 15 rules labeled `fail-condition` in `plugins/mochiko/schemas/implement.yaml` (section `impl.sec.fail-conditions`) — any one standing fails the run. If the schema's `fail-condition` count is not 15, the pair is out of sync: halt and surface it before closing.

Reworded:
> 3. **Not done — default FAIL:** the 15 rules of `kind: fail` in `plugins/mochiko/schemas/implement.yaml` (section `impl.sec.fail-conditions`) — any one standing fails the run. If the schema's `kind: fail` count is not 15, the pair is out of sync: halt and surface it before closing.

**feature** — current:
> 3. **Not done — default FAIL:** the 1 rule labeled `fail-condition` in `plugins/mochiko/schemas/feature.yaml` (section `feat.sec.fail-conditions`) — any one standing fails the visit. If the schema's `fail-condition` count is not 1, the pair is out of sync: halt and surface it before closing.

Reworded:
> 3. **Not done — default FAIL:** the 1 rule of `kind: fail` in `plugins/mochiko/schemas/feature.yaml` (section `feat.sec.fail-conditions`) — any one standing fails the visit. If the schema's `kind: fail` count is not 1, the pair is out of sync: halt and surface it before closing.

**specify** — current:
> 3. **Not done — default FAIL:** the 9 rules labeled `fail-condition` in `plugins/mochiko/schemas/specify.yaml` (section `spec.sec.fail-conditions`) — any one standing fails the run. If the schema's `fail-condition` count is not 9, the pair is out of sync: halt and surface it before closing.

Reworded:
> 3. **Not done — default FAIL:** the 9 rules of `kind: fail` in `plugins/mochiko/schemas/specify.yaml` (section `spec.sec.fail-conditions`) — any one standing fails the run. If the schema's `kind: fail` count is not 9, the pair is out of sync: halt and surface it before closing.

**architecture** — current:
> 3. **Not done — default FAIL:** the 1 rule labeled `fail-condition` in `plugins/mochiko/schemas/architecture.yaml` (section `arch.sec.fail-conditions`) — any one standing fails the visit. If the schema's `fail-condition` count is not 1, the pair is out of sync: halt and surface it before closing.

Reworded:
> 3. **Not done — default FAIL:** the 1 rule of `kind: fail` in `plugins/mochiko/schemas/architecture.yaml` (section `arch.sec.fail-conditions`) — any one standing fails the visit. If the schema's `kind: fail` count is not 1, the pair is out of sync: halt and surface it before closing.

**setup** — current:
> 3. **Not done — default FAIL:** the 6 rules labeled `fail-condition` in `plugins/mochiko/schemas/setup.yaml` (section `setup.sec.fail-conditions`) — any one standing fails the run. If the schema's `fail-condition` count is not 6, the pair is out of sync: halt and surface it before closing.

Reworded:
> 3. **Not done — default FAIL:** the 6 rules of `kind: fail` in `plugins/mochiko/schemas/setup.yaml` (section `setup.sec.fail-conditions`) — any one standing fails the run. If the schema's `kind: fail` count is not 6, the pair is out of sync: halt and surface it before closing.

**brainstorm** — current:
> 3. **Not done — default FAIL:** the 4 rules labeled `fail-condition` in `plugins/mochiko/schemas/brainstorm.yaml` (section `brainstorm.sec.fail-conditions`) — any one standing fails the run. If the schema's `fail-condition` count is not 4, the pair is out of sync: halt and surface it before closing.

Reworded:
> 3. **Not done — default FAIL:** the 4 rules of `kind: fail` in `plugins/mochiko/schemas/brainstorm.yaml` (section `brainstorm.sec.fail-conditions`) — any one standing fails the run. If the schema's `kind: fail` count is not 4, the pair is out of sync: halt and surface it before closing.

**Downstream of the re-key** (named here so no producer misses them):

- The `fail-condition` label is retired from `plugins/mochiko/schemas/command-labels.yaml` by
  registry edit, and its 36 `labels:` occurrences are removed from the fail nodes.
- `.claude/rules/mochiko/primitive-edits.md` criterion 3 currently reads *"FAIL survival keys to
  the **`fail-condition` label set**"* — it re-keys to `kind: fail` (build item 9).
- The checker keeps the bidirectional `.fail.`-segment ↔ `kind: fail` cross-check (I4), with
  `kind: fail` never defaulted on a `.fail.*` ID.

---

## I — F8 move

### I.1 — The reworded rule

`setup.baselines-bootstrap`, `plugins/mochiko/schemas/setup.yaml:224–234`. Current text:

> `` `Assumed` (feature-sizing record, open thread 4 — setup reconstruction burden + partial-baseline poisoning risk): the brownfield analysis also bootstraps the product baselines — data-model.md · contracts/ · constraints-and-decisions.md · quickstart.md — at ${product_dir}/ (ARCHITECTURE.md stays repo root, now as the store's derived index) — from the delivered code; greenfield seeds those baselines at the first implement run's design phase instead. ``

Reworded — the inline confidence mark and the record citation removed, everything else verbatim:

```yaml
      - id: setup.baselines-bootstrap
        labels: [landing]
        class: must
        text: >-
          The brownfield analysis also bootstraps the product baselines —
          data-model.md · contracts/ · constraints-and-decisions.md ·
          quickstart.md — at ${product_dir}/ (ARCHITECTURE.md stays repo root, now
          as the store's derived index) — from the delivered code; greenfield seeds
          those baselines at the first implement run's design phase instead.
```

No `when:` (both mode arms live in one obligation — B.5). Kind stays `constraint`. ID unchanged, so
this is a reword, backed by a strip entry carrying the removed clause verbatim.

### I.2 — The sidecar entry

`.mochiko/provenance.yaml` **already carries this key** at line 102:

```yaml
  setup.baselines-bootstrap: "2026-08-26 plan-stage-utility"
```

The F8 move adds a **second** anchor — the feature-sizing record the stripped citation names. Its
`DECISIONS.md` row is dated **2026-08-10** and its record slug is
**`feature-sizing-and-entry-points`**, so the anchor string is
`"2026-08-10 feature-sizing-and-entry-points"`.

The sidecar's declared anchor grammar is **one string per key**, so this needs a grammar decision.
Recommended entry, as a list:

```yaml
  setup.baselines-bootstrap:
    - "2026-08-26 plan-stage-utility"
    - "2026-08-10 feature-sizing-and-entry-points"
```

This requires the checker's anchor-resolution check to accept a list value alongside a string, and
one line in the sidecar header stating that a rule may carry more than one anchor. Both are small,
but neither is authorized by the record — see [J-10](#j-10--the-f8-move-needs-three-things-the-record-did-not-name).
The single-string alternative (replacing the plan-stage-utility anchor) would drop a live anchor and
is not recommended.

### I.3 — Two further leak sites the record did not name

The `Assumed` mark and the feature-sizing citation appear in **three** places, not one. The record's
D9 names only the schema rule.

1. **`plugins/mochiko/commands/setup.md:48`** — the `.md` Goal step carries the same mark and
   citation:
   > `` `Assumed` (feature-sizing record, open thread 4 — reconstruction burden, partial-baseline poisoning): brownfield close also carries the bootstrapped product baselines at `.mochiko/product/`; greenfield leaves **the baselines** to seed at the first implement run's design phase. ``

   Moving the schema's mark while the `.md` keeps its own leaves the pair internally inconsistent —
   and the `.md` half is what the pair audit reads first.

2. **`plugins/mochiko/schemas/architecture.yaml:243`**, in `arch.tools-brownfield-reconstruction`:
   > "The derivation inherits the setup-bootstrap `Assumed` caveat and its partial-baseline poisoning risk: say so when you present it."

   This **references** the caveat by name. Once the mark leaves `setup.baselines-bootstrap`, the
   reference points at something that exists in no schema — a D15 referential-closure break
   introduced by the fix itself.

Neither is resolved here. Both are flagged in [J-10](#j-10--the-f8-move-needs-three-things-the-record-did-not-name)
for the wave's user gate.

---

## J — Anomalies

Flagged, not resolved — each is either a policy edge the record did not anticipate or a fact that
contradicts something the record states.

### J-1 — Single-homing cannot be absolute: subject-carried conditions

D3 states single-homing without qualification: "rule-level activation conditions live in `when:`
alone and **leave** the `text`". Six rules are genuinely shape-gated but carry the condition in
their **subject noun**, where removing it either produces deixis or destroys the referent:

| Rule | Condition-bearing subject | What removal would leave |
|---|---|---|
| `impl.lane-never-widens` | "**The lane** never widens…" | "It never widens in place" — deixis, D15 defect |
| `impl.epic-shared-baseline-single-pen` | "**A shared-baseline delta**…" | nothing to remove; the noun *is* the condition |
| `spec.map-obligated-read` | "The **existing** feature map…" | changes the obligation's meaning |
| `spec.missing-map-surfaced` | "A **missing** feature map…" | "A feature map is surfaced" — false |
| `spec.governance-region-absent` | "A **missing** CLAUDE.md governance region…" | same |
| `spec.whole-feature-prototype` | "The whole feature **is prototyped**…" | nothing to remove |

**Recommendation:** admit a DECLARE disposition — `when:` added, `text` unchanged, no strip owed
(a pure addition rides the decision row). The alternative readings are (a) forbid `when:` on these
six, which surrenders the coverage benefit on exactly the present/absent pairs that make the report
useful, or (b) force the reword and accept D15 breaks. Neither looks right. **Needs a ruling.**

### J-2 — Two genuine branches with no declared dimension

Both are real run-shape branches the record's F4 enumeration does not cover, and each is used by
exactly **one** rule — which fails D3's "only dimensions that schema's rules actually use" spirit
in the other direction (a dimension minted for a single rule).

- **`brainstorm.pair-maps-independent`** — "In a review pair, both seats build their Phase 0 angle
  maps independently." A candidate `review_pairing: [solo, pair]` dimension, resolved at the sizing
  gate. Recommend **prose this wave**.
- **`impl.design-absent-baseline-seed`** — handled by minting `baseline: presence` (C.1), which is
  also a single-rule dimension. It is recommended IN because it is a surface-presence flag of the
  same family as `km_file` / `governance_region`, whereas `review_pairing` is a new kind of dial.

The inconsistency is deliberate but contestable. **A validator should rule whether single-rule
dimensions are admissible at all**; if not, `baseline` comes out and
`impl.design-absent-baseline-seed` stays fully prose.

### J-3 — An undeclared branch dimension: the zero-gap path

Three implement rules branch on whether the sufficiency check found gaps —
`impl.design-phase-fires-on-gap` ("Fire the design phase on any gap"),
`impl.zero-gap-map-assertion` ("On the zero-gap path…"), and `impl.design-gaps-only`. This is a
real, moment-resolved (`entry`) binary that the F4 dimension survey does not list. Declaring
`gaps: [none, some]` would make the zero-gap path's coverage checkable. **Recommend prose this
wave** (D3 permits only declared vocabulary, and the record's enumeration is the referent), but the
omission is worth recording as a graduation candidate.

### J-4 — One latitude member beyond the record's candidate list

`spec.epic-proposal-optional` was not on F3's candidate list but matches D1-as-amended's definition
("a grant of per-run/per-visit judgment, neither obligation nor prohibition"). It is ruled **IN**
here, giving latitude a total of 8 — which happens to match F3's "~8" only because
`brainstorm.record-review-independence` was ruled **OUT**. The two changes cancel. A validator
should check both calls independently rather than accept the total as confirmation.

It also raises a definitional question the record does not settle: **`spec.epic-proposal-optional`
grants judgment to a seat, not to the lead.** All seven other latitude rules grant to the lead. If
latitude is lead-only by definition, this rule is a `constraint` and the total is 7.

### J-5 — A fourth bound; the record named three

F3 names three bounds, all implement (`attempt-per-grade`, `gap-rework-bound`, `no-progress-stop`).
This inventory rules **five**:

- `impl.epic-member-halt` → `bound`, because it declares the **exhaustion route** limb (the third
  member of D1's "numeric budget with redeclaration point and exhaustion route") for epic scope. It
  carries no budget of its own. A validator could equally rule it `reservation` (its second clause
  is "the disposition … is the user's").
- `brainstorm.reopen-born-verify` → `bound` — one round, with a stated consequence ("no second
  reopen off it"). The record's census did not reach outside implement for this kind.

The discriminator applied throughout was **"a countable allowance whose exhaustion has a stated
consequence"**, not D1's full three-limb phrasing — because **no rule in the corpus carries all
three limbs in one block**, D12's grain having split the redeclaration points onto
`impl.user-runopen-rulings` and the `vars:` comments. `feat.single-flight-lane` (an allowance of
one, no stated consequence) was ruled `constraint` on the same discriminator. **The record's
three-limb definition does not match its own corpus; the two-limb discriminator does.**

### J-6 — Greenfield/brownfield language outside setup

F4 assigns `mode` to setup alone, but three other schemas use greenfield/brownfield language:

- `impl.absent-surfaces` — "A missing or stale `.mochiko/memory/codebase-analysis.md` **on
  brownfield**"
- `spec.frame-greenfield-inputs` — "On a **thin greenfield** intent…"
- `arch.dm-author-baseline` — "**greenfield** elicits it; **brownfield** reconstructs and confirms it"
- `arch.tools-brownfield-reconstruction` — "The first visit to **an existing repo**…"

All four stay prose under D3 (each is an intra-rule branch or a non-declared trigger), so nothing
breaks. But the corpus reads as though a project-level greenfield/brownfield fact exists that only
setup declares as a dimension. If a later ruling promotes it to a shared dimension, these four are
its population. **No action this wave; recorded so the asymmetry is not mistaken for an oversight.**

### J-7 — `amend` mode is unnamed in six rules, not two

F6-3 names two sites where amend mode is unaddressed (`setup.feature-map-*` and
`setup.fail.no-feature-map`). The corpus has **six**:

`setup.feature-map-brownfield` · `setup.feature-map-greenfield` · `setup.fail.no-feature-map` ·
`setup.store-scaffold-unconditional` · `setup.architecture-scope-handoff` ·
`setup.map-never-overwrite`

The last three each say "on **both** the greenfield and brownfield paths" or "**greenfield or
brownfield**" — phrasing that reads as exhaustive but names only two of the three declared mode
values. `setup.store-ruled-content-never-here` ("on either path") has the same shape. Declaring
`mode: [greenfield, brownfield, amend]` makes all of these visible at once. **The D9 hole ruling
should be scoped to the full six, not the record's two** — otherwise the coverage report will fire
on four rules the user gate never considered.

### J-8 — One fail node mirrors an obligation that lives in the `.md`

`spec.fail.blocking-gap` ("A blocking gap open.") contrapositives an obligation — *close every
blocking gap before acceptance* — that appears in specify's `.md` Goal step ("with no blocking gap
left open") and in **no schema rule**. D6 gives `enforces:` two legal shapes: a list of local IDs,
or `[]` with a reason naming a **pointer skill**. Neither fits a `.md`-owned obligation.

Mapped here to `[spec.stress-test-one-pass]` — the rule mandating the pass its gaps come from — as
the closest true statement. A validator may prefer `enforces: []` with the reason "the closure
obligation is the `.md`'s fixed done condition". **Needs a ruling on whether `.md`-owned is a third
legal empty-reason class.**

### J-9 — Two one-token misses

Two rules miss exact-duplicate status by a single token, and in one case that token decides whether
a block clears the 3+ bar:

1. **`setup.register`** — `per` where the block says `follows`. One word keeps setup out of a
   5-member block. No consequence beyond one un-deduplicated rule.
2. **`brainstorm.model-tiering`** — identical to `common.model-tiering` **except for backticks
   around `Explore`** (similarity 0.997). Under a whitespace-normalized comparison it is not a
   duplicate, so `model-tiering` counts **2** and reverts. **Had those two backticks been absent,
   it would count 3 and survive the C2 bar.** C2's revert of `model-tiering` is therefore correct
   as measured, but it rests on a typographic difference, not a substantive one.

Neither is resolved here: normalizing either would be a text change on a shipped rule requiring a
strip, and the record authorized no such normalization.

### J-10 — The F8 move needs three things the record did not name

D9 calls the F8 fix "mechanical". It is not:

1. **The sidecar's anchor grammar is one string per key**, and `setup.baselines-bootstrap` already
   holds an anchor (`"2026-08-26 plan-stage-utility"`). Adding the feature-sizing anchor needs a
   list-valued form plus a checker change (I.2).
2. **The sidecar has no home for a confidence mark.** It carries *decision anchors*, keyed by rule
   ID. D9 says the "`Assumed` mark and record citation move to `.mochiko/provenance.yaml`" — the
   citation maps to an anchor, but `Assumed` maps to nothing in the sidecar's grammar. Either the
   mark is **dropped** (a substantive change: a hedge on a shipped obligation disappears) or the
   sidecar grows a field.
3. **The leak has three sites, and the record names one** — `setup.md:48` carries the same mark and
   citation, and `arch.tools-brownfield-reconstruction` *references* the caveat by name, so moving
   it creates a D15 dangling reference in a third schema (I.3).

**All three belong at the wave's user gate beside the D9 hole content.** Fixing only the site the
record names would leave the pair inconsistent and introduce a referential break.

### J-11 — The record's 29-token citation population is not "rule text"

F5 (as corrected at M2/M3) states the D5 scan population is "every `<cmd>.*` ID token in any rule
text — is **29** at HEAD (impl 2 · feat 9 · setup 7 · arch 6 · spec 5)". Parsing gives:

- Scanning **each schema for its own prefix only**: impl **0**, feat 9, spec 5, arch 6, setup 7,
  brainstorm 0 = **27**. Implement's two `impl.*` tokens live in the **header comment**
  (`impl.fail.*` and `impl.sec.*`), not in any rule text.
- Scanning **every schema for all six prefixes**: implement contributes its two `spec.md` mentions
  (`impl.design-inputs`, `impl.gap-finding-blind-dispatch`) → **29 exactly**, and M3's "five live
  `spec.md` path mentions" resolves as 3 in specify + 2 in implement.

So the record's arithmetic is right, but only under an **all-six-prefix, whole-corpus** scan — which
D5's own wording ("must resolve to a live node **in the same schema**") does not describe, since a
`spec.*` token in `implement.yaml` can never resolve in implement. **The checker's scan surface
needs pinning:** own-prefix-only (population 27) or all-prefixes (population 29, of which 5 are
excluded file-suffix lookalikes and 2 of those sit in a schema that cannot resolve them either way).

Two further facts for the checker's negative tests:

- **`feat.staffing-latitude` cites a SECTION id** — "carried as the Delivery Manager rules in
  `feat.sec.roles`". A resolver that only knows rule IDs will report a false dangle. Section IDs
  must be in the resolution set.
- **Three citations are bare inline, not parenthetical** — `feat.staffing-latitude` →
  `feat.sec.roles`, `arch.staffing-latitude` → `arch.sound-loop-floor`, `arch.dm-author-baseline` →
  `arch.tools-brownfield-reconstruction`. Plus `setup.interrogation-inline`'s descriptive
  parenthetical "(agenda and deck named in `setup.interrogation-inputs`)". These four sit **outside**
  D5's "19 parenthetical citations" but **inside** the checkable population: 24 real ID tokens
  (20 sites), all of which resolve today. D5's ratified idiom is the parenthetical form; the check
  must cover the bare form too, or three live references go unchecked.

### J-12 — `moments:` clears its demonstrated-consumer bar for one schema only

D4-as-amended (I5) ships `moments:` because "its demonstrated consumer is D3's moment-resolved
conditions, a dependency inside this same wave" — and deferred `at:` for failing exactly that bar.

The corpus has **one** moment-resolved condition: specify's `ux_bearing` at `intent` ([C.3](#c3--specify)).
For the other five schemas the `moments:` block has no in-wave consumer; it is declared vocabulary
for prose to reference — the same standing that got `at:` deferred, in the same record.

D4's statement is unambiguous ("**each** command schema gains a top-level `moments:` block") and
this inventory applies it to all six. But a reviewer applying I5's own reasoning would ask why five
of the six blocks ship. **Recorded, not resolved** — the policy is settled; the asymmetry is not.

### J-13 — Two schema headers say "charter audit" where the scaffold made all six pair-form

`implement.yaml:18` and `architecture.yaml:19` describe `class: floor` as "must-survive under the
**charter** audit (M3)"; the other four say "**pair** audit". `command-md-scaffold-standardization`
D1/D2 made all six pair-form with one criteria set. Comment-only, and section G normalizes it — but
it is a live inconsistency in shipped files that no strip records, so a producer should not "fix" it
silently outside this wave's recorded scope.

### J-14 — Two implement rules changed shape in the working tree before any ruling

`impl.register` and `impl.author-grader-default-fail` currently carry `extends:` with **no local
`class:`**, so their class is readable only after resolving `common.yaml`. That is precisely the
D15 single-file-readability exposure C3's precedence clause was written to close — and
`impl.author-grader-default-fail` is a `class: floor` rule, the case C3 names. The prototype is
uncommitted, so nothing shipped in this state; noted so a producer diffing against HEAD does not
mistake the prototype's shape for the current grammar. Both are repaired by C3 (local `class:`) —
and `impl.author-grader-default-fail` reverts entirely ([F.3](#f3--the-two-reverting-blocks)).

---

## Appendix — verification method

Every count in this file was produced by parsing the schemas with PyYAML, not by reading them:

- **Rule and section rosters** — per-schema walk of `sections[].rules[]`; totals 104 / 49 / 51 /
  47 / 40 / 29 = 320, matching F1.
- **Exact-duplicate scan** (section F) — whitespace-normalized comparison of every rule text
  against every `common.yaml` block, with `git show HEAD:plugins/mochiko/schemas/implement.yaml`
  supplying text for the two prototype stubs that carry none. Near-misses reported down to
  similarity 0.75.
- **Citation population** (J-11) — regex over rule text for all six ID prefixes, with file-suffix
  (`.md` / `.yaml`) tokens separated out, resolved against the union of rule IDs and section IDs.
- **`enforces:` verification** (section E) — all 51 target IDs resolved against the parsed ID set;
  zero unresolved.
- **Reverse coverage** (E.7) — non-`fail`-section `class: floor` count, with HEAD-class fallback
  for the two class-less prototype stubs; total 74, reproducing I7 per-schema exactly.
