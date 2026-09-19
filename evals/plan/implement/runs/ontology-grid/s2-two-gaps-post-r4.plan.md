# Action Plan — `/mochiko:implement FEAT-002` (plan-only, not executed)

## Scenario facts established by reading, before planning

- `FEAT-002` resolves to a plain capability entry (not an `EPIC-XXX`) → **selection scope**, not epic/delta/lane.
- `.mochiko/features/FEAT-002/entry.md`: status `selected`, spec `.mochiko/specs/note-search/spec.md`, architecture link `— (not yet filled)`, two selected work rows — W1 (search by query, US-101, SC-101/SC-102) and W2 (index stays fresh, US-102, SC-103) — ratified 2026-08-26 from the spec's accepted selection. Dependency: FEAT-001, status `delivered` → does not block.
- `.mochiko/specs/note-search/spec.md` FR-103 requires a **background index worker** off the request path.
- `.mochiko/product/architecture/spine.md` states explicitly: *"Synchronous request/response only; no queues, no background workers."* This directly conflicts with FR-103 — a store trip/conflict to surface, not something to resolve unilaterally.
- `.mochiko/product/contracts/api.yaml` has no `/notes/search` path yet.
- `.mochiko/product/data-model.md` and `constraints-and-decisions.md` have no NFR/decision row for indexing or the 2s freshness bound.
- No `CLAUDE.md`, no `.claude/rules/mochiko/`, no `.mochiko/memory/knowledge-management.md`, no `.mochiko/memory/codebase-analysis.md`, no `.mochiko/memory/governance-ledger.md`, no `FEATURES.md`, no `.claude/worktrees` gitignore entry — all absent surfaces to be surfaced, not resolved, and not run-failing.
- `mochiko-cli` binary is not on PATH → `tasks.md` cycle cards render from the schema (`plugins/mochiko/schemas/tasks.yaml`), not the binary.

---

## Phase 1 — Pre-flight state scan

**Does:** Confirm the entry gates cleanly: capability exists, work rows selected, dependency status, spec accepted, and locate all baseline/store/governance surfaces (present or absent).
**Reads:** `.mochiko/features/FEAT-002/entry.md`, `.mochiko/features/FEAT-001/entry.md` (dependency check), `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, presence checks on `CLAUDE.md`, `.claude/rules/mochiko/`, `.mochiko/memory/knowledge-management.md`, `.mochiko/memory/codebase-analysis.md`, `.gitignore`.
**Writes:** nothing.
**Seats/skills:** locate/enumerate reads (directory listings, presence checks) route to a cheap `Explore` subagent under a `haiku` model override; the interpretive read of spec vs. spine (spotting the background-worker conflict) stays on the session tier — this is a judgment read, not an enumeration.
**Gate:** none.

## Phase 2 — Sufficiency check

**Does:** Grade each selected row (W1, W2) against the spec, the architecture store, and the product baselines, per the ten-clause sufficiency check, collapsing nothing (this is selection scope, not delta scope's three-clause form). Produces a binding per-row verdict: sufficient, or a named gap list.
**Reads:** the same artifact set as Phase 1, read fresh by the grading seat (never by summary).
**Writes:** `.mochiko/features/FEAT-002/sufficiency-report.md` — the store-consult result, any no-delta claim, the trips for the user, the `quickstart.md` null-path note (no real external-integration surface here — single-process, no external services), and no `[MODIFY]` amendment (nothing here touches a delivered feature's territory).
**Seats/skills:** a seat that authored none of spec/store/baselines and won't design or build this batch — e.g. `mochiko:validator` (kept distinct from `mochiko:devils-advocate`, reserved fresh and blind for the later gap-finding pass). Runs `mochiko:review-sufficiency`. Exempt from plan approval (a grading seat).
**Expected verdict (scenario-concrete, not asserted as final):** likely gaps — no `/notes/search` API contract, no architecture-store element/relationship for the background index worker (and its direct conflict with the spine's "no background workers" statement), no NFR concern row for the 2-second freshness bound. Any disputed clause the grader can't clear defaults to gap and goes to the user, never cleared by the grader alone.
**Gate:** none yet — the verdict is presented at run-open, not ruled here.

## Phase 3 — Run-open confirmation (the entry gate)

**Does:** One confirmation, no negotiation. Presents: the batch (FEAT-002, "Note search", rows W1+W2) and scope type (selection); both attempt bounds at their only redeclaration point — 3 verification attempts per cycle, 2 gap-rework rounds at final validation, both defaulted from schema `vars:` unless the user redeclares them now; the sufficiency verdict and its gap routing (design phase fires if any gap stands); the store trip — the FR-103 background-worker requirement vs. the spine's stated "no background workers" topology — for the user's ruling (ruled here, or deferred on the record); any other in-flight conflicts; the done condition (every cycle card checked, test-first, independently verified per-cycle and whole, code traces to requirements and governance, acceptance executed whole, run closes on accept/amend/reject).
**Reads:** the sufficiency report just written.
**Writes:** nothing (the confirmation itself is a conversational gate, not a file).
**Gate — user-owned, blocking, plain text (never a timed prompt):**
- *Confirmed as presented* → proceed to Phase 4 (design phase, since gaps stand) with the store trip ruled as the user directs (e.g. "build the worker as an in-process goroutine/thread, not an external queue — amend the spine wording, don't treat it as C-001 violation").
- *User rules the background worker out of scope for this run* (e.g. defers FR-103, wants synchronous-only search for v1) → scope shrinks; re-derive which of W1/W2 remain; if W2 is dropped, its row stays pending on the entry, not delivered — this changes the sufficiency verdict for W1 alone, which likely closes clean without a store delta, possibly skipping Phase 4 for a synchronous-only design.
- *User wants the capability re-specified first* → run aborts back to `/mochiko:feature` or `/mochiko:specify` rather than proceeding; no design or code follows.

## Phase 4 — Design phase (fires — gaps were named)

**Does:** Authors exactly the named gaps, nothing more, each on a plan the Delivery Manager approves first: (a) the API contract delta for `/notes/search` (query param, 200/400 responses, ranking behavior); (b) a technology decision for the search/index mechanism — since full-text indexing is a commodity category, an adopt-first check runs first (e.g. SQLite FTS5 vs. hand-rolled) with the actual commodity-category ruling reserved to the user, not the builder; (c) an architecture-store delta resolving the background-worker conflict per the user's Phase-3 ruling — a C4-container delta diagram plus a sequence diagram for the create→index→search flow (it qualifies as a flow), landing as a new/amended AX-XXX concern row (e.g. an "index worker" element and its NFR for the 2s bound) plus the topology text change; (d) the capability entry's design-implied dependencies/extent assertion with provenance, and the architecture link filled once the store delta exists.
**Reads:** `sufficiency-report.md`, the current spine and baselines, `spec.md` for cited acceptance criteria.
**Writes:** `.mochiko/features/FEAT-002/contracts/` delta (new search endpoint), `.mochiko/features/FEAT-002/data-model.md` delta if the index needs conceptual modeling, `.mochiko/features/FEAT-002/constraints-and-decisions.md`-style delta (new D-XXX for the index tech, new NFR-XXX for the 2s bound) — all beside their baselines, never edited in place; plus a store delta at `.mochiko/product/architecture/` per the store's own grammar (never merged into `spine.md` directly — stands beside it, signed, until landing folds it).
**Seats/skills:** `mochiko:technical-analyst` for the API-contract and technical-decision deltas (`mochiko:patterns-api-contracts`, `mochiko:patterns-technical-decisions`, `mochiko:patterns-adopt-first`); `mochiko:principal-architect` for the store delta (`mochiko:patterns-system-design`, `mochiko:authoring-architecture-store`); both plan first and work only on an approved plan, disclosing plan-minimalism rungs. Non-author review before the checkpoint: `mochiko:review-plan-artifacts` (conformance to the gap list, blocking) and `mochiko:review-feasibility` (buildability/contradiction — this is exactly where the worker-vs-spine conflict gets checked for resolution, not re-opened). Multi-seat composition here triggers the transport floor for any cross-seat messaging and the shared store-write surface.
**Gate — design checkpoint, user-owned:**
- *User signs the design and the store delta* (on the rendered diagram + the named AX-XXX row changes) → proceed to Phase 5; code writing may begin.
- *User asks for revision* → design seats rework within the same gap list (gap-rework bound applies at final-validation scope, not here — but a stuck design loop still surfaces to the user rather than iterating unbounded).
- *User stops here and defers the build* → run pauses cleanly; nothing downstream executes; resumable later at this checkpoint.

## Phase 5 — Cycle-card authoring

**Does:** Slices W1 and W2 into cycle cards (foundation cycles before feature cycles — e.g. an indexing-foundation cycle before the search-by-query feature cycle, if the design made indexing a prerequisite; or a single Simple cycle per row if no split is warranted). Each card: story/rationale, dependencies, acceptance criteria by ID (SC-101/102/103), a **TEST:** real-infrastructure gate, brownfield exposure (this extends the existing `api-service`/`notes-db` — a `[MODIFY]`/`[EXTEND]` touch on existing code, not greenfield). No task lists or file paths in the card — the builder decomposes those at build time.
**Reads:** the signed design deltas, `spec.md`.
**Writes:** `.mochiko/specs/note-search/tasks.md` (or wherever the feature's tasks file lives) rendered from the tasks schema.
**Seats/skills:** a design-class seat that is never the builder — e.g. `mochiko:technical-analyst` continuing from design — authors the cards; `mochiko:qa-engineer` authors the **TEST:** cases within its slicing (`mochiko:patterns-vertical-tdd`). The verification seat (`mochiko:qa-engineer`, kept distinct from the card author) reviews the cards for quality and buildability before confirm.
**Gate — card confirm, user-owned, blocking:**
- *User confirms the slicing* → build begins.
- *User asks for re-slicing* (e.g. merge two small cycles, or split one) → cards are re-authored and re-reviewed before a second confirm attempt.
- *User flags a card as infeasible* → escalated as a business-level scope decision, not a builder call; may shrink the batch back toward Phase 3's ruling.

## Phase 6 — Build cycles (test-first, per-cycle verification)

**Does:** For each confirmed card, in order: the builder decomposes into concrete tasks (disclosed in the cycle report), builds red→green→refactor against real infrastructure, follows brownfield-integration practice on the touched `api-service`/`notes-db` code, and discloses code-minimalism rungs at decomposition. Each cycle is then independently verified: real end-to-end evidence (actual HTTP calls, actual SQLite file, actual timing for the 2s bound — never mocked), plus the code-minimalism review lens over the diff and cycle report.
**Reads:** the signed design deltas, the confirmed `tasks.md`, existing `api-service`/`notes-db` source for brownfield fidelity.
**Writes:** the actual note-search implementation code; `.mochiko/features/FEAT-002/cycle-report-*.md` per cycle; verification reports; `tasks.md` checkbox flips as each card completes.
**Seats/skills:** `mochiko:staff-engineer` builds (`mochiko:executing-tdd-cycle`, `mochiko:brownfield-integration`, `mochiko:patterns-code-minimalism`); `mochiko:qa-engineer` verifies (`mochiko:testing-end-user`, `mochiko:review-code-minimalism`) — never the implementer.
**Attempt economy:** each grading pass on a cycle consumes one of the 3 per-cycle attempts (or the value the user redeclared at run-open); two consecutive rounds with unchanged findings is a no-progress stop — halt that cycle and present state to the user rather than continuing to retry.
**Gate (cycle checkpoint, not blocking by default):** reserved-to-user questions and findings batch here rather than interrupting mid-cycle, except a build-blocking question, which interrupts immediately. Minor findings default to a `BACKLOG.md` booking (note: no `BACKLOG.md` exists yet in this project — one would be minted at that point via the knowledge-management skill's discipline once it's needed, or simply created fresh, since `km_file` is currently absent and doesn't gate this). Important-or-above findings block the cycle and require a user ruling before continuing.

## Phase 7 — Final validation (whole-build verification)

**Does, since this is selection scope:**
- **Regression sweep:** re-runs FEAT-001's durable gate set (`.mochiko/features/FEAT-001/gates.md`, 3 TEST cases) plus any of this feature's gates exercising the FEAT-001 seam (search reads the notes store FEAT-001 built) — a failure here fails the run like any other regression.
- **Cold verification:** builds and runs the full quality-gate suite from a dependency-cold snapshot of the uncommitted working tree, copied to `.claude/worktrees/mochiko-<purpose>/` — first ensuring the `.claude/worktrees` gitignore entry exists (currently absent, so this run adds it as ordinary repo hygiene, not a baseline edit).
- **Gap-finding pass (runs — selection scope requires it):** a fresh, blind `mochiko:devils-advocate` dispatch, two-message protocol — first message carries only `spec.md`, `sufficiency-report.md`, the design deltas, and the baselines (never the code, `tasks.md`, TEST cases, or any report); the seat states its derived expectations before probing begins. Mutation lens only runs if governance depth reads `high` — with no governance region present here, depth is unresolved/absent, so the mutation lens is skipped with an explicit disclosure rather than silently omitted.
**Reads:** all cycle reports, the accumulated gate sets, the design deltas, `spec.md`.
**Writes:** `.mochiko/features/FEAT-002/final-validation-report.md` (regression results, cold-verification evidence, gap-finding findings split by kind — spec-required-and-broken fails the run, beyond-spec is advisory and the user disposes it as fix-now / backlog / accept-as-designed).
**Gate (folded into cycle-checkpoint-style batching, not a separate named gate):** any disputed finding-kind defaults advisory and goes to the user; the finder never gates alone. Gap-rework at this stage draws on the 2-round bound (or the user's redeclared value) unless a finding localizes to one cycle's own remaining attempts.

## Phase 8 — Landing (selection scope)

**Does:** Three-part store landing (the signed spine delta's elements flip to built, FEAT-002's key clears; the touched rows' As-built/Drift fields are written as judgment and independently graded by the landing verification seat; an orphan check runs over any in-flight element) plus the map's graduation batch: W1/W2 fold into FEAT-002's extent lines and disappear as rows; entry status flips to `delivered`, dated; the (currently nonexistent) `FEATURES.md` index gets its FEAT-002 line created/updated; the note-search spec's index row is touched, closing the spec once all its selected rows have folded. Every touched baseline (contracts, data-model, constraints-and-decisions, the store) folds exactly once via a graded three-way diff, checked by the landing verification seat — the store's fold is its own three-part process, not a diff. Gap findings ruled fix-now or backlog fold into a new `.mochiko/features/FEAT-002/gates.md`, authored in the TEST grammar by `mochiko:qa-engineer`; findings accepted as designed do not fold.
**Reads:** the final-validation report, the signed design/store deltas.
**Writes:** `.mochiko/product/architecture/spine.md` (folded), `.mochiko/product/contracts/api.yaml`, `.mochiko/product/data-model.md`, `.mochiko/product/constraints-and-decisions.md` (folded), the regenerated root `ARCHITECTURE.md` index, `.mochiko/features/FEAT-002/entry.md` (status → delivered), `FEATURES.md` (new/updated), `.mochiko/features/FEAT-002/gates.md`.
**Seats/skills:** the landing verification seat (independent, e.g. `mochiko:validator` or `mochiko:tech-lead` for the store judgment writes) checks every graded fold; `mochiko:authoring-architecture-store` and `mochiko:authoring-feature-map` own the mechanics.
**Note:** since `.mochiko/memory/knowledge-management.md` is absent, no additional knowledge-management landing obligations apply this run.

## Phase 9 — Final acceptance (closing gate)

**Does:** Presents the completed, verified build — every card checked, all evidence, the folded landing — for the user's ruling.
**Gate — user-owned, blocking, plain text:**
- *Accept* → run closes with a verdict against the done condition; FEAT-002 is delivered.
- *Amend* → the user specifies the change; the run reopens at the relevant phase (a cycle rework, a design tweak) rather than closing, and the affected phases above re-execute.
- *Reject* → run closes without acceptance; this alone is enough to fail the run's done condition (no other Not-done condition needs to also be true), and the work stays unaccepted pending a follow-up run.

---

## Cross-cutting notes for this specific run

- **Likely fail-risk points to watch, concretely:** the store landing lacking a built-vs-signed diff if the background-worker delta isn't carefully reconciled; a deviation left unresolved if the builder discovers the worker needs a boundary-crossing shape the signed delta didn't cover (this would halt that cycle and re-fire a scoped design-phase redo rather than being designed around silently); the regression sweep against FEAT-001's three gates, since search directly reads the store FEAT-001 built.
- **No git mutations** occur anywhere in this plan — only suggestions; the cold-verification worktree snapshot is the one filesystem copy involved, and it is self-removed, not a ref/history mutation.
- **All seat/sequencing choices above** (which seat drafts which delta, whether W1/W2 split into one or two cycles, foundation-vs-feature ordering) are run-level judgment calls within the schema's latitude, not fixed requirements — the concrete staffing named here is one reasonable instantiation, not the only legal one.