# Plan — `/mochiko:implement FEAT-001` (plan-only, not executed)

## Invocation-specific facts gathered before planning

- `FEAT-001` = capability **"Note capture"**, status `selected` in `FEATURES.md`, entry at `.mochiko/features/FEAT-001/entry.md`. Not an `EPIC-XXX` — plain feature run.
- Two work rows are marked selected: **W1** (create a note, US-001, criteria SC-001/SC-002) and **W2** (fetch a note by id, US-002, criteria SC-003). Selection source recorded on the entry: the spec's accepted selection, ratified 2026-08-20 → **selection scope**, not delta scope.
- Entry declares no dependencies ("first capability on the map") → nothing to block on.
- Spec `.mochiko/specs/note-capture/spec.md` is `accepted`, has exactly US-001/US-002 (no other rows), no Screens & Flows (HTTP-API-only feature).
- Product baselines exist and look populated: `data-model.md` (Note entity), `contracts/api.yaml` (POST /notes, GET /notes/{id}), `constraints-and-decisions.md` (C-001 single-process, D-001 SQLite, D-002 stdlib HTTP), architecture store `spine.md` (AX-001 persistence / NFR-001, AX-002 logging / NFR-002 — both `ruled`; AX-003 auth `n-a`).
- No governance region found (no `.claude/rules/mochiko/`, no `CLAUDE.md`) and no `.mochiko/memory/` at all (no `codebase-analysis.md`, no `knowledge-management.md`).
- No code exists anywhere in the repo outside `.mochiko/` and `plugins/` — genuinely greenfield, nothing to regress against, no prior feature gates to sweep.
- Notable environment mismatch: this working directory is **not a git repository**, yet the final-validation cold-verification step and the no-mutation rules are git-based (`git ls-files`, `git rev-parse HEAD:<dir>`). This would need to be surfaced as a blocking practical obstacle when the run actually reaches that step (see Phase 7).

---

## Phase 0 — Load the run's binding rules

**Done:** Read, in full, `plugins/mochiko/schemas/implement.yaml` and `plugins/mochiko/schemas/common.yaml` (already performed for this plan), plus `plugins/mochiko/schemas/command-labels.yaml` for label meanings. Substitute the run's variables at read time (per-cycle attempt bound 3, gap-rework bound 2, seat defaults: builder = staff-engineer, design = technical-analyst, architect = principal-architect, QA = qa-engineer, gap-finder = devils-advocate, explore model = haiku).
**Reads:** the two schema files above.
**Writes:** none.
**Seats/skills:** none yet — this is the Delivery Manager's own setup.
**Gate:** none.

## Phase 1 — Resolve entry and scope

**Done:** Confirm `FEAT-001` resolves to a real capability entry with ratified, selected work rows (W1, W2); confirm scope type is selection scope (not a delta card, not an epic); confirm no selected row depends on an undelivered row (entry states no dependencies, so nothing blocks).
**Reads:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/specs/note-capture/spec.md`.
**Writes:** none.
**Seats/skills:** Delivery Manager only.
**Gate:** none (informational resolution, not a user checkpoint).

## Phase 2 — Sufficiency check

**Done:** Stage a grading seat that authored none of the spec, the architecture store, or the product baselines, and who will not go on to design or build this batch — a candidate such as `mochiko:qa-engineer` or `mochiko:devils-advocate`, since the likely authors of these artifacts (technical-analyst for the baselines, principal-architect for the store) are the run's default design seats. That seat grades each selected row (W1, W2) against the ten-clause sufficiency check over the spec, the architecture store, and the baselines, per the sufficiency-review procedure. Any absent surface (here: no governance region, no `.mochiko/memory/`) is logged as a surfaced note, not auto-resolved and not run-failing.
**Reads:** `.mochiko/specs/note-capture/spec.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/features/FEAT-001/entry.md`.
**Writes:** `.mochiko/features/FEAT-001/sufficiency-report.md` — binding per-row verdict (sufficient, or a named gap list), the store-consult result, any trips for the user, the absent-governance-region note, and the missing-`.mochiko/memory/` note.
**Seats/skills:** the sufficiency-grading seat; procedure owned by the sufficiency-review skill.
**Gate:** none directly — the verdict feeds the run-open gate next. A disputed clause the grader can't clear defaults to "gap" and is queued for the user at that gate.

## Phase 3 — Run-open confirmation (USER GATE)

**What is confirmed:** one blocking, plain-text confirmation naming the batch ("FEAT-001 Note capture, selection scope, rows W1 + W2"); restating the two attempt bounds at their only redeclaration point (per-cycle verification attempts, default 3; gap-rework rounds at final validation, default 2); presenting the sufficiency verdict and, if any row came back with a gap, the routing into a design phase; presenting any store trips or in-flight conflicts (none currently visible, but this is where any would surface) for the user's ruling; and stating the fixed done condition (all cycle cards checked, built test-first, independently verified per-cycle and whole, code traces to criteria and governance, acceptance landing executed whole, run closes on accept/amend/reject).

**Onward branches:**
- **User confirms as presented (attempt bounds unchanged, verdict accepted):** proceed — to Phase 4 if any gap was named, otherwise to Phase 5 (zero-gap path).
- **User redeclares an attempt bound:** the new per-cycle or gap-rework number replaces the default for the remainder of this run only.
- **User rules on a disputed sufficiency clause or a store trip/conflict:** that ruling is recorded and either clears the row (proceed) or converts it into a named gap (route to Phase 4).
- **User decides the framing itself is wrong** (e.g., scope should be re-cut, or this isn't actually ready): the run does not open. It would route back — a genuinely new capability to `/mochiko:specify`, or a feature-keyed delta to `/mochiko:feature` — and this implement run ends without touching code.

**Writes:** none beyond what Phase 2 already wrote; this gate only records the user's ruling (appended to the sufficiency report or the eventual run report, not a new artifact by itself).

## Phase 4 — Design phase (fires only if Phase 2 named a gap)

**Done:** Staff design seats scoped to exactly the named gaps and nothing more — typically `technical-analyst` for a requirements/design delta, `principal-architect` if a structural/architecture-store delta is implicated, `qa-engineer` for any missing acceptance-test cases — each working only on a plan the Delivery Manager approved first, each justified against the simplest-adequate-artifact ladder. `staff-engineer` (the builder) never authors these. If the architecture store has no ruled content yet for a needed area, the first duty is seeding an empty scaffold and confirming it with the user at the checkpoint below, rather than fabricating history.

**Reads:** `sufficiency-report.md`, the current baselines (`data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `spine.md`), `spec.md`.
**Writes (candidate paths, only for the named gaps):** `.mochiko/features/FEAT-001/data-model.delta.md` and/or `.mochiko/features/FEAT-001/contracts/` delta, a before/after prose delta against `constraints-and-decisions.md`, and — only if a structural trigger fired — an architecture-store delta under `.mochiko/product/architecture/`. Also asserts sharpened dependencies/extent back onto the FEAT-001 entry with provenance.
**Review before checkpoint:** a non-author seat grades the package — conformance to the gap list and card-quality readiness (blocking), and a separate buildability/contradiction pass — before it ever reaches the user.

**Design checkpoint (USER GATE):** the user is shown the rendered delta diagram (or, absent a render surface, the source plus a changed-element table) and the named baseline deltas, and is asked to sign them.
- **Sign as presented:** the signed deltas become part of the design inputs for card authoring (Phase 5); this delta is now the anchor for any later deviation check.
- **Request amendment:** design seats revise within the gap-rework bound set at run-open, get re-reviewed, and are re-presented; exhausting the bound without new progress halts and reports state rather than looping silently.
- **Stop here, resume the build later:** the run pauses cleanly — nothing beyond the signed/pending deltas has been written, no code exists yet, and the batch can be picked up in a later `/mochiko:implement` invocation.

## Phase 4b — Zero-gap path (alternative to Phase 4)

**Done:** No design phase fires. The card-authoring seat in Phase 5 additionally makes the map-entry assertion the design phase would otherwise have made (dependencies/extent sharpening), and any drift between the entry's intended scope and what's about to be designed is surfaced at the card confirm rather than silently absorbed.

## Phase 5 — Cycle card authoring + card confirm (USER GATE)

**Done:** A design-class seat that is not the builder (not `staff-engineer`) slices W1 and W2 into cycle cards — foundation cycles before feature cycles, walking-skeleton first — following the vertical-slicing procedure. Given the baselines, the expected shape is a foundation cycle (SQLite file setup + stdlib HTTP server skeleton, matching D-001/D-002 and AX-001), then a create-note cycle covering SC-001/SC-002/FR-001/FR-002/FR-004, then a fetch-note cycle covering SC-003/FR-003 — but the exact slicing is this seat's judgment call, not predetermined here. `qa-engineer` authors the `**TEST:**` real-infrastructure gate on each card within its slice. Since nothing exists in the repo yet, every card's brownfield exposure is expected to read as new/greenfield rather than extend-or-modify. A verification seat (independent of the card author) reviews the cards for quality and buildability before they're shown to the user.

**Reads:** `sufficiency-report.md`, any signed design deltas from Phase 4 (or the baselines directly on the zero-gap path), `spec.md`.
**Writes:** `.mochiko/features/FEAT-001/tasks.md` — cycle cards (stories/rationale, dependencies, acceptance-criteria IDs, the `**TEST:**` gate, brownfield exposure) rendered from the tasks template.

**Card confirm gate:** the user rules on the slicing before any build starts.
- **Approve as sliced:** proceed to Phase 6 in the confirmed order.
- **Request re-slicing (merge/split cycles):** the card-authoring seat revises, gets re-reviewed, and re-presents.
- **Flag a card as business-infeasible:** it's escalated to the user as a scope decision, not silently dropped or reshaped by a seat.

## Phase 6 — Build: test-first cycles (loop over confirmed cards)

**Done, per card, in confirmed order (foundation card(s) first):** `staff-engineer` works only on a Delivery-Manager-approved plan, decomposes the card into concrete tasks at build time (disclosed later in the cycle report), applies the pre-code minimalism ladder at decomposition, and drives each task red→green→refactor. Since this repo has no existing code, the brownfield-integration procedure (read-whole-file-first, preserve interface) is not expected to trigger for this run — everything is new creation. Because no governance region exists, every code-touching brief for this run notes that fact as a surfaced absence rather than naming rule files to read.

**Per-cycle verification:** a seat independent of the builder runs the card's `**TEST:**` gate against real infrastructure (evidence captured, not assumed) and applies the advisory code-minimalism lens by reading the diff, the cycle report, and the surrounding code. The repository's full quality-gate suite runs and is never severity-triaged — any failure fails the cycle outright. Attempt economy: each grading pass consumes one of the per-cycle attempts (default 3, or whatever was redeclared at run-open); two consecutive rounds with unchanged findings is a hard stop that halts the cycle and presents state rather than retrying blindly.

**In-cycle sub-gates that can fire (each a USER GATE when triggered):**
- *Deviation from a signed design delta* (only relevant if Phase 4 fired and produced a structural delta): building discovers a need to add/remove a box or arrow, or move a responsibility, versus what was signed → stop, present the choice — build strictly as approved, or amend the delta first by the user's ruling.
- *Undesigned structure discovered mid-build* (can happen even on the zero-gap path): halt that cycle; a scoped design-phase re-fire runs (same review pair, same checkpoint as Phase 4) covering just the discovery, then building resumes.
- *A commodity-category decision arises* (e.g., choosing a UUID generation approach, a migration mechanism) that isn't already settled by D-001/D-002: an adopt-first ruling, or any infrastructure-provisioning call, halts to the user rather than the builder deciding.
- *A build-time decision collides with a ratified constraint* (e.g., something that would violate C-001's single-process constraint): filed as a constraint-challenge finding naming the constraint text, the real need behind it, and the excluded alternative — reserved to the user; only that decision pauses, the rest of the cycle continues.
- *A build-time technical decision is needed* (a new D-XXX/C-XXX/IP-XXX row): written as a minimal, appliable `baseline-delta.md` entry in the feature directory at the moment of discovery — never edited in place — graded later by the landing verification seat before acceptance.

**Writes per cycle:** `.mochiko/features/FEAT-001/cycle-report.md` (append/date per cycle — disclosed decomposition, honest difficulties, deviations, added domain dependencies); `tasks.md` checkbox flips as each card completes; possibly `.mochiko/features/FEAT-001/baseline-delta.md` if a build-time decision surfaced.

**Escalation batching:** non-build-blocking reserved-to-user questions accumulate and land as one batch at the cycle checkpoint rather than interrupting individually; anything the build genuinely cannot proceed without interrupts immediately.

## Phase 7 — Final validation (whole-build)

**Done:**
1. **Cold verification:** build and run the full quality-gate suite from a dependency-cold snapshot of the uncommitted working tree, copied via `git ls-files -co --exclude-standard :!.claude/worktrees` into `.claude/worktrees/mochiko-<purpose>/`, after confirming a `.claude/worktrees` ignore entry exists. **Practical obstacle for this specific invocation:** the working directory is reported as not a git repository, so this git-based snapshot mechanism has nothing to operate on — this would need to be surfaced to the user as a blocking discrepancy at this point (git init, or an alternate cold-snapshot approach, would need the user's ruling) rather than silently skipped.
2. **Regression sweep:** run the accumulated `**TEST:**` gates of any previously delivered feature in this feature's territory. Since FEAT-001 is the first capability on the map with no prior deliveries and no `gates.md` files anywhere yet, this sweep is expected to find nothing to run — still executed for form, not silently skipped.
3. **Gap-finding pass (required on this selection-scope run):** a fresh `devils-advocate` instance, never one that built these cycles or saw the design-time test cases, is dispatched blind in two messages — first only `spec.md`, `sufficiency-report.md`, any design deltas, and the relevant baseline excerpts (data model, contracts, the architecture concern rows' NFR targets) — asked to state derived expectations before it's allowed to see the code or run probes. The independent verification seat, which already holds code sight, also owes a mutation-lens result (or a stated skip) if running at high depth.
4. **Finding disposition:** any finding showing spec-required behavior broken (with evidence and the clause it violates) fails final validation until resolved; a beyond-spec finding is advisory and its disposition (fix now / book to `BACKLOG.md` / accept as designed) is the user's call; a disputed kind defaults to advisory and also goes to the user.
5. **Gap-rework loop:** bounded by the gap-rework attempts set at run-open (default 2 at run scope; a finding localized to one cycle instead charges that cycle's remaining budget); exhaustion or an unchanged-findings round halts the run and presents state for the user's disposition.

**Reads:** everything under `.mochiko/features/FEAT-001/`, the product baselines, the working code tree.
**Writes:** `.mochiko/features/FEAT-001/final-validation-report.md` (or dated append), including an explicit statement of what ran and what didn't (never a silent skip).
**Seats/skills:** the per-cycle verification seat (mutation lens), a fresh `devils-advocate` (gap-finding), coordinated by the Delivery Manager.

## Phase 8 — Acceptance landing (executed whole)

**Done (only after Phase 7 clears, or the user has ruled on remaining findings):** an independent landing-verification seat checks every graded fold before it lands.
- **Store landing** — only if a structural delta was actually signed in Phase 4: flip its elements to built, clear the FEAT-001 key, write graded As-built:/Drift: fields on the touched AX rows, run the orphan check, and regenerate the derived root `ARCHITECTURE.md` (never hand-edited). Given AX-001/AX-002 are already `ruled` rather than in-flight, this step is only expected to trigger if the design phase actually produced new structure.
- **Baseline folds:** any `baseline-delta.md` entries from Phase 6 fold into `constraints-and-decisions.md` / `data-model.md` / `contracts/api.yaml` via a graded three-way diff (pre-fold baseline + delta vs. folded result), checked by the landing seat.
- **KM landing:** skipped — no `.mochiko/memory/knowledge-management.md` exists in this repo.
- **Selection-scope graduation:** W1 and W2 fold into FEAT-001's extent and vanish from "selected"; the entry's status moves to `delivered`, dated, never regressing; the `FEATURES.md` index line updates; since the spec has exactly these two rows, `note-capture`'s spec is expected to read as closed once this folds.
- **Gates fold:** any gap findings ruled fix-now or backlog fold into `.mochiko/features/FEAT-001/gates.md` (minted here, since it doesn't exist yet), authored by `qa-engineer` in the `**TEST:**` grammar; anything accepted-as-designed does not fold.

**Writes:** `.mochiko/features/FEAT-001/entry.md` (status → delivered), `FEATURES.md`, `.mochiko/features/FEAT-001/gates.md`, the touched product-baseline files (via graded fold only, never in place), possibly `ARCHITECTURE.md` (regenerated, not hand-edited).

## Phase 9 — Final acceptance (USER GATE) and close

**What is confirmed:** a plain blocking summary of the final-validation results, exactly what folded where in the landing, and any remaining advisory/backlog items — presented against the fixed done condition.
- **Accept:** the run closes successful; the Delivery Manager issues a verdict against the done condition, having checked that none of the 15 "not-done" fail-conditions stand (unrecorded sufficiency verdict, skipped/unsigned design phase despite named gaps, card independence violation, an unchecked card, a failing quality gate, verification claimed without evidence, a regression, an in-place baseline edit, an unresolved deviation, an incomplete store landing, an ungraded fold, a missing gap-finding pass on this selection-scope run, an unstated skip, an unresolved spec-required gap finding, or missing acceptance itself); FEAT-001 shows delivered.
- **Amend:** the user specifies what needs to change; the run reopens the affected scope — anywhere from a targeted cycle rework (charged against the gap-rework bound) to a landing-detail correction — and re-presents for acceptance.
- **Reject:** the run does not close accepted; per the done condition this is treated as Not Done (no acceptance given), and the Delivery Manager surfaces state and stops rather than forcing a close — what happens next (rework, backlog, abandon) is the user's call.

**Writes at close:** a final run summary appended to the reports already in `.mochiko/features/FEAT-001/`, stating rounds consumed and seats spawned across the run, and the closing verdict.

---

### Seats/skills expected to be involved across the run
`mochiko:qa-engineer` or `mochiko:devils-advocate` (sufficiency grading), `mochiko:technical-analyst` and/or `mochiko:principal-architect` (design-phase authors, conditional), `mochiko:review-plan-artifacts` + `mochiko:review-feasibility` procedures (design/card review pair), a design-class seat for card authoring plus `mochiko:qa-engineer` for `**TEST:**` cases, `mochiko:staff-engineer` (builder, all cycles), an independent verification seat for per-cycle grading and the code-minimalism lens, a fresh `mochiko:devils-advocate` for the blind gap-finding pass, and a landing-verification seat for the acceptance landing. Cross-seat messaging and any shared-write surface (chiefly `tasks.md` and the feature-directory reports) run under the transport floor throughout, since this run composes more than one seat.