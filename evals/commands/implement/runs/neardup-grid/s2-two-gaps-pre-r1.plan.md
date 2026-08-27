# Action Plan — `/mochiko:implement FEAT-002` (plan-only, not executed)

## Scenario facts established by reading (no writes made)

- Schema reads: `plugins/mochiko/schemas/implement.yaml` (97 rules across 6 sections, 15 `kind: fail` nodes — count matches the `.md`'s hard-coded Not-done line, no halt needed) and `plugins/mochiko/schemas/common.yaml` (4 shared stubs), both raw and in full, per the command's first-action instruction.
- `FEAT-002` resolves via `FEATURES.md` → `.mochiko/features/FEAT-002/entry.md`: capability "Note search," status **selected**, scope source **the spec's accepted selection** (selection scope, not delta, not epic), rows **W1** (search by query, US-101/SC-101/SC-102) and **W2** (index freshness, US-102/SC-103), both marked selected. Dependency **FEAT-001** is `delivered` → does not block.
- Spec (`.mochiko/specs/note-search/spec.md`) is accepted, has FR-101/102/103 and SC-101/102/103 with a background-index-worker requirement (FR-103).
- Product baselines exist and are ruled: `data-model.md` (Note entity only), `constraints-and-decisions.md` (C-001 single-process/no external services, D-001 SQLite, D-002 stdlib HTTP, no IP rows), `contracts/api.yaml` (only `POST /notes`, `GET /notes/{id}` — **no search endpoint**), architecture `spine.md` (api-service + notes-db, explicitly **"Synchronous request/response only; no queues, no background workers"** — this directly contradicts FR-103's background worker).
- Absent surfaces: no `CLAUDE.md` governance region, no `.claude/rules/mochiko/`, no `.mochiko/memory/` at all (no `codebase-analysis.md`, no `governance-intent.md`, no `knowledge-management.md`), no `BACKLOG.md`, no `.mochiko/provenance.yaml`, no top-level source tree (`src/`, `app/`, etc.) despite FEAT-001 being marked delivered, and **the working directory is not a git repository**.

These facts drive gap detection and the trips raised at run-open below.

---

## Phase 0 — Rule load (already performed for this plan)

Read `plugins/mochiko/schemas/implement.yaml` and `plugins/mochiko/schemas/common.yaml` raw, in full, substituting `${var}` from the schema's `vars:` block (e.g. `attempt_bound_cycle: 3`, `gap_rework_bound: 2`, `builder_seat: staff-engineer`). No writes. This is the run's binding-rules floor; nothing below deviates from it without a named gate.

## Phase 1 — Entry gating

- **Done:** Resolve `FEAT-002` as a capability ID (not `EPIC-XXX`, not a delta card key). Confirm scope = **selection**. Confirm the one selected-row dependency (FEAT-001) is `delivered`, so W1/W2 do not block on dependency order.
- **Read:** `FEATURES.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/features/FEAT-001/entry.md`.
- **Write:** none yet.
- **Seats:** none spawned yet — this is the Delivery Manager's own gating.

## Phase 2 — Sufficiency check (per-row, before any code)

- **Done:** Dispatch the ten-clause per-row sufficiency check (`mochiko:review-sufficiency`) over W1 and W2, against `spec.md`, the architecture store, and the product baselines. Staffing is DM latitude, but the grading seat must have authored none of those sources — `staff-engineer` (builder), `technical-analyst`/`principal-architect` (likely baseline authors) are excluded; a reasonable choice is `mochiko:validator` or `mochiko:tech-lead`. Exempt from plan approval (grading seat).
- **Read (by the graded seat):** `spec.md`, `data-model.md`, `constraints-and-decisions.md`, `contracts/api.yaml`, `architecture/spine.md`.
- **Expected verdict shape, concretely:**
  - **W1** — likely gap: `contracts/api.yaml` has no `GET /notes/search` endpoint definition (API-contract gap). Data model and ranking (newest-first via `created_at`) otherwise look coverable without a new attribute.
  - **W2** — likely gaps: (a) FR-103's background index worker structurally contradicts the spine's ruled "no background workers" topology → needs an architecture-store delta (new element/arrow + NFR-XXX concern row for the 2s freshness bound); (b) the indexing mechanism itself (e.g. SQLite FTS5 vs. hand-rolled) is an undecided D-XXX technology decision, subject to an adopt-first check.
  - Absent-surface branch: governance region absent → surfaced, not auto-resolved, never fails the run; no `codebase-analysis.md` and no visible source tree despite FEAT-001 "delivered" → offer `/mochiko:setup` or proceed greenfield with a logged warning; store has *ruled* content, so no bootstrap offer; `quickstart.md` null path applies (C-001: no external integration surface).
- **Write:** `.mochiko/features/FEAT-002/sufficiency-report.md` — per-row verdict, gap list, absent-surface notes, quickstart null-path note.
- **Gate:** none yet (the binding verdict feeds the run-open confirmation next); a disputed clause inside the report still defaults to gap, per the schema, and is *presented*, not resolved, here.

## Phase 3 — Run-open confirmation (USER GATE — entry closes here)

One confirmation, no negotiation, covering exactly:
- **Batch and scope:** FEAT-002 "Note search," selection scope, rows W1 + W2, dependency FEAT-001 satisfied.
- **Attempt bounds** (their only redeclaration point): `attempt_bound_cycle = 3` per cycle, `gap_rework_bound = 2` rounds at run scope — defaults stated, open for the user to redeclare now only.
- **Sufficiency verdict + gap routing:** the Phase 2 report's per-row verdicts and the named gap list (contracts delta, architecture-store delta + NFR row, D-XXX indexing decision).
- **Trips/conflicts reserved to the user:** governance region absent; no codebase-analysis.md and no visible source tree despite FEAT-001 being "delivered" (an anomaly worth flagging before building on top of it); **not a git repository** — this directly conflicts with the floor obligation that final validation snapshot via `git ls-files -co --exclude-standard`, so it must be ruled here, not silently worked around.
- **Done condition:** stated verbatim from the command's fixed goal — every cycle card checked, test-first, independently verified against real infrastructure per-cycle and whole, code meeting criteria/traceability/governance, landing executed whole, run closed at final acceptance, and none of the 15 fail-conditions standing.

**Gate — branches:**
- **Confirm as presented →** proceed to Phase 4, design phase scoped to the named gap list, bounds as stated (3/2).
- **Redeclare attempt bounds →** carry the new numbers through every later phase that consumes them (Phases 6–8).
- **Rule on the git-repo trip** (e.g. "initialize git first," or "proceed, adapt cold-verification," or "exempt this run's cold-verification round") → binding for Phase 8; an exemption from the attempt count is explicitly the user's call only, never the DM's.
- **Rule on the missing-source-tree anomaly** (e.g. name the actual language/stack, or clarify FEAT-001's delivered claim) → binding constraint for design (Phase 4) and build (Phase 6).
- **Dispute a named gap** (e.g. argue the architecture delta isn't needed) → the schema is explicit: a disputed sufficiency clause still defaults to gap and still routes to design; the disagreement is recorded, not cleared solo.
- **Decline to proceed** → run pauses at entry; only `sufficiency-report.md` exists; FEAT-002 stays `selected`; nothing further happens this invocation.

## Phase 4 — Design phase (fires: gaps were named)

- **Done:** Author exactly the named gaps, nothing more, each on a DM-approved plan, rung-justified per `mochiko:patterns-plan-minimalism`. `staff-engineer` never designs its own gaps. Likely staffing: `principal-architect` for the architecture-store delta (new background-worker topology element, new/updated AX concern row carrying the freshness NFR-XXX), `technical-analyst` for the API-contract delta (`GET /notes/search` in `contracts/api.yaml`) and the D-XXX indexing-mechanism decision (adopt-first checked per `mochiko:patterns-adopt-first`; if a candidate would add infrastructure that collides with C-001's single-process constraint, that collision is a `constraint-challenge` finding reserved to the user, pausing only that decision), `qa-engineer` for the design-time test-case authoring feeding Phase 5.
- **Read:** `sufficiency-report.md`, `spec.md`, all product baselines, `architecture/spine.md`.
- **Write (deltas beside baselines, never in place):** `.mochiko/features/FEAT-002/contracts-delta` (new endpoint, appliable before/after), a D-XXX entry against `constraints-and-decisions.md` (delta form), an architecture-store delta at `.mochiko/product/architecture/` (in-flight-class elements only — the one legal store write before landing), `entry.md` updated with design-implied dependencies/sharpened extent (provenance recorded) and the architecture link once the delta exists.
- **Review pair (non-author, before checkpoint):** `mochiko:review-plan-artifacts` (conformance to the gap list, blocking) and `mochiko:review-feasibility` (buildability/contradiction — required here since a store delta exists).
- **Gate — design checkpoint (USER GATE):** confirms the design deltas and the store delta (rendered diagram + changed AX-XXX row table, since no render binary is asserted present — present source + table and record it).
  - **Sign →** proceed to Phase 5.
  - **Request changes →** design seats rework within this phase; re-reviewed; re-presented.
  - **Stop here, resume later →** explicitly permitted; run pauses with the design deltas standing as far as reached; no cards, no code yet.

## Phase 5 — Cycle card authoring

- **Done:** A design-class seat (not the builder) slices W1+W2 into cycle cards per `mochiko:patterns-vertical-tdd` — foundation cycles (the index-worker skeleton W2 needs) before feature cycles (the search endpoint W1 exposes), Simple/Split/Merge judgment, walking skeleton first. `qa-engineer` authors the `**TEST:**` cases within that slicing. Cards carry stories/rationale, dependencies, acceptance-criteria IDs, a real-infrastructure `**TEST:**` gate, and brownfield exposure (`[EXTEND]`/`[MODIFY]` — these cards touch FEAT-001's existing api-service/notes-db) — no task lists, no file paths.
- **Read:** signed design deltas, `spec.md` acceptance criteria, `tasks.yaml` schema (raw, as the fallback source of truth if `mochiko-cli template tasks` is unavailable).
- **Write:** `.mochiko/features/FEAT-002/tasks.md`.
- **Review before confirm:** an independent verification seat (not the card author) grades quality (`mochiko:review-plan-artifacts`) and buildability (own judgment).
- **Gate — card confirm (USER GATE):** rules the slicing before any build.
  - **Confirm →** proceed to Phase 6.
  - **Re-slice →** design-class seat revises, re-reviewed, re-presented.
  - **Reject/defer →** no card may be built; building before this confirm is itself one of the 15 fail conditions.

## Phase 6 — Build (per cycle, foundation before feature, test-first)

- **Done:** `staff-engineer` executes each confirmed card via `mochiko:executing-tdd-cycle`, decomposing into concrete tasks at build time (disclosed in the cycle report), applying the pre-code minimalism ladder (`mochiko:patterns-code-minimalism`) at decomposition, following `mochiko:brownfield-integration` since these cycles extend existing api-service/notes-db code, red→green→refactor per task, on a DM-approved plan.
- **Read:** the confirmed `tasks.md` card, the existing (extended) codebase, `${rules_dir}` governance files if a governance region existed (it doesn't here — noted absent).
- **Write:** product code changes for that cycle; `.mochiko/features/FEAT-002/cycle-report-<n>.md` (decomposition, difficulties, deviations, domain_deps_added, per `templates/report-format.md`).
- **Mid-cycle routing:** undesigned structure discovered → halt that cycle, re-fire Phase 4 scoped to the discovery only, same grade/checkpoint. An infeasible card escalates to the user as a business-scope decision, never a builder call.

## Phase 7 — Per-cycle verification

- **Done:** An independent seat (never `staff-engineer`) runs `mochiko:testing-end-user` against real infrastructure — actually starting the service against SQLite and exercising each card's `**TEST:**` Setup/Action/Assert (e.g. seed two notes → `GET /notes/search?q=milk`; missing `q` → 400; create-then-search round trip under the 2s bound) — plus the advisory `mochiko:review-code-minimalism` lens reading the diff, cycle report, and surrounding code.
- **Write:** per-cycle verification evidence/report in `.mochiko/features/FEAT-002/`.
- **Attempt economy:** each grading round consumes one of the run-open-declared per-cycle attempts (default 3); two consecutive rounds with unchanged findings is a no-progress stop — halt, present state, disposition is the user's.
- **Findings routing:** Important-or-above blocks the cycle and batches to the cycle checkpoint; Minor defaults to a `BACKLOG.md` booking (would be newly created — first touch of the knowledge-management surface, which is otherwise absent here); minimalism findings stay advisory always.
- **Checkpoint:** any reserved-to-user item (adopt-first tie, ambiguity, disputed finding kind) accumulates and lands as one batch here rather than interrupting mid-cycle, unless build-blocking. On clean grade, `tasks.md`'s checkbox for that card flips `[x]`.

Repeat Phases 6–7 per card until both W1's and W2's cards are checked.

## Phase 8 — Final validation (whole-build)

- **Done:**
  - **Regression sweep:** re-run FEAT-001's durable gate set (`.mochiko/features/FEAT-001/gates.md` — restart-persistence, empty-body 400, get/404) since this feature's territory touches it; a failure here fails the run.
  - **Cold verification:** build/test from a dependency-cold snapshot (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-note-search/`) after confirming the `.claude/worktrees` ignore entry. **This step is exactly where the Phase-3 git-repo trip resolves** — it runs as the user ruled at run-open (git initialized, an approved adaptation, or an explicit exemption), never worked around silently.
  - **Gap-finding pass** (fires: selection scope): fresh, blind `devils-advocate`, two-message dispatch per `mochiko:testing-gap-finding` — first message carries only `spec.md`, `sufficiency-report.md`, design deltas, and the baselines (never code/tasks.md/test cases/reports); seat states derived expectations before probing. Mutation lens applicability depends on `depth` (read from the governance region), which is absent here — this is a second trip to surface and rule on before this phase runs, not silently defaulted.
  - Findings split: spec-required-behavior-broken fails final validation until resolved; beyond-spec findings are advisory, disposition (fix now / backlog / accept-as-designed) reserved to the user.
  - Gap-rework bound: default 2 rounds (or user-redeclared); exhaustion or unchanged findings halts the run, disposition is the user's.
- **Write:** `.mochiko/features/FEAT-002/final-validation-report.md`.

## Phase 9 — Landing (executes whole, only at acceptance)

- **Done:** Store landing (signed delta's elements flip built, FEAT-002 key clears, As-built:/Drift: written and independently graded, orphan check, `spine.md` graded three-way-diff fold, `ARCHITECTURE.md` regenerated by the store skill — never hand-edited) · baseline folds for `contracts/api.yaml`, the D-XXX decision in `constraints-and-decisions.md`, and `data-model.md` if touched, each graded by a non-author landing verification seat · map graduation batch (selection scope): W1/W2 fold into FEAT-002's Extent, vanish from Work rows, status → `delivered` dated, `FEATURES.md` index row updates, spec reads closed once both rows folded · gates fold: any fix-now/backlog gap findings fold into a new `.mochiko/features/FEAT-002/gates.md` in `**TEST:**` grammar, authored by `qa-engineer`; accepted-as-designed findings don't fold · km-landing explicitly skipped and stated as such (no knowledge-management.md present).
- **Write:** `.mochiko/product/architecture/spine.md`, `ARCHITECTURE.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/data-model.md` (if touched), `.mochiko/features/FEAT-002/entry.md`, `FEATURES.md`, `.mochiko/features/FEAT-002/gates.md`.

## Phase 10 — Final acceptance (USER GATE)

Plain blocking text, never timed. Confirms the whole landing result — code, per-cycle and final-validation evidence, cold-verification and regression-sweep outcomes, gap-finding dispositions, folded baselines/store/map — against the fixed done condition.
- **Accept →** run closes; DM issues a close verdict, surfaces rounds/seats consumed, confirms none of the 15 fail-conditions stand.
- **Amend →** user specifies the amendment; scoped rework (possibly a delta-reverification if records/test-only) precedes re-presentation; landing does not finalize until re-accepted.
- **Reject →** run closes FAIL; the standing fail condition is "user acceptance not given"; nothing is treated as delivered.

## Phase 11 — Close

DM states the verdict against the done condition and re-confirms the 15-item fail-condition count is unchanged from the Phase 0 read (schema/`.md` pair in sync) before declaring the run closed.