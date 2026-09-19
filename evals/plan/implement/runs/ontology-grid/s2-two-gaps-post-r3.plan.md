# Action Plan — `/mochiko:implement FEAT-002` (plan-only, not executed)

## Grounding read already performed
- `plugins/mochiko/schemas/implement.yaml` (raw, full) and `plugins/mochiko/schemas/common.yaml` (raw, full) — 15 `kind: fail` nodes confirmed against the `.md`'s hard-coded Not-done count (no halt needed).
- Repo state: no `.git`, no `CLAUDE.md` (`governance_region` absent), no `.claude/rules/mochiko/`, no `.mochiko/memory/` (no `knowledge-management.md`, no `codebase-analysis.md`), no `.mochiko/epics/`, no `BACKLOG.md`/`ROADMAP.md`.
- `FEATURES.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/features/FEAT-001/{entry,gates}.md`, `ARCHITECTURE.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/{data-model.md,constraints-and-decisions.md,contracts/api.yaml}`, `.mochiko/specs/note-search/spec.md`.

## What entry resolves to
`FEAT-002` is a plain `FEAT-XXX`, not `EPIC-XXX` → **scope = selection**. Work rows W1/W2 carry ratified scope from the spec's accepted selection (2026-08-26). Dependency FEAT-001 is `delivered` → does not block. `depth` cannot be resolved (no governance region present) — carried into run-open as an unresolved condition, not silently defaulted.

A structural tension is already visible from the static content and will shape the whole run: the spec's FR-103 requires a **background index worker**, while the ruled architecture spine states flow is "synchronous request/response only; no queues, no background workers" (`.mochiko/product/architecture/spine.md`). This is very likely to surface as a sufficiency gap and, later, as a genuine structural (store) delta — flagged now, ruled nowhere yet.

---

## Phase 1 — Entry gating
**Does:** Resolve `FEAT-002` off `FEATURES.md`; confirm it carries selected work rows with ratified scope (`entry.md` W1, W2); confirm scope = selection (not epic/delta); check dependency FEAT-001 is `delivered` (no block).
**Reads:** `FEATURES.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/features/FEAT-001/entry.md`.
**Writes:** none yet.
**Seats:** none spawned yet — DM-only routing.
**Gate:** none at this phase (routing only).

## Phase 2 — Sufficiency check (binding verdict)
**Does:** Dispatch `mochiko:review-sufficiency`'s ten-clause check per selected row (W1, W2) over spec + architecture store + product baselines, run by a seat that authored none of those sources. Staffing is DM latitude (`impl.staffing-latitude`); no seat is pre-named for this in `vars:`, so a neutral grading seat (e.g. `mochiko:validator`) would be proposed, since `technical-analyst`/`principal-architect` plausibly authored the graded baselines/store.
**Reads:** `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/contracts/api.yaml`.
**Writes:** `.mochiko/features/FEAT-002/sufficiency-report.md` (verdict, per-row sufficient/gap list; any disputed clause defaults to gap and routes to the user, never cleared solo).
**Expected verdict (content-grounded, not asserted as fact):** gaps, at minimum — no `/notes/search` entry in `contracts/api.yaml`; no architecture-store element/AX row for a background index worker or search-index storage, directly conflicting with the spine's "no background workers" statement; no NFR for the 2-second freshness bound (SC-103); no data-model entry for the search index; no technology decision (D-XXX) for the indexing approach (commodity category → adopt-first territory).
**Gate:** none standalone — its output feeds the run-open confirmation next.

## Phase 3 — Run-open confirmation (the entry gate)
**Confirms with the user, in one message, no negotiation:**
- Batch and scope type: FEAT-002 "Note search," selection scope, selected rows W1/W2.
- Attempt bounds (this run's only redeclaration point): 3 verification attempts per cycle, 2 gap-rework rounds at final validation (schema defaults) — offered for override now or never again.
- The sufficiency verdict and its gap routing (Phase 2's list) — the design phase will fire scoped exactly to those gaps.
- Trips/conflicts for ruling: (a) absent governance region — surfaced per `impl.absent-surfaces`, not run-failing; (b) no git repository present at all — `impl.no-git-mutations` is moot (nothing to suggest against) but the final-validation `impl.cold-verification` floor rule needs a git snapshot (`git ls-files -co ...`) that cannot run without one — this is a `class: floor` obligation, non-waivable, so it needs the user's ruling on how to satisfy it (initialize git now, or an alternative the user accepts); (c) the spec-vs-spine background-worker conflict flagged above, which the design phase and its checkpoint exist to resolve, not something to design around silently; (d) `depth` unresolved (no governance region) — flagged so the user knows the mutation-lens condition can't be evaluated as stated.
- Done condition stated verbatim: every cycle card `[x]`, built test-first, independently verified per-cycle and whole against real infrastructure, code meeting criteria/tracing to requirements/aligned to governance, acceptance landing executed whole, run closes at final acceptance.

**Gate — what's confirmed and the branches:**
- *User confirms as presented* → proceed to Phase 4 (design phase fires, since gaps exist).
- *User changes attempt bounds* → new bounds recorded here (only place they can change) and carried through Phases 6–8.
- *User rules the git-repo gap by asking to `git init` first* → pause the run, that init happens outside this run's writes, then re-enter Phase 3.
- *User rules to proceed without git, accepting a stated deviation from the cold-verification floor* → recorded as an explicit user ruling at Phase 7, not a silent skip.
- *User disputes scope (e.g., wants this run as a delta card instead)* → reroute to `/mochiko:feature`; this run does not proceed.

## Phase 4 — Design phase (fires — gaps were named)
**Does:** Author exactly the named gaps, nothing more, each on a DM-approved plan (`impl.design-gaps-only`, ladder per `mochiko:patterns-plan-minimalism`).
**Seats (DM's call, per `vars:`):** `technical-analyst` (design_seat) for the API-contract delta, data-model delta, and the indexing technology decision (D-XXX, with an adopt-first pass — SQLite FTS5 vs. custom — since it's a commodity category; the adopt-first ruling itself is reserved to the user, never builder- or design-seat-decided); `principal-architect` (architect_seat) for the store delta — a new element/AX row for the background index worker and search-index storage, an NFR for the 2-second freshness bound, and the diagram delta against the currently-ruled "no background workers" spine; `qa-engineer` (qa_seat) for the design-time acceptance **TEST:** cases.
**Reads:** `sufficiency-report.md`, `spec.md`, current `.mochiko/product/architecture/spine.md`, `data-model.md`, `constraints-and-decisions.md`, `contracts/api.yaml`.
**Writes (deltas beside baselines, never in place):**
- `.mochiko/features/FEAT-002/data-model.md` (delta: search-index entity/shape)
- `.mochiko/features/FEAT-002/contracts/` (delta: `GET /notes/search` endpoint)
- `.mochiko/features/FEAT-002/baseline-delta.md` (or equivalent before/after prose delta) for the constraints-and-decisions D-XXX indexing decision
- A staged architecture-store delta (new AX row(s), diagram delta) per `mochiko:authoring-architecture-store` — not folded into `spine.md` yet, only the landing does that
- `.mochiko/features/FEAT-002/entry.md` gets the design-implied dependency/extent assertion and the architecture-link field filled once the store delta exists
**Independent review (non-author, before checkpoint):** `mochiko:review-plan-artifacts` (conformance to the gap list, card/coverage quality — blocking) and `mochiko:review-feasibility` (buildability/contradiction — e.g., does the proposed worker actually meet the 2s bound without blocking creates).
**Gate — the design checkpoint (user's):** presents the rendered diagram/diff plus the changed AX-XXX row table, and the design deltas.
- *Sign* → proceed to Phase 5.
- *Request changes* → design seats rework within the same gap scope (counts toward the 2 gap-rework rounds only if this recurs at final validation, not here — design-checkpoint iteration itself is not attempt-bounded by the schema).
- *Reject wholesale* (e.g., disagree that a background worker is the right shape) → run pauses; escalates to a scope/spec-level conversation outside this run.

## Phase 5 — Cycle-card authoring
**Does:** A design-class, non-builder seat (e.g. `technical-analyst` or `principal-architect`, whoever is least loaded) slices the signed design into cycle cards — foundation cycles before feature cycles, test-first, per `mochiko:patterns-vertical-tdd`. Given the design, expect: a foundation cycle (index storage + background worker skeleton, no user-facing behavior) ahead of a W1 cycle (`GET /notes/search`) and a W2 cycle (freshness-within-2s, background refresh). `qa-engineer` authors the **TEST:** cases within that slicing.
**Reads:** signed design deltas, `spec.md` (for cited acceptance-criteria IDs).
**Writes:** `.mochiko/features/FEAT-002/tasks.md` (cycle cards from the tasks template/schema — stories, dependencies, acceptance-criteria IDs, a **TEST:** real-infra gate per card, brownfield exposure — no task lists or file paths).
**Independent review:** the verification seat reviews cards for quality (`mochiko:review-plan-artifacts`) and buildability before confirm.
**Gate — the card confirm (user's):** presents the slicing.
- *Confirm* → proceed to Phase 6.
- *Amend slicing* → cards reworked, re-presented; still pre-build, no attempt charged.
- *Reject* → back to Phase 4 design rework.

## Phase 6 — Build (per cycle, foundation first)
**Does:** `staff-engineer` (builder_seat) decomposes each card into concrete tasks at build time, builds test-first (red→green→refactor) via `mochiko:executing-tdd-cycle`, follows `mochiko:brownfield-integration` on any touch to existing `api-service`/`notes-db` code, applies the pre-code minimalism ladder at decomposition (`mochiko:patterns-code-minimalism`, rungs disclosed).
**Writes (per cycle):** `.mochiko/features/FEAT-002/cycle-report.md`-class report (decomposition, honest difficulties, deviations, `domain_deps_added`); flips that card's checkbox in `tasks.md`.
**Escalations mid-build:** an undesigned-structure discovery halts that cycle and re-fires the design phase scoped to the discovery (back to something like Phase 4, same grade/checkpoint); an infeasible card escalates to the user as a scope decision; a build-time commodity/build-vs-buy call or IP-XXX provisioning call halts to the user, never builder-decided.

## Phase 7 — Per-cycle verification
**Does:** An independent seat (never the builder — e.g. `qa-engineer`) runs the card's **TEST:** gate against real infrastructure (`mochiko:testing-end-user`, evidence captured not assumed) plus the advisory code-minimalism lens (`mochiko:review-code-minimalism`) reading the diff, cycle report, and surrounding code.
**Writes:** a per-cycle verification report in `.mochiko/features/FEAT-002/`.
**Attempt economy:** each grading round charges one of the 3 (or user-redeclared) per-cycle attempts; 2 consecutive rounds with unchanged findings is a no-progress stop — halt that cycle, present state to the user.
**Escalation batching:** Important-or-above findings block the cycle and join the next checkpoint batch; Minor findings default to a `BACKLOG.md` booking (would need minting, since none exists), never an in-cycle fix.
**Deviation gate (floor, user's):** any cycle that adds/removes a box or arrow, or moves a responsibility, versus the signed delta stops and presents — build as approved, or amend the delta by the user's ruling.
Loop Phases 6–7 across all cycles (foundation → W1 → W2) until every card is `[x]`.

## Phase 8 — Final validation (whole-build)
**Does:**
- Full quality-gate suite run (`impl.gates-full-suite`, never triaged).
- Regression sweep: re-run FEAT-001's durable gate set (`.mochiko/features/FEAT-001/gates.md`, 3 TEST cases) since FEAT-002 shares `api-service`/`notes-db` territory (seam ownership sits with FEAT-002 as the later-landing feature).
- Cold verification: build/run gates from a dependency-cold snapshot of the uncommitted working tree at `.claude/worktrees/mochiko-<purpose>/` — **blocked as written**, since there is no `.git` in this workspace; this is exactly the Phase-3 trip that needs the user's prior ruling before this phase can execute as specified.
- Gap-finding pass (fires — scope is selection): two-message blind dispatch of a fresh `devils-advocate` (gap_finder_seat), first message carrying only `spec.md`, `sufficiency-report.md`, design deltas, and the baselines (`data-model.md`, `contracts/`, the store's NFR rows) — never code, `tasks.md`, TEST cases, or reports; seat states derived expectations before probing.
- Mutation lens: gated on `depth: high`, which is unresolved here (no governance region) — flagged, not silently skipped.
**Writes:** `.mochiko/features/FEAT-002/final-validation-report.md`-class report, stating the cold-verification blocker explicitly if unresolved, and (selection scope) never silently states a gap-finding skip since that pass does fire here.
**Findings routing:** spec-required-behavior-broken findings fail final validation until resolved; beyond-spec findings are advisory, disposition (fix now / backlog / accept-as-designed) reserved to the user.
**Gap-rework bound:** 2 rounds (or user-redeclared) at run scope; a finding localizing to one cycle's territory charges that cycle's remaining attempts instead; exhaustion or no-progress halts and presents state, disposition the user's.

## Phase 9 — Landing (on the path to acceptance)
**Does, executed whole:**
- Store landing: the signed delta's elements flip built, FEAT-002's key clears, `As-built:`/`Drift:` fields written as judgment and independently graded, orphan check runs, `ARCHITECTURE.md` regenerated (never hand-edited).
- Baseline folds (graded, three-way diff, each exactly once): `contracts/api.yaml` gains `GET /notes/search`; `data-model.md` gains the search-index entity; `constraints-and-decisions.md` gains the indexing D-XXX row.
- Map landing (selection scope): W1/W2 fold into FEAT-002's extent lines and vanish; entry status flips to `delivered` (dated); `FEATURES.md` index line updates; the note-search spec's index row derived-closes since both selected rows folded.
- Gates fold: fix-now/backlog gap findings fold into a newly minted `.mochiko/features/FEAT-002/gates.md`, authored by `qa-engineer` in the **TEST:** grammar; accepted-as-designed findings do not fold.
- `km-landing` skipped (no `knowledge-management.md` present).
**Writes:** all paths above, plus any `BACKLOG.md` bookings.

## Phase 10 — Final acceptance (the closing gate, user's)
**Presents:** the done-condition checklist against actuals — cards `[x]`, per-cycle and whole verification evidence, landing already executed, any accepted advisory findings/backlog items.
- *Accept* → run closes with a verdict against the done condition (`impl.dm-close-verdict`); this is the terminal state.
- *Amend* → named amendments route back to the relevant phase (cycle rework charges the gap-rework bound if at final-validation scope, or a cycle's own bound if localized); re-present at acceptance once resolved.
- *Reject* → run halts without closing; landing writes already made stand as record (baselines are never edited in place, so nothing to unwind there), and the user is asked how to proceed (revise spec/design, or route further work to `/mochiko:feature`).

## Not-done set carried through this run (all 15, none pre-tripped)
Sufficiency report unrecorded · design skipped/unsigned on named gaps · card built by its author or before confirm · a card left unchecked · a failing quality gate · verification claimed without real-infra evidence · a regression in FEAT-001's gates · a baseline edited in place instead of via graded delta · an unresolved deviation · store landing missing its built-vs-signed diff or an unflipped in-flight element · a baseline accepted without its graded fold · missing gap-finding pass (fires here — selection scope) · unresolved spec-required-behavior break · no user acceptance. Any one standing at closeout fails the run regardless of how far Phases 1–9 progressed.