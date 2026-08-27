## Plan-only evaluation: `/mochiko:implement FEAT-001`

*(No agents spawned, no files written, no waiting on input — every gate below is described with its onward branches instead of being taken.)*

### Phase 0 — Load the binding schema
- **Done:** Raw, full read of `plugins/mochiko/schemas/implement.yaml` (already performed for this plan) and `plugins/mochiko/schemas/command-labels.yaml`. Vars substituted: `attempt_bound_cycle=3`, `gap_rework_bound=2`, `builder_seat=staff-engineer`, `design_seat=technical-analyst`, `architect_seat=principal-architect`, `qa_seat=qa-engineer`, `gap_finder_seat=devils-advocate`, `explore_model=haiku`.
- **Read:** the schema itself, in full.
- **Verify:** fail-condition count in `impl.sec.fail-conditions` = 15, matching the command's hard-coded Not-done count — in sync, no halt needed.
- **Write:** none.

### Phase 1 — Resolve entry and scope
- **Done:** Confirm `FEAT-001` is a plain capability (not an `EPIC-XXX`), read its map entry, and establish this is **selection scope** (the spec's accepted selection, ratified 2026-08-20) — not a delta card, not an epic.
- **Read:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/specs/note-capture/spec.md`.
- **Findings folded in:** two selected work rows, W1 (create note, US-001/SC-001-002) and W2 (fetch by id, US-002/SC-003); no dependencies (first capability on the map, nothing to block on).
- **Write:** none yet.

### Phase 2 — Sufficiency check
- **Seat:** an independent grader that authored none of `spec.md`, the architecture spine, or the product baselines — e.g. `mochiko:validator` or `mochiko:qa-engineer` (my staffing call; not `mochiko:technical-analyst` or `mochiko:principal-architect`, since those plausibly authored the baselines). Runs per `mochiko:review-sufficiency`, per row (W1, W2).
- **Read (fence-bound):** `spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`, `entry.md`. Never code, `tasks.md`, or this batch's own run-output dir.
- **What I already see in the baselines** (informs likely grading, not a substitute for the seat's own verdict): contracts already publish `POST /notes` and `GET /notes/{id}`; `data-model.md` already has `Note`; architecture spine already carries AX-001 (persistence, NFR-001) and AX-002 (logging, NFR-002) both `ruled`; `constraints-and-decisions.md` already has D-001 (SQLite, adopt-first weighed) and D-002 (stdlib HTTP, minimalism-justified); no delivered or in-flight sibling features exist yet; no Screens & Flows (API-only, clause 8 n/a). This shape points toward a likely **zero-gap** verdict, but the actual clause-by-clause grading is the seat's call, not mine.
- **Absent-surface notes to log (never gap-forcing):** no governance region (`.claude/rules/mochiko/`, no `CLAUDE.md`) — surfaced to the user, not a gap. No `.mochiko/memory/codebase-analysis.md` — greenfield, proceed with the warning logged rather than gap.
- **Write:** `.mochiko/features/FEAT-001/sufficiency-report.md` (per-row verdict, gap list if any, store-consult result, any trips, any in-flight conflicts — none expected here).

### Phase 3 — Run-open confirmation (the entry gate)
- **Gate (user's), presented as one blocking confirmation, no negotiation:**
  - Batch identity: FEAT-001 "Note capture", selection scope, rows W1+W2.
  - Attempt bounds restated at their only redeclaration point: 3 attempts/cycle, 2 gap-rework rounds at run scope — open to the user changing either now.
  - The sufficiency verdict and its gap routing (per Phase 2's actual output).
  - Any trips/conflicts for ruling — plus one practical item I'd surface here: **this workspace is not a git repository**, and the run's cold-verification step and progress evidencing both assume one exists. This isn't a schema fail-condition by itself, but it blocks later steps unless resolved.
  - The done condition, stated plainly.
- **Branches:**
  - *User confirms as-is, verdict was zero-gap* → skip to Phase 5.
  - *User confirms as-is, verdict had gaps* → proceed to Phase 4.
  - *User redeclares attempt/rework bounds* → adopt the new numbers for the rest of the run.
  - *User disputes a graded clause* → that clause defaults to gap (floor rule), folded into the design-phase scope.
  - *User rules on the no-git-repo trip* → either "initialize git now" (I'd offer `git init` as a separate confirmable step before Phase 11) or "proceed, we'll sort cold-verification later" — recorded either way, not silently resolved.
  - *User wants to redirect entirely* (e.g., decides this isn't ready) → run does not open; route back to `/mochiko:specify` or `/mochiko:feature` as appropriate.

### Phase 4 — Design phase (fires only if Phase 2 named gaps)
- **Seats:** `technical-analyst` for spec/requirement-level gaps, `principal-architect` only if a store delta is triggered, `qa-engineer` for TEST-case-relevant gaps — each authors *exactly* the named gaps on a plan I approve first, ladder-justified per `mochiko:patterns-plan-minimalism`.
- **Write:** deltas beside baselines in `.mochiko/features/FEAT-001/` (e.g. `data-model.md` delta, `contracts/` delta, `constraints-and-decisions.md` delta in before/after form) plus a store delta under `.mochiko/product/architecture/` if a structural trigger fired.
- **Review pair (non-author seats):** `mochiko:review-plan-artifacts` (conformance to the gap list, blocking) and `mochiko:review-feasibility` (buildability/contradiction).
- **Gate — design checkpoint (user's):** sign the design and any store delta (rendered diagram + AX-XXX row changes, or source-plus-table if no render surface). Branches: *sign as-is* → Phase 6; *amend* → re-author the disputed piece, re-review, re-present; *stop here* → run pauses, resumable later without losing the signed work.

### Phase 5 — Zero-gap branch
- If Phase 2 returned zero gaps for both rows: no design phase fires. The card-authoring seat (Phase 6) makes the map-entry assertion the design phase would otherwise have made, and any drift surfaces at the card confirm instead.

### Phase 6 — Cycle card authoring
- **Seat:** a design-class seat distinct from the builder — `technical-analyst` (or another non-builder design seat) slices W1+W2 into vertical cycle cards per `mochiko:patterns-vertical-tdd` (walking skeleton first — likely a foundation cycle for the SQLite-backed store + minimal HTTP server, then a create-note cycle, then a fetch-by-id cycle). `qa_seat` (`qa-engineer`) authors the closing `**TEST:**` real-infrastructure gate per card.
- **Write:** `.mochiko/features/FEAT-001/tasks.md`, cards holding stories/rationale, dependencies, acceptance-criteria IDs, brownfield exposure (expected `[NEW]` throughout — no existing code to extend here), and the TEST gate — no task lists or file paths at this layer.
- **Review:** a verification seat (e.g. `qa-engineer`, independent of the builder) reviews cards for quality (`mochiko:review-plan-artifacts`) and buildability before confirm.
- **Gate — card confirm (user's):** rules the slicing. Branches: *confirm* → Phase 7; *re-slice* (split/merge a card) → re-author affected cards, re-review; *defer a card* → routes out of this run's scope (e.g. to BACKLOG.md), remaining cards proceed; *reject* → run halts here.

### Phase 7 — Build, cycle by cycle
- **Seat:** `staff-engineer` (builder), never the card author. Decomposes each confirmed card into concrete tasks (disclosed in `cycle-report.md`), builds test-first (red→green→refactor), applies `mochiko:patterns-code-minimalism` at decomposition (rungs disclosed). `mochiko:brownfield-integration` is not expected to fire — this is greenfield, no existing code to extend/modify.
- **Write:** actual source files (exact paths determined by the builder's decomposition — plausibly a small stdlib-HTTP server module, a SQLite-backed notes store module, and their tests) plus `.mochiko/features/FEAT-001/cycle-report.md` per cycle.
- **Order:** foundation cycle(s) before the two feature cycles (create, fetch).

### Phase 8 — Per-cycle verification
- **Seat:** an independent verification seat (never the builder — `qa-engineer`), executes each card's `**TEST:**` gate against real infrastructure (`mochiko:testing-end-user`), plus the advisory `mochiko:review-code-minimalism` lens over the diff and cycle report. Full quality-gate suite runs.
- **Attempt economy:** 3 attempts per cycle (or the user's redeclared bound from Phase 3); two consecutive rounds with unchanged findings is a no-progress stop — halt that cycle and present state to the user (their call: rework further, carve out, or accept partial).
- **Write:** verification report per cycle in `.mochiko/features/FEAT-001/`; `tasks.md` checkbox flips `[x]` on pass (the progress surface).
- **Repeat** across all cards until every card is `[x]` or a cycle halts and is escalated.

### Phase 9 — Gap-finding pass (fires: this is a selection-scope run)
- **Seat:** a fresh `devils-advocate`, blind — never one that built these cycles or saw the design-time TEST cases. Two-message dispatch: first message carries only `spec.md`, `sufficiency-report.md`, any design deltas, and the baselines (`data-model.md`, `contracts/`, NFR-bearing store rows) — never code, `tasks.md`, TEST cases, or reports. Seat states expectations, then probes the built system black-box.
- **Mutation lens:** runs on the verification seat (`qa-engineer`), which already holds code sight — owed at high depth or a stated skip.
- **Findings split:** spec-required behavior broken → fails final validation until resolved; beyond-spec → advisory, disposition reserved to the user (fix now / BACKLOG.md / accept as designed).
- **Write:** gap-finding findings folded into the final-validation report (Phase 10).

### Phase 10 — Final validation
- **Cold verification:** build and run the full quality-gate suite from a dependency-cold snapshot of the uncommitted working tree via `git ls-files -co --exclude-standard`, copied to `.claude/worktrees/mochiko-<purpose>/` — **contingent on the git-repo trip from Phase 3 being resolved first**.
- **Regression sweep:** re-run accumulated `**TEST:**` gates of previously delivered features in this territory — none exist yet (FEAT-001 is the first capability), so this sweep is a stated no-op, not a silent skip.
- **Gates never triaged:** any failing gate (quality gate or `**TEST:**`) fails the cycle/run outright, no severity discount.
- **Write:** `final-validation-report.md` in `.mochiko/features/FEAT-001/`.

### Phase 11 — Landing (executed whole, at acceptance)
- **Store landing** (`mochiko:authoring-architecture-store`) only if a structural delta exists (unlikely here since AX-001/AX-002 pre-exist and cover this batch) — flips delta elements built, writes graded As-built/Drift on touched rows, runs the orphan check, regenerates `ARCHITECTURE.md`.
- **Baseline graded folds** for any `baseline-delta.md` written during design or at build-time technical decisions.
- **Map graduation** (selection scope): W1/W2 fold into FEAT-001's extent and vanish from pending; entry status → `delivered`, dated; `FEATURES.md` row updates; the spec reads closed once all its selected rows have folded.
- **Gates fold:** fix-now/backlog gap-finding results fold into `.mochiko/features/FEAT-001/gates.md` (minted if absent), authored in TEST grammar by `qa-engineer`.
- **Independent check:** the landing verification seat checks all graded folds (three-way diff) before acceptance is presented.
- **Write:** `entry.md` status update, `FEATURES.md`, `gates.md`, `ARCHITECTURE.md` (if touched), `data-model.md`/`contracts/`/`constraints-and-decisions.md` (only if a baseline delta existed to fold).

### Phase 12 — Final acceptance (the user's)
- **Gate:** accept / amend / reject.
  - *Accept* → run closes; verdict recorded against the done condition.
  - *Amend* → specific amendment applied and re-verified at delta scope, then re-presented for acceptance.
  - *Reject* → run does not close; disposition (rework, escalate, or abandon) is the user's; state persists for resumption.

### Phase 13 — Close
- **Done:** State the verdict against the fixed done condition and check the 15 fail-conditions in `impl.sec.fail-conditions`; if none stand, the run is DONE. If any stand (e.g. an unresolved regression, a card left unchecked, acceptance never given), state the run as explicitly NOT DONE and name the standing clause(s).
- **Write:** none beyond what already landed in Phase 11; the close statement itself is conversational, per the run's reporting register (`templates/output-style.md`).