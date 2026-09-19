# Action Plan — `/mochiko:implement FEAT-001` (plan-only, not executed)

**Grounding read already performed** (mandatory first action, done before this plan): `plugins/mochiko/schemas/implement.yaml` (raw, full — 15 `kind: fail` nodes confirmed present, matching the `.md`'s hard-coded count) and `plugins/mochiko/schemas/common.yaml` (raw, full, for the `extends:` stubs) and `plugins/mochiko/schemas/command-labels.yaml`. Also read for entry context: `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/specs/note-capture/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`. Confirmed absent: `CLAUDE.md` / governance region, `.claude/rules/mochiko/`, `.mochiko/memory/knowledge-management.md`, `.mochiko/memory/codebase-analysis.md`, `BACKLOG.md`, `ROADMAP.md`, `.gitignore`, and any product source code — this is a genuinely greenfield capability, and the workspace is not currently a git repository.

---

## Phase 1 — Entry resolution

**Does:** Resolve `FEAT-001` against the map. It is a plain `FEAT-XXX` (not `EPIC-XXX`), so no epic-member lookup fires. `FEATURES.md` lists it as capability "Note capture," status `selected`. Its entry carries two selected work rows with selection-scope provenance (source: the spec's accepted selection, ratified 2026-08-20): W1 — Create a note (US-001, SC-001/SC-002) and W2 — Fetch a note by id (US-002, SC-003). Dependencies line reads "None. First capability on the map," so no dependency-ordering block applies. Scope type resolves to **selection**, not delta or epic — no delta card exists for this capability.

**Reads:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`.

**Writes:** none.

**Seats/skills:** none yet (DM-only resolution).

**Gate:** none — this phase is mechanical routing, not a user decision point.

---

## Phase 2 — Sufficiency check

**Does:** Grade both selected rows against the spec, the architecture store, and the product baselines, per the ten-clause check owned by `mochiko:review-sufficiency`. Run by a seat that authored none of the graded sources and will not design or build this batch — proposed: `mochiko:validator` (generic, non-author grading seat; independence is a hard requirement, not a suggestion). Given what's already on disk — FR-001..004 and SC-001..003 in the spec, AX-001/AX-002 in the architecture spine each carrying an NFR tied to a FEAT-001 success criterion, a matching `Note` entity in `data-model.md`, and both endpoints already contracted in `contracts/api.yaml` — the check is likely to clear cleanly, but the verdict is the grading seat's to render, not assumed here.

**Reads:** `.mochiko/specs/note-capture/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`.

**Writes:** `.mochiko/features/FEAT-001/sufficiency-report.md` — per-row verdict, any trips for the user, the quickstart.md null-path note (no external-integration surface exists here), any store-consult result.

**Seats/skills:** `mochiko:validator` (or equivalent independent grader); pointer skill `mochiko:review-sufficiency`.

**Two onward branches (both must be planned since the verdict isn't known yet):**
- **Branch A — both rows sufficient:** no design phase fires. The card-authoring seat later makes the map-entry assertion the design phase would have made (dependencies/extent, with provenance), surfacing any drift at the card confirm. Flow proceeds straight to Phase 4 after run-open.
- **Branch B — one or more clauses gap out (or a disputed clause defaults to gap):** the design phase fires, scoped to exactly the named gaps — Phase 3a below — before any code is written.

A disputed clause is never cleared by the grader alone; it defaults to gap and is carried into the run-open ruling (Phase 3).

---

## Phase 3 — Run-open confirmation (the entry gate)

**Does:** One blocking confirmation, no negotiation, presented as plain text (never a timed prompt). Assembles:
- Batch identity and scope type: "FEAT-001 — Note capture, selection scope, rows W1 + W2."
- Attempt bounds restated at their only redeclaration point: `attempt_bound_cycle = 3` per cycle, `gap_rework_bound = 2` rounds at run scope — carried as defaults unless the user redeclares here.
- The Phase 2 sufficiency verdict and its gap routing (Branch A or B).
- Trips and conflicts for the user's ruling, surfaced (never auto-resolved, never run-failing):
  - No governance region present (`CLAUDE.md` absent) — the `depth` condition (governance depth) can't resolve from a region; this affects whether the final-validation mutation lens fires later. Flagged for the user to rule on (treat as low/skip, or establish governance first via `/mochiko:setup`).
  - No `knowledge-management.md` — the landing's KM obligations simply don't apply; not a gap, just noted.
  - The workspace is not a git repository — this is a practical blocker for the final-validation cold-verification snapshot (which shells out to `git ls-files -co --exclude-standard`) and for the "suggest commits" duty having anything to suggest against. Surfaced here as a trip needing the user's direction (e.g., `git init` first) rather than silently worked around.
- The done condition, stated plainly: every cycle card checked, built test-first, independently verified against real infrastructure per-cycle and whole; code meets FR-001..004/SC-001..003, traces to requirements, aligns with governance (none present, noted as n/a); acceptance landing executed whole; run closes only at final accept/amend/reject.

**Reads:** Phase 2's sufficiency-report.md content (in-memory at this point, not yet re-read from disk).

**Writes:** none yet — this confirmation precedes any write beyond the sufficiency report already landed in Phase 2.

**Seats/skills:** none — this is the DM's own duty (`impl.dm-entry-gate`).

**Gate — what's confirmed:** batch + scope, attempt-bound values (as stated or as redeclared), the sufficiency verdict and its routing, and every listed trip.
- **User confirms as-is:** bounds stand at their defaults; proceed to Phase 3a (Branch B) or directly to Phase 4 (Branch A).
- **User redeclares an attempt bound:** the new value binds for the rest of this run; same onward routing.
- **User disputes the sufficiency verdict:** a disputed clause defaults to gap (never the other direction) — this can only add a gap to Branch B's scope, never clear one the grader found.
- **User rules on the git-repo trip:** either directs `git init` (or otherwise not-a-git-repo handling) before build starts, or explicitly accepts a degraded cold-verification path — either ruling is recorded and carried into Phase 6/7.
- **User redirects the batch itself** (wrong capability, wrong rows): run does not open as scoped; re-routes per the Adaptive Goal Protocol's entry rules (not expected here, since FEAT-001 resolves cleanly, but the branch exists).

---

## Phase 3a — Design phase (fires only under Branch B)

**Does:** Authors exactly the named gaps, nothing more, each on a DM-approved plan, rung-justified per `mochiko:patterns-plan-minimalism`. Proposed staffing (DM latitude, not fixed by the schema): `technical-analyst` (the `design_seat` var) for spec/data-model/contract-shaped gaps; `principal-architect` (`architect_seat` var) only if a store (`AX-XXX`) delta is actually implicated; `qa-engineer` for any design-time **TEST:** case gap. The builder (`staff-engineer`) never designs its own gaps. If the gap check surfaces an absent baseline, the phase's first duty is seeding an empty scaffold before anything else, confirmed with the user at the checkpoint.

**Reads:** `.mochiko/features/FEAT-001/sufficiency-report.md`, the current baselines at `.mochiko/product/` (`data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `architecture/spine.md`), `.mochiko/specs/note-capture/spec.md`.

**Writes:** Design deltas beside their baselines at `.mochiko/features/FEAT-001/` (e.g., a `data-model.md` delta, a `contracts/` delta, a prose before/after delta for `constraints-and-decisions.md`), plus an architecture-store delta only if the structural trigger fired.

**Seats/skills:** `technical-analyst` and/or `principal-architect` (producers, plan-approved); non-author review pair before the checkpoint — proposed `mochiko:validator` running `mochiko:review-plan-artifacts` (conformance to the gap list, card quality — blocking) and `mochiko:tech-lead` running `mochiko:review-feasibility` (contradiction/buildability).

**Gate — design checkpoint (floor, user's):** presents the rendered delta (or source-plus-changed-row table where no render surface exists) and any store delta's changed `AX-XXX` rows.
- **User signs:** the design (and store delta, written only as in-flight-class elements) stands; proceed to Phase 4.
- **User stops here:** the run pauses cleanly; explicitly resumable later without penalty.
- **User requests amendment:** the design seat reworks within this phase before re-presenting; no code has been written yet, so no rebuild is implicated.

---

## Phase 4 — Cycle-card authoring

**Does:** A design-class, non-builder seat (proposed: `technical-analyst`) slices W1 and W2 into cycle cards per `mochiko:patterns-vertical-tdd` — Simple/Split/Merge judgment, walking skeleton first. Plausible shape given the two stories (a minimal HTTP+SQLite skeleton, then Create-a-note, then Fetch-a-note-by-id) — but the actual slicing is that seat's call, not prescribed here. `qa-engineer` authors the **TEST:** real-infrastructure gate within that slicing (e.g., POST/GET round-trip + empty-text-400 for the create cycle; GET-existing + GET-missing-404 for the fetch cycle). Each card carries stories/rationale, dependencies, acceptance criteria by ID (FR-00x/SC-00x), the **TEST:** gate, and brownfield exposure (`[NEW]` throughout, since no code exists yet). Under Branch A (zero-gap), this same seat also makes the map-entry assertion the design phase would have made.

**Reads:** `.mochiko/features/FEAT-001/entry.md`, `.mochiko/specs/note-capture/spec.md`, any design deltas from Phase 3a, `plugins/mochiko/schemas/tasks.yaml` (or the rendered template if `mochiko-cli` is available).

**Writes:** `.mochiko/features/FEAT-001/tasks.md` (cycle cards, checkboxes unflipped).

**Seats/skills:** `technical-analyst` (card author), `qa-engineer` (TEST: cases); independent review before confirm by a verification seat (proposed: `qa-engineer` acting in its verification capacity, or `mochiko:validator`) against `mochiko:review-plan-artifacts` for quality plus its own buildability judgment.

**Gate — card confirm (floor, user's):** the user rules the slicing before any build.
- **Approve as-sliced:** proceed to Phase 5.
- **Request re-slicing** (merge/split cycles differently): the card-authoring seat revises, the verification seat re-reviews, re-confirm.
- **Reject/defer:** the run pauses here; rows stay unbuilt pending further direction.

---

## Phase 5 — Build (test-first, foundation before feature)

**Does:** For each confirmed card, in order (foundation/skeleton cycle first if the slicing named one, then Create-a-note, then Fetch-a-note-by-id): `staff-engineer` decomposes the card into concrete tasks at build time (disclosed in the cycle report), runs the pre-code minimalism ladder per `mochiko:patterns-code-minimalism`, and drives each task red→green→refactor per `mochiko:executing-tdd-cycle`. `mochiko:brownfield-integration` applies from the second cycle onward, once the first cycle has put a real `api-service`/`notes-db` on disk to extend. The card's checkbox in `tasks.md` flips only once its cycle is done.

Mid-build routing carried forward from the schema, applied as it arises rather than pre-resolved:
- Undesigned structure discovered mid-cycle halts that cycle and re-fires a scoped design phase (Phase 3a re-entry), then resumes.
- An infeasible card escalates to the user as a business-scope decision, never builder-decided.
- Any structural deviation from a signed delta (box/arrow/responsibility move) stops and is presented — build as approved or amend by the user's ruling; never silent. (Low likelihood here given the architecture is already `ruled` and stable for two CRUD-shaped endpoints, but the rule stands regardless.)
- A commodity-category decision not already covered by D-001 (SQLite)/D-002 (stdlib HTTP) — e.g. a UUID-generation approach — routes to the user per `mochiko:patterns-adopt-first`; anything colliding with C-001 (single-process, no external services) files a constraint-challenge finding and only that decision pauses.

**Reads:** `.mochiko/features/FEAT-001/tasks.md`, the confirmed cards, current baselines, and (from the second cycle on) the code already produced by prior cycles.

**Writes:** product source code (new — no existing tree to touch), `.mochiko/features/FEAT-001/cycle-report.md` per cycle (decomposition, difficulties, deviations, domain deps added), `tasks.md` checkbox flips.

**Seats/skills:** `staff-engineer` (builder); a verification seat (proposed `qa-engineer`, never the implementer) executing the **TEST:** gate against real infrastructure via `mochiko:testing-end-user`, plus the advisory `mochiko:review-code-minimalism` lens over the diff, the cycle report, and the surrounding code.

**Per-cycle checkpoint:** escalations and findings from that cycle land as one batch. Important-or-above findings block the cycle; Minor findings default to a `BACKLOG.md` booking (minted fresh, via `mochiko:grooming-operating-docs`, since none exists yet). Attempt economy: every grading pass burns one of the 3 (or user-redeclared) per-cycle attempts; two consecutive rounds with unchanged findings force a no-progress stop, halting the cycle for the user's disposition — exempting a round from the count is the user's call only, never the run's.

*(No user-input wait is modeled here in this plan-only run — each checkpoint's onward branches are: proceed once findings clear / halt and present state on exhaustion or no-progress, with disposition reserved to the user in the latter case.)*

---

## Phase 6 — Final validation

**Does, in order:**
1. Full repository quality-gate suite — never severity-triaged; any failure fails the run outright.
2. Regression sweep over accumulated `**TEST:**` gates of previously delivered features in this territory. FEAT-001 is the first capability on the map, so this sweep is vacuous (zero prior `gates.md` files) but still explicitly run and stated as such, not silently skipped.
3. Cold verification: build and gate from a dependency-cold snapshot of the uncommitted working tree (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-<purpose>/`), after ensuring a `/.claude/worktrees` ignore entry exists. **This step depends on the Phase 3 git-repo trip having been resolved** — if the user didn't direct `git init` there, this step cannot execute as specified and would need to be re-escalated here rather than silently worked around.
4. Gap-finding pass (selection scope → required, per `impl.gap-finding-scope`/`impl.fail.gap-finding-missing`): a fresh `devils-advocate` seat, never having built these cycles or seen the design-time TEST cases, is dispatched blind in two messages. First message carries only `spec.md`, `sufficiency-report.md`, any design deltas, and the baselines (`data-model.md`, `contracts/api.yaml`, the architecture spine's NFR-bearing rows) — never code, `tasks.md`, TEST cases, or reports. The seat states its derived expectations before probing the running service. The mutation lens fires only if `depth: high` resolves — flagged in Phase 3 as unresolved absent a governance region, so this depends on that trip's ruling; if it stays unresolved, the skip is explicitly disclosed rather than silently omitted.
5. Findings split: spec-required behavior broken (evidence + clause cited) fails final validation; beyond-spec findings are advisory, disposition reserved to the user (fix now / `BACKLOG.md` / accept as designed). A disputed finding kind defaults advisory and goes to the user.

**Reads:** the full built code tree, `spec.md`, `sufficiency-report.md`, design deltas, baselines, all cycle reports and verification reports.

**Writes:** `.mochiko/features/FEAT-001/final-validation-report.md`.

**Seats/skills:** `devils-advocate` (blind gap-finder, per `mochiko:testing-gap-finding`); the verification seat from Phase 5 for the mutation lens if it fires.

**Gap-rework bound:** default 2 rounds at run scope (a finding localized to one cycle's territory instead charges that cycle's remaining attempts). Exhaustion or an unchanged-findings round halts the run and presents state; disposition is the user's.

---

## Phase 7 — Landing

**Does:** Executes the acceptance landing whole, in one pass, once final validation clears:
- Store landing: any built structural change flips its architecture-store elements to `built`, clears their `FEAT-001` keys, writes graded `As-built:`/`Drift:` on the touched `AX-001`/`AX-002` rows, runs the orphan check, and regenerates the derived `ARCHITECTURE.md` (never hand-edited). Given both concern rows are already `ruled` rather than in-flight, this is likely a confirmation/no-structural-change pass, but it still runs and is graded.
- Any build-time technical decision discovered along the way (a new `D-XXX`/`C-XXX`/`IP-XXX`) was already written as `.mochiko/features/FEAT-001/baseline-delta.md` at the point of discovery in Phase 5/6; here its judgment content gets independently graded and it folds via a three-way diff against `constraints-and-decisions.md`.
- KM landing: skipped — no `knowledge-management.md` exists, so this obligation simply doesn't apply.
- Selection-scope map graduation: W1 and W2 fold into FEAT-001's extent lines and vanish as pending rows; entry status flips to `delivered`, dated; the `FEATURES.md` index line updates; the note-capture spec's index row is touched, reading closed once both rows have folded.
- Gap findings ruled fix-now or backlog fold into `.mochiko/features/FEAT-001/gates.md` (minted here, first gate set for this capability), authored in the `**TEST:**` grammar by `qa-engineer`; anything accepted-as-designed does not fold.

**Reads:** the final-validation report, all design/baseline deltas, the current architecture spine, `FEATURES.md`, the feature entry.

**Writes:** the graded folds into `.mochiko/product/architecture/spine.md` (and regenerated `ARCHITECTURE.md`), `.mochiko/product/constraints-and-decisions.md` (if a baseline-delta exists), `.mochiko/features/FEAT-001/entry.md` (status/extent), `FEATURES.md` (status column), `.mochiko/features/FEAT-001/gates.md`.

**Seats/skills:** an independent landing-verification seat (proposed `mochiko:validator`) checks every graded fold; `qa-engineer` authors folded gate entries; pointer skills `mochiko:authoring-architecture-store` and `mochiko:authoring-feature-map` own the grammar.

**Gate:** none standalone here — landing executes as one unit; its output feeds directly into Phase 8's acceptance gate. (No separate user ruling inside landing itself, per the "landing whole" duty.)

---

## Phase 8 — Final acceptance

**Does:** Closes the run against the done condition. Presents: the fully checked `tasks.md`, per-cycle and whole-build verification evidence, the final-validation report, the landing's fold/graduation summary, any accepted beyond-spec dispositions, and any bookings sitting in `BACKLOG.md`.

**Reads:** everything produced across Phases 1–7.

**Writes:** none new — this is the closing ruling itself.

**Seats/skills:** none — DM's own duty to close with a verdict (`impl.dm-close-verdict`).

**Gate — final acceptance (floor, user's), plain blocking text:**
- **Accept:** the run closes; FEAT-001 stands `delivered`; done condition satisfied.
- **Amend:** user specifies the change; the run reopens the implicated phase (a cycle rework, or a freshly scoped design re-fire), bounded by the same attempt/gap-rework economy, then re-lands and re-presents for acceptance.
- **Reject:** the run does not close; state and options (rework this run vs. hold it open) are presented explicitly — left unresolved, this is itself one of the 15 Not-done conditions (`impl.fail.no-acceptance`), so it is never silently treated as done.

---

## Phase 9 — Not-done audit (before declaring closed)

**Does:** Walks all 15 `kind: fail` rules under `impl.sec.fail-conditions` (sufficiency-unrecorded, design-skipped, card-independence, card-unchecked, quality-gate, no-evidence, regression, baseline-in-place, deviation-unresolved, store-landing-incomplete, ungraded-fold, gap-finding-missing, skip-unstated, spec-gap-unresolved, no-acceptance) and confirms none stand. The count was already cross-checked against the schema (15) as part of loading it in the first action; if that ever drifted, the run would halt and surface it before closing rather than proceeding silently.

**Reads:** the schema's fail-condition list (already loaded) against the accumulated run state.

**Writes:** none — this is a verdict check, folded into the Phase 8 closing message.

**Seats/skills:** none.

**Gate:** none additional — this audit is what makes Phase 8's "accept" verdict honest, not a separate user-facing checkpoint.