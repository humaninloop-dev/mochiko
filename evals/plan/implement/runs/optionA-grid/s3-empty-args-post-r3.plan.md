# Action Plan — `/mochiko:implement` (plan-only, not executed)

**Scenario resolved from repo state** (read, not invented): `` was empty. Feature map (`FEATURES.md`) lists two capabilities — `FEAT-001` Note capture (delivered 2026-08-22) and `FEAT-002` Note search (status `selected`, two ratified work rows W1/W2, selection scope, dependency on FEAT-001 which is already delivered so it does not block). No `EPIC-XXX` exists. This resolves to a **selection-scope run on FEAT-002**.

Notable facts already on the ground that shape the plan below: no `.git` directory anywhere in the tree; no source code anywhere in the tree despite FEAT-001 reading "delivered"; `.mochiko/memory/` is empty (no `codebase-analysis.md`); `.claude/rules/mochiko/` is empty (no governance region); the spec's FR-103 requires "a background index worker" while the ruled architecture spine states "Synchronous request/response only; no queues, no background workers"; `contracts/api.yaml` has no `/notes/search` path; `data-model.md` has no search-index entity; FEAT-002's entry has no architecture link filled.

---

## Phase 0 — Load binding rules

**Done:** Read `plugins/mochiko/schemas/implement.yaml` raw and in full, `plugins/mochiko/schemas/common.yaml` raw and in full (for every `extends: common.*` stub), and `plugins/mochiko/schemas/command-labels.yaml` for label meanings. Substitute `${var}` placeholders from `implement.yaml`'s `vars:` block: `attempt_bound_cycle=3`, `gap_rework_bound=2`, `builder_seat=staff-engineer`, `design_seat=technical-analyst`, `architect_seat=principal-architect`, `qa_seat=qa-engineer`, `gap_finder_seat=devils-advocate`, `explore_model=haiku`. Confirm the fail-conditions section (`impl.sec.fail-conditions`) carries exactly 15 rules — verified: it does. If it hadn't, the run would halt here and surface the mismatch instead of proceeding.

**Read:** the three schema files above.
**Written:** nothing.
**Seats:** none (DM-only).
**Gate:** none.

---

## Phase 1 — Entry resolution and routing

**Done:** Confirm `FEAT-002` is the target (already resolved above from `FEATURES.md` + its entry file). Confirm the selected work rows (W1, W2) carry ratified scope and cite their selection source (spec's accepted selection, 2026-08-26). Confirm no selected row depends on an undelivered row — FEAT-001 is delivered, so no block.

**Read:** `FEATURES.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/features/FEAT-001/entry.md`.
**Written:** nothing yet.
**Seats:** none (DM routing judgment only).
**Gate:** none — this phase only establishes the batch; the user-facing gate comes at Phase 3.

---

## Phase 2 — Sufficiency check

**Done:** Dispatch an independent grading seat that authored none of `spec.md`, the architecture store, or the product baselines, and will not design or build this batch (`impl.seat-sufficiency-independence`). None of the vars-named seats fit cleanly (`technical-analyst` and `principal-architect` are candidate authors of the baselines/store; `staff-engineer` is the builder) — staffing this seat is DM judgment per `impl.staffing-latitude`; the natural fit is `mochiko:validator` (never grades its own work, defaults FAIL). This seat runs the `mochiko:review-sufficiency` procedure: the ten-clause check per selected work row (W1, W2), grading spec + architecture store + product baselines for design sufficiency.

Based on what's already on record, this check is very likely to surface at least two named gaps:
- **W1** (search by query): no `/notes/search` path in `contracts/api.yaml`, no search-index entity in `data-model.md`.
- **W2** (index freshness): FR-103 requires a background index worker; the architecture spine's ruled concern catalog states no queues/background workers exist — a direct contradiction between spec and ruled architecture, which the sufficiency check treats as a gap (or a store trip) rather than something it can clear itself, per `impl.sufficiency-disputed-clause` ("a disputed sufficiency clause defaults to gap and goes to the user — the grader never clears alone").

Absent-surface handling fires here too, non-blocking: the missing governance region and the missing/absent `codebase-analysis.md` are surfaced, never auto-resolved, never run-failing (`impl.absent-surfaces`).

**Read (by the sufficiency seat):** `spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`, `FEAT-002/entry.md`.
**Written:** `.mochiko/features/FEAT-002/sufficiency-report.md` — per-row verdict (sufficient / gap list), the store-consult result, any trips for the user, the `quickstart.md` null-path note (no external-integration surface here), any `[MODIFY]` amendment note (none expected against FEAT-001, which is out of scope).
**Seats:** `mochiko:validator` (or DM's chosen independent seat) — exempt from plan approval, like every grading seat.
**Gate:** none yet; the verdict feeds Phase 3.

---

## Phase 3 — Run-open confirmation (USER GATE)

**Done:** One confirmation message, no negotiation, covering exactly:
- Batch and scope type: FEAT-002, selection scope (not epic, not delta).
- Both attempt bounds restated at their only redeclaration point: 3 verification attempts per cycle, 2 gap-rework rounds at final validation — offered for the user to change now or accept as default.
- The sufficiency verdict and its gap routing: presenting the report from Phase 2, including the FR-103-vs-architecture-spine contradiction as a trip requiring a ruling (ruled now, or explicitly deferred on the record), plus the absent-surface notes (no governance region, no codebase-analysis.md — offering `/mochiko:setup` or proceeding with the warning logged).
- The done condition: every cycle card checked, built test-first, independently verified per-cycle and at final validation against real infrastructure, code traces to FR-101/FR-102/FR-103 and SC-101/102/103, acceptance landing executed whole, run closes on accept/amend/reject.

**Read:** the sufficiency report just written.
**Written:** nothing (this is a confirmation, not an artifact).
**Seats:** DM only.
**Gate — the gate itself:** plain blocking text, not a timed prompt. What's confirmed: attempt-bound values, the trip ruling on the background-worker conflict, and disposition on the absent-surface offers.
- **If the user rules "proceed to design"** (most likely, given gaps exist): flow continues to Phase 4 (design phase fires — mandatory here per `impl.design-phase-fires-on-gap`, since Phase 2 found gaps).
- **If the user instead rules the FR-103 requirement out of scope for this run** (e.g., defers the background-worker question and asks for synchronous-only indexing): the gap list narrows accordingly before Phase 4 fires, and the deferral is recorded on the sufficiency report.
- **If the user declines to rule and asks for more analysis first:** the run pauses; no code phase opens until this gate closes.

---

## Phase 4 — Design phase (fires: gaps were named)

**Done:** Staff design seats scoped to exactly the named gaps, nothing more (`impl.design-gaps-only`, rung-justified per `mochiko:patterns-plan-minimalism`):
- `technical-analyst` (design_seat) for the API-contract and data-model deltas (the `/notes/search` endpoint, the search-index entity) and for the `constraints-and-decisions.md` delta if a new D-XXX/NFR-XXX is needed for the indexing approach.
- `principal-architect` (architect_seat) for the store delta — resolving the FR-103-vs-spine conflict: either a ruled amendment to the concern catalog admitting a background index worker (with a new/updated AX row and NFR target), or a redesigned synchronous-indexing approach that fits the existing "no background workers" stance. This is exactly the kind of structural call `impl.deviation-gate` and `impl.constraint-challenge` exist for — the ratified "no background workers" stance is never silently overridden; if the architect's recommendation collides with it, that collision is filed as a constraint-challenge finding for the user, not decided by the seat.
- `qa-engineer` (qa_seat) for the `**TEST:**` acceptance cases tied to SC-101/102/103.

Each design seat works only on a plan the DM approved first (`impl.plan-approval-producers`). Absent-baseline seeding does not apply here (baselines exist). The design phase also asserts design-implied dependencies/extent onto FEAT-002's entry with provenance, and fills the architecture link once the store delta exists (`impl.design-map-assertion`).

**Read (by design seats):** `sufficiency-report.md`, `spec.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `architecture/spine.md`.
**Written:** `.mochiko/features/FEAT-002/data-model.md` (delta), `.mochiko/features/FEAT-002/contracts/` (delta), possibly `.mochiko/features/FEAT-002/baseline-delta.md` if a constraints/decisions delta is needed, a store delta at `.mochiko/product/architecture/` per `mochiko:authoring-architecture-store` grammar (in-flight-class elements only — the one legal in-place carve, standing beside the ruled content until landing), `.mochiko/features/FEAT-002/entry.md` (map-delta: dependencies/extent/architecture-link fields).
**Seats:** `technical-analyst`, `principal-architect`, `qa-engineer` — each plan-gated by the DM first.
**Gate:** none inside this phase; output feeds Phase 5's independent review.

---

## Phase 5 — Design-phase review (non-author, before the checkpoint)

**Done:** A non-author seat pair grades the Phase 4 package before it reaches the user:
- `mochiko:review-plan-artifacts` for conformance to the gap list and card-quality/coverage (blocking verdict: ready / needs-revision / critical-gaps).
- `mochiko:review-feasibility` for buildability/contradiction — specifically whether the store delta actually resolves the FR-103 conflict without silently redesigning around it.

If either verdict is not clean, the design seats rework on the DM's approved plan and this phase re-runs (bounded by run judgment, not the numeric attempt bounds, which are cycle/final-validation specific).

**Read:** all Phase 4 outputs, `sufficiency-report.md`.
**Written:** review findings (as part of the reviewing seats' standard report output, landing in `.mochiko/features/FEAT-002/`).
**Seats:** a seat distinct from `technical-analyst`/`principal-architect`/`qa-engineer` — e.g. `mochiko:tech-lead` and/or `mochiko:devils-advocate`, DM's staffing call.
**Gate:** none yet (feeds Phase 6).

---

## Phase 6 — Design checkpoint (USER GATE)

**Done:** Present the signed-ready design package: the FEAT-002 deltas, the store delta rendered as a diagram plus its named `AX-XXX` row changes (or, absent a render surface, the source plus a changed-element table), and the review verdicts from Phase 5.

**Read:** nothing new — presentation of Phase 4/5 artifacts.
**Written:** nothing until the user signs; on sign, the store delta becomes the ruled anchor for later deviation checks.
**Seats:** DM presents; no seat rules.
**Gate — what's confirmed:** does the user sign the design and the store delta as-is? Branches:
- **Sign:** proceed to Phase 7 (cycle-card authoring) with this delta as the fixed deviation anchor.
- **Amend:** the user redirects the design (e.g., picks synchronous re-indexing over a background worker, or vice versa); Phase 4 re-scopes to the amendment and Phase 5 re-reviews before returning here.
- **Stop here, resume later:** the run pauses cleanly; nothing built. (The plan for this evaluation ends its live trace at this pause if chosen — no code phase opens.)

---

## Phase 7 — Cycle-card authoring and card review

**Done:** A design-class seat (never the builder) slices FEAT-002's signed design into cycle cards — foundation cycles before feature cycles, `Simple/Split/Merge` per `mochiko:patterns-vertical-tdd`. Likely shape: a foundation cycle for the search-index data structure/build path, then a feature cycle for W1 (query search, FR-101/102, SC-101/102), then a feature cycle for W2 (freshness, FR-103, SC-103) if the design didn't already merge W1/W2 into fewer cards. `qa-engineer` authors the `**TEST:**` real-infrastructure gate per card within its slicing. Cards carry stories/rationale, dependencies, acceptance-criteria IDs, the `**TEST:**` gate, and brownfield exposure — no task lists or file paths (builder decomposes those at build time).

A non-author verification seat reviews the cards before confirm — quality per `review-plan-artifacts`, buildability its own judgment (`impl.card-review-before-confirm`).

**Read:** the signed Phase 4 deltas, `spec.md` acceptance-criteria IDs, `plugins/mochiko/schemas/tasks.yaml` (as the tasks-template source of truth).
**Written:** `.mochiko/features/FEAT-002/tasks.md` (cycle cards, unchecked).
**Seats:** `technical-analyst` (or the DM's chosen design-class card author) to slice, `qa-engineer` for TEST cases, a distinct verification seat (e.g. `mochiko:qa-engineer` acting independently, or `mochiko:tech-lead`) to review before confirm.
**Gate:** feeds Phase 8.

---

## Phase 8 — Card confirm (USER GATE)

**Done:** Present the sliced cards and the reviewing seat's buildability/quality verdict.

**Read:** `tasks.md`.
**Written:** nothing until ruled.
**Seats:** none rule; DM presents.
**Gate — what's confirmed:** does the user accept this slicing before any build starts?
- **Accept:** proceed to Phase 9 in this exact card order.
- **Reject/reslice:** Phase 7 re-runs with the user's redirection (e.g., merge W1/W2 into one cycle, or split further).

---

## Phase 9 — Build, cycle by cycle (test-first, foundation before feature)

**Done, per card, in order:** `staff-engineer` (builder_seat) decomposes the card into concrete build-time tasks (disclosed in the cycle report), applies `mochiko:patterns-code-minimalism`'s pre-code ladder at decomposition (rungs disclosed), follows `mochiko:brownfield-integration` if the card touches existing code (relevant here since FEAT-001's HTTP layer/SQLite store is the base FEAT-002 builds search on top of — though notably no source tree currently exists in this repo, which the builder's first cycle would need to reconcile against the "delivered" FEAT-001 status), and drives red→green→refactor on a DM-approved plan. Each red/green/refactor pass builds on the actual codebase.

A non-implementer verification seat then grades the cycle against real infrastructure (`mochiko:testing-end-user`, evidence captured not assumed) plus the `mochiko:review-code-minimalism` lens on the diff, the cycle report, and the surrounding code. Each grading pass consumes one of the 3 per-cycle attempts. Two consecutive rounds with unchanged findings is a no-progress stop (halt, present state — DM/user judgment). Reserved-to-user questions batch at the cycle checkpoint unless build-blocking.

If a builder hits undesigned structure mid-cycle, that cycle halts and Phase 4's design phase re-fires scoped to the discovery (`impl.midrun-refire`), looping back through Phases 4–6 for that slice before resuming here.

**Read (per cycle):** the card, prior cycle reports, the relevant baseline/delta files, `${rules_dir}` files if a governance region exists (it doesn't currently — so this obligation is inert unless the Phase 3 absent-surface offer resulted in running `/mochiko:setup` first).
**Written (per cycle):** working code changes (paths depend on the eventual project layout the builder establishes/extends — none exists yet), `.mochiko/features/FEAT-002/cycle-report.md` entries (dated, appended), `tasks.md` checkbox flips as each card completes.
**Seats:** `staff-engineer` (build), a distinct verification seat e.g. `qa-engineer` (per-cycle grading).
**Gate:** the escalation-batching checkpoint at each cycle boundary is DM-managed, not a full user-blocking gate unless a build-blocking question or an infeasible-card / adopt-first / IP-XXX call arises — those specifically escalate to the user mid-cycle per `impl.sec.reserved`.

---

## Phase 10 — Final validation (whole-run verification)

**Done:** Once every card is `[x]`:
- Run the full repository quality-gate suite (`impl.gates-full-suite`) — no severity triage, any failure fails the run.
- Cold verification: build/test from a dependency-cold snapshot of the uncommitted working state. **Concrete blocker to flag:** this repo has no `.git` directory, and the snapshot mechanism (`git ls-files -co --exclude-standard :!.claude/worktrees`) is git-based — this cannot run as specified until the working tree is a git repository. This would be surfaced to the user as a run-blocking prerequisite, not silently skipped or worked around.
- Regression sweep: re-run FEAT-001's durable gate set (`gates.md`) since FEAT-002 builds in FEAT-001's territory (notes store).
- Gap-finding pass (selection-scope run → mandatory, `impl.gap-finding-scope`): a fresh `devils-advocate` (gap_finder_seat), dispatched blind — first message carries only `spec.md`, `sufficiency-report.md`, design deltas, and the baselines (never code, `tasks.md`, TEST cases, or reports); it states derived expectations before probing begins. Mutation lens applies if this run is staffed at high depth; otherwise a stated skip.
- Findings split by kind: spec-required behavior broken fails final validation (cited clause, evidence); beyond-spec findings are advisory and go to the user for fix-now/backlog/accept-as-designed.

**Read:** all cycle reports, the store delta, all baselines, `gates.md` for FEAT-001.
**Written:** `.mochiko/features/FEAT-002/final-validation-report.md`, gap-finding evidence, the built-vs-signed diff for the store delta.
**Seats:** the verification seat(s) already used for per-cycle grading, plus `devils-advocate` for gap-finding.
**Gate:** any findings needing disposition (beyond-spec) or disputed kind classification batch into the next user-facing checkpoint (Phase 11), bounded by the 2-round gap-rework attempt bound.

---

## Phase 11 — Landing (executed whole, at acceptance)

**Done, only once the user accepts (see Phase 12) or as the evidence package presented at acceptance:**
- Store landing (if a store delta exists from Phase 4): flip delta elements built, clear their FEAT-002 key, write graded `As-built:`/`Drift:` fields on touched rows, run the orphan check, regenerate the derived root `ARCHITECTURE.md`.
- Graded fold of every touched baseline (three-way diff: pre-fold + delta vs folded result) — `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md` as applicable.
- Map graduation batch: FEAT-002's delivered rows (W1, W2) fold into the capability's extent lines and vanish from "Work rows"; status → `delivered` (dated); `FEATURES.md` index line updates; the note-search spec's index row reads closed.
- Gates fold: any gap findings ruled fix-now/backlog fold into `.mochiko/features/FEAT-002/gates.md` (minted), authored by `qa-engineer` in `**TEST:**` grammar.
- KM landing: `.mochiko/memory/knowledge-management.md` doesn't exist in this repo, so this obligation is inert for this run.

**Read:** all graded artifacts from Phases 9–10.
**Written:** `.mochiko/product/architecture/spine.md` (store fold), `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md` (baseline folds), `.mochiko/features/FEAT-002/entry.md`, `FEATURES.md`, `.mochiko/features/FEAT-002/gates.md`, `ARCHITECTURE.md` (regenerated, single writer).
**Seats:** the landing verification seat checks the graded folds (independence rule — never the implementer).
**Gate:** executed whole, only at Phase 12's acceptance — never partially before it.

---

## Phase 12 — Final acceptance (USER GATE) and close

**Done:** Present the complete package: final-validation report, gap-finding disposition, the landing that would execute, and a verdict against the done condition checked against all 15 fail-conditions from `impl.sec.fail-conditions` (sufficiency recorded, design signed if gaps existed, card independence held, no unchecked cards, gates passing, evidence real, no regressions, baselines never edited in place, deviations resolved, store landing complete, folds graded, gap-finding present, skip stated if applicable, no unresolved spec-required gap finding, and this final one — acceptance itself).

**Read:** nothing new — synthesis of everything above.
**Written:** nothing until ruled; on accept, Phase 11's landing writes execute.
**Seats:** DM presents; no seat rules.
**Gate — what's confirmed:** accept / amend / reject, plain blocking text.
- **Accept:** Phase 11 landing executes whole; run closes PASS against the done condition.
- **Amend:** the user specifies a change (e.g., rework one cycle, revisit a beyond-spec finding's disposition); the relevant earlier phase (9, 10, or a targeted re-cycle) re-runs bounded by the gap-rework attempt count, then returns here.
- **Reject:** run closes FAIL; nothing lands; state is presented for the user's next move (e.g., route to `/mochiko:feature` for a delta, or hold).