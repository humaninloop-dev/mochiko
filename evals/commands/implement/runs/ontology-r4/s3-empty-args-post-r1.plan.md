Schema check confirms the fail-set count matches (15 `kind: fail` rules), so the run is not halted on that account. Repo state: `FEAT-001` (note capture) is `delivered` with a durable gate set; `FEAT-002` (note search) is `selected` with two ratified work rows (W1, W2) and a satisfied dependency (FEAT-001 delivered); no `EPIC-XXX`, no delta card, no governance region/CLAUDE.md, no `.mochiko/memory/`, and no `.git` or product source tree exist. Below is the full plan for this invocation.

---

# Action Plan — `/mochiko:implement` (no `$ARGUMENTS`)

## Phase 0 — Load the binding schema (mandatory first action)
- **Done:** Read `plugins/mochiko/schemas/implement.yaml` and `plugins/mochiko/schemas/common.yaml` raw, in full (already executed above), plus `command-labels.yaml` for label resolution. Verified the `kind: fail` count is 15, matching the `.md`'s hard-coded Not-done count — no sync halt needed.
- **Read:** the three schema files above.
- **Written:** nothing.
- **Seats/skills:** none — Delivery Manager (main loop) only.
- **Gate:** none.

## Phase 1 — Entry resolution
- **Done:** Resolve `$ARGUMENTS`. It is empty and does not name an `EPIC-XXX`, so per `impl.dm-entry-gate` the run must propose the next ready capability from `FEATURES.md`. `FEAT-001` is `delivered` (ineligible); `FEAT-002` is `selected`, carries ratified work rows W1 and W2 (selection source: the spec's accepted selection, 2026-08-26), and its one dependency (`FEAT-001`) is `delivered` — not blocked. `FEAT-002` is proposed as the batch, scope = **selection**.
- Note two environment facts to carry forward rather than resolve unilaterally: (a) no product source tree, `.git`, or `.mochiko/memory/codebase-analysis.md` exist even though `FEAT-001` is marked delivered — an absent/stale-surface situation; (b) no governance region/`CLAUDE.md` and no `${rules_dir}` exist.
- **Read:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/specs/note-search/spec.md` (already read above).
- **Written:** nothing yet.
- **Seats/skills:** none — main-loop judgment; a `Explore`/haiku-tier lookup would normally do this enumeration per `impl.model-tiering`, but the map is small enough it was read directly.
- **Gate:** none yet — proposal is confirmed together with sufficiency results at the run-open confirmation (Phase 3), per the protocol's "propose... and confirm with the user."

## Phase 2 — Sufficiency check
- **Done:** Dispatch an independent grading seat that has authored none of the spec, the architecture store, or the product baselines, and that will not later design or build this batch (`impl.seat-sufficiency-independence`) — e.g. `mochiko:validator`, run per `mochiko:review-sufficiency`. It grades each selected row (W1, W2) on the ten-clause check against `spec.md`, the architecture spine's concern catalog, and the product baselines.
- Concrete gaps this grading would almost certainly surface:
  - No API contract for `GET /notes/search` in `contracts/api.yaml` (only `POST /notes` and `GET /notes/{id}` exist) — gap against W1.
  - No data-model coverage for a searchable/indexed representation of `Note` — gap against W1/W2.
  - The architecture spine states "no background workers," but FR-103/W2 requires a background index worker — a direct contradiction between ruled architecture and required scope, not yet a signed-delta deviation (nothing is signed yet) but a hard sufficiency gap requiring an architecture-store delta.
  - No technology decision for the search mechanism (commodity category: full-text search) — triggers `mochiko:patterns-adopt-first` scrutiny later, and no NFR/concern row for the 2-second freshness bound (SC-103).
  - Separately: the delivered/no-code discrepancy from Phase 1 is not itself a sufficiency clause (the skill never reads code) but is logged as a trip for run-open.
- **Read:** `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/contracts/api.yaml`.
- **Written:** `.mochiko/features/FEAT-002/sufficiency-report.md` — per-row verdict (W1: gap list; W2: gap list), the store-consult result, trips for the user, the `quickstart.md` null-path note (no real external-integration surface), no `[MODIFY]` amendments needed.
- **Seats/skills:** `mochiko:validator` (grading), pointer skill `mochiko:review-sufficiency`.
- **Gate:** none directly — the verdict is binding but surfaces at the next gate.

## Phase 3 — Run-open confirmation (GATE, entry)
- **Done:** Present one blocking, plain-text confirmation (no negotiation, no timed prompt) that:
  - Names the batch (`FEAT-002`, scope: selection) and confirms it is the intended target (resolving the empty-`$ARGUMENTS` proposal from Phase 1).
  - Restates both attempt bounds at their only redeclaration point: per-cycle verification attempts = 3 (`attempt_bound_cycle`), gap-rework rounds at final validation = 2 (`gap_rework_bound`) — offering the user the chance to redeclare either.
  - Presents the sufficiency verdict and gap routing from Phase 2 (API contract gap, data-model gap, architecture "no background workers" contradiction, missing search technology decision) — all routed to the design phase.
  - Surfaces the absent-surface trips reserved to the user (`impl.user-runopen-rulings`, `impl.absent-surfaces`): no governance region found; no `.mochiko/memory/codebase-analysis.md` and no visible source tree despite `FEAT-001`'s delivered status.
  - States the done condition (every cycle card checked, test-first, independently verified per-cycle and whole; landing executed whole; run closes at accept/amend/reject; none of the 15 fail conditions standing).
- **Read:** nothing new — restates Phases 1–2 outputs.
- **Written:** nothing yet (the confirmation itself is conversational).
- **Seats/skills:** none — Delivery Manager only.
- **Gate — what is confirmed:** batch identity/scope, attempt bounds, gap routing, and disposition of the two absent-surface trips.
  - **Ruling: confirmed as proposed →** proceed to Phase 4 (design phase) exactly as scoped.
  - **Ruling: user redeclares attempt bounds →** carry the new bounds forward into Phases 8 and 9 instead of the defaults; no other change.
  - **Ruling: user disputes the target capability (wants a different one, or none is ready) →** re-run Phase 1 against the user's stated target, or halt the run with no batch open.
  - **Ruling: user treats the no-code/no-`.git` situation as brownfield needing analysis first →** pause this run and route to `/mochiko:setup` for `.mochiko/memory/codebase-analysis.md`, then resume entry once that lands.
  - **Ruling: user says proceed greenfield with the warning logged (code is out of view / stale metadata) →** log the warning in the sufficiency report addendum and continue to Phase 4 without a setup detour.

## Phase 4 — Design phase (fires — gaps were named)
- **Done:** Because Phase 2 named gaps, the design phase fires scoped to exactly those gaps, nothing more (`impl.design-gaps-only`, `impl.design-phase-fires-on-gap`). Each design seat works only on a Delivery-Manager-approved plan (design/grading/verification seats are exempt from plan approval, but design-class producer seats are not exempt from *this* rule — they plan first).
  - `technical-analyst` (`design_seat`) authors: an API-contract delta adding `GET /notes/search` (query param `q`, 200/400 responses) to `contracts/api.yaml`; a data-model delta covering the searchable representation; a `constraints-and-decisions.md` delta adding a D-XXX row for the search mechanism, run through `mochiko:patterns-adopt-first` (commodity category: full-text search) — SQLite FTS5 (bundled, in-process, consistent with C-001) versus a hand-rolled substring scan versus an external search service (excluded by C-001). The adopt-first ruling itself is reserved to the user (`impl.adopt-first-user-call`) — surfaced at the design checkpoint, not decided by the seat.
  - `principal-architect` (`architect_seat`) authors the architecture-store delta: a new in-flight element (background index-worker, in-process, same container as `api-service`, not a separate deployed service — keeping it inside C-001's single-process boundary), a new AX row with an NFR-XXX target for the ≤2s freshness bound (SC-103), and the amendment to the spine's "no background workers" note that this delta implies. This delta is drafted beside the baseline, never edited in place (`impl.baselines-never-in-place`), and is the one legal in-flight-class store write allowed pre-landing.
  - `qa-engineer` (`qa_seat`) authors the design-time **TEST:** acceptance cases for W1/W2 within the above slicing.
  - All design outputs land at `.mochiko/features/FEAT-002/` beside their baselines (delta form), plus the store delta at `${product_dir}/architecture/` per `mochiko:authoring-architecture-store`.
- Independent review pair grades the package before the checkpoint: `mochiko:review-plan-artifacts` (conformance to the exact gap list, card-quality-adjacent completeness — blocking) and `mochiko:review-feasibility` (buildability/contradiction — specifically whether the background-worker delta actually resolves the "no background workers" conflict without breaking C-001, and whether FTS5 is buildable under the stdlib-driver constraint).
- Multi-seat composition (validator, technical-analyst, principal-architect, qa-engineer all active) trips the transport floor (`impl.transport-floor`) — message legs for any inter-seat relay, topology/single-writer discipline on the shared architecture-store write.
- **Read:** `sufficiency-report.md`, `spec.md`, current `contracts/api.yaml`, `data-model.md`, `constraints-and-decisions.md`, `architecture/spine.md`.
- **Written:** `.mochiko/features/FEAT-002/contracts-delta` (or equivalent appliable-diff form), `.mochiko/features/FEAT-002/data-model-delta`, `.mochiko/features/FEAT-002/baseline-delta.md` (the D-XXX technology decision), the architecture-store in-flight delta under `.mochiko/product/architecture/`, `.mochiko/features/FEAT-002/design-review.md` (the two review verdicts).
- **Seats/skills:** `technical-analyst`, `principal-architect`, `qa-engineer` (producers); `mochiko:review-plan-artifacts`, `mochiko:review-feasibility` (independent reviewers); pointer skills `mochiko:patterns-adopt-first`, `mochiko:authoring-architecture-store`, `mochiko:patterns-system-design`, `mochiko:patterns-plan-minimalism`.
- **Gate:** none inside this phase — the checkpoint is next.

## Phase 5 — Design checkpoint (GATE, floor)
- **Done:** Present the rendered architecture-store delta diagram plus its named AX-row changes (or, absent a render surface, the source plus a changed-element table) alongside the API/data-model/constraints deltas and the two review verdicts, and explicitly surface the adopt-first ruling (FTS5 vs. custom vs. external) as a decision reserved to the user.
- **Read:** outputs of Phase 4.
- **Written:** nothing new until ruled; the store write from Phase 4 already exists as an in-flight delta (legal pre-checkpoint per the one carve in `impl.baselines-never-in-place`).
- **Seats/skills:** none new — Delivery Manager presents.
- **Gate — what is confirmed:** the design and the store delta as a signed whole, plus the adopt-first ruling.
  - **Ruling: sign as proposed (FTS5 chosen) →** the delta becomes the anchor for the deviation gate; proceed to Phase 6.
  - **Ruling: amend before signing (e.g., different search approach, reject the background worker in favor of synchronous reindex within the create request) →** the design seats rework only the amended slice, re-reviewed, re-presented — does not reopen the whole gap list.
  - **Ruling: user stops here and resumes the build later →** the run pauses cleanly; the signed design stands as the anchor for a future resumption; nothing beyond it is built.
  - **Ruling: reject outright →** run halts; no code is written; disposition of `FEAT-002`'s work rows (return to pending, or held) is the user's separate call.

## Phase 6 — Cycle card authoring
- **Done:** A design-class seat that will not build the cards — `technical-analyst` or `principal-architect`, never `staff-engineer` (`impl.seat-card-author-independence`) — slices the signed design into cycle cards per `mochiko:patterns-vertical-tdd`: foundation cycles first (e.g., persist/expose the search-index storage and the background worker scaffold), then feature cycles (W1 search-by-query, W2 freshness-under-2s). `qa_seat` authors the **TEST:** real-infrastructure gate per card within that slicing. Cards carry stories/rationale, dependencies, acceptance-criteria IDs, brownfield exposure — no task lists or file paths (those are the builder's build-time decomposition).
- **Read:** the signed design deltas, `spec.md`, `${tasks_schema}` (`plugins/mochiko/schemas/tasks.yaml`) for the card grammar.
- **Written:** `.mochiko/specs/note-search/tasks.md` (or the feature-scoped equivalent) — the cycle cards, unchecked.
- **Seats/skills:** `technical-analyst` (or `principal-architect`) authoring; `qa-engineer` for **TEST:** cases; pointer skill `mochiko:patterns-vertical-tdd`.
- **Gate:** none inside this phase.

## Phase 7 — Card review + card confirm (GATE, floor)
- **Done:** An independent verification seat (not the card author, not the future builder) reviews the cards for quality (`mochiko:review-plan-artifacts`) and buildability (own judgment) before presenting them.
- **Read:** `tasks.md`.
- **Written:** a card-review note (may fold into the same report envelope).
- **Seats/skills:** a verification seat (e.g. `qa-engineer`, distinct instance from the card-author check, or `mochiko:validator`).
- **Gate — what is confirmed:** the cycle slicing itself, before any build starts.
  - **Ruling: approved as sliced →** proceed to Phase 8.
  - **Ruling: reslice requested (e.g., split the background-worker foundation cycle further, or merge W1/W2 into one demonstrable cycle) →** the card author reworks only the disputed cards, re-reviewed, re-presented.

## Phase 8 — Build: TDD cycles, foundation before feature
- **Done:** `staff-engineer` (`builder_seat`) builds each confirmed card test-first, decomposing it into concrete tasks at build time (disclosed in `cycle-report.md`), following `mochiko:executing-tdd-cycle`, `mochiko:brownfield-integration` on any touch to existing code, and `mochiko:patterns-code-minimalism` at decomposition (rungs disclosed). Order: foundation cycle(s) (search-index storage + worker scaffold) before feature cycles (W1, then W2).
- Each cycle is independently verified against real infrastructure by a non-builder verification seat (e.g. `qa-engineer`) via `mochiko:testing-end-user`, plus the `mochiko:review-code-minimalism` advisory lens reading the diff, the cycle report, and the surrounding codebase. A cycle consumes one of its 3 default attempts per grading round; two consecutive rounds with unchanged findings triggers a no-progress stop.
- Escalations (e.g., a build-time discovery of undesigned structure triggering `impl.midrun-refire`, or a build-time technical decision needing a `baseline-delta.md` entry) batch to the cycle checkpoint rather than interrupting mid-cycle, unless build-blocking.
- **Read:** `tasks.md`, the signed design deltas, product baselines, existing (if any) product code.
- **Written:** product source code and tests (paths depend on the eventual stack — none exists yet, so the foundation cycle likely also establishes the module layout); `.mochiko/features/FEAT-002/cycle-report-<n>.md` per cycle; `tasks.md` checkboxes flipped `[x]` as cycles complete; any `baseline-delta.md` entries for build-time technical decisions.
- **Seats/skills:** `staff-engineer` (builder); `qa-engineer` (verifier); pointer skills `mochiko:executing-tdd-cycle`, `mochiko:brownfield-integration`, `mochiko:patterns-code-minimalism`, `mochiko:testing-end-user`, `mochiko:review-code-minimalism`.
- **Gate (cycle-checkpoint, recurring):** batched escalations and findings per cycle.
  - **Ruling: findings resolved / accepted →** cycle checkbox flips, next cycle proceeds.
  - **Ruling: infeasible card escalated as a business-scope decision →** halts to the user; branches to rescoping the card or accepting the added scope.
  - **Ruling: attempt/no-progress exhaustion →** halt that cycle, present state; user decides continue with an explicit exemption, rescope, or stop the run.

## Phase 9 — Final validation
- **Done, in order:**
  1. **Regression sweep:** re-run `FEAT-001`'s durable gate set (`.mochiko/features/FEAT-001/gates.md`) plus its cards' cases, since `FEAT-002` seams onto the already-delivered notes store.
  2. **Cold verification:** build and run the full quality-gate suite from a dependency-cold snapshot of the uncommitted working state. **Concrete blocker:** this environment has no `.git` — `git ls-files -co --exclude-standard` cannot run. This is presented as its own gate (see below) before this step can execute.
  3. **Blind gap-finding pass** (required — scope is selection): a fresh `devils-advocate` instance (`gap_finder_seat`), never having built these cycles or seen the design-time test cases, is dispatched two-message and blind — first message carries only `spec.md`, the sufficiency report, the design deltas, and the relevant baseline excerpts (data-model, contracts, the NFR-XXX concern rows), never code/tasks.md/TEST cases/reports. It states derived expectations, then probes the built system. Findings split: spec-required behavior broken → fails final validation; beyond-spec → advisory, disposition reserved to the user (fix now / BACKLOG.md / accept as designed).
- **Read:** `gates.md`, the built code, the design deltas, `spec.md`.
- **Written:** `.mochiko/features/FEAT-002/final-validation-report.md` (including the mutation-lens result or its stated skip, if governance depth is `high` — currently indeterminate, no governance region present); gap findings folded per disposition.
- **Seats/skills:** `devils-advocate` (blind gap-finder); pointer skill `mochiko:testing-gap-finding`.
- **Gate — the cold-verification environment blocker:**
  - **What is confirmed:** whether to `git init` this working tree (a repo-creating, not-purely-read action) so the cold snapshot can run, or to proceed some other way.
  - **Ruling: approve `git init` →** initialize, add the `.claude/worktrees` ignore entry, take the snapshot, continue cold verification normally.
  - **Ruling: decline →** cold verification cannot execute as specified; the run halts at this step and presents state — under the done condition, this leaves `impl.fail.no-evidence`-adjacent risk unresolved, so the run cannot close until the user picks a path (approve init later, or accept an explicitly-logged verification gap the user rules on).
- **Gate (escalation batch):** any Important-or-above advisory finding or a disputed finding-kind lands at this checkpoint for the user's ruling (fix-now / backlog / accept-as-designed / kind reclassified); Minor findings default to a BACKLOG.md booking without gating.

## Phase 10 — Landing (executed whole, at acceptance)
- **Done:** Once final validation is clean (or its findings are ruled), execute the landing as one whole action:
  - Fold the signed architecture-store delta: flip its elements built, clear the FEAT-002 key, write the graded As-built/Drift fields on touched rows, run the orphan check, and regenerate the derived `ARCHITECTURE.md`.
  - Fold the API-contract, data-model, and constraints-and-decisions deltas via graded three-way diffs (pre-fold baseline + delta vs. folded result).
  - Selection-scope map graduation: `FEAT-002`'s delivered work rows fold into its extent lines and vanish from "Work rows," status flips to `delivered` (dated), the `FEATURES.md` index line updates, and the `note-search` spec's index row is touched (closes once all its selected rows have folded).
  - Fold gap findings ruled fix-now/backlog into `.mochiko/features/FEAT-002/gates.md` (minted fresh) in **TEST:** grammar, authored by `qa-engineer`.
  - The landing verification seat checks every graded fold (transcription-only for the store; full three-way diff elsewhere).
- **Read:** all deltas and reports from Phases 4–9.
- **Written:** `.mochiko/product/architecture/spine.md` (fold), `ARCHITECTURE.md` (regenerated), `.mochiko/product/contracts/api.yaml`, `.mochiko/product/data-model.md`, `.mochiko/product/constraints-and-decisions.md` (folded), `.mochiko/features/FEAT-002/entry.md` (rows fold, status → delivered), `FEATURES.md`, `.mochiko/features/FEAT-002/gates.md`.
- **Seats/skills:** landing verification seat (independent, checks folds); pointer skills `mochiko:authoring-architecture-store`, `mochiko:authoring-feature-map`.
- **Gate:** none standalone — folded into the final acceptance gate below (`impl.dm-landing-whole` requires it execute whole at acceptance, so the plan sequences it as the action taken *by* the accept ruling below, not before it).

## Phase 11 — Final acceptance (GATE, floor) — run close
- **Done:** Present the completed build against the done condition: every cycle card `[x]`, test-first and independently verified per-cycle and whole; criteria traced; governance aligned (none declared, so n/a); the landing package from Phase 10 ready to execute whole; and a checklist against all 15 `kind: fail` conditions (none standing, assuming clean Phases 8–9).
- **Read:** all prior reports.
- **Written:** nothing until ruled; the landing writes from Phase 10 are staged as part of this action.
- **Seats/skills:** none — Delivery Manager presents; user decides.
- **Gate — what is confirmed:** accept / amend / reject, plain blocking text, never timed.
  - **Ruling: accept →** Phase 10's landing executes whole; the run closes with a verdict against the done condition, reporting rounds/seats consumed at this final checkpoint.
  - **Ruling: amend →** the specific amendment is scoped and routed back to the smallest phase that covers it (a card rework re-enters Phase 8/9's cycle-checkpoint economy; a design-level amendment re-enters Phase 4/5 with a re-fired, narrowly scoped design pass); landing does not execute until re-acceptance.
  - **Ruling: reject →** the run closes without landing; `FEAT-002`'s work rows remain unfolded (status stays as scoped), and the run's final report states the reject and its reason for the record.

---

**Not-done checklist carried through every phase above** (the 15 `impl.fail.*` rules): unrecorded sufficiency verdict, skipped/unsigned design phase, card independence or pre-confirm build, an unchecked card, a failing quality gate, verification without real-infrastructure evidence, a regression in `FEAT-001`'s gates, a baseline edited in place, an unresolved architecture deviation, an incomplete store landing, an ungraded fold, a missing gap-finding pass (selection scope — mandatory here), an unresolved spec-required gap finding, and missing user acceptance. Any one standing at Phase 11 fails the run regardless of how far the phases above progressed.