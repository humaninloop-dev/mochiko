# Action Plan — `/mochiko:implement` (no argument given)

Grounding performed for this plan (reads only, no writes, no agents spawned): read `plugins/mochiko/schemas/implement.yaml`, `common.yaml`, and `command-labels.yaml` raw and whole (15 fail-conditions confirmed against the schema, matching the `.md`'s hard-coded count); read `FEATURES.md`, `ARCHITECTURE.md`, `.mochiko/features/FEAT-001/{entry.md,gates.md}`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/specs/{note-capture,note-search}/spec.md`, `.mochiko/product/{architecture/spine.md,constraints-and-decisions.md,data-model.md,contracts/api.yaml}`. Notable finding baked into the plan below: the working tree has **no source code and is not a git repository**, despite FEAT-001 reading "delivered" on the map — this is carried through as a run-open trip.

## Phase 0 — Schema load (done, no gate)
Read whole and live-interpreted: `implement.yaml` (vars substituted: `attempt_bound_cycle=3`, `gap_rework_bound=2`, `builder_seat=staff-engineer`, `design_seat=technical-analyst`, `architect_seat=principal-architect`, `qa_seat=qa-engineer`, `gap_finder_seat=devils-advocate`, `explore_model=haiku`), `common.yaml` (for `extends:` blocks — `impl.register`, `impl.author-grader-default-fail`, `impl.model-tiering`, `impl.no-git-mutations`, `impl.acceptance-plain-text`, `impl.transport-floor`), `command-labels.yaml`. Nothing written.

## Phase 1 — Capability resolution (light gate)
No `$ARGUMENTS` given → propose the next ready capability from `FEATURES.md`. Only candidate: **FEAT-002 (Note search)**, status `selected`; FEAT-001 is `delivered` so FEAT-002's dependency on it is not blocking. FEAT-002's entry already carries ratified scope (selection scope, spec accepted 2026-08-26): work rows **W1** (US-101, SC-101/102) and **W2** (US-102, SC-103).

**Gate:** confirm with the user that FEAT-002 is the batch to run.
- *Confirmed* → proceed to Phase 2.
- *User names a different `FEAT-XXX`/`EPIC-XXX`* → re-resolve entry against that ID (epic path: resolve to members, `mochiko:authoring-epic` entry rules apply instead).
- *User says nothing is ready / wants something new* → route out: a brand-new capability to `/mochiko:specify`, a feature-keyed delta to `/mochiko:feature`; this run does not open.

## Phase 2 — Sufficiency check (scope-entry)
**Seat:** an independent grader that authored none of the graded sources — DM's staffing call, e.g. `mochiko:validator` or an unused `mochiko:tech-lead` instance (`impl.seat-sufficiency-independence`).
**Reads:** `spec.md` (note-search), `.mochiko/product/architecture/spine.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, FEAT-002 `entry.md`.
**Procedure:** ten-clause check per row (W1, W2), per `mochiko:review-sufficiency`; absent-baseline branch applied to the missing `.mochiko/memory/{codebase-analysis.md,governance-intent.md,knowledge-management.md}` and the absent `.claude/rules/mochiko/` governance region — surfaced, never auto-resolved, never failing.
**Expected verdict (both rows gap, given what's on disk):**
- FEAT-002's own entry states no architecture link / no store delta exists.
- Spec FR-103 requires a **background index worker**; the ruled spine states *"Synchronous request/response only; no queues, no background workers"* — a direct contradiction → architecture-store gap.
- `contracts/api.yaml` has no `GET /notes/search` path → API-contract gap.
- No `D-XXX`/`NFR-XXX` covers how a background worker can exist under `C-001` (single-process) or the 2s freshness bound → constraints/concern-catalog gap.
**Writes:** `.mochiko/features/FEAT-002/sufficiency-report.md` (verdict, gap list, trips for run-open, quickstart.md null-path note since no external-integration surface exists, no `[MODIFY]` amendment).
Any disputed clause defaults to gap and goes to the user (`impl.sufficiency-disputed-clause`) — none anticipated here, findings look objective.

## Phase 3 — Run-open confirmation (**the entry gate**)
DM presents in one blocking, non-timed confirmation (`impl.acceptance-plain-text`):
- batch = FEAT-002, selection scope, no epic, no delta card.
- both attempt bounds restated at their only redeclaration point: 3 per-cycle grading attempts, 2 gap-rework rounds.
- the sufficiency verdict and its gap routing (architecture-store, API-contract, constraints/NFR).
- trips for the user's ruling: the FR-103-vs-spine contradiction; the absent governance region; the absent/stale `codebase-analysis.md`; the anomaly that FEAT-001 reads "delivered" yet no source tree or `.git` exists in this workspace (blocks later cold-verification, Phase 11).
- the done condition (all cards `[x]`, test-first, verified per-cycle and whole against real infra, criteria traced, landing executed whole, closes at final acceptance).

**Gate branches:**
- Accept as presented → Phase 4.
- Redeclare attempt bounds here (only legal point) → new bounds carry through the run.
- Rule differently on a trip (e.g., "amend the spec instead of adding a worker," "initialize git first," "proceed without governance region") → downstream scope adjusts accordingly, ruling recorded.
- Halt/decline → run does not open; no design phase, no code touched.

## Phase 4 — Design phase (fires: gaps were named)
**Seats (DM's call, `impl.design-seats-staffing`):** `principal-architect` for the store delta (new background-index-worker element + arrow into `notes-db`, sequence diagram for create→index→searchable, `NFR-XXX` for the 2s freshness bound); `technical-analyst` for the API-contract delta (`GET /notes/search`, query/response schema, 400s) and the `constraints-and-decisions.md` delta (`D-XXX`: in-process worker mechanism honoring `C-001`) and any `data-model.md` delta the index needs; `qa-engineer` for design-time `**TEST:**` acceptance cases. Builder (`staff-engineer`) never designs its own gaps.
Each seat works only its named gap, on a DM-approved plan, applying the simplest-execution ladder (`mochiko:patterns-plan-minimalism`), rungs disclosed.
**Writes (deltas beside baselines, never in place):** `.mochiko/features/FEAT-002/{data-model-delta.md?,contracts-delta/,constraints-delta.md}`, `.mochiko/product/architecture/` store delta (in-flight-class elements only), map-entry provenance update on FEAT-002 `entry.md` if dependencies/extent sharpen.
**Sub-gate:** if the worker mechanism reads as a commodity choice (e.g., adopt a scheduling library vs. hand-roll a loop), the adopt-first ruling halts to the user (`mochiko:patterns-adopt-first`) — branches: pick the shelf candidate, or approve the minimal hand-rolled loop. If `C-001` appears to collide with a candidate, only that decision pauses as a constraint-challenge finding; the rest of design proceeds.
**Review:** a non-author seat (e.g. `tech-lead` or `devils-advocate`) grades via `mochiko:review-plan-artifacts` (conformance, blocking) and `mochiko:review-feasibility` (buildability/contradiction, including the architecture pass) before the checkpoint.

## Phase 5 — Design checkpoint (**user gate, floor**)
DM presents the rendered store-delta diagram (or source + changed-AX-row table if no render surface) plus the contract/constraints/data-model deltas and the review pair's verdicts. No code written before this signs.
- *Sign* → Phase 6.
- *Revise* → design seats rework within the same gap scope, checkpoint re-fires.
- *Stop here* → run pauses cleanly, resumable later; nothing built.

## Phase 6 — Cycle-card authoring
**Seat:** a design-class seat, never the builder — e.g. `technical-analyst`. Slices per `mochiko:patterns-vertical-tdd` (foundation before feature): plausibly Cycle 1 = index-worker scaffold (foundation), Cycle 2 = `GET /notes/search` (W1/US-101), Cycle 3 = freshness round-trip (W2/US-102) — actual slicing is the seat's call. `qa-engineer` authors each `**TEST:**` real-infrastructure gate. Cards carry stories/rationale, dependencies, acceptance-criteria IDs (SC-101–103), the TEST gate, and brownfield exposure (`[EXTEND]` on FEAT-001's existing notes store) — no task lists, no file paths.
**Writes:** `.mochiko/features/FEAT-002/tasks.md` (via `mochiko-cli template tasks`, or drafted directly against `plugins/mochiko/schemas/tasks.yaml` read raw if the binary is absent).
Independent verification seat reviews quality/buildability before confirm.

## Phase 7 — Card confirm (**user gate, floor**)
- *Rule slicing acceptable* → Phase 8.
- *Request re-slicing* → card-authoring seat revises, confirm re-fires.
- *Halt* → pauses before any code.

## Phase 8 — Build, per cycle, test-first
**Seat:** `staff-engineer`, never the design/card author. Per card: decompose into tasks (disclosed in `cycle-report.md`), `mochiko:brownfield-integration` on FEAT-001 touches, `mochiko:patterns-code-minimalism` at decomposition (rungs disclosed), red→green→refactor on a DM-approved plan.
**Writes:** application code (paths unknown until design lands — none exist yet), `.mochiko/features/FEAT-002/cycle-report-<n>.md`.
**Mid-cycle gates:** undesigned structure discovered → that cycle halts, design phase re-fires scoped to the discovery (loops back to Phases 4–5 for just that gap). An infeasible card escalates to the user as a scope call — branches: narrow scope, accept added cost, or defer the row.

## Phase 9 — Per-cycle verification
**Seat:** never the implementer — e.g. `qa-engineer`, via `mochiko:testing-end-user` (real infra, evidence captured) plus the advisory `mochiko:review-code-minimalism` lens. Each grading pass consumes one of the 3 per-cycle attempts. Gates run the full quality-gate suite, never triaged — any failure fails the cycle. Two unchanged-findings rounds = no-progress stop, halt and present. Passing cycles flip `[x]` in `tasks.md`. Minor findings → BACKLOG.md booking; Important+ → blocks cycle, joins next checkpoint batch.
**Gate at exhaustion/no-progress:** disposition (extend attempts, accept partial, halt) is the user's.
Repeats until all FEAT-002 cards are checked or a bound trips.

## Phase 10 — Regression sweep + gap-finding (selection scope, so it fires)
Runs FEAT-001's durable gates (`.mochiko/features/FEAT-001/gates.md` — 3 TEST cases) plus any seam FEAT-002 touches on FEAT-001's already-delivered side.
**Seat:** fresh `devils-advocate`, dispatched blind, two messages — first carries only `spec.md`, `sufficiency-report.md`, design deltas, and baselines (never code/tasks.md/TEST cases/reports); states derived expectations before probing.
Findings split: spec-required-behavior-broken → fails final validation; beyond-spec → advisory, user disposes (fix-now/backlog/accept-as-designed). Findings in FEAT-001's territory route to a `/mochiko:feature` delta card instead of in-run rework.

## Phase 11 — Cold verification
Build + full gate suite from a dependency-cold snapshot (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-<purpose>/`).
**Blocker carried from Phase 3's trip:** this workspace is currently not a git repository, so this step cannot run as specified.
**Gate:** 
- *User authorizes `git init` here* → snapshot proceeds normally.
- *User declares this out of scope for a plan-only/fixture context* → DM records the deviation; without this evidence, asserting verification-complete later would itself trip `impl.fail.no-evidence`, so acceptance cannot honestly close without resolving this one way or another.

## Phase 12 — Final validation / gap-rework loop
Spec-required findings from Phases 10–11 must resolve before acceptance. Rework rounds bounded at 2 (or the run-open redeclared value); a finding localized to one cycle instead charges that cycle's remaining attempts. Bound exhaustion or unchanged findings halts the run — disposition is the user's.

## Phase 13 — Landing (executed whole)
- **Graded folds** (three-way diff, checked by the landing verification seat): `contracts/api.yaml`, `constraints-and-decisions.md`, `data-model.md` if touched.
- **Store landing:** delta elements flip built, FEAT-002's key clears, touched `AX-XXX` rows get graded `As-built:`/`Drift:`, orphan check runs, `ARCHITECTURE.md` regenerated by the store skill — never hand-edited.
- **Map graduation:** W1/W2 fold into FEAT-002's Extent, status → `delivered` (dated), `FEATURES.md` index line updates, note-search spec reads closed.
- **Gates fold:** fix-now/backlog findings fold into a new `.mochiko/features/FEAT-002/gates.md`, authored by `qa-engineer`.
- KM landing skipped (`knowledge-management.md` absent).

## Phase 14 — Final acceptance (**user gate, floor, plain blocking text**)
DM presents the closing verdict against the done condition.
- *Accept* → run closes DONE; DM suggests a commit, never runs git mutations itself.
- *Amend* → user specifies changes; DM routes back to the relevant earlier phase (content amend → build/verify; scope amend → design).
- *Reject* → run closes NOT DONE regardless of everything else (`impl.fail.no-acceptance`).

## Phase 15 — Close-out
DM surfaces rounds consumed and seats spawned at each checkpoint, and closes with an explicit verdict checked against all 15 fail-condition clauses in `impl.sec.fail-conditions` (count cross-checked against the `.md`'s hard-coded number — matches at 15, no halt needed on that count).