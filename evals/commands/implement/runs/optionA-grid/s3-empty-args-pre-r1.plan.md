# Action Plan — `/mochiko:implement` (no argument given)

Rules loaded first, as required: `plugins/mochiko/schemas/implement.yaml` (full raw read) and `plugins/mochiko/schemas/command-labels.yaml`. Fail-condition count verified at 15 (matches the schema's own count claim). Project state read to ground the plan: `FEATURES.md`, `ARCHITECTURE.md`, `.mochiko/features/FEAT-001/entry.md` + `gates.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`. No `.mochiko/memory/`, no `.claude/rules/mochiko/`, no `.claude/worktrees/`, no `.gitignore`, and the workspace is not a git repository.

## Phase 1 — Scope resolution

**Does:** No capability ID was passed. FEATURES.md lists two capabilities: FEAT-001 (delivered, closed) and FEAT-002 "Note search" (status `selected`, two ratified work rows). FEAT-002 is the only ready candidate, so the DM proposes it.
**Reads:** `FEATURES.md`, `.mochiko/features/FEAT-002/entry.md`.
**Writes:** none.
**Gate (informational, folds into Phase 3's confirmation):** propose FEAT-002 to the user as the batch to run; the user could instead name a different capability or decline. Plan continues assuming FEAT-002.

## Phase 2 — Dependency and absent-surface surfacing

**Does:** Checks FEAT-002's declared dependency — FEAT-001 — is `delivered`, so it does not block. Surfaces environment gaps found while reading, none auto-resolved, none failing the run:
- No `.mochiko/memory/codebase-analysis.md` and no visible product source tree beyond the plugin itself, despite FEAT-001 being marked delivered — surfaced as an anomaly for the user to explain (where the delivered code lives) rather than assumed.
- No governance region (`.claude/rules/mochiko/`) — surfaced as absent, run proceeds.
- No `.mochiko/memory/knowledge-management.md` — KM landing obligations skipped, noted.
- Workspace is not a git repository — this blocks the later cold-verification step (Phase 15) as designed, since it depends on `git ls-files`. Surfaced now so the user can rule on it early rather than at the point of failure.
**Reads:** directory listings only.
**Writes:** none yet — these become lines in `sufficiency-report.md` (Phase 4).

## Phase 3 — Sufficiency check

**Does:** An independent grading seat — not the builder, not a design seat for this batch, and author of none of the spec/store/baselines (e.g. `validator`) — grades FEAT-002's two work rows against the ten-clause checklist (`mochiko:review-sufficiency`), reading only `spec.md`, the architecture store, the product baselines, and the map entries.
**What the read of current state suggests it will likely find** (the seat's verdict is binding, this is not a substitute for it):
- Clause 3 (data exposure): the spec needs a search index but `data-model.md` names only `Note` — likely gap.
- Clause 4 (structural trigger): the spec's FR-103 requires a **background index worker**, but the architecture spine states explicitly "Synchronous request/response only; no queues, no background workers" — a direct collision, near-certain gap requiring a store delta.
- Clause 5 (NFR targets): SC-103's 2-second freshness bound has no matching concern row in the spine's concern catalog — likely gap.
- Clause 6 (commodity exposure): search/indexing technology has no adopt-first-weighed decision in `constraints-and-decisions.md` — likely gap.
- Clause 2 (contract exposure): `/notes/search` is absent from `contracts/api.yaml` — judgment call on locatable-vs-unattachable, flagged for the seat.
- Clause 9 (delivered-feature exposure): if the index lives beside `notes-db` (FEAT-001's territory), this auto-fires a `[MODIFY]` amendment on FEAT-001's entry — flagged as a likely trigger.
**Reads:** `spec.md`, `spine.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `FEAT-002/entry.md`.
**Writes:** `.mochiko/features/FEAT-002/sufficiency-report.md` (per-row verdicts, gap list keyed to clauses, store-consult result, any trips, any in-flight conflicts).

## Phase 4 — Run-open confirmation (USER GATE)

**Does:** One confirmation, no negotiation, presenting: the batch (FEAT-002, selection scope) · both attempt bounds at their only redeclaration point (default 3 verification attempts per cycle, default 2 gap-rework rounds at run scale) · the sufficiency verdict and its gap routing from Phase 3 · any store trips or in-flight conflicts (none currently apparent from the read baselines, but the seat's actual run may surface more) · the git-repository absence from Phase 2 as a trip needing a ruling before cold verification can run · the done condition.
**Reads:** none new.
**Writes:** none yet (attempt-bound redeclaration, if any, is recorded in the run's own state, not a durable artifact).
**Gate — branches:**
- **Confirm as-is:** proceed to Phase 5 (design phase fires, since gaps are expected).
- **Redeclare attempt bounds:** new values carried for the rest of the run.
- **Rule on the git-repo trip:** e.g. authorize `git init` before build starts, or accept a degraded cold-verification path — either way this is decided here, not silently.
- **Reject / defer FEAT-002:** run ends without proceeding; DM reports no work done.

## Phase 5 — Design phase (fires — gaps were named)

**Does:** Design-class seats each author exactly the named gaps, nothing more, each on a DM-approved plan:
- `technical-analyst`: `data-model.md` delta (search-index representation), `contracts/api.yaml` delta (`/notes/search` endpoint, 400 on missing/short `q`), `constraints-and-decisions.md` delta (adopt-first-weighed indexing decision, e.g. SQLite FTS5 vs. hand-rolled, plus the NFR-XXX freshness target) — per `mochiko:authoring-technical-requirements`.
- `principal-architect`: architecture store delta introducing the background index worker (new element or amended flow), a rendered C4-container delta diagram, sequence diagram if qualifying, per `mochiko:patterns-system-design` / `mochiko:authoring-architecture-store`.
- `qa_seat` (qa-engineer): design-time **TEST:** cases within its slicing.
- The commodity/adopt-first ruling on the indexing approach is **not** any seat's call — it is flagged here for the user's ruling, taken at the design checkpoint (Phase 6) rather than decided silently.
**Reads:** `sufficiency-report.md`, existing baselines, `spec.md`.
**Writes:** `.mochiko/features/FEAT-002/data-model.md` (delta), `.mochiko/features/FEAT-002/contracts/` (delta), `.mochiko/features/FEAT-002/constraints-and-decisions-delta.md` (appliable before/after form), a store delta under `.mochiko/product/architecture/` (in-flight-class elements only — the one legal in-place carve), `FEAT-002/entry.md` updated with design-implied dependencies/sharpened extent and the architecture link (currently blank).
**Review pair (non-author):** `mochiko:review-plan-artifacts` (conformance to the gap list, blocking) and `mochiko:review-feasibility` (buildability/contradiction, plus the architecture pass since a store delta exists) — run by a seat that authored none of Phase 5's outputs.

## Phase 6 — Design checkpoint (USER GATE, floor)

**Does:** Presents the rendered diagram plus the changed AX-row table (or source + table if no render surface), the design deltas, the review-pair verdicts, and the deferred adopt-first/IP-XXX rulings from Phase 5.
**Reads:** none new.
**Writes:** the store delta becomes signed truth (still standing beside the store, not folded — folding happens at Phase 15 landing).
**Gate — branches:**
- **Sign:** design + store delta are approved; proceed to card authoring (Phase 7). User may instead sign and stop here, resuming the build in a later run.
- **Amend:** user directs specific revisions; the same design seats revise within the same gap scope, re-reviewed, re-presented.
- **Reject / narrow scope:** e.g. drop W2 (freshness) from this run, keep W1 only — routes back through Phase 1 scoping; run may end here.

## Phase 7 — Card authoring

**Does:** A design-class seat (not the builder) slices the two work rows — plus the FEAT-001 `[MODIFY]` amendment if clause 9 fired — into cycle cards, foundation cycles first (index storage/backfill, background worker scaffold) then feature cycles (search endpoint, freshness path), per `mochiko:patterns-vertical-tdd`. `qa_seat` authors each card's **TEST:** real-infrastructure gate.
**Reads:** signed design deltas, `spec.md` (SC-101/102/103), `tasks.yaml` schema (fallback if the render binary is absent).
**Writes:** `.mochiko/features/FEAT-002/tasks.md`.
**Review (independent, before confirm):** verification seat grades quality (`mochiko:review-plan-artifacts`) and buildability, its own judgment.

## Phase 8 — Card confirm (USER GATE, floor)

**Does:** Presents the sliced cards for the user to rule on before any building starts.
**Gate — branches:**
- **Confirm:** build begins (Phase 9).
- **Resplit / reorder:** design seat revises cards, re-reviewed, re-confirmed.
- **Reject:** run halts here.

## Phase 9 — Build (per cycle, test-first)

**Does:** `staff-engineer` executes each confirmed card via `mochiko:executing-tdd-cycle`: decomposes into concrete tasks (disclosed in the cycle report), works only on a DM-approved plan, drives red→green→refactor, applies `mochiko:brownfield-integration` on any touch to the existing `api-service`/`notes-db` code, and runs the `mochiko:patterns-code-minimalism` ladder at decomposition (rungs disclosed). Any Explore-dispatched locate/enumerate reads route through a haiku-model subagent per model tiering.
**Reads:** signed design deltas, existing code around each touch point.
**Writes:** production code (paths determined by the actual codebase, not enumerated at design time), `.mochiko/features/FEAT-002/cycle-report-<n>.md` per cycle.
**Mid-build contingencies (no separate numbered phase — loop back when tripped):**
- Undesigned structure discovered → halt that cycle, design phase re-fires scoped to the discovery (back to Phase 5/6 for that slice only).
- A deviation from the signed delta (box/arrow/responsibility change) → stop, present to the user: build as approved or amend the delta first.
- A commodity/adopt-first or IP-XXX call surfaces mid-build → halts to the user, never decided by the builder.

## Phase 10 — Per-cycle verification

**Does:** An independent verification seat (never the implementer) runs the card's **TEST:** gate against real infrastructure with captured evidence (`mochiko:testing-end-user`) and the advisory `mochiko:review-code-minimalism` lens over the diff and cycle report. Attempt economy: default 3 attempts per cycle (or the run-open redeclared value); two consecutive rounds with unchanged findings is a no-progress stop — halt the cycle, present state to the user. Minor findings default to a BACKLOG.md booking; Important-or-above findings block the cycle and join the next checkpoint batch.
**Reads:** the diff, `cycle-report.md`, surrounding codebase.
**Writes:** verification report per cycle in `FEAT-002/`; on pass, flips the card's checkbox in `tasks.md`; Minor findings appended to `BACKLOG.md`.
**Gate (only on exhaustion/no-progress):** presents state to the user — carve the cycle out, extend the bound, or hold; user's call.

## Phase 11 — Repeat Phases 9–10

Until every card for W1, W2, and any FEAT-001 `[MODIFY]` amendment is checked.

## Phase 12 — Gap-finding pass

**Does:** Runs because this is a selection-scope run (not delta/lane, so not skipped). A fresh `devils-advocate` — never a seat that built these cycles or saw the design-time test cases — is dispatched blind, two-message: first message carries only `spec.md`, `sufficiency-report.md`, design deltas, and the baselines' NFR-XXX concern rows (no Screens & Flows section applies — API-only spec); the seat states derived expectations before probing begins. The mutation lens runs on the verification seat (already holding code sight) at high depth, or a stated skip. Spec-required findings fail final validation; beyond-spec findings are advisory and go to the user for disposition (fix now / backlog / accept-as-designed).
**Reads:** as scoped above; code only after the blind first message.
**Writes:** gap-finding findings folded into the final-validation report; fix-now/backlog findings later fold into `.mochiko/features/FEAT-002/gates.md` at landing.

## Phase 13 — Regression sweep

**Does:** Re-runs FEAT-001's durable gate set from `gates.md` (restart-persistence, empty-body 400, get/404) plus any FEAT-002 gates exercising the notes-db seam. A failure here fails the run like any other regression.
**Reads:** `.mochiko/features/FEAT-001/gates.md`.
**Writes:** results folded into the final-validation report.

## Phase 14 — Cold verification

**Does:** Builds and runs the full quality-gate suite from a dependency-cold snapshot of the uncommitted working state, copied via `git ls-files -co --exclude-standard :!.claude/worktrees` into `.claude/worktrees/mochiko-<purpose>/`, after ensuring a `/.claude/worktrees` ignore entry exists.
**Blocker carried from Phase 2/4:** this workspace is not currently a git repository, so `git ls-files` cannot run as designed. This phase is gated on the Phase 4 ruling — either the user authorized `git init` (a repo-creating, low-risk but state-changing action, confirmed in advance) or the DM adopts an alternate cold-snapshot method the user accepts. Absent either, this step halts and reports the blocker rather than faking a result.
**Reads:** working tree.
**Writes:** `.gitignore` entry for `/.claude/worktrees` (if git-init was authorized); ephemeral worktree copy (self-removed, not a ref/history mutation).

## Phase 15 — Acceptance landing (executed whole)

**Does:** Landing verification seat checks every graded fold:
- Store delta folds: elements flip `built`, FEAT-002's key clears, `As-built:`/`Drift:` written and independently graded, orphan check run, `ARCHITECTURE.md` regenerated (never hand-edited).
- `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md` deltas each fold via a three-way diff (pre-fold + delta vs. folded result).
- If clause 9 fired, FEAT-001's marked `[MODIFY]` delta folds into `FEAT-001/entry.md` too.
- `gates.md` gains any fix-now/backlog gap-finding findings, authored in **TEST:** grammar by `qa_seat`.
- Map graduation: W1/W2 fold into FEAT-002's extent, status → `delivered` (dated), `FEATURES.md` index line updates, the note-search spec's closed status is derived (all selected rows folded).
**Reads:** all graded deltas and reports from prior phases.
**Writes:** the fold targets listed above.

## Phase 16 — Final acceptance (USER GATE, floor)

**Does:** Presents the completed landing package and a verdict against the done condition, having checked all 15 fail-conditions clean (sufficiency recorded, design signed, card independence held, no unchecked cards, quality gates passing, evidence real, no regressions, no in-place baseline edits, no unresolved deviation, store landing complete, every fold graded, gap-finding present, no spec-required gap left open, and this gate itself pending).
**Gate — branches:**
- **Accept:** run closes; the DM surfaces rounds consumed and seats spawned, then closes with a PASS verdict.
- **Amend:** user specifies changes; bounded rework (within the gap-rework attempt bound, or the localized cycle's remaining attempts) then re-presented here.
- **Reject:** run closes with a FAIL verdict against the done condition; landing already executed stands as delivered work, but the run itself is recorded not-accepted.

## Phase 17 — Close

**Does:** Reports the final verdict, rounds/seats summary, and — if any fail-condition is still standing at this point — a FAIL closes the run regardless of how far building progressed.
**Writes:** none beyond the reports already produced; this is the terminal narrative summary to the user.