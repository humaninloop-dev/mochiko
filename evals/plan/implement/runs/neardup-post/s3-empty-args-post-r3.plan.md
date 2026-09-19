# Action Plan — `/mochiko:implement` (no `` given), notely repo

**Plan-only. No agents spawned, no files written, no gates awaited — this is the plan I would execute.**

## Phase 0 — Load the governing schema (done as part of planning)
- **Read** `plugins/mochiko/schemas/implement.yaml` (raw, full) — confirmed 15 `kind: fail` rules in `impl.sec.fail-conditions` (count matches the `.md`'s hard-coded Not-done count, so no integrity halt is triggered).
- **Read** `plugins/mochiko/schemas/common.yaml` (raw, full) — resolves the four `extends:` stubs used later: `impl.tools-referenced-never-restated`, `impl.register`, `impl.author-grader-default-fail`, `impl.model-tiering`, `impl.transport-floor`.
- No writes. This is the mandatory first action before any gating.

## Phase 1 — Resolve the target (`` was empty)
- **Read**: `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/features/FEAT-001/gates.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/specs/note-search/spec.md`.
- Findings: FEAT-001 "Note capture" is `delivered`; FEAT-002 "Note search" is `selected`, carrying two ratified work rows (W1 — search by query, W2 — index freshness), selection source: the spec's accepted selection (2026-08-26). Its dependency, FEAT-001, is delivered — not blocking. FEAT-002 is the only ready capability, so it is the proposal.
- Two anomalies to surface now rather than mid-build (`impl.absent-surfaces`): (a) no `.mochiko/memory/codebase-analysis.md` and no governance region (`CLAUDE.md`, `.claude/rules/mochiko/`) exist anywhere in the repo; (b) no application source tree exists at all despite FEAT-001 being marked delivered — the "brownfield `[EXTEND]`" assumption for this batch has no code to extend against. Both get surfaced to the user, neither auto-resolved, neither fails the run.
- No writes yet.

## Phase 2 — Sufficiency check (still pre-gate)
- Seat: an independent grader that authored none of `spec.md`, `spine.md`, or the product baselines and won't design/build this batch — **mochiko:validator**, running the procedure owned by `mochiko:review-sufficiency`.
- **Read**: `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`.
- Concrete gaps this grading pass would surface:
  - No `/notes/search` endpoint in `contracts/api.yaml` (W1 — API contract gap).
  - No data-model coverage of a search index (W1/W2 — data-model gap).
  - No `D-XXX` decision record evaluating a search/indexing approach — a commodity category (`mochiko:patterns-adopt-first` applies) — missing from `constraints-and-decisions.md`.
  - A direct conflict: spec FR-103 requires "a background index worker," but `spine.md` states the topology is "Synchronous request/response only; no queues, no background workers," and carries no AX row for search/indexing. This is a **store trip**, not an ordinary gap — reserved to the user, never auto-resolved by the grader.
- **Write**: `.mochiko/features/FEAT-002/sufficiency-report.md` (per `templates/report-format.md`) — per-row verdict (W1: sufficient-with-gaps; W2: insufficient), the gap list, and the trip flagged for run-open.

## Phase 3 — Gate: run-open confirmation (`impl.gate-design-checkpoint`'s upstream gate, `impl.acceptance-plain-text`)
**What would be confirmed**, as one plain blocking-text bundle, no negotiation:
- Batch: FEAT-002 "Note search," scope type **selection**, work rows W1 + W2.
- Attempt bounds, their only redeclaration point: per-cycle verification attempts (default 3), gap-rework rounds (default 2) — accept or override now.
- Sufficiency verdict and gap routing: API-contract gap, data-model gap, missing indexing decision → design phase scoped to exactly these.
- The store trip: FR-103's "background index worker" vs. the ruled spine's "no background workers" — presented for the user's ruling.
- Done condition stated: both cards `[x]`, test-first, verified per-cycle and whole against real infrastructure; code meets SC-101/102/103 and traces to FR-101–103; landing executed whole; run closes at accept/amend/reject.

**Branches on the user's ruling:**
- *Amend the architecture* (add a background-index element via a signed store delta) → design phase proceeds with that scope; continue at Phase 4.
- *Keep the architecture, redesign FR-103 as synchronous* (e.g., SQLite FTS5 triggers, no literal worker thread) → same design phase, but the architect's brief targets a sync-compatible mechanism satisfying FR-103's intent (≤2s freshness) without a new box; continue at Phase 4 with that framing.
- *Defer the trip on the record* → run opens, but W2's design gap cannot close at the design checkpoint until the trip resolves; the run would stall there pending the user.
- *Reject FEAT-002 entirely* → no other ready capability exists in the map; the run ends here, routing the user toward `/mochiko:specify` (new capability) or `/mochiko:feature` (delta) instead.
- Redeclared attempt bounds (whatever the user sets, or the defaults) lock in for the rest of the run.

*(Continuing the plan along the first branch — architecture amended via signed delta — as the representative path; the second branch only changes the architect's brief content, not the run's shape.)*

## Phase 4 — Design phase (fires: gaps were named)
Seats, each working only on a plan I approved (`impl.plan-approval-producers`), each authoring exactly its named gap (`impl.design-gaps-only`, rung-justified per `mochiko:patterns-plan-minimalism`):
- **principal-architect**: proposes the store delta resolving the sync/background-worker conflict (`mochiko:patterns-system-design` for the C4-container delta + register; `mochiko:authoring-architecture-store` for grammar) — in-flight-class elements only, never editing `spine.md` in place.
- **technical-analyst**: `contracts/api.yaml` delta for `/notes/search` (`mochiko:patterns-api-contracts`); `data-model.md` delta if an index-related entity is needed (`mochiko:patterns-entity-modeling`); `constraints-and-decisions.md` delta recording the adopt-first evaluation (SQLite FTS5 vs. hand-rolled) — the adopt-first ruling itself is reserved to the user (`impl.adopt-first-user-call`), so it surfaces at the checkpoint rather than being decided by the seat.
- **qa-engineer**: design-time **TEST:** acceptance cases for the eventual cycle cards, tied to SC-101–103.
- **Writes** (all deltas beside baselines, never in place — `impl.baselines-never-in-place`): API-contract delta, data-model delta (if needed), constraints-and-decisions delta, and store-delta artifacts, all under `.mochiko/features/FEAT-002/`; a map assertion of design-implied dependencies/extent with provenance onto `FEAT-002/entry.md`, filling the architecture link.
- **Review pair** (non-authors): `mochiko:review-plan-artifacts` (conformance to the gap list + card quality — blocking) and `mochiko:review-feasibility` (buildability/contradiction, including the architecture pass, since a store delta exists) — e.g. **validator** and **tech-lead** respectively, neither having authored this delta.

## Phase 5 — Gate: design checkpoint (`impl.gate-design-checkpoint`, floor)
**What would be confirmed**: the whole design package — API-contract delta, data-model delta, the constraints delta (including the adopt-first ruling), and the store delta rendered as a diagram plus its named AX-row changes (or source + changed-element table if no render surface).
**Branches**: sign as proposed → Phase 6; request amendment → the relevant design seat reworks on a revised approved plan, re-review, re-checkpoint; stop here → run pauses, resumable later at this exact point.

## Phase 6 — Cycle-card authoring
- Card-authoring seat: a design-class seat other than the builder (e.g. **technical-analyst**), slicing per `mochiko:patterns-vertical-tdd` (walking skeleton first). Likely slice: a foundation cycle (index storage/backfill), Cycle A (W1 — search happy path + 400s, FR-101/102), Cycle B (W2 — freshness on create, FR-103). **qa-engineer** authors the **TEST:** cases within that slicing.
- **Write**: `.mochiko/features/FEAT-002/tasks.md` from the cards template (or `tasks.yaml` schema directly if the binary is absent) — cards carry stories, dependencies, acceptance-criteria IDs, a **TEST:** real-infra gate, and brownfield exposure; no task lists or file paths in the cards.
- **Review before confirm**: an independent verification seat (e.g. **qa-engineer** if not the author, else **validator**) grades quality (`mochiko:review-plan-artifacts`) and buildability.

## Phase 7 — Gate: card confirm (`impl.gate-card-confirm`, floor)
**What would be confirmed**: the cycle slicing itself, before any build starts.
**Branches**: approve → Phase 8; request re-slicing → card author revises on an approved plan, re-review, re-confirm; stop/defer → pauses, resumable.

## Phase 8 — Build (foundation cycle, then Cycle A, then Cycle B; test-first)
- Builder: **staff-engineer** (never designs its own gaps). Per card: decompose into concrete tasks at build time (disclosed in the cycle report), run the pre-code ladder (`mochiko:patterns-code-minimalism`), follow `mochiko:brownfield-integration` on any `[EXTEND]` touch, drive red→green→refactor (`mochiko:executing-tdd-cycle`).
- Note: since no governance region exists, `impl.briefs-name-rules-files` does not fire — already flagged once at entry rather than re-raised per cycle. Note also: with no source tree currently on disk, the concrete file paths the builder introduces are only knowable at decomposition time.
- **Write**: application source (paths TBD at decomposition) plus `cycle-report.md` per cycle under `.mochiko/features/FEAT-002/`.
- Verification per cycle: an independent seat (e.g. **qa-engineer**) runs `mochiko:testing-end-user` against real infrastructure for the **TEST:** gate, plus the advisory `mochiko:review-code-minimalism` lens on the diff/report/surrounding code. Full quality-gate suite runs (`impl.gates-full-suite`); any failure fails the cycle, untriaged (`impl.gates-never-triaged`).
- Attempt economy (floor): 3 verification attempts per cycle (or the run-open-redeclared number); two consecutive unchanged-finding rounds triggers a no-progress halt, presented to the user rather than continuing to spend attempts.
- Cycle checkpoint: escalations/findings batch per cycle (`impl.escalation-batching`); Minor findings default to a `BACKLOG.md` booking; Important-or-above findings block the cycle and join the batch presented to the user.

## Phase 9 — Final validation (selection scope → gap-finding is in scope)
- **Regression sweep**: re-run FEAT-001's durable gate set (`.mochiko/features/FEAT-001/gates.md`, three **TEST:** cases) plus any seam FEAT-002 exercises on FEAT-001's already-delivered side; a failure here fails the run like any other regression.
- **Cold verification**: snapshot the uncommitted working tree into `.claude/worktrees/mochiko-<purpose>/` (ensuring the `.claude/worktrees` ignore entry exists first — the one small `.gitignore`-style write in this run outside report/artifact surfaces, never a git mutation), build and gate from that cold copy.
- **Gap-finding pass**: a fresh **devils-advocate**, never a seat that built these cycles or saw the design-time TEST cases, dispatched two-message and blind — first message carries only `spec.md`, `sufficiency-report.md`, the design deltas, and the baselines (never code, `tasks.md`, TEST cases, or reports); states derived expectations before probing.
- **Mutation lens**: gated on governance depth = high, which is unresolved (no governance region) — surfaced to the user as an absent surface rather than assumed skipped or run.
- Findings split by kind: spec-required-and-broken → evidence + clause cited, fails final validation; beyond-spec → advisory, disposition reserved to the user (fix now / `BACKLOG.md` / accept as designed). A finding inside FEAT-001's delivered territory routes to a `/mochiko:feature` delta card instead of being reworked here.
- Gap-rework bound (floor): default 2 rounds at run scope (or the redeclared number); a finding localized to one cycle's territory instead charges that cycle's remaining attempts; exhaustion/no-progress halts the run, disposition reserved to the user.
- **Write**: `.mochiko/features/FEAT-002/final-validation-report.md`.

## Phase 10 — Landing (executed whole, at acceptance)
- **Store landing**: the signed delta's elements flip built, FEAT-002's key clears; touched rows' As-built:/Drift: written as judgment and independently graded; orphan check runs; `ARCHITECTURE.md` regenerated by the store skill (never hand-edited).
- **Graded fold** (floor): each touched baseline (`contracts/api.yaml`, `data-model.md`, `constraints-and-decisions.md`) folds exactly once via a checked three-way diff — **writes** to those files under `.mochiko/product/`.
- **Map graduation** (selection scope): W1/W2 fold into FEAT-002's extent and vanish as pending rows; `entry.md` status → delivered (dated); `FEATURES.md` index line updates; the note-search spec reads closed once all its rows have folded.
- **Gates fold**: fix-now/backlog gap findings fold into a newly minted `.mochiko/features/FEAT-002/gates.md`, authored in **TEST:** grammar by qa-engineer; accepted-as-designed findings do not fold.
- Knowledge-management landing: skipped — `.mochiko/memory/knowledge-management.md` is absent.

## Phase 11 — Gate: final acceptance (`impl.gate-final-acceptance`, floor)
**What would be confirmed**: accept / amend / reject, closing the run.
**Branches**:
- *Accept* → run closes; I report the verdict against the done condition, rounds consumed, and seats spawned.
- *Amend* → the user names the specific change; addressed as bounded rework (charged against remaining gap-rework attempts if gap-shaped, else new scope requiring explicit acceptance per `impl.scope-escalation-fail`); returns to this same gate.
- *Reject* → run closes Not Done (`impl.fail.no-acceptance`); since landing only executes "whole" at acceptance, nothing from Phase 10 has been committed yet to unwind.

## Phase 12 — Close
- Report the verdict against the fixed done condition and the 15-item Not-done set; confirm none of the 15 `kind: fail` conditions stand (sufficiency recorded, design signed, card independence held, all cards checked, gates clean, evidence real, no regression, no in-place baseline edit, no unresolved deviation, store landing complete, every fold graded, gap-finding run and not skipped-unstated, no unresolved spec-required gap, acceptance given).
- No further writes.