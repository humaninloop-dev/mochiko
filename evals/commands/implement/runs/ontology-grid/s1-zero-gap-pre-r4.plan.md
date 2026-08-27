# Action Plan — `/mochiko:implement FEAT-001` (plan-only, nothing executed)

## Ground truth read so far (no writes performed)

- `plugins/mochiko/schemas/implement.yaml` read raw, in full (135 rules across 6 sections, `vars:` block resolved: `attempt_bound_cycle=3`, `gap_rework_bound=2`, `builder_seat=staff-engineer`, `design_seat=technical-analyst`, `architect_seat=principal-architect`, `qa_seat=qa-engineer`, `gap_finder_seat=devils-advocate`, `explore_model=haiku`). Fail-condition count in `impl.sec.fail-conditions` = 15, matching the command's hard-coded Not-done count — no halt triggered.
- `FEATURES.md`: single capability, `FEAT-001 | Note capture | selected`.
- `.mochiko/features/FEAT-001/entry.md`: status `selected`, spec linked, architecture link `AX-001, AX-002`, work rows `W1` (create, US-001, SC-001/SC-002) and `W2` (fetch, US-002, SC-003), both selection-source-ratified 2026-08-20, no dependencies (first capability on the map).
- `.mochiko/specs/note-capture/spec.md`: accepted, US-001/US-002 with Given/When/Then, FR-001..FR-004, SC-001..SC-003, edge cases (10k-char text, concurrent-create id uniqueness). No UX surface.
- `.mochiko/product/data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md` (C-001 single-process, D-001 SQLite adopt-first, D-002 stdlib HTTP), `architecture/spine.md` (AX-001 persistence/NFR-001, AX-002 logging/NFR-002, AX-003 auth n/a) — all present and ruled.
- No `.claude/rules/mochiko/`, no `CLAUDE.md` → **no governance region exists**.
- No `.mochiko/memory/` directory at all → no `codebase-analysis.md`, no `knowledge-management.md`, no `governance-intent.md`.
- No application source anywhere in the repo → this is a **greenfield** build (codebase-analysis absence is therefore not itself a gap).
- Root `ARCHITECTURE.md` exists (derived index) — the store already carries ruled content, so the "store with no ruled content" absent-surface branch does not apply.
- Environment is **not a git repository** — relevant to the cold-verification step later.

---

## Phase 1 — Entry & scope resolution
**Does:** Resolve `FEAT-001` as a capability ID (not an `EPIC-XXX`), confirm it is not empty, confirm selection scope (spec's accepted selection) rather than delta scope (no `/mochiko:feature` delta card exists for this capability). Check dependency-order blocking: entry declares no dependencies and this is the first map row, so nothing blocks.
**Reads:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `.mochiko/specs/note-capture/spec.md`.
**Writes:** none.
**Seats/skills:** Delivery Manager only (no dispatch yet).
**Gate:** none.

## Phase 2 — Sufficiency check
**Does:** Grade each selected row (W1, W2) against the spec, the architecture store, and the product baselines per the ten-clause check, via `mochiko:review-sufficiency`. Staffed to a seat that authored none of the graded sources — `technical-analyst` and `principal-architect` are disqualified (likely authors of the baselines/store), so this would be staffed to **qa-engineer** (independent, not yet touching design or build). Also surfaces the absent governance region (`.claude/rules/mochiko/`, `CLAUDE.md`) as a non-failing, non-auto-resolved item.
**Reads:** the spec, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `architecture/spine.md`.
**Writes:** `.mochiko/features/FEAT-001/sufficiency-report.md` — per-row verdict (`sufficient` or a named gap list), the governance-absence surfacing, and any store-consult/trip content.
**Seats/skills:** `qa-engineer` running `mochiko:review-sufficiency`.
**Gate:** none directly (a disputed clause would default to gap and roll into the run-open gate below), but the verdict is binding and feeds Phase 3.

Given how complete the read artifacts already look (FR/SC coverage, ruled AX rows, ruled D-rows), the two live branches to plan for:
- **Sufficient (no gaps):** skip the design phase entirely → go to Phase 5 with the zero-gap map-assertion duty attached to the card-authoring seat.
- **Gaps named:** the design phase (Phase 4) fires, scoped to exactly those gaps.

## Phase 3 — Run-open confirmation (the entry gate)
**Does:** One confirmation, no negotiation. States: the batch (`FEAT-001` "Note capture", selection scope, rows W1+W2), both attempt bounds at their only redeclaration point (`attempt_bound_cycle=3` per cycle, `gap_rework_bound=2` per run — offered for change now only), the sufficiency verdict and its gap routing, the surfaced governance-region absence, and the done condition (all cards `[x]`, test-first, independently verified per-cycle and whole, criteria/traceability/governance met, acceptance landing executed whole, run closes at final acceptance).
**Reads:** `sufficiency-report.md` from Phase 2.
**Writes:** none yet (this is presentation, not a landing write).
**Gate — what is confirmed:** the batch/scope framing, the two attempt-bound numbers, the sufficiency verdict and any gap routing, and how to handle the missing governance region.
**Branches on the user's ruling:**
- *Confirms as-is* → proceed to Phase 4/5 per the sufficiency branch.
- *Adjusts an attempt bound* → the new number is recorded as binding for this run only, proceed.
- *Rules to pause and establish governance first* → this run holds here; the onward action would be routing to `/mochiko:setup` for governance authoring before resuming.
- *Proceeds without governance* → recorded as a logged warning; note `impl.briefs-name-rules-files` (obligated rules-file reads in code-touching briefs) simply does not fire since no governance region exists.
- *Rejects the scope itself* (e.g. wants different rows selected) → routes back to the spec/feature-map layer, this run does not open.

## Phase 4 — Design phase (fires only if Phase 2 named gaps)
**Does:** Design seats author exactly the named gaps, each on a plan the DM approves first. Staffing plan: `technical-analyst` for any design-artifact deltas, `principal-architect` only if a structural/store delta is triggered, `qa-engineer` for the `**TEST:**` case shapes. Given the batch's current baselines already look complete (SQLite persistence, stdlib HTTP, ruled AX rows, ruled D-rows), the realistic gap surface here — if any — is narrow (e.g., a missing NFR target or an under-specified error-response shape), so this phase would likely be small even if it fires.
**Reads:** `sufficiency-report.md`, `spec.md`, current product baselines, architecture spine.
**Writes (deltas beside baselines, never in place):** `.mochiko/features/FEAT-001/` delta files — e.g. `data-model.md` delta, `contracts/` delta, a prose baseline before/after delta — plus a store delta under `.mochiko/product/architecture/` in-flight form if a structural trigger fired; and the design-implied dependency/extent assertion written onto `entry.md` with provenance.
**Non-author review pair (before the checkpoint):** since `technical-analyst`/`principal-architect` would be the authors here, review would be staffed to two different seats — e.g. **tech-lead** running `mochiko:review-feasibility` (buildability/contradiction, and specifically qualified to grade any store As-built/Drift-style judgment content) and **validator** running `mochiko:review-plan-artifacts` (conformance to the gap list + card-quality, BLOCKING).
**Gate — the design checkpoint (user's):** presents the design delta(s), any store delta as a rendered diagram plus its changed `AX-XXX` rows (or source + changed-element table if no render surface), and the review pair's findings.
**Branches:**
- *Signs as presented* → design lands as the seed for Phase 5; no code yet.
- *Signs with amendments* → amendments applied, re-graded if material, then proceeds.
- *Stops here* → run pauses; nothing lost, resumable later exactly at this point.
- *Rejects* → design seat reworks within the `gap_rework_bound` (2 rounds); exhaustion halts the run and presents state for the user's disposition.

## Phase 5 — Cycle-card authoring
**Does:** A design-class seat (never the builder) authors `tasks.md` cycle cards from `spec.md` plus any signed design deltas, per `mochiko:patterns-vertical-tdd` — walking-skeleton/foundation cycles first, then feature cycles. For this batch a plausible (not asserted — the authoring seat's actual call) shape: a foundation cycle (SQLite file + stdlib HTTP skeleton wiring, AX-001/AX-002), then a create-note cycle (W1/US-001, SC-001+SC-002, FR-001/FR-002/FR-004), then a fetch-note cycle (W2/US-002, SC-003, FR-003). Each card carries stories/rationale, dependencies, acceptance-criteria IDs, a `**TEST:**` real-infrastructure gate, and brownfield exposure (none — greenfield). `qa-engineer` authors the `**TEST:**` cases within whatever slicing is chosen. If Phase 4 was skipped (zero-gap path), this same seat also makes the map-entry assertion the design phase would have made, surfacing any drift at the confirm below.
**Reads:** `spec.md`, `sufficiency-report.md`, design deltas (if any), `data-model.md`, `contracts/api.yaml`.
**Writes:** `.mochiko/features/FEAT-001/tasks.md` (rendered from the tasks template; `plugins/mochiko/schemas/tasks.yaml` is the fallback source of truth since no `mochiko-cli` binary is confirmed present here).
**Card review before confirm:** an independent verification seat (e.g. `qa-engineer` if not the card author, otherwise `validator`) grades quality per `mochiko:review-plan-artifacts` and buildability by its own judgment.
**Gate:** none yet — feeds Phase 6.

## Phase 6 — Card confirm (the user's)
**Does:** Presents the sliced cycle cards and the independent review's findings.
**Gate — what is confirmed:** the cycle slicing itself (card boundaries, ordering, dependencies) before any build starts.
**Branches:**
- *Confirms* → proceed to build (Phase 7).
- *Requests a re-slice* (merge/split cycles) → card-authoring seat reworks, re-reviewed, re-confirmed.
- *Defers* → run pauses here; work rows stay unselected for build until resumed.

## Phase 7 — Build (per cycle, test-first, foundation before feature)
**Does, per cycle card in order:**
- `staff-engineer` (fixed builder seat, never designs its own gaps) plans the concrete task decomposition on a DM-approved plan, runs the pre-code minimalism ladder (`mochiko:patterns-code-minimalism`) at decomposition, and builds strictly red→green→refactor per `mochiko:executing-tdd-cycle`. `mochiko:brownfield-integration` does not fire for this batch (no existing files to extend/modify — greenfield).
- Writes application source/tests (exact paths are the builder's decomposition call, bounded by D-001/D-002 — SQLite + stdlib HTTP, no framework) plus `.mochiko/features/FEAT-001/cycle-report.md` (appended per cycle, dated; disclosed decomposition, honest difficulties, deviations, `domain_deps_added`).
- `qa-engineer` (verification seat, never the implementer) executes the card's `**TEST:**` gate against real infrastructure (`mochiko:testing-end-user` — a real SQLite file, real HTTP calls, not mocks) and runs the code-minimalism lens (`mochiko:review-code-minimalism`) against the diff + cycle report + surrounding code, emitting advisory `minimalism:` findings (never gate-failing). Writes a per-cycle verification report under the same feature dir.
- Full repository quality-gate suite runs each cycle; any failure fails that cycle outright, never severity-triaged.
- Attempt economy: each verification grading consumes one of the 3 per-cycle attempts; two consecutive rounds with unchanged findings is a no-progress stop (halt, present state).
- Undesigned structure discovered mid-cycle → halts that cycle and re-fires the design phase scoped to the discovery (loops back through a Phase-4/6-shaped mini-cycle for that discovery only).
- An infeasible card → escalated to the user as a business-level scope call — **embedded gate**, branches: keep as scoped / amend the card's scope / drop the row (returns to pending on the map).
- A commodity-category adopt-first ruling or an IP-XXX provisioning call, if either arises during decomposition, halts to the user rather than the builder deciding — **embedded gate**, branches: approve the named candidate / pick a stated alternative / defer.
- `tasks.md` checkboxes flip `[x]` as each cycle clears; this is the visible progress surface.
- Repeat until both W1 and W2's cards are `[x]`.

**Writes across this phase:** source/test files, `cycle-report.md` (appended), per-cycle verification reports, `tasks.md` checkbox flips.

## Phase 8 — Final validation (whole-build)
**Does:**
- Regression sweep over previously delivered features' accumulated `**TEST:**` gates. FEAT-001 is the first capability on the map, so this sweep would find no prior `gates.md` files to run — that absence gets stated explicitly, not silently skipped.
- Cold verification: build and run quality gates from a dependency-cold snapshot of the uncommitted working tree (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-<purpose>/`), after confirming a `/.claude/worktrees` ignore entry exists. **Blocker to flag:** this workspace is currently not a git repository, so this step cannot run as specified without one existing. This would surface as an explicit blocker/gate rather than being worked around silently — branches: user authorizes initializing a git repo (and the ignore entry) so cold verification can run / user directs an alternative snapshot approach / user accepts the run cannot satisfy this floor obligation and rules on how to proceed.
- Gap-finding pass (fires — this is a selection-scope run, not delta/lane): a **fresh** `devils-advocate` instance, kept clean of any design-review exposure from Phase 4 to preserve blindness, dispatched two-message. First message carries only `spec.md`, `sufficiency-report.md`, design deltas (if any), and the baselines (`data-model.md`, `contracts/`, the store's NFR-001/NFR-002 concern rows) — never code, `tasks.md`, `**TEST:**` cases, or reports. The seat states derived expectations before probing begins.
- Mutation lens: runs on the verification seat (`qa-engineer`, which already holds code sight), only at high depth, owing either results or a stated skip.
- Findings split by kind: spec-required behavior broken → fails final validation, evidence + clause cited; beyond-spec → advisory, disposition reserved to the user (fix now / BACKLOG.md / accept as designed) — **embedded gate** per finding.
**Writes:** `final-validation-report.md`, gap-finding evidence, and (on fix-now/backlog findings) `.mochiko/features/FEAT-001/gates.md` (minted), authored in `**TEST:**` grammar by `qa-engineer`.
**Attempt economy:** gap-rework at run scope uses the `gap_rework_bound` (2 rounds unless redeclared at Phase 3); a finding localized to one cycle's territory charges that cycle's own remaining attempts instead. Exhaustion or a no-progress round halts the run and presents state — disposition is the user's.

## Phase 9 — Final acceptance (the user's gate)
**Does:** Presents the full run summary — done-condition verdict, rounds/seats consumed, all evidence and reports, and the **proposed** landing exactly as it would execute (map graduation, baseline folds, store landing) — without having executed it yet, since the landing is executed whole only at acceptance.
**Gate — what is confirmed:** accept / amend / reject the finished build against the stated done condition.
**Branches:**
- *Accept* → proceed to Phase 10, landing executes whole, run closes with a PASS verdict.
- *Amend* → user specifies the change; a small in-scope fix re-enters Phase 7 for the affected cycle within its remaining attempts, a larger change is scope escalation (`impl.scope-escalation-fail`) and stays FAIL unless the user explicitly accepts the larger scope — either way, re-presented at a fresh acceptance gate afterward.
- *Reject* → run closes FAIL (`impl.fail.no-acceptance`); no landing writes occur — baselines, map, and store remain exactly as they are now.

## Phase 10 — Landing execution (only on Accept)
**Does, atomically:**
- Independent landing-verification seat checks every touched baseline's graded fold via three-way diff (pre-fold baseline + delta vs folded result) — proposed staffing: **validator** for the mechanical fold checks, **tech-lead** for grading the store's As-built/Drift judgment content specifically (matches tech-lead's charter for grading the architecture store's judgment writes).
- Store landing (if a structural delta exists): delta elements flip to built, `FEAT-001` keys clear, `As-built:`/`Drift:` fields written and independently graded, orphan check runs, root `ARCHITECTURE.md` regenerated by the store skill (never hand-edited here).
- Map graduation: W1 and W2 fold into FEAT-001's extent lines; `entry.md` status set `delivered`, dated; `FEATURES.md` index line updated; the note-capture spec's closed status is derived once all its selected rows have folded — no separate close stage.
- KM landing obligations: `.mochiko/memory/knowledge-management.md` does not exist in this repo, so this landing carries no KM step.
**Writes:** `.mochiko/features/FEAT-001/entry.md`, `FEATURES.md`, `.mochiko/product/architecture/spine.md` (As-built/Drift, status), root `ARCHITECTURE.md` (regenerated), plus whatever product baselines carried deltas.
**No git mutations are ever run** by any seat in this plan — commits would only be suggested, never executed or pushed.

## Phase 11 — Close
**Does:** Records the close verdict against the done condition (all cards `[x]` and independently verified, criteria/traceability/governance met, landing executed whole, acceptance given), and surfaces rounds/seats consumed across the whole run per the reporting obligations. Checks the run against the 15-item fail-condition set one last time before declaring done.
**Writes:** the close verdict as part of the final acceptance/landing reporting (no new file beyond what Phases 8–10 already wrote).
**Gate:** none — this is reporting only, following Phase 9's ruling.