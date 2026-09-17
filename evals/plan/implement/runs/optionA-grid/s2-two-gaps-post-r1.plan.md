# Action Plan — `/mochiko:implement FEAT-002` (Plan-Only, Not Executed)

## Phase 0 — Schema load (done during planning)

- **Read** `plugins/mochiko/schemas/implement.yaml` (raw, full) and `plugins/mochiko/schemas/common.yaml` (raw, full — pulled in via `extends: common.*` stubs) and `plugins/mochiko/schemas/command-labels.yaml`.
- Counted the `impl.sec.fail-conditions` rules: 15 (`sufficiency-unrecorded`, `design-skipped`, `card-independence`, `card-unchecked`, `quality-gate`, `no-evidence`, `regression`, `baseline-in-place`, `deviation-unresolved`, `store-landing-incomplete`, `ungraded-fold`, `gap-finding-missing`, `skip-unstated`, `spec-gap-unresolved`, `no-acceptance`). Matches the command's hard-coded count — no halt needed.
- Vars resolved: `attempt_bound_cycle=3`, `gap_rework_bound=2`, `builder_seat=staff-engineer`, `design_seat=technical-analyst`, `architect_seat=principal-architect`, `qa_seat=qa-engineer`, `gap_finder_seat=devils-advocate`, `explore_model=haiku`.
- Nothing written in this phase.

## Phase 1 — Entry resolution

- **Read** `FEATURES.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/specs/note-search/spec.md`, `.mochiko/features/FEAT-001/entry.md`.
- Findings: `FEAT-002` is a plain capability ID (not `EPIC-XXX`, no epic lookup needed). Scope source is **selection scope** — the spec's accepted selection (ratified 2026-08-26), not a delta card — so `/mochiko:authoring-epic` entry rules don't apply. Selected work rows: W1 (US-101, SC-101/SC-102), W2 (US-102, SC-103). Dependency FEAT-001 is `delivered`, so no dependency block.
- Nothing written.

## Phase 2 — Sufficiency check (entry gate)

- **Seat:** an independent grader that authored none of `spec.md`, the architecture store, or the product baselines — none of `technical-analyst` (baselines' likely author), `principal-architect` (store's author), or `requirements-analyst` (spec's author) qualify. Assign `mochiko:validator`, run per `mochiko:review-sufficiency`, ten-clause check per row (W1, W2).
- **Read** (grounding for this plan): `.mochiko/product/contracts/api.yaml`, `.mochiko/product/data-model.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/architecture/spine.md` — confirms concretely:
  - No `/notes/search` endpoint exists in the API contract (W1 gap).
  - No search/index entity in the data model (W1/W2 gap).
  - The architecture spine explicitly rules "Synchronous request/response only; no queues, no background workers" — FR-103's background index worker directly contradicts this ruled topology (W2 gap, and a **store trip**, not a routine gap).
  - No NFR-XXX concern row for the ≤2s freshness target (W2 gap).
  - No technology decision for the search/indexing approach — this is a commodity category (full-text search), so the adopt-first check applies and is unresolved (W1/W2 gap).
- **Verdict (binding):** insufficient on both rows; gap list = {API contract delta, data-model delta, architecture-store delta + NFR row, D-XXX indexing-approach decision}. This fires the design phase over exactly these gaps.
- **Would write:** `.mochiko/features/FEAT-002/sufficiency-report.md` — store-consult result, the "no background workers" ruled-topology trip flagged for run-open, the `quickstart.md` null path noted (no real external-integration surface — C-001 single-process, no external services), no `[MODIFY]` amendment involved.
- Also surfaced here, per the absent-surfaces rule (never auto-resolved, never failing): `.claude/rules/mochiko/` is absent (no governance region) — surfaced, run proceeds; `.mochiko/memory/codebase-analysis.md` is absent and no application source tree was found in this workspace despite FEAT-001 reading "delivered" — surfaced as an open brownfield/greenfield question for the user, not resolved here.

## Phase 3 — Run-open confirmation (**user gate**)

One confirmation, no negotiation, presenting:
- Batch: FEAT-002 "Note search", selection scope, rows W1+W2, no epic, no delta card.
- Attempt bounds at their only redeclaration point: 3 verification attempts per cycle, 2 gap-rework rounds at final validation (defaults, unless the user redeclares now).
- The sufficiency verdict and its gap routing (Phase 2's four named gaps → design phase).
- Trips/conflicts for ruling: (a) the background-worker requirement contradicts the ruled "no background workers" architecture statement, (b) the commodity-category adopt-first call on the indexing approach is reserved to the user and will surface again at the design checkpoint, (c) the absent-governance-region and absent/inconclusive brownfield status.
- Done condition: all cycle cards checked, built test-first, independently verified per-cycle and whole, criteria traced, governance-aligned, acceptance landing executed whole, run closes accept/amend/reject.

**Gate — what's being confirmed:** does the user ratify this batch, scope, and bounds, and how do they rule the surfaced trips?
- **Ruling A — confirm as-is:** proceed to Phase 4 with default bounds, architecture trip provisionally deferred to the design checkpoint's sign-off, greenfield assumed with warning logged (no `/mochiko:setup` requested).
- **Ruling B — adjust bounds:** user raises/lowers `attempt_bound_cycle` and/or `gap_rework_bound`; recorded, carried through the rest of the run.
- **Ruling C — rules a trip now** (e.g., pre-approves the background-worker topology change, or requests `/mochiko:setup` first for the brownfield question): proceed to Phase 4 (or to `/mochiko:setup` first) with that ruling binding on the design phase.
- **Ruling D — declines to open the run:** run halts here; nothing downstream executes.

## Phase 4 — Design phase (fires — gaps named)

Seats, each working only on a plan I (DM) approve first, scoped to exactly the four named gaps:
- **`technical-analyst`** (design_seat): API contract delta (new `/notes/search` GET endpoint, query param, 200/400 responses) per `mochiko:patterns-api-contracts`; data-model delta (search/index representation) per `mochiko:patterns-entity-modeling`; the D-XXX technology decision for the indexing approach (e.g., SQLite FTS5, extending the existing D-001 SQLite choice, vs. a hand-rolled scan) per `mochiko:patterns-technical-decisions` + `mochiko:patterns-adopt-first` — the adopt-first ruling itself is called out again for the user, never decided silently by the seat.
- **`principal-architect`** (architect_seat): architecture-store delta — new AX-XXX row (background index worker), revised topology/flow, new NFR-XXX row (≤2s freshness), a C4-container delta diagram and a sequence diagram for the create→index→searchable flow, per `mochiko:patterns-system-design` and `mochiko:authoring-architecture-store`. This is the deviation from the ruled "no background workers" statement — carried as the store trip into the checkpoint.
- **`qa-engineer`** (qa_seat): design-time acceptance test cases fed into the cycle cards' `**TEST:**` gates (not the cards themselves yet).
- **Would write** (all as deltas beside baselines, never in place — floor `impl.baselines-never-in-place`): `.mochiko/features/FEAT-002/contracts-delta.md`, `.mochiko/features/FEAT-002/data-model-delta.md`, `.mochiko/features/FEAT-002/baseline-delta.md` (the D-XXX decision), `.mochiko/features/FEAT-002/architecture-delta.md` plus diagrams; an update to `entry.md` asserting design-implied dependencies/sharpened extent and filling the Architecture link once the store delta exists.
- If mid-build a builder later hits undesigned structure, this phase would re-fire scoped to that discovery (not planned further here — noted as a contingency).

## Phase 5 — Design-phase review pair (non-author, before checkpoint)

- **`mochiko:validator`** grades conformance to the gap list and artifact quality per `mochiko:review-plan-artifacts` — a blocking 3-state verdict (ready / needs-revision / critical-gaps).
- **`mochiko:tech-lead`** grades cross-artifact feasibility, contradiction, and buildability per `mochiko:review-feasibility`, including the architecture-store pass since a store delta exists — 3-state verdict (feasible / needs-revision / infeasible).
- Neither seat authored the Phase 4 outputs. If either verdict is not clean, Phase 4 seats revise scoped to the review feedback and this phase re-runs (loop) before the checkpoint is presented.
- Nothing written by this plan beyond the review verdicts feeding Phase 6.

## Phase 6 — Design checkpoint (**user gate**, floor)

Presented: the rendered C4-delta and sequence diagrams (or, absent a render surface, the source plus the changed-element table, recorded as such), the API/data-model/technical-decision deltas, both review verdicts, and the still-open adopt-first ruling on the indexing approach.

- **Ruling A — sign:** design and store delta become binding; the store delta's in-flight-class elements write now (the one legal non-landing store write); proceed to Phase 7.
- **Ruling B — request changes:** Phase 4 seats revise scoped to the feedback, Phase 5 re-reviews, re-presented here.
- **Ruling C — stop here:** run pauses; resumable later at the build stage per the rule permitting this.
- **Ruling D — reject the background-worker approach / reshape scope:** e.g., defer W2, build only W1 now; batch is re-scoped and Phase 3's contract is effectively amended before continuing.

## Phase 7 — Cycle-card authoring

- **Seat:** `qa-engineer` (design-class, never the builder; already owns `**TEST:**` authorship) authors cards in `tasks.md` per `mochiko:patterns-vertical-tdd` — foundation cycle before feature cycles: (1) search-index foundation/walking skeleton, (2) US-101 query + ranking + 400 validation (SC-101, SC-102), (3) US-102 background freshness (SC-103).
- Each card: stories/rationale, dependencies, acceptance-criteria IDs, a `**TEST:**` real-infrastructure gate, brownfield exposure (`[EXTEND]` on the existing api-service/notes-db, since it builds on FEAT-001's delivered surface) — no task lists, no file paths.
- On the zero-gap path this step would also carry the map-entry assertion, but gaps existed here so Phase 4 already carried it.
- **Would write:** `.mochiko/features/FEAT-002/tasks.md` (rendered from `plugins/mochiko/schemas/tasks.yaml` if `mochiko-cli template tasks` is unavailable).

## Phase 8 — Card review before confirm

- **Seat:** `mochiko:validator` (independent of the card author) reviews quality per `mochiko:review-plan-artifacts` and buildability by its own judgment. Feeds Phase 9.

## Phase 9 — Card confirm (**user gate**)

- **Ruling A — approve:** proceed to build as sliced.
- **Ruling B — re-slice:** request merge/split of cycles; `qa-engineer` revises, re-reviewed, re-presented.
- **Ruling C — reject/defer:** build does not start; cards stay unconfirmed.

## Phase 10 — Build (per cycle, foundation first)

- **Seat:** `staff-engineer` (builder_seat), never designs its own gaps. Per `mochiko:executing-tdd-cycle`: decomposes each card into concrete tasks (disclosed in the cycle report), applies `mochiko:brownfield-integration` on touches to the existing api-service/notes-db code, applies `mochiko:patterns-code-minimalism`'s pre-code ladder at decomposition (rungs disclosed), drives red→green→refactor test-first, on a DM-approved plan.
- Model-tiering note carried in every brief: locate/enumerate reads (e.g., locating the actual api-service source tree, which was not found during this planning pass and is flagged as an open item for the builder's opening brief) dispatch to a native Explore subagent at `model: haiku`; interpretive reads stay on session tier.
- **Would write:** application source/test changes (paths depend on the located codebase layout — undetermined at plan time) plus `.mochiko/features/FEAT-002/cycle-report.md` per cycle (decomposition, difficulties, deviations, `domain_deps_added`).
- Mid-cycle deviation gate (floor): any add/remove of a box or arrow beyond the signed delta halts that cycle and presents to the user — build as approved, or amend the delta first.

## Phase 11 — Per-cycle verification

- **Seat:** `qa-engineer` — never the implementer. Executes `**TEST:**` against real infrastructure per `mochiko:testing-end-user`, plus the `mochiko:review-code-minimalism` lens (advisory only) reading the diff, cycle report, and surrounding code.
- Attempt economy (floor): each grading round consumes one of 3 per-cycle attempts; two consecutive rounds with unchanged findings halts the cycle and presents state to the user.
- Finding routing: Minor → BACKLOG.md booking, never in-cycle fix; Important+ → blocks the cycle, batches to the checkpoint. Build-blocking questions interrupt immediately rather than waiting for the batch.
- Repeats across the foundation, W1, and W2 cycles until every card in `tasks.md` is `[x]`.

## Phase 12 — Gap-finding pass (fires — selection-scope run)

- **Seat:** a fresh `devils-advocate` (gap_finder_seat), dispatched blind and two-message per `mochiko:testing-gap-finding`: first message carries only `spec.md`, `sufficiency-report.md`, the design deltas, and the baseline NFR-XXX rows — never code, `tasks.md`, `**TEST:**` cases, or reports.
- Mutation lens (if run at high depth) executes on `qa-engineer` (already holds code sight), disclosed or explicitly stated as skipped.
- Findings split: spec-required behavior broken → evidence-backed, fails final validation; beyond-spec → advisory, disposed by the user (fix now / BACKLOG.md / accept as designed) at the checkpoint batch.

## Phase 13 — Final validation (once, whole build)

- Cold verification: snapshot the uncommitted working tree (`git ls-files -co --exclude-standard :!.claude/worktrees`) into `.claude/worktrees/mochiko-feat-002/`, run the full quality-gate suite from that snapshot. **Open item surfaced, not resolved here:** this workspace is reported as not a git repository, so this step cannot run as specified until that's addressed — flagged for the user rather than worked around.
- Regression sweep: re-run FEAT-001's durable gates (`.mochiko/features/FEAT-001/gates.md` — the three `**TEST:**` cases for create/restart persistence, empty-body 400, get/404), since FEAT-002 shares the api-service/notes-db seam.
- Quality gates: full repository suite, never severity-triaged; any failure fails the run.
- **Seat:** landing verification (`mochiko:validator` or `mochiko:tech-lead`, independent) checks the graded folds.
- Gap-rework bound: 2 rounds default; a finding localized to one cycle's territory instead charges that cycle's own remaining attempts; exhaustion or unchanged-findings round halts the run, presents state, user disposes.
- **Would write:** `.mochiko/features/FEAT-002/final-validation-report.md`.

## Phase 14 — Acceptance landing (executed whole, at acceptance)

- Store landing: the architecture delta's elements flip built, FEAT-002 keys clear, As-built:/Drift: fields written and independently graded, orphan check run, `ARCHITECTURE.md` regenerated by the store skill.
- Map landing (selection scope): W1/W2 fold into FEAT-002's extent, vanish from the work-row list; status → `delivered` (dated 2026-08-27); `FEATURES.md` index line updates; note-search spec reads closed since both selected rows folded.
- Gates fold: fix-now/backlog gap findings fold into `.mochiko/features/FEAT-002/gates.md` (minted), authored by `qa-engineer` in `**TEST:**` grammar.
- Baseline folds: `contracts/api.yaml`, `data-model.md`, `constraints-and-decisions.md` each fold exactly once via a graded three-way diff.
- KM landing: skipped — no `.mochiko/memory/knowledge-management.md` exists.

## Phase 15 — Final acceptance (**user gate**, floor)

Presented: verdict against the done condition, full evidence bundle (cycle reports, verification reports, final-validation report, built-vs-signed diff, regression results, gap-finding dispositions), rounds-consumed/seats-spawned summary.

- **Ruling A — accept:** Phase 14's landing finalizes; run closes PASS against the done condition (assuming none of the 15 fail-conditions stand).
- **Ruling B — amend:** user specifies changes; DM routes the amendment back to the appropriate earlier phase (design, cards, or build) scoped to the amendment, then returns here.
- **Ruling C — reject:** run closes as not-done; landing does not execute; disposition (revert to an earlier gate, or end the run) is the user's.

## Phase 16 — Close

- DM issues the final verdict citing the done condition explicitly against all 15 fail-condition clauses (each confirmed not standing, or the run is FAIL naming which stand), with the full rounds/seats-spawned tally surfaced.