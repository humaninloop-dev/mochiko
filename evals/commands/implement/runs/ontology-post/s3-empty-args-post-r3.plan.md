# Action Plan — `/mochiko:implement` (no argument)

Based on the governing schema (`plugins/mochiko/schemas/implement.yaml`, `common.yaml`, `command-labels.yaml` — all read in full) and the current workspace state (`FEATURES.md`, `.mochiko/features/FEAT-001,FEAT-002`, `.mochiko/specs/note-capture,note-search`, `.mochiko/product/*`, `ARCHITECTURE.md`; no `CLAUDE.md`, no `.mochiko/memory/`, no `.claude/`, no `BACKLOG.md`, no git repository, and no application source code anywhere in the tree).

## Phase 0 — Load the governing schema
**Does:** Raw, full read of `plugins/mochiko/schemas/implement.yaml` and `plugins/mochiko/schemas/common.yaml` (required in the same first action per the command's own rule), plus `command-labels.yaml` for label resolution. Already performed as part of scoping this plan.
**Reads:** the three schema files above.
**Writes:** none.
**Gate:** none.

## Phase 1 — Entry fact-finding & batch resolution
**Does:** Resolve `$ARGUMENTS` (empty here) → propose the next ready capability. `FEATURES.md` shows FEAT-001 `delivered`, FEAT-002 `selected`. FEAT-002's entry carries two ratified work rows (W1 — search by query, W2 — index freshness), selection source = the note-search spec's accepted selection (2026-08-26), dependency on FEAT-001 which reads `delivered` so it does not block on paper.
**Reads:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/features/FEAT-001/gates.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/specs/note-search/spec.md`, `.mochiko/specs/note-capture/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `ARCHITECTURE.md`; presence-checks for `CLAUDE.md`/governance region, `.mochiko/memory/knowledge-management.md`, `.mochiko/memory/codebase-analysis.md`, `.claude/rules/mochiko/`, `BACKLOG.md`, and git-repo presence.
**Findings surfaced (not auto-resolved):**
- Governance region absent — surfaced, not run-failing.
- `codebase-analysis.md` absent — offer `/mochiko:setup` or proceed greenfield with a logged warning.
- **No git repository exists**, and **no application source code exists anywhere in the workspace**, despite FEAT-001 being marked `delivered` with a durable gate set. This is a genuine in-flight conflict, not a cosmetic gap — it threatens the regression sweep (Phase 7/8) and the cold-verification mechanism (Phase 8), both of which assume FEAT-001's code and a git tree exist.
**Writes:** none.
**Gate:** none yet — these findings are carried into the Phase 3 run-open confirmation for the user's ruling, per the reservation that absent surfaces and in-flight conflicts are never auto-resolved.

## Phase 2 — Sufficiency check (moment: `entry`)
**Does:** Dispatch a grading seat that authored none of `spec.md`, `spine.md`, or the product baselines (DM's staffing call — e.g. `validator` or `tech-lead`, per `mochiko:review-sufficiency`) to grade each selected row (W1, W2) on the ten-clause check, never reading code or `tasks.md`.
**Reads (by that seat):** `.mochiko/specs/note-search/spec.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`.
**Likely gap indicators already visible from Phase 1 (to be confirmed by the grading seat, not asserted here):**
- Architecture-store gap: FR-103 requires a background index worker; the ruled spine states "no queues, no background workers," and FEAT-002's entry shows no architecture link at all — a structural gap.
- Technology-decision gap: no `D-XXX` row commits to a search-indexing approach; this is also an adopt-first commodity-category question (e.g. SQLite FTS5 vs. custom).
- API-contract gap: `contracts/api.yaml` has no `/notes/search` path.
**Writes:** `.mochiko/features/FEAT-002/sufficiency-report.md` — per-row verdict, store-consult result, trips for run-open, quickstart.md null-path note, any `[MODIFY]` amendment (none expected).
**Gate:** none directly — a disputed clause defaults to gap and rides into run-open.

## Phase 3 — Run-open confirmation (moment: `run-open`) — USER GATE
**Does:** One blocking confirmation, no negotiation on content: states the batch (FEAT-002, rows W1+W2) and scope type (selection); restates both attempt bounds (3 per-cycle verification attempts, 2 gap-rework rounds — this run's only redeclaration point); presents the Phase 2 sufficiency verdict and gap routing; presents the Phase 1 trips (the FEAT-001-delivered-but-no-code conflict, absent governance region, absent codebase-analysis.md, absent git repo) for the user's ruling; states the done condition.
**Reads:** Phase 1/2 outputs only.
**Writes:** none (the confirmation itself is recorded as part of the sufficiency report / run narrative, not a new file).
**Gate — what's confirmed:** the batch/scope, the two attempt-bound numbers (or the user's override), and rulings on each surfaced trip/conflict.
- **Confirm as proposed** → proceed to Phase 4 with default bounds and the Phase 2 gap list.
- **User changes an attempt bound** → proceed with the new number; no further redeclaration point exists this run.
- **User rules the FEAT-001/no-code conflict and absent surfaces** (e.g. "treat as fresh build," "run `/mochiko:setup` first," "proceed greenfield, warning logged") → proceed to Phase 4 under that ruling.
- **User rejects the proposed batch / wants a different one** → run does not open; route per the entry rule — a new capability to `/mochiko:specify`, a feature-keyed delta to `/mochiko:feature`.
- **User pauses** → run stays unopened; nothing downstream executes.

## Phase 4 — Design phase (moment: `design-checkpoint`) — fires on any named gap
**Does:** Given Phase 2's likely gaps, this phase is expected to fire. Staffing (DM's latitude): `technical-analyst` for the search FR/decision deltas and API-contract delta, `principal-architect` for the store delta (background-worker element + NFR row, a structural change), `qa-engineer` for design-time `**TEST:**` case shapes. Each seat works only on a DM-approved plan, scoped to exactly the named gaps (rung-justified per `mochiko:patterns-plan-minimalism`).
**Reads:** `sufficiency-report.md`, `spec.md`, product baselines, `spine.md`.
**Writes (deltas beside baselines, never in place):** `.mochiko/features/FEAT-002/data-model-delta.md` (if the index needs a first-class entity), `.mochiko/features/FEAT-002/contracts/api-delta.yaml` (adds `GET /notes/search`), `.mochiko/features/FEAT-002/constraints-and-decisions-delta.md` (new `D-XXX` for the indexing technology — subject to an adopt-first ruling, reserved to the user), an architecture-store delta (in-flight-class elements only) plus a delta diagram per `mochiko:patterns-system-design`, and the design-implied assertion onto `FEAT-002/entry.md`'s architecture link/extent lines with provenance.
**Review:** a non-author seat runs `mochiko:review-plan-artifacts` (conformance/blocking) and `mochiko:review-feasibility` (buildability/contradiction, including the architecture pass) before the checkpoint.
**Gate — what's confirmed:** the rendered delta diagram (or source + changed-AX-XXX-row table) and the design deltas.
- **Sign as presented** → deltas stand unfolded; proceed to Phase 5.
- **Amend** (e.g. reject the FTS5 candidate, redirect the worker's shape) → design seat reworks under the same grade/checkpoint before proceeding.
- **Stop here, resume later** → run pauses with the (un)signed state preserved; no code written.
- If an adopt-first or `IP-XXX` call surfaces mid-design, it halts to this same checkpoint rather than being seat-decided.

## Phase 5 — Cycle-card authoring & card confirm (moment: `card-confirm`) — USER GATE
**Does:** A design-class, non-builder seat slices the signed design into cycle cards per `mochiko:patterns-vertical-tdd` — foundation cycle (index-worker/storage plumbing) before feature cycles (W1 search endpoint, W2 freshness) — `qa-engineer` authors each card's `**TEST:**` gate citing SC-101–103.
**Writes:** `.mochiko/features/FEAT-002/tasks.md` (from the tasks template, or `plugins/mochiko/schemas/tasks.yaml` raw if the binary is absent) — no task lists or file paths in the cards.
**Review:** the verification seat reviews quality (`mochiko:review-plan-artifacts`) and buildability before the confirm.
**Gate — what's confirmed:** the cycle slicing.
- **Approve** → proceed to Phase 6.
- **Request re-slicing** → card-authoring seat revises and re-presents.
- **Defer/pause** → run holds with cards unbuilt.

## Phase 6 — Build & per-cycle verification loop (moment: `cycle-checkpoint`), repeated per card
**Does, per card:** `staff-engineer` (builder) decomposes into concrete tasks at build time (disclosed in the cycle report), applies `mochiko:brownfield-integration` on any touch to FEAT-001's existing note-storage code, applies `mochiko:patterns-code-minimalism` at decomposition, drives red→green→refactor per `mochiko:executing-tdd-cycle`. An independent seat (e.g. `qa-engineer`) then executes the card's `**TEST:**` gate against real infrastructure (`mochiko:testing-end-user`), runs the full quality-gate suite, and applies the advisory `mochiko:review-code-minimalism` lens.
**Writes:** source/test code; `.mochiko/features/FEAT-002/cycle-report-<n>.md`; `.mochiko/features/FEAT-002/verification-report-<n>.md`; flips the card's `[x]` in `tasks.md` only on a clean grade.
**Attempt economy:** 3 attempts per cycle (or the Phase-3 redeclared number); two consecutive unchanged-finding rounds is a no-progress stop; bound exhaustion halts and presents state, disposition reserved to the user.
**Gate (conditional, fires only if triggered):** an architecture-delta deviation, an infeasible card, or a constraint collision each stop that cycle and present to the user.
- **Build as approved / amend the delta first / accept escalated scope / carve the card out and continue** — whichever the user rules, that's the onward branch.
**Midrun refire:** undesigned structure discovered mid-build halts that cycle and re-fires Phase 4 scoped to the discovery, same grade/checkpoint.

## Phase 7 — Flagging the FEAT-001 regression risk (folded into Phase 8, called out separately here)
**Does:** Names explicitly, before final validation runs, that FEAT-001's durable gate set (`gates.md`, 3 `**TEST:**` cases) must re-pass against real infrastructure — and since Phase 1 found zero source code anywhere in this workspace, this sweep is where the "delivered but no code" conflict becomes concrete. Either the Phase-3 ruling already authorized rebuilding FEAT-001's own implementation as part of this run's scope, or this sweep fails outright as a regression. No plan step here papers over that outcome.
**Reads/Writes:** none new — this is a risk flag threaded from Phase 1/3 into Phase 8.
**Gate:** none directly (the outcome lands in Phase 8/10).

## Phase 8 — Final validation (moment: `final-validation`)
**Does:**
- **Cold verification:** build/run the full quality-gate suite from a dependency-cold snapshot (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-<purpose>/`). **Blocker:** this workspace is not a git repository — there is no `git ls-files` output to snapshot. If unresolved earlier at Phase 3, this re-surfaces here as a hard blocker: final validation cannot close without the user resolving it (e.g. authorizing repo initialization, or ruling an alternative evidence path).
- **Regression sweep:** re-run FEAT-001's `gates.md` cases plus any cases on its own cards; run FEAT-002's own gates against any already-delivered seam.
- **Gap-finding pass** (fires — scope is selection): a fresh, blind `devils-advocate` seat, two-message dispatch — first message carries only `spec.md`, `sufficiency-report.md`, Phase-4 deltas, and the baselines, never code/tasks.md/TEST cases/reports; states derived expectations before probing.
- Findings split by kind: spec-required-behavior-broken fails the pass until resolved; beyond-spec is advisory, disposition reserved to the user (fix now / book to `BACKLOG.md` — which doesn't exist yet and would need creating via `mochiko:grooming-operating-docs` conventions — / accept as designed).
- Gap-rework bound: 2 rounds (or Phase-3's number) at run scope, with cycle-local findings charged to that cycle's own remaining attempts instead; exhaustion or unchanged findings halts and presents state, disposition reserved to the user.
**Writes:** `.mochiko/features/FEAT-002/final-validation-report.md`.
**Gate:** none standalone — outcomes (including the cold-verification blocker and any regression) roll into Phase 10.

## Phase 9 — Landing (moment: `landing`)
**Does, executed whole only at acceptance:** graded three-way-diff folds of the Phase-4 baseline deltas into `.mochiko/product/*` and the signed architecture-store delta into `spine.md` (status flips, As-built:/Drift: written and independently graded, orphan check run, `ARCHITECTURE.md` regenerated — never hand-edited); the map graduation batch folds W1/W2 into FEAT-002's extent, flips its status to `delivered` (dated), updates `FEATURES.md`, and closes the note-search spec's index row; gap findings ruled fix-now/backlog fold into a freshly minted `.mochiko/features/FEAT-002/gates.md`. Knowledge-management landing is skipped (no `knowledge-management.md` present).
**Reads/Writes:** as above, all in `.mochiko/product/`, `.mochiko/features/FEAT-002/`, `FEATURES.md`, `ARCHITECTURE.md`.
**Gate:** none directly — this executes only after Phase 10's accept ruling; described here for completeness per the landing-whole duty.

## Phase 10 — Final acceptance (moment: `acceptance`) — USER GATE, closes the run
**Does:** Presents the verdict against the done condition — card status, verification evidence (or the unresolved cold-verification blocker), regression-sweep result (including the FEAT-001-code-discrepancy outcome), gap-finding result, landing summary, and an explicit check of all 15 `kind: fail` conditions in `impl.sec.fail-conditions` (count verified to match the command's hard-coded "15" — no halt-and-surface needed).
**Reads:** all prior phase outputs.
**Writes:** none beyond what Phase 9 already landed.
**Gate — what's confirmed:** accept / amend / reject, plain blocking text.
- **Accept** → run closes done; DM states the verdict.
- **Amend** → user specifies the change; DM routes back to the relevant phase (cycle rework, finding-disposition change, landing correction) and re-presents Phase 10.
- **Reject** → run closes not-done; DM records which of the 15 fail conditions stood (if any) and why.