# Action Plan — `/mochiko:implement FEAT-001` (plan-only, non-executing)

## Phase 0 — Load the rule surface (already performed as this plan's first action)

**Done:** Read `plugins/mochiko/schemas/implement.yaml` raw, in full, plus `plugins/mochiko/schemas/common.yaml` (for the `extends: common.*` stubs) and `plugins/mochiko/schemas/command-labels.yaml` (label vocabulary), per the run's own opening obligation. Counted the `kind: fail` nodes in `impl.sec.fail-conditions`: **15**, matching the Not-done line — schema and command copy are in sync, so no halt is triggered here.

**Read:** the three files above.
**Written:** nothing.
**Seats:** none — this is the run lead's own binding load, not delegated.
**Gate:** none.

## Phase 1 — Entry resolution and routing

**Done:** Resolve `FEAT-001`. It is a capability ID, not `EPIC-XXX`, so the epic path (`mochiko:authoring-epic`) does not apply. Read the capability entry and cross-check the map index.

**Read:**
- `.mochiko/features/FEAT-001/entry.md` — status `selected`; two work rows, **W1** (Create a note, US-001, SC-001/SC-002) and **W2** (Fetch a note by id, US-002, SC-003), both marked `selected`; scope source = the spec's accepted selection (2026-08-20) → **scope = selection**; Dependencies: `None` — no dependency-blocks-batch condition trips.
- `FEATURES.md` — confirms FEAT-001 "Note capture" at status `selected`, one-row map (first capability, consistent with "no dependencies").
- `.mochiko/specs/note-capture/spec.md` — accepted status, carries US-001/US-002, FR-001…FR-004, SC-001…SC-003, edge cases. No Screens & Flows (API-only, no UX surface) — this is stated in the spec itself, not a gap.

Also check for environment-level absent surfaces relevant to the whole run:
- `CLAUDE.md` — **absent** → governance region absent. Per `impl.absent-surfaces`, this is surfaced to the user, never auto-resolved, and never fails the run.
- `.mochiko/memory/knowledge-management.md` — **absent** → `impl.km-landing`'s `when: {km_file: present}` never fires this run; no knowledge-management landing obligation.
- `.claude/rules/mochiko/` — **absent**, consistent with governance region being absent → `impl.briefs-name-rules-files` (`when: {governance_region: present}`) does not bind; seat briefs need not name rules-dir reads.
- This working directory is **not a git repository**. This is a concrete environmental fact worth flagging early because `impl.cold-verification` (final-validation phase, Phase 7 below) is specified as a `git ls-files -co --exclude-standard` snapshot into `.claude/worktrees/`. With no git repo present, that literal mechanic cannot run as written. This isn't one of the schema's named absent-surface branches, so it doesn't auto-resolve — it would be raised to the user as a real obstacle at or before run-open, with the routing question "initialize git so cold-verification can snapshot the tree, or rule an alternative cold-snapshot mechanism" left to the user's ruling (not the run's own call, since it touches how `impl.fail.no-evidence`/evidence integrity is met).

**Written:** nothing yet.
**Seats:** none (lead-level routing).
**Gate:** none yet — this phase feeds the sufficiency check and the run-open confirmation.

## Phase 2 — Sufficiency check (binding verdict, per `mochiko:review-sufficiency`)

**Done:** Dispatch the sufficiency check per row (W1, W2) under selection scope (the ten-clause form, not the delta three-clause form), against spec.md, the architecture store, and the product baselines. The grading seat must have authored none of those sources — concretely, not `technical-analyst` (likely author of constraints-and-decisions.md/data-model.md under normal authorship) and not `principal-architect` (author of the architecture spine); a seat such as `qa-engineer` or `validator` would be the natural independent choice, confirmed at dispatch against actual authorship provenance. This seat is exempt from plan approval (grading seats always are).

**Read (by the sufficiency seat, per the pointer skill's clause set):**
- `.mochiko/specs/note-capture/spec.md`
- `.mochiko/product/data-model.md` — the `Note` entity, fields id/text/created_at, sensitivity Internal
- `.mochiko/product/contracts/api.yaml` — POST /notes and GET /notes/{id} already fully specified with request/response schemas and status codes matching FR-001…FR-004
- `.mochiko/product/constraints-and-decisions.md` — C-001 (single-process), D-001 (SQLite, adopt-first already argued against hand-rolled and against Postgres), D-002 (stdlib HTTP, no framework)
- `.mochiko/product/architecture/spine.md` — AX-001 (persistence, NFR-001 explicitly graded by FEAT-001 SC-001) and AX-002 (logging, NFR-002) both status `ruled`; AX-003 (auth) explicitly n/a for v1 with a stated revisit trigger

**Would write:** `.mochiko/features/FEAT-001/sufficiency-report.md` — the store-consult result, any no-delta claim, trips for the user at run-open, the `quickstart.md` null-path note (no external-integration surface exists here — SQLite is local, not an external integration), and any `[MODIFY]` amendment against a delivered feature (none — FEAT-001 is the first capability, nothing delivered yet to amend).

**Observation informing the plan (not a pre-ruling):** the baselines read here are unusually complete and already keyed specifically to FEAT-001 (AX-001's NFR-001 literally cites "graded by FEAT-001 SC-001"). That shape is consistent with a `sufficient` verdict with an empty gap list, but the actual verdict is the independent seat's call, not this plan's — both branches are carried forward below.

**Gate/reservation:** any sufficiency clause the grader could not clear, or any disputed clause (`impl.sufficiency-disputed-clause` — defaults to gap, goes to the user, grader never clears alone), is reserved to the user and folds into the run-open confirmation next.

## Phase 3 — Run-open confirmation (the entry gate)

**What would be confirmed**, in one blocking, non-timed plain-text prompt:
- The batch: capability FEAT-001 "Note capture", scope type **selection**, rows W1 + W2, no cross-row dependency block.
- Both attempt bounds redeclared at their only redeclaration point: `attempt_bound_cycle = 3` (per cycle, per verification grading round) and `gap_rework_bound = 2` (final-validation gap-rework rounds) — or the user's override of either, stated here and nowhere else.
- The sufficiency verdict and its gap routing (from Phase 2): either "no gaps, design phase does not fire" or the named gap list that will scope an in-run design phase.
- Any trips and conflicts for the user's ruling: the absent governance region (surfaced per `impl.absent-surfaces`), the not-a-git-repo obstacle to `impl.cold-verification` flagged in Phase 1, any disputed sufficiency clause from Phase 2, and any store-consult trip the sufficiency report recorded.
- The done condition, stated plainly: every cycle card checked off, built test-first and independently verified against real infrastructure per-cycle and whole, code meeting SC-001/SC-002/SC-003 and tracing to FR-001…FR-004, governance-aligned (vacuously, absent a governance region, but surfaced as such), landing executed whole, run closed at final acceptance.

**Branches on the user's ruling:**
- **User confirms as presented, no gaps named** → proceed directly to Phase 5 (cycle-card authoring), skipping Phase 4 entirely; the card-authoring seat makes the map-entry assertion the design phase would otherwise have made (`impl.zero-gap-map-assertion`), surfacing any drift at the card confirm.
- **User confirms as presented, gaps named** → proceed to Phase 4 (design phase), scoped to exactly the named gaps.
- **User adjusts attempt bounds** → the adjusted `attempt_bound_cycle`/`gap_rework_bound` values carry for the rest of the run; no further redeclaration point exists.
- **User rules a trip/conflict differently than presented** (e.g., declines to initialize git, names an alternate cold-verification approach, waives a disputed clause a particular way, or defers a store trip on the record) → that ruling is recorded and binds downstream phases; a deferred trip re-surfaces at the moment it becomes load-bearing (e.g., at Phase 7's cold-verification step).
- **User declines to open the run** (e.g., wants a different capability first, or wants to route to `/mochiko:specify`/`/mochiko:feature` instead because scope isn't actually ratified) → the run does not open; no further phases execute.

## Phase 4 — Design phase (conditional: fires only if Phase 2/3 named gaps)

**Done (only on the gaps branch):** Spawn design-class seat(s) — typically `technical-analyst` for design deltas, `principal-architect` only if a store delta is implicated — each working on a plan the lead approves first (design seats are not exempt from plan approval; grading/verification seats are). Each seat authors exactly the named gaps, nothing more, rung-justified per `mochiko:patterns-plan-minimalism`. The builder (`staff-engineer`) never designs its own gaps.

**Read:** `sufficiency-report.md`, the product baselines (`data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, the architecture store spine), `spec.md`'s cited acceptance criteria.

**Would write (only for whatever the gap list actually names — nothing is written speculatively):** deltas beside their baselines at `.mochiko/features/FEAT-001/` — e.g. a `data-model.md` delta, a `contracts/` delta, a prose-baseline before/after delta, and — only if the structural trigger fires — a store delta as in-flight-class rows added to `.mochiko/product/architecture/spine.md` (the one legal in-place carve at the design checkpoint's sign-off; everywhere else baselines are never edited in place). Also updates the capability entry's dependencies/extent with provenance and fills the architecture link if a store delta exists.

**Independent review before the checkpoint:** a non-author seat runs `mochiko:review-plan-artifacts` (conformance to the gap list, card quality — blocking) and `mochiko:review-feasibility` (buildability/contradiction, and the architecture pass if a store delta exists).

**Gate — the design checkpoint (user's):** presents the design and any store delta (a rendered diagram plus the named AX-XXX row changes, or — no render surface available here — the source plus a changed-element table) for sign-off.
- **User signs** → proceed to Phase 5; the signed delta becomes the anchor for the deviation gate and the built-vs-signed diff at landing.
- **User asks for rework** → the design seat revises on the same gap scope; re-review; re-present. Design-phase attempts aren't separately bounded in the schema beyond the general no-progress stop (two consecutive unchanged rounds halts and presents state, disposition the user's).
- **User stops here and resumes later** → explicitly permitted; the run pauses with the signed design as the resumption point.

## Phase 5 — Cycle-card authoring and card confirm

**Done:** A design-class seat (never the builder) slices the ratified scope (design output, or — zero-gap branch — spec.md directly) into cycle cards per `mochiko:patterns-vertical-tdd` (foundation cycles before feature cycles; Simple/Split/Merge judgment; walking skeleton first). Given W1 and W2 are both P1, independently testable, and small (create + fetch against SQLite), a plausible slicing is: one foundation cycle (HTTP server + SQLite wiring) then two feature cycles (W1 create, W2 fetch) — but the actual slicing is the authoring seat's judgment, confirmed by the user, not pre-decided here. `qa-engineer` authors the **TEST:** real-infrastructure gate within that slicing.

**Read:** the signed design deltas (or spec.md directly on the zero-gap branch), `plugins/mochiko/schemas/tasks.yaml` (cards template schema, since `mochiko-cli template tasks` binary status is unknown here and the schema is the fallback source of truth).

**Would write:** `.mochiko/features/FEAT-001/tasks.md` — cycle cards carrying stories/rationale, dependencies, acceptance criteria by ID (SC-001, SC-002, SC-003), a **TEST:** gate per card, and brownfield exposure (this is greenfield — first capability, no prior code — so exposure is `[NEW]` throughout, no `[EXTEND]`/`[MODIFY]`).

**Independent review before confirm:** the verification seat (`qa-engineer`) reviews the cards — quality per `review-plan-artifacts`, buildability its own judgment.

**Gate — card confirm (user's), blocking:**
- **User confirms the slicing as presented** → build proceeds card by card in the confirmed order.
- **User asks to re-slice** (e.g., merge W1/W2 into one cycle, or split further) → the authoring seat revises and re-presents; no code is built until this gate clears.
- Building any card before this confirmation, or a card built by the seat that authored it, is itself one of the run's fail conditions (`impl.fail.card-independence`) — the plan enforces the confirm strictly before Phase 6 starts.

## Phase 6 — Build: cycle-by-cycle, test-first, per-cycle verification

**Done, per confirmed card:** `staff-engineer` (never having designed the gap or the card) decomposes the card into concrete tasks at build time — disclosed in the cycle report — and drives red→green→refactor test-first, binding `mochiko:executing-tdd-cycle`, `mochiko:brownfield-integration` (not applicable here — everything is `[NEW]`), and `mochiko:patterns-code-minimalism` at decomposition (rungs disclosed: e.g., "does stdlib `net/http` + `database/sql` cover this before reaching for a framework or ORM" — consistent with D-001/D-002 already ruling that direction).

Independent verification, per cycle: `qa-engineer` (never the implementer) runs `mochiko:testing-end-user` against real infrastructure — a real SQLite file, real HTTP requests, not mocks — plus the `mochiko:review-code-minimalism` lens (advisory only, never gates the cycle) reading the diff, the cycle report, and surrounding code.

**Read per cycle:** the confirmed card in `tasks.md`, the relevant design delta or baseline slice, the growing codebase.

**Would write per cycle:**
- Product code (paths determined by the builder at decomposition — no existing `src/` observed, so this is genuinely greenfield; e.g. an HTTP handler module, a SQLite persistence module, tests alongside)
- `.mochiko/features/FEAT-001/cycle-report.md` (appended, dated, per cycle — decomposition, honest difficulties, deviations, `domain_deps_added`)
- A verification report per cycle in the same directory
- `tasks.md` checkbox flipped `[x]` on pass

**Attempt economy (floor, non-waivable):** each grading round of a cycle consumes one of the `attempt_bound_cycle` (default 3, or the user's run-open override) attempts. Two consecutive unchanged-findings rounds is a no-progress stop — halt that cycle, present state to the user.

**Reserved-to-user items that could interrupt or batch here:**
- An **infeasible card** → escalated as a business-level scope decision, not the builder's or verifier's call.
- A **commodity-category adopt-first ruling** or an **IP-XXX provisioning call** → halts to the user (though D-001/D-002 already pre-rule storage and HTTP layer, so this is unlikely to re-open unless the builder hits an undesigned need).
- **Requirement ambiguity** or a flagged judgment call → goes to the user, batched at the cycle checkpoint unless build-blocking.
- **Undesigned structure discovered mid-build** → halts that cycle; the design phase re-fires scoped to just that discovery, with its own grade and checkpoint (loops back to a mini Phase 4/gate for that slice only).
- **A deviation** (adding/removing a box or arrow, or moving a responsibility across a boundary of any signed delta) → stops and is presented: build as approved, or amend the delta by the user's ruling first. Never silently designed around.
- Non-build-blocking reserved questions **accumulate and land as one batch at the cycle checkpoint**, not one-by-one.

**Cycle checkpoint (recurring, not a single gate):** escalations and findings from however many cycles have landed since the last checkpoint arrive together for the user's ruling; branches are exactly the reserved-item branches above, applied per item.

**Attempt bound exhaustion at a cycle** → halts that cycle, presents state; the disposition (extend, accept partial, abandon that cycle's approach) is the user's, not automatic.

## Phase 7 — Final validation (whole-build pass)

**Done, once, after all cards are `[x]`:**
- **Regression sweep:** run the accumulated durable **TEST:** gate sets of any previously delivered features in this feature's territory. FEAT-001 is the first capability on the map with nothing yet delivered, so this sweep is expected to have an empty prior-gates set to run against — still executed as a formal step, not skipped by assumption.
- **Cold verification:** build and run the full quality-gate suite from a dependency-cold snapshot of the uncommitted working tree, copied via `git ls-files -co --exclude-standard :!.claude/worktrees` into `.claude/worktrees/mochiko-<purpose>/`, after confirming the `.claude/worktrees` ignore entry exists. **This is where the Phase 1 git-absence flag becomes concrete and blocking**: with no git repository present, this literal snapshot mechanic has no `git ls-files` to run. This is presented to the user as a blocking question at this point if it wasn't already resolved at run-open — branches: (a) the user has the run initialize a git repository so the mechanic runs as specified, (b) the user rules an alternative cold-snapshot approach and that ruling is recorded as the evidence basis, or (c) the user accepts a stated limitation, which itself must be weighed against `impl.fail.no-evidence` (verification claimed without real-infrastructure evidence is a fail condition) — meaning option (c) is risky and would need to still produce equivalent real-infrastructure evidence some other way, not merely waive it.
- **Gap-finding pass:** fires because scope is **selection** (`impl.gap-finding-scope`; skipping it here — rather than on delta/lane scope — would itself trip `impl.fail.gap-finding-missing`). Dispatch a fresh `devils-advocate` blind, two-message: first message carries only `spec.md`, `sufficiency-report.md`, any design deltas, and the baselines (`data-model.md`, `contracts/api.yaml`, the store's NFR-001/NFR-002 concern rows) — never the code, `tasks.md`, the **TEST:** cases, or any report. The seat states derived expectations first; only then does probing begin against the real running service.
- **Mutation lens:** only if governance `depth: high` — depth is read from the governance region, which is absent here, so this condition cannot currently resolve to `high`; the plan treats the mutation lens as not firing unless a governance region and depth setting are established before this phase (would be surfaced as part of the absent-governance-region flag from Phase 1).

**Findings routing:** spec-required behavior broken → fails final validation (evidence captured, clause cited). Beyond-spec findings → advisory, disposition reserved to the user (fix now / BACKLOG.md / accept as designed). Disputed finding-kind → defaults advisory, goes to the user, finder never gates alone.

**Would write:** `.mochiko/features/FEAT-001/final-validation-report.md` (states the cold-verification approach used, the regression-sweep result, gap-finding findings and dispositions, or an explicit statement if any step's scope genuinely had nothing to run against).

**Gap-rework bound (floor):** default 2 rounds at run scope (or the user's run-open override); a finding localized to one cycle's territory instead charges that cycle's remaining per-cycle attempts. Exhaustion or an unchanged-findings round halts the run and presents state — disposition the user's.

## Phase 8 — Landing (executed whole, only at user acceptance — see Phase 9)

**Done (selection-scope landing, `impl.landing-selection`):**
- Store landing (only if a store delta exists from Phase 4): the delta's elements flip to built, FEAT-001 keys clear, the touched AX-001/AX-002 (or new) rows get their As-built:/Drift: fields written as judgment and independently graded by the landing verification seat, orphan check runs, derived root `ARCHITECTURE.md` regenerated by the store skill (never hand-edited).
- Graduation batch: W1 and W2 fold into FEAT-001's extent lines and vanish from the entry's pending work-row list; entry status flips to `delivered`, dated; `FEATURES.md` index line updates FEAT-001 to `delivered`; the specs-index row for `note-capture` closes now that all its selected rows have folded.
- Every touched baseline (data-model.md, contracts/api.yaml, constraints-and-decisions.md, the store) folds exactly once via a graded three-way diff, checked by the landing verification seat.
- Gap findings ruled fix-now or backlog fold into a newly minted `.mochiko/features/FEAT-001/gates.md` (authored in **TEST:** grammar by `qa-engineer`); findings accepted as designed do not fold.
- Knowledge-management landing: skipped — `km_file` is absent.

**Would write:** `.mochiko/product/architecture/spine.md` (As-built/Drift fields, if a delta existed), `ARCHITECTURE.md` (regenerated), `.mochiko/features/FEAT-001/entry.md` (status → delivered, extent folded), `FEATURES.md`, `.mochiko/features/FEAT-001/gates.md`, any `BACKLOG.md` entries for beyond-spec findings booked rather than fixed.

**Seats:** the landing verification seat (independent of whoever authored the folded content) checks every graded fold; this run carries no map-delta boundary check beyond the ordinary graduation batch (that check is lane-specific and doesn't apply here).

## Phase 9 — Final acceptance (the closing gate, user's)

**What would be confirmed:** the whole done-condition claim — every card `[x]` and test-first built, per-cycle and whole-build verification evidence (including how the cold-verification git question from Phase 7 was actually resolved), traceability to FR-001…FR-004/SC-001…SC-003, the sufficiency and design records, the final-validation report and its finding dispositions, and the landing package about to execute (or having executed, depending on ordering convention) as one whole.

**Branches:**
- **Accept** → the run closes with a verdict against the done condition; Phase 8's landing executes whole (if not already staged) at this acceptance.
- **Amend** → the user specifies what must change; the relevant upstream phase re-opens (a cycle, the final-validation findings, or even a design-checkpoint reopening if the amendment is structural), and Phase 9 re-fires once that's resolved.
- **Reject** → the run closes without landing; work rows W1/W2 remain unfolded/pending on the map for a future run to pick up.

## Phase 10 — Close: verdict against the done condition

**Done:** State explicitly, against each of the 15 `kind: fail` conditions read in Phase 0, whether it stands (fail) or is clear — sufficiency recorded, design signed if gaps existed, card independence and confirm honored, no card left unchecked, all quality gates passing and never triaged, real-infrastructure evidence present (including how the git-repo gap was actually closed), no regression, no baseline edited in place, no unresolved deviation, complete store landing if applicable, every touched baseline graded-folded, gap-finding pass present (selection scope requires it) and not silently skipped, no unresolved spec-required gap finding, and user acceptance actually given. Any one standing unresolved fails the run regardless of how far the phases above progressed.

**Would write:** nothing beyond what Phases 7–8 already wrote; this is the closing summary delivered to the user, in the register of `templates/output-style.md`, machine-first frontmatter per `templates/report-format.md` for the reports already listed above.