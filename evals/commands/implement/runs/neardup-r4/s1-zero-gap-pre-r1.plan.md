# Action Plan — `/mochiko:implement FEAT-001` (plan-only, not executed)

**Grounding read** (already performed as this plan's first action, per the command's own rule): `plugins/mochiko/schemas/implement.yaml` raw+full, and `plugins/mochiko/schemas/common.yaml` raw+full (a rule in `impl.sec.tools` — `impl.register` — carries `extends: common.register`), plus `plugins/mochiko/schemas/command-labels.yaml` for label meanings. Fail-condition count verified: 15 `kind: fail` nodes in `impl.sec.fail-conditions` — matches the `.md`'s hard-coded Not-done count, so no halt-and-surface is triggered.

**Entry facts read from the repo** (also part of this plan's first action):
- `FEATURES.md` — one capability on the map: `FEAT-001` "Note capture," status `selected`.
- `.mochiko/features/FEAT-001/entry.md` — scope source is the spec's **accepted selection** (ratified 2026-08-20) → **scope = selection**, not delta, not epic. Two selected work rows: W1 (create, US-001, SC-001/SC-002) and W2 (fetch by id, US-002, SC-003). Dependencies: none — first capability on the map, so the dependency-order block never fires.
- `.mochiko/specs/note-capture/spec.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/product/architecture/spine.md` — all present and materially on-point for W1/W2 (AX-001/AX-002 already ruled, D-001/D-002 already decided, Note entity and both endpoints already modeled).
- No `CLAUDE.md`, no `.claude/rules/mochiko/`, no `.mochiko/memory/knowledge-management.md`, no `.mochiko/memory/codebase-analysis.md`, no application source found — greenfield, governance region absent, km-file absent.

These facts fix which conditional rules in the schema bind for this specific run (`scope: selection`, `governance_region: absent`, `km_file: absent`, `baseline: present`) — used throughout the phases below.

---

## Phase 1 — Sufficiency check (moment: `entry`)

**Does:** Dispatches an independent grading seat — one that authored none of `spec.md`, the architecture store, or the product baselines, and will not design or build this batch (`impl.seat-sufficiency-independence`) — to run the per-row check from `mochiko:review-sufficiency` over W1 and W2. Candidate seat for this run: `mochiko:validator` (built for binary, checklist-anchored, default-FAIL grading, and holds no authorship stake in any graded source); staffing itself is latitude (`impl.staffing-latitude`), so this is a judgment call, not a fixed assignment.

**Reads:** `spec.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `architecture/spine.md`, `FEAT-001/entry.md`. Absent-surface handling applies rather than failing anything: governance region absent → surfaced to the user, non-blocking; `codebase-analysis.md` absent on an apparently-greenfield tree → offer `/mochiko:setup` or proceed greenfield with the warning logged (the check still runs and grades the absence per its absent-baseline branch) — this is a routing rule, not a fail condition (`impl.absent-surfaces`).

**Would write:** `.mochiko/features/FEAT-001/sufficiency-report.md` — per-row verdict (`sufficient` or a named gap list), the store-consult result, any no-delta claim, the quickstart.md null-path statement (this feature has no external-integration surface — HTTP API over local SQLite only), and any trips for the user. Follows `templates/report-format.md`.

**Seats/skills:** grading seat (TBD at run time, latitude) · `mochiko:review-sufficiency` (pointer, owns the clause set and gap forms).

**Gate:** none yet — a disputed clause inside the check defaults to gap and routes to the user (`impl.sufficiency-disputed-clause`), but that's folded into the report, not a standalone stop here.

---

## Phase 2 — Design-phase branch (moment: `design-checkpoint`, conditional)

**Gate:** *If* Phase 1 names any gap against W1 or W2, the design phase fires before any code, scoped to exactly those named gaps and nothing else (`impl.design-phase-fires-on-gap`, `impl.design-gaps-only`). *If* Phase 1 clears both rows with zero gaps, this phase is skipped outright and the card-authoring seat (Phase 4) makes the map-entry assertion the design phase would otherwise have made, surfacing any drift at the card confirm (`impl.zero-gap-map-assertion`).

- **Ruling: gaps found.** Design-class seats staff per gap type (typically `technical-analyst` for a requirements/design delta, `principal-architect` for a store delta, `qa-engineer` for `**TEST:**` case authoring) — never `staff-engineer`, which never designs its own gaps (`impl.builder-never-designs`). Each seat works only on a lead-approved plan. Outputs land as deltas beside their baselines at `.mochiko/features/FEAT-001/` — never edited in place (`impl.baselines-never-in-place`, floor) — plus a store delta if the structural trigger fired, following `mochiko:patterns-system-design` / `mochiko:authoring-architecture-store`. A non-author seat then grades the package: `mochiko:review-plan-artifacts` (conformance/card quality, blocking) and `mochiko:review-feasibility` (buildability/contradiction) before the checkpoint. **Gate (floor, `impl.gate-design-checkpoint`):** the user signs the design and any store delta — the store delta signed on a rendered diagram plus its `AX-XXX` row changes, or the source-plus-changed-table fallback where no render surface exists. Onward branches: *sign* → proceed to Phase 3 with a signed anchor for the deviation gate; *stop here* → the user may pause the whole run and resume the build later, which this plan would honor as an explicit stopping point, not a failure.
- **Ruling: no gaps.** Skip straight to Phase 3.

**Would write (if gaps found):** design deltas under `.mochiko/features/FEAT-001/` (e.g. `data-model.md` delta, `contracts/` delta, an appliable prose delta), a store delta under `.mochiko/product/architecture/` if triggered, `mochiko:review-plan-artifacts`/`mochiko:review-feasibility` grading output.

---

## Phase 3 — Run-open confirmation (the entry gate)

**Does:** One confirmation, no negotiation, assembling: the batch name and scope type (`FEAT-001` "Note capture," selection scope — no epic members, no delta-card check since this isn't delta scope) · both attempt bounds at their only redeclaration point — default per-cycle `3` (`attempt_bound_cycle`), default gap-rework `2` (`gap_rework_bound`), carried from the schema's `vars:` unless the user overrides them here · the Phase 1 sufficiency verdict, its gap routing, and any trips/conflicts for ruling (here: the absent governance region and absent codebase-analysis.md, surfaced per Phase 1) · the done condition stated plainly (every cycle card `[x]`, test-first, independently verified against real infrastructure per-cycle and whole, criteria traced to requirements, governance-aligned where a region exists, acceptance landing executed whole, run closes at final acceptance).

**Gate:** plain blocking text, never a timed prompt (`impl.acceptance-plain-text`). 
- **Ruling: confirmed as stated** → proceed to Phase 4 with the stated bounds and scope locked. 
- **Ruling: user redirects** (e.g., changes an attempt bound, disputes a trip, or contests the sufficiency verdict) → the redirected value becomes binding for the rest of the run (bounds are redeclarable *only* here); a disputed sufficiency clause reserved to the user (`impl.user-runopen-rulings`) is resolved before continuing. Either way, planning continues into Phase 4 once the user's ruling lands.

**Would write:** nothing new — this confirmation is recorded as part of the run's reporting trail, not a separate artifact.

---

## Phase 4 — Cycle-card authoring

**Does:** A design-class seat that is not the builder (per `impl.seat-card-author-independence`) slices W1 and W2 into TDD cycle cards per `mochiko:patterns-vertical-tdd` — foundation cycles before feature cycles, walking skeleton first. Given the shape here (two thin HTTP endpoints over one SQLite table, no prior cycles), a plausible slicing is one foundation cycle (service scaffold + SQLite wiring) followed by one cycle per story (create-note, fetch-note) — but the exact cut is the authoring seat's judgment, not fixed by this plan. `qa-engineer` authors the `**TEST:**` real-infrastructure gate within that slicing, citing SC-001/SC-002/SC-003 by ID. An independent verification seat reviews the cards before confirm (`impl.card-review-before-confirm`) — quality via `mochiko:review-plan-artifacts`, buildability its own call.

**Would write:** `.mochiko/features/FEAT-001/tasks.md`, cycle cards rendered from the tasks template (`mochiko-cli template tasks`, or `plugins/mochiko/schemas/tasks.yaml` read raw if the binary is absent) — each card carrying stories/rationale, dependencies, acceptance-criteria IDs, a `**TEST:**` gate, and brownfield exposure (here: none, greenfield). No task lists or file paths on the cards themselves — the builder decomposes at build time.

**Seats/skills:** card-authoring seat (design-class, not `staff-engineer`) · `qa-engineer` for TEST cases · independent verification seat for pre-confirm review · `mochiko:patterns-vertical-tdd` (pointer).

---

## Phase 5 — Card confirm (moment: `card-confirm`)

**Gate (floor, `impl.gate-card-confirm`):** the user rules on the cycle slicing before any card is built.
- **Ruling: approved as-is** → proceed to Phase 6 with the current card set locked.
- **Ruling: resequence/adjust** → the authoring seat revises the affected cards, the verification seat re-reviews, and the confirm re-fires before any build starts; nothing is built against an unconfirmed slicing.

---

## Phase 6 — Build, cycle by cycle (moments: `cycle-checkpoint`, repeated per card)

**Does, per card:** `staff-engineer` (the fixed builder seat) executes the card via `mochiko:executing-tdd-cycle` — decomposes it into concrete tasks (disclosed in the cycle report), applies the `mochiko:patterns-code-minimalism` pre-code ladder at decomposition with rungs disclosed, drives red→green→refactor, and works only on a lead-approved plan (`impl.plan-approval-producers`). Since this is greenfield with no prior code, `mochiko:brownfield-integration` never fires for these two cycles. An independent verification seat (never the implementer — `impl.seat-verification-independence`) then runs `mochiko:testing-end-user` against real infrastructure (an actual SQLite file, an actual running HTTP server — never mocks) for the card's `**TEST:**` gate, plus the advisory `mochiko:review-code-minimalism` lens reading the diff, the cycle report, and the surrounding code.

**Attempt economy (floor):** each grading pass consumes one of the per-cycle attempts (default 3, or as redeclared at run-open). Two consecutive rounds with unchanged findings is a no-progress stop — halt that cycle and present state to the user (`impl.no-progress-stop`). Findings route by severity: Minor → BACKLOG.md booking, never an in-cycle fix; Important-or-above → blocks the cycle and joins the checkpoint batch. Any build-blocking reserved-to-user question (infeasible-card escalation, an adopt-first/IP-XXX call, a deviation-gate trip, requirement ambiguity) interrupts immediately rather than waiting for the batch; everything else accumulates and lands as one batch at the cycle checkpoint.

**Would write, per cycle:** the flipped checkbox on `tasks.md`, `cycle-report.md` and the verification report under `.mochiko/features/FEAT-001/`.

**Gate (implicit, per escalation type surfaced during build):** e.g. an infeasible card → escalated to the user as a business-scope decision, branches are *narrow the card's scope and continue* or *the user accepts the larger scope, which then becomes part of the run's committed bounds*; a deviation-gate trip (a box/arrow/boundary change against a signed delta) → branches are *build as approved* or *amend the delta by the user's ruling first* — never silently designed around (floor, `impl.deviation-gate`).

This phase repeats until every card on `tasks.md` is `[x]`.

---

## Phase 7 — Final validation (moment: `final-validation`)

**Does:** Because scope is `selection`, the full validation battery runs (an epic or delta/lane run would change which pieces fire, but neither applies here):
- Quality gates over the full repository suite, never severity-triaged (floor).
- Regression sweep of accumulated `**TEST:**` gates from previously delivered features in this territory — none exist yet (first capability on the map), so this sweep is expected to be a no-op, but it still runs and is still reported, per the rule.
- Cold verification: a dependency-cold snapshot of the uncommitted working tree (`git ls-files -co --exclude-standard :!.claude/worktrees`, copied to `.claude/worktrees/mochiko-note-capture/`), confirming first that a `/.claude/worktrees` ignore entry exists, then building and running quality gates from that snapshot as acceptance evidence.
- Gap-finding pass — **mandatory** here since scope is `selection` (`impl.gap-finding-scope`; its absence would be an automatic run-failing condition, `impl.fail.gap-finding-missing`). Dispatches a *fresh* `devils-advocate` seat, blind, two-message: message one carries only `spec.md`, `sufficiency-report.md`, any design deltas, and the baselines (`data-model.md`, `contracts/api.yaml`, the store's NFR-001/NFR-002 concern rows) — never the code, `tasks.md`, the TEST cases, or any report; the seat states its derived expectations before any probing begins. Findings split by kind: spec-required behavior broken → fails final validation; beyond-spec → advisory, disposition reserved to the user (fix now / BACKLOG.md / accept as designed).

**Would write:** `.mochiko/features/FEAT-001/final-validation-report.md`, plus the built-vs-signed diff if a store delta was signed in Phase 2.

**Gate:** gap-rework bound is a floor — default 2 rounds (or as redeclared at run-open), unless a finding localizes to one cycle's territory, in which case it charges that cycle's remaining attempts instead. Exhaustion or an unchanged-findings round halts the run and presents state; the disposition (retry differently, narrow scope, or stop) is the user's call, not this plan's to preempt.

---

## Phase 8 — Landing (moment: `landing`) and final acceptance (moment: `acceptance`)

**Gate (floor, `impl.gate-final-acceptance`):** the user rules accept / amend / reject, in plain blocking text.

- **Ruling: accept.** The acceptance landing executes whole (`impl.dm-landing-whole`): the store landing (only if a delta was signed) — delta elements flip built, `FEAT-XXX` keys clear, As-built:/Drift: fields written and independently graded, the orphan check runs, `ARCHITECTURE.md` regenerates; every touched baseline (`data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md` if touched) folds exactly once via a graded three-way diff checked by the landing verification seat; because scope is `selection`, the map graduation batch also runs — W1/W2 fold into FEAT-001's extent lines and vanish from the work-row list, status flips to `delivered` (dated), the `FEATURES.md` index line updates, and the spec's closed status is derived (all its selected rows folded); any ruled fix-now/backlog gap findings fold into `.mochiko/features/FEAT-001/gates.md` in `**TEST:**` grammar, authored by `qa-engineer`. `km-landing` is skipped since no `knowledge-management.md` exists in this repo. The run then closes with a verdict stated against the done condition (`impl.dm-close-verdict`), and rounds-consumed/seats-spawned are surfaced as part of that close (`impl.dm-surface-rounds`).
- **Ruling: amend.** The user names what must change; the run routes back to whichever phase the amendment targets (a build cycle, the card set, or — if it reopens design — the design phase under its own re-checkpoint) before returning to this gate. No landing executes until a subsequent accept.
- **Ruling: reject.** No landing executes; the run does not close as delivered. State is presented and the disposition (rework, abandon, or restart earlier in the pipeline) is the user's, not assumed by this plan.

**Would write (on accept only):** the folded baselines under `.mochiko/product/` and `.mochiko/product/architecture/` (via the graded fold, never in-place edits), the updated `FEATURES.md` and `.mochiko/features/FEAT-001/entry.md`, `.mochiko/features/FEAT-001/gates.md`, and the regenerated root `ARCHITECTURE.md`.

---

## Standing constraints applied across every phase above

- No output is cleared by its author — every grading step above (sufficiency, design-package review, card review, per-cycle verification, landing fold grading) uses a seat distinct from the one that produced the artifact (floor, `impl.author-grader-default-fail`).
- Git: commits would be suggested at natural checkpoints, never run or pushed by this plan (`impl.no-git-mutations`).
- Locate/enumerate reads (e.g. finding prior art, scanning for existing source) would route to a native `Explore` subagent spawned with `model: haiku`; interpretive reads (grading spec content, judging sufficiency, judging feasibility) stay on the session tier (`impl.model-tiering`).
- Because this run composes more than one seat (builder, verification, sufficiency grader, possibly design seats, the gap-finder), the transport floor (`mochiko:patterns-transport-floor`) governs any cross-seat messaging and any shared-write surface from the moment a second seat is spawned onward — referenced, never re-derived here.
- The run would be judged not-done if any of the 15 `kind: fail` conditions in `impl.sec.fail-conditions` stands at close (e.g. an unrecorded sufficiency verdict, a skipped-but-required design phase, a card built before its confirm, an unchecked card, a failing or triaged gate, unevidenced verification, a regression, an in-place baseline edit, an unresolved deviation, an incomplete store landing, an ungraded fold, a missing or unstated-skip gap-finding pass, an unresolved spec-required gap finding, or missing user acceptance) — this plan treats each of those as the concrete failure mode to watch for at its corresponding phase above, not an abstract rule to cite.