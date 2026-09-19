# Action Plan — `/mochiko:implement FEAT-002` (plan-only, not executed)

## Grounding read already performed
`plugins/mochiko/schemas/implement.yaml`, `plugins/mochiko/schemas/common.yaml`, and `plugins/mochiko/schemas/command-labels.yaml` were read raw and whole (the run's mandatory first action). Supporting reads: `FEATURES.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/features/FEAT-001/gates.md`. No `CLAUDE.md`, no `.claude/rules/mochiko/`, no `.mochiko/memory/knowledge-management.md`, no `.gitignore`, and the workspace is not a git repository.

Resolved run shape: `FEAT-002` = capability "Note search" (not an `EPIC-XXX`) → **scope: selection**. Work rows W1, W2 carry ratified scope from the spec's accepted selection (2026-08-26). Dependency FEAT-001 is `delivered`, so nothing blocks. `governance_region`: absent. `km_file`: absent. `baseline`: present (spine.md carries ruled AX-001..003).

---

## Phase 1 — Entry resolution & scope typing
**Does:** Confirms FEAT-002 resolves via the map, not an epic; confirms both selected rows' dependency (FEAT-001) is already delivered so nothing blocks the batch.
**Reads:** `FEATURES.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/specs/note-search/spec.md`.
**Writes:** none.
**Gate:** none yet.

## Phase 2 — Sufficiency check (entry)
**Does:** Runs the ten-clause sufficiency check per selected row (W1, W2) against spec.md, the architecture store, and product baselines, per `mochiko:review-sufficiency`, by a seat that authored none of those sources (e.g. `mochiko:validator` or `mochiko:tech-lead` — staffing is my call).
**Reads:** spec.md, `spine.md`, `constraints-and-decisions.md`, `data-model.md`, `contracts/api.yaml`, and a check for `quickstart.md` (absent).
**Concrete findings this batch surfaces:**
- **Structural gap:** FR-103 requires a *background index worker* rebuilding the index off the request path; `spine.md` currently rules "Synchronous request/response only; no queues, no background workers" and lists only `api-service` + `notes-db`. This is a spec-vs-ruled-architecture conflict → names a store-delta gap (new element + `AX-XXX` row + NFR target for the ≤2s freshness bound).
- **Contract gap:** `contracts/api.yaml` has no `/notes/search` path at all → names an API-contract gap for both W1 and W2.
- **Possible gap:** whether the indexing approach is a technology decision (`D-XXX`) worth an adopt-first pass (e.g., stdlib substring scan vs. an indexing library) — flagged, not resolved, by the grader.
- **Absent-surface notes (never auto-resolved, never fail):** no `quickstart.md` (null path, recorded); no governance region in `CLAUDE.md` (surfaced, not a gap).
**Writes:** `.mochiko/features/FEAT-002/sufficiency-report.md` — per-row verdict, the store-consult result, the trips for the user, the quickstart null path.
**Gate:** none directly; verdict is binding input to Phase 3.

## Phase 3 — Run-open confirmation (USER GATE, the entry gate)
**Does:** One blocking confirmation (plain text, never timed): names the batch (FEAT-002, scope: selection), restates attempt bounds at their only redeclaration point (`attempt_bound_cycle=3` per cycle, `gap_rework_bound=2` at run scope — defaults unless the user redeclares now), presents the sufficiency verdict and its gap list/trips for ruling, states the done condition.
**Reads:** the sufficiency report just written.
**Writes:** none (ruling recorded in the report/run record).
**What is confirmed:** whether to proceed with the batch as scoped; attempt bounds as default or redeclared; disposition of each named trip (architecture-delta gap, contract gap, possible D-XXX flag).
**Onward branches:**
- *Confirms as presented* → proceed to Phase 4 (design phase fires, since gaps exist — this isn't optional once gaps are named).
- *Disputes a gap* (e.g., insists indexing can stay synchronous, contradicting FR-103) → that's a spec-level disagreement, not something this run can silently redesign around; the branch is to pause and recommend a spec amendment via `/mochiko:feature` or `/mochiko:specify` rather than proceeding.
- *Declines to proceed at all* → run halts at entry; nothing written beyond the sufficiency report; resumable later.

## Phase 4 — Design phase (fires: gaps were named)
**Does:** Authors exactly the named gaps, nothing more, each on a plan I approve first, rung-justified per `mochiko:patterns-plan-minimalism`.
**Seats:** `mochiko:principal-architect` (architecture store delta: new background-index-worker element, new `AX-XXX` row, NFR target, C4-container delta diagram + sequence diagram for the async refresh flow); `mochiko:technical-analyst` (API contract delta for `GET /notes/search`; a `constraints-and-decisions.md` delta / `D-XXX` if the indexing approach is ruled a technology decision — any commodity-category adopt-first call here is reserved to the user, not the design seat); `mochiko:qa-engineer` (design-time `**TEST:**` cases for W1/W2).
**Reads:** sufficiency-report.md, `.mochiko/product/*` baselines, spec.md's cited SC-101/102/103.
**Writes:** `.mochiko/features/FEAT-002/contracts/api.yaml` (delta), an architecture-delta package (diagram + changed-`AX-XXX` table) per `mochiko:authoring-architecture-store`, possibly `constraints-and-decisions-delta.md`, and an update to `entry.md`'s Architecture link + provenance.
**Review (non-author, before checkpoint):** `mochiko:review-plan-artifacts` (conformance to the gap list, blocking on material divergence) and `mochiko:review-feasibility` (buildability/contradiction, including the architecture pass since a store delta exists); `mochiko:tech-lead` independently grades the store delta's judgment content.
**Gate:** none yet — feeds Phase 5.

## Phase 5 — Design checkpoint (USER GATE)
**Does:** Presents the rendered diagram plus the changed-`AX-XXX` row table (or source + table if no render surface), the API contract delta, any `D-XXX` delta, the updated map assertion, and both review verdicts.
**What is confirmed:** sign the design and store delta as drafted, or amend; rule any adopt-first/commodity flag surfaced by the design seats (reserved to the user); may stop here and resume later.
**Onward branches:**
- *Signs as drafted* → proceed to Phase 6; the signed delta stands beside the ruled baseline in-flight until landing.
- *Signs with amendment* → design seats revise before the checkpoint closes.
- *Rejects / stops* → run pauses cleanly; nothing built; resumable from this checkpoint later.

## Phase 6 — Cycle-card authoring
**Does:** A design-class seat (e.g. `mochiko:technical-analyst`, never `mochiko:staff-engineer`) slices the signed design into cycle cards per `mochiko:patterns-vertical-tdd` (walking-skeleton-first). Plausible slicing for this batch: Cycle 1 — minimal `GET /notes/search` wired end-to-end (walking skeleton); Cycle 2 — full query correctness closing W1 (FR-101/102, SC-101/102); Cycle 3 — background index worker + freshness bound closing W2 (FR-103, SC-103), flagged `[EXTEND]` against FEAT-001's create path. `mochiko:qa-engineer` authors each card's `**TEST:**` gate.
**Writes:** `.mochiko/features/FEAT-002/tasks.md`.
**Review before confirm:** `mochiko:qa-engineer` (independent of the card author) grades quality (`mochiko:review-plan-artifacts`) and buildability — blocking.
**Gate:** none yet — feeds Phase 7.

## Phase 7 — Card confirm (USER GATE)
**What is confirmed:** the cycle slicing itself, before any build starts.
**Onward branches:**
- *Confirmed* → Phase 8 begins.
- *Amend slicing* (merge/split/reorder) → card author revises, re-reviews, re-presents.
- *Reject wholesale* → escalates back toward Phase 4 design, or further back to spec amendment if the disagreement is about story boundaries.

## Phase 8 — Build: test-first cycles, foundation before feature
**Does:** For each confirmed cycle in order: `mochiko:staff-engineer` builds test-first (red→green→refactor) on an approved plan, never designing its own gaps; `mochiko:brownfield-integration` binds for the `[EXTEND]` touch to FEAT-001's create path (read the whole file first, preserve its interface); `mochiko:patterns-code-minimalism` runs at decomposition before any red-phase test.
**Writes:** code changes (paths are the builder's call at decomposition — likely a search module, an index-worker module, route wiring); `cycle-report.md` per cycle in `.mochiko/features/FEAT-002/` (disclosed decomposition, deviations, `domain_deps_added`).
**Per-cycle verification (independent seat, e.g. `mochiko:qa-engineer`):** `mochiko:testing-end-user` runs the `**TEST:**` gate against real infrastructure (live SQLite + running server, no mocks); `mochiko:review-code-minimalism` lens (advisory, never gate-failing); full repository quality-gate suite (a failure fails the cycle, never triaged). Each grading pass consumes one of 3 attempts/cycle; two consecutive unchanged-finding rounds triggers a no-progress stop presented to the user.
**Deviation floor:** if a cycle needs a structural change beyond the signed delta (e.g. an unforeseen queue element), that cycle halts and is presented — build as approved or amend the delta first, never silently redesigned.
**Writes:** verification report per cycle; `tasks.md` checkbox flips on green.
**Gate (cycle checkpoint):** escalations/advisory findings batch here; only a build-blocking question interrupts mid-cycle.

## Phase 9 — Final validation
**Does:**
- **Regression sweep:** re-runs FEAT-001's durable gate set (`.mochiko/features/FEAT-001/gates.md`, 3 `**TEST:**` cases) since FEAT-002 sits in FEAT-001's territory and now owns the create→search seam; a failure here fails the whole run.
- **Cold verification:** snapshot the uncommitted working tree via `git ls-files -co --exclude-standard :!.claude/worktrees` into `.claude/worktrees/mochiko-<purpose>/`, confirm the `/.claude/worktrees` ignore entry first, run the full quality-gate suite cold. **Concrete blocker for this run:** the workspace is not a git repository and has no `.gitignore` — this sub-step cannot execute as specified until a git repo exists. This would be surfaced to the user as a run-blocking prerequisite (offer `git init`) rather than silently skipped or worked around.
- **Gap-finding pass** (fires — selection scope): a fresh `mochiko:devils-advocate`, dispatched blind, two-message per `mochiko:testing-gap-finding` — first message carries only spec.md, sufficiency-report.md, the design deltas, and the baselines' NFR rows, never code/tasks.md/TEST cases/reports.
**Writes:** `.mochiko/features/FEAT-002/final-validation-report.md` (states the gap-finding pass ran, and cold-verification status/blocker).
**Gate:** findings route per kind — spec-required breakage fails validation; beyond-spec is advisory and its disposition (fix now / backlog / accept) is reserved to the user. Gap-rework bound: 2 rounds at run scope, or charged to a cycle's remaining attempts if localized.

## Phase 10 — Landing (executed whole, only at acceptance)
**Does (described here as what Phase 11's "accept" branch would write):** store landing — the signed delta's elements flip to built, `FEAT-002` keys clear, `As-built:`/`Drift:` fields written and independently graded, orphan check runs, derived `ARCHITECTURE.md` regenerated; graded three-way-diff folds for `contracts/api.yaml` and any `constraints-and-decisions.md` delta; selection-scope graduation — W1/W2 fold into FEAT-002's extent, entry status → `delivered` (dated), `FEATURES.md` index line updates, the note-search spec closes (both rows folded); any fix-now/backlog gap findings fold into a newly-minted `.mochiko/features/FEAT-002/gates.md` in `**TEST:**` grammar.
**Writes:** `entry.md`, `FEATURES.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/architecture/spine.md` + `ARCHITECTURE.md`, possibly `.mochiko/product/constraints-and-decisions.md`, `.mochiko/features/FEAT-002/gates.md`.
**Gate:** the landing verification seat checks the graded folds match the diff (part of Phase 11's evidence).

## Phase 11 — Final acceptance (USER GATE, closes the run)
**Does:** presents the done-condition verdict — all cards `[x]`, per-cycle and whole real-infrastructure verification, code traced to FR-101/102/103 and SC-101/102/103, rounds/seats consumed at each checkpoint, and a verdict against the 15-item Not-done fail set (none of: unrecorded sufficiency, skipped/unsigned design, card independence violation, unchecked card, failing gate, unevidenced verification, regression, in-place baseline edit, unresolved deviation, incomplete store landing, ungraded fold, missing/unstated gap-finding, unresolved spec-gap finding, no acceptance).
**What is confirmed:** accept / amend / reject, plain blocking text.
**Onward branches:**
- *Accept* → Phase 10's landing executes now, whole; run closes with an accept verdict.
- *Amend* → in-scope amendments loop back into the relevant open cycle/checkpoint (charging remaining attempts); out-of-scope amendments stay FAIL unless the user explicitly accepts the widened scope, which would reopen sufficiency/design for the new slice.
- *Reject* → run closes without landing (`impl.fail.no-acceptance` — Not-done); all reports persist as record for a future attempt.