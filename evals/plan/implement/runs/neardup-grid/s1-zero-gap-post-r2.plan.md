# Action Plan — `/mochiko:implement FEAT-001` (plan-only, not executed)

## Phase 0 — Load binding rules (already done as part of this planning pass)
- **Read** `plugins/mochiko/schemas/implement.yaml` raw, in full, and `plugins/mochiko/schemas/common.yaml` raw, in full (the `extends:` targets it uses: `common.tools-referenced-never-restated`, `common.register`, `common.author-grader-default-fail`, `common.model-tiering`, `common.transport-floor`, `common.no-acceptance`).
- **Verify** the `kind: fail` count in `impl.sec.fail-conditions` is 15 (counted: sufficiency-unrecorded, design-skipped, card-independence, card-unchecked, quality-gate, no-evidence, regression, baseline-in-place, deviation-unresolved, store-landing-incomplete, ungraded-fold, gap-finding-missing, skip-unstated, spec-gap-unresolved, no-acceptance = 15) — matches; no halt needed.
- No writes. No seats.

## Phase 1 — Resolve entry and read sources
- **Read**: `FEATURES.md` (capability map — FEAT-001 "Note capture", status `selected`), `.mochiko/features/FEAT-001/entry.md` (work rows W1 create-note, W2 fetch-note, both `selected`, source = spec's accepted selection dated 2026-08-20; Dependencies: none — first capability on the map, so no dependency-order block), `.mochiko/specs/note-capture/spec.md` (US-001/US-002, FR-001..004, SC-001..003, edge cases), and the product baselines `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/architecture/spine.md` (AX-001 persistence/NFR-001, AX-002 logging/NFR-002, AX-003 auth n/a).
- **Determine scope type**: `FEAT-001` is a `FEAT-XXX`, not an `EPIC-XXX` → **selection scope**, single feature, no epic member resolution.
- **Surface absent-surface checks** (never auto-resolved, never fail the run):
  - No `CLAUDE.md` found at repo root → governance region absent; surfaced to the user as a note, `depth` condition (low/high) left unresolved by this run.
  - No `.mochiko/memory/codebase-analysis.md` found and no application source tree found → brownfield-analysis surface absent; since there is no existing code to analyze, this is read as a greenfield project — offer `/mochiko:setup`, or proceed greenfield with the absence logged (a call for the user at the run-open gate, not decided here).
  - No `.mochiko/memory/knowledge-management.md` found → the KM-landing obligation (`impl.km-landing`) does not apply this run (it is `when: {km_file: present}`).
  - The architecture store (`spine.md`) has ruled content (AX-001, AX-002 ruled; AX-003 n-a) → no store-bootstrap offer needed.
- No writes yet.

## Phase 2 — Sufficiency check (entry gate, binding verdict)
- **Dispatch**: one seat, wired per `mochiko:review-sufficiency`, that authored none of `spec.md`, the architecture store, or the product baselines, and will not design or build this batch (independence per `impl.seat-sufficiency-independence`) — e.g. a fresh `mochiko:tech-lead` or `mochiko:devils-advocate` instance depending on who the DM determines is clean of authorship; exempt from plan approval as a grading seat.
- **Grade** per work row (W1, W2) the ten-clause sufficiency check against spec, store, and baselines — each clause a `sufficient` or `gap` call; a disputed clause defaults to `gap` and routes to the user, never cleared by the grader alone.
- **Write**: `.mochiko/features/FEAT-001/sufficiency-report.md` — the store-consult result and any no-delta claim, trips for the user, the `quickstart.md` null-path note (this feature has no external-integration surface — local SQLite + stdlib HTTP only — so the note states the null path applies), and any `[MODIFY]` amendment against a delivered feature (none possible: no feature has been delivered yet).
- **Outcome branches** (verdict is binding, not mine to predict):
  - **Fully sufficient** for both rows → no design phase fires; proceed to Phase 4 with the zero-gap map-assertion duty carried by the card-authoring seat.
  - **One or more gaps named** → those exact gaps scope an in-run design phase (Phase 3) before any code.

## Phase 3 — Run-open confirmation (user gate, entry closes here)
- **Gate content** (one confirmation, no negotiation): name the batch (FEAT-001, "Note capture", W1+W2) and scope type (selection — no epic members, no delta card); restate attempt bounds at their only redeclaration point — 3 verification attempts per cycle, 2 gap-rework rounds at final validation (both defaults from `vars:`, redeclarable only here); present the Phase 2 sufficiency verdict and its gap routing (if any); present any trips/conflicts for ruling (e.g. the absent-governance-region note, the greenfield/`/mochiko:setup` offer); state the done condition (every cycle card checked, test-first, independently verified per-cycle and whole, criteria traced, governance aligned, landing executed whole, run closes at final acceptance).
- **Branches**:
  - User confirms as stated → run proceeds to Phase 4 (or Phase 3a if gaps were named).
  - User redeclares an attempt bound → the new value is carried for the rest of the run.
  - User rules a trip/conflict (e.g. declines `/mochiko:setup`, accepts proceeding greenfield) → that ruling is recorded and binds downstream phases.
  - User does not confirm / wants changes first → run does not open; no cards, no design work, no code — return to Phase 1/2 with the requested adjustment.

## Phase 3a — Design phase (fires only if Phase 2 named a gap)
- **Staffing** (DM's call, guided by `vars`): `technical-analyst` for design/requirement deltas, `principal-architect` for any store delta, `qa-engineer` for the design-time `**TEST:**` cases — each on a plan the DM approves first (plan-approval-producers), each authoring exactly the named gaps and nothing more, rung-justified per `mochiko:patterns-plan-minimalism`. `staff-engineer` (the builder) never designs its own gaps.
- **Writes**: gap-scoped deltas beside their baselines at `.mochiko/features/FEAT-001/` (e.g. `data-model-delta.md`, a `contracts/` delta, a before/after prose delta for `constraints-and-decisions.md`), plus an architecture-store delta at `.mochiko/product/architecture/` only if the structural trigger fired — written as in-flight-class elements beside the ruled spine, never merged in place. The design phase also asserts sharpened dependencies/extent onto `.mochiko/features/FEAT-001/entry.md` with provenance, and fills the architecture link if a store delta exists.
- **Review pair** (non-author, before the checkpoint): `mochiko:review-plan-artifacts` (conformance to the gap list + card quality — blocking) and `mochiko:review-feasibility` (buildability/contradiction).
- **Gate — design checkpoint (user)**: present the design deltas plus, where a store delta exists, a rendered diagram diff and the changed AX-XXX row table (or, absent a render surface, the source plus the changed-element table). Branches:
  - **Sign** → design and store delta are locked; proceed to Phase 4 (cards authored against the signed design).
  - **Request revision** → design seat reworks within the gap-rework economy; re-review, re-present.
  - **Stop here** → the user may pause the run and resume the build later; no code has been written.

## Phase 4 — Cycle-card authoring
- **Seat**: a design-class seat independent of the builder (e.g. `technical-analyst` or `qa-engineer`), never `staff-engineer`. Slices W1 and W2 into cycle cards per `mochiko:patterns-vertical-tdd` — foundation cycles before feature cycles (e.g. a walking-skeleton cycle standing up the stdlib HTTP server + SQLite datastore + create-note path, then a second cycle for fetch-by-id) — on a DM-approved plan. `qa-engineer` authors the `**TEST:**` real-infrastructure gate within that slicing.
- **Write**: `.mochiko/features/FEAT-001/tasks.md`, rendered from the tasks template (`mochiko-cli template tasks`, falling back to `plugins/mochiko/schemas/tasks.yaml` read raw if the binary is absent). Each card carries stories/rationale, dependencies, acceptance-criteria IDs (SC-001..003), a `**TEST:**` gate, and brownfield exposure (all `[NEW]` here — no existing app code to extend/modify).
- If Phase 2 found no gaps: this seat also carries the **zero-gap map assertion** — making the map-entry assertion the design phase would have made, surfacing any drift at the card confirm.
- **Review**: `qa-engineer` (independent of the card author and the builder) reviews the cards for quality (`mochiko:review-plan-artifacts`) and buildability before the confirm.

## Phase 5 — Card confirm (user gate)
- **Gate content**: present the sliced cards, their build order, `**TEST:**` gates, and the reviewer's findings.
- **Branches**:
  - **Confirm** → proceed to Phase 6 build, in the confirmed order.
  - **Request re-slice** → card-authoring seat reworks the slicing within the run's economy; re-review, re-present.
  - **Reject** → run halts pending the user's direction (may mean returning to Phase 3a/4, or stopping the run).

## Phase 6 — Build: cycle execution (test-first, per confirmed card)
For each card, in dependency/foundation-first order:
- **Builder** (`staff-engineer`, on a DM-approved plan): decomposes the card into concrete tasks at build time, drives red→green→refactor, applies `mochiko:patterns-code-minimalism`'s pre-code ladder at decomposition (rungs disclosed — likely landing on stdlib HTTP + the platform's bundled SQLite driver per already-ruled D-001/D-002), and follows `mochiko:brownfield-integration` for any cycle that touches a prior cycle's code.
- **Write**: `.mochiko/features/FEAT-001/cycle-report-<n>.md` per card — disclosed decomposition, honest difficulties, deviations, `domain_deps_added`. Application code itself lands wherever the project's source layout calls for it (to be established by the first cycle, since none exists yet).
- **Verify**: an independent seat (`qa-engineer`, never the implementer) runs the `**TEST:**` gate against real infrastructure (`mochiko:testing-end-user` — an actual running service and actual SQLite file, not mocks) and applies the `mochiko:review-code-minimalism` lens against the diff, the cycle report, and the surrounding code (advisory `minimalism:` findings only).
- **Cycle checkpoint**: findings batch — Important-or-above blocks the cycle and joins the checkpoint; Minor defaults to a `BACKLOG.md` booking. Attempt economy: 3 verification attempts per cycle by default; two consecutive rounds with unchanged findings triggers a no-progress stop (halt, present state, user disposes).
- On pass: flip that card's checkbox `[x]` in `tasks.md`.
- Any undesigned structure discovered mid-build halts that cycle and re-fires Phase 3a scoped to the discovery (same grade, same checkpoint), rather than being designed around silently.

## Phase 7 — Final validation (whole-build verification)
- **Regression sweep**: run the accumulated `**TEST:**` gates of previously delivered features in this territory — none exist (FEAT-001 is the first capability), so this step records "no prior gates to sweep" rather than a violation.
- **Quality gates**: run the full repository suite established during Phase 6 (lint/build/test).
- **Cold verification**: snapshot the uncommitted working tree (`git ls-files -co --exclude-standard :!.claude/worktrees`) into `.claude/worktrees/mochiko-<purpose>/`, confirming `/.claude/worktrees` is gitignored first, and build/run the quality gates there as acceptance evidence.
- **Gap-finding pass** (required — selection scope, not skippable): a fresh `devils-advocate` seat, never one that built these cycles or saw the design-time test cases, dispatched blind in two messages — first message carries only `spec.md`, `sufficiency-report.md`, any design deltas, and the baselines (`data-model.md`, `contracts/api.yaml`, the store's NFR rows); never the code, `tasks.md`, `**TEST:**` cases, or reports. The seat states derived expectations before probing the running system. Mutation lens runs on the verification seat only if governance `depth: high` — `depth` is unresolved here (no governance region present), so this is disclosed as an open item rather than silently skipped.
- **Write**: the final-validation report at `.mochiko/features/FEAT-001/`, including the regression-sweep result, cold-verification evidence, and gap-finding findings.
- **Findings routing**: spec-required behavior broken → fails final validation, reworked within the 2-round gap-rework bound (or charged to the originating cycle's remaining attempts if localized); beyond-spec findings → advisory, disposed by the user at the checkpoint (fix now / `BACKLOG.md` / accept as designed).

## Phase 8 — Landing (executed whole, at acceptance)
- **Store landing**: any in-flight store-delta elements from Phase 3a flip to built, FEAT-001's key clears from them; the touched AX-001/AX-002(/AX-003) rows get judgment `As-built:`/`Drift:` writes, independently graded by a non-author landing-verification seat; the orphan check runs; `ARCHITECTURE.md` is regenerated by the store skill, never hand-edited.
- **Baseline folds**: any `baseline-delta.md` produced during build (a build-time D-XXX/C-XXX/IP-XXX or model/contract touch) folds via a three-way diff, graded by the landing verification seat.
- **Map graduation** (selection scope): W1 and W2 fold into FEAT-001's extent lines and vanish from "selected"; status flips to `delivered`, dated; `FEATURES.md`'s FEAT-001 row updates; the spec's closure is derived — `note-capture/spec.md` reads closed once both rows have folded.
- **Gates fold**: gap findings ruled fix-now or backlog fold into `.mochiko/features/FEAT-001/gates.md` (minted here, since it doesn't yet exist) in `**TEST:**` grammar, authored by `qa-engineer`; accepted-as-designed findings do not fold.
- **KM landing**: skipped — no `knowledge-management.md` present, so this obligation does not apply.
- **Landing verifier**: independent seat checks every graded fold above.

## Phase 9 — Final acceptance (user gate, closes the run)
- **Gate content**: the DM presents the full run state — all cards `[x]`, verification evidence per cycle and whole, the final-validation report, the landing's graded folds — against the done condition, as plain blocking text (never timed).
- **Branches**:
  - **Accept** → run closes; DM states the verdict against the done condition and surfaces total rounds/seats consumed (`impl.dm-close-verdict`, `impl.dm-surface-rounds`).
  - **Amend** → user specifies the change; it is scoped and routed back into Phase 6 (build) or Phase 7 (re-validation) as a fresh cycle or delta-reverification, then returns to this gate.
  - **Reject** → run does not close as delivered; DM presents outstanding state and awaits the user's direction (could mean reopening an earlier phase or halting the run entirely).

## Phase 10 — Not-done audit (checked before any closing verdict is stated)
- DM checks the run against all 15 `impl.fail.*` conditions from `impl.sec.fail-conditions` (sufficiency unrecorded, design skipped without sign-off, card independence/confirm violated, an unchecked card, a failing quality gate, verification without real-infrastructure evidence, a regression, a baseline edited in place, an unresolved deviation, an incomplete store landing, an ungraded fold, a missing gap-finding pass, an unstated skip, an unresolved spec-required gap finding, no user acceptance). Any one standing fails the run regardless of the Phase 9 outcome; none standing plus Phase 9 acceptance is what closes the run as done.