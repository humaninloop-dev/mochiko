# Action Plan — `/mochiko:implement` (no argument supplied)

## Phase 0 — Load the binding rules (mandatory first action)
- **Done:** Read `plugins/mochiko/schemas/implement.yaml` raw, in full, and `plugins/mochiko/schemas/common.yaml` raw, in full (co-read because `implement.yaml` carries five `extends: common.*` stubs). Confirmed the `kind: fail` count in `impl.sec.fail-conditions` is exactly 15, matching the command's Not-done line — no halt needed on that check.
- **Read:** `plugins/mochiko/schemas/implement.yaml`, `plugins/mochiko/schemas/common.yaml`, `plugins/mochiko/commands/implement.md` (narrative).
- **Written:** nothing.
- **Seats/skills:** none yet — this is the run lead's own obligation, not a seat spawn.
- **Gate:** none.

## Phase 1 — Resolve entry: `$ARGUMENTS` is empty
- **Done:** Since no `FEAT-XXX`/`EPIC-XXX` was passed, survey the map for the next ready capability instead of guessing.
- **Read:** `FEATURES.md` (two rows: `FEAT-001` delivered 2026-08-22, `FEAT-002` selected), `.mochiko/features/FEAT-001/entry.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/specs/note-search/spec.md`.
- **Finding:** `FEAT-002` ("Note search") is the only capability carrying ratified, undelivered work rows — W1 (search by query, US-101, SC-101/SC-102) and W2 (index stays fresh, US-102, SC-103), selection source: the spec's accepted selection (2026-08-26). Its one dependency, `FEAT-001`, is already `delivered`, so it doesn't block. Scope type resolves to **selection** (not delta, not epic — `$ARGUMENTS` named no `EPIC-XXX`).
- **Written:** nothing yet — this is a proposal, not a commit.
- **Gate A (pre-formal, entry-resolution confirmation):** Present "propose `FEAT-002` / Note search, rows W1+W2, selection scope" to the user.
  - **If confirmed:** proceed to Phase 2 against `FEAT-002`.
  - **If the user names a different capability or `EPIC-XXX` instead:** re-resolve entry against that ID (epic path would additionally resolve members via `mochiko:authoring-epic` lookup — not applicable here since none was named).
  - **If the user says neither is ready / wants something new:** route out of this run — a brand-new capability to `/mochiko:specify`, a feature-keyed delta to `/mochiko:feature` — and this run ends without opening.

## Phase 2 — Sufficiency check (entry-gated, before run-open)
- **Done:** Grade each selected row (W1, W2) against the spec, the architecture store, and the product baselines, per `mochiko:review-sufficiency`, run by a seat that authored none of those sources and will not design or build this batch. Staffing candidates otherwise slated for this batch — `technical-analyst` (design_seat), `principal-architect` (architect_seat), `qa-engineer` (qa_seat), `staff-engineer` (builder_seat) — are all disqualified by that independence rule, so I'd staff `mochiko:validator` here (unclaimed by any other role at this point in the run).
- **Read:** `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`.
- **Concrete gaps this grading would surface:**
  1. **API contract gap** — `contracts/api.yaml` defines only `POST /notes` and `GET /notes/{id}`; there is no `GET /notes/search` operation, so W1's FR-101/FR-102 (query param, 400 on missing `q`, ranked results) have no contract to build against.
  2. **Architecture contradiction** — the ruled spine states flatly "Synchronous request/response only; no queues, no background workers," while FR-103 (W2) requires "a background index worker" that rebuilds off the request path. This is not a coverage gap but an active conflict between ruled baseline and required behavior — it needs a store delta, not just an addition.
  3. **Possible data-model gap** — no entity/notes about the search index structure; the grader would judge whether that's implementation detail (no baseline needed) or needs a documented shape.
- **Written:** `sufficiency-report.md` in `.mochiko/features/FEAT-002/` — verdict (row → sufficient / gap list), the store-consult result, any no-delta claim, trips for run-open, the `quickstart.md` null-path note (no real external-integration surface here), any absent-baseline branch notes.
- **Gate:** none yet standalone — a disputed clause would default to "gap" and route to the user (`impl.sufficiency-disputed-clause`), folded into Gate 1 below.

## Phase 3 — Run-open confirmation (the entry gate)
- **Done:** One confirmation, no negotiation, presenting:
  - Batch and scope: `FEAT-002`, selection scope, rows W1 + W2.
  - Attempt bounds at their only redeclaration point: per-cycle = 3 (`attempt_bound_cycle`), gap-rework = 2 (`gap_rework_bound`), unless the user redeclares now.
  - The sufficiency verdict and its gap routing (the contract gap, the architecture contradiction, and the possible data-model gap from Phase 2).
  - Trips/conflicts for the user's ruling: the ruled-spine-vs-FR-103 contradiction; the missing governance region (no `CLAUDE.md` found anywhere in the repo — absent surface, surfaced per `impl.absent-surfaces`, never auto-resolved, never run-failing); the absence of `.mochiko/memory/knowledge-management.md` (so landing will carry no KM obligations); and a call-out risk — `FEAT-001` is marked `delivered` in its entry but no application source code exists anywhere in the repository tree (only plugin/schema/spec scaffolding was found) — worth flagging since it affects what the regression sweep and cold-verification snapshot will actually build against.
  - Done condition stated: every cycle card `[x]`, test-first, independently verified against real infrastructure per-cycle and whole; code meets criteria, traces to requirements, aligns with governance; landing executed whole; run closes at final acceptance; none of the 15 fail conditions standing.
- **Written:** nothing new (the confirmation itself is recorded in the run's user-facing register, not a separate file).
- **Gate 1 (`run-open`, blocking, plain text):**
  - **Confirmed as-is:** proceed — since gaps were found, straight into Phase 4 (design phase); no code is written first.
  - **User redeclares attempt bounds:** new bounds apply for the rest of the run.
  - **User rules on the store contradiction now:** e.g. explicitly approves adding a background-worker element in principle — recorded, and the architect's design-phase delta implements that ruling rather than re-litigating it.
  - **User rules to defer the contradiction to the design checkpoint:** the architect drafts the reconciling delta and it's presented fresh at Gate 2.
  - **User halts here:** run does not open; state is presented; nothing downstream fires.

## Phase 4 — Design phase (fires — two named gaps: contract + architecture)
- **Done:** Author exactly the named gaps, nothing more, each on a plan I approve first (design seats are plan-approval producers, not exempt).
  - `technical-analyst` (design_seat): drafts the `GET /notes/search` contract delta against `contracts/api.yaml` (query param, 200/400 responses, ranked-results schema) and any data-model delta needed for the index shape.
  - `principal-architect` (architect_seat): drafts the architecture-store delta reconciling the "no background workers" spine text with FR-103 — a new container element (index worker), the arrow(s) connecting it to `notes-db`, and a new `AX-004` concern row carrying the freshness NFR (≤2s, SC-103) — scoped and diagrammed per `mochiko:patterns-system-design`, grammar per `mochiko:authoring-architecture-store`. Given C-001 (single-process, no external services), this delta would need to justify an in-process worker rather than an external job-queue service — a natural point where `mochiko:patterns-adopt-first` gets consulted (commodity-category check on "background job scheduling"); if a shelf candidate would collide with C-001, that's a constraint-challenge finding routed to the user, not a silent override.
  - `qa-engineer` (qa_seat): drafts the design-time `**TEST:**`-shaped acceptance cases for W1/W2 to seed the later card-authoring pass.
  - Since neither `contracts/api.yaml` nor `spine.md` is a wholly absent baseline (both exist, both are being extended/amended), the absent-baseline seed rule does not apply — these are ordinary deltas beside existing baselines, never edited in place.
- **Read:** `sufficiency-report.md`, `spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/data-model.md`, `.mochiko/product/constraints-and-decisions.md`.
- **Written:** deltas under `.mochiko/features/FEAT-002/` beside their baselines (a contract delta, a data-model delta if warranted, the architecture-store delta with its rendered diagram + changed-`AX-XXX`-row table), plus the map-entry assertion (dependencies/extent sharpened onto `FEAT-002`'s entry with provenance, architecture link filled).
- **Non-author review pair before the checkpoint:**
  - `mochiko:validator` runs `mochiko:review-plan-artifacts` — conformance to exactly the two named gaps, card-quality precursor checks (blocking).
  - `mochiko:tech-lead` runs `mochiko:review-feasibility` — specifically hunting the spine-vs-FR-103 contradiction for buildability/contradiction, and independently grading the architecture-store judgment content (its own described specialty) before the store delta reaches sign-off.
- **Gate 2 (`design-checkpoint`, blocking, plain text) — the user signs the design and the store delta:**
  - **Signed as-is:** proceed to Phase 5 (card authoring); the store delta stands beside the ruled spine as an in-flight-class element until landing.
  - **Amend (e.g., reject the in-process worker shape, or push back on the AX-004 wording):** design phase re-fires scoped to just that amendment, re-reviewed by the same non-author pair, re-presented — doesn't restart the whole phase.
  - **User stops here:** explicitly allowed — the run may pause and resume the build later; no code has been written.

## Phase 5 — Cycle-card authoring + card confirm
- **Done:** A design-class, non-builder seat (`technical-analyst`, continuing from design) slices W1 and W2 into cycle cards per `mochiko:patterns-vertical-tdd` — foundation cycles before feature cycles, so a plausible slicing is: Cycle 1 = index-worker plumbing + notes-db read path (foundation), Cycle 2 = `GET /notes/search` endpoint behavior (SC-101/SC-102), Cycle 3 = create-to-searchable freshness bound (SC-103) — final slicing is the authoring seat's call, not mine to fix here. `qa-engineer` authors the `**TEST:**` real-infrastructure gate on each card within that slicing. Cards carry stories/rationale, dependencies, acceptance-criteria IDs, brownfield exposure (this territory touches `FEAT-001`'s delivered `notes-db`/API surface — `[EXTEND]`), no task lists or file paths.
- **Read:** the signed design deltas, `spec.md` for cited acceptance-criteria IDs.
- **Written:** `.mochiko/features/FEAT-002/tasks.md` from the tasks template (or `plugins/mochiko/schemas/tasks.yaml` raw if the render binary is absent).
- **Independent review before confirm:** `qa-engineer`, as the verification seat, reviews card quality (`mochiko:review-plan-artifacts`) and buildability (own judgment) before the cards reach the user.
- **Gate 3 (`card-confirm`, blocking, plain text):**
  - **Confirmed as-is:** build begins at Cycle 1.
  - **Amend (re-slice, split/merge, reorder):** re-sliced by the same authoring seat, re-reviewed, re-presented — never built pre-confirm (a card built before this gate or by its own author is one of the 15 fail conditions).
  - **Reject:** halt back to Phase 4 or Phase 1 to rescope, per the user's stated reason.

## Phase 6 — Build: TDD cycles, per-cycle verification
- **Done, per cycle:** `staff-engineer` (builder_seat) decomposes the card into concrete tasks at build time (disclosed in the cycle report), runs `mochiko:patterns-code-minimalism`'s pre-code ladder at decomposition, follows `mochiko:brownfield-integration` for the `[EXTEND]` touches to `FEAT-001`'s code, and drives red→green→refactor test-first per `mochiko:executing-tdd-cycle`. `qa-engineer` then verifies against real infrastructure per `mochiko:testing-end-user` (never mocks) plus the `mochiko:review-code-minimalism` lens over the diff and cycle report — never clearing its own or the builder's claims on trust.
- **Read (per cycle):** the card, the signed design deltas, existing FEAT-001 code paths being extended.
- **Written (per cycle):** working code; `cycle-report.md` in `.mochiko/features/FEAT-002/` (decomposition, honest difficulties, deviations, `domain_deps_added`); `tasks.md` checkbox flip on pass.
- **Attempt economy (floor):** each grading pass by `qa-engineer` consumes one of the 3 per-cycle attempts; two consecutive rounds with unchanged findings is a no-progress stop (halt, present state); exempting a round from the count is the user's call only.
- **Mid-cycle escalation:** if the worker's implementation collides with C-001 or a commodity-category candidate is on the table, that adopt-first/IP-XXX ruling and any infeasible-card discovery halt to the user immediately (build-blocking); everything else batches to the cycle checkpoint.
- **Gate (cycle-checkpoint, batched, recurring — not a single accept/amend/reject):** presents accumulated escalations (e.g., "in-process ticker vs. an off-the-shelf scheduler for the index worker" if not already settled at Gate 2) and Important-or-above findings.
  - **User rules per item:** approve the builder's approach / pick an alternative / defer as designed — each ruling unblocks just that item; a Minor finding defaults to a `BACKLOG.md` booking without a ruling needed.
- **Deviation check:** if a cycle needs to add/remove a box or arrow beyond the signed delta, that cycle stops and is presented — build as approved, or the user amends the delta first; never silently redesigned.

## Phase 7 — Final validation (whole-build)
- **Done:** Quality gates run the full repository suite; regression sweep re-runs `FEAT-001`'s durable gate set (`.mochiko/features/FEAT-001/gates.md` — restart-survival, empty-body 400, 404-on-random-id) plus any seam `FEAT-002` now exercises; cold-verification builds and gates from a dependency-cold snapshot of the uncommitted working tree at `.claude/worktrees/mochiko-<purpose>/` (ensuring the ignore entry exists first) — this is where the earlier-flagged "no source code found" anomaly would surface concretely, since the snapshot needs real, buildable code for both `FEAT-001` and the new work. Because scope is **selection**, the blind gap-finding pass fires: a fresh `devils-advocate` instance — never one that touched this feature's design or cycles — is dispatched two-message-blind per `mochiko:testing-gap-finding`, first message carrying only `spec.md`, the sufficiency report, design deltas, and the relevant baselines (never code, `tasks.md`, `**TEST:**` cases, or reports).
- **Read:** the accumulated durable gate sets, the cold snapshot's built code, the design deltas (for the blind dispatch's first message).
- **Written:** `final-validation-report.md` in `.mochiko/features/FEAT-002/` — gate results, regression results, cold-verification evidence, gap-finding findings split by kind (spec-required-broken vs. beyond-spec), any disputed kind defaulted advisory and routed to the user.
- **Gate (folded into escalation batching / final acceptance):** spec-required findings left unresolved would fail the run outright; beyond-spec findings' disposition (fix now / backlog / accept as designed) is the user's call, gathered here.
- **Gap-rework bound:** default 2 rounds at run scope (or charged against a cycle's remaining attempts if the finding localizes); exhaustion or no-progress halts the run and presents state, disposition the user's.

## Phase 8 — Landing (executed whole, only at acceptance)
- **Done:** Store landing — the delta's elements (the new index-worker container, `AX-004`) flip to built, `FEAT-002` keys clear, the touched rows' `As-built:`/`Drift:` fields written as judgment and independently graded (by `tech-lead`, not `principal-architect` who authored the delta), orphan check run, derived `ARCHITECTURE.md` regenerated by the store skill. Map graduation batch (selection scope): W1/W2 fold into `FEAT-002`'s extent lines and vanish from the work-rows list, status flips to `delivered` dated, `FEATURES.md` index line updates, the `note-search` spec's closed-status is derived (all its selected rows folded). Any baseline delta (the API contract, data model) folds via a three-way diff (pre-fold baseline + delta vs. folded result), graded by the landing verification seat. Gap findings ruled fix-now/backlog fold into `.mochiko/features/FEAT-002/gates.md` (minted fresh), authored in `**TEST:**` grammar by `qa-engineer`. No KM-landing step (the KM file is absent).
- **Read:** the final-validation report, all signed deltas.
- **Written:** `.mochiko/product/architecture/spine.md` (folded), `ARCHITECTURE.md` (regenerated), `.mochiko/product/contracts/api.yaml` and `data-model.md` (folded), `FEATURES.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/features/FEAT-002/gates.md`.
- **Gate:** landing only executes after Gate 5 below rules "accept" — it does not run speculatively.

## Phase 9 — Final acceptance
- **Done:** Present the whole done-condition package: all cards `[x]`, final-validation evidence, the landing package about to execute.
- **Gate 5 (`acceptance`, blocking, plain text — closes the run):**
  - **Accept:** Phase 8's landing executes whole; run closes with verdict **DONE** against the fixed done condition, provided none of the 15 fail conditions stand.
  - **Amend:** a scoped rework loop opens (charged to the gap-rework bound or the relevant cycle's remaining attempts), re-running only the affected final-validation subset, then re-presenting this same gate.
  - **Reject:** run closes without landing; state presented; the batch's rows remain unfolded for a later run to pick up.

## Phase 10 — Close
- **Done:** State the verdict against the fixed done condition and check all 15 `kind: fail` conditions from `impl.sec.fail-conditions` — sufficiency recorded, design phase not skipped where gaps existed, card independence honored, no card left unchecked, no failing quality gate, no unevidenced verification claim, no regression, no baseline edited in place, no unresolved deviation, store landing complete with its built-vs-signed diff, every touched baseline graded-folded, gap-finding pass present (selection scope — mandatory here, not skippable), no unresolved spec-required gap finding, acceptance given. Any one standing → FAIL, stated plainly with which clause tripped.
- **Written:** nothing further — this is the closing report, itself following `templates/report-format.md`.