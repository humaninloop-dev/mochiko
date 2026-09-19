# Action Plan — `/mochiko:implement FEAT-001` (plan-only; nothing executed)

## Ground truth gathered before planning

- **Schema load (first action, done):** Read `plugins/mochiko/schemas/implement.yaml` and `plugins/mochiko/schemas/common.yaml` raw, in full, plus `command-labels.yaml`. Counted the `fail-condition`-labeled rules in `impl.sec.fail-conditions`: **15**, matching the command narrative's hard-coded count — no halt needed.
- **Entry target:** `FEAT-001` resolves directly to `.mochiko/features/FEAT-001/entry.md` — "Note capture," status `selected`. It does **not** name an `EPIC-XXX` (no `.mochiko/epics/` directory exists), so the epic branch (`mochiko:authoring-epic`) does not apply.
- **Scope type:** The entry's work rows carry a `selection source: the spec's accepted selection, 2026-08-20` — this is **selection scope**, not delta scope (no `/mochiko:feature` delta card exists for FEAT-001 — only `entry.md` is present in that directory).
- **Work rows:** W1 — Create a note (US-001, SC-001/SC-002), W2 — Fetch a note by id (US-002, SC-003). Both `selected`, neither checked off.
- **Dependencies:** none declared; FEAT-001 is the first and only row in `FEATURES.md` — nothing to block on.
- **Sources in view:** `.mochiko/specs/note-capture/spec.md` (accepted), `.mochiko/product/data-model.md` (Note entity), `.mochiko/product/contracts/api.yaml` (POST /notes, GET /notes/{id}), `.mochiko/product/constraints-and-decisions.md` (C-001 single-process, D-001 SQLite, D-002 stdlib HTTP), `.mochiko/product/architecture/spine.md` (AX-001 persistence ruled, AX-002 logging ruled, AX-003 auth n/a).
- **Absent surfaces observed:** no `.claude/rules/mochiko/` and no `CLAUDE.md` (governance region absent) · no `.mochiko/memory/codebase-analysis.md` · no source code anywhere in the tree (greenfield — no prior implementation) · no `.mochiko/memory/knowledge-management.md` · no `governance-ledger.md`. The store already carries ruled content, so the "empty store" absent-surface branch does not trigger.

---

## Phase 1 — Entry gating

**Does:** Confirms the capability entry carries ratified, selected work rows and that no selected row depends on an undelivered row. Classifies scope as selection scope.
**Reads:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/specs/note-capture/spec.md` (to confirm the cited selection is the accepted one).
**Writes:** none.
**Seats/skills:** none spawned — DM-only bookkeeping.
**Gate:** none yet.

## Phase 2 — Sufficiency check

**Does:** Runs the ten-clause sufficiency check per selected row (W1, W2) over the spec, the architecture store, and the product baselines, per `mochiko:review-sufficiency`. Grades whether existing guidance (spec FR/SC, `data-model.md`'s Note entity, `contracts/api.yaml`'s two endpoints, C-001/D-001/D-002, AX-001/AX-002) is sufficient to build W1 and W2 without further design, or names a gap list.
**Reads:** the same five source files above, plus a check for `quickstart.md` (absent — C-001 states no external services, so the null-path applies) and for the governance region (absent — surfaced, not gap-scored).
**Writes:** `.mochiko/features/FEAT-001/sufficiency-report.md` — the binding per-row verdict, the quickstart null-path note, the governance-region-absent surface, and any trips for the user.
**Seats:** a grading seat that authored none of the spec/store/baselines and will not design or build this batch — recommend **validator** (fresh, otherwise unstaffed this run) over `qa-engineer`/`technical-analyst`/`principal-architect`, since those are candidates for later design or build roles and reuse would blur the "will not design or build" clause. Exempt from plan approval.
**Gate:** none directly — a disputed clause defaults to gap and is *carried into* the run-open gate below, never cleared solo by the grader.

## Phase 3 — Run-open confirmation (**user gate**)

**Does:** One confirmation, no negotiation. States:
- Batch and scope type: FEAT-001 "Note capture," selection scope, rows W1 + W2.
- Both attempt bounds at their only redeclaration point: `attempt_bound_cycle = 3` per cycle, `gap_rework_bound = 2` per run.
- The Phase 2 sufficiency verdict and its gap routing (if any).
- Trips/conflicts for the user's ruling: the absent governance region (offer nothing auto — surface only), and any disputed sufficiency clause.
- The done condition: every cycle card checked, test-first, independently verified against real infrastructure per-cycle and whole; code traces to FR-001–FR-004/SC-001–SC-003; acceptance landing executed whole; run closes on accept/amend/reject.

**Reads:** `sufficiency-report.md` from Phase 2.
**Writes:** none (the confirmation itself is recorded as part of the eventual report envelope, not a standalone file at this point).
**Onward branches:**
- **User confirms as-is** → proceed to Phase 4 with default bounds (3/2) and the verdict as graded.
- **User redeclares bounds** (e.g., raises `gap_rework_bound` to 3) → those values carry for the rest of the run; no other redeclaration point exists later.
- **User rules on a trip** (e.g., "proceed without governance region," or "run `/mochiko:setup` first") → if the ruling is to proceed, continue into Phase 4/5 with the trip logged; if the ruling is to fix the trip first, this run pauses and the user is routed to the prerequisite command (`/mochiko:setup`) before implement resumes.
- **User rejects entry** (e.g., disputes the selection itself) → route to `/mochiko:specify` (new capability work) or `/mochiko:feature` (delta card) per the Adaptive Goal Protocol's entry rule; this run does not proceed.

## Phase 4 — Design phase (conditional on named gaps)

**Does:** Fires only if Phase 2 named a gap. Authors exactly the named gaps, nothing more, each on a DM-approved plan, rung-justified per `mochiko:patterns-plan-minimalism`.
**Reads:** `sufficiency-report.md`, current baselines (`data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `architecture/spine.md`).
**Writes (only for named gaps):** design deltas beside baselines at `.mochiko/features/FEAT-001/` (e.g., a `data-model-delta.md` or `contracts-delta/` if the Note shape or endpoints need extension; a store delta only if a structural (AX-XXX) change is triggered — unlikely here since AX-001/AX-002 already cover persistence and logging for exactly this shape). Also asserts design-implied dependencies/extent onto `entry.md` with provenance.
**Seats:** `technical-analyst` (design deltas) and, only if a store delta is needed, `principal-architect`; `qa-engineer` for early **TEST:** case shaping within the design. Reviewed before checkpoint by a non-author pair: `mochiko:review-plan-artifacts` (conformance/card quality — blocking) and `mochiko:review-feasibility` (buildability/contradiction) — recommend `tech-lead` for feasibility.
**Gate (user, conditional — the design checkpoint):** presents the design and any store delta (rendered diagram + changed AX-XXX rows, or source + changed-element table if no render surface) for signature.
- **User signs as-is** → proceed to Phase 5 with these deltas as design inputs.
- **User amends** → design seat reworks the named point, re-reviewed by the same non-author pair before re-presenting; this consumes a design-phase round, not a cycle-attempt.
- **User stops here** → the run may pause; build resumes later from the signed design.

**If Phase 2 found zero gaps:** this phase is skipped entirely; the card-authoring seat in Phase 5 makes the map-entry assertion itself (`impl.zero-gap-map-assertion`), and design inputs are `sufficiency-report.md` plus the existing baselines directly. Given what Phase 2's sources already show (FR/SC fully covered, Note entity modeled, both endpoints contracted, storage/HTTP decisions ruled, both concern rows ruled), a zero-gap verdict is plausible but not something this plan can assert — it is the grading seat's call, not mine.

## Phase 5 — Cycle-card authoring

**Does:** Authors `tasks.md` cycle cards from the design inputs (or the zero-gap baseline set), per `mochiko:patterns-vertical-tdd` and the `tasks.yaml` grammar: cards as bundles of named `**TEST:**` cases, first cycle a walking skeleton (this is a brand-new end-to-end path — no existing code anywhere in the repo), citing US-001/US-002/FR-00x/SC-00x per case, real-infrastructure test grammar (e.g., `curl` against a running stdlib HTTP server backed by an actual SQLite file, never mocked).
**Reads:** `sufficiency-report.md`, design deltas (if any), `spec.md`, `data-model.md`, `contracts/api.yaml`, `plugins/mochiko/schemas/tasks.yaml` (raw, since no `mochiko-cli` binary was found in this workspace).
**Writes:** `.mochiko/features/FEAT-001/tasks.md`.
**Seats:** a design-class, non-builder seat — `technical-analyst` — with `qa-engineer` authoring the **TEST:** cases within that slicing. Card content: stories/rationale, dependencies, acceptance-criteria IDs per case, brownfield exposure (`none`, since greenfield) — no task lists, no file paths (builder decomposes at build time).
**Reviewed before confirm:** the verification seat that will later grade cycles (`qa-engineer`) reviews for quality (`mochiko:review-plan-artifacts`) and buildability (own judgment) — `impl.card-review-before-confirm`.
**Gate (user — the card confirm):** the user rules the slicing (e.g., "Cycle 1: walking skeleton — create+fetch round trip," "Cycle 2: reject empty text," or however the seat bundles W1/W2's cases) before any build starts.
- **User approves the slicing** → proceed to Phase 6.
- **User asks for a re-slice** (e.g., wants W1 and W2 as two separate cards instead of merged, or vice versa) → card-authoring seat reworks, re-reviewed by the verification seat, re-presented.
- **User flags an infeasible card** → escalated per `impl.infeasible-card-escalation` as a business-level scope decision before build.

## Phase 6 — Build (per cycle, test-first)

**Does:** `staff-engineer` builds each confirmed card in order (walking skeleton first), decomposing into concrete tasks at build time with the code in view, driving red→green→refactor per `mochiko:executing-tdd-cycle`, applying `mochiko:patterns-code-minimalism`'s pre-code ladder at decomposition (disclosed), and `mochiko:brownfield-integration` on any touch to code the walking-skeleton cycle itself introduced (later cycles touch existing project files).
**Reads:** `tasks.md`, cited spec/design IDs, the evolving codebase.
**Writes:** application code (new — a stdlib-HTTP-server + SQLite-backed notes API per D-001/D-002), plus `.mochiko/features/FEAT-001/cycle-report-<cycle>.md` per cycle (decomposition disclosed, difficulties, deviations, `domain_deps_added`).
**Contingent mid-cycle gates (only if triggered, never silent):**
- **Undesigned structure discovered** → that cycle halts; Phase 4's design phase re-fires scoped to the discovery, same review pair, same checkpoint.
- **A deviation from a signed store delta** (box/arrow/boundary change) → halts and presents to the user: build as approved, or amend the delta first.
- **A commodity-category build-vs-buy question or an IP-XXX provisioning need** arises → halts to the user per `mochiko:patterns-adopt-first`; never the builder's call.
- **Requirement ambiguity** → escalated to the user, batched at the cycle checkpoint unless build-blocking.

## Phase 7 — Per-cycle verification (independent seat)

**Does:** `qa-engineer` (never the builder) executes the card's `**TEST:**` cases against real infrastructure — a live server process against an actual SQLite file, not mocks — per `mochiko:testing-end-user`, capturing evidence. Also applies the `mochiko:review-code-minimalism` lens over the diff and cycle report (advisory-only findings).
**Reads:** the diff, `cycle-report.md`, running service output.
**Writes:** a verification report in `.mochiko/features/FEAT-001/` per `templates/report-format.md`.
**Attempt economy:** each grading round consumes one of the 3 per-cycle attempts (`attempt_bound_cycle`, as set at Phase 3). Two consecutive rounds with unchanged findings is a no-progress stop — halts that cycle and presents state to the user rather than continuing to spend attempts.
**Outcome:** on green, the card's checkbox in `tasks.md` flips — this is the progress surface. On a failing `**TEST:**` or quality gate, the cycle fails outright (gates are never severity-triaged); an Important-or-above minimalism/other advisory finding joins the checkpoint batch, a Minor one defaults to a `BACKLOG.md` booking rather than an in-cycle fix.

## Phase 8 — Repeat Phases 6–7 per remaining card

Cycle 2 (and any further card) proceeds the same way, in dependency order, until every card in `tasks.md` is checked.

## Phase 9 — Final validation (whole-build verification)

**Does, since this is a selection-scope run (gap-finding is mandatory here, not skippable):**
- **Cold verification:** snapshot the uncommitted working tree (`git ls-files -co --exclude-standard :!.claude/worktrees`) into `.claude/worktrees/mochiko-<purpose>/` and run the full quality-gate suite there, dependency-cold.
- **Regression sweep:** run the accumulated `**TEST:**` gates of previously delivered features' territory — none exist yet, since FEAT-001 is the first capability ever delivered on this map, so this sweep is vacuous by construction this run (still executed, per the rule, just with nothing to regress against).
- **Gap-finding pass:** dispatch a fresh, blind `devils-advocate` — first message carries only `spec.md`, `sufficiency-report.md`, any design deltas, and the baselines (`data-model.md`, `contracts/api.yaml`, the store's NFR-XXX rows) — never the code, `tasks.md`, `**TEST:**` cases, or any report. The seat states its derived expectations before probing begins, per `mochiko:testing-gap-finding`'s two-message blind protocol.
- **Mutation lens:** runs on the verification seat if this run is staffed "high depth" (a DM staffing call at Phase 1/3, not yet made in this plan) — otherwise skipped and stated as such.
**Reads:** everything above; **writes:** `.mochiko/features/FEAT-001/final-validation-report.md`, plus `.mochiko/features/FEAT-001/gates.md` (minted, holding any fix-now/backlog gap findings in the `**TEST:**` grammar, authored by `qa-engineer`).
**Findings routing:** spec-required behavior broken → fails final validation, must be resolved (gap-rework bound: 2 rounds at run scope, redeclared to whatever Phase 3 set, else a finding localized to one cycle charges that cycle's own remaining attempts instead). Beyond-spec findings are advisory; disposition (fix now / backlog / accept-as-designed) is the user's call, batched at this checkpoint.

## Phase 10 — Acceptance landing (**user gate — final acceptance**)

**Does:** Presents the whole build for accept/amend/reject. If accepted, executes the landing whole, atomically:
- Any touched baseline (only if Phase 4 or a build-time technical decision produced a `baseline-delta.md`) folds via a graded three-way diff, checked by the verification seat.
- Store landing (only if a store delta was signed in Phase 4) — element status flips, As-built:/Drift: written and independently graded, orphan check run, `ARCHITECTURE.md` regenerated from the store (never hand-edited).
- Map graduation: W1 and W2 fold into FEAT-001's extent lines and vanish as pending rows; `entry.md` status → `delivered`, dated; `FEATURES.md`'s FEAT-001 row updates; the spec is read as closed once both its selected rows have folded (derived, not asserted).
- Gap findings ruled fix-now/backlog fold into `gates.md`; accepted-as-designed findings do not fold.
- No `knowledge-management.md` exists in this repo, so the KM-landing obligation does not apply this run.

**Onward branches:**
- **Accept** → landing executes as above; run closes with a verdict against the done condition, reporting rounds consumed and seats spawned.
- **Amend** → the specific amendment is scoped and reworked (a card rework, a design amendment, or a finding disposition change) before re-presenting acceptance; this consumes gap-rework/attempt budget as applicable.
- **Reject** → the run closes without landing; `impl.fail.no-acceptance` stands, and the run reports FAIL against the done condition with state preserved for a future run.

## Phase 11 — Close

**Does:** DM states the run's verdict against the fixed done condition, checked against all 15 fail-condition clauses (sufficiency recorded, design signed if gaps existed, card independence/checkbox discipline honored, gates never failing, evidence real, no regression, baselines never edited in place, deviations resolved, store landing complete if triggered, every touched baseline graded-folded, gap-finding present, any skip stated, spec-gaps resolved, acceptance given). Reports rounds/seats consumed per `impl.dm-surface-rounds` and `impl.dm-close-verdict`.
**Writes:** none beyond what Phase 10 already wrote — this is a reporting step, not a new artifact.