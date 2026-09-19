# Action Plan — `/mochiko:implement FEAT-001` (plan-only; nothing executed beyond reads)

**Grounding reads already performed for this plan:** `plugins/mochiko/schemas/implement.yaml`, `plugins/mochiko/schemas/common.yaml`, `plugins/mochiko/schemas/command-labels.yaml`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/specs/note-capture/spec.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/architecture/spine.md`. Confirmed: FEAT-001 is a plain capability (not `EPIC-XXX`), entry status `selected`, two work rows W1/W2, selection-scope source (spec's accepted selection, 2026-08-20), no dependencies. No `CLAUDE.md`, no `.mochiko/memory/codebase-analysis.md`, no `src/`, no top-level `FEATURES.md` found in this workspace — greenfield, and the map index's location needs confirming before landing.

The 15-entry fail-condition set in `impl.sec.fail-conditions` was counted against the schema read: 15 rules present, in sync with the command's Not-done line.

---

## Phase 1 — Entry resolution

**Does:** Confirms `FEAT-001` names a capability entry (not an epic), reads its ratified scope, and checks dependency order.
**Reads:** `.mochiko/features/FEAT-001/entry.md` (Status: selected; W1 — create a note, W2 — fetch a note by id; Dependencies: none); `.mochiko/specs/note-capture/spec.md` (Status: accepted, selection ratified 2026-08-20).
**Writes:** none.
**Seats/skills:** none spawned yet — this is the Delivery Manager's own read.
**Gate:** none. Entry is honest: selection scope, no blocked dependency, nothing to route to `/mochiko:specify` or `/mochiko:feature`.

## Phase 2 — Sufficiency check

**Does:** Grades each selected work row (W1, W2) against the ten-clause sufficiency checklist, per `mochiko:review-sufficiency`, run by a seat that authored none of the spec, store, or baselines (i.e., not `requirements-analyst`, not `technical-analyst`/`principal-architect` who wrote the baselines — a fresh grading seat, e.g. `validator` or `tech-lead`, exempt from plan approval like all grading seats).
**Reads:** `spec.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/architecture/spine.md` (AX-001 persistence, AX-002 logging, both `ruled` and already keyed with NFR targets graded by FEAT-001's own SC-001). Also checks for absent surfaces: governance region (`.claude/rules/mochiko/` / `CLAUDE.md` — absent, to be surfaced, never auto-resolved, never fails the run), `.mochiko/memory/codebase-analysis.md` (absent — greenfield, offer `/mochiko:setup` or proceed with warning logged), architecture store ruled content (present).
**Writes:** `.mochiko/features/FEAT-001/sufficiency-report.md` — binding per-row verdict (sufficient, or the exact gap list), the store-consult result, any no-delta claim, the trips carried to run-open, the quickstart.md null-path note (no external-integration surface here), any `[MODIFY]` amendment (none — first capability on the map).
**Seats:** one independent grading seat.
**Gate:** none blocking here — a *disputed* clause (grader can't clear alone) defaults to gap and is queued for the run-open gate, not resolved here.

## Phase 3 — Run-open confirmation (the entry gate)

**Does:** One blocking confirmation, no negotiation. States: the batch (FEAT-001, "Note capture," rows W1+W2) and scope type (selection scope, not epic, not delta); both attempt bounds at their only redeclaration point — 3 verification attempts per cycle, 2 gap-rework rounds at run scope; the sufficiency verdict from Phase 2 and its gap routing; any store trips or in-flight conflicts (none currently visible — AX-001/AX-002 are `ruled`, not in-flight) and absent-surface notices (no governance region, no codebase-analysis.md) for the user's ruling; the done condition (all cycle cards `[x]`, test-first, independently verified against real infrastructure per-cycle and whole, criteria traced, governance aligned, acceptance landing executed whole, closed accept/amend/reject).
**Writes:** none (the confirmation is recorded in the run's reporting, not a new file).
**Gate — user's ruling, described in full:**
- **Confirm as-is:** attempt bounds stand at 3/2; run proceeds to Phase 4 branch per the Phase-2 verdict.
- **Redeclare an attempt bound:** e.g. raise gap-rework to 3 rounds — new bound applies for the rest of this run only, recorded at this gate (its only redeclaration point).
- **Rule a store trip or conflict:** any surfaced trip is decided here or explicitly deferred on the record; a deferred trip carries forward as an open item to the design or landing checkpoint where it becomes actionable.
- **Rule on an absent surface:** e.g. accept greenfield with the codebase-analysis.md warning logged and proceed, or ask for `/mochiko:setup` to run first — if the latter, this run pauses/exits pending that setup.
- **Reject / stop:** run does not open; no further phases execute.

## Phase 4 — Branch on the sufficiency verdict

Two mutually exclusive continuations, selected by the Phase 2 verdict as ruled at Phase 3:

### Phase 4a — Design phase (fires only if any row carries a gap)

**Does:** Authors exactly the named gaps, nothing more, each on a Delivery-Manager-approved plan, rung-justified per `mochiko:patterns-plan-minimalism`.
**Reads:** `sufficiency-report.md`'s gap list, existing baselines to delta against (`data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `architecture/spine.md`), `spec.md` for cited acceptance criteria.
**Writes:** deltas beside baselines at `.mochiko/features/FEAT-001/` — e.g. `data-model-delta.md`, `contracts/api-delta.yaml`, or a prose before/after delta — plus an architecture store delta (in-flight-class elements only) if a structural trigger fired, following `templates/artifact-format.md`. If any baseline is wholly absent, the phase's first duty is an empty, stated seed at `.mochiko/product/`. The design phase also asserts design-implied dependencies/extent onto `.mochiko/features/FEAT-001/entry.md` with provenance (map-entry assertion), per `mochiko:authoring-feature-map`.
**Seats:** `technical-analyst` (design deltas) typically, `principal-architect` if a store delta is needed, `qa-engineer` for the `**TEST:**` case shapes — staffing is the Delivery Manager's call. `staff-engineer` never designs its own gaps.
**Review pair (non-author, before checkpoint):** `mochiko:review-plan-artifacts` (conformance to the gap list, card-quality coverage — blocking) and `mochiko:review-feasibility` (buildability/contradiction, plus the architecture pass if a store delta exists).
**Gate — design checkpoint (user's), described in full:**
- **Sign the design and store delta:** the delta package (source diagram/table if no render surface) is accepted; the store delta records as signed; run proceeds to Phase 5 (card authoring) using this delta as an added design input.
- **Sign design, reject store delta (or vice versa):** the accepted half proceeds; the rejected half returns to the design seat for revision under the same gap scope; re-review before re-presenting.
- **Stop at the checkpoint:** the user may pause the run here and resume the build later — no code is written before this sign-off in any case.
- **Reject outright:** design seat reworks within the same named-gap scope (never expands scope silently); re-enters the review pair.

### Phase 4b — Zero-gap path (fires only if every row is graded sufficient)

**Does:** Skips design entirely. The card-authoring seat (Phase 5) makes the map-entry assertion the design phase would have made, and any intended-vs-designed drift is surfaced at the card confirm instead of a separate checkpoint.
**Writes:** none in this phase — folded into Phase 5's card authoring.

*(Given what Phase 2's baseline reads show — AX-001/AX-002 already `ruled` and directly keyed to FEAT-001's SC-001, data model and API contract already covering both W1 and W2, D-001/D-002 already decided — this batch looks well-positioned for 4b, but that verdict is the grading seat's call in Phase 2, not asserted here.)*

## Phase 5 — Cycle-card authoring

**Does:** A design-class seat (never `staff-engineer`, the builder) slices the batch into cycle cards, foundation cycles before feature cycles, per `mochiko:patterns-vertical-tdd`. For this batch, a plausible foundation-then-feature slicing is: (1) SQLite persistence + note schema foundation, (2) W1 create-note cycle (SC-001, SC-002), (3) W2 fetch-by-id cycle (SC-003) — but the exact bundling (Simple/Split/Merge) is the authoring seat's judgment, not predetermined here.
**Reads:** design deltas or sufficiency-report.md (per branch), `spec.md`'s acceptance criteria, `data-model.md`, `contracts/api.yaml`.
**Writes:** `.mochiko/features/FEAT-001/tasks.md` — cards rendered from the tasks template (`mochiko-cli template tasks`, or `plugins/mochiko/schemas/tasks.yaml` read raw if the binary is absent). Each card: stories/rationale, dependencies, acceptance criteria by ID, a `**TEST:**` real-infrastructure gate, brownfield exposure (none — greenfield here, so no `[EXTEND]`/`[MODIFY]` marks expected). `qa-engineer` authors the `**TEST:**` cases within its slicing.
**Seats:** the design-class authoring seat (e.g. `technical-analyst` or `principal-architect`, whichever carried the design), plus `qa-engineer` for TEST grammar.
**Review (independent, before confirm):** the verification seat (`qa-engineer`, since it didn't author the cards, or another non-author) reviews for quality (`mochiko:review-plan-artifacts`) and buildability.
**Gate — card confirm (user's), described in full:**
- **Approve the slicing as-is:** build proceeds card-by-card in the confirmed order.
- **Request re-slicing (e.g. merge/split a cycle):** cards return to the authoring seat, re-reviewed, re-presented — no card builds before this gate clears.
- **Flag a card as infeasible:** escalates as a business-level scope decision — user rules cut/rescope/accept-risk before build starts on that card.

## Phase 6 — Build cycles (test-first)

**Does:** For each confirmed card in order, `staff-engineer` decomposes it into concrete tasks (disclosed in the cycle report), builds test-first (red→green→refactor) per `mochiko:executing-tdd-cycle`, on a Delivery-Manager-approved plan. Runs the pre-code minimalism ladder at decomposition (`mochiko:patterns-code-minimalism`, rungs disclosed) — relevant here since D-001 (SQLite via bundled driver, adopt-first already ruled) and D-002 (stdlib HTTP, no framework) are already decided, so decomposition should not re-litigate those, only apply them. `mochiko:brownfield-integration` does not fire (no existing code to extend/modify — first capability, greenfield).
**Reads (per card):** the card itself, cited baselines/deltas, any governance rules dir (absent here — so no obligated-read binding fires for this batch unless a governance region appears mid-run).
**Writes (per card):** application code under whatever repo root houses the service (not yet created — this is the batch's first code); `.mochiko/features/FEAT-001/cycle-report.md` (or per-cycle dated entries) — decomposition, honest difficulties, deviations, `domain_deps_added`; `tasks.md` checkbox flips as the progress surface.
**Seats:** `staff-engineer` (builder); `qa-engineer` (or another non-implementer) as the per-cycle verification seat.
**Per-cycle verification:** runs against real infrastructure via `mochiko:testing-end-user` (never mocks) — for this feature, a real SQLite file and a real HTTP round-trip (POST /notes, GET /notes/{id}, restart-and-reread for SC-001, empty-body 400 for SC-002, missing-id 404 for SC-003) — plus the advisory `mochiko:review-code-minimalism` lens (reads diff + cycle report + surrounding code; never gates the cycle, always reported).
**Attempt economy:** each grading pass consumes one of the 3 per-cycle attempts; two consecutive rounds with unchanged findings triggers a no-progress stop (halt that cycle, present state to the user — not a silent retry).
**Gate (conditional, mid-cycle only):** a build-blocking reserved-to-user question (e.g. an architecture deviation — added/removed box, redirected arrow, boundary move) halts immediately rather than waiting for the batch checkpoint. Non-blocking reserved questions accumulate and land as one batch at the cycle checkpoint instead.
  - **Deviation ruling:** build as originally approved, or amend the signed delta first by the user's ruling — never silently designed around.
  - **Adopt-first / IP-XXX collision (not expected here given D-001/D-002 already ruled, but binding if a new commodity decision arises mid-build):** halts to the user's checkpoint; run continues elsewhere while that one decision pauses.

## Phase 7 — Final validation (whole-build verification)

**Does:** Runs once the last card is `[x]`.
1. **Quality gates:** full repository suite (build, lint, test) — exit-code checks.
2. **Cold verification:** a dependency-cold snapshot of the uncommitted working tree (`git ls-files -co --exclude-standard :!.claude/worktrees`) copied to `.claude/worktrees/mochiko-<purpose>/`; first ensures a `.claude/worktrees` ignore entry exists (none was found in this workspace — would be added if missing) before snapshotting.
3. **Regression sweep:** accumulated `**TEST:**` gates of previously delivered features in this territory — none exist yet (FEAT-001 is the map's first capability), so this sweep is vacuous; still logged, not silently skipped.
4. **Gap-finding pass** (fires — this is a selection-scope run): a fresh, blind `devils-advocate` seat that built nothing here and saw no test cases. Two-message dispatch: first message carries only `spec.md`, `sufficiency-report.md`, any design deltas, and the baselines (`data-model.md`, `contracts/`, the store's NFR-001/NFR-002 rows) — never code, `tasks.md`, `**TEST:**` cases, or reports; the seat states derived expectations before any probing begins. Its brief carries the model-tiering rule (`mochiko:patterns-model-tiering`: locate/enumerate reads to a `haiku`-model `Explore` subagent, interpretive reads stay on-seat).
5. **Mutation lens:** runs on the verification seat (already holding code sight) only at high depth; skips are disclosed, not silent.
**Reads:** everything above plus the full built codebase.
**Writes:** `.mochiko/features/FEAT-001/final-validation-report.md` (or dated equivalent), a built-vs-signed diff if a store delta was signed in Phase 4a.
**Seats:** verification seat (quality gates, mutation lens), `devils-advocate` (gap-finding, blind).
**Gate (conditional):** each gap-finding's kind (spec-required-broken vs beyond-spec) is confirmed at the checkpoint against its cited clause — a disputed kind defaults advisory, never gated alone by the finder.
  - **Spec-required behavior broken:** fails final validation; must be fixed within the gap-rework bound (2 rounds at run scope, or charged to the owning cycle's remaining attempts if the finding localizes there). Bound exhaustion or an unchanged-findings round halts the run and presents state — disposition is the user's.
  - **Beyond-spec finding:** disposition is the user's — fix now, book to `BACKLOG.md`, or accept as designed.

## Phase 8 — Acceptance landing (executed whole)

**Does:** Once validation clears (or the user accepts with known, ruled exceptions), executes the landing as one whole action, checked by a non-author landing verification seat.
- Store landing (if a delta was signed in 4a): three-part fold — delta elements flip `built`, `FEAT-001` key clears; touched rows' `As-built:`/`Drift:` fields written as judgment and independently graded; orphan check runs; derived `ARCHITECTURE.md` regenerated by the store skill (never hand-edited).
- Any `baseline-delta.md` entries (build-time D-XXX/C-XXX/IP-XXX decisions, if any arose in Phase 6) graded by the landing verification seat before acceptance; landing's three-way diff stays transcription-only.
- Map graduation batch: W1/W2 fold into FEAT-001's extent lines and vanish from the pending list; entry status → `delivered`, dated; the `FEATURES.md` index line updates (location to be confirmed — not found in this workspace snapshot, would need locating or seeding as part of this fold); the specs-index row updates, spec `note-capture` reads closed since both its rows fold.
- Gap findings ruled fix-now or backlog fold into `.mochiko/features/FEAT-001/gates.md` (minted here, since absent), `qa-engineer` authoring each in `**TEST:**` grammar; accepted-as-designed findings do not fold.
- KM obligations if `.mochiko/memory/knowledge-management.md` exists (not found — skipped, not required).
**Reads:** all graded artifacts from Phases 4–7.
**Writes:** `.mochiko/product/architecture/spine.md` (via store skill, not hand-edited), `.mochiko/features/FEAT-001/entry.md` (status flip), `FEATURES.md` index, `.mochiko/features/FEAT-001/gates.md`, `.mochiko/specs/note-capture/spec.md` (closed marker, derived).
**Seats:** landing verification seat (folds check), `mochiko:authoring-architecture-store` / `mochiko:authoring-feature-map` skills as the writing procedures.
**Gate — final acceptance (user's), described in full:**
- **Accept:** landing executes as described; run closes with a PASS verdict against the done condition (assuming no fail-condition stands).
- **Amend:** user specifies the change; targeted rework re-enters Phase 6/7 scoped to the amendment, then re-lands.
- **Reject:** landing does not execute; run closes without delivering; state is presented for the user's next move (rework, descope, or abandon this batch).

## Phase 9 — Close

**Does:** States rounds consumed and seats spawned across the run at this final checkpoint (surfaced continuously, summarized here); checks the run against all 15 `impl.sec.fail-conditions` rules (sufficiency recorded, design signed if gaps existed, card independence held, no card left unchecked, no failing quality gate, no unevidenced verification claim, no regression, no in-place baseline edit, no unresolved deviation, complete store landing, every fold graded, gap-finding pass present for this selection-scope run, no unstated skip, no unresolved spec-required gap, user acceptance given); closes with an explicit verdict (PASS/FAIL) against the done condition.
**Writes:** the run's closing verdict, appended to reporting (no new baseline writes beyond Phase 8).
**Gate:** none further — this phase only reports the outcome of the Phase 8 gate.

---

### Summary of every user gate in this run, at a glance
1. **Run-open confirmation** (Phase 3) — batch/scope, attempt bounds, sufficiency verdict routing, trips.
2. **Design checkpoint** (Phase 4a, conditional on gaps) — sign design + store delta, or stop/rework.
3. **Card confirm** (Phase 5) — approve slicing, re-slice, or escalate an infeasible card.
4. *(mid-build, conditional)* deviation / adopt-first / ambiguity escalations as they arise.
5. *(final validation, conditional)* disputed finding-kind rulings, beyond-spec disposition.
6. **Final acceptance** (Phase 8) — accept / amend / reject, closing the run.