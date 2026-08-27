# Action Plan — `/mochiko:implement FEAT-001` (notely: "Note capture")

*Plan-only. No agents dispatched, no files written, no user input awaited — gates below are described, not executed.*

## Repo state found during research

- **FEAT-001** exists on the feature map (`FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`), status `selected`, two selected work rows: **W1 — Create a note** (US-001, SC-001/SC-002) and **W2 — Fetch a note by id** (US-002, SC-003). Scope source is the spec's accepted selection (ratified 2026-08-20) → this is a **selection-scope** run, not an epic, not a delta card. No dependencies to block on.
- Baselines are present and look ruled: `spec.md` (US-001/002, FR-001..004, SC-001..003), architecture spine (`api-service`, `notes-db`, AX-001 persistence, AX-002 logging, AX-003 auth n/a), `data-model.md` (Note entity), `constraints-and-decisions.md` (C-001, D-001 SQLite, D-002 stdlib HTTP), `contracts/api.yaml` (POST /notes, GET /notes/{id}).
- No product code exists anywhere in the tree — this is a **greenfield build**, first capability on the map.
- `.mochiko/memory/` does not exist → no `codebase-analysis.md`, no `governance-intent.md`, no `knowledge-management.md`. `.claude/rules/mochiko/` does not exist → no governance region.
- No `.mochiko/features/FEAT-001/` artifacts beyond `entry.md` (no `sufficiency-report.md`, no `tasks.md`, no prior cycle reports) — clean first run.
- **The working directory is not a git repository.** This is a concrete blocker for the final-validation cold-verification snapshot step, which requires `git ls-files -co --exclude-standard`, and for the run's ordinary "suggest commits" behavior. This must be surfaced as an explicit trip at run-open.

---

## Phase 0 — Load the binding schema

**Does:** Read `plugins/mochiko/schemas/implement.yaml` raw and whole (already done above), plus `plugins/mochiko/schemas/command-labels.yaml` for the label vocabulary. Substitute `${var}` placeholders: `attempt_bound_cycle=3`, `gap_rework_bound=2`, `builder_seat=staff-engineer`, `design_seat=technical-analyst`, `architect_seat=principal-architect`, `qa_seat=qa-engineer`, `gap_finder_seat=devils-advocate`, `explore_model=haiku`, `features_dir=.mochiko/features`, `product_dir=.mochiko/product`, `epics_dir=.mochiko/epics`, `rules_dir=.claude/rules/mochiko`, `tasks_schema=plugins/mochiko/schemas/tasks.yaml`.
**Reads:** the two schema files.
**Writes:** none.
**Gate:** none — this is a precondition, not a checkpoint.

## Phase 1 — Entry resolution

**Does:** Resolve `FEAT-001` as a capability ID (not empty, not `EPIC-XXX`) against `FEATURES.md`. Confirm its work rows (W1, W2) carry ratified scope from the spec's accepted selection, and that no selected row depends on an undelivered row (Dependencies: None, confirmed).
**Reads:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/specs/note-capture/spec.md`.
**Writes:** none yet.
**Seats/skills:** none — DM-level routing.
**Gate:** none.

## Phase 2 — Sufficiency check

**Does:** Dispatch the sufficiency skill (`mochiko:review-sufficiency`) to grade each selected row (W1, W2) against the spec, the architecture store, and the product baselines, using a seat that authored none of those three sources. Because `technical-analyst` and `principal-architect` are the likely authors of the baselines/store, propose a seat outside that authorship line — e.g. **validator** — as the DM's staffing call. The check also grades the absent-surfaces branch: missing `codebase-analysis.md` (greenfield → proceed, warning logged, `/mochiko:setup` offered as an option), missing governance region (surfaced, not blocking), no store trips found in the spine as read.
**Reads:** `spec.md`, `.mochiko/product/architecture/spine.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`.
**Writes:** `.mochiko/features/FEAT-001/sufficiency-report.md` — per-row verdict (`sufficient` or a gap list), any trips, the codebase-analysis null-path note.
**Seats:** validator (proposed; independent of spec/store/baseline authorship).
**Gate:** none directly — the verdict feeds the run-open confirmation next. A disputed clause inside the check itself defaults to "gap" and is deferred to the run-open gate rather than resolved by the grading seat alone.

## Phase 3 — Run-open confirmation (USER GATE)

**Does:** Present one confirmation, no negotiation:
- Batch identity: FEAT-001 "Note capture", selection scope, work rows W1 + W2.
- Attempt bounds restated at their only redeclaration point: 3 verification attempts per cycle, 2 gap-rework rounds at run scope — offer the user the chance to redeclare either now.
- The Phase 2 sufficiency verdict and any gap list / gap routing.
- Trips and conflicts for the user's ruling: (a) missing governance region, (b) missing `codebase-analysis.md` on a greenfield tree, (c) **the workspace is not a git repository**, which blocks the cold-verification snapshot step later in the run and the run's usual "suggest a commit" behavior.
- The done condition: every cycle card checked off, built test-first, independently verified against real infrastructure per cycle and once whole, code traced to FR/SC criteria, acceptance landing executed whole, run closes on accept/amend/reject.

**Reads:** nothing new — synthesizes Phases 1–2.
**Writes:** nothing yet (the confirmation itself is not a file write).
**Gate — what's confirmed:** the four bullets above, as one blocking plain-text ruling.
**Onward branches:**
- **Confirm as-is, defaults kept:** proceed to Phase 4 (or Phase 5 if Phase 2 found zero gaps).
- **Redeclare an attempt bound:** carry the new bound forward through the rest of the run; no further redeclaration point exists.
- **Rule on the git-repo trip — accept "init now":** the DM would surface a light confirmation to run `git init` (a low-risk but real workspace mutation, so it gets its own explicit ask) before build starts, so the cold-verification snapshot has a ref to diff against later; if the user instead says "skip cold verification" or "we'll init later," that constraint is carried forward and the final-validation phase's cold-verification step is deferred/blocked accordingly, to be re-raised at that checkpoint.
- **Rule that scope is wrong / misrouted:** run aborts this invocation and routes to `/mochiko:specify` (new capability) or `/mochiko:feature` (delta card) as appropriate — not applicable here since FEAT-001 is a legitimate, ratified selection, but stated for completeness.
- **Reject / stop here:** run does not proceed past entry; nothing further is written.

## Phase 4 — Design phase (fires only if Phase 2 named a gap)

**Does:** For each named gap, and *only* those gaps, dispatch design-class seats on a DM-approved plan each: `technical-analyst` for design deltas (e.g., an addition to `constraints-and-decisions.md` or `data-model.md` if the sufficiency check found a modeling or decision gap), `principal-architect` only if a structural/store delta is triggered (unlikely here — AX-001/AX-002/AX-003 already cover persistence, logging, and auth-n/a for this feature's extent). Given the baselines read in research already look complete against FR-001..004/SC-001..003, this phase may legitimately find **zero gaps** — in which case it is skipped entirely and Phase 5 carries the zero-gap map assertion instead.
**Reads (if it fires):** `sufficiency-report.md`, existing baselines, `spec.md`.
**Writes (if it fires):** design deltas beside their baselines under `.mochiko/features/FEAT-001/` (e.g. `data-model.md` delta, `contracts/` delta, before/after prose deltas for `constraints-and-decisions.md`), plus a store delta at `.mochiko/product/architecture/` only if a structural trigger fired; the design phase also asserts sharpened dependencies/extent onto `entry.md` with provenance.
**Seats:** technical-analyst / principal-architect as authors; a non-author review pair grades the package before the checkpoint — proposed **tech-lead** for feasibility/buildability/contradiction, **validator** for conformance-to-gap-list and card-quality (this leg is blocking: material drift beyond the named gap list auto-fails the review).
**Gate — design checkpoint (USER):** presents the signed design deltas (and store delta table + provenance if any) for the user to sign before any code is written.
**Onward branches:**
- **Sign as-is:** proceed to Phase 5.
- **Amend:** design seat reworks within the gap-rework bound; re-reviewed by the same non-author pair; re-presented.
- **Stop here, resume later:** run pauses cleanly; a future `/mochiko:implement FEAT-001` invocation resumes from the signed (or still-pending) design state.

## Phase 5 — Cycle-card authoring

**Does:** A design-class, non-builder seat (proposed **qa-engineer**, matching its role as the seat that authors the `**TEST:**` cases within its slicing) authors `tasks.md` cycle cards from the tasks template (`mochiko-cli template tasks`, falling back to a raw read of `plugins/mochiko/schemas/tasks.yaml` if the binary is unavailable — not verified in this plan-only pass). Slicing follows vertical-TDD discipline: foundation cycle(s) before feature cycles. For this batch, a reasonable slice (subject to the authoring seat's own judgment) is: (1) a foundation cycle standing up the stdlib HTTP server + SQLite datastore per D-001/D-002, (2) a feature cycle for W1 (POST /notes, US-001, SC-001/SC-002), (3) a feature cycle for W2 (GET /notes/{id}, US-002, SC-003). Each card carries story/rationale, dependencies, acceptance-criteria IDs, a `**TEST:**` real-infrastructure gate, and brownfield exposure (all `[NEW]` — greenfield). If Phase 4 was skipped (zero gaps), this seat also makes the map-entry assertion the design phase would have made, and surfaces any intended-vs-designed drift at the confirm below.
**Reads:** `sufficiency-report.md`, any design deltas from Phase 4, `spec.md`, baselines.
**Writes:** `.mochiko/features/FEAT-001/tasks.md`.
**Seats:** qa-engineer (author); an independent verification seat (proposed **tech-lead**, distinct from the design-review pair's conformance leg or reused if judged fine) reviews the cards before confirm for quality and buildability.
**Gate:** feeds directly into Phase 6.

## Phase 6 — Card confirm (USER GATE)

**Does:** Present the sliced cycle cards for the user to rule on before any build begins.
**Gate — what's confirmed:** the cycle slicing itself (card count, order, foundation-first sequencing, acceptance-criteria mapping).
**Onward branches:**
- **Confirm as-is:** proceed to Phase 7.
- **Request resplit/reorder/merge:** card-authoring seat revises, re-reviewed, re-presented.
- **Reject / halt:** run pauses without building; state preserved for resumption.

## Phase 7 — Build (test-first, per cycle)

**Does:** For each confirmed card, in order (foundation cycle first):
- `staff-engineer` (builder_seat) plans the concrete task breakdown, on a DM-approved plan, disclosed in `cycle-report.md`.
- Executes red→green→refactor per task, binding `mochiko:executing-tdd-cycle`; `mochiko:brownfield-integration` does not apply (nothing pre-existing to touch); `mochiko:patterns-code-minimalism` rungs disclosed at decomposition (e.g., "stdlib HTTP" and "bundled SQLite driver" are already ratified decisions, not open choices, so the ladder mostly resolves to "use the ratified decision").
- Independent verification seat (proposed **qa-engineer**) runs `mochiko:testing-end-user` against real infrastructure — an actual running process and an actual SQLite file, never mocks — for the card's `**TEST:**` gate, plus the advisory `mochiko:review-code-minimalism` lens reading the diff, cycle report, and surrounding code.
- Attempt economy: 3 grading attempts per cycle (or as redeclared at Phase 3); two consecutive unchanged-findings rounds is a no-progress stop, halting that cycle and presenting state to the user.
- Any undesigned structural need discovered mid-build halts that cycle and re-fires a scoped Phase 4 design refire against the same checkpoint discipline; any deviation from a signed store delta (box/arrow/responsibility change) halts and is presented rather than silently worked around.
- Reserved-to-user questions (ambiguity, adopt-first calls, IP-XXX calls, scope escalation) batch at the cycle checkpoint unless build-blocking, in which case they interrupt immediately.
**Reads:** the card under build, baselines, any design deltas.
**Writes:** application source code (paths not yet known — first code in the repo), `.mochiko/features/FEAT-001/cycle-report-*.md` per cycle, `tasks.md` checkbox flips as each card completes.
**Seats:** staff-engineer (build), qa-engineer (per-cycle verify).
**Gate:** none per-cycle beyond the attempt-economy halt condition (which itself routes to the user only on exhaustion/no-progress).

## Phase 8 — Final validation (whole-build)

**Does, once all cards are `[x]`:**
- Runs the full repository quality-gate suite (exact commands depend on what Phase 7 establishes — not yet known).
- **Cold verification:** builds and runs gates from a dependency-cold snapshot of the uncommitted working tree, copied via `git ls-files -co --exclude-standard :!.claude/worktrees` into `.claude/worktrees/mochiko-notes/`. **This step is blocked until the git-repo trip from Phase 3 is resolved** (either the user approved a `git init` earlier, or this step is explicitly re-raised here as still-blocked and the run halts pending the user's ruling).
- Regression sweep over previously delivered features' durable gate sets — vacuously empty here, since FEAT-001 is the first capability and no `gates.md` files exist yet elsewhere.
- **Gap-finding pass** (required — this is selection scope, not delta/lane): a fresh, blind `devils-advocate` — never used elsewhere in this run's staffing — dispatched in two messages. First message carries only `spec.md`, `sufficiency-report.md`, any design deltas, and the baselines' NFR-XXX rows (AX-001 NFR-001 restart-survival, AX-002 NFR-002 4xx/5xx reason field) — never code, `tasks.md`, `**TEST:**` cases, or reports. The seat states derived expectations before probing the running system.
- Mutation lens runs only if verification depth is judged "high" (a DM call); otherwise explicitly stated as skipped rather than silently omitted.
- Findings split: spec-required-behavior-broken fails final validation (evidence + clause cited); beyond-spec findings are advisory, disposed by the user (fix now / BACKLOG.md / accept as designed).
- Gap-rework: up to 2 rounds at run scope (or as redeclared), localized findings charging their originating cycle's remaining attempts instead.
**Reads:** running system, all cycle reports, baselines, spec.
**Writes:** `.mochiko/features/FEAT-001/final-validation-report.md`, gap-finding findings recorded therein.
**Seats:** devils-advocate (gap-finding, fresh/blind); verification seat(s) from Phase 7 for the whole-build gate run.
**Gate:** none directly — feeds Phase 10's acceptance presentation; a build-blocking finding could still interrupt per the escalation-batching rule.

## Phase 9 — Landing preparation (executed only inside acceptance, not before)

**Does (drafted/staged conceptually here, executed atomically at Phase 10 accept):**
- If any structural change was built, a store landing: flip built status on the relevant store elements, clear their FEAT-XXX keys, write graded As-built/Drift fields, run the orphan check, regenerate `ARCHITECTURE.md` from the store (never hand-edited).
- Any baseline touched during build (e.g., a build-time D-XXX/C-XXX/IP-XXX decision, or a data-model/contract adjustment) folds via a graded three-way diff, checked by an independent landing-verification seat (proposed **validator**).
- Selection-scope landing: W1/W2 fold into FEAT-001's extent lines and vanish from the pending row list in `entry.md`; status set to `delivered`, dated; `FEATURES.md` index line updates; the note-capture spec's index entry reads closed once both its selected rows have folded.
- Gate findings ruled fix-now or backlog fold into `.mochiko/features/FEAT-001/gates.md` (minted, since it doesn't yet exist), authored by qa-engineer in the `**TEST:**` grammar.
- KM landing: not applicable — no `.mochiko/memory/knowledge-management.md` exists.
**Writes (at accept only):** `entry.md`, `FEATURES.md`, `ARCHITECTURE.md` (if store touched), `.mochiko/features/FEAT-001/gates.md`, any folded baselines.
**Seats:** validator (landing verification/grading).

## Phase 10 — Final acceptance (USER GATE)

**Does:** Presents the final-validation report, all cycle reports, the proposed landing (Phase 9 contents) as one whole package.
**Gate — what's confirmed:** accept / amend / reject of the entire batch's delivered state and the landing it would execute.
**Onward branches:**
- **Accept:** DM executes the Phase 9 landing whole, in one pass; closes the run with a verdict against the done condition; suggests (never runs) a commit — and since no git repo exists, the DM's suggestion would need to lead with "initialize a git repository, then commit" rather than assuming one.
- **Amend:** scoped rework routes back to the relevant Phase 7 cycle or Phase 8 finding, consuming gap-rework/attempt-economy budget as applicable; re-presented at this same gate afterward.
- **Reject:** run closes without landing; nothing folds; state is preserved (cards, reports, code) for a future invocation rather than discarded.

## Phase 11 — Run close

**Does:** Checks the run's outcome against the 15 fail-condition clauses in the schema's `impl.sec.fail-conditions` section (sufficiency unrecorded, design skipped/unsigned, card independence violated, card left unchecked, failing quality gate, unevidenced verification, regression, baseline edited in place, unresolved deviation, incomplete store landing, ungraded fold, missing gap-finding pass, unstated skip, unresolved spec-gap finding, missing user acceptance). Confirms the schema's fail-condition count is exactly 15 (verified during this research pass — it is) before declaring a verdict; a mismatch would halt and surface rather than close silently. Reports a verdict: done (all cycles `[x]`, verified per-cycle and whole, traced, landed, accepted) or not-done with the specific standing clause(s) named.
**Writes:** the closing verdict as part of the acceptance/close report (already covered by Phase 8/10's report paths — no separate file).
**Gate:** none — this is the terminal report following Phase 10's ruling.