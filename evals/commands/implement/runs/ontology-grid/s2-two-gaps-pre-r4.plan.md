## Action Plan — `/mochiko:implement FEAT-002` (plan-only, not executed)

**Grounding read already performed for this plan** (the run's own first-action rule): `plugins/mochiko/schemas/implement.yaml` read raw and whole — 15 fail-condition rules confirmed, matching the command's hard-coded count, so the schema/`.md` pair is in sync. Also read for concreteness: `.mochiko/features/FEAT-002/entry.md`, `FEATURES.md`, `.mochiko/specs/note-search/spec.md`, `.mochiko/product/{data-model.md, constraints-and-decisions.md, architecture/spine.md, contracts/api.yaml}`, `ARCHITECTURE.md`, `.mochiko/features/FEAT-001/{entry.md, gates.md}`. `.mochiko/memory/` and `.claude/rules/mochiko/` are both absent from the tree.

---

### Phase 0 — Load binding rules
- **Done:** Full raw read of `implement.yaml` (already satisfied above); labels cross-checked against `plugins/mochiko/schemas/command-labels.yaml`.
- **Read:** `plugins/mochiko/schemas/implement.yaml`, `plugins/mochiko/schemas/command-labels.yaml`.
- **Written:** nothing.
- **Seats/skills:** none — Delivery Manager only.
- **Gate:** none.

### Phase 1 — Resolve entry and scope
- **Done:** `FEAT-002` parsed as a plain capability ID (not `EPIC-XXX`, not empty — no epic-lookup or "propose next ready capability" branch needed). Entry confirmed selection scope: the map row's status is `selected`, work rows W1/W2 carry ratified scope sourced from the spec's accepted selection (dated 2026-08-26), not a delta card, so no `/mochiko:feature` desk-confirmed-card path applies. Dependency check: FEAT-001 is `delivered`, so nothing blocks batch ordering.
- **Read:** `.mochiko/features/FEAT-002/entry.md`, `FEATURES.md`, `.mochiko/specs/note-search/spec.md`, `.mochiko/features/FEAT-001/entry.md`.
- **Written:** nothing yet.
- **Seats/skills:** none.
- **Gate:** none (routing only fires if entry were absent — it isn't).

### Phase 2 — Sufficiency check
- **Done:** Per-row (W1, W2), ten-clause sufficiency grading over spec + architecture store + product baselines, run by a seat that authored none of those sources and will not design or build this batch. Staffing call: qa-engineer, since technical-analyst (baselines) and principal-architect (store spine) are disqualified by authorship, and staff-engineer is disqualified as builder regardless of authorship.
  Based on the baselines as currently read, the check has visible material to surface as gaps (subject to the grading seat's actual verdict, not asserted here as fact):
  - The architecture spine states explicitly "no queues, no background workers," while spec FR-103 requires a background index worker — a direct store/spec conflict, not just a missing row.
  - No AX concern row or NFR-XXX target exists for search latency (SC-103's ≤2s bound).
  - No D-XXX technology decision exists for the indexing/search mechanism (a commodity-category choice, e.g. SQLite FTS5 vs. hand-rolled).
  - `contracts/api.yaml` has no `/notes/search` endpoint.
  - `data-model.md` has no search-index entity/attribute.
  - `.mochiko/memory/codebase-analysis.md` is absent, and no source tree is visible despite FEAT-001 reading `delivered` — an absent-surface condition to surface, not resolve.
  - No governance region (`.claude/rules/mochiko/`) exists — surfaced, never run-failing.
  Any disputed clause defaults to gap and goes to the user, never cleared by the grader alone.
- **Read:** `note-search/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/contracts/api.yaml`, `FEAT-002/entry.md`.
- **Written:** `.mochiko/features/FEAT-002/sufficiency-report.md` — per-row verdict, store-consult result, trips for run-open, quickstart.md null-path note (no external-integration surface here), any absent-surface findings.
- **Seats/skills:** qa-engineer, bound to `mochiko:review-sufficiency`.
- **Gate:** none directly — its output feeds the run-open gate next.

### Phase 3 — Run-open confirmation (USER GATE)
- **Done:** One confirmation, no negotiation. States: the batch (FEAT-002, selection scope, rows W1/W2) · both attempt bounds at their only redeclaration point (per-cycle default 3, gap-rework default 2 — user may redeclare here only) · the sufficiency verdict and its gap routing · every store trip (the "no background workers" conflict) and any conflict the grader could not clear, each reserved to the user · the absent-codebase-analysis and absent-governance-region surfacing (offer `/mochiko:setup` or proceed greenfield with a logged warning) · the done condition stated plainly.
- **Read:** sufficiency-report.md (just written).
- **Written:** nothing new; the report is referenced, not restated.
- **Seats/skills:** none — Delivery Manager presents, user rules.
- **Gate — what's confirmed:** batch/scope identity, attempt bounds (accept defaults or redeclare), disposition on each trip/conflict, and the absent-surface offers.
  - **If the user rules "fix the conflict via design"** → proceed to Phase 4 (design phase fires, scoped to the named gaps).
  - **If the user rules to defer a trip on the record** → that trip is logged as deferred, not resolved; run continues but the deferred item stays open for later ruling before it would block landing.
  - **If the user stops here** → run pauses at this checkpoint; resumable later, nothing built.

### Phase 4 — Design phase (fires — gaps were named)
- **Done:** Design seats author exactly the named gaps, nothing more, each on a plan the DM approved, rung-justified per the plan-minimalism ladder. Staffing: technical-analyst for the design deltas (data-model delta, `/notes/search` contract delta, the D-XXX indexing-technology decision — routes through the adopt-first commodity check, which if it needs a build vs. off-the-shelf ruling on indexing tech halts to the user rather than being builder-decided); principal-architect for the store delta (new AX row for the index/background-worker concern, the topology and flow change, an NFR-XXX search-latency target) — this is a genuine structural addition (new box/arrow) drafted against the store, following the diagram-and-register discipline, never merged into the ruled baseline directly. First duty check: is a baseline seed needed anywhere? No — baselines already exist and are populated, so no empty-scaffold seeding applies here.
- **Read:** sufficiency-report.md, `spec.md`, current `data-model.md`, `constraints-and-decisions.md`, `architecture/spine.md`, `contracts/api.yaml`.
- **Written (deltas beside baselines, never in place):** `.mochiko/features/FEAT-002/data-model-delta.md` (or equivalent appliable before/after form), `.mochiko/features/FEAT-002/contracts/api-delta.yaml`, `.mochiko/features/FEAT-002/constraints-and-decisions-delta.md` (new D-XXX), and the store delta — an in-flight-class addition written directly onto the store's ruled content per the one legal carve (new AX row for the index worker, flow arrow to a background component), pending sign-off.
- **Seats/skills:** technical-analyst (`mochiko:authoring-technical-requirements`, `mochiko:patterns-technical-decisions`, `mochiko:patterns-adopt-first` if the indexing-tech question trips the commodity check), principal-architect (`mochiko:patterns-system-design`, `mochiko:authoring-architecture-store`), all under `mochiko:patterns-plan-minimalism` for rung discipline.
- **Gate:** none yet — review pair comes first.

### Phase 5 — Design-phase review pair (non-author)
- **Done:** Two independent, non-author grades before the checkpoint: conformance to the gap list and card-quality standard (blocking on material divergence), and a separate feasibility/contradiction/buildability pass over the same deltas including the store delta.
- **Read:** the Phase 4 deltas, sufficiency-report.md (as the scope-of-record).
- **Written:** review findings (feed the checkpoint presentation; not a standalone landing artifact).
- **Seats/skills:** a design-class seat not among the Phase 4 authors, bound to `mochiko:review-plan-artifacts` (conformance/completeness — blocking) and `mochiko:review-feasibility` (contradiction/buildability, including the architecture pass since a store delta exists).
- **Gate:** none directly — feeds Phase 6.

### Phase 6 — Design checkpoint (USER GATE)
- **Done:** Present the design deltas, the store delta as a rendered diagram plus its named AX-row changes (or source-plus-changed-element-table if no render surface), and the two reviews' verdicts.
- **Read:** Phase 4/5 outputs.
- **Written:** none new (the sign-off is a ruling, recorded against the already-written deltas).
- **Seats/skills:** none — presentation only.
- **Gate — what's confirmed:** whether the design and the store delta are correct as drafted.
  - **Sign as-is** → deltas and store delta are now the anchor for the deviation check and the eventual built-vs-signed diff; proceed to Phase 7.
  - **Amend** → back to Phase 4 scoped to the amendment, then re-review, then re-present here.
  - **Stop and resume later** → run pauses; nothing built; resumable directly at this checkpoint later.

### Phase 7 — Cycle-card authoring
- **Done:** A design-class seat (never the builder) slices W1/W2 into cycle cards from the now-signed design — foundation cycles before feature cycles (e.g., a foundation cycle standing up the index storage/background worker mechanism per the signed store delta, then a feature cycle wiring `GET /notes/search`). qa-engineer authors the `**TEST:**` real-infrastructure gate within its slicing, citing SC-101/SC-102/SC-103 by ID. Cards carry stories/rationale, dependencies, acceptance-criteria IDs, and brownfield exposure (this batch touches the existing `api-service` and `notes-db` — `[EXTEND]` classification) — no task lists, no file paths.
- **Read:** signed design deltas, signed store delta, `spec.md` (for cited acceptance-criteria IDs), `tasks.yaml` schema (rendered via `mochiko-cli template tasks`, or the schema Read raw if the binary is absent).
- **Written:** `.mochiko/features/FEAT-002/tasks.md` (cycle cards, unchecked).
- **Seats/skills:** technical-analyst or principal-architect as card-author (DM's staffing call), qa-engineer for `**TEST:**` cases, bound to `mochiko:patterns-vertical-tdd`.
- **Gate:** none directly — verification review comes first.

### Phase 8 — Card review (non-author, pre-confirm)
- **Done:** The verification seat (not the card author) reviews the cards for quality (per the same plan-artifacts standard) and buildability, its own judgment.
- **Read:** `tasks.md`.
- **Written:** review notes feeding Phase 9's presentation.
- **Seats/skills:** the verification seat slated for build-time grading (qa-engineer, distinct instance/role from the card author if that was also qa-engineer — staffing must preserve author/grader separation).
- **Gate:** none directly.

### Phase 9 — Card confirm (USER GATE)
- **Done:** Present the sliced cards and the review verdict.
- **Read:** `tasks.md`, Phase 8 review.
- **Written:** none new.
- **Seats/skills:** none — presentation only.
- **Gate — what's confirmed:** whether the slicing is correct before any build starts.
  - **Confirm as-is** → build begins at Phase 10.
  - **Request re-slice** → back to Phase 7, re-reviewed, re-presented here.
  - **Stop** → run pauses; resumable at this checkpoint.

### Phase 10 — Build (test-first, foundation before feature)
- **Done:** staff-engineer decomposes each confirmed card into concrete build-time tasks (disclosed in the cycle report), builds test-first, foundation cycle first. Touches to `api-service`/`notes-db` follow the whole-file-read-first, pattern-preserving discipline for existing code. Each task passes through the pre-code minimalism ladder before any red-phase test, disclosed by rung.
- **Read:** the confirmed card, the existing `api-service`/`notes-db` code in full before any edit to it, the signed design/store deltas as the source of truth for what's being built.
- **Written:** production and test code for the cycle; `.mochiko/features/FEAT-002/cycle-report-<n>.md` per cycle (decomposition, honest difficulties, deviations, any `domain_deps_added`).
- **Seats/skills:** staff-engineer, bound to `mochiko:executing-tdd-cycle`, `mochiko:brownfield-integration`, `mochiko:patterns-code-minimalism`.
- **Gate:** none directly (build-blocking escalations — ambiguity, an infeasible card, an adopt-first/IP-XXX call, a mid-signed-delta deviation — interrupt immediately rather than waiting for a checkpoint; everything else batches to the next checkpoint).

### Phase 11 — Per-cycle verification
- **Done:** A verification seat that did not build the cycle runs its `**TEST:**` gate against real infrastructure (an actual SQLite file and actual HTTP server, never mocks) and grades the code-minimalism lens by reading the diff, the cycle report, and the surrounding codebase. Attempt economy: each grading pass consumes one of the 3-per-cycle budget; two consecutive rounds with unchanged findings halts the cycle and presents state rather than continuing to spend attempts.
- **Read:** the cycle's diff, `cycle-report-<n>.md`, running service/database state as evidence.
- **Written:** `.mochiko/features/FEAT-002/verification-report-<n>.md` (evidence captured, never assumed) with an advisory `minimalism:` findings block.
- **Seats/skills:** qa-engineer, bound to `mochiko:testing-end-user`, `mochiko:review-code-minimalism`.
- **Gate:** none directly per cycle; a failed gate fails that cycle outright (never severity-triaged) and reworks within the attempt bound; unresolved exhaustion or no-progress escalates to the user.

*(Phases 10–11 repeat per cycle until every card is `[x]`.)*

### Phase 12 — Regression sweep
- **Done:** Because FEAT-002 builds inside FEAT-001's territory (`api-service`, `notes-db`) and is the later-landing feature on that seam, its final validation additionally re-runs FEAT-001's durable gate set.
- **Read:** `.mochiko/features/FEAT-001/gates.md` (3 `**TEST:**` cases: restart-survival, empty-body rejection, get-by-id/404).
- **Written:** results fold into the final-validation report (Phase 14).
- **Seats/skills:** same verification seat, real-infrastructure execution.
- **Gate:** a regression here fails the run outright — not severity-triaged.

### Phase 13 — Gap-finding pass (fires — selection-scope run)
- **Done:** A fresh devils-advocate instance, never having built these cycles or seen the design-time `**TEST:**` cases, runs a blind, two-message dispatch: first message carries only `spec.md`, `sufficiency-report.md`, the signed design deltas, and the baselines (`data-model.md`, `contracts/`, the store's NFR-XXX rows) — never code, `tasks.md`, `**TEST:**` cases, or any report. It states derived expectations before probing begins. If the verification seat ran at high depth, the mutation lens result (or an explicit stated skip) accompanies this.
- **Read:** as above, by the blind seat; nothing beyond that fence, including in any delegated sub-reads.
- **Written:** gap-finding findings feeding the final-validation report; each finding tagged spec-required (blocking) vs. beyond-spec (advisory), with disputed kind defaulting advisory and going to the user.
- **Seats/skills:** devils-advocate, bound to `mochiko:testing-gap-finding`.
- **Gate:** none directly — findings triage happens with the user at the checkpoint that follows (a spec-required finding is build-blocking and interrupts immediately rather than waiting).

### Phase 14 — Final validation
- **Done:** Builds and runs the full quality-gate suite from a dependency-cold snapshot of the uncommitted working tree (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-note-search/`, confirming the `/.claude/worktrees` gitignore entry first). Assembles: quality-gate results, the Phase 12 regression sweep, the Phase 13 gap-finding results (with a stated skip if this had been delta/lane scope — it isn't), any spec-gap findings left unresolved (auto-FAIL condition if so).
- **Read:** cold snapshot of the working tree; Phases 11–13 outputs.
- **Written:** `.mochiko/features/FEAT-002/final-validation-report.md`, the built-vs-signed diff against the Phase 6 store delta.
- **Seats/skills:** the verification seat, real-infrastructure execution throughout.
- **Gate:** none directly — feeds the acceptance landing and final acceptance.

### Phase 15 — Acceptance landing (executed whole)
- **Done:** All in one pass: the store delta's elements flip to built, `As-built:`/`Drift:` fields written as judgment and independently graded, orphan check run over the store; each touched baseline (`data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`) folds exactly once via a graded three-way diff (pre-fold + delta vs. folded result — landing verification seat checks it, never the DM's own say-so); W1/W2 fold into FEAT-002's extent and vanish as pending rows; FEAT-002 status set `delivered`, dated; `FEATURES.md` index line updates; the note-search spec's closure derives from all its selected rows having folded; `ARCHITECTURE.md` regenerated from the store (never hand-edited); `.mochiko/features/FEAT-002/gates.md` minted, qa-engineer authoring any fix-now/backlog gap finding in `**TEST:**` grammar; no knowledge-management landing since `.mochiko/memory/knowledge-management.md` doesn't exist.
- **Read:** all Phase 4–14 artifacts as the landing's input set.
- **Written:** `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/architecture/spine.md`, `ARCHITECTURE.md`, `FEATURES.md`, `.mochiko/features/FEAT-002/entry.md`, `.mochiko/features/FEAT-002/gates.md`.
- **Seats/skills:** landing verification seat (folds + map-delta boundary check), bound to `mochiko:authoring-architecture-store`, `mochiko:authoring-feature-map`.
- **Gate:** the landing executes whole only once Phase 16 accepts — sequenced here for plan legibility, but gated on the acceptance ruling below.

### Phase 16 — Final acceptance (USER GATE)
- **Done:** Present the complete package: final-validation report, regression results, gap-finding disposition (each beyond-spec finding needing its own fix-now/backlog/accept-as-designed ruling), the landing about to execute.
- **Read:** everything assembled through Phase 15.
- **Written:** none new at this point — the ruling triggers Phase 15's writes.
- **Seats/skills:** none — presentation only.
- **Gate — what's confirmed:** accept / amend / reject, closing the run.
  - **Accept** → Phase 15 landing executes whole; run closes with a done-condition verdict.
  - **Amend** → the specific amendment routes back to the phase it belongs to (a code fix reopens Phase 10/11; a design correction reopens Phase 4); re-validated before returning here.
  - **Reject** → run closes without landing; state is preserved for a future resumed run.

### Phase 17 — Close
- **Done:** Verdict stated against the fixed done condition; the 15 fail-condition clauses checked explicitly (sufficiency recorded, design signed if gaps fired, card independence and full-checked state, gates clean, evidence real, no regression, no in-place baseline edit, no unresolved deviation, complete store landing, graded folds, gap-finding present, any skip stated, no unresolved spec-gap, acceptance given); rounds consumed and seats spawned surfaced in the closing report.
- **Read:** the full run's artifact trail.
- **Written:** closing summary (part of the final-validation/acceptance record, no new file class beyond what Phase 14–15 already wrote).
- **Seats/skills:** none — Delivery Manager closes.
- **Gate:** none — this is the terminal state reached only via the Phase 16 "Accept" branch (or a "Reject"/pause branch, which closes the run in a not-done state instead).