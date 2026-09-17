# Action Plan — `/mochiko:implement FEAT-001` (plan-only, not executed)

## Phase 0 — Rule load (source-of-truth intake)

- **Does:** Reads the two schema files raw and in full, as the command's own first-action instruction requires, before any gating or seat work.
- **Reads:** `plugins/mochiko/schemas/implement.yaml` (six sections: `impl.sec.roles`, `impl.sec.reserved`, `impl.sec.tools`, `impl.sec.ways-of-working`, `impl.sec.boundaries`, `impl.sec.fail-conditions`); `plugins/mochiko/schemas/common.yaml` (for the `extends: common.register` stub); `plugins/mochiko/schemas/command-labels.yaml` for label vocabulary.
- **Verifies:** the `kind: fail` node count in `impl.sec.fail-conditions` is exactly 15 (it is — counted directly from the file: sufficiency-unrecorded, design-skipped, card-independence, card-unchecked, quality-gate, no-evidence, regression, baseline-in-place, deviation-unresolved, store-landing-incomplete, ungraded-fold, gap-finding-missing, skip-unstated, spec-gap-unresolved, no-acceptance). No halt triggered.
- **Writes:** nothing.
- **Seats/skills:** none — this is the delivery manager's own read.
- **Gate:** none.

## Phase 1 — Entry resolution for `FEAT-001`

- **Does:** Resolves `FEAT-001` against the feature map. It is a plain capability ID (not an `EPIC-XXX`), so no epic member-lookup branch fires.
- **Reads:** `FEATURES.md` (capability row, status `selected`); `.mochiko/features/FEAT-001/entry.md` (extent, architecture link `AX-001`/`AX-002`, two selected work rows W1 "Create a note" and W2 "Fetch a note by id", both unchecked, no unmet dependencies — "None. First capability on the map."); `.mochiko/specs/note-capture/spec.md` (status `accepted`, selection ratified 2026-08-20 — this is **selection scope**, not a delta card).
- **Checks:** no selected row depends on an undelivered row (dependency list is empty) → no dependency-order block.
- **Writes:** nothing yet.
- **Seats/skills:** none — DM's own resolution.
- **Gate:** none at this sub-step; feeds the sufficiency check next.

## Phase 2 — Sufficiency check (binding verdict)

- **Does:** Grades each selected row (W1, W2) against the spec, the architecture store, and the product baselines, per `mochiko:review-sufficiency`'s clause set. The grading seat must have authored none of the graded sources — since the spec, data model, contracts, constraints, and architecture spine already exist (presumably authored by requirements-analyst / technical-analyst / principal-architect during a prior `/mochiko:specify` run), the grader is staffed as an outsider to those artifacts — e.g. `mochiko:validator` or `mochiko:tech-lead`, exempt from plan approval as a grading seat.
- **Reads:** `.mochiko/specs/note-capture/spec.md` (FR-001..004, SC-001..003); `.mochiko/product/data-model.md` (Note entity); `.mochiko/product/contracts/api.yaml` (POST /notes, GET /notes/{id}); `.mochiko/product/constraints-and-decisions.md` (C-001, D-001, D-002); `.mochiko/product/architecture/spine.md` (AX-001 persistence NFR-001, AX-002 logging NFR-002, AX-003 n/a).
- **Observed surfaces (informational, not pre-judged):** the baselines here look materially complete for both rows — persistence, HTTP contract, and data model are all already ruled — so a `sufficient` verdict on both rows is plausible, though the actual grading is the sufficiency seat's judgment call, not the DM's.
- **Also surfaces (per `impl.absent-surfaces`, never auto-resolved, never run-failing):**
  - No `CLAUDE.md` governance region and no `.claude/rules/mochiko/` files exist → surfaced to the user as a missing governance region (this also leaves the `depth` condition — low/high — unresolved at entry, since it reads from the governance region; flagged as an open item rather than defaulted).
  - No `.mochiko/memory/codebase-analysis.md` — but there is no existing source tree either (greenfield: no `src/`, no code files found), so the correct branch is "proceed greenfield with the warning logged," not an offer to run `/mochiko:setup`.
  - No `.mochiko/memory/knowledge-management.md` → `km_file` reads absent; the landing's knowledge-management obligations (Phase 8) do not apply this run.
- **Writes:** `.mochiko/features/FEAT-001/sufficiency-report.md` — the binding per-row verdict, any trips, the quickstart.md null-path note (no external-integration surface exists here), and any `[MODIFY]` amendment claim (none expected — no prior delivered feature exists to amend).
- **Seats/skills:** independent sufficiency grader (`mochiko:validator` or `mochiko:tech-lead`, per `impl.seat-sufficiency-independence`); pointer skill `mochiko:review-sufficiency`.
- **Gate:** a disputed clause the grader cannot clear alone defaults to a gap and routes to the user at run-open (Phase 3) rather than being silently resolved here.

## Phase 3 — Run-open confirmation (USER GATE)

- **Does:** Closes entry with one confirmation, no negotiation. States: the batch (`FEAT-001`, work rows W1+W2) and scope type (selection); both attempt bounds at their only redeclaration point — per-cycle verification default `3`, gap-rework default `2`; the sufficiency verdict and any gap routing from Phase 2; the trips/conflicts surfaced so far — missing governance region, unresolved `depth`, and the fact that **this is not a git repository**, which blocks `impl.cold-verification`'s dependency-cold snapshot step at final validation (Phase 7) unless remediated; and the done condition (every cycle card `[x]`, test-first, independently verified per-cycle and whole, criteria traced, governance-aligned, landing executed whole, closed at final acceptance).
- **Reads:** nothing new — assembles Phase 1/2 outputs.
- **Writes:** nothing (the confirmation itself is not a file write; it becomes the anchor other reports cite).
- **Seats/skills:** none — DM-authored, user-facing.
- **Gate — what is confirmed:** the batch/scope statement, the attempt bounds, the sufficiency verdict + gap routing, and disposition of the trips (governance region, `depth`, no-git-repo).
  - **Ruling: confirm as-is (accept default bounds, verdict, and trip dispositions).** → If the sufficiency verdict carried no gaps: proceed directly to Phase 5 (cycle cards). If it carried gaps: proceed to Phase 4 (design phase). The no-git-repo trip, if accepted "proceed anyway," is carried forward as a known final-validation blocker to be re-raised at Phase 7 rather than resolved now.
  - **Ruling: redeclare attempt bounds (e.g., raise the 3/2 defaults).** → New bounds are recorded here (the only point they may be redeclared) and used for the rest of the run; otherwise proceeds as above.
  - **Ruling: rule on a trip differently** — e.g., authorize `git init` on this directory before build starts, or explicitly waive cold verification for this run. → Since `git init` is a repo-initializing action, it would itself be confirmed as a discrete step before running (not bundled silently into "confirm"); once ruled, the ruling is recorded and downstream phases follow it.
  - **Ruling: contest the sufficiency verdict itself (dispute a clause).** → Per `impl.sufficiency-disputed-clause`, a disputed clause defaults to gap; the run re-enters Phase 2's gap list with the disputed row added, then re-presents run-open.
  - **Ruling: halt / decline to proceed.** → The run stops here; nothing downstream fires; the sufficiency report stands as the artifact of record for a future resumption.

## Phase 4 — Design phase (conditional — fires only if Phase 2/3 named a gap)

- **Does:** Authors exactly the named gaps, nothing more, each on a plan the DM approved, rung-justified per `mochiko:patterns-plan-minimalism`. Given the baselines observed in Phase 2 (contract, data model, constraints, and architecture spine already covering both rows), the concrete, scenario-likely outcome is **zero gaps**, in which case this phase is skipped entirely and `impl.zero-gap-map-assertion` applies instead: the card-authoring seat (Phase 5) makes the map-entry assertion the design phase would have made, surfacing any drift at the card confirm. This phase is described here for completeness in case Phase 2/3 does name a gap (e.g., a disputed clause).
- **Reads (if fired):** `sufficiency-report.md`; the same product baselines as Phase 2; `spec.md` for cited acceptance criteria.
- **Writes (if fired):** deltas beside the touched baselines at `.mochiko/features/FEAT-001/` (e.g. a `data-model.md` delta, a `contracts/` delta, a prose before/after delta) — never edited in place; a store delta only if a structural trigger fired, per `mochiko:patterns-system-design` / `mochiko:authoring-architecture-store`; the design phase also asserts sharpened dependencies/extent onto `entry.md` with provenance.
- **Seats/skills:** design seats per `impl.design-seats-staffing` — `technical-analyst` for a design delta, `principal-architect` for a store delta, `qa-engineer` for **TEST:** cases; independent review pair before checkpoint — `mochiko:review-plan-artifacts` (conformance to the gap list, blocking) and `mochiko:review-feasibility` (buildability/contradiction), run by a non-author seat (e.g. `validator` for the former, `tech-lead` for the latter).
- **Gate — design checkpoint (USER GATE, floor):** what is confirmed — the drafted design deltas and, where one exists, the store delta (presented as a rendered diagram plus its named `AX-XXX` row changes, or source + changed-element table if no render surface).
  - **Ruling: sign as drafted.** → No code has been written yet; proceeds to Phase 5.
  - **Ruling: request revision.** → Design seat reworks the named gap only, re-reviewed by the same independent pair, re-presented; not itself bounded by the final-validation gap-rework counter (this is pre-code).
  - **Ruling: stop here.** → The user may stop at the checkpoint and resume the build later; the run pauses with the design deltas as the artifact of record.

## Phase 5 — Cycle card authoring + card confirm

- **Does:** A design-class, non-builder seat (`technical-analyst`) slices W1 and W2 into cycle cards, test-first, foundation cycles before feature cycles. The concrete likely shape given the dependency (fetch requires a stored note to exist): **Cycle 1 — create-and-persist a note** (W1, US-001, SC-001/SC-002, FR-001/FR-002/FR-004) as the foundation/walking-skeleton cycle; **Cycle 2 — fetch a note by id** (W2, US-002, SC-003, FR-003) as the feature cycle built on it. Each card carries stories/rationale, dependencies, acceptance criteria by ID, a `**TEST:**` real-infrastructure gate (`qa-engineer`-authored within the slicing), and brownfield exposure (here: none — everything is new code, greenfield).
- **Reads:** `sufficiency-report.md`, any Phase 4 design deltas (if produced), `spec.md`, product baselines.
- **Writes:** `.mochiko/features/FEAT-001/tasks.md` — rendered from the tasks template (`mochiko-cli template tasks`, or `plugins/mochiko/schemas/tasks.yaml` read raw if the binary is absent).
- **Seats/skills:** `technical-analyst` (card author) + `qa-engineer` (TEST case authoring within slicing); pointer skills `mochiko:patterns-vertical-tdd` (slicing/TEST grammar), `mochiko:authoring-feature-map` (zero-gap map assertion if Phase 4 was skipped). Card review before confirm: verification seat (`qa-engineer`) grades quality (`mochiko:review-plan-artifacts`) and buildability (own judgment).
- **Gate — card confirm (USER GATE, floor):** what is confirmed — the two-cycle slicing, its ordering, and each card's acceptance-criteria mapping, before any card is built.
  - **Ruling: approve the slicing.** → Build begins at Phase 6, Cycle 1.
  - **Ruling: request re-slicing** (e.g., split further, merge, reorder). → `technical-analyst` revises, re-reviewed by the verification seat, re-presented; no code exists yet, so this has no attempt-bound cost.
  - **Ruling: pause/reject.** → Run halts before any build task starts; `tasks.md` stands as the record.

## Phase 6 — Build execution (per confirmed cycle, test-first)

- **Does, per cycle:** `staff-engineer` decomposes the card into concrete tasks at build time (disclosed in the cycle report), runs the pre-code minimalism ladder (`mochiko:patterns-code-minimalism`, rungs disclosed — relevant here given D-002's "no framework" decision and D-001's "no hand-rolled storage"), and builds red→green→refactor on a DM-approved plan (`mochiko:executing-tdd-cycle`). Cycle 1 stands up the SQLite-backed Note persistence and `POST /notes`; Cycle 2 adds `GET /notes/{id}`.
- **Verifies, per cycle:** an independent verification seat (`qa-engineer`, never the implementer) runs `mochiko:testing-end-user` against real infrastructure — actual HTTP calls against the running stdlib server, actual SQLite file reads/writes, actual process-restart check for FR-001/SC-001 — plus the `mochiko:review-code-minimalism` lens (advisory) over the diff and cycle report. Attempt economy: `3` grading attempts per cycle (or the run-open-redeclared bound); two consecutive rounds with unchanged findings triggers a no-progress halt presenting state to the user.
- **Reads:** `tasks.md` (the card being built), the design deltas/baselines from Phase 4/5, existing code from any already-built cycle (Cycle 2 reads Cycle 1's persistence layer as existing code — but since it was built *within this same run*, this is same-run continuation, not the `mochiko:brownfield-integration` "existing code on disk from a prior run" case).
- **Writes:** the actual application source (a `src/`-equivalent tree for the note-capture service — path chosen by the builder, since none exists yet); `.mochiko/features/FEAT-001/cycle-report.md` per cycle (decomposition, difficulties, deviations, `domain_deps_added`); per-cycle verification report under the same feature dir; `tasks.md` checkbox flips (`[x]`) as the progress surface, per cycle.
- **Escalation handling:** reserved-to-user items (an infeasible card, an adopt-first/IP-XXX call, requirement ambiguity, a scope escalation, a constraint challenge against C-001/D-001/D-002, a disputed finding kind) accumulate and land as one batch at the cycle checkpoint, unless build-blocking, in which case they interrupt mid-cycle immediately. None of these are anticipated given how settled the baselines are, but the mechanism is live for the whole build.
- **Gate — cycle checkpoint (USER GATE, batched):** what is confirmed — any accumulated escalations/findings for the cycle just graded, plus rounds/seats consumed (`impl.dm-surface-rounds`).
  - **Ruling: no open escalations, cycle passed.** → Proceeds to the next cycle, or to Phase 7 once both cycles are `[x]`.
  - **Ruling: resolves an escalation** (e.g., approves a scope addition, rules a constraint challenge, exempts a round from the attempt count). → Build continues under that ruling; an exempted round is explicitly the user's call, never the run's own.
  - **Ruling: carries a no-progress or attempt-exhaustion halt.** → Presented state; the user decides to grant more attempts (their call only), accept the cycle as-is if criteria are otherwise met, or stop the run with the cycle's state as record.

## Phase 7 — Final validation (whole-build verification)

- **Does:** Runs the accumulated `**TEST:**` gates of previously delivered features in this territory — none exist yet (FEAT-001 is the first capability on the map), so this reduces to running FEAT-001's own gates plus the full quality-gate suite. Runs a dependency-cold build from a snapshot of the uncommitted working tree via `git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-<purpose>/`. **This step is currently blocked**: the workspace is not a git repository, so there is no ref/index to snapshot from — this is re-raised here as the trip flagged at run-open (Phase 3) for the user's ruling if not already resolved there. Because scope is `selection`, the blind gap-finding pass fires (`impl.gap-finding-scope`): a fresh `devils-advocate` seat, dispatched two-message and blind — first message carries only `spec.md`, `sufficiency-report.md`, design deltas (if any), and the baselines (data-model.md, contracts/, the store's NFR rows) — never the code, `tasks.md`, TEST cases, or reports — states derived expectations, then probes.
- **Reads:** as above; also the store's NFR-001/NFR-002 rows for concern-targeted probing.
- **Writes:** `.mochiko/features/FEAT-001/final-validation-report.md` (regression sweep result, cold-verification result or its blocking status, gap-finding findings split by kind — spec-required-behavior-broken vs. beyond-spec advisory).
- **Seats/skills:** `devils-advocate` (blind gap finder, `mochiko:testing-gap-finding`); quality gates run as the full repository suite (`impl.gates-full-suite`, never severity-triaged).
- **Gate (folded into the batched checkpoint/escalation mechanism, not a separate named moment):** any spec-required-behavior-broken finding fails the run's final validation outright; a beyond-spec finding's disposition (fix now / BACKLOG.md / accept as designed) is the user's call, batched with any other open escalations.
  - **Branch if cold verification remains blocked (no git repo) and the user has not authorized a remediation:** this is presented as an unresolved final-validation prerequisite — not silently skipped — with the disposition (authorize `git init` now, accept an alternate frozen-copy method, or accept the run stops short of this evidence) reserved to the user.

## Phase 8 — Landing (selection-scope)

- **Does:** Folds every touched baseline exactly once via a graded fold (three-way diff, checked by the landing verification seat — an independent, non-author seat, e.g. `tech-lead` or `validator`). For selection scope: the store landing (if a store delta was signed in Phase 4 — none expected here since no structural change was anticipated) plus the map's graduation batch — W1/W2 fold into FEAT-001's extent lines and vanish from the "selected" row list, `entry.md` status flips to `delivered` (dated), the `FEATURES.md` index line updates, and the spec's closure is derived (closes exactly when all its selected rows have folded — here, both W1 and W2, so `note-capture` spec closes). Gap findings ruled fix-now or backlog fold into `.mochiko/features/FEAT-001/gates.md` (minted fresh, since it doesn't exist yet), authored by `qa-engineer` in the `**TEST:**` grammar; findings accepted as designed do not fold.
- **Reads:** all Phase 4–7 artifacts; current `FEATURES.md`, `entry.md`, architecture spine.
- **Writes:** `.mochiko/features/FEAT-001/entry.md` (status → delivered, extent folded), `FEATURES.md` (status column), `.mochiko/features/FEAT-001/gates.md` (new), architecture store fold (conditional — only if Phase 4 signed a delta), any `baseline-delta.md` grading close-out.
- **Seats/skills:** landing verification seat (independent of every producing seat this run) per `impl.landing-verifier-folds`; pointer `mochiko:authoring-feature-map`, `mochiko:authoring-architecture-store`.
- **Gate:** landing executes whole only at user acceptance (Phase 9) — it does not fire ahead of that ruling; described here as the prepared, not-yet-executed content of that acceptance action.

## Phase 9 — Final acceptance (USER GATE) and run close

- **Does:** Presents the whole-run evidence — every cycle `[x]`, both per-cycle and whole-build real-infrastructure verification, criteria traced to FR/SC IDs, governance alignment (noting the absent governance region from Phase 2/3), the prepared landing content from Phase 8 — as plain blocking text, never a timed prompt.
- **Reads:** the full run's report set.
- **Writes:** nothing until ruled.
- **Gate — what is confirmed:** accept / amend / reject against the done condition.
  - **Ruling: accept.** → Phase 8's landing executes whole; the run closes with a verdict against the done condition, checked against the 15-item Not-done set (any standing item would instead force a FAIL verdict even on an "accept" ruling — the DM's closing duty is to check this, not just relay the user's word).
  - **Ruling: amend.** → Names the specific rework; charges the gap-rework bound (default `2`, or the run-open-redeclared value) if it's whole-run-scoped, or the owning cycle's remaining per-cycle attempts if it localizes to one cycle's territory; loops back to the relevant phase (6 or 7) and re-presents acceptance afterward.
  - **Ruling: reject.** → Run closes without landing; state is preserved (cycle reports, tasks.md, final-validation report) as the record for a future resumption; verdict is Not-done (`impl.fail.no-acceptance`).