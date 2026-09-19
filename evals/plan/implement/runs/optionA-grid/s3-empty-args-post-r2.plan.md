# Action Plan — `/mochiko:implement` (this invocation)

**Invocation:** no `$ARGUMENTS` given (empty). Per the command's Entry step, an empty argument means: propose the next ready capability from the map and confirm with the user. This is not an epic run (no `EPIC-XXX` named).

This is a plan-only pass: reads below have already been performed (reading is permitted); no agent was spawned, no file was written, and no user gate was actually resolved. Each gate below is described as what would be confirmed, with the onward branch for every possible ruling.

---

## Phase 0 — Load the run's rule surface (already performed, first action)

- **Read, raw and whole:** `plugins/mochiko/schemas/implement.yaml`, `plugins/mochiko/schemas/common.yaml`, `plugins/mochiko/schemas/command-labels.yaml`.
- **Result:** `vars` resolved — `attempt_bound_cycle=3`, `gap_rework_bound=2`, `builder_seat=staff-engineer`, `design_seat=technical-analyst`, `architect_seat=principal-architect`, `qa_seat=qa-engineer`, `gap_finder_seat=devils-advocate`, `explore_model=haiku`, `features_dir=.mochiko/features`, `product_dir=.mochiko/product`, `epics_dir=.mochiko/epics`, `rules_dir=.claude/rules/mochiko`, `tasks_schema=plugins/mochiko/schemas/tasks.yaml`.
- **Self-check:** counted the rules under `impl.sec.fail-conditions` — 15, matching the command `.md`'s hard-coded count. No halt needed.
- **Writes:** none.

## Phase 1 — Entry & scope resolution

- **Read:** `.mochiko/features/FEAT-001/entry.md`, `.mochiko/features/FEAT-002/entry.md`. (No `FEATURES.md` index file exists on disk — an operational gap in the map surface, worth surfacing to the user alongside the entry-gate confirmation, though it doesn't block resolving a candidate from the entry files themselves.)
- **Finding:** FEAT-001 (note capture) is `delivered`. FEAT-002 (note search) is `selected`, carries two ratified, unchecked work rows (W1 — search by query, W2 — index freshness), selection source is the spec's accepted selection (ratified 2026-08-26). Its one dependency, FEAT-001, is already `delivered`, so the row is not blocked.
- **Decision:** propose **FEAT-002** as the capability-batch, scope type **selection scope**. This proposal is itself something the command requires confirming with the user — folded here into the Phase 3 run-open confirmation rather than a separate round-trip.
- **Model tiering note:** this enumeration was small enough for the session tier directly; a real run's larger map sweep (or later, locating the actual source tree) would route through a native Explore subagent at `model: haiku` per `mochiko:patterns-model-tiering`.
- **Writes:** none.

## Phase 2 — Sufficiency check (per `mochiko:review-sufficiency`)

- **Seat:** an independent grading seat, staffed at the DM's judgment (`impl.staffing-latitude`), that authored none of the graded sources and will not design or build this batch — e.g. `mochiko:validator` or `mochiko:devils-advocate`.
- **Reads (by the seat, real, not mine to substitute):** `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/contracts/api.yaml` — the ten-clause check per selected row (W1, W2).
- **What I can already see in the artifacts that bears on this grading** (not a substitute for the seat's own verdict): FR-103 requires "a background index worker" rebuilding the index off the request path; the architecture spine states "Synchronous request/response only; no queues, no background workers" and carries no AX row for an index worker. `contracts/api.yaml` has no `/notes/search` path. No concern row captures the SC-103 freshness/latency target. FEAT-002's own entry notes "no store delta exists for this capability." These make a gap verdict likely, but the binding call is the grading seat's.
- **Absent-surface handling (surfaced, never auto-resolved, never run-failing):** `.claude/rules/mochiko` (governance region) does not exist — surfaced as absent. `.mochiko/memory/codebase-analysis.md` does not exist, and in fact no `.mochiko/memory/` directory exists at all, despite FEAT-001 showing `delivered` — this inconsistency (a delivered feature with no discoverable codebase-analysis trail) is a trip worth naming explicitly at run-open rather than silently assumed either way.
- **Writes (would be):** `.mochiko/features/FEAT-002/sufficiency-report.md` — the store-consult result, any no-delta claim, the trips for run-open, the `quickstart.md` null-path note (no real external-integration surface here), and no `[MODIFY]` amendment (FEAT-002 hasn't shipped yet).

## Phase 3 — Run-open confirmation (**user gate**)

One confirmation, no negotiation. What's presented:

- Batch and scope type: FEAT-002 / note search, selection scope, rows W1 + W2.
- Both attempt bounds at their only redeclaration point: 3 per-cycle grading attempts, 2 gap-rework rounds at final validation — unless the user changes them here.
- The sufficiency verdict, its gap routing, and every trip/conflict for ruling: the likely architecture-store gap (background worker vs. the ruled synchronous-only topology), the missing `/notes/search` contract entry, the missing freshness NFR/concern row, the absent governance region, and the absent-memory/delivered-feature inconsistency.
- The done condition, stated plainly.

**Branches:**
- *Confirm as presented* → proceed to Phase 4 if any gap was named (expected here), else skip straight to Phase 5.
- *Adjust attempt bounds* → new values carry through the rest of the run; no further redeclaration point exists.
- *Rule on a trip* (e.g., "proceed without governance region," or "run `/mochiko:setup` first before trusting brownfield state") → downstream path adjusts accordingly; a `/mochiko:setup` ruling would pause this run to let that command run first.
- *Reject the proposed batch* (wants a different capability) → this run does not open on FEAT-002; re-route per the Entry step to another capability, or to `/mochiko:specify` / `/mochiko:feature` if no ratified scope exists elsewhere.

## Phase 4 — Design phase (fires only if Phase 2/3 named gaps — expected)

- **Seats:** `technical-analyst` (API contract delta for `/notes/search`, any data-model/NFR addition), `principal-architect` (store delta: models the index-worker component/arrow, resolves the contradiction with "no background workers"), `qa-engineer` (design-time **TEST:** cases feeding the cycle cards). Each plans first; DM approves the plan before any seat produces. Each authors exactly the named gaps, nothing more, rung-justified per `mochiko:patterns-plan-minimalism`.
- **Deviation-gate note:** introducing a background worker is a structural change against the spine's currently-ruled "no background workers" statement — this must be explicitly presented and ruled at the checkpoint below, never designed around silently.
- **Adopt-first note:** if the worker design considers any off-the-shelf scheduler, the adopt-first call is reserved to the user, not the design seat.
- **Independent review pair (non-author):** `mochiko:review-plan-artifacts` (conformance to the gap list — blocking) and `mochiko:review-feasibility` (buildability/contradiction, including whether the addition stays inside C-001's single-process constraint).
- **Writes (would be):** deltas beside baselines in `.mochiko/features/FEAT-002/` (contract delta, any data-model/constraints delta), plus an in-flight store delta under `.mochiko/product/architecture/` — the one legal in-place store write, as in-flight-class elements only, standing beside ruled truth until landing folds it. FEAT-002's entry gains the design-implied dependency/extent assertion and its architecture link once the delta exists.

**Gate: Design checkpoint (user's).** Presented: the store delta (rendered diagram, or source + changed-AX-row table if no render surface) and the other design deltas.
- *Sign* → deltas ratified; proceed to Phase 5. User may instead stop here and resume later — run pauses with the signed design on record.
- *Amend* (e.g., redirect to synchronous on-write indexing, avoiding the topology deviation) → design seats rework under the new direction, re-reviewed, re-presented.
- *Reject only the deviation* → only that element reworks; the rest proceeds.

## Phase 5 — Cycle-card authoring & confirm

- **Seat:** a design-class, non-builder seat slices W1/W2 into cycle cards per `mochiko:patterns-vertical-tdd` — a foundation cycle (search over existing notes, no freshness guarantee) before a feature cycle (freshness path), each card carrying stories, dependencies, acceptance-criteria IDs (SC-101–SC-103), a **TEST:** real-infrastructure gate, and brownfield exposure (`[EXTEND]` on the existing api-service/notes-db, not `[MODIFY]`). `qa-engineer` authors the **TEST:** cases within its slicing.
- **Review:** a verification seat (never the author, never the builder) reviews the cards before confirm — quality via `review-plan-artifacts`, buildability its own judgment.
- **Writes (would be):** `.mochiko/features/FEAT-002/tasks.md`, rendered from the tasks template (or `tasks.yaml` schema raw-read fallback).

**Gate: Card confirm (user's), blocking.** Presented: card count, ordering, per-card scope, **TEST:** gates.
- *Approve* → proceed to Phase 6.
- *Reslice* → card-authoring seat reworks, re-reviewed, re-presented.
- *Reject / send back to design* → routes to Phase 4, scoped to the newly discovered gap.

## Phase 6 — Build, cycle by cycle (test-first)

- **Builder:** `staff-engineer`, plans first (DM approves), decomposes each card into concrete tasks disclosed in the cycle report, follows `mochiko:brownfield-integration` on the `[EXTEND]` touches, applies `mochiko:patterns-code-minimalism`'s pre-code ladder at decomposition, drives red→green→refactor.
- **Discovery note:** no source tree is visible anywhere in this repository snapshot despite FEAT-001 reading `delivered` — a real run would need to locate it (routed through Explore/haiku per model-tiering) or treat its absence as a build-blocking ambiguity escalated to the user rather than inventing structure.
- **Per-cycle verification:** a non-implementer seat runs `mochiko:testing-end-user` against real infrastructure for the card's **TEST:** gate, plus the advisory `mochiko:review-code-minimalism` lens. Important-or-above findings block the cycle and join the checkpoint batch; Minor findings default to a BACKLOG.md booking. Each grading pass consumes one of the 3 per-cycle attempts (or the redeclared bound); two consecutive unchanged-finding rounds triggers a no-progress halt.
- **Mid-build hazard:** if the builder hits undesigned structure (plausible — e.g. a schema column the signed delta didn't cover), that cycle halts and Phase 4 re-fires scoped to the discovery, anchored to the already-signed delta.
- **Writes (would be):** source/test changes (paths TBD at build time), `.mochiko/features/FEAT-002/cycle-report-<cycle>.md` per cycle, `tasks.md` checkbox flips.

## Phase 7 — Final validation (once, whole build)

- **Seats:** an independent verification seat runs the full repository quality-gate suite from a dependency-cold snapshot. The `.claude/worktrees` gitignore entry does not currently exist — it would be added first (mechanical, not judgment), then `git ls-files -co --exclude-standard :!.claude/worktrees` snapshots into `.claude/worktrees/mochiko-<purpose>/`.
- **Regression sweep:** re-runs FEAT-001's `.mochiko/features/FEAT-001/gates.md` (its three existing **TEST:** cases) plus any FEAT-002 gates exercising the shared notes-db seam.
- **Gap-finding (mandatory — selection scope):** a fresh `devils-advocate`, dispatched blind, two-message: first message carries only `spec.md`, `sufficiency-report.md`, design deltas, and the baselines — never code, `tasks.md`, TEST cases, or reports. States derived expectations before probing. Mutation lens applies only at high verification depth, owing results or a stated skip.
- **Finding routing:** spec-required breaks fail final validation and consume the gap-rework bound (2 rounds, or the redeclared value, or the localized cycle's own remaining attempts); beyond-spec findings are advisory and dispositioned by the user (fix now / BACKLOG.md / accept as designed); a disputed kind defaults advisory.
- **Writes (would be):** `.mochiko/features/FEAT-002/final-validation-report.md`, the built-vs-signed diff against the Phase-4 store delta, any gap-rework cycle reports.

## Phase 8 — Acceptance landing (executed whole, only on acceptance)

- **Landing verification seat** checks the graded three-way-diff fold for every touched baseline (`contracts/api.yaml`, `data-model.md` if touched, `constraints-and-decisions.md` if a new D-XXX landed), plus the store landing's special fold (elements flip built, FEAT-002 key clears, As-built:/Drift: fields graded, orphan check). `knowledge-management.md` landing is inapplicable — the file doesn't exist here.
- **Map graduation:** W1/W2 fold into FEAT-002's extent, `entry.md` status → `delivered` (dated), the (currently absent) `FEATURES.md` index line would need creating rather than silently assumed, `gates.md` gains any fix-now/backlog gap findings.

**Gate: Final acceptance (user's), blocking plain text.** Presented: the whole landing package plus final-validation evidence.
- *Accept* → DM executes the landing whole, atomically; closes the run with a verdict against the done condition, citing all 15 fail-conditions as cleared.
- *Amend* → routes to the smallest covering phase (card-level → Phase 6; design-level → Phase 4); scope growth beyond the ratified batch stays FAIL unless the user explicitly accepts the widened scope. Landing does not execute until re-accepted.
- *Reject* → run closes FAIL; no baseline is touched; evidence and state remain for a future run.

## Phase 9 — Close & report

- DM surfaces the rounds consumed and seats spawned at each checkpoint (running tally), states the close verdict in `templates/output-style.md` register, and re-confirms the fail-condition count (15) still matches the schema before closing.
- **Writes:** none beyond what Phase 8 already committed on acceptance.