# Action Plan — `/mochiko:implement FEAT-002` (plan-only, not executed)

## Grounding read (already performed, informs every phase below)

Read raw and in full: `plugins/mochiko/schemas/implement.yaml` (D6/D14 grammar, 15 `kind: fail` nodes confirmed — count matches the Not-done line) and `plugins/mochiko/schemas/common.yaml` (four `extends:` targets this run pulls: `tools-referenced-never-restated`, `register`, `author-grader-default-fail`, `model-tiering`, `transport-floor`, `no-acceptance`). Also read the current fixture state: `.mochiko/features/FEAT-002/entry.md`, `.mochiko/features/FEAT-001/{entry,gates}.md`, `.mochiko/specs/note-search/spec.md`, `.mochiko/product/{data-model.md,constraints-and-decisions.md,architecture/spine.md,contracts/api.yaml}`, root `ARCHITECTURE.md`. Confirmed absent: `CLAUDE.md` / `.claude/rules/mochiko/` (no governance region), `.mochiko/memory/` entirely (no `codebase-analysis.md`, no `knowledge-management.md`, no `governance-intent.md`), and a root `.mochiko/FEATURES.md` map index.

---

## Phase 1 — Entry gating

**Does:** Resolves `FEAT-002` as a capability ID (not `EPIC-XXX`, so no epic lookup). Confirms it carries selected work rows with ratified scope.
**Reads:** `.mochiko/features/FEAT-002/entry.md` — status `selected`, scope source is the spec's *accepted selection* (2026-08-26) → **scope = selection**, not delta. Work rows: W1 (search by query, US-101/SC-101/SC-102) and W2 (index freshness, US-102/SC-103). Dependency line: `FEAT-001 (delivered)` — checked against `.mochiko/features/FEAT-001/entry.md` (status `delivered`) → dependency satisfied, no block.
**Writes:** none this phase.
**Seats/skills:** none spawned yet — DM-only read.
**Gate:** none here; this phase only establishes what will be presented at the run-open gate in Phase 3.

## Phase 2 — Sufficiency check

**Does:** Dispatches the ten-clause sufficiency check per selected row (W1, W2), per `mochiko:review-sufficiency`, to a seat that authored none of spec.md, the architecture store, or the product baselines. Candidates ruled out by authorship: `technical-analyst` (likely author of data-model/contracts/constraints deltas), `principal-architect` (author of the architecture store), `requirements-analyst` (likely spec author). DM's staffing call (`impl.staffing-latitude`): stage this to `validator` (generic independent checklist grader) — exempt from plan approval as a grading seat.
**Reads (by that seat):** `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/contracts/api.yaml`.
**Concrete findings this check would surface, given the fixture:**
- **Structural gap (blocking a store trip):** FR-103 requires "a background index worker" off the request path; the spine's Topology says *"Synchronous request/response only; no queues, no background workers."* Direct contradiction — a store delta is required, and this is a store trip reserved to the user at run-open (`impl.user-runopen-rulings`).
- **Contract gap:** `contracts/api.yaml` has no `GET /notes/search` operation — needed for W1.
- **Design gap:** no technical decision (`D-XXX`) exists for the indexing mechanism (in-process ticker vs. something else) against C-001's single-process constraint — commodity-category territory, so any adopt-first ruling here is later reserved to the user (`impl.adopt-first-user-call`), never builder-decided.
- **Data-model question:** whether a search-index needs its own modeled entity or is a query-time device over `Note` — left to the design phase to resolve.
- **Absent surfaces surfaced, not failing:** no governance region (missing `CLAUDE.md`/`.claude/rules/mochiko/`) → surfaced per `impl.absent-surfaces`. No `.mochiko/memory/codebase-analysis.md` → brownfield ambiguity, offer `/mochiko:setup` or proceed greenfield with a logged warning. No root `.mochiko/FEATURES.md` index → surfaced as a map-index gap that landing (Phase 8) will need to create/update.
**Writes:** `.mochiko/features/FEAT-002/sufficiency-report.md` — per-row verdict (both W1 and W2 come back **gap**, not sufficient), the gap list scoping Phase 3, the trips list for run-open, the `quickstart.md` null-path note (no real external-integration surface — API-only feature), any `[MODIFY]` amendment claims (none here — FEAT-001's delivered rows aren't being amended).
**Gate:** none blocking yet — the verdict feeds the run-open confirmation next. A disputed clause, if the grader can't clear it alone, defaults to gap and routes to the user (`impl.sufficiency-disputed-clause`) — none anticipated here since the spine/spec conflict is unambiguous.

## Phase 3 — Run-open confirmation (the entry gate)

**Does:** One confirmation, no negotiation, presented to the user before anything downstream fires.
**What would be confirmed:**
- Batch: FEAT-002 "Note search," scope type **selection**, rows W1+W2.
- Attempt bounds restated at their only redeclaration point: 3 verification attempts per cycle, 2 gap-rework rounds at final validation (both from `implement.yaml`'s `vars:` defaults — no override proposed).
- Sufficiency verdict: both rows gapped; gap list = {architecture-store delta for the background index worker/search element, `GET /notes/search` contract addition, indexing-mechanism technical decision}.
- Trips/conflicts for ruling: the FR-103-vs.-spine contradiction (store trip — rule here, or defer to the design phase's own presentation), the three absent surfaces (governance region, codebase-analysis.md, FEATURES.md index).
- Done condition stated: every cycle card checked, test-first, independently verified against real infra (per-cycle and whole), criteria traced, governance aligned (none present to conflict with here), acceptance landing executed whole, run closes on accept/amend/reject.
**Writes:** none — a confirmation record only (folded into the sufficiency report / run narrative, not a separate file).
**Gate — user's ruling here:**
- **If confirmed as scoped:** proceed to Phase 4 (design phase) exactly as presented.
- **If the user rules the store trip differently** (e.g., disputes that a background worker is needed, or wants the "no background workers" line held and the freshness requirement met synchronously instead): the gap list and design-phase brief narrow accordingly — Phase 4 designs to the ruled shape, not the as-spec'd one.
- **If the user defers a trip or absent-surface question** (e.g., "proceed greenfield, skip `/mochiko:setup` for now"): logged, run proceeds; that deferral does not fail the run (`impl.absent-surfaces`).
- **If the user wants to widen/narrow the attempt bounds:** the new values are recorded here and apply for the rest of the run — no further redeclaration point exists.

## Phase 4 — Design phase (fires: gaps were named)

**Does:** Fires because Phase 2 named gaps (`impl.design-phase-fires-on-gap`) — no code is written before this closes and the user signs it. Design seats author *exactly* the named gaps, each on a plan the DM approves first (`impl.plan-approval-producers`), rung-justified per `mochiko:patterns-plan-minimalism`.
**Seats:**
- `technical-analyst` (design_seat) — the contract delta (`GET /notes/search` operation + response schema) and the technical-decision delta (D-XXX: in-process background indexing mechanism, checked against C-001 single-process and D-002 stdlib-only), plus any NFR-XXX target for the 2-second freshness bound.
- `principal-architect` (architect_seat) — the architecture-store delta: new element(s) for the background index worker and (if warranted) a search-index concern row, a C4-container delta diagram and delta register per `mochiko:patterns-system-design`, reconciling against the currently-ruled "no background workers" line — this is the deviation the store delta exists to resolve, never designed around silently (`impl.deviation-gate` is the floor this anticipates once a delta is signed).
- Whether the background-worker mechanism is itself a commodity-category adopt-first question is flagged by the architect but *ruled by the user*, not designed around (`impl.adopt-first-user-call`).
**Reads:** `sufficiency-report.md`, `spec.md`, current `.mochiko/product/{data-model.md,contracts/api.yaml,constraints-and-decisions.md,architecture/spine.md}`.
**Writes (deltas beside baselines, never in place — `impl.baselines-never-in-place`):**
- `.mochiko/features/FEAT-002/contracts-delta.md` (or equivalent appliable before/after form for the `GET /notes/search` addition)
- `.mochiko/features/FEAT-002/data-model-delta.md` if a search-index entity is warranted
- `.mochiko/features/FEAT-002/baseline-delta.md` carrying the D-XXX indexing decision against `constraints-and-decisions.md`
- `.mochiko/features/FEAT-002/architecture-delta.md` (or the store's delta convention) — the signed-pending store delta with its diagram and AX-XXX row changes
- Feature-map assertion: dependencies/extent sharpened onto `FEAT-002/entry.md` with provenance, architecture link filled once the delta exists (`impl.design-map-assertion`)
**Review pair (non-author, before the checkpoint):** `mochiko:review-plan-artifacts` (conformance to the gap list, blocking) and `mochiko:review-feasibility` (buildability/contradiction, e.g. `tech-lead`) — both grade the design package, neither authored it.
**Gate — the design checkpoint (`impl.gate-design-checkpoint`, floor):** the user's. Presented: the rendered delta diagram plus the named AX-XXX row changes (source + changed-element table if no render surface), the contract/data-model/decision deltas.
- **If signed:** proceed to Phase 5. The user may also choose to stop here and resume the build later — the run holds at this checkpoint indefinitely without failing.
- **If amended:** design seats rework the flagged portion only, re-reviewed, re-presented — does not restart the whole phase.
- **If rejected outright** (e.g., user wants freshness met synchronously, no worker at all): Phase 4 re-fires scoped to the new shape; the FR-103 requirement itself may need to route back to spec (out of this run's authority) if the rejection contradicts spec text — flagged as a scope question, not silently resolved.

## Phase 5 — Cycle-card authoring and card confirm

**Does:** A design-class, non-builder seat authors cycle cards from the signed design, foundation cycles before feature cycles (walking skeleton first, per `mochiko:patterns-vertical-tdd`). DM's call: `technical-analyst` authors the cards (already holds the design context), `qa-engineer` authors the embedded `**TEST:**` real-infrastructure gates within that slicing (`impl.seat-card-author-independence`).
**Anticipated cycle shape:** Cycle 1 (foundation) — background index worker + index structure wired to `notes-db`, no user-facing behavior yet, TEST gate exercises the worker starting/rebuilding. Cycle 2 (feature) — `GET /notes/search` endpoint over the existing index (W1, SC-101/SC-102). Cycle 3 (feature) — create-to-searchable freshness bound (W2, SC-103).
**Reads:** the signed design deltas, `spec.md` for cited acceptance-criteria IDs.
**Writes:** `.mochiko/features/FEAT-002/tasks.md` — cycle cards from the tasks template (`mochiko-cli template tasks`, falling back to `plugins/mochiko/schemas/tasks.yaml` read raw if the binary is absent). Each card: stories/rationale, dependencies, acceptance criteria by ID, a `**TEST:**` gate, brownfield exposure (all three cycles touch/extend the existing `api-service` and `notes-db` — `[EXTEND]`) — no task lists, no file paths.
**Verification seat review before confirm:** `qa-engineer` (held as this run's consistent verification seat) reviews the cards for quality (`mochiko:review-plan-artifacts`) and buildability (own judgment) — independent of the card author.
**Gate — the card confirm (`impl.gate-card-confirm`, floor):** the user's, blocking, before any card is built.
- **If confirmed:** Phase 6 begins.
- **If the user re-slices** (e.g., merges Cycles 2/3, or wants the worker cycle split further): cards revised, re-reviewed, re-presented — no code has been written yet, so this is cheap.

## Phase 6 — Build (test-first, per cycle)

**Does:** `staff-engineer` (builder_seat) executes each confirmed card via `mochiko:executing-tdd-cycle`: decomposes into concrete tasks disclosed in the cycle report, runs the pre-code minimalism ladder at decomposition (`mochiko:patterns-code-minimalism`, rungs disclosed), follows `mochiko:brownfield-integration` for every `[EXTEND]` touch to the existing `api-service`/`notes-db` code, builds red→green→refactor on a DM-approved plan.
**Per-cycle verification:** `qa-engineer` (never the implementer) grades against real infrastructure via `mochiko:testing-end-user`, plus the advisory `mochiko:review-code-minimalism` lens (reads the diff, the cycle report, and the surrounding codebase — reuse claims never on trust). Attempt economy: 3 attempts per cycle (or the run-open-redeclared value); two consecutive unchanged-finding rounds is a no-progress stop, halting that cycle and presenting state to the user.
**Writes per cycle:** code changes to the (currently fixture-only) `api-service`; `.mochiko/features/FEAT-002/cycle-report-<n>.md`; `.mochiko/features/FEAT-002/verification-report-<n>.md`; `tasks.md` checkbox flips as each card completes.
**Escalation handling:** Minor findings default to a `BACKLOG.md` booking (never an in-cycle fix); Important+ findings block the cycle and batch into the cycle checkpoint; an infeasible card or a build-discovered undesigned-structure hit halts that cycle and either escalates to the user (business-scope call, `impl.infeasible-card-escalation`) or re-fires a scoped Phase 4 redo (`impl.midrun-refire`) — same grade, same checkpoint, anchored to the signed delta.
**Gate:** cycle-checkpoint batches (escalations, findings) land together for the user's ruling; only a build-blocking question interrupts mid-cycle rather than waiting for the batch.

## Phase 7 — Final validation (whole-build)

**Does:** After all cycles are checked, runs the whole-build verification pass.
- **Quality gates:** full repository suite (`impl.gates-full-suite`), never severity-triaged — any failure fails the run.
- **Regression sweep:** re-runs `.mochiko/features/FEAT-001/gates.md`'s accumulated `**TEST:**` cases (FEAT-002 owns the seam since it lands later into FEAT-001's `notes-db` territory) plus FEAT-002's own new gates.
- **Cold verification:** builds/tests from a dependency-cold snapshot of the uncommitted working tree (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-note-search/`), first confirming a `/.claude/worktrees` ignore entry exists.
- **Gap-finding pass:** REQUIRED — scope is `selection`, so `impl.gap-finding-scope` fires, not skipped (`impl.fail.gap-finding-missing` would trip if it were omitted). Fresh, blind `devils-advocate` dispatched two-message: first message carries only `spec.md`, `sufficiency-report.md`, the design deltas, the baselines (`data-model.md`, `contracts/`, the store's NFR-XXX rows) — no Screens & Flows (spec states API-only, none exist) — never the code, `tasks.md`, TEST cases, or any report. Seat states derived expectations before probing begins.
- **Mutation lens:** conditioned on governance `depth: high`; with no governance region present, depth cannot resolve from a governance surface — flagged to the user rather than silently assumed either way, and the run states whichever disposition the user rules (skip disclosed, or run it on the user's instruction).
**Writes:** `.mochiko/features/FEAT-002/final-validation-report.md` (with an explicit statement of the gap-finding pass having run, and the mutation-lens disposition), plus any gap findings.
**Finding routing:** spec-required-behavior breaks fail the final validation and must resolve before acceptance; beyond-spec findings are advisory, disposition reserved to the user (fix now / `BACKLOG.md` / accept as designed) at the checkpoint; a disputed finding kind defaults advisory and goes to the user.
**Gate:** gap-rework bound is 2 rounds (or run-open-redeclared) at run scope, or charges the localized cycle's remaining attempts if a finding pins to one cycle's territory; bound exhaustion or a no-progress round halts the run and presents state — disposition is the user's.

## Phase 8 — Landing (executed whole, at acceptance)

**Does:** Executed as one whole action, never partially, once the user accepts (`impl.dm-landing-whole`).
- **Store landing:** the signed architecture delta's elements flip to `built`, FEAT-002's key clears from them; the touched rows' `As-built:`/`Drift:` fields are written as judgment and independently graded (`tech-lead`, per its role grading the store's judgment writes); orphan check runs; the store skill regenerates the derived `ARCHITECTURE.md` (never hand-edited).
- **Graded folds:** each touched baseline (`contracts/api.yaml`, `data-model.md` if changed, `constraints-and-decisions.md`) folds exactly once via a three-way diff checked by the landing verification seat (`qa-engineer`).
- **Selection-scope map landing:** W1/W2 fold into FEAT-002's extent lines and vanish from the work-row list; status flips to `delivered`, dated; the root `.mochiko/FEATURES.md` index line updates — and since that index is currently absent from this repo, its first-write/creation is itself surfaced as part of this landing rather than assumed pre-existing; the specs-index row for `note-search` closes since both selected rows fold.
- **Gates fold:** any gap findings ruled fix-now or backlog fold into a newly minted `.mochiko/features/FEAT-002/gates.md`, authored by `qa-engineer` in the `**TEST:**` grammar; accepted-as-designed findings do not fold.
- **Knowledge-management landing:** skipped — no `.mochiko/memory/knowledge-management.md` exists (`impl.km-landing` is conditioned on its presence).
**Writes:** the above paths, plus `.mochiko/features/FEAT-002/built-vs-signed-diff.md`.

## Phase 9 — Final acceptance and close

**Gate — final acceptance (`impl.gate-final-acceptance`, floor):** the user's, plain blocking text, never timed. Presented: the landing summary, final-validation evidence, any open beyond-spec dispositions.
- **Accept:** DM closes the run with a verdict checked against the done condition and the full 15-item Not-done set (`impl.sec.fail-conditions`) — confirming none stand: sufficiency recorded, design signed (not skipped), no card built pre-confirm or by its own author, no card left unchecked, no failing quality gate, no unevidenced verification claim, no regression, no in-place baseline edit, no unresolved deviation, store landing complete with its diff, every touched baseline graded-folded, gap-finding pass present and its report states so, no unresolved spec-required gap finding, and acceptance itself given. Run reports closed.
- **Amend:** the specific amendment routes back to the narrowest phase that owns it (a card slicing amendment → Phase 5; a design amendment → Phase 4; a finding disposition change → Phase 7/8) rather than restarting the whole run.
- **Reject:** the run halts without landing; nothing folds; state is presented for the user's next instruction (e.g., re-scope, abandon, or redirect to a delta card for partial rework).