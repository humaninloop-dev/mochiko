# Action Plan — `/mochiko:implement FEAT-002` (plan-only, not executed)

## Ground truth gathered before planning (reads performed)

- `plugins/mochiko/schemas/implement.yaml` (full, raw) and `common.yaml`, `command-labels.yaml` — binding rules for this run. Fail-condition count under `impl.sec.fail-conditions` = **15**, matching the `.md`'s hard-coded count, so no halt-and-surface is triggered on that check.
- `.mochiko/features/FEAT-002/entry.md` — capability "Note search," status **selected**, two work rows **W1** (search by query, US-101/SC-101/SC-102) and **W2** (index freshness, US-102/SC-103), both marked *selected*, source: "the spec's accepted selection, 2026-08-26" → **selection scope**, not delta scope.
- Dependency line: `FEAT-001 (delivered)`. FEAT-001's own entry confirms status **delivered (2026-08-22)** → the dependency-order block does not trip.
- `.mochiko/specs/note-search/spec.md` — accepted 2026-08-26, no UX surface (HTTP API only, no Screens & Flows). FR-103 requires "a background index worker" that "rebuilds the index off the request path."
- `.mochiko/product/architecture/spine.md` — ruled topology states "Synchronous request/response only; no queues, no background workers." **This directly contradicts FR-103** — a structural trip the sufficiency check must surface, not resolve.
- `.mochiko/product/contracts/api.yaml` — only `POST /notes` and `GET /notes/{id}` exist; no `/notes/search` path → a real API-contract gap.
- `.mochiko/product/data-model.md` — Note has no index-related attributes or state; no existing decision covering search-index representation.
- `.mochiko/product/constraints-and-decisions.md` — `C-001` (single-process, no external services) and `D-001` (SQLite via bundled driver, adopt-first already ruled for storage) are live constraints any index-tech choice must respect.
- No `.mochiko/FEATURES.md` index file found (only per-feature `entry.md` files exist) — an absent/incomplete map surface.
- No `.mochiko/memory/` directory at all → no `codebase-analysis.md`, no `knowledge-management.md`, no `governance-intent.md`.
- No `.claude/rules/mochiko/` directory → governance region absent.
- No application source tree found anywhere under the working directory, despite FEAT-001 being marked delivered.
- Environment context states **this working directory is not a git repository** (`Is a git repository: false`), and no `.gitignore` exists.
- `ARCHITECTURE.md` exists at repo root (the store's derived index); no worktrees ignore entry to check since there's no `.gitignore` at all.

These facts drive every phase below.

---

## Phase 1 — Rules ingestion (done, no gate)

**Done:** Raw, full Read of `implement.yaml` and `common.yaml` (first-class, no build step); labels cross-checked against `command-labels.yaml`; fail-condition count verified at 15/15.
**Read:** the three schema files above.
**Written:** nothing.
**Seats:** none — this is the Delivery Manager's own action, no seat spawned yet.
**Gate:** none.

## Phase 2 — Entry resolution and scope typing

**Done:** Confirm `FEAT-002` is a plain capability (no `EPIC-XXX` lookup needed — no `.mochiko/epics/` directory exists), confirm selection-scope entry (spec's accepted selection, not a delta card from `/mochiko:feature`), confirm the one named dependency (FEAT-001) is `delivered` so it does not block.
**Read:** `.mochiko/features/FEAT-002/entry.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/specs/note-search/spec.md`.
**Written:** nothing yet.
**Seats:** none (Delivery Manager only).
**Gate:** none — this feeds the run-open gate in Phase 4.

## Phase 3 — Sufficiency check

**Done:** Dispatch the ten-clause-per-row sufficiency grading (`mochiko:review-sufficiency`) over W1 and W2 against spec.md, the architecture store spine, and the product baselines. Staffed with a seat that authored none of those sources and will not design or build this batch — my call under `impl.staffing-latitude`/`impl.seat-sufficiency-independence` would be **mochiko:validator** (generic independent grader, uninvolved in authoring the spec/store/baselines), exempt from plan approval as a grading seat.
**Read (by the sufficiency seat):** `spec.md`, `product/architecture/spine.md`, `product/data-model.md`, `product/contracts/api.yaml`, `product/constraints-and-decisions.md`, `features/FEAT-002/entry.md`.
**Written:** `.mochiko/features/FEAT-002/sufficiency-report.md` — per-row verdict, the store-consult result, the FR-103-vs-spine contradiction as a trip for the user, the missing `/notes/search` contract, the missing index-tech decision, and (per its absent-baseline branch) a note on the absent `codebase-analysis.md`/governance region. Given what the files already show, both rows are very likely to grade **gap**, not sufficient — but that verdict is the seat's to render, not mine to assert here.
**Seats:** mochiko:validator (sufficiency grading only).
**Gate:** none directly — a disputed clause defaults to gap and is escalated at the next gate, never cleared solo.

## Phase 4 — Run-open confirmation (**user gate**)

**What is confirmed, in one non-negotiable message:**
- Batch identity and scope type: FEAT-002, selection scope, rows W1 + W2, ratified 2026-08-26.
- Attempt bounds restated at their only redeclaration point: `attempt_bound_cycle = 3` per cycle, `gap_rework_bound = 2` at run scope — accept defaults or redeclare now.
- The sufficiency verdict and its gap routing (from Phase 3).
- Every trip and conflict surfaced for the user's ruling, concretely:
  1. **Architecture contradiction:** FR-103 requires a background index worker; the ruled spine says no background workers exist. This is a store trip requiring the user's ruling (rule now, or explicitly defer on the record).
  2. **Absent map index:** no `.mochiko/FEATURES.md` found — surfaced, not run-failing.
  3. **Absent governance region:** `.claude/rules/mochiko/` doesn't exist — surfaced.
  4. **Absent/no codebase analysis, no application source found:** offer `/mochiko:setup`, or proceed greenfield with the warning logged, per `impl.absent-surfaces`.
  5. **Not a git repository:** the final-validation cold-verification step (`impl.cold-verification`) needs `git ls-files -co --exclude-standard`, which requires a git repo. Flagged now since it will block Phase 8 otherwise.
- The done condition stated plainly: every cycle card checked, test-first, independently verified against real infrastructure per-cycle and whole; code traces to FR-101/FR-102/FR-103 and SC-101/102/103; aligns with governance; acceptance landing executed whole; run closes accept/amend/reject.

**Branches:**
- **User rules the store trip in favor of the async worker, accepts proceeding without codebase-analysis.md, and either offers to `git init` or accepts the cold-verification limitation as a known gap to resolve before Phase 8** → run proceeds to Phase 5 (design phase fires — gaps are real and named).
- **User instead directs the git-repo issue be fixed first, or asks for `/mochiko:setup` before continuing** → run pauses here; nothing downstream executes until resolved.
- **User rejects the async-worker approach outright (e.g. wants synchronous reindex-on-write to avoid a new architecture element)** → this reframes the Phase 5 gap scope (design phase still fires, but scoped to a different resolution) rather than blocking entry.
- **User disputes a sufficiency clause the grader called sufficient** → per `impl.sufficiency-disputed-clause`, it defaults to gap and folds into Phase 5's scope regardless.

## Phase 5 — Design phase (fires: real gaps exist)

**Done:** Author exactly the named gaps, nothing more, each on a plan the Delivery Manager approves first (`impl.plan-approval-producers`).
- **mochiko:principal-architect** (architect_seat): resolves the FR-103/spine contradiction — either a store delta adding a new `AX-XXX` async in-process index-worker element (C4-container delta diagram, sequence diagram for the create→index→search flow) per `mochiko:patterns-system-design`, structured/lifecycled per `mochiko:authoring-architecture-store`; or, if the user ruled synchronous reindex in Phase 4, a smaller delta reflecting that instead. Either way this crosses `mochiko:patterns-adopt-first` if a commodity search-index technology (e.g. SQLite FTS5) is a candidate — that adopt-first ruling and any IP-XXX provisioning call are the user's, not builder-decided, and would halt to a checkpoint inside this phase.
- **mochiko:technical-analyst** (design_seat): authors the `/notes/search` endpoint contract via `mochiko:patterns-api-contracts`, and any `D-XXX`/`NFR-XXX` decision content for the index technology via `mochiko:authoring-technical-requirements`, traced to FR-101/102/103.
- **mochiko:qa-engineer** (qa_seat), optionally, seeds design-time acceptance test cases.
**Read:** `sufficiency-report.md`, `spec.md`, current baselines (`data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `architecture/spine.md`).
**Written (deltas beside baselines, never in place):**
- `.mochiko/features/FEAT-002/contracts/` — delta adding `GET /notes/search`.
- `.mochiko/features/FEAT-002/constraints-and-decisions.md` (delta) or equivalent — the index-tech decision.
- `.mochiko/features/FEAT-002/` — the architecture store delta (rendered C4 delta diagram + changed `AX-XXX` row table).
- Design-phase plan artifacts per `mochiko:patterns-plan-minimalism` (rung-justified, disclosed).
**Review pair (non-author, before checkpoint):** `mochiko:review-plan-artifacts` (conformance to the gap list, blocking) and `mochiko:review-feasibility` (buildability/contradiction — specifically whether the new worker element still respects `C-001` single-process).
**Gate — Design checkpoint (`impl.gate-design-checkpoint`, user's):** the user signs the design deltas and the store delta (rendered diagram plus the changed `AX-XXX` row table).
**Branches:**
- **Sign as presented** → proceed to Phase 6.
- **Amend** (e.g., swap the async worker for synchronous reindex, or reject the proposed index technology) → the authoring seats rework within the same phase, re-reviewed by the same pair, re-presented; no fixed numeric bound is declared for design rework in this schema — it's user-paced up to this checkpoint.
- **Stop here and resume later** — explicitly permitted; the run pauses with the design incomplete/unsigned and nothing built.

## Phase 6 — Card authoring and card confirm

**Done:** A design-class, non-builder seat authors `tasks.md` as cycle cards, foundation before feature, walking skeleton first, per `mochiko:patterns-vertical-tdd`. Given `qa_seat` must author the `**TEST:**` cases within its slicing, the natural card-author here is **mochiko:qa-engineer**. Concretely likely slicing: a walking-skeleton cycle (empty-result round trip through the new endpoint), then W1 (query matching/ranking/400 handling, SC-101/SC-102), then W2 (background-freshness round trip within 2s, SC-103) — but the actual bundling is the seat's judgment, not mine.
**Read:** the signed design deltas and store delta, `spec.md` (for cited acceptance-criteria IDs), `tasks_schema` (`plugins/mochiko/schemas/tasks.yaml`) as the grammar source.
**Written:** `.mochiko/features/FEAT-002/tasks.md` — cards with stories/rationale, dependencies, acceptance criteria by ID, a `**TEST:**` real-infrastructure gate per card, and brownfield exposure per card (e.g. `[EXTEND]` on the router/contract, `[NEW]` on the worker module).
**Review (independent, before confirm):** a verification seat distinct from the card author — e.g. **mochiko:validator** or **mochiko:tech-lead** — grades quality via `mochiko:review-plan-artifacts` and buildability by its own judgment.
**Gate — Card confirm (`impl.gate-card-confirm`, user's):** the user rules the slicing before any build starts.
**Branches:**
- **Approve as-is** → proceed to Phase 7.
- **Request re-slice** (merge/split/reorder cycles) → card author reworks, re-reviewed, re-presented.
- **Flag an infeasible card** → escalated to the user as a business-level scope decision (`impl.infeasible-card-escalation`), not resolved by any seat.

## Phase 7 — Build, cycle by cycle

**Done:** **mochiko:staff-engineer** (builder_seat) executes confirmed cards in order, decomposing each into concrete build-time tasks disclosed in `cycle-report.md`, test-first, applying `mochiko:patterns-code-minimalism` at decomposition (e.g., checking whether the chosen index approach is already a stdlib/installed-dep rung before writing anything new) and `mochiko:brownfield-integration` on any touch to the existing note-capture handler/store code. Since no governance region exists, the builder's brief states that absence explicitly rather than naming rule files (`impl.briefs-name-rules-files` only applies when a region is present).
**Read:** `tasks.md`, the signed design deltas, existing code the cycle touches.
**Written:** working code plus, per cycle, `.mochiko/features/FEAT-002/cycle-report.md` entries (append-and-date, per `templates/report-format.md`); `tasks.md` checkboxes flipped as cycles complete.
**Per-cycle verification (independent of the builder):** a verification seat (e.g. **mochiko:qa-engineer** or **mochiko:validator**, whichever wasn't the card author) runs `mochiko:testing-end-user` against real infrastructure — actual HTTP calls, the actual SQLite file, never mocks — for the card's `**TEST:**` gate, plus the advisory `mochiko:review-code-minimalism` lens over the diff and cycle report.
**Attempt economy:** each grading round consumes one of `attempt_bound_cycle = 3` per cycle; two consecutive rounds with unchanged findings triggers a no-progress stop, halting that cycle and presenting state to the user (exemption from the count is the user's call only).
**Mid-run design re-fire:** if the builder hits undesigned structure (e.g., the chosen index technology needs a schema migration the delta didn't cover), that cycle halts and Phase 5 re-fires scoped to the discovery, same grading, same checkpoint, before resuming.
**Deviation gate:** if a built shape adds/removes a box or arrow, or moves a responsibility, beyond the signed delta (e.g. builder splits the worker into a separate process, crossing `C-001`), the cycle stops and is presented to the user: build as approved, or amend the delta first — never silently designed around.
**Written also (if triggered):** `.mochiko/features/FEAT-002/baseline-delta.md` for any build-time technical decision discovered at decomposition (a `D-XXX`/`C-XXX`/`IP-XXX` row), graded later by the landing verification seat, never edited in place.

## Phase 8 — Final validation (selection scope: full set applies)

**Done:**
- **Regression sweep** runs FEAT-001's durable gate set (`.mochiko/features/FEAT-001/gates.md`) plus its card cases, since FEAT-002 sits in FEAT-001's territory (reads the notes store FEAT-001 built). A failure here fails the run like any other gate.
- **Cold verification:** builds and runs the full quality-gate suite from a dependency-cold snapshot (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-<purpose>/`). **This is where the "not a git repository" fact from Phase 4 becomes load-bearing** — if unresolved, this step cannot execute as specified and the run would need to halt here and surface it, rather than skip it silently.
- **Gap-finding pass fires** (selection scope, not skipped): a fresh, blind **mochiko:devils-advocate** (gap_finder_seat), two-message dispatch per `mochiko:testing-gap-finding` — first message carries only `spec.md`, `sufficiency-report.md`, the design deltas, and the baselines (never code, `tasks.md`, `**TEST:**` cases, or any report); the seat states its derived expectations before probing.
- **Mutation lens:** runs on the verification seat if run at high depth (my call at run scope); at lower depth, skipped and stated as such — never silently omitted.
**Read:** all of the above sources; the delivered code tree.
**Written:** `.mochiko/features/FEAT-002/final-validation-report.md`, the built-vs-signed diff against the store delta.
**Findings routing:** spec-required behavior broken fails final validation (bounded by `gap_rework_bound = 2` rounds at run scope, or charged to the owning cycle's remaining attempts if localized); beyond-spec findings are advisory, disposition reserved to the user (fix now / BACKLOG.md / accept as designed); a disputed finding-kind defaults advisory, never gated alone by the finder.

## Phase 9 — Landing (**executed whole, at acceptance**)

**Done, once accepted:**
- Selection-scope landing: W1 and W2 fold into FEAT-002's extent lines, status set `delivered` (dated), the map index line updated (minted if `.mochiko/FEATURES.md` truly doesn't exist, since it's needed for this write), the `note-search` spec's row closes when all its selected rows have folded.
- Store landing: the delta's elements (the new/adjusted `AX-XXX` async-worker row, or whatever the user ruled in Phase 5) flip to built, `FEAT-002` keys clear, `As-built:`/`Drift:` fields written and independently graded, orphan check runs, `ARCHITECTURE.md` regenerated by the store skill (never hand-edited).
- Every touched baseline (`contracts/api.yaml`, `data-model.md` if touched, `constraints-and-decisions.md`) folds exactly once via a graded three-way diff, checked by the landing verification seat.
- Gap findings ruled fix-now or backlog fold into `.mochiko/features/FEAT-002/gates.md` (minted, since none exists yet), authored in `**TEST:**` grammar by `qa_seat`.
**Written:** all of the above, plus the run's close-out surfacing rounds/seats spawned at each checkpoint.
**Gate — Final acceptance (`impl.gate-final-acceptance`, user's):** accept / amend / reject.
**Branches:**
- **Accept** → the acceptance landing executes whole (all folds together, not piecemeal), and the run closes with a verdict stated explicitly against the done condition and the 15-item fail-condition checklist (confirming none stand).
- **Amend** → the specific amendment is applied, re-graded where it touches a graded fold, then re-presented for acceptance.
- **Reject** → the run does not close as delivered; state is presented as-is; this would leave `impl.fail.no-acceptance` standing until resolved in a later run.