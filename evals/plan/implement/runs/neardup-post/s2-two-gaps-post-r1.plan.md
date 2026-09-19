# Action Plan — `/mochiko:implement FEAT-002` (plan-only, not executed)

## Governing facts established before planning (reads performed)

- Schema reads complete: `plugins/mochiko/schemas/implement.yaml` (full), `plugins/mochiko/schemas/common.yaml` (full, for every `extends:` stub), `plugins/mochiko/schemas/command-labels.yaml`. `kind: fail` count = 15, matching the Not-done line — no halt needed.
- Entry target: `FEAT-002` resolves directly (not an `EPIC-XXX`) → **scope: selection**, source: `.mochiko/features/FEAT-002/entry.md`, work rows W1/W2, selection ratified 2026-08-26 against `.mochiko/specs/note-search/spec.md` (status: accepted).
- Dependency check: entry.md lists `FEAT-001 (delivered)`. FEAT-001's own entry.md confirms `delivered (2026-08-22)`. No blocking dependency.
- Condition resolution so far: `scope=selection`; `governance_region` = **absent** (no `CLAUDE.md` found anywhere in the tree); `km_file` = **absent** (no `.mochiko/memory/knowledge-management.md`); `baseline` = **present** for data-model/contracts/constraints/architecture spine, but each is missing the specific content this batch needs (see Phase 1); `seats` will resolve to **multi** the moment a second seat is composed (fires the transport floor); `depth` cannot be read (no governance region to read it from) — flagged as an absent surface, not a blocker.
- Environment-integrity observation (not a rule clause, but material to sequencing): this workspace has no `.git` repository and no application source tree at all, despite FEAT-001's entry.md and `gates.md` asserting delivered, verified code. This directly affects the cold-verification step (`git ls-files`) and brownfield reads later in the plan — flagged for the user at entry rather than silently assumed away.
- Read the batch's own targets: `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/features/FEAT-001/gates.md`, `FEATURES.md`. Two load-bearing findings surface immediately, ahead of the formal sufficiency check:
  1. `contracts/api.yaml` defines only `POST /notes` and `GET /notes/{id}` — no `GET /notes/search` endpoint exists anywhere in the API contract that W1 needs.
  2. FR-103 (W2, index freshness) explicitly requires "a background index worker" off the request path, but the ruled architecture spine states in `## Topology`: *"Synchronous request/response only; no queues, no background workers."* — a direct spec-vs-ruled-architecture contradiction with no AX row, no NFR row, and `entry.md`'s own Architecture link says "not yet filled; no store delta exists."

---

## Phase 1 — Entry & sufficiency check

**Does:** Grades W1 and W2 against the pointer skill's clause set (`mochiko:review-sufficiency`), per row, over spec + store + baselines. The grading seat authored none of the graded sources — `technical-analyst` authored data-model/contracts/constraints, `principal-architect` authored the spine, so neither can grade — this run assigns the independent **validator** seat (`mochiko:validator`), exempt from plan approval like any grading seat.

**Reads:** `spec.md`, `.mochiko/product/architecture/spine.md`, `constraints-and-decisions.md`, `data-model.md`, `contracts/api.yaml`, `.mochiko/features/FEAT-002/entry.md`.

**Writes:** `.mochiko/features/FEAT-002/sufficiency-report.md` — per-row verdict, store-consult result, any no-delta claim, trips for run-open, `quickstart.md` null-path note (no external-integration surface here), any `[MODIFY]` amendment note.

**Expected verdict (from the reads already done):**
- **W1 — gap.** No API contract for `GET /notes/search` (query param, 400 case, response shape); no architecture row covers "search" at all.
- **W2 — gap, and a store-level conflict, not just an absence.** FR-103 requires a background worker; the ruled spine forbids one. This is a disputed-clause case — per `impl.sufficiency-disputed-clause`, a disputed clause defaults to **gap** and routes to the user; the grading seat never clears it alone.
- Both gaps fire the design phase (`impl.design-phase-fires-on-gap`) before any code is written.

**Seats/skills:** validator seat; `mochiko:review-sufficiency` (pointer, procedure not restated here).

**Gate:** none yet — sufficiency is a binding verdict, not a user checkpoint by itself; its findings feed the run-open confirmation next.

---

## Phase 2 — Run-open confirmation (the entry gate)

**Does:** One blocking confirmation, plain text, no negotiation per `impl.acceptance-plain-text`. States: the batch is FEAT-002 / Note search, scope type selection; both attempt bounds at their only redeclaration point — `attempt_bound_cycle = 3` per cycle, `gap_rework_bound = 2` rework rounds — carried as defaults unless the user redeclares them here; the sufficiency verdict from Phase 1 (two gaps, one of them a spec-vs-architecture conflict); the done condition (every cycle card checked, built test-first, independently verified against real infrastructure per-cycle and whole, code traces to requirements and governance, acceptance landing executed whole, run closes on accept/amend/reject).

**Reads:** `sufficiency-report.md` (just written).

**Writes:** nothing new — this is a confirmation, not an artifact.

**Gate — reserved to the user (`impl.user-runopen-rulings`):**
- **What's confirmed:** the store trip (FR-103's background-worker requirement vs. the spine's "no background workers, sync-only" rule) — ruled here, or explicitly deferred to the design checkpoint on the record; the API-contract gap for W1; the two attempt-bound defaults; and the absent-governance-region / absent-git-repo observations from the pre-read, surfaced (never auto-resolved, never run-failing per `impl.absent-surfaces`).
- **Branch — user rules "amend the architecture, proceed":** design phase is scoped to author a store delta introducing an in-process background indexing component plus the missing API contract and NFR row; run continues to Phase 3.
- **Branch — user rules "keep sync-only, change the requirement instead":** this becomes a scope/requirement change outside what this run was framed to build (`impl.scope-escalation-fail` territory) — the run would route this back rather than silently reinterpreting FR-103; practically this means pausing implement and returning to `/mochiko:feature` or the spec to amend FR-103/SC-103 before re-entering. Plan continues under the assumption the user re-confirms scope once that's settled.
- **Branch — user declines to rule now:** the trip stays open and un-cleared; per `impl.sufficiency-disputed-clause` it stays a gap, and the run cannot open past this point until it's ruled — a hard stop, not a silent default.
- **For the remainder of this plan**, the "amend the architecture, proceed" branch is carried forward, since it is the only branch under which an implement run continues in this session.

---

## Phase 3 — Design phase (fires: both W1 and W2 named gaps)

**Does:** Authors exactly the named gaps, nothing more, each on a plan the lead approves first (`impl.design-gaps-only`, rung-justified per `mochiko:patterns-plan-minimalism`). `staff-engineer` (builder) never touches this phase (`impl.builder-never-designs`).

**Seats:**
- `technical-analyst` (design_seat) — authors the `contracts/api.yaml` delta for `GET /notes/search` (query param, 400/200 shapes) and any `data-model.md` delta needed for the index representation; authors the NFR-XXX content (create-to-searchable ≤2s at ≤10k notes) as concern-row text for the architect to land.
- `principal-architect` (architect_seat) — authors the architecture store delta: a new AX row for the search/index component, the topology change (background worker as an in-process component, not a new external service — checked against C-001's single-process constraint, which it does not violate), a C4-container delta diagram and sequence diagram for the index-refresh flow, per `mochiko:patterns-system-design`; grammar and lifecycle per `mochiko:authoring-architecture-store`.

**Reads:** `sufficiency-report.md`, current `spec.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `architecture/spine.md`.

**Writes (deltas beside baselines, never in place — `impl.baselines-never-in-place`):**
- `.mochiko/features/FEAT-002/contracts-delta.md` (or equivalent appliable before/after form for `contracts/api.yaml`)
- `.mochiko/features/FEAT-002/data-model-delta.md` (if the index needs modeling)
- `.mochiko/features/FEAT-002/architecture-delta.md` + rendered diagrams (new AX row, delta register, sequence diagram) — the one legal in-place carve is the store's own in-flight-class delta elements written under the design checkpoint's sign-off
- `.mochiko/features/FEAT-002/entry.md` — design-implied dependencies/extent asserted with provenance, architecture link filled once the delta exists

**Review pair (non-author, before checkpoint):** `mochiko:review-plan-artifacts` (conformance to the two-item gap list, blocking) and `mochiko:review-feasibility` (buildability/contradiction — this is exactly where the background-worker-vs-C-001/D-002 tension gets adversarially checked). Natural seat fit: **tech-lead** for feasibility (its stated role is hunting cross-artifact contradictions and buildability conflicts), **validator** for plan-artifact conformance (already independent from Phase 1, still non-author of this phase's output).

**Gate — design checkpoint (`impl.gate-design-checkpoint`, floor):**
- **What's confirmed:** the rendered architecture delta diagram plus its named AX-row changes (or, absent a render surface, the source plus a changed-element table) and the API-contract/data-model deltas, together as one signed package.
- **Branch — user signs:** design outputs and the store delta are locked as the anchor for the deviation gate later; run proceeds to Phase 4.
- **Branch — user asks for rework:** design seats revise within the same named-gap scope (no scope creep); re-review, re-present — this does not consume a build-cycle attempt, since it's pre-card.
- **Branch — user stops here:** run pauses; resumable later at the build stage, design already signed and durable.

---

## Phase 4 — Cycle-card authoring & card confirm

**Does:** Cards authored from the signed design (or zero-gap map assertion, not applicable here since gaps existed) by a design-class seat that is never the builder. `technical-analyst` authors the cards; `qa-engineer` authors the **TEST:** real-infrastructure gate within that slicing (`impl.seat-card-author-independence`). Cycle mapping follows `mochiko:patterns-vertical-tdd` — walking skeleton first, foundation cycles before feature cycles. Given W1 (search) depends conceptually on W2's indexing existing, and both need the background worker component, a plausible slicing (not prescriptive — staffing/slicing judgment sits with the design seat) is: Cycle 1 — index worker foundation (background rebuild loop, no HTTP surface yet); Cycle 2 — `GET /notes/search` happy path + 400 (W1, SC-101/SC-102); Cycle 3 — create-to-searchable freshness bound (W2, SC-103).

**Reads:** signed design deltas, `spec.md` (for cited acceptance-criteria IDs), `constraints-and-decisions.md`, architecture delta.

**Writes:** `.mochiko/features/FEAT-002/tasks.md` — cycle cards from the tasks template (`mochiko-cli template tasks`, falling back to `plugins/mochiko/schemas/tasks.yaml` read raw if the binary is absent). Each card: stories/rationale, dependencies, acceptance-criteria IDs, a `**TEST:**` gate, brownfield exposure — no task lists, no file paths.

**Card review before confirm:** verification seat (`qa-engineer`, never the implementer) grades quality via `mochiko:review-plan-artifacts` and buildability by its own judgment.

**Gate — card confirm (`impl.gate-card-confirm`, floor):**
- **What's confirmed:** the cycle slicing itself — three cards, their order, their **TEST:** gates — before any build.
- **Branch — user approves as-is:** proceeds to Phase 5 with this slicing.
- **Branch — user re-slices (merge/split cycles):** cards are re-authored by the same design-class seat, re-reviewed, re-presented; still pre-build, no attempt consumed.
- **Branch — user rejects the whole slicing:** returns to Phase 4 authoring with different guidance; design signed artifacts remain the anchor.

---

## Phase 5 — Build: TDD cycles, test-first, per-cycle verification

**Does, per card, in order:** `staff-engineer` (builder) decomposes the card into concrete build-time tasks (disclosed in the cycle report), runs the pre-code minimalism ladder at decomposition (`mochiko:patterns-code-minimalism`), follows `mochiko:brownfield-integration` for any touch to existing code (Cycle 1's worker and Cycle 2/3's HTTP handlers likely extend the existing single-process service — full-file read first, given the note above about the FEAT-001 code's actual presence needing to be re-confirmed at this point), and drives red→green→refactor per `mochiko:executing-tdd-cycle`. Builder works only on a plan the lead approved.

Verification: `qa-engineer`, never the implementer, executes the card's `**TEST:**` gate against real infrastructure (`mochiko:testing-end-user`) and applies the `mochiko:review-code-minimalism` lens (advisory only, never gates the cycle) by reading the diff, the cycle report, and the surrounding code.

**Reads per cycle:** the card, signed design deltas, existing code touched.

**Writes per cycle:** `.mochiko/features/FEAT-002/cycle-report-<n>.md` (decomposition, honest difficulties, deviations, `domain_deps_added`); `.mochiko/features/FEAT-002/verification-report-<n>.md`; `tasks.md` checkbox flips as cycles complete (the progress surface).

**Attempt economy (floor, from Phase 2's bound):** each verification grading consumes one of the 3 per-cycle attempts, whatever the round is called. Two consecutive rounds with unchanged findings is a no-progress stop — halts that cycle, presents state to the user rather than continuing to spend attempts.

**Escalation batching:** reserved-to-user questions (an infeasible-card ruling, an adopt-first/IP-XXX call, ambiguity a producer flags) accumulate and land as one batch at each cycle checkpoint — only a build-blocking question interrupts mid-cycle. Given the store delta is already signed going into this phase, a deviation-gate trip here (adding/removing a box beyond the signed delta) is a live possibility if the worker's real shape needs to change once code is underway — surfaced as its own gate if it happens (see below), not silently designed around.

**Gate — mid-cycle escalation (conditional, only if triggered):**
- **What's confirmed:** whichever reserved item fired — e.g., a deviation from the signed architecture delta, or an infeasible-card business call.
- **Branch — user rules to proceed as approved:** cycle resumes on the existing plan/delta.
- **Branch — user amends the delta/scope:** the anchor moves; design phase re-fires scoped to just that discovery (`impl.midrun-refire`), then resumes the cycle under the amended anchor.
- **Branch — exhaustion/no-progress without a ruling:** cycle halts, state presented, run does not proceed past it silently.

---

## Phase 6 — Final validation (selection scope: full suite, including gap-finding)

**Does:**
- Full quality-gate suite run, never severity-triaged (`impl.gates-full-suite`, `impl.gates-never-triaged`).
- Regression sweep: re-runs FEAT-001's durable gate set at `.mochiko/features/FEAT-001/gates.md` (the three existing `**TEST:**` cases) plus any seam this feature's territory shares with it.
- Cold verification: build/test from a dependency-cold snapshot of the uncommitted working tree via `git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-note-search/` — **this step is currently blocked by the environment-integrity finding from the pre-read: no `.git` repository exists in this workspace.** This would need to be surfaced and resolved (e.g., `git init` offered, per the general "investigate before working around absent infra" posture) before this step can run; it is not something to route around silently.
- Gap-finding pass (fires — scope is selection): a **fresh `devils-advocate`**, never having built these cycles or seen the design-time **TEST:** cases, dispatched blind in two messages — first carrying only `spec.md`, `sufficiency-report.md`, design deltas, and the baselines (data-model, contracts, the store's NFR rows) — never code, `tasks.md`, **TEST:** cases, or any report. It states derived expectations before probing begins.

**Reads:** the above baselines/deltas (for gap-finding's first message); the built code, evidence, and reports (for the qa seat's regression/cold-verification work, kept separate from the gap-finder's blind fence).

**Writes:** `.mochiko/features/FEAT-002/final-validation-report.md` (regression results, cold-verification evidence, gap-finding findings split spec-required vs. beyond-spec).

**Gate — finding-kind disputes / beyond-spec disposition (conditional):**
- **What's confirmed:** for any disputed finding kind, the cited clause vs. the finder's classification; for any beyond-spec finding, its disposition.
- **Branch — user confirms spec-required:** finding fails final validation, routes to gap-rework (2-round bound, or charges the localized cycle's remaining attempts if it's cycle-scoped).
- **Branch — user rules beyond-spec, "fix now":** folds into this run's remaining work before landing.
- **Branch — "book to BACKLOG.md":** written to `BACKLOG.md`, does not block this run's landing.
- **Branch — "accept as designed":** no fold, no backlog entry, closed as-is.

---

## Phase 7 — Landing (selection-scope landing)

**Does:** Executes the store landing (delta elements flip built, FEAT-XXX keys clear, As-built/Drift fields written as graded judgment, orphan check run, derived `ARCHITECTURE.md` regenerated — never hand-edited) plus the map's graduation batch for selection scope: W1/W2 fold into FEAT-002's extent lines, status flips to `delivered` (dated), `FEATURES.md` index line updates, the note-search spec's row is touched (closes once all its selected rows have folded). Every other touched baseline (contracts/api.yaml, data-model.md, constraints-and-decisions.md if an NFR/D-XXX row landed) folds exactly once via a graded three-way diff, checked by the landing verification seat (`qa-engineer`), never self-graded by the authoring seat. Gap findings ruled fix-now or backlog fold into `.mochiko/features/FEAT-002/gates.md` (minted fresh — none exists yet for this feature), authored by `qa-engineer` in the **TEST:** grammar.

**Reads:** all signed deltas, `final-validation-report.md`, current baselines.

**Writes:** `.mochiko/product/architecture/spine.md` (folded), `ARCHITECTURE.md` (regenerated), `.mochiko/product/contracts/api.yaml`, `.mochiko/product/data-model.md`, `.mochiko/product/constraints-and-decisions.md` (as applicable), `.mochiko/features/FEAT-002/entry.md` (extent folded, status delivered), `FEATURES.md`, `.mochiko/features/FEAT-002/gates.md`, `.mochiko/features/FEAT-002/built-vs-signed-diff.md`.

**Note:** this phase executes *at* user acceptance, whole — it is staged here for planning clarity but its actual writes happen only after Phase 8's ruling, per `impl.dm-landing-whole`.

---

## Phase 8 — Final acceptance (the closing gate)

**Gate — final acceptance (`impl.gate-final-acceptance`, floor):**
- **What's confirmed:** the whole landing package — every card checked, per-cycle and whole-build real-infrastructure evidence, the regression sweep and gap-finding results, the store delta's built-vs-signed diff, the graded folds — against the done condition and the 15-item Not-done set.
- **Branch — accept:** Phase 7's landing writes execute; run closes with a verdict against the done condition (`impl.dm-close-verdict`).
- **Branch — amend:** user specifies the change; routes back to the relevant phase (a cycle rework, a design amendment, or a finding-disposition change) without re-litigating already-settled ground; re-presents for acceptance.
- **Branch — reject:** run closes without landing; `impl.fail.no-acceptance` stands, and the run is recorded Not-done.

---

## Summary of what would actually get built, and the one thing that most needs a ruling

The concrete work is a `GET /notes/search` endpoint (case-insensitive substring match, newest-first, 400 on missing/short query) backed by an index kept fresh by a background worker — and that worker requires amending the currently-ruled architecture spine, which today explicitly forbids background workers. That amendment is the one decision this plan cannot make on the run's behalf: it surfaces at run-open (Phase 2) and is locked at the design checkpoint (Phase 3), and every later phase treats that signed delta as the anchor no cycle may silently drift from.