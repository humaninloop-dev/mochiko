# Action Plan — `/mochiko:implement` (plan-only, not executed)

**Invocation:** no capability ID was supplied in the command. Working directory contains a real `.mochiko/` tree (`notely` product), so the plan below is grounded in what's actually on disk rather than hypothetical.

---

## Phase 0 — Load the binding rules

**Done:** Read `plugins/mochiko/schemas/implement.yaml` raw and in full, plus `plugins/mochiko/schemas/common.yaml` (pulled in because several rules use `extends: common.*`) and `plugins/mochiko/schemas/command-labels.yaml`. This has already happened in this turn as the mandatory first action — no run gates open before it.

**Read:** the three schema files above.

**Write:** none.

**Verified:** the schema's fail-condition set contains exactly 15 `kind: fail` nodes, matching what the run's Not-done line expects. No halt needed on that check.

---

## Phase 2 — Resolve the target capability

**Done:** since no capability ID was given, treat this as the empty-argument path: propose the next ready capability from the map rather than guessing.

**Read:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/features/FEAT-002/entry.md`.

**Findings:** the map lists two capabilities — `FEAT-001` (Note capture, status *delivered*, no pending rows) and `FEAT-002` (Note search, status *selected*, two ratified work rows: W1 "search by query" and W2 "index stays fresh"). `FEAT-002`'s only dependency, `FEAT-001`, is already delivered, so it isn't blocked. `FEAT-002` is the sole candidate with ratified scope and is unblocked — it's the natural proposal.

**Gate (capability proposal):** present "`FEAT-002` — Note search, selection scope, rows W1/W2 — proceed?" to the user.
- *User confirms* → continue to Phase 3 against `FEAT-002`.
- *User names a different capability or a delta card instead* → restart resolution against that target (route to `/mochiko:specify` if it has no ratified selection yet, or to `/mochiko:feature` if it's meant to be a delta card).
- *User says nothing is ready / wants to abort* → run does not open; no further phases execute.

The rest of this plan assumes the first branch.

---

## Phase 3 — Sufficiency check

**Done:** grade guidance sufficiency for each selected row (W1, W2) against `FEAT-002`'s spec, the product architecture store, and the product baselines. This check is run by a seat that authored none of those documents and will not design or build this batch — a natural fit is `mochiko:validator` (independent, defaults to FAIL, never grades its own output); final staffing is a per-run call.

**Read:** `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`.

**Concrete findings surfaced by this read** (these drive Phase 5, not asserted as final grading, since the grading itself is not executed in this plan-only run):
- The spec's FR-103 requires "a background index worker" that rebuilds the search index off the request path. The ruled architecture spine states flatly: *"Synchronous request/response only; no queues, no background workers."* That's a direct topology contradiction — a strong candidate gap.
- `contracts/api.yaml` has no `/notes/search` path at all — the API contract layer is silent on the capability the spec describes.
- `data-model.md` has no entity or attribute for a search index — nothing to trace FR-101/FR-103 against.
- The architecture concern catalog (AX-001..003) has no NFR row for the 2-second create-to-searchable latency in SC-103.

**Write (would be produced by the grading seat, not by me here):** `.mochiko/features/FEAT-002/sufficiency-report.md` — per-row verdict (sufficient / gap list), any store-consult result, and any trips for the user.

**Skill involved:** `mochiko:review-sufficiency` (owns the clause set, gap forms, and the absent-baseline branch — not restated here).

Expected outcome given the findings above: both W1 and W2 come back with at least one gap, most likely all four items listed — which fires the design phase before any code is written.

---

## Phase 4 — Run-open confirmation (the entry gate)

**Done:** one confirmation bundling everything the schema requires at this single point, no negotiation beyond it.

**Would be presented to the user:**
- Batch and scope: `FEAT-002` / Note search, selection scope, rows W1 + W2.
- Attempt bounds (their only redeclaration point): 3 verification attempts per cycle, 2 gap-rework rounds at final validation — carried as defaults unless the user wants to change them now.
- The sufficiency verdict and gap routing from Phase 3 (design phase will fire).
- Trips and open conflicts for the user's ruling:
  - No governance region found (no `CLAUDE.md`, no `.claude/rules/mochiko/`) — surfaced, not run-failing; proceeding without a constitution overlay.
  - No `.mochiko/memory/codebase-analysis.md`, and this is effectively a brownfield repo (FEAT-001 is already built and running) — offer `/mochiko:setup` first, or proceed greenfield-style with the warning logged.
  - No governance depth is resolvable (no governance region to read it from), so whether the deeper mutation-testing lens runs at final validation can't be determined from the repo alone — flagged for the user to rule on, or it will be treated as a disclosed skip.
  - A likely collision: satisfying FR-103's background worker may require a scheduling/queueing mechanism. If any commodity candidate considered for that needs an external service, it collides with the ratified single-process constraint (C-001) — that specific decision would pause for the user later; the rest of the run proceeds around it.
- Done condition stated plainly: every cycle card checked, built test-first, independently verified against real infrastructure per cycle and as a whole; code traces to FEAT-002's acceptance criteria and aligns with governance; the acceptance landing executes whole; the run closes on the user's accept/amend/reject.

**Gate:** the user rules on all of the above as one confirmation.
- *User approves as presented* → proceed to Phase 5 with defaults (3/2 attempt bounds, mutation lens treated as a disclosed skip, proceed brownfield-without-fresh-analysis logged as a warning).
- *User changes an attempt bound* → that value is fixed for the rest of the run (no further redeclaration point).
- *User asks to run `/mochiko:setup` first* → this run pauses/exits in favor of that command; no design or build work starts.
- *User rules on the depth/mutation-lens ambiguity explicitly* → that ruling is carried forward and applied at Phase 8 instead of the disclosed-skip default.
- *User rejects opening the run* → stop; nothing further executes.

Remaining phases assume approval.

---

## Phase 5 — Design phase (fires: gaps were named)

**Done:** author exactly the named gaps, nothing more, each seat working only on a plan the delivery manager approves first. No code is written yet.

**Seats/skills likely involved:**
- `mochiko:principal-architect` — the architecture-store delta: whether the background index worker is a new topology element or an added responsibility inside the existing `api-service` container, updating the "no background workers" line, adding the NFR row for the 2-second freshness target. Uses `mochiko:patterns-system-design` for the delta diagram/register and `mochiko:authoring-architecture-store` for grammar and lifecycle. Also runs the commodity-category check (`mochiko:patterns-adopt-first`) for the scheduling mechanism — in-process (stdlib ticker/goroutine, consistent with the existing no-framework precedent) is the natural default; anything needing an external service collides with C-001 and gets filed as a constraint-challenge finding reserved to the user rather than silently built.
- `mochiko:technical-analyst` — the API contract delta (`GET /notes/search` added to `contracts/api.yaml`, via `mochiko:patterns-api-contracts`) and the data-model delta (a search-index entity or indexed-attribute addition, via `mochiko:patterns-entity-modeling`) in appliable before/after form.
- `mochiko:qa-engineer` — typically drafts the **TEST:** case shapes that will anchor the cycle cards (SC-101/102 query behavior, SC-103 freshness-under-2s).

Every design element is run through the simplest-execution ladder (`mochiko:patterns-plan-minimalism`) before being authored, and each producing seat plans first and works only on a plan I'd approve as delivery manager.

**Write (by the design seats, not by me here):** deltas beside their baselines at `.mochiko/features/FEAT-002/` — a `data-model.md` delta, an `api.yaml`/contract delta, plus an architecture-store delta (never editing `spine.md` in place). `FEAT-002/entry.md` also gets the design-implied dependencies and sharpened extent asserted with provenance, and its architecture link filled once the store delta exists.

**Review pair (non-author seats, before the checkpoint):** `mochiko:review-plan-artifacts` for conformance to the gap list and card quality (blocking), `mochiko:review-feasibility` for buildability/contradiction — and since a store delta exists, its architecture pass runs too.

**Gate (design checkpoint — floor, non-waivable):** present the rendered architecture delta diagram plus the changed-element table (new/changed AX rows), the API contract delta, the data-model delta, and the review pair's findings.
- *User signs as presented* → proceed to Phase 6.
- *User asks for changes* → the relevant design seat revises on a fresh approved plan and re-presents; no bound on redesign rounds is stated beyond ordinary latitude.
- *User stops here to resume later* → explicitly permitted; run pauses with no code written, state preserved for a later resume.
- *A constraint-challenge surfaced during design (external-service collision with C-001)* → only that specific decision pauses for the user; the rest of the design phase proceeds.

---

## Phase 6 — Card authoring and card confirm

**Done:** a design-class seat (never the builder) slices the signed design into cycle cards — foundation cycles before feature cycles, per vertical-TDD slicing (`mochiko:patterns-vertical-tdd`). Given the two rows, a plausible shape: a foundation cycle for the index-worker/index-storage plumbing, then a feature cycle for W1 (search endpoint against SC-101/SC-102), then a feature cycle for W2 (freshness round-trip against SC-103) — but the actual Simple/Split/Merge call is the authoring seat's judgment, not preset here.

**Write:** `.mochiko/features/FEAT-002/tasks.md`, rendered from the tasks template/schema — cards carrying stories, dependencies, acceptance-criteria IDs, a **TEST:** real-infrastructure gate per card, and brownfield exposure (this touches the existing `api-service`/`notes-db`, so `[EXTEND]` classification applies). `mochiko:qa-engineer` authors the **TEST:** cases within the slicing.

**Independent review before confirm:** a verification seat (not the card author) reviews the cards for quality (`mochiko:review-plan-artifacts`) and buildability.

**Gate (card confirm — floor, non-waivable):** present the card slicing to the user.
- *User approves* → proceed to Phase 7.
- *User wants a different slicing (merge/split cards)* → the authoring seat re-slices and re-presents.
- *User finds a card infeasible as scoped* → escalates as a business-level scope decision before build starts.

---

## Phase 7 — Build, cycle by cycle

**Done, per cycle:** `mochiko:staff-engineer` decomposes the card into concrete tasks (disclosed in the cycle report), drives each through red→green→refactor test-first, applies the pre-code minimalism ladder at decomposition (`mochiko:patterns-code-minimalism`) and the brownfield-safe-touch discipline (`mochiko:brownfield-integration`) since this extends the running `api-service`/`notes-db`. Locate/enumerate reads route to a cheap Explore-style dispatch; interpretive reads stay on the main seat.

**Write (by the builder, not by me here):** working code changes to the notely service; `.mochiko/features/FEAT-002/tasks.md` checkbox flipped per completed card; `.mochiko/features/FEAT-002/cycle-report-<n>.md` per cycle (decomposition, difficulties, deviations, any new domain dependencies).

**Verification, per cycle:** an independent seat (e.g. `mochiko:qa-engineer`, never the builder) runs the card's **TEST:** gate against real infrastructure (the actual service, the actual SQLite file, a real create-then-search round trip timed against the 2-second bound) via `mochiko:testing-end-user`, applies the code-minimalism review lens, and runs the full quality-gate suite. Up to 3 grading attempts per cycle (or the value set at Phase 4); two consecutive rounds with unchanged findings halts that cycle and presents state to the user rather than continuing silently. Minor advisory findings default to a backlog booking; Important-or-above findings block the cycle and join the next checkpoint batch.

**Contingent gate (mid-build):** if the builder hits structure the design didn't cover, that cycle halts and the design phase re-fires scoped to just the discovery — re-running Phase 5's review and design-checkpoint gate before the cycle resumes. If a structural change (new box, new/removed/redirected arrow, moved responsibility) is needed beyond what was signed, it stops and is presented: build as approved, or get the user's ruling to amend the delta first — never resolved silently.

**Cycle checkpoint:** accumulated findings and any reserved-to-user questions land as one batch at each cycle's checkpoint; only a truly build-blocking question interrupts mid-cycle.

This phase repeats until every card in `tasks.md` is checked.

---

## Phase 8 — Final validation (whole-build pass)

**Done** (selection scope, so nothing here is skippable):
- **Regression sweep:** re-run `FEAT-001`'s durable gate set (`.mochiko/features/FEAT-001/gates.md` — restart-persistence, empty-body 400, get/404) plus this feature's own gates exercising the seam onto the already-delivered notes store.
- **Cold verification:** build and run the quality gates from a dependency-cold snapshot of the current uncommitted working tree, copied to `.claude/worktrees/mochiko-<purpose>/` (first checking/adding the `.claude/worktrees` ignore entry). This snapshot is ephemeral and self-removed — not a git mutation.
- **Gap-finding pass** (required for selection scope): a fresh `mochiko:devils-advocate` instance, dispatched blind in two messages — first carrying only `spec.md`, the sufficiency report, the design deltas, and the baselines (data-model, contracts, NFR concern rows), never the code, `tasks.md`, TEST cases, or any report. The seat states its expectations, then probes (including the mutation lens, if the Phase 4 ruling turned it on).

**Read:** the same design-time artifacts named above, staged for the blind dispatch; the accumulated gate files; the fresh working-tree snapshot.

**Write:** `.mochiko/features/FEAT-002/final-validation-report.md` (or equivalent per the reports envelope) — regression results, cold-verification results, and gap-finding findings split by kind (spec-required-behavior-broken vs. beyond-spec-advisory), with a disputed kind defaulting advisory and going to the user.

**Contingent handling:** spec-required findings must be resolved (bounded by 2 gap-rework rounds at run scope, or the localized cycle's remaining attempts if the finding is confined to one cycle's territory); exhausting the bound or hitting unchanged findings halts the run and presents state for the user's disposition. Beyond-spec findings' disposition (fix now / book to backlog / accept as designed) is the user's call, taken at the next checkpoint batch rather than assumed.

---

## Phase 9 — Landing (executes whole, at acceptance)

**Done:** fold everything graded above, in one pass, once the user accepts (Phase 10) — not staged incrementally.

**Write (by the relevant seats, not by me here):**
- Architecture store: the signed delta's elements flip to built, `FEAT-002` keys clear, the touched rows' As-built/Drift fields are written and independently graded, the orphan check runs, and the derived `ARCHITECTURE.md` is regenerated by the store skill (never hand-edited).
- Product baselines (`data-model.md`, `contracts/api.yaml`) fold via a graded three-way diff (pre-fold baseline + delta vs. folded result).
- Map graduation batch (selection scope): W1 and W2 fold into `FEAT-002/entry.md`'s extent and vanish from work rows; status flips to delivered, dated; `FEATURES.md`'s `FEAT-002` line updates; the note-search spec reads closed since both its selected rows folded.
- Gates fold: any findings ruled fix-now or backlog fold into `.mochiko/features/FEAT-002/gates.md` (minted if it doesn't already exist), authored in the **TEST:** grammar by `mochiko:qa-engineer`; anything accepted as designed does not fold.
- Knowledge-management landing: not applicable — no `.mochiko/memory/knowledge-management.md` exists in this repo.

**Verification:** the landing verification seat checks each graded fold before/alongside the acceptance ruling.

---

## Phase 10 — Final acceptance (the closing gate)

**Gate (floor, non-waivable):** present the completed build, the final-validation results, and the landing package for accept / amend / reject, as plain blocking text — never a timed prompt.
- *Accept* → proceed to Phase 11 as a closed, successful run.
- *Amend* → the user specifies what to change; the run loops back to whichever phase the amendment actually targets (a build fix re-enters Phase 7 under its attempt economy, a scope change requires explicit user acceptance of the larger scope or the run stays FAIL for that portion).
- *Reject* → the run closes without executing landing (or, if landing already executed as part of "whole" execution, that becomes the point of failure to report); state is presented, nothing further is pushed or mutated.

---

## Phase 11 — Close with verdict

**Done:** state a verdict against the done condition, checking that none of the 15 fail-conditions stand — sufficiency was recorded, gaps (if any) were designed and signed, no card was built by its own author or before its confirm, no card is left unchecked, no quality gate failed, no verification claim lacks real-infrastructure evidence, no regression in `FEAT-001`'s gates, no baseline was edited in place outside a graded fold, no unresolved architecture deviation, the store delta landed with its built-vs-signed diff (or none was needed), every touched baseline has its graded fold, the gap-finding pass ran (it must, for selection scope) and any spec-required finding is resolved, and user acceptance was actually given.

**Write:** none beyond what Phase 9 already produced — this is a reporting step, following the project's output-style register.

**No further gates** — this is the terminal phase.