# Action Plan — `/mochiko:implement` (this invocation)

**Invocation state observed:** `$ARGUMENTS` is empty. Repo has `FEATURES.md` listing `FEAT-001` (Note capture, delivered 2026-08-22) and `FEAT-002` (Note search, status `selected`, work rows W1/W2 ratified via the spec's accepted selection on 2026-08-26 — **selection scope**). No `EPIC-XXX` exists. No `.mochiko/memory/`, no `CLAUDE.md` governance region, no `.claude/rules/mochiko/`. No product source tree found under any common extension despite `FEAT-001` reading "delivered."

---

## Phase 0 — Load the binding rules (mandatory first action, already executed in this planning pass)

- **Done:** Raw, full Read of `plugins/mochiko/schemas/implement.yaml`, `plugins/mochiko/schemas/common.yaml` (for the one `extends: common.register` stub), and `plugins/mochiko/schemas/command-labels.yaml`. `${var}` substituted live: `attempt_bound_cycle=3`, `gap_rework_bound=2`, `builder_seat=staff-engineer`, `design_seat=technical-analyst`, `architect_seat=principal-architect`, `qa_seat=qa-engineer`, `gap_finder_seat=devils-advocate`, `explore_model=haiku`.
- **Verified:** the schema's `kind: fail` node count is 15, matching the command's hard-coded Not-done count — no halt-and-surface needed on that sync check.
- **Read:** `plugins/mochiko/commands/implement.md` (narrative/protocol).
- **Written:** nothing.
- **Seats/skills:** none — this is the Delivery Manager's own read.

## Phase 1 — Entry resolution and dependency check

- **Done:** `$ARGUMENTS` empty → propose the next ready capability from the map rather than gate on a named one.
- **Read:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/specs/note-search/spec.md`.
- **Finding:** `FEAT-002` is the only capability at status `selected`; its sole dependency, `FEAT-001`, reads `delivered` on the map, so the dependency-order block does not trip. Scope type resolves to **selection** (source: the spec's accepted selection, not a `/mochiko:feature` delta card). No `EPIC-XXX` is in play, so none of the epic-only rules (`impl.epic-*`) activate.
- **Flag carried forward (not a gate, an honesty note per `impl.dm-entry-gate`):** `FEAT-001` is marked delivered but no matching source tree was found in the repo. This doesn't block entry — it's surfaced at the run-open confirmation as a risk to the later regression sweep and cold-verification step, which depend on `FEAT-001`'s code actually existing.
- **Written:** nothing yet.
- **Seats:** none — Delivery Manager judgment call, proposed to the user, not yet confirmed.

## Phase 2 — Absent-surface surfacing (`impl.absent-surfaces`)

- **Done:** check the three named absent-surface classes.
  - Governance region in `CLAUDE.md`: **absent** → surfaced to the user, non-blocking. Consequence: `depth` (low/high) is unresolvable from a governance region that doesn't exist, so the `impl.mutation-lens` rule's `when: {depth: high}` activation is itself undetermined until the user rules on it.
  - `.mochiko/memory/codebase-analysis.md`: **absent**, and this batch touches existing (`FEAT-001`) territory → brownfield branch: offer `/mochiko:setup`, or proceed greenfield with the absence logged.
  - Architecture store with no ruled content: **not the case** — `spine.md` has ruled elements (`api-service`, `notes-db`), so this branch doesn't fire.
- **Read:** `CLAUDE.md` (absent), `.mochiko/memory/` (absent), `.mochiko/product/architecture/spine.md`.
- **Written:** nothing. These are presented, never auto-resolved, never run-failing.
- **Seats:** none.

## Phase 3 — Sufficiency check (`impl.sufficiency-binding-verdict`, pointer: `mochiko:review-sufficiency`)

- **Done:** dispatch a grading seat that authored none of `spec.md`, the architecture store, or the product baselines, and will not design or build this batch (`impl.seat-sufficiency-independence`) — candidate: **`mochiko:validator`** (purpose-built independent grader), exempt from plan approval. Runs the full ten-clause check per selected row (W1, W2) since this is selection scope, not the three-clause delta-scope form.
- **Read (by the sufficiency seat):** `.mochiko/specs/note-search/spec.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`. Never reads code, `tasks.md`, or cycle reports.
- **Concrete gap candidates visible from the baselines as read in this planning pass** (the seat's verdict is authoritative, not this plan's):
  - `contracts/api.yaml` defines only `POST /notes` and `GET /notes/{id}` — no `/notes/search` path for W1/US-101/FR-101/FR-102.
  - `spine.md`'s ruled topology states *"Synchronous request/response only; no queues, no background workers"* — direct contradiction with spec FR-103's requirement for *"a background index worker … off the request path"* for W2/US-102. This is a structural, store-level gap, not just a documentation gap.
  - `FEAT-002/entry.md`'s Architecture link is explicitly unfilled — an absent-baseline trip on that row.
  - `data-model.md` models `Note` only; whether a search-index concept needs modeling is an open question for the design phase.
- **Written:** `sufficiency-report.md` at `.mochiko/features/FEAT-002/` — per-row verdict (`sufficient` or gap list), the store-consult result, any trips for run-open, the `quickstart.md` null-path note (no external-integration surface exists here), no `[MODIFY]` amendment expected since nothing here touches a *delivered* feature's entry.
- **Any disputed clause** the grader can't clear on its own defaults to gap and routes to the user (`impl.sufficiency-disputed-clause`) — never cleared by the grader alone.
- **Seats:** `mochiko:validator` (or an equivalently independent seat) — single-seat phase, no transport-floor trigger yet.

## Phase 4 — Run-open confirmation — **USER GATE**

- **What is confirmed, in one shot, no negotiation:**
  - Batch: `FEAT-002` — Note search, scope type: selection.
  - Attempt bounds at their only redeclaration point: 3 attempts/cycle, 2 gap-rework rounds at final validation (schema defaults) — offered for adjustment now, frozen after.
  - The sufficiency verdict and its gap routing: the API-contract gap and the background-worker/architecture-spine contradiction (and any others the actual grading turns up), each destined for the design phase.
  - Trips/conflicts reserved to the user: the unfilled Architecture-link trip on `FEAT-002/entry.md`, the missing-governance-region and missing-codebase-analysis absences, the `FEAT-001`-marked-delivered-but-no-code-found discrepancy.
  - Done condition, stated verbatim: every cycle card checked, test-first, independently verified per-cycle and whole, criteria traced, governance aligned (none exists to align to here, noted), acceptance landing executed whole, run closes on accept/amend/reject.
- **Branches:**
  - **User confirms as proposed** → proceed to Phase 5 with default attempt bounds.
  - **User adjusts attempt bounds** (e.g., raises cycle attempts) → carried as the frozen bounds for the rest of the run.
  - **User picks a different capability or wants to defer** → re-run Phase 1–3 against the new target; nothing below fires yet.
  - **User orders `/mochiko:setup` for the missing codebase analysis** → that command is out of this run's scope; this run pauses pending it, or the user waves it through with the warning logged.
  - **User rejects/stops here** → run ends, nothing written beyond the sufficiency report already produced.
- **Written:** nothing beyond Phase 3's report; this is a confirmation, not a write step.
- **Seats:** none — Delivery Manager presents, user rules.

## Phase 5 — Design phase (fires — the sufficiency check named gaps)

- **Done:** design seats author *exactly* the named gaps, nothing more, each on a plan the Delivery Manager approved first (`impl.plan-approval-producers`, `impl.design-gaps-only`, rung-justified per `mochiko:patterns-plan-minimalism`). Staffing call (`impl.design-seats-staffing`, advisory latitude):
  - **`technical-analyst`** (`design_seat`) — the `/notes/search` API contract addition (`mochiko:patterns-api-contracts`) and any `data-model.md` delta the index concept needs.
  - **`principal-architect`** (`architect_seat`) — the store delta resolving the FR-103-vs-spine contradiction: a new in-flight element (background index worker) plus its concern row, diagram delta per `mochiko:patterns-system-design`, landed per `mochiko:authoring-architecture-store`. This is a structural change, so it also routes through `mochiko:patterns-adopt-first` if a commodity indexing/search approach (e.g., SQLite FTS5 vs hand-rolled) is in play — an adopt-first ruling that is never the builder's, reserved to the user (`impl.adopt-first-user-call`).
  - **`qa-engineer`** (`qa_seat`) — design-time `**TEST:**` acceptance cases for SC-101/102/103, within the card-authoring seat's slicing (done in Phase 7, but the QA seat's design-time authorship starts here).
  - **`staff-engineer`** (`builder_seat`) is not staffed here — it never designs its own gaps (`impl.builder-never-designs`).
- **Read:** `sufficiency-report.md`, `spec.md`, current `spine.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`.
- **Written (deltas beside baselines, never in place — `impl.baselines-never-in-place`):**
  - `.mochiko/features/FEAT-002/contracts/` — delta adding `/notes/search`.
  - `.mochiko/features/FEAT-002/data-model.md` delta, if the grading confirms it's needed.
  - `.mochiko/features/FEAT-002/architecture-delta.md` (or equivalent, per `authoring-architecture-store`'s grammar) — the new in-flight element/row, signed only at the checkpoint.
  - `FEAT-002/entry.md` gets the design-implied dependency/extent assertion and the architecture-link fill (`impl.design-map-assertion`).
- **Independent review pair before the checkpoint** (`impl.design-review-pair`, non-author): **`mochiko:validator`** or a fresh seat for `mochiko:review-plan-artifacts` (conformance to the gap list, card quality — blocking), and **`mochiko:tech-lead`** for `mochiko:review-feasibility` (buildability/contradiction — this pass specifically has to judge whether the background-worker delta actually resolves the spine contradiction cleanly).
- **Seats:** multi (technical-analyst, principal-architect, qa-engineer, plus two independent reviewers) → the transport floor (`impl.transport-floor`) activates from here on for all cross-seat messaging and the shared-write surfaces (the store delta in particular).

## Phase 6 — Design checkpoint — **USER GATE** (floor, `impl.gate-design-checkpoint`)

- **What is confirmed:** the design deltas (contract, data-model, map assertion) and the store delta specifically — presented as a rendered diagram plus the named `AX-XXX` row changes (or, absent a render surface, the source plus a changed-element table, recorded either way).
- **Branches:**
  - **Sign as-is** → proceed to Phase 7.
  - **Request revision** (e.g., different indexing approach, or reject the new background-worker element in favor of synchronous inline indexing) → design seats revise scoped to the feedback, re-reviewed by the same independent pair, re-presented; loop until signed.
  - **Stop here** → the run may pause entirely; resumable later with the signed (or partially signed) design standing as the anchor.
- **Written:** nothing new here beyond what Phase 5 staged; the checkpoint only ratifies it. The one legal in-place store write (`impl.baselines-never-in-place`'s single carve) happens here: the store delta's in-flight-class elements become the user's signed record, still standing beside the ruled baseline until the landing folds it.
- **Seats:** none new — Delivery Manager presents, user rules.

## Phase 7 — Card authoring

- **Done:** a design-class seat (never the builder — `impl.seat-card-author-independence`) authors `tasks.md` cycle cards from the signed design, foundation cycles before feature cycles (`mochiko:patterns-vertical-tdd`): likely Cycle 1 — index-worker foundation/scaffold; Cycle 2 — W1 search endpoint; Cycle 3 — W2 freshness behavior. `qa-engineer` authors the `**TEST:**` real-infrastructure gate within each card's slicing. Cards carry stories, dependencies, acceptance criteria by ID (SC-101/102/103), the `**TEST:**` gate, and brownfield exposure (`[EXTEND]` on `api-service`/`notes-db`, since this touches `FEAT-001`'s delivered code) — no task lists, no file paths (the builder decomposes at build time).
- **Read:** the signed design deltas, `spec.md`, `.mochiko/schemas/tasks.yaml` (raw, if the `mochiko-cli template tasks` binary is absent).
- **Written:** `.mochiko/features/FEAT-002/tasks.md`.
- **Independent review before confirm** (`impl.card-review-before-confirm`, a seat that is neither the card author nor `qa-engineer` who co-authored the `**TEST:**` cases): **`mochiko:validator`** — quality per `mochiko:review-plan-artifacts`, buildability its own judgment.
- **Seats:** technical-analyst (or principal-architect) + qa-engineer + validator — multi-seat, transport floor still governs.

## Phase 8 — Card confirm — **USER GATE** (floor, `impl.gate-card-confirm`)

- **What is confirmed:** the cycle slicing itself — foundation-first ordering, card boundaries, whether the background-worker foundation cycle is scoped right before any feature cycle depends on it.
- **Branches:**
  - **Confirm as-is** → build begins (Phase 9).
  - **Re-slice** (split/merge/reorder) → card-authoring seat revises, re-reviewed, re-presented.
  - **Stop here** → pause, resumable; no code written yet.
- **Written:** nothing new.
- **Seats:** none new.

## Phase 9 — Build: TDD cycles, per-cycle verification, cycle checkpoints

- **Done, per card, in foundation-then-feature order:**
  - **`staff-engineer`** (builder, on the DM-approved plan) decomposes the card into concrete tasks at build time, red→green→refactor, test-first (`mochiko:executing-tdd-cycle`), touching existing `api-service`/`notes-db` code via `mochiko:brownfield-integration`, running the pre-code minimalism ladder at decomposition (`mochiko:patterns-code-minimalism`, rungs disclosed).
  - **`qa-engineer`** (verification seat, never the implementer) executes the card's `**TEST:**` gate against real infrastructure (`mochiko:testing-end-user`) — actual SQLite file, actual HTTP server, not mocks — plus the `mochiko:review-code-minimalism` advisory lens over the diff and cycle report.
  - **Attempt economy:** every grading round, whatever it's called, consumes one of the 3 per-cycle attempts (`impl.attempt-per-grade`); 2 consecutive rounds with unchanged findings is a no-progress stop — halt that cycle, present state to the user (`impl.no-progress-stop`).
  - **Mid-cycle interrupts, reserved to the user, never batched:** an infeasible card (business-level scope decision, `impl.infeasible-card-escalation`); a commodity-category adopt-first ruling if the search-matching approach (FTS5 vs LIKE-scan vs hand-rolled) wasn't already closed at design time (`impl.adopt-first-user-call`); undesigned structure the builder hits mid-build, which halts that cycle and re-fires the design phase scoped to the discovery, re-graded, re-checkpointed (`impl.midrun-refire`); any architecture deviation — added/removed box or arrow, or a moved responsibility versus the signed delta — which stops and is presented rather than silently designed around (`impl.deviation-gate`, floor).
  - **Everything else** (Minor findings, non-blocking questions) batches at the cycle checkpoint moment.
- **Written per cycle:** `.mochiko/features/FEAT-002/cycle-report.md` entries (decomposition disclosed, honest difficulties, deviations, `domain_deps_added`), `tasks.md` checkbox flips as each card completes (`impl.progress-surface`).
- **Cycle checkpoint — implicit gate, batched, not a hard stop unless build-blocking:** escalations and Important-or-above findings present as one batch; the user rules each; Minor findings default to a `BACKLOG.md` booking. This repeats per card until all cards read `[x]`.
- **Seats:** staff-engineer + qa-engineer, multi-seat, transport floor active; locate/enumerate reads within any seat's work route to a native `Explore` subagent at `model: haiku` (`impl.model-tiering`).

## Phase 10 — Final validation (moment: `final-validation`)

- **Done, once all cards are `[x]`:**
  - **Regression sweep** (`impl.regression-sweep`): re-run `FEAT-001`'s durable gates from `.mochiko/features/FEAT-001/gates.md` (the create/restart/get round trip, empty-body 400, get-existing-vs-random 404) plus `FEAT-002`'s own new gates, since this feature's territory touches `FEAT-001`'s (`notes-db`, `api-service`). **Note carried from Phase 1:** this step needs `FEAT-001`'s actual source to exist; if it doesn't, that surfaces here as a blocking discrepancy, not a silent pass.
  - **Cold verification** (`impl.cold-verification`): after confirming the `/.claude/worktrees` gitignore entry exists, copy the uncommitted working tree (`git ls-files -co --exclude-standard :!.claude/worktrees`) to `.claude/worktrees/mochiko-<purpose>/`, build and run the full quality-gate suite from that dependency-cold snapshot.
  - **Gap-finding pass** — required, selection scope (`impl.gap-finding-scope`; its absence would trip `impl.fail.gap-finding-missing`): a **fresh `devils-advocate`** seat, never one that built these cycles or saw the design-time `**TEST:**` cases, dispatched two-message blind (`mochiko:testing-gap-finding`). First message carries only `spec.md`, `sufficiency-report.md`, the signed design deltas (including the store delta for the index worker), `data-model.md`, `contracts/`, and the store's NFR rows (NFR-001, NFR-002, plus any new NFR the index-worker delta introduced) — never code, `tasks.md`, `**TEST:**` cases, or any report. The seat states derived expectations before probing the running system.
  - **Mutation lens:** conditional on `depth: high`, itself unresolved per the Phase 2 absent-governance-region flag — this needs the user's ruling on how to treat that gap before the lens either runs or is disclosed as skipped.
  - **Findings routing:** spec-required behavior broken → fails final validation unless resolved (`impl.fail.spec-gap-unresolved`); beyond-spec findings are advisory, disposition reserved to the user (fix now / `BACKLOG.md` / accept as designed — `impl.beyond-spec-disposition`); a disputed finding kind defaults advisory and goes to the user, never gated by the finder alone.
  - **Gap-rework bound:** 2 rounds at run scope (as frozen at run-open); a finding localized to one cycle's territory instead charges that cycle's remaining attempts; bound exhaustion or a no-progress round halts the run, disposition reserved to the user.
- **Written:** `.mochiko/features/FEAT-002/final-validation-report.md`, the built-vs-signed diff against the store delta.
- **Seats:** fresh `devils-advocate` (gap-finder) + whichever seat runs the regression/cold-verification gates (qa-engineer).

## Phase 11 — Final acceptance — **USER GATE** (floor, `impl.gate-final-acceptance`), plain blocking text

- **What is presented:** final-validation results (regression sweep, cold-verification outcome, gap-finding findings and dispositions), a preview of what the landing (Phase 12) will do, and the done-condition checklist status.
- **Branches:**
  - **Accept** → Phase 12 executes whole; run closes with a verdict against the done condition (`impl.dm-close-verdict`), stating rounds consumed and seats spawned (`impl.dm-surface-rounds`).
  - **Amend** → user specifies the change; run loops back to the relevant phase (a cycle for rework, or design if scope itself changes); re-verification scopes to the delta only — a test-only or records-only change gets a delta-grade of the changed surface, never a full gate re-sweep, and the graded object stays the code-tree HEAD (`impl.delta-reverification`).
  - **Reject** → run closes without landing; nothing folds (deltas already sit beside baselines, never edited in place, so rejection leaves the baselines untouched); `FEAT-002`'s work rows remain `selected` for a future run; the run closes with a FAIL-toned verdict against the done condition.
- **Written:** nothing beyond the report already staged, until the branch resolves.
- **Seats:** none — Delivery Manager presents, user rules.

## Phase 12 — Landing (executed whole, only on Accept; moment: `landing`)

- **Done** (selection scope, `impl.landing-selection`):
  - **Store landing** (`impl.store-landing`): the signed delta's elements (the background-index-worker element) flip to `built`, the `FEAT-002` key clears; the touched rows' `As-built:`/`Drift:` fields are written as judgment and independently graded by a landing verification seat (e.g., `tech-lead`, distinct from whoever authored the delta); the orphan check runs; the store skill regenerates the derived `ARCHITECTURE.md` — never hand-edited here.
  - **Graded fold** for every touched baseline (`impl.graded-fold`): `contracts/api.yaml` (new `/notes/search` path), `data-model.md` if touched, `constraints-and-decisions.md` if the adopt-first search-approach decision produced a new `D-XXX` row — each a three-way diff (pre-fold baseline + delta vs folded result) checked by the landing verification seat.
  - **Map graduation batch:** W1/W2 fold into `FEAT-002/entry.md`'s Extent and vanish from the work-rows list; status flips `selected → delivered`, dated, never regressing; `FEATURES.md`'s `FEAT-002` line updates; the `note-search` spec reads closed since both its selected rows folded (derived, never asserted).
  - **Gates fold:** any fix-now/backlog-ruled gap findings fold into a new `.mochiko/features/FEAT-002/gates.md`, authored by `qa-engineer` in `**TEST:**` grammar; accepted-as-designed findings don't fold.
  - **km-landing:** skipped — `.mochiko/memory/knowledge-management.md` is absent, so this rule's `when:` doesn't hold.
- **Written:** `.mochiko/product/architecture/spine.md` (folded), `ARCHITECTURE.md` (regenerated), `.mochiko/product/contracts/api.yaml` (folded), possibly `.mochiko/product/data-model.md` and `constraints-and-decisions.md` (folded), `FEAT-002/entry.md`, `FEATURES.md`, `.mochiko/features/FEAT-002/gates.md`.
- **Seats:** the landing verification seat (independent grader of the As-built/Drift writes and the folds) + the Delivery Manager executing the fold sequencing.
- **No git mutations are run** — commits are suggested only, never executed or pushed (`impl.no-git-mutations`); the cold-verification snapshot under `.claude/worktrees/` is ephemeral and self-removed, not a ref/content mutation.

## Phase 13 — Close

- **Done:** state the run's verdict against the fixed done condition, explicitly checking the 15 `kind: fail` predicates from `impl.sec.fail-conditions` — any one standing (unrecorded sufficiency, skipped/unsigned design, card-independence violation, an unchecked card, a failing quality gate, unevidenced verification, a regression, an in-place baseline edit, an unresolved deviation, an incomplete store landing, an ungraded fold, a missing gap-finding pass on this selection-scope run, an unresolved spec-required gap finding, or missing user acceptance) fails the run regardless of everything else completed.
- **Written:** the close statement itself (part of the final-validation/acceptance reporting already staged, not a new file).
- **Seats:** none — Delivery Manager only.