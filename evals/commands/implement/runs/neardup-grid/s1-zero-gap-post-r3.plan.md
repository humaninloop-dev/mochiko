# Action Plan — `/mochiko:implement FEAT-001` (plan-only, not executed)

## Phase 0 — Load the rule schema (already performed as this run's mandatory first action)

- **Done:** Read `plugins/mochiko/schemas/implement.yaml` raw, in full; read `plugins/mochiko/schemas/common.yaml` raw, in full (for every `extends: common.*` stub); read `plugins/mochiko/schemas/command-labels.yaml` for label vocabulary.
- **Read:** the three files above.
- **Verified:** the `kind: fail` node count in `impl.sec.fail-conditions` is 15 (`sufficiency-unrecorded, design-skipped, card-independence, card-unchecked, quality-gate, no-evidence, regression, baseline-in-place, deviation-unresolved, store-landing-incomplete, ungraded-fold, gap-finding-missing, skip-unstated, spec-gap-unresolved, no-acceptance`) — matches the command's hard-coded Not-done count, so no halt is triggered here.
- **Written:** nothing.
- **Seats/skills:** none (Delivery Manager's own reading duty).
- **Gate:** none.

## Phase 1 — Resolve entry and scope type

- **Done:** Resolve `FEAT-001` against `FEATURES.md` and `.mochiko/features/FEAT-001/entry.md`. `FEAT-001` is a plain capability ID, not an `EPIC-XXX`, so this is **selection scope**, not epic scope. Entry carries ratified selected work rows: **W1 — Create a note** (US-001, SC-001/SC-002) and **W2 — Fetch a note by id** (US-002, SC-003), selection source: the spec's accepted selection (2026-08-20). Dependencies line reads "None. First capability on the map." — no blocked rows, no dependency-order stall.
- **Read:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/specs/note-capture/spec.md`.
- **Discovered conditions to carry forward:**
  - No `CLAUDE.md` / governance region present → the `depth` condition (`low`/`high`) cannot be entry-derived. This is an absent surface: surfaced to the user, never auto-resolved, never run-failing (`impl.absent-surfaces`). Any `depth: high`-gated rule (the mutation lens, `impl.mutation-lens`) stays inapplicable/undetermined until it resolves, and that gap is disclosed rather than silently skipped.
  - No `.mochiko/memory/codebase-analysis.md` — expected, since there is no existing source tree at all (true greenfield, not brownfield), so the brownfield branch of `impl.absent-surfaces` does not apply; noted as a non-issue rather than a gap.
  - No `.mochiko/memory/knowledge-management.md` → `impl.km-landing`'s `when: {km_file: present}` never fires this run; no obligation to disclose beyond noting the condition didn't trigger.
  - The workspace is **not a git repository** (confirmed by environment state). `impl.cold-verification` requires `git ls-files -co --exclude-standard` against a real repo to build the dependency-cold snapshot at final validation, and `impl.no-git-mutations` presumes git is in play for suggested commits. This is a load-bearing blocker for a later phase, flagged now rather than discovered late.
- **Written:** nothing.
- **Seats/skills:** none yet — Delivery Manager's own resolution work.
- **Gate:** none yet (feeds the run-open confirmation in Phase 3).

## Phase 2 — Sufficiency check (binding, before any code)

- **Done:** Dispatch the ten-clause sufficiency check (`mochiko:review-sufficiency`) per selected work row (W1, W2) over `spec.md`, the architecture store, and the product baselines. The grading seat must have authored none of those sources — candidate seat: **tech-lead** (authored none of the spec, store, or baselines; independently grades architecture-store judgment writes elsewhere, a natural independence fit) or **validator**. This seat is exempt from plan approval (grading seat).
- **Read (by that seat, per the skill's fence):** `.mochiko/specs/note-capture/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/features/FEAT-001/entry.md`. Never code, `tasks.md`, `**TEST:**` cases, or cycle reports (none exist yet regardless).
- **Concrete per-row read against what's on disk today** (informing, not pre-empting, the grading seat's actual verdict):
  - Clause 1 (testable criteria): SC-001/SC-002 (W1) and SC-003 (W2) are stateable oracles.
  - Clause 2 (contract exposure): `POST /notes` and `GET /notes/{id}` are both already named and locatable in `contracts/api.yaml`.
  - Clause 3 (data exposure): the `Note` entity (`id`, `text`, `created_at`) in `data-model.md` covers both rows.
  - Clause 4 (structural trigger): AX-001 (persistence) and AX-002 (logging) already cover the touched surfaces; no new architecture element appears required — a no-delta claim looks recordable.
  - Clause 5 (NFR targets): AX-001/NFR-001 targets W1 directly. AX-002/NFR-002 ("every 4xx/5xx carries a reason field") applies to both rows' error paths, but `contracts/api.yaml`'s `400` and `404` responses carry no response body schema at all — a plausible **gap**: the NFR names a shape the contract doesn't specify, leaving the builder without an attachable surface for the reason field.
  - Clause 6 (commodity exposure): storage (D-001) and HTTP layer (D-002) are both already resolved with weighed alternatives.
  - Clause 7 (dependency order): resolvable, no in-batch blocking dependency.
  - Clause 8 (UX trace): n/a — spec states no Screens & Flows section.
  - Clause 9 (delivered-feature exposure): n/a — FEAT-001 is the first capability, nothing delivered yet.
  - Clause 10 (in-flight exposure): n/a — no other in-flight feature touches these surfaces.
  - **Trip check:** AX-003 (Auth) is marked `n-a` in v1, not `open`/`not-now`, and isn't touched by W1/W2 — no trip fires.
- **Written:** `sufficiency-report.md` in `.mochiko/features/FEAT-001/` — the store-consult result, any no-delta claim, the plausible NFR-002/contract-shape gap (or its clearance, if the grading seat judges the plain-text 400/404 descriptions sufficient), trips (none expected), and a stated `quickstart.md` null path (no external-integration surface exists here).
- **Seats/skills:** tech-lead (or validator) running `mochiko:review-sufficiency`.
- **Gate:** the verdict is binding but not itself a user-blocking gate — a disputed clause (if the grading seat can't clear one) defaults to gap and is routed to the user at run-open (`impl.sufficiency-disputed-clause`).

## Phase 3 — Run-open confirmation (the entry gate — user's)

- **Done:** Present one confirmation, no negotiation:
  - Batch and scope type: FEAT-001 "Note capture", selection scope, rows W1 + W2.
  - Attempt bounds at their only redeclaration point: `attempt_bound_cycle = 3` (per-cycle verification attempts), `gap_rework_bound = 2` (gap-rework rounds at final validation) — carried as-is from `implement.yaml`'s `vars:` unless the user redeclares them here.
  - The sufficiency verdict and its gap routing (from Phase 2) — most likely "sufficient" on both rows with one named NFR-002/contract-shape gap, or fully sufficient if the grading seat clears clause 5 against the plain-text descriptions.
  - Trips and conflicts for the user's ruling: none found on the map/store side; but the **no-git-repository condition** discovered in Phase 1 is surfaced here as an in-flight blocker for `impl.cold-verification`, since that's a floor obligation the run cannot silently skip.
  - The done condition, stated plainly: every cycle card `[x]`, built test-first, independently verified against real infrastructure per-cycle and whole, code meeting SC-001/SC-002/SC-003 and FR-001–004, aligned to governance (none declared, so vacuously), acceptance landing executed whole, run closed at final accept/amend/reject, with none of the 15 fail-conditions standing.
- **Read:** nothing new — presents Phase 1/2 outputs.
- **Written:** nothing (the confirmation itself is not a file write; `sufficiency-report.md` from Phase 2 is already on disk).
- **Gate — reserved to the user (`impl.gate-*` / `impl.user-runopen-rulings`):**
  - **If the user approves as stated:** proceed to Phase 4 (or Phase 5 directly if the verdict was zero-gap).
  - **If the user disputes the sufficiency verdict or names the disputed clause differently:** that clause is treated as a gap and folds into Phase 4's design-phase scope.
  - **If the user rules on the no-git-repo blocker:** two live branches — (a) initialize a git repository now so `impl.cold-verification` can run as specified later, or (b) accept a stated, disclosed deviation from `impl.cold-verification` for this run, recorded as a ruling rather than a silent skip. Both are legitimate outcomes; only a silent skip is not.
  - **If the user redeclares attempt bounds:** the new values replace the defaults for the remainder of this run only.
  - **If the user declines to open the run:** the plan halts here; nothing downstream fires.

## Phase 4 — Design phase (fires only if Phase 2/3 named a gap)

- **Condition:** fires only over the exact named gaps (e.g., the NFR-002 error-response-shape gap, if it survived run-open as a gap rather than being cleared).
- **Done:** A design-class seat — likely **technical-analyst** (`design_seat`) for a contract/NFR-shape delta, since no structural/store delta is implicated — authors exactly that gap on a Delivery-Manager-approved plan, applying the simplest-execution ladder (`mochiko:patterns-plan-minimalism`): e.g., adding a minimal error-body schema (`{"error": "..."}`) to `contracts/api.yaml`'s 400/404 responses and stating how it satisfies NFR-002, nothing more.
- **Read:** `sufficiency-report.md`, `spec.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/architecture/spine.md` (to confirm no structural trigger fires).
- **Written (as deltas beside baselines, never in place):** `.mochiko/features/FEAT-001/contracts-delta.md` (or equivalent appliable before/after delta) — no architecture-store delta is expected here since no box/arrow/element changes.
- **Reviewed before the checkpoint (non-author seat, `impl.design-review-pair`):** `mochiko:review-plan-artifacts` (conformance to the gap list, blocking) and `mochiko:review-feasibility` (buildability/contradiction) — candidate reviewer: **qa-engineer** or **tech-lead** (not the author).
- **Design-map assertion:** the design phase asserts any sharpened extent/dependencies onto `.mochiko/features/FEAT-001/entry.md` with provenance (`mochiko:authoring-feature-map`) — expected to be a no-op here since no new dependency surfaces.
- **Gate — the design checkpoint, the user's (`impl.gate-design-checkpoint`):**
  - **If the user signs the design and delta as presented:** proceed to Phase 5.
  - **If the user asks for revision:** the design seat reworks within the same named-gap scope (still counted under the design phase, not a build-cycle attempt).
  - **If the user stops here:** the run may pause; it can resume the build later without redoing Phases 1–4.
  - (No store delta exists in this scenario, so the "sign on a rendered diagram plus AX-XXX row changes" clause of this rule doesn't engage — a source-plus-changed-table presentation isn't needed either, since there is no changed table.)

## Phase 5 — Cycle-card authoring

- **Done:** A design-class, non-builder seat (candidate: **qa-engineer**, since it also authors the `**TEST:**` cases within its slicing, or **technical-analyst**) slices W1 and W2 into cycle cards per `mochiko:patterns-vertical-tdd` — walking skeleton first. Expected shape: a foundation cycle (HTTP server skeleton + SQLite schema/connection, no user-visible behavior yet) followed by a W1 cycle (create note) and a W2 cycle (fetch note), each demonstrable independently. If Phase 4 did not fire (zero-gap path), this same seat also makes the map-entry assertion the design phase would have made (`impl.zero-gap-map-assertion`).
- **Read:** `spec.md`, `sufficiency-report.md`, any Phase-4 deltas, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `plugins/mochiko/schemas/tasks.yaml` (schema fallback if `mochiko-cli template tasks` is unavailable).
- **Written:** `.mochiko/features/FEAT-001/tasks.md` — cycle cards carrying stories/rationale, dependencies, acceptance criteria by ID (SC-001/002/003, FR-001–004), a `**TEST:**` real-infrastructure gate per card, and brownfield exposure (expected: `[NEW]` throughout, since there is no existing code) — no task lists or file paths (builder decomposes those at build time).
- **Reviewed before card confirm (verification seat, non-author, `impl.card-review-before-confirm`):** quality per `mochiko:review-plan-artifacts`, buildability by the reviewer's own judgment — candidate: **staff-engineer is excluded** (it will build these cards); reviewer is qa-engineer or tech-lead, whichever didn't author the cards.
- **Gate — the card confirm, the user's (`impl.gate-card-confirm`):**
  - **If the user approves the slicing:** proceed to Phase 6.
  - **If the user wants different slicing (e.g., merge W1/W2 into one cycle, or split the foundation cycle further):** the authoring seat re-slices before any build starts; no code has been written yet, so this costs no build attempts.
  - **If the user rejects entirely:** halts before Phase 6; earlier phases stand.

## Phase 6 — Build cycles (test-first, foundation before feature)

- **Done:** **staff-engineer** (`builder_seat`) executes each confirmed card in order via `mochiko:executing-tdd-cycle`: decomposes the card into concrete tasks at build time (disclosed in the cycle report), applies the pre-code minimalism ladder at decomposition (`mochiko:patterns-code-minimalism`, rungs disclosed — e.g., "stdlib HTTP + stdlib SQLite driver, no framework" per D-002), and drives red→green→refactor test-first. Since every file is `[NEW]`, `mochiko:brownfield-integration` doesn't engage for this batch. Builder works only on a plan the Delivery Manager approved; never designs its own gaps.
- **Read (by the builder):** the confirmed `tasks.md` card, `data-model.md`, `contracts/api.yaml` (plus its Phase-4 delta if any), `constraints-and-decisions.md`.
- **Written:** the actual application source (HTTP handlers, SQLite persistence layer, note model) — paths not yet fixed since no source tree exists; a `cycle-report.md` per cycle in `.mochiko/features/FEAT-001/` (decomposition, honest difficulties, deviations, `domain_deps_added`); `tasks.md` checkboxes flipped `[x]` as the progress surface as each cycle completes.
- **Per-cycle verification (independent seat, never the builder, `impl.seat-verification-independence`):** candidate **qa-engineer** runs the card's `**TEST:**` gate against real infrastructure (`mochiko:testing-end-user` — a real SQLite file, real HTTP requests, never mocked) and the `mochiko:review-code-minimalism` lens over the diff + cycle report + surrounding code (advisory findings only, never gating).
- **Attempt economy (floor, `impl.attempt-per-grade`, `impl.no-progress-stop`):** each grading round of a cycle consumes one of the 3 default attempts; two consecutive rounds with unchanged findings halts that cycle and presents state rather than continuing to burn attempts.
- **Escalation batching:** non-build-blocking reserved-to-user questions accumulate to the cycle checkpoint; a build-blocking one (e.g., an infeasible card, a commodity-category or IP-XXX call) interrupts immediately and routes to the user per `impl.infeasible-card-escalation` / `impl.adopt-first-user-call`.
- **Gate — cycle checkpoint (per cycle, batched):**
  - **If findings are clean / Minor-only:** Minor findings book to `BACKLOG.md` (not written by this run unless one exists — none does currently, so this would be a first creation flagged to the user), cycle proceeds.
  - **If an Important-or-above finding lands:** blocks the cycle, joins the checkpoint batch for user ruling — fix now vs. other disposition.
  - **If attempts exhaust or no-progress trips:** halts the cycle, presents state; disposition (retry with guidance, descope, or abandon this row for the run) is the user's.

## Phase 7 — Final validation (whole-build)

- **Done:**
  - **Quality gates:** full repository suite run as exit-code checks (never severity-triaged — any failure fails the run, `impl.fail.quality-gate`).
  - **Regression sweep:** accumulated `**TEST:**` gates of previously delivered features in this territory — none exist (FEAT-001 is first), so this sweep is a stated no-op rather than a silent skip.
  - **Cold verification:** build and run quality gates from a dependency-cold snapshot of the uncommitted working state via `git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-<purpose>/`. **This is the point where Phase 3's git-repo ruling resolves in practice:** if the user chose to `git init` at run-open, this executes as specified; if the user accepted a disclosed deviation, this step runs the alternative the user ruled on instead, recorded as such (never silently skipped).
  - **Gap-finding pass:** fires because scope is `selection` (`impl.gap-finding-scope`). A **fresh devils-advocate** (`gap_finder_seat`), never having built these cycles or seen the design-time test cases, receives a first message with only `spec.md`, `sufficiency-report.md`, any design deltas, and the baselines (`data-model.md`, `contracts/`, store NFR-XXX rows) — never code, `tasks.md`, `**TEST:**` cases, or reports — states derived expectations, then probes the running system blind.
- **Read:** the accumulated artifacts above; the built code and its evidence (by the quality-gate/cold-verification steps, not by the gap-finder in its blind first message).
- **Written:** `final-validation-report.md` in `.mochiko/features/FEAT-001/` — gate results, regression-sweep result (stated no-op), cold-verification evidence, gap-finding findings split by kind.
- **Findings routing:**
  - Spec-required behavior broken → evidence captured, clause cited, fails final validation, must be resolved (charges the gap-rework bound, or the owning cycle's remaining attempts if localized).
  - Beyond-spec finding → advisory, disposition reserved to the user: fix now / `BACKLOG.md` / accept as designed.
  - Disputed finding kind → defaults advisory, goes to the user.
- **Gate — cycle-checkpoint-style batch for final-validation findings:**
  - **If clean:** proceed to Phase 8.
  - **If spec-gaps found:** route back into a bounded gap-rework round (default 2 rounds total for the run, `impl.gap-rework-bound`); exhaustion or unchanged-findings halts the run and presents state, disposition the user's.
  - **If only beyond-spec findings:** proceed to Phase 8 once the user has ruled each disposition.

## Phase 8 — Landing (acceptance-time execution, whole)

- **Done:**
  - **Store landing:** only engages if Phase 4 produced a signed store delta — not expected in this scenario (no structural change), so this executes as a no-op check rather than a fold.
  - **Selection-scope landing (`impl.landing-selection`):** W1 and W2 fold into FEAT-001's extent lines and vanish as pending rows; entry status set `delivered`, dated; `FEATURES.md` index line for FEAT-001 updates from "selected" to "delivered"; the spec's specs-index row reads closed since all its selected rows (W1, W2) have folded.
  - **Graded fold:** any touched baseline (the Phase-4 contracts delta, if it fired) folds exactly once via a three-way diff, checked by the landing verification seat (not the design author).
  - **Gates fold:** any final-validation gap findings ruled fix-now or backlog fold into `.mochiko/features/FEAT-001/gates.md` (minted fresh here, since none exists), authored by qa-engineer in the `**TEST:**` grammar; findings accepted as designed do not fold.
- **Read:** `sufficiency-report.md`, Phase-4 deltas (if any), `final-validation-report.md`, `.mochiko/features/FEAT-001/entry.md`, `FEATURES.md`.
- **Written:** `.mochiko/features/FEAT-001/entry.md` (status/extent fold), `FEATURES.md` (index line), `contracts/api.yaml` fold (if a delta existed), `.mochiko/features/FEAT-001/gates.md` (minted).
- **Seats:** landing verification seat (qa-engineer or tech-lead, independent of whoever authored the folded content) checks the graded folds.
- **Gate:** none new here — landing executes as one whole unit once Phase 7 clears; the ruling gate is Phase 9.

## Phase 9 — Final acceptance (closes the run — user's)

- **Done:** Present the completed landing, the final-validation evidence, and the verdict against the done condition as plain blocking text (never a timed prompt).
- **Gate — final acceptance, the user's (`impl.gate-final-acceptance`):**
  - **Accept:** run closes DONE — all 15 fail-conditions checked absent, done condition met.
  - **Amend:** the user names a specific change; scoped rework re-enters at the smallest phase that covers it (a build-only fix re-enters Phase 6/7; a scope change may re-open Phase 3).
  - **Reject:** run closes NOT DONE against `impl.fail.no-acceptance`; state is preserved for a future run rather than discarded.
- **Written:** none beyond what Phase 8 already wrote; the Delivery Manager's closing statement is conversational, citing the verdict.

## Phase 10 — Close with verdict

- **Done:** State the run's outcome against the done condition explicitly, re-confirm no `kind: fail` rule stands (or name which one does, if the run halted earlier instead of reaching acceptance), and surface rounds/seats consumed across the run (`impl.dm-surface-rounds`).
- **Written:** nothing beyond prior phases — this is the reporting duty, not a new artifact.
- **Gate:** none — this is the run's own closing act, contingent entirely on Phase 9's ruling.