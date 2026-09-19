Read `plugins/mochiko/schemas/implement.yaml` in full (147 lines, 6 sections, 15 `fail-condition` rules — count matches the command's hard-coded "15"), the FEAT-002 entry, the note-search spec, the architecture spine, the product baselines, FEAT-001's entry/gates, and confirmed there is no `.claude/rules/mochiko/`, no `.mochiko/memory/`, and no application source in this workspace. That grounding is used below to make each phase concrete rather than generic.

# Action Plan — `/mochiko:implement FEAT-002`

## Phase 0 — Load rules (no seat spawned)
- **Done:** Read `plugins/mochiko/schemas/implement.yaml` raw, in full, substituting `${var}` placeholders (`attempt_bound_cycle=3`, `gap_rework_bound=2`, `builder_seat=staff-engineer`, `design_seat=technical-analyst`, `architect_seat=principal-architect`, `qa_seat=qa-engineer`, `gap_finder_seat=devils-advocate`, `explore_model=haiku`).
- **Read:** the schema; `plugins/mochiko/schemas/command-labels.yaml` for label meanings.
- **Written:** nothing.
- **Gate:** none. Run is not "open" until this completes.

## Phase 1 — Entry resolution
- **Done:** `FEAT-002` is a plain capability ID, not `EPIC-XXX` (no epic lookup). Resolve scope source: `.mochiko/features/FEAT-002/entry.md` shows two selected work rows (W1 — search by query, US-101; W2 — index freshness, US-102), selection source = the spec's accepted selection (2026-08-26) → **selection scope**, not delta scope. Dependency check: entry lists FEAT-001, whose own entry shows `Status: delivered` → unblocked, batch may run now.
- **Read:** `.mochiko/features/FEAT-002/entry.md`, `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/specs/note-search/spec.md`.
- **Written:** nothing yet.
- **Gate:** none (routing only fires on missing/blocked entry, which isn't the case here).

## Phase 2 — Sufficiency check
- **Seat:** an independent grading seat that authored none of `spec.md`, the architecture store, or the product baselines — staffing is the DM's call; `qa-engineer` or `validator` fit (neither is a plausible author of those artifacts), per `mochiko:review-sufficiency`.
- **Read (fence):** `spec.md` (no Screens & Flows — API only), `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`, the FEAT-001/FEAT-002 map entries. Never code, `tasks.md`, or this batch's own run-output dir.
- **Concrete grading against what's on disk**, clause by clause:
  1. Testable criteria — SC-101/102/103 all stateable → clear.
  2. Contract exposure — `contracts/api.yaml` has no `/notes/search` path → **gap**.
  3. Data exposure — `data-model.md` models only `Note`, no search-index structure → **gap**.
  4. Structural trigger — the spine states "Synchronous request/response only; no queues, no background workers," but FR-103 requires a background index worker → store consult finds a real structural need → **gap**, store delta required.
  5. NFR targets — no AX/NFR row names a latency target for SC-103's ≤2s bound → **gap**.
  6. Commodity exposure — search mechanism (substring scan vs. SQLite FTS5 vs. external engine) has no weighed alternative anywhere → **gap**, adopt-first owed.
  7. Dependency order — only FEAT-001, delivered → clear.
  8. UX trace — no Screens & Flows manifest → n/a.
  9. Delivered-feature exposure — if the design ends up touching the notes table/persistence surface FEAT-001 delivered (e.g., an FTS index beside it), that surface is owned by a *delivered* row → likely **gap/trigger**, pending the design's actual touch; flagged for the design phase to confirm and, if so, author the `[MODIFY]` amendment on FEAT-001's entry.
  10. In-flight exposure — no other in-flight row touches notes surfaces → clear.
  - Absent surfaces surfaced (never auto-resolved, never failing): no `.claude/rules/mochiko/` governance region; no `.mochiko/memory/codebase-analysis.md` even though FEAT-001 is marked delivered (brownfield ambiguity) — both go to the user at run-open, not resolved here.
- **Written:** `.mochiko/features/FEAT-002/sufficiency-report.md` (per `templates/report-format.md`) — per-row verdict for W1 and W2, the gap list keyed to clauses 2/3/4/5/6/(9), the store-consult result, zero trips, zero in-flight conflicts, the two absent-surface notes.
- **Gate:** none yet — the verdict is binding input to the next gate, not a clearing.

## Phase 3 — Run-open confirmation (USER GATE, entry gate)
One confirmation, no negotiation. Presented to the user:
- Batch name and scope type: FEAT-002 "Note search," selection scope, rows W1+W2, dependency FEAT-001 satisfied.
- Attempt bounds at their only redeclaration point: per-cycle default 3, gap-rework default 2 — offered for override now or carried as-is.
- The sufficiency verdict and its gap routing (clauses 2/3/4/5/6/9 above), zero trips, zero in-flight conflicts, and the two absent-surface notices for ruling.
- The done condition (every card `[x]`, test-first, independently verified per cycle and whole, landing executed whole, closes at final acceptance).

**Gate branches:**
- *User confirms as stated, rules on the absent surfaces (e.g., "proceed brownfield, warning logged" or "run `/mochiko:setup` first"), accepts default bounds* → proceed to Phase 4.
- *User redeclares attempt bounds* → new bounds carried for the rest of this run, recorded in the report.
- *User disputes a clause verdict* → per `impl.sufficiency-disputed-clause`, a disputed clause defaults to gap; the user's ruling is recorded and the gap list is adjusted accordingly before Phase 4.
- *User asks for `/mochiko:setup` first* → this run pauses/exits; `/mochiko:setup` is a separate command, out of this run's scope.
- *User wants different scope (e.g., defer W2)* → routes back to `/mochiko:feature` or `/mochiko:specify` to re-rule selection before re-entering `/mochiko:implement`.

## Phase 4 — Design phase (fires: gaps were named)
Design seats author **exactly** the named gaps, each on a DM-approved plan (producer plan-approval, not a user gate), rung-justified per `mochiko:patterns-plan-minimalism`. `staff-engineer` never designs.
- **`principal-architect`** (architect_seat): closes clauses 4 & 5 — a container-delta diagram plus (since it's an async path) a sequence diagram per `mochiko:patterns-system-design`; a new in-flight-class AX row for search/indexing concern with an NFR-XXX target (create-to-searchable ≤2s at ≤10k notes, traced to SC-103); if the design touches the notes datastore, drafts the `[MODIFY]` amendment on FEAT-001's entry (clause 9).
- **`technical-analyst`** (design_seat): closes clause 2 (contracts delta: new `GET /notes/search` path, `q` param, 200/400) and clause 3 (data-model delta: either models a search-index structure or documents it as derived/non-modeled); closes clause 6 by running the commodity check per `mochiko:patterns-adopt-first` (search mechanism candidates narrowed to in-process options given C-001 — e.g., SQLite FTS5 vs. hand-rolled scan) and drafts new D-XXX rows for both the search mechanism and the background-worker mechanism in a constraints-and-decisions delta.
- **Written (deltas beside baselines, never in place):** `.mochiko/features/FEAT-002/contracts/api.yaml` (delta), `.mochiko/features/FEAT-002/data-model.md` (delta), `.mochiko/features/FEAT-002/constraints-and-decisions.md` (delta), an architecture-store delta artifact (rendered diagram + changed-AX-row table) per `mochiko:authoring-architecture-store`, and — if clause 9 fires — a marked delta on `FEAT-001/entry.md`. The design phase also asserts sharpened extent/dependencies onto `FEAT-002/entry.md` with provenance.
- **Gate inside this phase (not yet the checkpoint):** the commodity-category adopt-first ruling and any IP-XXX call are reserved to the user — these questions batch into the design checkpoint below rather than halting build-blocking mid-phase (nothing here is build-blocking pre-code).

## Phase 5 — Design review pair (independent, pre-checkpoint)
- **`mochiko:review-plan-artifacts`** (blocking): grades conformance — every named gap (2/3/4/5/6/9) closed, nothing authored beyond the gap list.
- **`mochiko:review-feasibility`**: grades buildability/contradiction — e.g., does the background-worker addition actually respect C-001 (single-process, no external services)? Does the FTS5 candidate collide with D-001 (SQLite, bundled driver)?
- **Written:** two review reports under `templates/report-format.md` in the feature dir.
- **Gate:** none user-facing yet; a `needs-revision`/`critical-gaps`/`infeasible` verdict sends the relevant design seat back to rework before Phase 6.

## Phase 6 — Design checkpoint (USER GATE, floor)
Present the rendered diagram, the changed AX-XXX row table, and the contracts/data-model/constraints deltas (plus any FEAT-001 `[MODIFY]` amendment). No code is written before this signs — the sign-off *is* the store's write gate.
**Branches:**
- *Sign as presented* → store delta is written (in-flight-class, keyed FEAT-002), `FEAT-002/entry.md`'s Architecture link fills in, proceed to Phase 7.
- *Request amendment* → design seats rework only the named delta, re-run Phase 5, return here.
- *Stop and resume later* → run pauses; deltas persist as resumable state; a later `/mochiko:implement FEAT-002` invocation resumes at card authoring.

## Phase 7 — Card authoring
- **Seats:** a design-class seat (e.g. `technical-analyst`) owns slicing per `mochiko:patterns-vertical-tdd`; `qa-engineer` authors the `**TEST:**` case content within that slicing. Neither is `staff-engineer`.
- **Concrete shape expected:** since the background-index-worker path is genuinely new, cycle 1 is likely a walking skeleton (worker ticks once, indexes an existing note, one trivial case green); cycle 2 covers US-101 (`GET /notes/search`, FR-101/102, SC-101/102, brownfield exposure: extends the existing api-service router); cycle 3 covers US-102 end to end (create→search round trip ≤2s, SC-103, depends on cycles 1–2). Final slicing judgment belongs to the authoring seat.
- **Written:** `tasks.md` (cycle cards) in the spec folder, per `plugins/mochiko/schemas/tasks.yaml`'s skeleton — stories/rationale, dependencies, acceptance-ID citations, cycle-level brownfield exposure, `**TEST:**` bundles; no task lists, no file paths.
- **Review before confirm:** an independent verification seat (not the card author, e.g. `validator`) grades quality per `mochiko:review-plan-artifacts` plus its own buildability judgment.

## Phase 8 — Card confirm (USER GATE, floor)
User rules the slicing before any build. **Branches:** accept as-is → Phase 9; request reorder/split/merge → card author revises, re-reviewed, re-presented here.

## Phase 9 — Build (per cycle, in dependency order, foundation before feature)
- **Seat:** `staff-engineer` (builder_seat) only — never designs its own gaps.
- **Bindings:** `mochiko:executing-tdd-cycle` (decomposes each card into concrete tasks at build time, disclosed in `cycle-report.md`, red→green→refactor, test-first) · `mochiko:brownfield-integration` on touches to the existing api-service/notes-db · `mochiko:patterns-code-minimalism` at decomposition (rungs disclosed). Since no `.claude/rules/mochiko/` exists, that obligated-read line is moot and noted as such in the brief.
- **Written:** code changes (paths TBD by the builder at decomposition time — e.g. router extension, worker loop, FTS setup), `.mochiko/features/FEAT-002/cycle-report-<N>.md` per cycle, `tasks.md` checkbox flips.
- **Mid-cycle escalation:** an infeasible card or undesigned structure discovered mid-build halts that cycle and re-fires a scoped Phase 4 re-fire (same review pair, same checkpoint) before resuming.

## Phase 10 — Per-cycle verification
- **Seat:** an independent verification seat, never `staff-engineer` (e.g. `qa-engineer`).
- **Bindings:** `mochiko:testing-end-user` (real SQLite file, real HTTP calls, actual timing checks against the 2s bound — never mocked) · `mochiko:review-code-minimalism` (advisory lens on the diff + cycle report + surrounding code).
- **Attempt economy:** 3 attempts per cycle (or as redeclared at run-open); two consecutive rounds with unchanged findings = no-progress stop, halted and presented to the user; Important-or-above findings join the next checkpoint batch, Minor findings default to a `BACKLOG.md` booking.
- **Written:** per-cycle verification report in the feature dir.
- Repeat Phases 7–10 across the remaining cycles.

## Phase 11 — Final validation (whole build)
- Full repository quality-gate suite, run from a dependency-cold snapshot (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-<purpose>/`); first verifies/adds the `/.claude/worktrees` gitignore entry (this repo currently has no `.gitignore`, so that entry would need creating).
- Regression sweep: re-runs FEAT-001's durable gates (`.mochiko/features/FEAT-001/gates.md` — the three persistence/validation/404 `**TEST:**` cases), since search shares FEAT-001's notes territory.
- Gap-finding pass (fires — selection scope): a fresh `devils-advocate`, two-message blind dispatch — first message carries only `spec.md`, `sufficiency-report.md`, the design deltas, and the baselines, never code/`tasks.md`/`**TEST:**` cases/reports; states expectations, then probes the running system. Spec-required findings fail final validation until resolved; beyond-spec findings are advisory, disposition (fix now / backlog / accept) reserved to the user.
- Landing verification seat grades the judgment content of the constraints-and-decisions delta (new D-XXX rows) and the store delta's As-built/Drift writes, and checks the built-vs-signed diff both directions.
- **Written:** `final-validation-report.md`, built-vs-signed diff.
- **Gap-rework bound:** 2 rounds at run scope (or as redeclared); a localized finding instead charges its cycle's remaining per-cycle attempts. Exhaustion or unchanged findings halts the run, disposition reserved to the user.

## Phase 12 — Final acceptance (USER GATE, floor)
Present: all cards `[x]`, full-suite + regression-sweep + gap-finding results, the full sufficiency→design→build trail, the landing package about to execute, and any beyond-spec dispositions owed.
**Branches:**
- *Accept* → Phase 13 executes the landing whole.
- *Amend* → user names specific changes; routes back to the implicated phase (build-level → Phase 9; design-level → Phase 6), same attempt/gap-rework economy still in force, re-presented at a fresh Phase 12.
- *Reject* → run closes FAIL; no landing executes; state (cards, reports, unsigned/unlanded deltas) is preserved for future re-entry; reject reason recorded in the final-validation report.

## Phase 13 — Acceptance landing (executed whole, only on Accept)
- Store landing (three parts): flip in-flight elements to `built`, clear their FEAT-002 keys · write graded `As-built:`/`Drift:` on touched rows · orphan check · regenerate `ARCHITECTURE.md`.
- Graded folds (three-way diff, pre-fold baseline + delta vs. folded result) for `contracts/api.yaml`, `data-model.md`, `constraints-and-decisions.md`, and the `FEAT-001` `[MODIFY]` amendment if it fired.
- Map graduation: `FEAT-002/entry.md` — W1/W2 fold into Extent, `Status: delivered` (dated), `FEATURES.md` row updates; specs-index row touched (closes exactly when all selected rows have folded).
- `gates.md` fold: accepted-fix-now/backlog gap findings written in `**TEST:**` grammar by `qa-engineer`; accepted-as-designed findings do not fold.
- **Written:** all of the above, in place, via the graded fold only.
- **Close:** run closes with a verdict against the done condition, citing the 15 fail-conditions in `impl.sec.fail-conditions` all cleared (count re-checked against the schema; if it were not 15, the run would halt and surface the mismatch before closing — not applicable here, it matched).

## Cross-cutting, applied throughout (not phase-bound)
- No git mutations beyond the ephemeral, self-removed cold-verification snapshot; commits are suggested, never made.
- Reserved-to-user questions batch at the next checkpoint; only a build-blocking one interrupts mid-cycle.
- Model tiering: locate/enumerate reads dispatch to a native Explore subagent at `model: haiku`; interpretive/absence-driven reads (e.g. grading, sufficiency clauses) stay on the session tier.
- Transport floor applies once more than one seat composes (it does, from Phase 2 on): message legs on any cross-seat relay, topology legs on shared writes (e.g. only one pen-holder at a time for any given delta file).
- Author never grades own output at any step (default FAIL) — every seat pairing above respects that.