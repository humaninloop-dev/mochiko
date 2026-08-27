# Action Plan — `/mochiko:implement FEAT-001` (plan-only, not executed)

## Pre-flight facts gathered by reading (no writes)

- **Rules loaded:** `plugins/mochiko/schemas/implement.yaml` (full) and `plugins/mochiko/schemas/common.yaml` (full, for the `extends: common.register` stub). Confirmed the `kind: fail` set contains exactly 15 nodes — count matches the `.md`'s Not-done line, no halt needed.
- **Entry target:** `FEAT-001` resolves to `.mochiko/features/FEAT-001/entry.md` — capability "Note capture," status `selected`, **not** an `EPIC-XXX` → this is a plain capability run, `scope: selection`.
- **Selected work rows (ratified, selection source: spec's accepted selection, 2026-08-20):** W1 — Create a note (US-001, SC-001/SC-002); W2 — Fetch a note by id (US-002, SC-003). Dependencies: none ("First capability on the map") — no dependency-block check needed.
- **Spec:** `.mochiko/specs/note-capture/spec.md` — accepted, FR-001..004, SC-001..003, edge cases present, no UX surface (API-only, no Screens & Flows).
- **Product baselines present and populated:** `.mochiko/product/data-model.md` (Note entity fully modeled), `.mochiko/product/contracts/api.yaml` (both endpoints fully specified), `.mochiko/product/constraints-and-decisions.md` (C-001, D-001, D-002 ruled), `.mochiko/product/architecture/spine.md` (AX-001 persistence, AX-002 logging, AX-003 auth n/a — all `status: ruled`, not empty).
- **Absent surfaces:** no `CLAUDE.md` (→ `governance_region` = absent), no `.claude/rules/mochiko/`, no `.mochiko/memory/` directory at all (no `codebase-analysis.md`, no `knowledge-management.md`, no `governance-intent.md`), no `.mochiko/epics/`. No application source code anywhere in the repo (no `src/`, no `package.json`/`go.mod`/etc.) → this is **greenfield**, nothing delivered yet. Not a git repository (per session context) — flagged below, it affects the final-validation cold-snapshot step.
- **Condition resolutions for this run:** `scope=selection`, `depth`=unresolved (no governance region to read it from → treated as not-`high`, mutation lens skipped and disclosed), `km_file`=absent, `governance_region`=absent, `baseline`=present (all three baselines exist and are ruled), `seats`=will flip to `multi` once more than one seat is spawned (fires the transport floor, referenced not restated).

---

## Phase 1 — Entry gating (moment: `entry`)

**Does:** Confirms FEAT-001 gates validly — a capability entry with selected, ratified work rows (W1, W2), no unresolved dependency. Confirms it is not an epic and there is no delta card in play (this is selection scope only).
**Reads:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/specs/note-capture/spec.md`.
**Writes:** none yet.
**Seats:** none spawned yet — DM-only reasoning.
**Absent-surface handling (`impl.absent-surfaces`, non-failing, surfaced not auto-resolved):**
- Missing governance region → surfaced to the user as a note, run proceeds.
- Missing `.mochiko/memory/codebase-analysis.md` — since the repo is greenfield (no source to analyze), proceed greenfield with a logged warning rather than offering `/mochiko:setup`.
- Store has ruled content (not empty) → the `/mochiko:architecture` bootstrap offer does **not** fire.
- Flag (not a gate): the workspace is not a git repository, which the final-validation cold-snapshot step and delta-reverification's `git rev-parse HEAD:<code-dir>` mechanism depend on — this will need resolving before Phase 9 runs.

## Phase 2 — Sufficiency check (moment: `entry`, tool: `mochiko:review-sufficiency`)

**Does:** Grades W1 and W2 (per-row, ten-clause form — this is selection scope, not delta) against the spec, the architecture store, and the product baselines. Verdict is binding: `sufficient` or a named gap list per row.
**Reads (by the grading seat):** `.mochiko/specs/note-capture/spec.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/architecture/spine.md`. Never reads code, `tasks.md`, `**TEST:**` cases, or cycle reports (none exist yet regardless).
**Seat:** one seat that authored none of the spec/store/baselines being graded — e.g. `mochiko:validator` or `mochiko:qa-engineer` (not `technical-analyst`/`principal-architect`, who are the presumed authors of the baselines and store; not `staff-engineer`, reserved as builder). Exempt from plan approval, per every grading seat.
**Writes:** `sufficiency-report.md` in `.mochiko/features/FEAT-001/` — records the store-consult result, any no-delta claim, trips for run-open, the `quickstart.md` null-path note (no external-integration surface here — pure HTTP+SQLite), and any `[MODIFY]` amendment (none expected — nothing delivered yet to amend).
**On this evidence** (both rows already carry a modeled entity, full contract, ruled constraints/decisions, and ruled architecture concerns with named NFRs), a `sufficient`-for-both verdict is plausible — but the grading seat, not the DM, rules it. Two branches carry forward:
- **Sufficient (zero-gap) for both rows** → Phase 3A (skip design phase).
- **Gap found on either row** → Phase 3B (design phase fires, scoped exactly to the named gaps).
- A disputed clause the grader cannot clear defaults to gap and routes to the user at run-open — never cleared by the grader alone.

## Phase 3A — Zero-gap path (if Phase 2 is clean)

**Does:** No design phase. The card-authoring seat (Phase 5) makes the map-entry assertion the design phase would have made — dependencies/extent onto FEAT-001's entry — surfacing any drift at the card confirm rather than a separate checkpoint.
**Writes:** none here; deferred into Phase 5's card-authoring write.
**Gate:** none — proceeds straight to Phase 4.

## Phase 3B — Design phase (if Phase 2 names gaps) — moment: `design-checkpoint`

**Does:** Fires only on the named gaps, nothing more. Staffing per the gap kind: `technical-analyst` for a spec/design delta, `principal-architect` for a store delta, `qa-engineer` for `**TEST:**` case authoring. Each seat works only on a plan the DM approved, rung-justified per `mochiko:patterns-plan-minimalism`. `staff-engineer` (builder) never designs its own gaps.
**Reads:** `sufficiency-report.md`, the existing baselines, `spec.md`.
**Writes:** deltas beside their baselines in `.mochiko/features/FEAT-001/` (`data-model.md`/`contracts/` deltas, or a prose-baseline before/after delta), plus a store delta at `.mochiko/product/architecture/` only if a structural trigger fired — grammar and lifecycle per `mochiko:authoring-architecture-store`, diagram/register craft per `mochiko:patterns-system-design`. Also asserts sharpened extent/dependencies onto FEAT-001's entry with provenance.
**Review pair (non-author, before checkpoint):** `mochiko:review-plan-artifacts` (conformance to the gap list + card quality — blocking) and `mochiko:review-feasibility` (buildability/contradiction — never defaults to pass).
**Gate — design checkpoint (`impl.gate-design-checkpoint`, floor, the user's):** presents the authored deltas (and, if a store delta exists, either a rendered diagram + the changed AX-XXX row table, or — with no render surface — the source plus the changed-element table) for the user to sign.
- **User signs** → proceeds to Phase 4 with the signed deltas as binding design inputs.
- **User stops here** → the run pauses; it may resume the build later from this checkpoint; nothing beyond this point executes now.
- **User requests changes** → the design seat revises on the same gap scope, re-reviewed, re-presented; does not consume a build-cycle attempt.

## Phase 4 — Run-open confirmation (moment: `run-open`, the entry gate)

**Does:** One confirmation, no negotiation, folding in everything decided so far:
- Names the batch (`FEAT-001`, scope: selection — W1, W2).
- Restates both attempt bounds at their only redeclaration point: 3 verification attempts per cycle, 2 gap-rework rounds at final validation (schema defaults; the user may redeclare either number here and only here).
- Presents the Phase 2 sufficiency verdict and, if Phase 3B fired, its gap routing and outcome.
- Surfaces the trips/absences from Phase 1 (no governance region, greenfield warning, not-a-git-repo flag) and any in-flight conflicts for the user's ruling — these are reserved to the user, never resolved by the DM alone.
- States the done condition verbatim (every cycle card `[x]`, test-first, independently verified per-cycle and whole; criteria traced; governance-aligned; landing executed whole; run closes at final acceptance).

**Gate — the branch:**
- **User confirms as stated** → Phase 5 proceeds with the declared bounds (3 / 2) unchanged.
- **User redeclares different bounds** (e.g., "make it 5 attempts per cycle") → those replace the defaults for the rest of this run only.
- **User flags/rules on a surfaced trip** (e.g., "skip the git-repo requirement," "note the missing governance region and continue") → that ruling is recorded and shapes how Phase 9's cold-verification step is handled later.
- **User declines to open the run** → the run halts here; nothing is built, no code touched, no further phases run.

## Phase 5 — Cycle-card authoring (moment: `card-confirm`, before the gate)

**Does:** A design-class seat that is **not** the builder (e.g. `technical-analyst`) slices W1/US-001 and W2/US-002 into cycle cards per `mochiko:patterns-vertical-tdd` — foundation cycles before feature cycles, walking skeleton first. Given the greenfield state, expect something like: Cycle 1 = minimal HTTP-server + SQLite-file foundation (walking skeleton), Cycle 2 = Create a note (W1), Cycle 3 = Fetch a note by id (W2). `qa-engineer` authors the `**TEST:**` real-infrastructure gate within each card's slicing. If Phase 3A's zero-gap path was taken, this same seat also makes the map-entry assertion here, surfacing any drift.
**Reads:** signed design deltas (if any), `sufficiency-report.md`, `spec.md`, baselines.
**Writes:** `tasks.md` (cycle cards, from the tasks template / `plugins/mochiko/schemas/tasks.yaml` grammar) — per card: stories/rationale, dependencies, acceptance criteria by ID, a `**TEST:**` gate, brownfield exposure (none expected here — pure greenfield build) — no task lists, no file paths.
**Review (independent, before confirm):** the verification seat (e.g. `qa-engineer`, distinct from the authoring seat) reviews card quality per `mochiko:review-plan-artifacts` and judges buildability.

## Phase 6 — Card confirm (moment: `card-confirm`, floor gate, the user's)

**Does:** Presents the sliced cards for the user's ruling before any build starts.
**Gate branches:**
- **Approved as-is** → Phase 7 begins.
- **User requests resequencing/split/merge** → the authoring seat revises, re-reviewed, re-presented; no build has started so no attempt is consumed.
- **User rejects/halts** → run pauses here.

## Phase 7 — Build (per cycle, foundation first) — tool binding `mochiko:executing-tdd-cycle`

**Does, per cycle card:** `staff-engineer` (builder) decomposes the card into concrete tasks at build time (disclosed in the cycle report), runs the pre-code minimalism ladder (`mochiko:patterns-code-minimalism`) at decomposition, builds test-first (red→green→refactor) on the DM-approved plan. `mochiko:brownfield-integration` applies once a later cycle touches an earlier cycle's already-built code (e.g., Cycle 3 touching Cycle 1's server scaffold) — not on Cycle 1 itself, since nothing exists yet.
**Writes:** application code (no path fixed by the schema — the builder's decomposition decides it); `cycle-report.md` in `.mochiko/features/FEAT-001/` per `templates/report-format.md`, disclosing decomposition, honest difficulties, deviations, and any `domain_deps_added`; flips that card's checkbox in `tasks.md`.
**Mid-cycle halts:** if the builder hits undesigned structure, that cycle halts and the design phase re-fires scoped to the discovery (same grade, same checkpoint as Phase 3B). If a commodity-category adopt-first question or an IP-XXX provisioning call arises, it halts to the user rather than the builder deciding (e.g., if a UUID or SQLite-driver choice beyond D-001 surfaces mid-build).

## Phase 8 — Per-cycle verification (moment: `cycle-checkpoint`)

**Does:** An independent verification seat (never the implementer — e.g. `qa-engineer`) runs the card's `**TEST:**` gate against real infrastructure (`mochiko:testing-end-user` — a real SQLite file and a real running HTTP process, not mocks) and the full quality-gate suite, plus the advisory `mochiko:review-code-minimalism` lens over the diff and cycle report.
**Writes:** a verification report in `.mochiko/features/FEAT-001/`; evidence captured, never assumed.
**Attempt economy (floor):** each grading pass consumes one of the 3 per-cycle attempts (or the run-open-redeclared number); two consecutive rounds with unchanged findings is a no-progress stop — halt that cycle and present state to the user rather than retrying blindly. Minor findings default to a `BACKLOG.md` booking; Important-or-above findings block the cycle and join the checkpoint batch. Quality gates and `**TEST:**` gates are never severity-triaged — any failure fails the cycle outright.
**Loop:** Phases 7–8 repeat per remaining cycle card until every card is `[x]`, or the run halts on exhaustion/no-progress and surfaces the state for the user's disposition.

## Phase 9 — Final validation (moment: `final-validation`, whole-build)

**Does (selection scope → gap-finding pass is required, not skippable):**
- **Regression sweep** of previously delivered features' accumulated `**TEST:**` gates — FEAT-001 is the first capability on the map, so this sweep's scope is empty; disclosed as such rather than silently omitted.
- **Cold verification** — builds and runs quality gates from a dependency-cold snapshot of the uncommitted working state (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-note-capture/`, after ensuring the `.claude/worktrees` gitignore entry exists). **Blocker to flag:** the workspace is reported as not a git repository — this mechanism cannot run as specified until that's resolved; this gets raised as a trip at Phase 4's run-open gate and, if unresolved, escalated again here rather than silently skipped.
- **Mutation lens:** `depth` condition is unresolved (no governance region) — treated as not-`high`, skipped, and the skip is disclosed rather than silently omitted.
- **Gap-finding pass (blind, two-message dispatch):** a fresh `devils-advocate` instance that built nothing and saw no design-time test cases. First message carries only `spec.md`, `sufficiency-report.md`, any design deltas, and the baselines (`data-model.md`, `contracts/api.yaml`, the store's NFR-001/NFR-002 rows) — never code, `tasks.md`, `**TEST:**` cases, or reports. The seat states its derived expectations before probing begins, per `mochiko:testing-gap-finding`.
**Writes:** `final-validation-report.md` in `.mochiko/features/FEAT-001/`, including the built-vs-signed diff if a store delta was signed in Phase 3B.
**Findings routing:** spec-required behavior broken → fails final validation, must resolve (bounded by the gap-rework attempt count from Phase 4, default 2 rounds, localized findings instead charge the owning cycle's remaining attempts); beyond-spec findings are advisory, and each disposition (fix now / book to `BACKLOG.md` / accept as designed) is the user's call, batched at this checkpoint.

## Phase 10 — Landing (moment: `landing`, selection scope)

**Does:** Executes as one whole action, not piecemeal:
- **Store landing** (only if a store delta was signed): transcribes the delta's elements to built, clears their FEAT-XXX keys, writes graded `As-built:`/`Drift:` fields on the touched AX-XXX rows, runs the orphan check, regenerates the derived `ARCHITECTURE.md` (never hand-edited).
- **Graded fold** of any touched baseline (data-model, contracts, constraints-and-decisions) via a three-way diff (pre-fold + delta vs folded result), checked by the landing verification seat — never a same-seat self-grade.
- **Map graduation batch** (`impl.landing-selection`): W1 and W2 fold into FEAT-001's extent lines and vanish from the work-row list; capability status flips to `delivered`, dated; the `FEATURES.md` index line updates; the note-capture spec's row in the specs index reads closed now that both its selected rows have folded.
- **Gates fold:** any gap findings ruled fix-now or backlog fold into `.mochiko/features/FEAT-001/gates.md` (minted fresh, since none exists), authored by `qa-engineer` in the `**TEST:**` grammar; accepted-as-designed findings do not fold.
- `km_file` is absent → no knowledge-management landing obligations fire.
**Writes:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/product/architecture/spine.md` (if applicable), `ARCHITECTURE.md`, `.mochiko/features/FEAT-001/gates.md`, any folded baseline files.

## Phase 11 — Final acceptance (moment: `acceptance`, floor gate, the user's)

**Does:** Presents the completed landing, the final-validation evidence, and the verdict against the done condition, as plain blocking text (never a timed prompt).
**Gate branches:**
- **Accept** → the run closes; Phase 12 issues the closing verdict.
- **Amend** → specific changes are named; the run loops back to the relevant phase (a cycle rework within the gap-rework bound, or a landing correction) and re-presents acceptance afterward.
- **Reject** → the run does not close as delivered; nothing beyond what was explicitly ruled stays landed, and the state (which rows actually folded vs. remain pending) is reported plainly rather than assumed.

## Phase 12 — Close

**Does:** Issues a verdict against the done condition, checked explicitly against all 15 `kind: fail` conditions (sufficiency unrecorded, design skipped, card independence, card unchecked, quality gate, no evidence, regression, baseline-in-place, deviation unresolved, store-landing incomplete, ungraded fold, gap-finding missing, skip unstated, spec-gap unresolved, no acceptance) — any one standing fails the run regardless of how far it progressed. Surfaces total rounds consumed and seats spawned across the run (per `impl.dm-surface-rounds`).
**Writes:** none beyond what Phase 10 already wrote — this is a reporting step only.