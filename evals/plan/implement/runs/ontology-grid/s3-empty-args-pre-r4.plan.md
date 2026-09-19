Read `plugins/mochiko/schemas/implement.yaml` in full (six sections, 15 fail-condition rules confirmed — count matches the command's hard-coded expectation) and cross-checked the actual workspace: `FEATURES.md`, `.mochiko/features/FEAT-001/{entry,gates}.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, and confirmed no `.claude/`, no `.mochiko/memory/`, and no git repo exist in this workspace. Here is the concrete plan for this invocation.

# Action Plan — `/mochiko:implement` (this invocation)

No `$ARGUMENTS` was supplied with the command. The map has exactly one non-delivered capability (FEAT-001 is `delivered`; FEAT-002 is `selected`), so the whole run resolves around FEAT-002. This is a plain selection-scope run, not an epic — `EPIC-XXX` naming never appears.

## Phase 1 — Scope resolution
**Does:** Since the argument is empty, treat this as "propose the next ready capability." Scan `FEATURES.md` for capabilities not yet `delivered`; only FEAT-002 ("Note search," status `selected`) qualifies. Check its one listed dependency — FEAT-001 — against `delivered` status: it is delivered, so nothing blocks. Candidate batch: FEAT-002, work rows W1 ("Search notes by query string," US-101, SC-101/SC-102) and W2 ("Index stays fresh after creates," US-102, SC-103), both selection scope, ratified 2026-08-26 in the spec's accepted selection.
**Reads:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/features/FEAT-002/entry.md`.
**Writes:** none yet.
**Seats/skills:** none spawned yet — this is the Delivery Manager's own scan.
**Gate:** none standalone; the proposal folds into the single run-open confirmation in Phase 3.

## Phase 2 — Sufficiency check
**Does:** Grade FEAT-002's two work rows for design sufficiency against the spec, the architecture store, and the product baselines, using the sufficiency-review procedure. Staff a seat that authored none of those sources — Tech Lead fits (didn't write the spec, the spine, or the baselines) and is exempt from plan approval as a grading seat. Concretely, this grading surfaces two real issues found by inspection:
- **Contract gap (W1):** `contracts/api.yaml` defines only `POST /notes` and `GET /notes/{id}`; there is no `GET /notes/search` path, request schema, or response shape for US-101/FR-101/FR-102. → gap.
- **Architecture conflict (W2):** FR-103 requires "a background index worker" that rebuilds off the request path, but `spine.md`'s Topology note says "Synchronous request/response only; no queues, no background workers," and the ruled element table has no worker element. This isn't a plain gap — it's a collision between required behavior and already-ruled architecture, i.e., a **store trip**, not something the grader can silently resolve.
- Also checked and clean: `data-model.md`'s Note entity is sufficient for W1's matching/ranking; no obvious gap there beyond whatever the index mechanism itself needs (folded into the W2 trip).
- Also noted, not a gap and not run-failing: no `.claude/rules/mochiko` governance region and no `.mochiko/memory/` directory exist in this workspace (no `codebase-analysis.md`, no `knowledge-management.md`, no `governance-intent.md`). Absent surfaces — surfaced to the user, not auto-resolved, not a run failure.
**Reads:** `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`.
**Writes:** `.mochiko/features/FEAT-002/sufficiency-report.md` — per-row verdict (W1: gap — contract; W2: gap + trip — architecture conflict), the store-consult result, the trip queued for the user, and the governance-region/memory absences logged.
**Seats/skills:** Tech Lead, running the sufficiency-review procedure.
**Gate:** none standalone — any disputed clause would default to "gap" and roll into the run-open confirmation; there's no dispute here, both verdicts are clear-cut from the artifacts.

## Phase 3 — Run-open confirmation (entry gate)
**Does:** One confirmation bundling everything decided so far: names the batch (FEAT-002, selection scope, work rows W1+W2, dependency on delivered FEAT-001 satisfied) · restates the two attempt bounds at their only redeclaration point (3 verification attempts per cycle, 2 gap-rework rounds at run scope — defaults, unless the user wants to redeclare now) · presents the sufficiency verdict and gap routing (W1 → contract design gap; W2 → contract + architecture-conflict design gap) · surfaces the FR-103-vs-spine store trip explicitly for a ruling · notes the absent governance region and absent `.mochiko/memory/` files · states the done condition (every cycle card checked, test-first, independently verified per-cycle and whole, criteria traced, landing executed whole, closes at final acceptance).
**Reads:** the sufficiency report just written, `implement.yaml`'s `vars:` block for the default bounds.
**Writes:** nothing new (the confirmation is presented; the sufficiency report already exists).
**Gate — what's confirmed:** the batch/scope naming, the two attempt bounds (as-is or redeclared), and, specifically, how to handle the FR-103/spine conflict.
- **Ruling: proceed, reconcile via architecture delta** → Phase 4 fires with the architect authorized to propose adding a background-worker element to the store (in-flight class).
- **Ruling: proceed, but push back on FR-103** → Phase 4 is scoped only to the W1 contract gap for now; the W2 requirement gets flagged back toward the spec (a note for a future `/mochiko:feature` or `/mochiko:specify` touch), and W2 is deferred out of this run's build scope pending that resolution — the user decides whether that shrinks this run or blocks it.
- **Ruling: defer the whole batch** → the run stops here, nothing built, nothing landed; state is left exactly as read.
- **Ruling: redeclare an attempt bound** → the new value carries for the rest of the run; everything else proceeds as above.
In every "proceed" branch, planning continues into Phase 4 for whatever scope survived the ruling.

## Phase 4 — Design phase (fires — gaps were named)
**Does:** Author exactly the two named gaps, nothing more, each seat working only on a plan the Delivery Manager approved first:
- Technical Analyst drafts the `GET /notes/search` contract delta (query param `q`, 400 on missing/short query, response array of `Note`, ranked newest-first) against `contracts/api.yaml`, and any data-model delta the search/index mechanism needs beyond the existing `Note` entity.
- Principal Architect resolves the store trip: proposes how a background index worker fits under the single-process constraint (C-001) — e.g., an in-process scheduled goroutine/thread rather than an external queue — as a new in-flight-class concern row (a prospective AX-004) plus updated topology notes, with a container-delta diagram per the system-design skill.
- QA Engineer drafts the **TEST:** cases these gaps imply (SC-101, SC-102, SC-103, the sub-2-second freshness bound, the short-query edge case, the in-flight-rebuild edge case).
Each design seat's rung-justification (why this much design and no more) is disclosed per the plan-minimalism ladder.
**Reads:** `sufficiency-report.md`, `.mochiko/specs/note-search/spec.md`, current `contracts/api.yaml`, `data-model.md`, `spine.md`, `constraints-and-decisions.md`.
**Writes:** `.mochiko/features/FEAT-002/contracts/api.yaml` (delta, appliable before/after form) · a data-model delta file if the worker/index needs modeling · a store delta at `.mochiko/product/architecture/` (new AX-004 in-flight row + diagram) · the map entry `.mochiko/features/FEAT-002/entry.md` gets the design-implied dependencies/extent and the architecture link filled once the store delta exists (status stays as the scope source set it — `selected`).
**Seats/skills:** Technical Analyst (API contracts, entity modeling), Principal Architect (system design, architecture store), QA Engineer (test-case authoring) — none of them is Staff Engineer, who never designs its own gaps.
**Gate:** none inline — review happens next, before the checkpoint.

## Phase 5 — Design review pair (before the checkpoint)
**Does:** A non-author seat grades the design output for conformance to the gap list and card-adjacent quality (blocking if it diverges materially from what was named as a gap), and a second pass grades feasibility/buildability/contradiction — this run's architecture pass fires because a store delta exists. Validator takes the conformance pass; Tech Lead — already independent of the spec/store/baselines and a natural fit given its architecture-store grading specialty — takes the feasibility pass, checking specifically whether the proposed in-process worker genuinely respects C-001 (single-process, no external services) and doesn't quietly reintroduce a queue.
**Reads:** the Phase 4 deltas, the sufficiency report, `constraints-and-decisions.md`.
**Writes:** two review notes/findings (feasibility verdict: feasible / needs-revision / infeasible; conformance verdict: ready / needs-revision / critical-gaps) — not full standalone report files unless either skill's own template calls for one.
**Seats/skills:** Validator (conformance/plan-artifacts review), Tech Lead (feasibility review).
**Gate:** none standalone — if either verdict is "needs-revision" or worse, Phase 4 iterates before the design checkpoint opens; only a clean pair proceeds to Phase 6.

## Phase 6 — Design checkpoint (user gate)
**Does:** Present the signed-off design deltas for the user's signature: the `/notes/search` contract addition, any data-model delta, and the store delta rendered as a diagram plus the changed-AX-row table (AX-004 proposed, its trigger and status) — since this is a structural change (a new element), it's presented, not silently applied.
**Reads:** the reviewed Phase 4 outputs.
**Writes:** nothing until signed; on signing, the store delta is legally allowed to exist beside the ruled spine content (the one carve for in-place-adjacent writes) pending the eventual landing fold.
**Gate — what's confirmed:** does the user accept this shape for search + the background worker, as drawn?
- **Sign as presented** → proceed to Phase 7 (cycle cards); the run may also pause here and resume the build later at the user's choice.
- **Amend** → back to Phase 4, scoped to just the requested change, then re-reviewed (Phase 5) and re-presented — repeats until signed or the user stops.
- **Reject the worker approach entirely** (e.g., decides FR-103 should be dropped or rescoped) → treated as a scope decision: W2 drops out of this run's build (a note goes back toward the spec/map for a future touch), and the run either continues build-only on W1 or the user chooses to defer the whole batch.
- **Stop / resume later** → run pauses cleanly at this checkpoint; no code has been written yet in any branch of this phase.

## Phase 7 — Cycle-card authoring
**Does:** QA Engineer (design-class, never the builder) slices the signed design into cycle cards per the vertical-TDD approach: foundation cycles before feature cycles. Concretely: a foundation cycle for the index storage + worker skeleton (needed by both rows), then a feature cycle for `GET /notes/search` (W1: SC-101, SC-102, short-query edge case), then a feature cycle for freshness (W2: SC-103, in-flight-rebuild edge case). Each card carries stories/rationale, dependencies, acceptance-criteria IDs, a real-infrastructure **TEST:** gate, and brownfield exposure — this touches the already-delivered `api-service` and `notes-db`, so each card is marked `[EXTEND]`, no task lists or file paths (the builder decomposes those at build time).
**Reads:** the signed design deltas, `spec.md`, `tasks.yaml` schema for the card template.
**Writes:** `.mochiko/features/FEAT-002/tasks.md` (new cycle cards, all unchecked).
**Seats/skills:** QA Engineer authoring; the vertical-TDD-cycles skill governs slicing and the `**TEST:**` grammar; the API-contracts and entity-modeling skills already informed the design the cards trace to.
**Gate:** none inline — card review happens next.

## Phase 8 — Card review before confirm
**Does:** An independent verification seat reviews the drafted cards for quality (conformance to the design/gap scope) and buildability, distinct from QA Engineer who authored them and from Staff Engineer who will build them. Validator takes this pass.
**Reads:** `tasks.md`.
**Writes:** a review note (quality/buildability judgment).
**Seats/skills:** Validator.
**Gate:** none standalone — feeds straight into Phase 9's user confirm.

## Phase 9 — Card confirm (user gate)
**Does:** Present the sliced cards (foundation → W1 → W2) for the user to rule on before any build starts.
**Gate — what's confirmed:** is this slicing right?
- **Approve as-is** → Phase 10 (build) begins in the given order.
- **Request re-slicing** (e.g., merge two cards, split one) → back to Phase 7, scoped to the requested change, re-reviewed (Phase 8), re-presented.
- **Defer one row** (e.g., build W1 now, hold W2 for further architecture discussion) → the confirmed card set shrinks to just the approved cards; the deferred row stays `selected` on the map for a later run.
In every branch, once a card set is approved, planning continues into Phase 10 for exactly that set.

## Phase 10 — Build, cycle by cycle
**Does:** Staff Engineer executes the approved cards in order (foundation cycle first, then W1, then W2), decomposing each into concrete tasks disclosed in that cycle's report, building test-first (red→green→refactor), following the brownfield-integration approach on every touch to the existing `api-service`/`notes-db` code, and disclosing the pre-code-reuse-ladder rungs at decomposition. Staff Engineer never designs its own gaps — if, say, the worker skeleton turns out to need a schema element nobody modeled, that cycle halts and Phase 4 (design) re-fires scoped to just that discovery, then re-checkpoints, before the cycle resumes.
After each cycle, an independent verification seat — QA Engineer, who already authored these `**TEST:**` cases at card time — runs them against real infrastructure (an actual SQLite file, an actual running HTTP server, the actual worker on a real clock for the 2-second bound) plus a reuse/minimalism read of the diff and cycle report against the surrounding code. Each grading pass consumes one of the cycle's (default 3) attempts; two consecutive rounds with unchanged findings halts that cycle and surfaces state to the user rather than continuing to spend attempts. A failing `**TEST:**` gate or quality-gate run fails the cycle outright — no severity triage. Minor findings default to a `BACKLOG.md` booking rather than an in-cycle fix; Important-or-above findings block the cycle and join the next checkpoint batch presented to the user. `tasks.md` checkboxes flip as each cycle passes.
**Reads (per cycle):** the relevant card in `tasks.md`, the signed design deltas, the existing `api-service`/`notes-db` code (not present in this workspace as inspected — the actual build would run against the real product repo).
**Writes (per cycle):** the application code changes themselves (not enumerable in advance — builder's call at decomposition) · `.mochiko/features/FEAT-002/cycle-report.md` (appended per cycle: decomposition, difficulties, deviations, any domain dependencies added) · a verification report per cycle.
**Seats/skills:** Staff Engineer (build), QA Engineer (per-cycle verification) — using the TDD-cycle execution skill, the brownfield-integration skill, and the code-minimalism ladder.
**Gate:** any Important-or-above finding, any infeasible-card discovery, or any build-time technical decision collides-with-a-ratified-constraint case escalates to the user in a batch at the next cycle checkpoint — not mid-cycle unless the build genuinely cannot proceed without a ruling.

## Phase 11 — Final validation (whole-build verification)
**Does, once every card is checked:**
- **Cold verification:** snapshot the current uncommitted working tree into an isolated worktree and re-run the full quality-gate suite from there, dependency-cold. (Caveat surfaced to the user: this workspace, as inspected, is not currently a git repository and has no `.gitignore` — the real build environment would need both before this step can run as specified; flagged as an absent-surface/precondition, not silently skipped.)
- **Regression sweep:** re-run FEAT-001's durable gate set — the three `**TEST:**` cases in `.mochiko/features/FEAT-001/gates.md` (restart-persistence, empty-body-400, get-by-id 200-vs-404) — since FEAT-002 reads FEAT-001's notes store and sits in its territory. Any failure here fails the whole run, no exceptions.
- **Gap-finding pass:** required, since this is a selection-scope run. A fresh Devils Advocate instance — one that authored none of these cycles and never saw the design-time `**TEST:**` cases — is dispatched blind in two messages: first, only `spec.md`, `sufficiency-report.md`, the design deltas, and the relevant baseline slices (never the code, `tasks.md`, the test cases, or any report); it states its own expected behavior; only then does it probe the running system. Findings split: anything showing spec-required behavior broken (cited against FR-101/102/103 or SC-101/102/103) fails final validation until resolved; anything beyond spec is advisory and goes to the user to fix now / book to `BACKLOG.md` / accept as designed. Anything the finder attributes to FEAT-001's own already-delivered territory routes out to a future `/mochiko:feature` delta card rather than being reworked here.
**Reads:** the built code, `tasks.md`, `gates.md` (FEAT-001), `spec.md`, the design deltas, the baselines.
**Writes:** `.mochiko/features/FEAT-002/final-validation-report.md` (cold-verification result, regression-sweep result, gap-finding findings and dispositions).
**Seats/skills:** Devils Advocate (blind gap-finding), QA Engineer or Tech Lead running the quality-gate/regression execution.
**Gate:** any spec-required-behavior finding, or any regression, routes back into a bounded rework loop (default 2 rounds at run scope, or charged against a single cycle's remaining attempts if the finding localizes there) before final validation can be called clean; exhausting that bound, or a round with unchanged findings, halts the run and presents state — disposition is the user's (carry on with a redeclared bound, accept partial scope, or stop).

## Phase 12 — Final acceptance (user gate) and landing
**Does:** Present the full readiness package: every card checked, test-first with independent per-cycle and whole verification, criteria traced to FR-101/102/103 and SC-101/102/103, governance alignment noted (none to conflict with, since no governance region exists here), rounds-consumed and seats-spawned summary, the store delta pending fold (AX-004), and the final-validation outcome.
**Gate — what's confirmed:** accept, amend, or reject.
- **Accept** → the landing executes as one whole operation, not written until this moment: the store delta's AX-004 flips to built and its FEAT-002 key clears, with As-built/Drift fields written and independently graded (Tech Lead) and the orphan check run, regenerating the derived `ARCHITECTURE.md`; the contract and data-model deltas fold into `.mochiko/product/contracts/api.yaml` and `.mochiko/product/data-model.md` via a graded three-way diff (Validator); any build-time technical-decision entries fold into `constraints-and-decisions.md` the same way; W1/W2 fold into FEAT-002's extent line and vanish as pending rows, status flips to `delivered` (dated) in both `.mochiko/features/FEAT-002/entry.md` and `FEATURES.md`; the note-search spec is read as closed since all its selected rows folded; any gap findings ruled fix-now or backlog fold into a newly minted `.mochiko/features/FEAT-002/gates.md`, authored by QA Engineer in the `**TEST:**` grammar. No git mutation is performed by the run itself — commits are suggested to the user, never made or pushed. The run then closes with an explicit verdict against the done condition, including a check that the schema's fail-condition count is still 15 (already confirmed at Phase 0/here).
- **Amend** → the requested change is scoped either as one more bounded rework round (charged to the gap-rework bound) or, if it's bigger than this run was framed for, escalated as a scope decision the user must explicitly accept before it proceeds — otherwise it stays out of scope and the run re-presents for acceptance on the narrower set.
- **Reject** → nothing lands; baselines and the map stay exactly as they were pre-run (nothing was ever edited in place); the run closes without acceptance, which is itself one of the conditions that would fail the done condition — the close verdict states that plainly rather than papering over it.