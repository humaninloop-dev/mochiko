# Action Plan — `/mochiko:implement` (plan-only, not executed)

**Grounded against this workspace's actual state** (read, not assumed): two feature entries exist — `FEAT-001` (Note capture, status `delivered`) and `FEAT-002` (Note search, status `selected`, work rows `W1`/`W2` ratified 2026-08-26 from the spec's accepted selection). No `EPIC-XXX` directories exist. No `FEATURES.md` index, no `.mochiko/memory/knowledge-management.md`, no `CLAUDE.md` governance region, and no `.claude/rules/mochiko/` were found. The architecture spine (`AX-001..003`) explicitly states "Synchronous request/response only; no queues, no background workers," while `FEAT-002`'s spec (FR-103) requires a background index worker — a real conflict this run would have to surface. The API contract (`contracts/api.yaml`) has no `/notes/search` path. `$ARGUMENTS` is empty for this invocation.

## Phase 0 — Load the binding rules (first action, before any gating)
- **Done:** Read `plugins/mochiko/schemas/implement.yaml` raw in full, and `plugins/mochiko/schemas/common.yaml` raw in full (four `extends:` stubs pull from it: `impl.tools-referenced-never-restated`, `impl.author-grader-default-fail`, `impl.model-tiering`, `impl.transport-floor`, `impl.register`), and `plugins/mochiko/schemas/command-labels.yaml` for label vocabulary.
- **Verify:** count the `kind: fail` nodes in `impl.sec.fail-conditions` — must equal 15 (the `.md`'s hard-coded Not-done count). If it doesn't, halt and surface the mismatch before opening the run.
- **Written:** nothing.
- **Gate:** none — this is a precondition, not a user checkpoint.

## Phase 1 — Entry resolution
- **Done:** `$ARGUMENTS` is empty → propose the next ready capability from the map rather than gating on a named ID.
- **Read:** `.mochiko/features/FEAT-001/entry.md`, `.mochiko/features/FEAT-002/entry.md` (only two capability entries present). `FEAT-001` is `delivered` (not eligible — nothing pending). `FEAT-002` is `selected` with ratified rows `W1`, `W2`, depending on `FEAT-001`, which is already `delivered` — dependency satisfied, batch not blocked.
- **Proposal:** `FEAT-002` (scope: **selection**) is the candidate batch to run.
- **Observation to surface (not a binding rule, but worth flagging to the user alongside the proposal):** no `FEATURES.md` index file exists even though per-capability entry files do — an anomaly noted for the run-open confirmation, not auto-fixed.
- **Written:** nothing yet.
- **Gate:** none standalone — this proposal is folded into the single run-open confirmation in Phase 3 ("one confirmation, no negotiation"), not a separate stop.

## Phase 2 — Sufficiency check (entry, per row)
- **Done:** grade `W1` and `W2` per `mochiko:review-sufficiency`'s clause set, run by a seat that authored none of `spec.md`, the architecture store, or the product baselines, and exempt from plan approval (e.g. `mochiko:validator`, staffed fresh so a later blind gap-finding dispatch isn't compromised).
- **Read:** `.mochiko/specs/note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`.
- **Absent-surface handling:** `.mochiko/memory/codebase-analysis.md` is missing on what is functionally a brownfield capability (FEAT-001 already delivered) — surfaced per the routing rule as "offer `/mochiko:setup`, or proceed greenfield with the warning logged," never auto-resolved, never failing the run. The missing governance region (`CLAUDE.md`) is likewise surfaced, not gated.
- **Concrete verdict this batch would produce:**
  - `W1` (search by query) → **gap**: no `/notes/search` path in `contracts/api.yaml`.
  - `W2` (index freshness) → **gap**: the architecture store has no element for a background index worker, no `NFR-XXX` concern row for the ≤2s freshness target — and the store's ruled topology note ("no queues, no background workers") directly conflicts with FR-103. This conflict is a **store trip**, reserved to the user, not something the grader clears alone (a disputed clause defaults to gap).
- **Written:** `.mochiko/features/FEAT-002/sufficiency-report.md` — the store-consult result, the two gaps, the conflict flagged as a trip for run-open, and the codebase-analysis absence noted.
- **Gate:** none standalone — feeds directly into Phase 3.

## Phase 3 — Run-open confirmation (the entry gate) — **USER GATE**
- **Presented as one blocking, plain-text confirmation, no negotiation:**
  - Batch: `FEAT-002`, scope type: selection.
  - Attempt bounds at their only redeclaration point: 3 verification attempts per cycle, 2 gap-rework rounds at run scope (defaults; user may redeclare here only).
  - Sufficiency verdict: `W1` gap (missing API contract), `W2` gap (missing architecture element + NFR row) — both route to an in-run design phase scoped to exactly these two gaps.
  - The trip: the ruled "no background workers" topology note conflicts with FR-103's required background worker — needs the user's ruling (amend the ruled note, or reject/reshape the requirement) or an explicit deferral on the record.
  - Done condition restated verbatim (fixed goal text from the command).
- **Branches:**
  - *User confirms as presented* → proceed to Phase 4 with the conflict ruled (e.g., "amend the note to permit an in-process worker").
  - *User redirects to a different capability* → re-derive scope from the newly named entry, redo Phase 2 for it, return here.
  - *User changes attempt bounds* → record and proceed.
  - *User defers the store-conflict ruling* → it stays open on the record; the `W2` design work that depends on it is blocked while `W1`'s design work can still proceed; this is surfaced as a standing block rather than silently dropped.
  - *User rejects the batch / wants it routed elsewhere* → run ends here without touching code; DM routes per the entry rule (new capability → `/mochiko:specify`, feature-keyed delta → `/mochiko:feature`).
- **Written:** nothing yet (the confirmation itself is recorded as part of the sufficiency report's trip disposition, not a new file).

## Phase 4 — Design phase (fires because gaps were named)
- **Done:** author exactly the two named gaps, nothing more, each seat working only on a plan the DM approved first.
  - `technical-analyst` (design seat): API-contract delta adding `GET /notes/search` (query param, ranking, 400 on missing `q`), following `mochiko:patterns-api-contracts`; lands as a delta beside `contracts/api.yaml`, never edited in place.
  - `principal-architect` (architect seat): architecture-store delta — a new element/flow for the background index worker, an `NFR-XXX` row for the ≤2s freshness target, reconciling the ruled "no background workers" note per the user's Phase 3 ruling. Any commodity-category call inside this (e.g., reaching for a job-queue library) routes to the user via the adopt-first pointer rather than being builder- or architect-decided alone.
  - `qa-engineer` (qa seat): design-time acceptance test cases within the eventual card slicing.
  - Design-implied dependencies and the sharpened extent get asserted onto `FEAT-002/entry.md` with provenance; the architecture link fills once the store delta exists.
- **Read:** the sufficiency report, `spec.md`, the current baselines (already read in Phase 2).
- **Written:** `.mochiko/features/FEAT-002/contracts/` delta (or equivalent appliable before/after form), a store delta at `.mochiko/product/architecture/` (staged, not yet folded), updated map-entry fields on `FEAT-002/entry.md`.
- **Independent review pair (before the checkpoint, blocking):** `mochiko:review-plan-artifacts` (conformance to the two-item gap list, card-quality-adjacent checks) and `mochiko:review-feasibility` (buildability/contradiction, including the architecture pass since a store delta exists) — run by a seat that authored none of this design output (e.g. `tech-lead` and/or `validator`).
- **Gate — design checkpoint (FLOOR) — USER GATE:** no code is written before the user signs both the design and the store delta (presented as source plus a changed-element table, since no render surface exists here).
  - *Signs as presented* → proceed to Phase 5. User may also stop here and resume the build later.
  - *Requests changes* → design seats revise on a new approved plan, re-review, return to this gate.
  - *Rejects the background-worker amendment outright* → FR-103 as specified becomes infeasible under the current architecture; this escalates to the user as a business-level scope decision (build a synchronous fallback that still meets ≤2s, or amend the ratified scope).

## Phase 5 — Cycle-card authoring
- **Done:** a design-class seat (not the builder) authors `tasks.md` cycle cards from the tasks template (or the raw schema fallback), foundation cycles before feature cycles. Plausible slicing for this batch: a foundation cycle adding the search endpoint against the existing store (covers `W1`/SC-101/SC-102), then a feature cycle adding the background index worker and wiring freshness (covers `W2`/SC-103). `qa-engineer` authors the `**TEST:**` real-infrastructure gate per card within that slicing; each card also states its brownfield exposure (`api-service` as `[MODIFY]`).
- **Written:** `.mochiko/specs/note-search/tasks.md` (or the feature-dir equivalent) with the two cards, unchecked.
- **Independent review before confirm:** the verification seat reviews the cards for quality (`mochiko:review-plan-artifacts`) and buildability (its own judgment).
- **Gate — card confirm (FLOOR) — USER GATE:** blocking; the user rules the slicing before any build.
  - *Confirms* → proceed to Phase 6.
  - *Requests re-slicing* → card-authoring seat revises on an approved plan, re-review, return to this gate.
  - *Pauses* → `tasks.md` persists unchecked, resumable later.

## Phase 6 — Build, cycle by cycle (test-first)
For each card, in foundation → feature order:
- **Done:** `staff-engineer` (builder — never the card author, never the gap's own designer) plans first, decomposes the card into concrete tasks at build time (disclosed in `cycle-report.md`), drives red→green→refactor via `mochiko:executing-tdd-cycle`, follows `mochiko:brownfield-integration` for the `[MODIFY]` touch, and runs the `mochiko:patterns-code-minimalism` pre-code ladder at decomposition (rungs disclosed — e.g. checking whether a stdlib timer suffices before any indexing library).
- **Verification per cycle:** an independent seat (never the implementer) runs `mochiko:testing-end-user` against real infrastructure for the card's `**TEST:**` gate, plus the advisory `mochiko:review-code-minimalism` lens over the diff, cycle report, and surrounding code. Full repository quality-gate suite runs, never triaged — one failure fails the cycle.
- **Attempt economy:** 3 attempts per cycle by default; two consecutive rounds with unchanged findings is a no-progress stop (halt, present state); only the user can exempt a round from the count.
- **Mid-build discovery branch:** if the builder hits undesigned structure (e.g. the indexing approach needs a schema change not in the signed delta), that cycle halts and the design phase re-fires scoped to just that discovery, re-graded, re-confirmed at the same gates. A build-time technology decision that isn't structural instead gets written as `baseline-delta.md` at discovery, graded by the landing verification seat before acceptance.
- **Deviation gate (FLOOR):** any box/arrow/responsibility change beyond the signed delta stops that cycle for the user's ruling (build as approved, or amend the delta) — never designed around silently.
- **Escalations:** reserved-to-user items (ambiguity, adopt-first calls) batch at the cycle checkpoint unless build-blocking, in which case they interrupt immediately.
- **Written:** `cycle-report.md` per cycle in `.mochiko/features/FEAT-002/`, `tasks.md` checkbox flipped on completion.
- **Gate:** no standalone user gate per cycle unless a reserved item needs ruling (deviation, exhaustion, ambiguity) — those interrupt as described above; otherwise the cycle proceeds and its escalations land at the batched checkpoint.

## Phase 7 — Final validation (whole-build)
- **Regression sweep:** re-run `FEAT-001`'s durable gate set (`.mochiko/features/FEAT-001/gates.md`, three `**TEST:**` cases already on file) since `FEAT-002` reads the store `FEAT-001` built — any regression here fails the run.
- **Cold verification:** snapshot the uncommitted working tree via `git ls-files -co --exclude-standard :!.claude/worktrees` into `.claude/worktrees/mochiko-<purpose>/` and run the quality gates there. **Concrete blocker to flag:** this workspace reports `Is a git repository: false` — this floor rule's git plumbing cannot execute as-is. The plan would surface this explicitly to the user at this point (offer `git init`, per the general obstacle-handling guidance) rather than silently skip or fabricate the snapshot.
- **Gap-finding pass:** fires (scope is selection). A fresh, blind `devils-advocate` dispatch, two-message protocol — first message carries only `spec.md`, `sufficiency-report.md`, the design deltas, and the baselines (never code, `tasks.md`, `**TEST:**` cases, or reports); the seat states derived expectations before probing. Mutation lens applies only if governance depth resolves to `high`; since the governance region is absent here, depth cannot be entry-derived — the plan would disclose this as a stated skip rather than silently omit it.
- **Findings routing:** spec-required behavior broken → fails final validation until resolved; beyond-spec findings are advisory, disposition (fix now / BACKLOG.md / accept as designed) is the user's; anything landing in `FEAT-001`'s already-delivered territory routes to a `/mochiko:feature` delta card instead of being reworked here.
- **Gap-rework bound:** 2 rounds by default at run scope (or charged to a single cycle's remaining attempts if the finding localizes); exhaustion or an unchanged-findings round halts the run for the user's disposition.
- **Written:** the final-validation report in `.mochiko/features/FEAT-002/`, stating the gap-finding pass ran (not skipped, since scope is selection) and the cold-verification blocker if unresolved.

## Phase 8 — Landing (executed whole, at acceptance)
- **Done:** presented to the user alongside the final-acceptance ask, then executed atomically on "accept":
  - Store landing: the signed delta's new element flips built, `FEAT-002`'s key clears, touched `AX-XXX` rows get graded `As-built:`/`Drift:` writes, orphan check runs, `ARCHITECTURE.md` regenerates (never hand-edited).
  - Graded fold: `contracts/api.yaml` and the store each fold exactly once via a three-way diff, checked by the landing verification seat.
  - Landing-selection: `W1`/`W2` fold into `FEAT-002`'s extent and vanish from pending rows; status flips to `delivered`, dated; the `FEATURES.md` index line is updated (and, given it's currently missing, would need to be created/restored here); `note-search`'s spec closes since both its selected rows folded.
  - Gates fold: any fix-now/backlog gap findings fold into a newly-minted `.mochiko/features/FEAT-002/gates.md`, authored by `qa-engineer` in `**TEST:**` grammar.
  - `km-landing` does not apply — no `.mochiko/memory/knowledge-management.md` exists.
- **Written:** `.mochiko/features/FEAT-002/entry.md` (status/extent update), `.mochiko/features/FEAT-002/gates.md` (new), `.mochiko/product/architecture/spine.md` (folded), `ARCHITECTURE.md` (regenerated), `.mochiko/product/contracts/api.yaml` (folded), `FEATURES.md` (updated/created).

## Phase 9 — Final acceptance (FLOOR) — **USER GATE**
- **Presented:** the run's verdict against the done condition, rounds/seats consumed, and the landing preview, as plain blocking text.
- **Branches:**
  - *Accept* → landing (Phase 8) executes whole; run closes as done, all 15 fail-conditions checked clear (or the one exception — `km-landing` — noted as not-applicable rather than failed).
  - *Amend* → user specifies the change; routes back to the relevant stage (cycle rework or design revision) under the same attempt/gap-rework economy, or books to `BACKLOG.md`.
  - *Reject* → landing does not execute; the run explicitly fails its done condition (`impl.fail.no-acceptance`); no destructive residue since only commits are ever suggested, never run, and the cold-verification snapshot is ephemeral/self-removed.

## Cross-cutting, throughout every phase
- **Transport floor** fires the moment more than one seat is composed (true from Phase 2 onward) — governs cross-seat messaging and any shared write to `FEAT-002/entry.md` or the store, non-waivable once triggered.
- **Model tiering:** locate/enumerate fact-finding (e.g. "does a search module already exist?") dispatches to a haiku-model `Explore` subagent; interpretive/grading reads stay on the session tier.
- **No git mutations:** commits are suggested at cycle/landing boundaries, never executed or pushed by the run.
- **Reports envelope:** every report (sufficiency, per-cycle, verification, final-validation, built-vs-signed diff) lands under `.mochiko/features/FEAT-002/`, following the report-format template.