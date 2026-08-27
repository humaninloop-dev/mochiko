## Pre-flight (already performed)

Read `plugins/mochiko/schemas/implement.yaml` raw, in full (39 KB, 6 sections). Confirmed the fail-condition set is exactly **15** rules (`impl.sec.fail-conditions`) — matches the command's hard-coded Not-done count, so no halt-and-surface is needed on that check. Also read `command-labels.yaml` (label registry), and surveyed the working tree: `FEATURES.md`, `ARCHITECTURE.md`, `.mochiko/features/FEAT-001/{entry.md,gates.md}`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/specs/{note-capture,note-search}/spec.md`, `.mochiko/product/{constraints-and-decisions.md,data-model.md,architecture/spine.md,contracts/api.yaml}`. Checked for `.claude/rules/mochiko/` (absent), `.mochiko/memory/` (absent — no `codebase-analysis.md`, no `knowledge-management.md`), and git status (not a git repo).

No `` was supplied in the invocation.

---

## Phase 1 — Entry & scope resolution

**Does:** No target named → propose the next ready capability from the map (`impl` "empty → propose... confirm with the user").
**Reads:** `FEATURES.md` (FEAT-001 `delivered`, FEAT-002 `selected`), `.mochiko/features/FEAT-002/entry.md`.
**Finding:** FEAT-002 "Note search" carries two ratified work rows, selection scope, spec accepted 2026-08-26: W1 (search by query, US-101, SC-101/SC-102) and W2 (index freshness, US-102, SC-103). Dependency on FEAT-001 is satisfied (`delivered`) — nothing blocks.
**Proposal to user (not yet the run-open gate):** "Take up FEAT-002 as this run's batch?" — informal, precedes the formal gate in Phase 3.
**Writes:** none yet.
**Seats:** none spawned yet (DM-only reasoning).

## Phase 2 — Sufficiency check

**Does:** Runs the ten-clause check per selected row (W1, W2) against spec, store, and baselines, per `mochiko:review-sufficiency`.
**Seat:** a seat that authored none of spec.md, the architecture store, or the product baselines, and will not design/build this batch — propose **mochiko:validator** (checklist-grading, never grades own work).
**Reads:** `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/data-model.md`, `.mochiko/product/constraints-and-decisions.md`.
**Concrete gaps this grading would surface:**
- `contracts/api.yaml` has no `/notes/search` endpoint at all — W1 gap.
- `spine.md` states flatly *"Synchronous request/response only; no queues, no background workers"*, while spec FR-103 requires *"a background index worker rebuilds the index off the request path"* — a direct conflict, not just an absence. This is a **trip**, not a plain gap.
- `data-model.md` has no representation of a search index or ranking mechanism.
- `constraints-and-decisions.md` has no technology decision for full-text search (commodity category → adopt-first applies: SQLite FTS5 vs. hand-rolled).
- No NFR-XXX concern row anywhere carries the SC-103 ≤2s latency target.
- Possible `[MODIFY]` amendment against FEAT-001's entry if the chosen index approach extends the SQLite schema FEAT-001 built.
**Writes:** `.mochiko/features/FEAT-002/sufficiency-report.md` — per-row verdict, store-consult result, the architecture trip flagged for user ruling, quickstart.md null-path note (no external integration surface), the possible `[MODIFY]` flag.
**Note:** absent `.mochiko/memory/codebase-analysis.md` and absent governance region are surfaced here too (never auto-resolved, never run-failing) — offered as informational only since this is greenfield-shaped work.

## Phase 3 — Run-open confirmation (the entry gate)

**Gate — what is confirmed, in one blocking, non-timed message:**
- Batch: FEAT-002 "Note search", selection scope, rows W1+W2, ratified 2026-08-26.
- Attempt bounds restated at their only redeclaration point: 3 verification attempts per cycle, 2 gap-rework rounds at final validation (defaults from `vars:`) — offer to change now or carry defaults.
- Sufficiency verdict and gap routing: API-contract gap, data-model gap, missing NFR row, missing search-tech decision — all route to an in-run design phase scoped to exactly these.
- The trip: FR-103's required background worker collides with the ruled "no queues, no background workers" architecture statement — presented as a constraint-challenge for the user to rule on now (amend the ruled statement vs. reconsider the requirement), not silently designed around.
- Done condition stated: both cards' cycles `[x]`, test-first, per-cycle + whole-build real-infrastructure verification, code traces to FR-101/102/103 and meets SC-101/102/103, landing executes whole, run closes at final acceptance.

**Onward branches:**
- **Approve as framed, rule the trip "amend the architecture" (allow an async index worker):** proceed to Phase 4 with the store delta explicitly permitted to add an async element/flow.
- **Approve as framed, rule the trip "keep architecture as ruled" (no background workers):** design phase is re-scoped — FR-103 must be satisfied synchronously (e.g., reindex inline on write within budget) or the requirement itself is escalated back to spec as a change; still proceeds to Phase 4 but with a different design constraint.
- **Defer the trip on the record:** design phase proceeds on every other gap; the trip stays open and blocks only the store-delta portion of the design checkpoint.
- **Reject the batch / send back:** if the user decides FEAT-002 isn't ready or wants different scope, this run does not open on it — route to `/mochiko:feature` (delta) or back to spec rework; no code is written, run ends here without a FAIL verdict (it never opened).

Plan continues assuming the first branch (approve, allow the architecture amendment) since that's the least-constrained path to trace fully.

## Phase 4 — Design phase (fires — gaps were named)

**Seats (proposed staffing, DM's call under `impl.staffing-latitude`):**
- **principal-architect**: architecture-store delta — new/modified element for the index worker, changed AX-XXX rows, NFR-XXX row for the 2s bound, C4-container delta diagram + register per `mochiko:patterns-system-design`.
- **technical-analyst**: `contracts/api.yaml` delta (`GET /notes/search?q=`, 200 ranked list, 400 for missing/short `q`), `data-model.md` delta (index representation), `constraints-and-decisions.md` delta (new D-XXX for FTS5-vs-custom, adopt-first analysis).
- **qa-engineer**: design-time `**TEST:**` acceptance cases tied to SC-101/102/103.

Each works only on a plan I (DM) approve first; each authors *exactly* the named gaps, rung-justified per `mochiko:patterns-plan-minimalism` (`impl.design-gaps-only`).

**Mid-phase user checkpoint (conditional gate):** the adopt-first call (SQLite FTS5 vs. hand-rolled index) is never the design seat's or builder's to make — halts to the user. **Branches:** adopt FTS5 → constrains the D-XXX text and the architecture element's implementation note; require custom → same, with the custom rationale recorded against the adopt-first check; unresolved → design phase cannot finalize that D-XXX row and the checkpoint (Phase 6) surfaces it as still-open.

**Writes:** deltas beside baselines at `.mochiko/features/FEAT-002/` (data-model delta, contracts delta, constraints-and-decisions delta in before/after appliable form) plus the architecture-store delta (in-flight class elements — the one legal in-place carve); design-implied dependencies/extent asserted onto FEAT-002's `entry.md` with provenance and the architecture link filled; if warranted, a `[MODIFY]` delta written onto FEAT-001's `entry.md`.

## Phase 5 — Design-phase review pair (independent, before checkpoint)

- **mochiko:review-plan-artifacts** conformance/card-quality (blocking) — propose **qa-engineer** (continuity with the verification-seat identity that will also do card review and per-cycle grading, per `impl.verification-design-time-grades`).
- **mochiko:review-feasibility** buildability/contradiction + the architecture pass (a store delta exists) — propose **mochiko:tech-lead** (its stated specialty: cross-artifact feasibility and independently grading architecture-store judgment writes).

**Writes:** review findings appended to `sufficiency-report.md`'s successor artifacts or a dedicated review note under `.mochiko/features/FEAT-002/`, per `templates/report-format.md`.

## Phase 6 — Design checkpoint (user gate, floor)

**Confirmed:** the rendered C4-container delta diagram (or source + changed-AX-XXX table if no render surface) showing the new/changed architecture element(s) for indexing, the API contract delta, data-model delta, the new D-XXX row, and the design-time `**TEST:**` case set.

**Branches:**
- **Sign as presented:** design + store delta signed; store write for the in-flight elements is now legal (the one baseline carve); proceed to Phase 7.
- **Amend:** user redirects specifics (e.g., different worker cadence, reject FTS5) → Phase 4 iterates on the amended scope only, re-reviewed (Phase 5), re-presented.
- **Stop here:** user may halt at the checkpoint and resume the build later — run pauses with the signed (or partially signed) design as the resume point; nothing below Phase 6 executes this session.

## Phase 7 — Card authoring

**Seat:** a design-class, non-builder seat — propose **technical-analyst** (continuity with the design authorship), with **qa-engineer** authoring the `**TEST:**` cases within its slicing (`impl.seat-card-author-independence`).
**Reads:** `plugins/mochiko/schemas/tasks.yaml` (raw, since no `mochiko-cli` binary is evidenced in this tree), the signed design deltas.
**Slicing (foundation before feature, per `mochiko:patterns-vertical-tdd`):**
1. *Foundation* — background index-worker infrastructure wired to `notes-db` (brownfield `[EXTEND]` on FEAT-001's SQLite layer); no user-facing behavior; `**TEST:**` verifies the worker runs and indexes a seeded note.
2. Search-by-query endpoint returns ranked matches (US-101, FR-101, SC-101); `**TEST:**` real HTTP call against real SQLite.
3. Missing/short `q` → 400 (FR-102, edge case); `**TEST:**` real HTTP call.
4. Create→searchable within 2s round trip (US-102, FR-103, SC-103); `**TEST:**` timed real round trip.
**Writes:** `.mochiko/features/FEAT-002/tasks.md` — cards with stories/rationale, dependencies, acceptance-criteria IDs, `**TEST:**` gate, brownfield exposure; no task lists or file paths.

## Phase 8 — Card review (independent, before confirm)

**Seat:** qa-engineer (verification-class, not the card author) grades quality (`review-plan-artifacts`) and buildability.
**Writes:** review note appended per report-format.

## Phase 9 — Card confirm (user gate, floor)

**Confirmed:** the 4-cycle slicing and its dependency order above.
**Branches:**
- **Approve:** proceed to build.
- **Amend** (merge/split/reorder cycles): cards revised, re-presented before any build starts.

## Phase 10 — Build (per cycle, foundation → feature order)

**Builder:** staff-engineer, per `mochiko:executing-tdd-cycle` — decomposes each card into concrete tasks (disclosed in `cycle-report.md`), builds test-first, applies `mochiko:brownfield-integration` on the `notes-db` extension, applies `mochiko:patterns-code-minimalism` at decomposition (rungs disclosed). Works only on a plan I approved.
**Reads per cycle:** the signed design deltas, `sufficiency-report.md`, the relevant baseline files; since no `.claude/rules/mochiko/` exists, no governance-rules read is obligated (surfaced as absent, not failed).
**Writes per cycle:** code changes; `.mochiko/features/FEAT-002/cycle-report-N.md` (decomposition, difficulties, deviations, `domain_deps_added`); flips the card's checkbox in `tasks.md` on pass.
**Verification seat:** qa-engineer — runs the `**TEST:**` gate against real infrastructure (real SQLite file, real HTTP server, real requests), runs the full quality-gate suite, applies the `review-code-minimalism` lens (advisory) reading the diff + report + surrounding code. Each grading pass consumes one of the 3 per-cycle attempts.
**Conditional gates within this phase:**
- **No-progress stop** (floor): two consecutive rounds with unchanged findings → halt cycle, present state to user. Branches: user grants an exemption round (only the user can exempt an attempt from the count) or accepts the halt and rules disposition.
- **Deviation gate** (floor): if the builder needs to add/remove a box or arrow, or move a responsibility across a boundary beyond the signed delta — stop, present to user. Branches: **build as approved** (revert to the signed shape) or **amend the delta first** (mini-loop back through Phases 4–6 scoped to the discovery, per `impl.midrun-refire`).
- **Undesigned structure discovered mid-build:** same re-fire path as above.

Repeat for cycles 1→4; cycle 1 (foundation) must complete and pass before 2–4 begin.

## Phase 11 — Final validation (whole-build)

**Cold verification:** build/test from a dependency-cold snapshot of the uncommitted working state. **Blocker to flag concretely:** this working directory is not a git repository, so `git ls-files -co --exclude-standard` cannot run and the `.claude/worktrees` ignore-entry precondition doesn't apply yet — this would be surfaced to the user as a prerequisite (e.g., "initialize git, or confirm an alternative snapshot method") rather than silently skipped, since claimed verification without real evidence is fail-condition `impl.fail.no-evidence`.
**Regression sweep:** re-runs FEAT-001's durable gates at `.mochiko/features/FEAT-001/gates.md` (restart-survival, empty-body 400, 404-on-random-id) plus its card cases, since FEAT-002 reads the store FEAT-001 built.
**Gap-finding pass** (fires — selection scope): fresh **devils-advocate**, blind two-message dispatch. First message: `spec.md`, `sufficiency-report.md`, the signed design deltas (contracts delta, data-model delta, architecture delta, NFR rows) — never code, `tasks.md`, `**TEST:**` cases, or reports. Seat states derived expectations, then black-box probes the running service. Mutation lens runs on qa-engineer if operating at high depth (or a disclosed skip).
**Findings routing:** spec-required behavior broken → fails final validation (evidence + clause cited); beyond-spec findings → advisory, user disposition (fix now / BACKLOG.md / accept-as-designed), batched at the checkpoint; anything localizing to FEAT-001's territory routes to a `/mochiko:feature` delta card instead of being reworked here.
**Writes:** `.mochiko/features/FEAT-002/final-validation-report.md`.

## Phase 12 — Landing (executed whole, at acceptance)

**Store landing** (`mochiko:authoring-architecture-store`): signed elements flip to built, FEAT-002 key clears; As-built:/Drift: fields written as judgment by the landing verification seat and independently graded (propose **tech-lead**, matching its store-grading specialty); orphan check runs; `ARCHITECTURE.md` regenerated (never hand-edited).
**Graded folds** (one three-way diff each, checked by the landing verification seat — propose qa-engineer): `contracts/api.yaml`, `data-model.md`, `constraints-and-decisions.md`.
**Map graduation:** W1/W2 fold into FEAT-002's extent lines and vanish from work rows; entry status → `delivered` (dated 2026-08-27); `FEATURES.md` index line updates; note-search spec reads closed. Any FEAT-001 `[MODIFY]` delta folds too.
**Gates fold:** `.mochiko/features/FEAT-002/gates.md` minted; qa-engineer authors accepted fix-now/backlog findings in `**TEST:**` grammar.
**KM landing:** `.mochiko/memory/knowledge-management.md` is absent, so this obligation is a no-op (would be re-checked live, not assumed).
**Git:** commits are suggested only, never run — and since no repo exists yet, that suggestion would explicitly include "run `git init`" as a first step, surfaced rather than silently worked around.

## Phase 13 — Final acceptance (user gate, floor)

**Confirmed:** full run summary — cycles built/verified with evidence, quality gates green, regression sweep clean, gap-finding results and their dispositions, the landing diff (store built-vs-signed, baseline folds, map graduation), rounds/seats consumed vs. the 3/2 bounds.
**Branches:**
- **Accept:** run closes with a PASS verdict against the done condition (Phase 14).
- **Amend:** user specifies a targeted change (e.g., different disposition on a beyond-spec finding) → bounded rework loop, re-presented.
- **Reject:** run closes without acceptance — `impl.fail.no-acceptance` stands, so the run is Not-done/FAIL; all artifacts (cards, reports, deltas) persist for a future resumed run rather than being discarded.

## Phase 14 — Close

DM closes with an explicit verdict against the done condition, checked against all 15 fail-conditions individually (sufficiency recorded, design signed, no card built pre-confirm or unchecked, gates green, evidence real, no regression, no baseline edited in place, no unresolved deviation, store landing complete, every fold graded, gap-finding present, no unstated skip, no unresolved spec-gap finding, acceptance given) — reports total rounds/seats consumed across the run.