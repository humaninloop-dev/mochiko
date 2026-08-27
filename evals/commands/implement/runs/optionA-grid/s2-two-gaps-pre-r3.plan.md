# Action Plan — `/mochiko:implement FEAT-002` (plan-only, not executed)

Grounding already gathered by reading (no writes made): `plugins/mochiko/schemas/implement.yaml` (full, 15 fail-conditions confirmed), `plugins/mochiko/commands/implement.md`, `plugins/mochiko/skills/review-sufficiency/SKILL.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/specs/note-search/spec.md`, `.mochiko/features/FEAT-001/entry.md` + `gates.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `FEATURES.md`, `ARCHITECTURE.md`, and confirmed absence of `.claude/rules/mochiko/`, `CLAUDE.md`, `.mochiko/memory/`, and any application source tree.

## Phase 0 — Load binding rules
**Done:** Read `plugins/mochiko/schemas/implement.yaml` raw and whole (already complete above) before any gating or seat spawn. Verified the `impl.sec.fail-conditions` block carries exactly 15 `fail-condition` rules, matching the command's hard-coded count — no halt needed on that check. `vars:` resolved: `attempt_bound_cycle=3`, `gap_rework_bound=2`, `builder_seat=staff-engineer`, `design_seat=technical-analyst`, `architect_seat=principal-architect`, `qa_seat=qa-engineer`, `gap_finder_seat=devils-advocate`, `explore_model=haiku`.
**Read:** the schema file only.
**Written:** nothing.
**Seats/skills:** none yet.
**Gate:** none.

## Phase 1 — Resolve entry and scope type
**Done:** `FEAT-002` is a capability ID, not an epic (no `EPIC-XXX` lookup fires). Its map entry (`.mochiko/features/FEAT-002/entry.md`) carries two selected work rows (W1, W2) sourced from `.mochiko/specs/note-search/spec.md`'s accepted selection (ratified 2026-08-26) — this is **selection scope**, not delta scope. Dependency check: entry lists FEAT-001 as a dependency, and `FEATURES.md` shows FEAT-001 status `delivered` — the dependency is satisfied, so W1/W2 do not block on ordering.
**Read:** `.mochiko/features/FEAT-002/entry.md`, `.mochiko/specs/note-search/spec.md`, `FEATURES.md`.
**Written:** nothing.
**Seats/skills:** none (lead-only routing judgment).
**Gate:** none yet — feeds the run-open gate in Phase 3.

## Phase 2 — Sufficiency check
**Done:** Dispatch a seat that authored none of `spec.md`, the architecture store, or the product baselines, and that will not design or build this batch (rules out `technical-analyst`, `principal-architect`, `qa-engineer`, `staff-engineer`, and `devils-advocate` — the last is reserved fresh/blind for the later gap-finding pass). `mochiko:validator` fits the independence requirement and is the natural fit. It grades the ten clauses from `mochiko:review-sufficiency` per selected work row (W1, W2), fenced to `spec.md`, the architecture store, the baselines, and the map entries only — never code, `tasks.md`, or this run's own output directory.

From my own read of the sources (not a binding verdict — the dispatched seat's grading is), several clauses look likely to surface as gaps and are worth flagging as things the seat should specifically weigh:
- **Structural trigger (clause 4):** the spec's FR-103 requires a background index worker, but `spine.md` states "Synchronous request/response only; no queues, no background workers" — a live contradiction between what's specified and what's ruled.
- **Contract exposure (clause 2):** `contracts/api.yaml` has no `/notes/search` path — no seam to attach to.
- **NFR targets (clause 5):** SC-103's 2-second create-to-searchable bound has no matching concern row in the store's concern catalog.
- **Commodity exposure (clause 6):** full-text search is a commodity category (adopt-first applies) with no weighed alternative on record, and any candidate must respect C-001 (single-process, no external services).
- **Delivered-feature exposure (clause 9):** the touched surfaces (`api-service`, `notes-db`) are owned by the already-`delivered` FEAT-001 row, which auto-fires the design phase and requires a `[MODIFY]` amendment on FEAT-001's entry.

**Read:** (by the validator seat) `spec.md`, `spine.md`, `constraints-and-decisions.md`, `data-model.md`, `contracts/api.yaml`, both feature map entries.
**Written:** `.mochiko/features/FEAT-002/sufficiency-report.md` (per-row verdicts, gap list keyed to clauses, store-consult result, any trips/in-flight conflicts for the user, the `[MODIFY]` naming if clause 9 fires, `quickstart.md` null-path record since there's no real external-integration surface).
**Seats/skills:** `mochiko:validator`, running `mochiko:review-sufficiency`.
**Gate:** none directly — a disputed clause would default to gap and ride into the run-open gate below; nothing here is user-facing yet.

## Phase 3 — Run-open confirmation (**user gate**)
**What's confirmed:** one blocking, non-negotiable confirmation naming: the batch (FEAT-002, selection scope, rows W1+W2) · both attempt bounds at their only redeclaration point (per-cycle = 3, gap-rework = 2, or the user's override) · the sufficiency verdict and its gap routing from Phase 2 · any store trips (none expected — all three spine rows read `ruled`, none `open`/`not-now`) and any in-flight conflicts (none expected — FEAT-001 is `delivered`, not in-flight) · the absent-surface notices (no governance region present, no `.mochiko/memory/codebase-analysis.md`, offering `/mochiko:setup` or proceeding greenfield-with-warning; a store with ruled content exists so no bootstrap offer needed there) · the done condition stated plainly.

**Onward branches:**
- **User approves as stated (gaps present):** proceed to Phase 4, the design phase, scoped to exactly the gap list from Phase 2.
- **User approves as stated (hypothetically zero gaps):** skip Phase 4 entirely, go straight to Phase 5 (cards), with the card-authoring seat making the map-entry assertion the design phase would otherwise have made.
- **User redeclares an attempt bound:** the new bound is recorded here and carried for the rest of the run — no later redeclaration point exists.
- **User disputes a sufficiency clause or rules a trip/conflict differently than proposed:** that ruling is recorded and changes the Phase 2 gap list before Phase 4 scopes off it.
- **User halts here:** run stops cleanly; nothing downstream has been written yet except the sufficiency report.

**Read:** nothing new — restates Phase 1/2 outputs.
**Written:** nothing (the confirmation itself is a recorded exchange, not a file write).
**Seats/skills:** lead only.

## Phase 4 — Design phase (expected to fire, given the likely gaps above)
**Done:** Scoped to exactly the named gaps, nothing more, each design seat working on a plan the lead approves first (`mochiko:patterns-plan-minimalism` rung-justifies each element). Given the anticipated gap shape:
- `technical-analyst` (design_seat) authors the contract delta (a `GET /notes/search` path with the `q` param, 200/400 responses) and, if the seat judges an index entity worth modeling, a data-model delta — plus, since a build-time technology decision is needed for the search mechanism, a `constraints-and-decisions.md` delta proposing a D-XXX (e.g., an in-process full-text mechanism that satisfies C-001) via `mochiko:patterns-adopt-first`'s weighed-alternatives discipline. If a candidate would require an external service, that collides with C-001 and files a constraint-challenge finding reserved to the user rather than silently overriding it.
- `principal-architect` (architect_seat) authors the store delta: adds the background index worker and search-index element to the spine (a structural change — adds boxes/arrows to the ruled topology), the corresponding AX-XXX row(s) with NFR targets covering SC-103's 2-second bound, using `mochiko:patterns-system-design` for the delta diagram and `mochiko:authoring-architecture-store` for the row grammar.
- `qa-engineer` (qa_seat) authors the **TEST:** case shapes for W1/W2 from the spec's existing Given/When/Then scenarios.
- The FEAT-001 `[MODIFY]` amendment (clause 9) is written as the marked delta on FEAT-001's own entry file, not merged in place.

**Non-author review pair** (before the checkpoint): a seat distinct from the authors runs `mochiko:review-plan-artifacts` (conformance to the exact gap list, card-quality — blocking) and a seat distinct from the authors runs `mochiko:review-feasibility` (buildability/contradiction, and the architecture pass since a store delta exists here) — `mochiko:tech-lead` is the natural fit for the feasibility half given its cross-artifact review remit.

**Read:** `sufficiency-report.md`, `spec.md`, current baselines and store spine (as anchors for the deltas).
**Written:** design-phase deltas in `.mochiko/features/FEAT-002/` (e.g. `contracts/api.delta.yaml` or equivalent appliable form, a data-model delta if warranted, `baseline-delta.md` for the D-XXX decision) plus a signed store delta beside `.mochiko/product/architecture/spine.md` (the one legal in-place store write: in-flight-class delta elements only) plus the `[MODIFY]` delta on `.mochiko/features/FEAT-001/entry.md`.
**Seats/skills:** `technical-analyst`, `principal-architect`, `qa-engineer` as authors; a non-author pair (e.g. `validator` + `tech-lead`) as reviewers.

## Phase 5 — Design checkpoint (**user gate**)
**What's confirmed:** the design deltas and the store delta (rendered diagram plus the changed AX-XXX row table, or source-plus-changed-table if no render surface) are presented for sign-off; nothing downstream is written before this signs.

**Onward branches:**
- **User signs:** the store delta becomes the anchor for the deviation gate in Phase 7; proceed to Phase 6 (cycle cards).
- **User asks for amendment:** design seats rework within the gap-rework bound (2 rounds by default, per Phase 3); bound exhaustion or a round with unchanged findings halts and re-presents state, disposition reserved to the user.
- **User stops here to resume later:** the run may pause at this checkpoint and the build resumes on the signed design in a future run.

**Read/Written:** none beyond Phase 4's outputs.
**Seats/skills:** lead-facilitated; no new seat spawns unless rework fires.

## Phase 6 — Cycle-card authoring and card confirm
**Done:** A design-class seat that is not the builder (`technical-analyst` or `qa-engineer`, since staff-engineer never authors its own cards) slices the signed design into `tasks.md` cycle cards using `mochiko:patterns-vertical-tdd`, foundation cycles before feature cycles — concretely, a card establishing the background index worker + minimal search path (serving W1/SC-101/SC-102) before a card hardening the freshness bound (W2/SC-103), each card citing acceptance criteria by ID, its brownfield exposure (this touches the existing `api-service`/`notes-db` built by FEAT-001, so `[EXTEND]`/`[MODIFY]` classification applies), and a **TEST:** real-infrastructure gate — no task lists or file paths in the card itself.
An independent verification seat reviews the cards before confirm (`mochiko:review-plan-artifacts` for quality, its own judgment for buildability).

**User gate — card confirm:** the user rules the slicing.
- **User confirms as-is:** proceed to build.
- **User asks to re-slice:** the authoring seat revises and re-presents (no separate attempt-bound governs this — it's pre-build).
- **User rejects the slicing outright / wants scope changed:** routes back toward Phase 5 or Phase 3 depending on whether it's a design problem or a scope problem.

**Read:** the signed design deltas, `${tasks_schema}` (`plugins/mochiko/schemas/tasks.yaml`) if the `mochiko-cli template tasks` binary is absent.
**Written:** `.mochiko/features/FEAT-002/tasks.md`.
**Seats/skills:** `technical-analyst` or `qa-engineer` as card author; an independent verification seat as reviewer.

## Phase 7 — Build, per cycle
**Done:** For each confirmed card, `staff-engineer` (builder_seat) decomposes into concrete tasks (disclosed in the cycle report), runs the pre-code minimalism ladder at decomposition, follows `mochiko:brownfield-integration` since the touched files (`api-service`, `notes-db` layer) already exist, and drives red→green→refactor test-first. A cycle that discovers undesigned structure halts and re-fires Phase 4 scoped to the discovery rather than improvising.
**Read:** the existing `api-service`/`notes-db` source in full before touching it (brownfield discipline) — **note:** no application source tree was found under this working directory during scoping; if that holds at build time, this becomes a discovery in its own right (the "delivered" FEAT-001 status would have no code to extend), and per the entry-gate honesty rule this would need surfacing rather than silently proceeding.
**Written:** application source changes (paths not yet known — scoped by the cards) plus `.mochiko/features/FEAT-002/cycle-report-<n>.md` per cycle (decomposition, difficulties, deviations, `domain_deps_added`), plus `tasks.md` checkbox flips as each card completes.
**Seats/skills:** `staff-engineer`.
**Gate:** none directly (verification below is not a user gate unless it escalates).

## Phase 8 — Per-cycle verification
**Done:** An independent verification seat (never the builder) runs `mochiko:testing-end-user` against real infrastructure for the card's **TEST:** gate, applies the advisory `mochiko:review-code-minimalism` lens over the diff and cycle report, and — because this cycle's territory overlaps FEAT-001's delivered territory (`api-service`, `notes-db`) — re-runs FEAT-001's durable gates from `.mochiko/features/FEAT-001/gates.md` (the 3 existing **TEST:** cases) as part of the regression sweep for that cycle's territory.
Attempt economy: each grading round consumes one of the 3 per-cycle attempts; two consecutive rounds with unchanged findings is a no-progress stop presenting state to the user; only the user may exempt a round from the count.
**Escalation batching:** Important-or-above findings join a batch presented at the cycle checkpoint; Minor findings default to a `BACKLOG.md` booking rather than an in-cycle fix; a build-blocking question interrupts immediately rather than waiting for the batch.
**Read:** the diff, the cycle report, surrounding codebase (for reuse-claim verification).
**Written:** `.mochiko/features/FEAT-002/verification-report-<n>.md`.
**Seats/skills:** an independent verification seat (e.g. `qa-engineer`, distinct from whichever seat authored that cycle's card).
**Gate:** implicit — Important-or-above findings and any escalation reserved to the user (ambiguity, infeasible-card, adopt-first/IP-XXX calls) surface here as plain blocking text, not a timed prompt.

*(Phase 7 and 8 repeat per card until all cards in `tasks.md` are `[x]`.)*

## Phase 9 — Final validation (whole build)
**Done:** Full repository quality-gate suite, run from a dependency-cold snapshot (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-note-search/`, first confirming the `/.claude/worktrees` ignore entry exists). Because this is selection scope, the gap-finding pass is mandatory: a fresh `devils-advocate` instance is dispatched two-message and blind — first message carries only `spec.md`, `sufficiency-report.md`, the design deltas, and the baseline NFR rows, never the code, `tasks.md`, the **TEST:** cases, or any report; the seat states its derived expectations before probing begins. The same verification seat that already holds code sight runs the mutation lens if the run is at high depth (owing either mutation results or a stated skip). The regression sweep re-runs FEAT-001's accumulated gates plus any territory seams. Findings split by kind: spec-required behavior broken fails final validation; beyond-spec findings are advisory and go to the user for disposition (fix now / backlog / accept as designed) at the checkpoint, each finding's kind confirmed against its cited clause, defaulting to advisory if disputed.
**Read:** the built code tree, `spec.md`, prior reports (by the verification seat only — the gap-finder stays fenced per above).
**Written:** `.mochiko/features/FEAT-002/final-validation-report.md`, the built-vs-signed architecture diff.
**Seats/skills:** verification seat (mutation lens + regression), fresh `devils-advocate` (blind gap-finding, `mochiko:testing-gap-finding`).
**Gate:** gap-rework bound (2 rounds default from Phase 3) governs any rework loop here; bound exhaustion or unchanged findings halts and presents state, disposition reserved to the user.

## Phase 10 — Landing (executed whole, at acceptance)
**Done:** Store landing (delta elements flip `built`, FEAT-002 key clears, As-built/Drift fields written as judgment and independently graded, orphan check run) · graded folds for every touched baseline (`data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md` — three-way diff: pre-fold + delta vs folded result) · the FEAT-001 `[MODIFY]` amendment folds onto its entry · map landing for selection scope (W1/W2 fold into FEAT-002's extent, status flips `delivered` dated, `FEATURES.md` index line updates, specs-index row touched) · gap findings ruled fix-now or backlog fold into a newly minted `.mochiko/features/FEAT-002/gates.md`, authored by `qa-engineer` in the **TEST:** grammar · `ARCHITECTURE.md` regenerated from the store, never hand-edited.
**Read:** all prior-phase outputs as the transcription source.
**Written:** `.mochiko/product/architecture/spine.md` (store fold), `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/features/FEAT-001/entry.md`, `FEATURES.md`, `ARCHITECTURE.md`, `.mochiko/features/FEAT-002/gates.md`.
**Seats/skills:** landing verification seat (grades the folds and the As-built/Drift writes), `qa-engineer` (gate authoring).

## Phase 11 — Final acceptance (**user gate**, closes the run)
**What's confirmed:** every card `[x]`, verified per-cycle and whole, the landing executed as described in Phase 10, gap findings dispositioned, and the run's verdict against the done condition (all 15 fail-conditions checked clear: sufficiency recorded, design signed if gaps fired, no card built pre-confirm or by its author, no card left unchecked, no failing gate, no unevidenced verification claim, no regression, no in-place baseline edit, no unresolved deviation, complete store landing, every fold graded, gap-finding present for this selection-scope run, no unresolved spec-required gap finding).

**Onward branches:**
- **Accept:** run closes `done`; landing stands as executed.
- **Amend:** specific items are named for rework; the run re-enters the relevant phase (a code issue → Phase 7/8; a landing/fold issue → Phase 10) rather than closing.
- **Reject:** run closes `not done`; state is presented as-is for the user's next move (e.g., re-scope, escalate, or abandon this batch).

**Read/Written:** nothing new beyond a close-out note in the final-validation/landing reports.
**Seats/skills:** lead only, presenting the verdict; no seat may close the run on the user's behalf.