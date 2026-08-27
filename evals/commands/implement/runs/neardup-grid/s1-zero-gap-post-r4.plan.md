# Action Plan — `/mochiko:implement FEAT-001` (plan-only, not executed)

**Scenario facts established by reading (no dispatch, no writes):**
- `FEAT-001` = "Note capture," status `selected`, entry at `.mochiko/features/FEAT-001/entry.md`, architecture link `AX-001` (persistence), `AX-002` (logging). Not an `EPIC-XXX` — resolves to itself, **scope = selection**.
- Work rows: `W1` — create a note (US-001, SC-001/SC-002), `W2` — fetch a note by id (US-002, SC-003). Both selected, dependency line reads "None. First capability on the map."
- Spec `.mochiko/specs/note-capture/spec.md` is `accepted`, ratified 2026-08-20, has FR-001..004, SC-001..003, edge cases, no UX surface (API-only, no Screens & Flows).
- Product baselines are already populated and internally consistent: `data-model.md` (Note entity, full), `contracts/api.yaml` (POST /notes, GET /notes/{id}, both matched to the spec), `constraints-and-decisions.md` (C-001, D-001, D-002 ratified), architecture spine (`AX-001`, `AX-002` status `ruled` with NFR-001/NFR-002 traced to SC-001; `AX-003` explicitly n/a with a stated revisit trigger).
- Absent surfaces: no `CLAUDE.md` (**governance_region: absent**), no `.claude/rules/mochiko/`, no `.mochiko/memory/knowledge-management.md` (**km_file: absent**), no `.mochiko/memory/codebase-analysis.md` and no `src/` tree at all (genuinely greenfield, not "stale brownfield analysis"), no `.mochiko/features/FEAT-001/gates.md` yet (nothing delivered before this run), no `.gitignore`.
- `vars` resolved for this run: `attempt_bound_cycle=3`, `gap_rework_bound=2`, `builder_seat=staff-engineer`, `design_seat=technical-analyst`, `architect_seat=principal-architect`, `qa_seat=qa-engineer`, `gap_finder_seat=devils-advocate`, `explore_model=haiku`.

---

## Phase 1 — Entry gating and sufficiency check

**Does:** Confirm `FEAT-001` is a real capability with selected, ratified rows (`W1`, `W2`) and no unmet dependency (entry states none, and it's the map's first capability — so nothing blocks). Resolve `scope: selection`. Note `governance_region: absent` and `depth` therefore unresolved from any region — this is an absent surface, surfaced to the user later, never auto-resolved and never run-failing. Dispatch the sufficiency check per `mochiko:review-sufficiency`, staffed by a seat that authored none of `spec.md`, the architecture store, or the product baselines and will not design or build this batch (candidates: `qa-engineer` or `validator` — `technical-analyst`, `principal-architect`, and `requirements-analyst` are disqualified as likely authors of those baselines). It grades the ten-clause check per row (`W1`, `W2`) against `spec.md`, `.mochiko/product/architecture/spine.md`, and `.mochiko/product/*`.

**Reads:** `.mochiko/features/FEAT-001/entry.md`, `.mochiko/specs/note-capture/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`, `FEATURES.md`.

**Writes:** `.mochiko/features/FEAT-001/sufficiency-report.md` — per-row verdict (sufficient, or a named gap list), the store-consult result, any no-delta claim, a `quickstart.md` null-path note (no external-integration surface exists here), and any run-open trips.

**Seats:** one independent sufficiency-grading seat (exempt from plan approval, per `impl.staffing-latitude`/`impl.seat-sufficiency-independence`).

**Gate:** none yet — the verdict is binding but is *presented*, not ruled, at the next phase.

## Phase 2 — Run-open confirmation (user gate)

**Does:** One blocking confirmation, no negotiation. States: batch = `FEAT-001` "Note capture," scope = selection; restates the attempt bounds at their only redeclaration point (`attempt_bound_cycle=3` per cycle, `gap_rework_bound=2` at run scope) so the user may change them here only; presents the Phase 1 sufficiency verdict and any gap routing; presents trips/conflicts for ruling — here: the absent governance region (depth unresolved), the absent `knowledge-management.md`, the absent `codebase-analysis.md`/no existing code (offer `/mochiko:setup` or proceed greenfield with the warning logged); states the done condition (all cycle cards `[x]`, test-first, verified per-cycle and whole-build against real infrastructure, acceptance executed).

**Reads:** nothing new — synthesizes Phase 1 output.

**Writes:** nothing (the confirmation itself is a blocking-text exchange, not an artifact).

**Gate — what's confirmed:** batch identity/scope, attempt bounds (as declared or amended), the sufficiency verdict and its gap routing, the listed trips, the done condition.
**Branches:**
- *User confirms as-is* → proceed to Phase 3A (if gaps were named) or Phase 3B (if the verdict is clean).
- *User amends attempt bounds* → new bounds recorded here (their only legal redeclaration point); proceed.
- *User rules on a trip* (e.g., "proceed greenfield," "skip governance-region setup for now") → recorded, proceed.
- *User wants to route elsewhere* (e.g., scope isn't actually ratified, or wants `/mochiko:feature` first) → run does not open; ends here.

## Phase 3A — Design phase (fires only if Phase 1 named a gap)

**Does:** Fires strictly over the named gaps, nothing more, each on a plan approved by the run before any seat writes. Given how complete the existing baselines already are (populated data model, OpenAPI contract, ratified constraints/decisions, a `ruled` architecture spine with traced NFRs), a materialized gap is plausible but not guaranteed — this branch is conditional, not assumed. Staffing: `technical-analyst` for a design-artifact delta, `principal-architect` only if a structural/store delta is implicated, `qa-engineer` for any **TEST:** case content — each runs `mochiko:patterns-plan-minimalism`'s ladder, disclosed. `staff-engineer` never designs its own gaps.

**Reads:** `sufficiency-report.md`, `spec.md`, the product baselines listed above.

**Writes:** deltas beside the touched baselines under `.mochiko/features/FEAT-001/` (e.g. a `data-model.md` delta, a `contracts/` delta, a before/after prose delta for a constraints/decisions change), plus an architecture-store delta only if the structural trigger fired (per `mochiko:patterns-system-design` / `mochiko:authoring-architecture-store`); the design phase also asserts design-implied dependencies/sharpened extent onto `.mochiko/features/FEAT-001/entry.md` with provenance.

**Review (non-author, before checkpoint):** `mochiko:review-plan-artifacts` (conformance to the gap list, card quality — blocking) and `mochiko:review-feasibility` (buildability/contradiction) by a seat that authored none of the Phase 3A output.

**Gate — design checkpoint (user's):** what's confirmed — the design deltas and any store delta (rendered diagram + changed `AX-XXX` rows, or source + a changed-element table if no render surface), plus the two reviewers' findings.
**Branches:**
- *Sign* → this becomes the deviation-gate anchor; proceed to Phase 4.
- *Amend* → design-gaps-only scope stays fixed to the named gaps; a broader ask is either a fresh gap-list iteration or a scope escalation, reconciled before re-presenting.
- *Stop here* → run pauses; resumable later without loss.
- *Reject* → gaps stand unresolved; the run cannot proceed to code.

## Phase 3B — Zero-gap path (fires only if Phase 1's verdict is clean for both rows)

**Does:** No design-seat authoring, no design checkpoint. Instead the card-authoring seat (Phase 4) makes the map-entry assertion the design phase would have made — dependencies/extent onto `FEAT-001/entry.md` — surfacing any drift at the card confirm rather than a separate checkpoint.

**Reads/Writes:** folded into Phase 4's card-authoring step.

**Gate:** none of its own — proceeds straight to Phase 4's card confirm.

## Phase 4 — Cycle-card authoring and card confirm (user gate)

**Does:** A design-class seat that is not the builder — `technical-analyst` or `principal-architect`, not `staff-engineer` — slices `US-001`/`US-002` into cycle cards per `mochiko:patterns-vertical-tdd` (walking skeleton first, foundation before feature). Given two P1 stories each with a stated independent test, a plausible slicing is Cycle 1 = foundation + create-note (covers FR-001, FR-002, FR-004, SC-001, SC-002) and Cycle 2 = fetch-by-id (covers FR-003, SC-003) — but the actual Simple/Split/Merge call belongs to the authoring seat, not asserted here. `qa-engineer` authors the **TEST:** real-infrastructure gate within each card's slicing. Cards carry stories/rationale, dependencies, acceptance criteria by ID, the **TEST:** gate, and brownfield exposure (here: net-new files, no existing code touched) — no task lists, no file paths.

**Reads:** the (possibly delta-augmented) `spec.md` and baselines; `plugins/mochiko/schemas/tasks.yaml` if the `mochiko-cli template tasks` binary is unavailable.

**Writes:** `.mochiko/features/FEAT-001/tasks.md` (cycle cards, checkboxes unchecked).

**Review (non-author, before confirm):** an independent verification seat grades quality (`mochiko:review-plan-artifacts`) and buildability.

**Gate — card confirm (user's):** what's confirmed — the cycle slicing and its order, before any build starts.
**Branches:**
- *Approve* → proceed to Phase 5.
- *Resplit requested* → card-authoring seat revises, re-reviewed, re-presented.
- *A card ruled infeasible* → escalated as a business-level scope decision; may return a row to pending.

## Phase 5 — Build and per-cycle verification (repeats per card, foundation first)

**Does, per cycle card in order:** `staff-engineer` decomposes the card into concrete tasks at build time (disclosed in the cycle report), builds test-first (red→green→refactor) on the approved card, runs `mochiko:patterns-code-minimalism`'s ladder at decomposition (disclosed), and — since this touches no pre-existing code — `mochiko:brownfield-integration` applies only from Cycle 2 onward if it touches files Cycle 1 created. A seat independent of the builder (e.g. `qa-engineer`) then verifies against real infrastructure per `mochiko:testing-end-user` (actually running the service against a real SQLite file, real HTTP calls — no mocks), runs the full repository quality-gate suite, and applies the advisory `mochiko:review-code-minimalism` lens reading the diff, the cycle report, and surrounding code.

**Reads:** the card, the working tree, `.mochiko/features/FEAT-001/tasks.md`.

**Writes:** the feature's source files (net-new, e.g. an HTTP handler, SQLite storage layer); `.mochiko/features/FEAT-001/cycle-report-<n>.md` per `templates/report-format.md` (decomposition, difficulties, deviations, `domain_deps_added`); flips the card's checkbox in `tasks.md` on pass.

**Attempt economy (floor):** each grading round of a cycle spends one of 3 attempts; two consecutive rounds with unchanged findings is a no-progress stop that halts the cycle and presents state; only the user may exempt a round from the count.

**Deviation gate (floor):** if a cycle would add/remove a box or arrow, or move a responsibility across a boundary already ruled in the architecture spine (e.g. needing a queue, needing auth beyond `AX-003`'s stated n/a), the cycle stops and is presented — never silently designed around.

**Cycle checkpoint:** reserved-to-user questions (Minor findings → BACKLOG.md booking; Important+ → blocks the cycle) batch here; only a build-blocking question interrupts mid-cycle immediately.

This phase repeats until every card in `tasks.md` is `[x]`.

## Phase 6 — Final validation

**Does:** Runs the regression sweep over previously delivered features' durable gate sets in this territory — since `FEAT-001` is the map's first capability, this is a real but empty sweep (nothing prior to regress against). Runs cold verification: builds and runs quality gates from a dependency-cold snapshot of the uncommitted working tree copied to `.claude/worktrees/mochiko-<purpose>/`, first checking (and, if absent, needing) a `.claude/worktrees` ignore entry — currently no `.gitignore` exists at all, so this is a concrete gap this phase must close before the snapshot. Runs the gap-finding pass — **mandatory** for selection scope — via a fresh, blind `devils-advocate` that built nothing and saw no test cases, two-message dispatch: first message carries only `spec.md`, `sufficiency-report.md`, any design deltas, and the baselines (`data-model.md`, `contracts/api.yaml`, the store's NFR rows) — never code, `tasks.md`, **TEST:** cases, or reports; the seat states derived expectations before probing. The mutation lens fires only if `depth: high`, which is unresolved here (governance region absent) — this must be explicitly stated as a disclosed skip/unresolved condition, not silently omitted.

**Reads:** all delivered cycle reports, `spec.md`, baselines, the running built system.

**Writes:** `.mochiko/features/FEAT-001/final-validation-report.md` (per `templates/report-format.md`); a `.gitignore` entry for `/.claude/worktrees` if genuinely required to run cold verification.

**Findings routing:** a spec-required-behavior-broken finding fails final validation and must be resolved within the gap-rework bound (2 rounds at run scope, or charged to the localized cycle's remaining attempts); a beyond-spec finding is advisory, disposition reserved to the user (fix now / BACKLOG.md / accept as designed).

**Gate:** none of its own yet — findings disposition and final acceptance are combined at Phase 7.

## Phase 7 — Landing and final acceptance (user gate)

**Does:** Presents the whole acceptance package: all cards `[x]`, per-cycle and whole-build real-infrastructure evidence, the final-validation report (including gap-finding results and any beyond-spec dispositions the user must rule on), and the landing about to execute. Landing (selection scope): `W1` and `W2` fold into `FEAT-001`'s extent and vanish from "selected," status flips to `delivered` (dated), `FEATURES.md`'s status column updates, and `.mochiko/specs/note-capture/spec.md` reads closed once both rows have folded (true here, since W1+W2 are its whole selection). If a store delta was signed in Phase 3A, it folds now — `AX-001`/`AX-002` (or whichever rows were touched) flip built, `As-built:`/`Drift:` fields are written and independently graded, the orphan check runs, and the derived root `ARCHITECTURE.md` is regenerated (never hand-edited). Any gap findings ruled fix-now or backlog fold into a freshly minted `.mochiko/features/FEAT-001/gates.md`, authored by `qa-engineer` in **TEST:** grammar. `km_file` is absent, so no knowledge-management landing obligation fires. No git mutations are run — commits are suggested only.

**Reads:** the final-validation report, `tasks.md`, all cycle reports, current baselines and store.

**Writes:** `.mochiko/features/FEAT-001/entry.md` (status → delivered), `FEATURES.md` (status column), `.mochiko/specs/note-capture/spec.md` (closed marker), `.mochiko/features/FEAT-001/gates.md` (minted), the folded baseline/store files (via graded three-way diff), `ARCHITECTURE.md` (regenerated) if a store delta landed.

**Gate — final acceptance (user's), plain blocking text, no timed prompt:**
**Branches:**
- *Accept* → the landing above executes as described; the run closes with a verdict against the done condition, surfacing total rounds consumed and seats spawned.
- *Amend* → user specifies the change; run loops back to the implicated phase (typically a cycle rework or a beyond-spec disposition change), bounded by the gap-rework attempt economy, then re-presents this gate.
- *Reject* → landing does not execute; the run closes without delivering `FEAT-001`, state recorded as not accepted.

## Phase 8 — Close

**Does:** States the closing verdict against the done condition; if any of the 15 `kind: fail` conditions in `impl.sec.fail-conditions` stands (e.g. `impl.fail.gap-finding-missing`, `impl.fail.no-acceptance`), the run is FAIL regardless of how far Phases 1–7 progressed. Confirms the fail-condition count is 15 (verified by the count above) — a mismatch would itself halt and surface before closing. Surfaces total verification rounds consumed and every seat spawned across the run.

**Reads:** the full trail of reports written in Phases 1–7.

**Writes:** nothing beyond the verdict stated in the close message (already-written reports stand as the evidence trail).