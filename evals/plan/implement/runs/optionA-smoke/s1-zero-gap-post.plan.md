# Action Plan — `/mochiko:implement FEAT-001` (plan-only, nothing executed)

## Phase 0 — Load the governing schema (first action, already performed)
- **Reads:** `plugins/mochiko/schemas/implement.yaml` (raw, full — all 6 sections), `plugins/mochiko/schemas/common.yaml` (raw, full — for the `extends: common.*` stubs), `plugins/mochiko/schemas/command-labels.yaml`.
- **Result:** `vars` resolved (`attempt_bound_cycle=3`, `gap_rework_bound=2`, `builder_seat=staff-engineer`, `design_seat=technical-analyst`, `architect_seat=principal-architect`, `qa_seat=qa-engineer`, `gap_finder_seat=devils-advocate`, `explore_model=haiku`, plus the four path vars). Counted `impl.sec.fail-conditions` rules labeled `fail-condition`: **15**, matching the command's hard-coded Not-done count — in sync, no halt needed.
- **Writes:** none.

## Phase 1 — Entry resolution & context assembly
- **Reads:** `.mochiko/features/FEAT-001/entry.md` (status `selected`, work rows W1 "Create a note" and W2 "Fetch a note by id", both selected, selection source = the note-capture spec's accepted selection, ratified 2026-08-20; dependencies: none — first capability on the map), `.mochiko/specs/note-capture/spec.md` (US-001/US-002, FR-001..004, SC-001..003, edge cases; no UX surface), `.mochiko/product/architecture/spine.md` (AX-001 persistence, AX-002 logging both `ruled`; AX-003 auth `n/a`), `.mochiko/product/constraints-and-decisions.md` (C-001, D-001, D-002 present), `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`.
- **Determination:** `FEAT-001` is a plain capability ID, not `EPIC-XXX` — no epic resolution, `mochiko:authoring-epic` not invoked. Scope type is **selection scope**, not delta scope (no `/mochiko:feature` delta card present). No selected row depends on an undelivered row, so nothing blocks on dependency order.
- **Absent-surface check** (`impl.absent-surfaces`): no source code found anywhere in the tree and no `.mochiko/memory/codebase-analysis.md` → this reads as **greenfield**; proceeds with a logged warning rather than offering `/mochiko:setup` (that offer is reserved for brownfield/stale analysis). `.claude/rules/mochiko/` is empty → governance region is **missing**; this is surfaced to the user later but never blocks or fails the run on its own. The architecture store already has ruled content, so no bootstrap trip fires.
- **Writes:** none yet.

## Phase 2 — Sufficiency check (binding verdict, per row)
- **Seat:** an independent grading seat that authored none of spec.md, the architecture store, or the product baselines (`impl.seat-sufficiency-independence`). Candidate: `mochiko:qa-engineer` (staffing is a DM judgment call, `impl.staffing-latitude`) — it is exempt from plan approval like any grading seat.
- **Procedure:** runs the ten-clause check per selected row (W1, W2) per `mochiko:review-sufficiency` (procedure owned there, referenced not restated) against spec.md, the architecture spine, and the three product baselines. Never reads code, tasks.md, `**TEST:**` cases, or cycle reports (none exist yet regardless).
- **Writes:** `.mochiko/features/FEAT-001/sufficiency-report.md` — the store-consult result, any no-delta claim, trips for the user, the `quickstart.md` null-path note (no external integration surface here — in-process SQLite + stdlib HTTP only), and any `[MODIFY]` amendment (none — nothing delivered yet to amend).
- **Outcome branches** (the verdict is a computed grading result, not something this plan asserts in advance):
  - **Branch A — zero-gap:** all clauses clear for W1 and W2 (plausible here: FR/SC/edge cases, data model, API contract, and store NFR-001/NFR-002 traced to FEAT-001, D-001/D-002/C-001 covering storage and transport, are all already present and specific). Design phase does not fire.
  - **Branch B — gaps named:** one or more clauses fail (e.g., an untraced NFR, an ambiguous edge case, a missing decision) → design phase fires scoped to exactly those named gaps.
  - A disputed clause never self-clears — it defaults to gap and is escalated to the user (`impl.sufficiency-disputed-clause`).

## Phase 3 — Run-open confirmation **(USER GATE)**
One confirmation, no negotiation, per `impl.dm-entry-gate` / the Adaptive Goal Protocol:
- States: batch = FEAT-001 "Note capture", selection scope, rows W1+W2, no epic, no delta card.
- Restates attempt bounds at their only redeclaration point: `attempt_bound_cycle=3` per cycle-grading round, `gap_rework_bound=2` rounds at final validation — user may redeclare either now, never later.
- Presents the Phase 2 verdict, its gap routing (Branch A or B), and every trip/conflict for ruling: the missing governance region, the greenfield warning, any disputed sufficiency clause.
- States the done condition verbatim (all cards `[x]`, test-first, per-cycle + whole real-infra verification, criteria/requirements/governance alignment, whole-landing acceptance, close on accept/amend/reject).
- **Gate — onward branches:**
  - *User declines / wants scope changed* → run does not open; routes back to `/mochiko:specify` (new capability work) or `/mochiko:feature` (delta) as applicable; no design or build proceeds.
  - *User accepts (with or without redeclared bounds)* → run opens; proceeds per whichever branch Phase 2 fixed.

## Phase 4 (conditional — fires only on Branch B) — Design phase
- **Seats:** `technical-analyst` (design_seat) for requirement/decision-shaped gaps, `principal-architect` (architect_seat) only if a store delta is implicated, `qa-engineer` (qa_seat) for any needed `**TEST:**` case material — staffing mix is a call within `impl.design-seats-staffing`. `staff-engineer` never designs its own gaps (`impl.builder-never-designs`). Every producing seat plans first; DM approves the plan before any write (`impl.plan-approval-producers`).
- **Scope discipline:** seats author exactly the named gaps and nothing more, each rung-justified per `mochiko:patterns-plan-minimalism` (`impl.design-gaps-only`).
- **Writes:** design deltas beside baselines at `.mochiko/features/FEAT-001/` (e.g. a `data-model.md` delta, a `contracts/` delta, a before/after prose delta against `constraints-and-decisions.md`), plus an architecture-store delta if the structural trigger fired (`mochiko:patterns-system-design` for the diagram/register, `mochiko:authoring-architecture-store` for grammar/lifecycle). No baseline seed needed — all three baselines already exist. The phase also asserts design-implied dependencies/sharpened extent onto `FEAT-001/entry.md` with provenance and fills the architecture link if a store delta exists (`impl.design-map-assertion`).
- **Review (non-author seat, before checkpoint):** `mochiko:review-plan-artifacts` (conformance to the gap list, card-adjacent quality — blocking) and `mochiko:review-feasibility` (buildability/contradiction, plus the architecture pass if a store delta exists).
- **Gate — design checkpoint (USER):** the user signs the design and any store delta (store delta signed on a rendered diagram plus its changed `AX-XXX` row table, or source+table if no render surface).
  - *Signs* → proceed to Phase 5, cards built from spec.md + sufficiency-report.md + the signed deltas.
  - *Requests rework* → design seats revise within the gap-rework bound; exhaustion or two consecutive unchanged rounds halts and presents state, disposition is the user's.
  - *Stops here* → run pauses at this checkpoint, resumable later; no FAIL yet since no code was written.

## Phase 5 — Cycle-card authoring + card confirm **(USER GATE)**
- **Seat:** a design-class seat, never the builder (`impl.seat-card-author-independence`) — `qa-engineer` (authors the `**TEST:**` cases within its slicing) or `technical-analyst`.
- **Structure:** `mochiko:patterns-vertical-tdd` owns slicing (Simple/Split/Merge) and the walking-skeleton-first rule — this opens a brand-new end-to-end path (client → api-service → notes-db doesn't exist yet), so Cycle 1 is a walking skeleton. Likely (author's judgment, not fixed here): C1 = thinnest POST /notes → SQLite write → 201; C2 = full create-validation bundle (US-001, SC-001/SC-002 incl. empty-text 400); C3 = fetch-by-id bundle (US-002, SC-003, 200 + 404). Brownfield exposure on every card = `none` (pure greenfield).
- **Writes:** `.mochiko/features/FEAT-001/tasks.md`, rendered from `mochiko-cli template tasks` or, absent the binary, read raw from `plugins/mochiko/schemas/tasks.yaml` as source of truth. Cards carry Stories/Depends-on/Case/Brownfield-exposure and closing `**TEST:**` bundles citing US-#/SC-# per case — no task lists, no file paths (builder decomposes at build time).
- **Review:** an independent verification seat reviews cards before confirm — quality per `mochiko:review-plan-artifacts`, buildability its own judgment.
- **Branch A only:** the card-authoring seat also makes the map-entry assertion the skipped design phase would have made, surfacing any drift at this confirm (`impl.zero-gap-map-assertion`).
- **Gate — card confirm:**
  - *Approved* → proceeds to Phase 6.
  - *Re-slicing requested* → author revises, re-reviews, re-confirms.

## Phase 6 — Build loop, per cycle, test-first
For each card in dependency order (foundation/walking-skeleton first):
- **Builder:** `staff-engineer`, on a DM-approved plan, runs `mochiko:executing-tdd-cycle` — decomposes the card into concrete tasks at build time (disclosed in the cycle report), drives red→green→refactor, applies `mochiko:brownfield-integration` only if touching existing code (n/a, pure greenfield) and `mochiko:patterns-code-minimalism` at decomposition with rungs disclosed.
- **Writes:** production code (api-service handlers, SQLite persistence per D-001, stdlib HTTP per D-002), test code, and `.mochiko/features/FEAT-001/cycle-report.md` entries (dated, appended) disclosing decomposition, honest difficulties, deviations, `domain_deps_added`.
- **Verification (independent seat, e.g. `qa-engineer`):** `mochiko:testing-end-user` executes the card's `**TEST:**` cases against real infrastructure (a real running service, a real SQLite file — never mocked), evidence captured; plus the advisory `mochiko:review-code-minimalism` lens over the diff, cycle-report.md, and surrounding code.
- **Attempt economy:** each grading pass consumes one of `attempt_bound_cycle` (3, or as redeclared) attempts for that cycle; two consecutive unchanged-findings rounds triggers a no-progress halt; only the user may exempt a round.
- **Deviation gate:** undesigned structure discovered mid-build halts that cycle and is presented — build as approved or amend the delta by the user's ruling; never silently designed around.
- **Escalation batching:** reserved-to-user questions accumulate to the next checkpoint unless build-blocking; Minor advisory findings default to a `BACKLOG.md` booking, Important+ blocks the cycle and joins the batch.
- **Progress surface:** the card's checkbox in `tasks.md` flips to `[x]` only when its named cases show green on real infrastructure.

## Phase 7 — Final validation (whole-build)
- **Gap-finding pass (required — selection scope, `impl.gap-finding-scope`):** two-message, blind dispatch to a fresh `devils-advocate` that authored/built/saw nothing of this cycle (`impl.seat-gap-finder-blind`, `mochiko:testing-gap-finding`). First message: spec.md, sufficiency-report.md, any design deltas, and the baselines (data-model.md, contracts/api.yaml, the store's NFR-001/NFR-002 rows) only — never code, tasks.md, TEST cases, or reports. Seat states derived expectations before probing.
- **Mutation lens:** runs on the verification seat at high depth only, owing mutation results or a stated skip.
- **Finding routing:** spec-required behavior broken → fails final validation, evidence + clause cited; beyond-spec → advisory, disposition (fix now / `BACKLOG.md` / accept as designed) is the user's; disputed kind defaults advisory and escalates.
- **Regression sweep:** FEAT-001 is the first capability on the map — no previously delivered feature `gates.md` exists yet, so this leg is a documented no-op, not a skip requiring `impl.fail.skip-unstated` (that clause is for delta/lane runs that skip gap-finding entirely, which doesn't apply here).
- **Cold verification (`impl.cold-verification`):** builds and runs the full quality-gate suite from a dependency-cold snapshot (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-<purpose>/`); first confirms a `.claude/worktrees` ignore entry — no `.gitignore` was found in this workspace, so this step would create/verify one as part of the run, or flag the gap if it cannot.
- **Quality gates:** the full repository suite, never severity-triaged — any failure fails the run outright.
- **Writes:** `.mochiko/features/FEAT-001/final-validation-report.md` (gap-finding results, mutation results/skip, regression-sweep note, cold-verification evidence).
- **Gap-rework bound:** default 2 rounds (or redeclared), unless a finding localizes to one cycle's territory (then that cycle's own remaining attempts are charged instead); exhaustion or an unchanged-findings round halts the run — disposition is the user's.

## Phase 8 — Acceptance landing (executed whole) + final acceptance **(USER GATE)**
- **Landing verification seat** (independent, never the implementer) checks every graded fold — a three-way diff per touched baseline (pre-fold + delta vs folded result, delta applied whole, nothing else changed).
- **If a store delta was signed in Phase 4:** store landing folds in three parts — delta elements flip `built`, FEAT-001 keys clear; touched `AX-XXX` rows get judgment `As-built:`/`Drift:` writes, independently graded; orphan check runs; `mochiko:authoring-architecture-store` regenerates the derived root `ARCHITECTURE.md` (never hand-edited).
- **Selection-scope landing** (`impl.landing-selection`, `mochiko:authoring-feature-map`): W1/W2 fold into FEAT-001's extent lines and vanish from pending rows; `entry.md` status → `delivered`, dated; `FEATURES.md` index line updated; the note-capture spec's index row is touched — the spec reads closed once all its selected rows have folded.
- **Gates fold:** findings ruled fix-now or backlog fold into `.mochiko/features/FEAT-001/gates.md` (minted fresh — first fold for this capability), authored by `qa-engineer` in the `**TEST:**` grammar; findings accepted as designed do not fold.
- **Any build-time baseline-delta.md entries** (unplanned D-XXX/C-XXX/IP-XXX rows discovered during Phase 6) are graded by the landing verification seat before acceptance.
- **KM landing:** `.mochiko/memory/knowledge-management.md` is absent in this workspace → this leg is not applicable.
- **Writes:** landed baselines (`data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, architecture spine, `FEATURES.md`, `FEAT-001/entry.md`, `gates.md`), plus the built-vs-signed diff document — each via its graded fold, never hand-edited directly.
- **Gate — final acceptance:** plain blocking text presenting the done-condition checklist, all evidence, and the fold summary.
  - *Accept* → run closes, verdict recorded; FEAT-001 marked delivered.
  - *Amend* → user specifies the amendment; targeted rework loops back into Phase 6/7 scoped to it, re-consuming attempt budget as applicable; landing re-executes once green.
  - *Reject* → run closes without acceptance — this is `impl.fail.no-acceptance`, a Not-done condition; run reports FAIL against the done condition with state preserved for a future run to resume.

## Phase 9 — Close & verdict
- Checks the run against all 15 `fail-condition` clauses in `impl.sec.fail-conditions` (count re-confirmed = 15, matching the command's hard-coded Not-done line).
- If none stand and Phase 8 ended in Accept → **SUCCESS** close: every card `[x]`, test-first, per-cycle + whole real-infra verification complete, landing executed whole, acceptance given.
- Any standing fail-condition (unresolved deviation, missing evidence, an ungraded fold, no acceptance, etc.) → **FAIL** close regardless of other progress (default-FAIL discipline — no output clears itself).
- Final report surfaces rounds consumed and seats spawned at every checkpoint across the whole run.

---

**Cross-cutting, applies throughout:** no git mutations beyond suggested commits (the cold-verification worktree snapshot is explicitly exempted, not a mutation); with more than one seat in play here (sufficiency grader, design seats, card author, builder, verifier, gap-finder, landing verifier), the transport floor (`mochiko:patterns-transport-floor`) governs message legs on any multi-seat messaging and topology legs on shared writes, non-waivable once triggered; every producing seat's brief carries the model-tiering rule (locate/enumerate reads spawned as a native `Explore` subagent with `model: haiku`, interpretive reads on session tier) and, once a governance region exists, names the relevant `.claude/rules/mochiko/` files as an obligated read — currently dormant since that region is absent here.