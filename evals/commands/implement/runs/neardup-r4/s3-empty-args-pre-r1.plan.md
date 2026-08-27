# Action Plan — `/mochiko:implement` (this invocation)

**Resolved target:** no argument was given, so this run must propose the next ready capability rather than assume one. `FEATURES.md` lists `FEAT-001` (delivered) and `FEAT-002 — Note search` (status: selected, ratified 2026-08-26). `FEAT-002` is the only entry carrying ratified, unfolded work rows, and its sole dependency (`FEAT-001`) is delivered, so it is not blocked. This plan proceeds against **FEAT-002, selection scope, work rows W1 (search by query) and W2 (index freshness)**.

**Prep already done (not a phase, a precondition):** `plugins/mochiko/schemas/implement.yaml` and `plugins/mochiko/schemas/common.yaml` were read raw and in full, per the run's binding first action. The `kind: fail` count in `impl.sec.fail-conditions` was checked against the command's Not-done line: 15 present, 15 expected — in sync, no halt triggered. `command-labels.yaml` was read to resolve the label vocabulary used across rules.

**Two anomalies surface immediately from reading the repo state, ahead of any gate, because absent/contradictory surfaces are never auto-resolved:**
- The working directory holds **no application code anywhere** (no `src/`, no language manifest, nothing) even though `FEAT-001/entry.md` claims "delivered" status with a durable gate set. There is also no `.mochiko/memory/codebase-analysis.md`. This is a brownfield-vs-greenfield contradiction that has to be surfaced to the user, not guessed past.
- The working directory **is not a git repository** (confirmed by environment metadata) and carries no governance region (`CLAUDE.md` absent) and no `.claude/rules/mochiko/`. The final-validation cold-verification step is git-based (`git ls-files -co --exclude-standard`), so this is a mechanical blocker, not just a policy gap.

Both are carried forward into the phases below rather than resolved silently.

---

## Phase 1 — Propose and gate the capability entry

**Does:** States the proposed batch (`FEAT-002`, work rows W1/W2, selection scope) and the dependency check (`FEAT-001` delivered, not blocking).
**Reads:** `FEATURES.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/features/FEAT-001/entry.md`.
**Writes:** none yet.
**Seats/skills:** none spawned yet — this is the delivery manager's own read.

**Gate (proposal confirmation, since the invocation carried no explicit capability):** confirms with the user that `FEAT-002` is the intended target for this run, and flags the two anomalies above for early awareness.
- *User confirms FEAT-002* → continue to Phase 2.
- *User names a different capability or a delta card instead* → restart resolution against that target; this plan's remainder would be re-derived against it.
- *User wants the code-absence/no-git anomalies resolved first (e.g., via `/mochiko:setup`)* → this run pauses/exits and routes there before re-entering.

---

## Phase 2 — Sufficiency check

**Does:** Grades each selected work row (W1, W2) against the spec, the product baselines, and the architecture store, per the ten-clause sufficiency check. This seat must have authored none of the spec/store/baselines and will not design or build this batch — a fresh grading seat, not `technical-analyst`, `principal-architect`, or `staff-engineer`.
**Reads:** `.mochiko/specs/note-search/spec.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/architecture/spine.md`.
**Writes:** `.mochiko/features/FEAT-002/sufficiency-report.md`.
**Seats/skills:** one independent grading seat (`mochiko:tech-lead` or `mochiko:validator` — a staffing call, not fixed by the schema) running `mochiko:review-sufficiency`.

From what's on disk, the check would very likely surface these as gaps or trips:
1. No `/notes/search` endpoint exists in `contracts/api.yaml` — a contract gap for W1.
2. The architecture spine explicitly states "Synchronous request/response only; no queues, no background workers," but the spec's FR-103 requires a background index worker for W2 — a direct conflict with ruled architecture, not a mere absence. This reads as a **store trip**, reserved to the user, not something the grader clears alone.
3. No technology decision exists for the search/indexing mechanism (a commodity category — adopt-first applies) — a gap in `constraints-and-decisions.md`.
4. No architecture concern row carries an NFR target for the 2-second freshness bound in SC-103 — a gap in the architecture store's concern catalog.
5. No index/staleness representation exists in `data-model.md` to support W2.
6. The code-absence anomaly from Phase 1 bears on the check's brownfield-vs-greenfield framing and gets carried into its report rather than resolved by the grader.

A disputed clause among these defaults to "gap," never a grader-side clear.

---

## Phase 3 — Run-open confirmation (the entry gate)

**Does:** One batched confirmation — no negotiation inside it.
**Reads:** the sufficiency report just written.
**Writes:** none (the report already landed in Phase 2).
**Seats/skills:** none new.

**Gate contents:** names the batch (`FEAT-002`, selection scope) · restates the two attempt bounds at their only redeclaration point — 3 verification attempts per cycle, 2 gap-rework rounds for the whole run, unless the user redeclares them here · presents the sufficiency verdict and its gap list (items 1–5 above) plus the store trip (item 2) and the two anomalies (code-absence, no git) for ruling · states the done condition (every cycle card checked, test-first, independently verified per-cycle and whole, traces to FR-101–103/SC-101–103, landing executed whole, none of the 15 fail conditions standing).

- *User rules the store trip in favor of a background-worker architecture delta, accepts proceeding greenfield-with-warning on the code-absence anomaly, and accepts the default attempt bounds* → gaps 1–5 scope the design phase; continue to Phase 4.
- *User instead narrows scope to avoid the architecture conflict* (e.g., defers W2 or requires synchronous-only indexing) → the batch re-gates on the amended scope; the sufficiency verdict is re-checked against the narrower rows before Phase 4.
- *User wants `git init` and/or `/mochiko:setup` resolved before any design or build work* → this run pauses here; those actions happen outside this run, and re-entry resumes at this same gate.
- *User rejects the batch outright* → run ends without opening; routes back to `/mochiko:specify` or `/mochiko:feature` as appropriate.

The rest of this plan assumes the first branch (proceed as scoped) to stay concrete.

---

## Phase 4 — Design phase (fires because Phase 2 named gaps)

**Does:** Authors exactly the five named gaps, nothing more, each on a plan the delivery manager approves first.
**Reads:** `sufficiency-report.md`, the same product baselines read in Phase 2, `.mochiko/features/FEAT-002/entry.md`.
**Writes:**
- `.mochiko/product/architecture/` delta beside `spine.md`: a new concern row (e.g. an indexing/search element) plus the topology change legalizing a background worker, with an NFR target for the 2-second freshness bound — authored via `mochiko:patterns-system-design` (C4-container delta diagram, sequence diagram for the create→index→search flow), landed per `mochiko:authoring-architecture-store` grammar.
- API contract delta beside `contracts/api.yaml`: the new `GET /notes/search` endpoint, its 400 case, response schema — via `mochiko:patterns-api-contracts`.
- Data-model delta beside `data-model.md`: an index/staleness representation supporting W2 — via `mochiko:patterns-entity-modeling`.
- Technology-decision delta beside `constraints-and-decisions.md`: a new D-XXX row for the indexing mechanism, run through the adopt-first evaluation (commodity category — full-text search) — via `mochiko:authoring-technical-requirements` / `mochiko:patterns-technical-decisions` / `mochiko:patterns-adopt-first`. The actual adopt-first pick is reserved to the user, not the design seat's call, and surfaces at the checkpoint (or sooner if it blocks the design work).
- Map assertion on `.mochiko/features/FEAT-002/entry.md`: design-implied dependencies and sharpened extent, with the Architecture link field filled once the new AX row exists.
- If a build-time-shaped technical decision is discovered instead of a pure design one, it would be written the same way — as a delta, never in place.

**Seats/skills:** `mochiko:principal-architect` for the architecture-store delta; `mochiko:technical-analyst` for the API contract, data-model, and technology-decision deltas — each working only on a delivery-manager-approved plan, rung-justified per `mochiko:patterns-plan-minimalism` (disclosed, not assumed minimal).

**Review pair (non-author, before the checkpoint):** `mochiko:review-plan-artifacts` (blocking — conformance to the exact gap list, nothing more, card-quality once cards exist) and `mochiko:review-feasibility` (buildability/contradiction, plus the architecture pass since a store delta exists) — run by a seat that authored none of this package, e.g. `mochiko:tech-lead`.

**Gate — design checkpoint (the user's):** presents the architecture delta (source plus the changed-AX-row table, since no render surface is evident in this environment) and the three baseline deltas, for sign-off.
- *User signs* → the store write becomes legal (the one carve in an otherwise never-in-place baseline discipline), and the run proceeds to Phase 5.
- *User asks for revision* (different indexing tech, different NFR bound) → the design seat revises on an approved plan, review pair re-grades, checkpoint re-presented.
- *User stops here to resume later* → run pauses cleanly with state recorded; no code has been written.
- *User rejects the background-worker approach entirely* → this is a scope-level call; either the spec is amended first (outside this run) or, if the user explicitly accepts the larger scope here, the design phase re-scopes around a synchronous-only alternative and repeats its review/checkpoint cycle.

---

## Phase 5 — Cycle-card authoring and card confirm

**Does:** Slices the ratified, signed design into cycle cards — foundation before feature, walking skeleton first. A design-class seat authors the cards, never the builder; the QA seat authors the `**TEST:**` cases within that slicing.
**Reads:** the signed design deltas, `spec.md` (for acceptance-criteria IDs), `mochiko:patterns-vertical-tdd` grammar.
**Writes:** `.mochiko/features/FEAT-002/tasks.md` — likely two cards:
- Cycle 1 (foundation): naive query-string search over the existing notes store, covering SC-101/SC-102 — the walking skeleton.
- Cycle 2 (feature): the background index worker and freshness bound, covering SC-103, built on Cycle 1.

Each card carries stories/rationale, dependencies, acceptance-criteria IDs, a real-infrastructure `**TEST:**` gate, and its brownfield exposure — both cycles are `[EXTEND]`s of the existing (nominally delivered, physically absent) `api-service`/`notes-db`, which folds the Phase-1 code-absence anomaly directly into how build-time brownfield reading has to work.

**Seats/skills:** `mochiko:technical-analyst` or `mochiko:principal-architect` continuing as card author; `mochiko:qa-engineer` for the `**TEST:**` cases. A verification seat (non-author — e.g. `mochiko:tech-lead`) reviews the cards for quality and buildability before confirm.

**Gate — card confirm (the user's, blocking):** presents the two-cycle slicing and dependency order.
- *User confirms* → build begins at Cycle 1.
- *User asks for different slicing* (merge to one cycle, or split further) → card author revises, re-reviewed, re-presented.
- *User rejects the underlying design at this point* → escalates back to the design checkpoint rather than proceeding.

---

## Phase 6 — Build and per-cycle verification

**Does, per cycle (Cycle 1, then Cycle 2):**
- Builder (`mochiko:staff-engineer`) decomposes the card into concrete tasks at build time, disclosed in the cycle report, and builds test-first (red→green→refactor), applying brownfield-integration discipline to the `[EXTEND]` touches and the pre-code minimalism ladder at decomposition — on a delivery-manager-approved plan.
- Verification (`mochiko:qa-engineer`, never the implementer) executes the card's `**TEST:**` gate against real running infrastructure, captures evidence, and applies the code-minimalism lens by reading the diff, the cycle report, and the surrounding code — minimalism findings are advisory only, never gate-failing.

**Reads:** the signed design deltas, the card in `tasks.md`, the existing (would-be) `api-service`/`notes-db` code for the `[EXTEND]` touches.
**Writes:** application source (paths depend on the language/stack actually chosen — undetermined given the code-absence anomaly, so this would need resolving in practice before Cycle 1 starts), `.mochiko/features/FEAT-002/cycle-report.md` (×2, dated/appended), verification evidence, `tasks.md` checkbox flips per completed cycle.

**Bounds:** 3 verification attempts per cycle; two consecutive rounds with unchanged findings halts that cycle and presents state rather than continuing to spend attempts. Important-or-above findings block the cycle and batch into the cycle checkpoint; Minor findings default to a `BACKLOG.md` booking instead of an in-cycle fix.

**Gate-like moments within this phase:**
- *Deviation gate:* if a cycle needs a structural change beyond the signed delta (an extra box/arrow), it stops and presents to the user — build as approved, or amend the delta by ruling, before continuing.
- *Mid-run design re-fire:* if the builder hits undesigned structure, that cycle halts and Phase 4's design work re-fires scoped to just the discovery, through the same review/checkpoint sequence, before the cycle resumes.
- *Cycle checkpoint:* non-blocking escalations and findings accumulate and land as one batch here rather than interrupting; only a build-blocking question interrupts mid-cycle.

---

## Phase 7 — Final validation

**Does:** Because this is a selection-scope run, the gap-finding pass is required, not optional.
- **Regression sweep:** re-runs `FEAT-001`'s durable gate set (`gates.md` — 3 cases) plus this feature's own gates, since W1/W2 read the notes store FEAT-001 built.
- **Cold verification:** builds and runs the full quality-gate suite from a dependency-cold snapshot of the uncommitted working tree. **This step is currently blocked mechanically** — the working directory is not a git repository, so the `git ls-files`-based snapshot cannot run until that's addressed (e.g., `git init`), which would need to be raised to the user rather than skipped silently.
- **Gap-finding pass:** a fresh `mochiko:devils-advocate`, never a seat that built these cycles or saw the design-time test cases, dispatched blind in two messages — first carrying only `spec.md`, the sufficiency report, the design deltas, and the relevant baselines (never code, `tasks.md`, the TEST cases, or any report), stating its own derived expectations before probing the live system.
- **Mutation lens:** applies only at governance depth "high" — but the governance region is absent here, so depth cannot resolve from a surface; this gets surfaced to the user rather than assumed either way.

**Reads:** `gates.md` (FEAT-001), `spec.md`, `sufficiency-report.md`, design deltas, baselines.
**Writes:** `.mochiko/features/FEAT-002/final-validation-report.md`; a `baseline-delta.md` if any build-time technical decision surfaced during Cycle 1/2 decomposition and wasn't already captured in Phase 4.

**Bounds:** gap-rework at this stage draws from the run-scope pool of 2 rounds (redeclared, if at all, only back at Phase 3); a finding localized to one cycle's territory instead charges that cycle's remaining attempts.

Findings split: anything showing spec-required behavior broken fails final validation outright; beyond-spec findings are advisory and go to the user for disposition (fix now / backlog / accept as designed) at the acceptance gate.

---

## Phase 8 — Final acceptance and landing

**Does:** Presents the whole build for the user's ruling; only on acceptance does the landing execute, as one whole action, never partially.
**Reads:** the final-validation report, the signed design deltas, `tasks.md`.
**Writes (only upon Accept):**
- Store landing: the architecture delta's elements flip to built, the FEAT-002 key clears off them, As-built/Drift fields are written and independently graded, the orphan check runs, `ARCHITECTURE.md` is regenerated (never hand-edited).
- Map graduation: W1/W2 fold into `FEAT-002`'s extent in `entry.md`, status → delivered (dated); `FEATURES.md` index line updates; the `note-search` spec closes since all its selected rows folded.
- Graded folds (three-way diff each, checked by a non-author verification seat): `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md` each absorb their design-phase deltas exactly once.
- `.mochiko/features/FEAT-002/gates.md` (minted) gains any fix-now/backlog-disposed gap findings, authored by `mochiko:qa-engineer` in `**TEST:**` form.
- No knowledge-management landing obligations fire — `.mochiko/memory/knowledge-management.md` is absent in this repo.

**Seats/skills:** the landing verification seat (non-author) grades every fold; `mochiko:authoring-architecture-store` and `mochiko:authoring-feature-map` own the respective write grammars.

**Gate — final acceptance (the user's, plain blocking text, never timed):**
- *Accept* → landing executes whole as above; the run closes with a verdict stated against the done condition: all cards checked, test-first, independently verified per-cycle and whole; code traces to FR-101–103/SC-101–103; governance alignment noted as inapplicable/absent rather than silently assumed; none of the 15 not-done conditions standing (assuming the git/cold-verification blocker from Phase 7 was resolved along the way — otherwise that alone would leave the run not-done).
- *Amend* → the user specifies what changes; small amendments re-open a cycle or a design delta within remaining attempt/rework budget; amendments exceeding this run's framing route to a fresh spec/feature touch instead of landing here.
- *Reject* → nothing folds; the run ends with state recorded honestly, and the work stays unlanded pending further direction.