# Action Plan — `/mochiko:implement FEAT-001` (plan-only, not executed)

**Scenario resolved from the repo:** FEAT-001 is "Note capture" on the `notely` map (`FEATURES.md`, status `selected`). Its entry (`.mochiko/features/FEAT-001/entry.md`) carries two ratified work rows sourced from the spec's accepted selection (2026-08-20): W1 — create a note, W2 — fetch a note by id. No dependencies; it's the first capability on the map. The spec (`.mochiko/specs/note-capture/spec.md`), and all three product baselines plus the architecture spine, already exist and look populated (SQLite/stdlib decisions ruled, AX-001/AX-002 already cite FEAT-001 as their grading source). No `CLAUDE.md` governance region, no `.mochiko/memory/knowledge-management.md`, and no product source code exist yet — this is a greenfield build, and the working directory is **not a git repository**, which matters later for the cold-verification step.

This resolves scope = **selection** (not epic, not delta), which fixes which conditional rules apply below (gap-finding runs, epic-only rules don't, delta-only rules don't).

---

## Phase 0 — Load binding rules (done)
Read `plugins/mochiko/schemas/implement.yaml` and `plugins/mochiko/schemas/common.yaml` raw and in full, plus `plugins/mochiko/schemas/command-labels.yaml` for label meaning. Confirmed the fail-condition set is exactly 15 entries, matching the run's hard-coded count — no halt needed on that check. No writes. No seats.

## Phase 1 — Entry gating
**Does:** Resolve `FEAT-001` → capability "Note capture," confirm it carries ratified, selected work rows (not empty, not an unratified draft), and check no selected row depends on an undelivered row (none here — first capability, no deps).
**Reads:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/specs/note-capture/spec.md`, `CLAUDE.md` (absent), `.mochiko/memory/knowledge-management.md` (absent), `.mochiko/memory/codebase-analysis.md` (absent), `.claude/rules/mochiko/` (absent), `.mochiko/product/architecture/spine.md`.
**Absent-surface handling (surfaced to the user, never auto-resolved, never fails the run):** missing governance region; missing/stale codebase analysis on what is greenfield with no existing code — proceed greenfield with a logged warning rather than routing to `/mochiko:setup`, since there's nothing to analyze; the store is not empty (it carries ruled content), so no architecture-bootstrap offer is needed.
**Writes:** none.
**Seats/skills:** delivery-manager judgment only.
**Gate:** none yet — this phase only sets up what's presented at run-open.

## Phase 2 — Sufficiency check
**Does:** An independent seat that authored none of the spec, store, or baselines (proposed: `mochiko:validator`, since `qa-engineer`/`technical-analyst`/`principal-architect` are earmarked for later design/build/verify roles and `devils-advocate` is reserved fresh for the later blind gap-finding pass) grades all ten clauses per work row (W1, W2) against the spec, the architecture store, and the product baselines — testable criteria, contract/data exposure, structural trigger, NFR targets, commodity exposure, dependency order, UX trace (n/a, no Screens & Flows), delivered/in-flight exposure. Given the baselines already explicitly cite FEAT-001 in their ruled rows, a `sufficient` verdict on both rows is the likely outcome, but the check runs regardless and can still surface a gap or a store trip.
**Reads:** spec.md, architecture spine, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, the FEAT-001 entry — never code, `tasks.md`, `**TEST:**` cases, or this run's own output directory.
**Writes:** `.mochiko/features/FEAT-001/sufficiency-report.md` — per-row verdict, any gap list keyed to clause, store-consult result, any trips or in-flight conflicts for the user, the `quickstart.md` null-path note (no real external-integration surface here).
**Seats/skills:** the independent grading seat; procedure owned by `mochiko:review-sufficiency`.
**Gate:** none directly — feeds the run-open confirmation next. A disputed clause the grader can't clear defaults to gap and rides into that confirmation for the user's ruling.

## Phase 3 — Run-open confirmation (GATE 1, blocking)
**Does:** Presents to the user, in one shot: the batch name and scope type (FEAT-001, selection scope, rows W1+W2); both attempt bounds at their only redeclaration point (default 3 verification attempts per cycle, default 2 gap-rework rounds at run scope) with the option to change either now; the sufficiency verdict and its gap routing; any store trips or in-flight conflicts for ruling; the done condition (every card checked, test-first, independently verified against real infrastructure per-cycle and whole, landing executed whole, run closes on accept/amend/reject).
**Reads/writes:** none beyond what Phase 2 produced.
**Gate — what's confirmed:** the batch and scope, the attempt bounds as stated or redeclared, and how to route the sufficiency verdict.
**Branches:**
- *User confirms, verdict is sufficient* → skip Phase 4, proceed straight to Phase 5 (cycle cards).
- *User confirms, verdict named gaps* → proceed to Phase 4, scoped to exactly those gaps.
- *User redeclares attempt bounds* → the new values carry for the rest of the run.
- *User disputes the entry itself (wrong batch, wants a different capability, or the row shouldn't have been selected)* → the run does not open; route back to `/mochiko:specify` (new capability) or `/mochiko:feature` (delta), per the entry-gating rule — no code touched.

## Phase 4 — Design phase (fires only if Phase 2/3 named a gap)
**Does:** Design-class seats (proposed: `technical-analyst` for spec/design-artifact deltas, `principal-architect` only if a structural/store delta is actually implicated, `qa-engineer` for `**TEST:**` case content) author exactly the named gaps, nothing more, each on a plan the delivery manager approves first, rung-justified against the simplest-execution ladder. Given the baselines already look complete and pre-cite FEAT-001, this phase is likely to be thin or not fire at all — but the plan must account for it firing.
**Reads:** `sufficiency-report.md`, existing baselines, spec.md.
**Writes (only the named gaps):** deltas beside the baselines in `.mochiko/features/FEAT-001/` (e.g. a `data-model.md` delta, a `contracts/` delta, a before/after prose delta against `constraints-and-decisions.md`); a store delta under `.mochiko/product/architecture/` only if a structural trigger actually fired; a map-entry assertion on `entry.md` (dependencies/extent with provenance).
**Independent review pair (before the checkpoint, non-author):** conformance-and-card-quality review against the gap list (blocking) and a separate feasibility/buildability review.
**Gate 2 — design checkpoint (blocking, the user's):** signs the design and any store delta (on a rendered diagram plus the changed row table, or the source-plus-table fallback if no render surface exists). The user may stop here and resume the build later.
**Branches:**
- *Sign as-is* → proceed to Phase 5.
- *Request amendment, still inside the named gap scope* → design seat revises, re-review, re-present.
- *Stop/pause* → run holds open, resumable later without rework.

## Phase 5 — Cycle-card authoring
**Does:** The design-class seat (`technical-analyst`) slices W1+W2 into vertical cycle cards (`mochiko:patterns-vertical-tdd`); `qa-engineer` authors the `**TEST:**` case content within that slicing. Since no HTTP service or persistence path exists yet, the first cycle is a walking skeleton — the thinnest end-to-end path (stdlib HTTP endpoint + SQLite file + one trivial case green) — with the create-note and fetch-note-by-id behavior built out from there. A plausible (not prescribed) shape: one cycle bundling the walking skeleton with W1/US-001 (create, SC-001/SC-002), a second cycle for W2/US-002 (fetch, SC-003) depending on the first's persistence path.
**Reads:** the sufficiency report or, if Phase 4 fired, the signed design deltas; spec.md's acceptance criteria; the product baselines; `plugins/mochiko/schemas/tasks.yaml` for the card skeleton.
**Writes:** `.mochiko/features/FEAT-001/tasks.md` — unchecked cycle cards (stories/rationale, dependencies, named `**TEST:**` cases, brownfield exposure, Simple/Split/Merge rationale; no task lists, no file paths).
**Zero-gap path note:** if Phase 4 never fired, the card-authoring seat itself makes the map-entry assertion the design phase would have made, surfacing any drift at the confirm below.
**Independent review before confirm:** a verification seat distinct from the card author checks card quality and buildability.
**Gate 3 — card confirm (blocking, the user's):** rules the slicing before any build starts.
**Branches:**
- *Approve* → proceed to Phase 6.
- *Request re-slice* → design seat revises, re-review, re-present.
- *Reject/halt* → run pauses, no build begins.

## Phase 6 — Build (per cycle, foundation first, test-first)
**Does, per card in dependency order (skeleton/W1 cycle, then W2 cycle):** `staff-engineer` decomposes the card into concrete tasks at build time (disclosed in the cycle report), drives red→green→refactor, follows brownfield-integration handling once code from the first cycle exists to extend, and discloses the code-minimalism ladder at decomposition. The builder never designs its own gaps — if it hits undesigned structure mid-cycle, that cycle halts and a scoped design-phase re-fire (Phase 4, narrowed to the discovery) runs before it resumes.
**Reads:** the confirmed `tasks.md` card, signed design deltas (if any), product baselines, spec.md's cited criteria.
**Writes:** the actual `notely` source (HTTP handlers, SQLite persistence, note model) and its tests; `.mochiko/features/FEAT-001/cycle-report.md` per cycle (decomposition, difficulties, deviations, any dependency additions).
**Per-cycle verification:** `qa-engineer`, never the implementer, runs the card's `**TEST:**` cases against real infrastructure (an actual SQLite file, actual HTTP calls, never mocks), plus an advisory code-minimalism read of the diff and surrounding code. The full repository quality-gate suite runs and is never severity-triaged — a failing gate fails the cycle.
**Attempt economy:** each grading pass consumes one of the cycle's attempt budget (default 3, or the run-open redeclared value); two consecutive rounds with unchanged findings is a no-progress stop — halt the cycle and present state for the user's disposition.
**Mid-cycle escalations (only build-blocking ones interrupt immediately; the rest batch to the checkpoint):** any deviation from a signed architecture delta (add/remove a box or arrow, move a responsibility) stops and is presented — build as approved or amend by the user's ruling; any commodity-category adopt-first call or infrastructure-provisioning call halts to the user (unlikely here — storage/HTTP-layer decisions are already ruled as D-001/D-002 and C-001 blocks external infra).
**Gate (cycle checkpoint, blocking, batched):** accumulated escalations and findings land together per cycle; the user rules them before the card checks off.
**Writes on completion:** `tasks.md` checkbox flips to `[x]` once the card's `**TEST:**` gate and quality gates are green.
Repeat until every card is checked.

## Phase 7 — Final validation (whole-build, once)
**Does:**
- Re-runs the full quality-gate suite.
- Runs the regression sweep of previously delivered features' durable gate sets in this territory — there are none yet (FEAT-001 is the map's first capability), so this leg is real but trivially empty; stated explicitly rather than silently skipped.
- Runs the cold-verification build: copies the tracked/untracked working state to a fresh `.claude/worktrees/mochiko-<purpose>/` snapshot via `git ls-files -co --exclude-standard`. **Environmental flag:** the working directory is currently not a git repository, so this step cannot run as written. That gets surfaced to the user as an entry-level environmental gap — offering `git init` — rather than improvised around, since initializing a repository is exactly the kind of state-changing action that needs a confirmation, not a silent workaround.
- Runs the blind gap-finding pass (required at selection scope, never skippable here): a **fresh** `devils-advocate` instance, never having built these cycles, gets a first message carrying only spec.md, the sufficiency report, any design deltas, and the baselines (never code, `tasks.md`, `**TEST:**` cases, or reports); it states its expectations, then probes the running system.
- Splits findings: anything showing spec-required behavior broken fails final validation and must be resolved; beyond-spec findings are advisory, with disposition (fix now / book to `BACKLOG.md` / accept as designed) reserved to the user. A disputed finding kind defaults advisory and goes to the user.
**Reads:** the built code, spec.md, sufficiency-report.md, design deltas, baselines, `tasks.md`, cycle reports.
**Writes:** `.mochiko/features/FEAT-001/final-validation-report.md`.
**Gap-rework bound:** default 2 rounds at run scope (or the redeclared value), except a finding localized to one cycle's territory charges that cycle's own remaining budget instead; exhaustion or no-progress halts and presents state for the user's disposition.

## Phase 8 — Landing (executed whole, at acceptance)
**Does:** Folds every touched surface exactly once, graded by an independent landing-verification seat (never the design author or builder):
- Store landing (only if a store delta exists): elements flip built, `FEAT-XXX` keys clear, As-built/Drift fields written and graded, orphan check runs, root `ARCHITECTURE.md` regenerated by the store skill. If Phase 4 produced no store delta, this is a no-op transcription check, stated as such.
- Graded three-way-diff fold of any touched baseline delta (only if Phase 4 wrote one).
- Map graduation batch (selection scope): W1 and W2 fold into FEAT-001's extent lines and vanish from the selected-rows list; status set `delivered`, dated; `FEATURES.md`'s FEAT-001 line updates; the spec's row reads closed once both rows have folded.
- Gates fold: gap-finding findings ruled fix-now or backlog fold into a freshly minted `.mochiko/features/FEAT-001/gates.md`, authored by `qa-engineer` in the `**TEST:**` grammar; accepted-as-designed findings do not fold.
- Knowledge-management landing: skipped, stated as such — no `knowledge-management.md` exists.
**Reads:** signed deltas, cycle reports, `final-validation-report.md`, current `FEATURES.md`/`entry.md`/spine/baselines.
**Writes:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/product/architecture/spine.md` + `ARCHITECTURE.md` (only if a store delta exists), the three product baseline files (only if Phase 4 touched them), `.mochiko/features/FEAT-001/gates.md` (new).

## Phase 9 — Final acceptance (GATE, blocking, closes the run)
**Does:** Presents the done-condition checklist against reality — every card checked and independently verified per-cycle and whole against real infrastructure; code traces to the spec's criteria; governance alignment noted as n/a (no governance region exists); the landing executed whole above — plus an explicit pass/fail read of each of the 15 not-done conditions (unrecorded sufficiency, skipped/unsigned design, card independence/unchecked, failing/triaged gates, unevidenced verification, regression, in-place baseline edit, unresolved deviation, incomplete store landing, ungraded fold, missing/unstated gap-finding, unresolved spec-gap finding, no acceptance).
**Branches:**
- *Accept* → run closes; a close-verdict report is issued against the done condition.
- *Amend* → the user names specific changes; the run routes back to the relevant open phase (a cycle rework, a design amendment, a landing correction) under the still-open run, drawing on remaining attempt/gap-rework budget.
- *Reject* → run closes without delivery; the graduation batch in Phase 8 is not executed (or is held pending the user's explicit instruction, never unwound via a destructive git action without separate confirmation); W1/W2 stay `selected` for a future run.

---

**Cross-cutting, all phases:** never a git mutation beyond the ephemeral, self-removing cold-verification snapshot — commits are suggested, never made or pushed; all user-facing text follows the plain, ultra-register house style; because this run composes more than one seat (grader, design seats, builder, verifiers, gap-finder), the transport floor governs every cross-seat message and every shared write surface (notably `tasks.md` and the product baselines) from the first multi-seat moment onward.