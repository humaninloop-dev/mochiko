# Action Plan — `/mochiko:implement FEAT-001` (plan-only, not executed)

## Phase 0 — Load the binding schema
**Done:** Read `plugins/mochiko/schemas/implement.yaml` raw and in full before any gating or seat spawn (already performed above as a required first action). Confirmed the `impl.sec.fail-conditions` section carries exactly 15 `fail-condition`-labeled rules, matching the command's hard-coded count — no halt-and-surface needed. Also read `plugins/mochiko/schemas/command-labels.yaml` to resolve every `labels:` value used below.
**Read:** `plugins/mochiko/schemas/implement.yaml`, `plugins/mochiko/schemas/command-labels.yaml`, `plugins/mochiko/commands/implement.md`.
**Written:** nothing.
**Seats/skills:** none (lead-only).
**Gate:** none.

## Phase 1 — Resolve entry
**Done:** `$ARGUMENTS` = `FEAT-001`, not an `EPIC-XXX`, so this is a single-capability run, selection scope. Confirmed against `FEATURES.md` (row `FEAT-001 | Note capture | selected`) and `.mochiko/features/FEAT-001/entry.md`: two selected work rows, W1 (create a note, US-001, SC-001/SC-002) and W2 (fetch a note by id, US-002, SC-003), both marked "ratified scope, selection source: the spec's accepted selection, 2026-08-20." Dependencies section states "None. First capability on the map" — so no in-batch row blocks on an undelivered dependency; W1/W2 run in any mutually consistent order (W2 naturally sequences after W1 establishes the store).
**Read:** `FEATURES.md`, `.mochiko/features/FEAT-001/entry.md`, `ARCHITECTURE.md` (index only, to confirm no stale-index mismatch).
**Written:** nothing.
**Seats/skills:** none.
**Gate:** none (routing gate only fires on missing scope, which is not the case here).

## Phase 2 — Sufficiency check
**Done:** Dispatch the ten-clause check per selected work row (W1, W2) per `mochiko:review-sufficiency`, run by a seat that authored none of `spec.md`, the architecture store, or the product baselines. Candidate seat: **`mochiko:validator`** (independent, defaults-to-FAIL grader; did not author any of the three sources) — a staffing judgment call under `impl.staffing-latitude`, not a schema mandate. The grader is fenced from reading code, `tasks.md`, `**TEST:**` cases, cycle reports, or this run's own `FEAT-001/` output.

Based on what already exists in the product baselines, the clause-by-clause read for W1/W2 will most plausibly land as:
- Clause 1 (testable criteria): SC-001/SC-002/SC-003 each state a stateable oracle — likely clear.
- Clause 2/3 (contract/data exposure): `contracts/api.yaml` already publishes `POST /notes` and `GET /notes/{id}`; `data-model.md` already publishes the `Note` entity — likely locatable, no gap.
- Clause 4 (structural trigger): `AX-001`/`AX-002` are already `ruled` and not derived from this feature's own delta — likely a no-delta claim.
- Clause 5 (NFR targets): AX-001 states NFR-001 (restart survival, graded by SC-001), AX-002 states NFR-002 (4xx/5xx reason field) — likely targets present.
- Clause 6 (commodity exposure): D-001 (SQLite, adopt-first reasoning recorded) already resolves the storage need — likely no gap.
- Clause 7 (dependency order): none, first capability — resolvable.
- Clause 8 (UX trace): spec states "No UX surface... no Screens & Flows section" — n/a.
- Clause 9 (delivered-feature exposure): no feature has ever been delivered yet — n/a in effect.
- Clause 10 (in-flight exposure): FEAT-001 is the only capability on the map — n/a.

This is the grading seat's call, not mine — the plan branches on the actual verdict:
- **Branch A — `sufficient`, zero gaps:** design phase is skipped; the run proceeds straight to card authoring (Phase 5) with the card-authoring seat making the map-entry assertion the design phase would otherwise have made.
- **Branch B — one or more gaps:** the design phase (Phase 4) fires, scoped to exactly the named gaps.
- **Disputed clause:** defaults to gap and is escalated to the user at run-open rather than cleared by the grader alone.

Independently of the verdict, two **absent surfaces** must be surfaced (never auto-resolved, never failing) at run-open: no `.claude/rules/mochiko/` governance region exists on disk, and no `.mochiko/memory/codebase-analysis.md` exists — the latter is moot here since there is no existing source code (confirmed by an empty glob for source files), so this is a greenfield run and the "offer `/mochiko:setup` or proceed with warning" branch resolves to "proceed, warning logged."

**Read (by the grading seat):** `.mochiko/specs/note-capture/spec.md`, `.mochiko/product/architecture/spine.md`, `.mochiko/product/data-model.md`, `.mochiko/product/contracts/api.yaml`, `.mochiko/product/constraints-and-decisions.md`, `.mochiko/features/FEAT-001/entry.md`.
**Written:** `.mochiko/features/FEAT-001/sufficiency-report.md` (per-row verdicts, gap list keyed to clause if any, store-consult/no-delta claim, any store trips, any in-flight conflicts, the `quickstart.md` null-path record since there's no external-integration surface) — envelope per `templates/report-format.md`.
**Seats/skills:** `mochiko:validator` (proposed) running `mochiko:review-sufficiency`.
**Gate:** none yet — the verdict is input to the run-open gate below, not a standalone checkpoint.

## Phase 3 — Run-open confirmation (the entry gate)
**Done:** One blocking, plain-text confirmation to the user (`impl.acceptance-plain-text`), no negotiation, covering:
- Batch and scope: "FEAT-001 — Note capture, selection scope, work rows W1 + W2."
- Both attempt bounds restated at their only redeclaration point: per-cycle verification attempts default **3** (`attempt_bound_cycle`), gap-rework rounds default **2** (`gap_rework_bound`) — the user may redeclare either number here and nowhere else in the run.
- The sufficiency verdict from Phase 2, its gap routing (Branch A or B), and any store trips/in-flight conflicts for ruling (none currently visible in the store spine, which shows both `AX-001` and `AX-002` `ruled` and `AX-003` explicitly `n-a` with a stated trigger — not a trip).
- The absent governance region and the greenfield/no-codebase-analysis state, surfaced for awareness.
- The done condition: every cycle card `[x]`, built test-first and independently verified against real infrastructure, code meeting SC-001/SC-002/SC-003 and tracing to FR-001–FR-004, acceptance landing executed whole, run closing at final acceptance.

**Gate (user's ruling — reserved, `impl.user-runopen-rulings`):**
- *User confirms as presented* → proceed to Phase 4 (if Branch B) or Phase 5 (if Branch A) with default bounds (3 / 2).
- *User redeclares attempt bounds* → carry the new numbers forward for the rest of the run; no further redeclaration point exists.
- *User rules on a trip/conflict/disputed clause* → that ruling is recorded in the sufficiency report and binds downstream routing (e.g., a disputed clause the user resolves as "gap" folds into Phase 4's scope).
- *User declines to open the run* → run does not proceed past this point; no code, no cards, no further writes.

**Read:** none new.
**Written:** the run-open confirmation content is spoken/recorded, not a separate artifact (it is folded into the sufficiency report's trip/conflict fields where applicable).
**Seats/skills:** none (lead-user interaction only).

## Phase 4 — Design phase *(conditional: fires only on Branch B — one or more named gaps)*
**Done:** Design seats author exactly the named gaps and nothing more, each on a plan the lead approves first (`impl.plan-approval-producers`). Staffing default: `technical-analyst` for design/requirement deltas, `principal-architect` only if clause 4 named an actual structural delta (unlikely here given AX-001/AX-002 are already ruled and no new element appears needed for W1/W2), `qa-engineer` for any new `**TEST:**`-relevant acceptance detail. Each design seat's output is rung-justified per `mochiko:patterns-plan-minimalism` (only build what the gap list requires).

A non-author seat then grades the design-phase package before the checkpoint: `mochiko:review-plan-artifacts` (conformance to the gap list, card quality — blocking) and `mochiko:review-feasibility` (buildability/contradiction, plus the architecture pass if a store delta was produced).

**Design checkpoint — user gate (`impl.gate-design-checkpoint`, floor):** no code is written before the user signs the design and any store delta. If no store delta exists, the sign-off covers the design deltas alone.
- *User signs* → proceed to Phase 5 with the signed deltas as design inputs.
- *User requests changes* → one gap-rework round is consumed (bounded by the run-open-declared `gap_rework_bound`, default 2); the design seat revises the same, and only the same, named gaps.
- *User stops here and resumes later* → run pauses at this checkpoint; no further phases run until resumed.

**Read:** `sufficiency-report.md`, `spec.md`, product baselines, architecture spine.
**Written:** design deltas beside their baselines in `.mochiko/features/FEAT-001/` (e.g., a `data-model.md` delta or `contracts/` delta only if a named gap required one; a store delta in `.mochiko/product/architecture/` only if clause 4 fired) plus `review-plan-artifacts` and `review-feasibility` reports in `.mochiko/features/FEAT-001/`.
**Seats/skills:** `technical-analyst` / `principal-architect` / `qa-engineer` (producers, per gap) → `mochiko:patterns-plan-minimalism` (rung discipline) → independent reviewer(s) running `mochiko:review-plan-artifacts` + `mochiko:review-feasibility`.
**Gate:** design checkpoint (user, floor-level, described above).

## Phase 5 — Cycle card authoring
**Done:** A design-class, non-builder seat (e.g. `technical-analyst` or `qa-engineer`, never `staff-engineer`) authors `tasks.md` cycle cards from `tasks.yaml`'s template shape: cards as bundles of named `**TEST:**` cases (Setup/Action/Assert/Capture), each case citing spec/design IDs (`US-001`/`SC-001`, `US-002`/`SC-003`, `FR-001`–`FR-004`), no task lists or file paths in the card. Given no existing end-to-end path, the first card is a walking skeleton (e.g., thinnest create→persist→read round trip); a second card extends to the full W1 acceptance scenarios (empty-body 400, restart durability) and W2's 200/404 cases. `qa-engineer` authors the `**TEST:**` cases within the card-authoring seat's slicing. On the zero-gap path (Branch A), this same seat also makes the map-entry assertion the design phase would have made (dependencies/extent sharpening on `entry.md`).

An independent verification seat (never the author, never the builder) reviews the cards for quality (`mochiko:review-plan-artifacts`) and buildability (own judgment) before the card confirm.

**Card confirm — user gate (`impl.gate-card-confirm`, floor):** the user rules the slicing before any build starts.
- *User approves slicing* → proceed to Phase 6.
- *User asks for resequencing/merging/splitting* → cards are revised pre-build; this does not consume a build attempt.
- *User flags a card as out of frame (scope escalation)* → per `impl.scope-escalation-fail`, that portion stays FAIL unless the user explicitly accepts the larger scope.

**Read:** `sufficiency-report.md`, design deltas (if Phase 4 ran) or product baselines directly (if Branch A), `spec.md`, `plugins/mochiko/schemas/tasks.yaml`.
**Written:** `.mochiko/features/FEAT-001/tasks.md` (cycle cards, unchecked), plus the card-review report in `.mochiko/features/FEAT-001/`.
**Seats/skills:** `technical-analyst` (or equivalent design-class seat) + `qa-engineer` for TEST cases, per `mochiko:patterns-vertical-tdd`; independent reviewer per `mochiko:review-plan-artifacts`.
**Gate:** card confirm (user, floor-level).

## Phase 6 — Build (test-first, per cycle)
**Done:** `staff-engineer` builds each confirmed card in dependency order (walking skeleton first, then the W1/W2 feature card), decomposing it into concrete tasks at build time with the code in view, disclosed in the cycle report — never pre-decomposed. Build is test-first (red→green→refactor) per `mochiko:executing-tdd-cycle`. Since this is greenfield (no existing source files found), `mochiko:brownfield-integration` does not apply to this batch. `mochiko:patterns-code-minimalism`'s pre-code ladder is run and disclosed at each decomposition point (e.g., "stdlib HTTP server, no framework" already decided at D-002; SQLite via bundled driver already decided at D-001 — so most commodity calls are pre-resolved, minimizing new adopt-first escalations mid-build). `staff-engineer` never designs its own gaps — any undesigned structure discovered mid-build halts that cycle and re-fires Phase 4 scoped to the discovery (`impl.midrun-refire`).
**Read:** `.mochiko/features/FEAT-001/tasks.md`, design deltas/baselines, `.claude/rules/mochiko/` (none present — briefs would name it if it existed).
**Written:** working code (no fixed paths yet — build-time decomposition decides them), plus `.mochiko/features/FEAT-001/cycle-report.md` per cycle (decomposition, difficulties, deviations, `domain_deps_added` — `full` register on any failure narrative, else clean frontmatter).
**Seats/skills:** `staff-engineer` (builder) via `mochiko:executing-tdd-cycle`, `mochiko:patterns-code-minimalism`.
**Gate:** none mid-cycle unless a build-blocking question arises (escalated immediately per `impl.escalation-batching`); a deviation from a signed store delta stops that cycle and reserves the choice to the user (`impl.deviation-gate`, floor).

## Phase 7 — Per-cycle verification
**Done:** An independent verification seat (never `staff-engineer`) runs `mochiko:testing-end-user` against real infrastructure — an actual running instance of the stdlib HTTP server backed by a real SQLite file, executing the card's literal `curl` actions and asserting status codes/response bodies/persistence-after-restart for W1, and 200/404 behavior for W2 — plus the `mochiko:review-code-minimalism` lens reading the diff, the cycle report, and the surrounding code. Each grading pass consumes one attempt against the per-cycle bound (3, or as redeclared at run-open); two consecutive rounds with unchanged findings is a no-progress stop that halts the cycle and presents state to the user rather than retrying blindly. Findings route by severity: Important+ blocks the cycle and joins the next checkpoint batch; Minor defaults to a `BACKLOG.md` booking, never an in-cycle fix. `minimalism:` findings are always advisory, never gating.
**Read:** the working diff for the cycle, `cycle-report.md`, surrounding code.
**Written:** a verification report per cycle in `.mochiko/features/FEAT-001/`; `tasks.md`'s checkbox flips to `[x]` only once the cycle's named cases show green against real infrastructure.
**Seats/skills:** an independent verifier (e.g. `qa-engineer` or `tech-lead`, distinct from the card-authoring seat and from `staff-engineer`) via `mochiko:testing-end-user` + `mochiko:review-code-minimalism`.
**Gate:** implicit per-cycle pass/fail (not a user gate by itself, but Important+ findings and exhaustion of the attempt bound escalate to the user).

*(Repeat Phase 6 → 7 for each remaining card until every card in `tasks.md` is `[x]`.)*

## Phase 8 — Final validation (whole-build)
**Done, in sequence:**
1. **Quality gates:** full repository suite run, never severity-triaged — any failure fails the run (`impl.fail.quality-gate`).
2. **Regression sweep:** accumulated `**TEST:**` gates of previously delivered features in this feature's territory. FEAT-001 is the first capability on the map, so there is no prior delivered territory — this sweep is stated as an explicit empty/no-op in the report, not silently skipped.
3. **Cold verification:** build and run quality gates from a dependency-cold snapshot of the uncommitted working state via `git ls-files -co --exclude-standard :!.claude/worktrees` copied to `.claude/worktrees/mochiko-<purpose>/`. **Concrete blocker to flag:** the working directory is currently **not a git repository** (`Is a git repository: false`), so `git ls-files` cannot enumerate tracked/untracked state as specified. This is a real precondition gap the run would need to surface to the user before Phase 8 can execute faithfully — either the user authorizes a `git init` (a decision, not something the run does unilaterally, consistent with never taking destructive/irreversible actions without confirmation), or an equivalent cold-snapshot mechanism is agreed. This is presented as a checkpoint item, not silently bypassed.
4. **Gap-finding pass:** required on this selection-scope run (`impl.gap-finding-scope`). Fresh `devils-advocate`, dispatched blind via the two-message protocol — first message carries only `spec.md`, `sufficiency-report.md`, any design deltas, and the baselines (`data-model.md`, `contracts/api.yaml`, the store's NFR rows) — never the code, `tasks.md`, `**TEST:**` cases, or any report. The seat states derived expectations before probing begins. Mutation lens runs if the verification seat is operating at high depth; otherwise the skip is disclosed, not silent.
5. Findings split: spec-required behavior broken → fails final validation, must resolve before acceptance; beyond-spec findings → advisory, disposition reserved to the user (fix now / `BACKLOG.md` / accept as designed). A disputed finding-kind defaults advisory and goes to the user.

**Read:** `spec.md`, `sufficiency-report.md`, design deltas, `data-model.md`, `contracts/api.yaml`, architecture spine NFR rows (gap-finder's first message only); full codebase and all cycle artifacts (verification seat, second message and quality-gate execution).
**Written:** `.mochiko/features/FEAT-001/final-validation-report.md` (gates results, regression-sweep statement, cold-verification evidence or the flagged git-repo blocker, gap-finding findings, mutation-lens result or stated skip).
**Seats/skills:** verification seat (quality gates, cold verification, mutation lens) + fresh `devils-advocate` via `mochiko:testing-gap-finding`.
**Gate:** any spec-required finding or gate failure halts final validation and enters the gap-rework bound (default 2 rounds, or the cycle-scoped attempt count if the finding localizes to one cycle); exhaustion or no-progress halts the run and presents state, disposition reserved to the user.

## Phase 9 — Acceptance landing (executed whole)
**Done, only once Phase 8 is clean or user-accepted with dispositions recorded:**
- Store landing (only if Phase 4 produced a signed store delta — plausibly none here since AX-001/AX-002 were already ruled pre-existing): transcribe built elements, write graded As-built:/Drift: fields, run the orphan check, regenerate `ARCHITECTURE.md`.
- Graded fold of any touched baseline (`data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`) via three-way diff, checked by the landing verification seat — not a store landing carve since no structural store delta is expected.
- Gates fold: any gap-finding findings ruled fix-now/backlog fold into `.mochiko/features/FEAT-001/gates.md` (minted fresh, since it doesn't yet exist), authored by `qa-engineer` in `**TEST:**` grammar; accepted-as-designed findings do not fold.
- Map landing (selection scope): W1 and W2 fold into `entry.md`'s extent lines and vanish as pending rows (none remain pending since both are selected and built); `entry.md` status flips to delivered, dated; `FEATURES.md`'s `FEAT-001` row updates; the spec's closure is derived (all of `note-capture/spec.md`'s selected rows folded → spec reads closed).
- Knowledge-management landing: `.mochiko/memory/knowledge-management.md` does not exist, so this obligation is n/a for this run.

**Final acceptance — user gate (`impl.gate-final-acceptance`, floor):**
- *Accept* → run closes DONE; verdict is checked against every one of the 15 fail-conditions from Phase 0 (each explicitly confirmed not standing) before the close statement is given.
- *Amend* → a bounded rework loop reopens the specific finding/cycle under the remaining attempt/gap-rework budget; landing does not execute until re-accepted.
- *Reject* → run closes without landing; disposition (e.g., route unresolved items to a `/mochiko:feature` delta or back to `/mochiko:specify`) is the user's call, stated at close.

**Read:** `final-validation-report.md`, all cycle/verification reports, current baselines.
**Written:** graded folds to `.mochiko/product/data-model.md` / `contracts/api.yaml` / `constraints-and-decisions.md` (baseline files, via fold only, never in-place edits), `.mochiko/product/architecture/spine.md` + derived `ARCHITECTURE.md` (only if a store delta exists), `.mochiko/features/FEAT-001/gates.md`, `.mochiko/features/FEAT-001/entry.md`, `FEATURES.md`.
**Seats/skills:** landing verification seat (independent, grades the folds and any map-delta boundary) via `mochiko:authoring-architecture-store` and `mochiko:authoring-feature-map`.
**Gate:** final acceptance (user, floor-level, described above).

## Phase 10 — Close
**Done:** State the run's verdict against the fixed done condition (Phase 9's outcome) and explicitly re-confirm none of the 15 `impl.sec.fail-conditions` rules stand (sufficiency recorded, design gaps signed if any fired, no card built pre-confirm or by its own author, no card left unchecked, no failing quality gate, no unevidenced verification claim, no regression, no in-place baseline edit, no unresolved deviation, no incomplete store landing, no ungraded fold, gap-finding pass present, any skip stated, no unresolved spec-gap finding, user acceptance given). Report rounds consumed and seats spawned at this final checkpoint per `impl.dm-surface-rounds`.
**Read:** all reports produced across the run.
**Written:** the close statement itself (no new artifact beyond what Phase 9 already wrote).
**Seats/skills:** none (lead-only synthesis).
**Gate:** none (this phase only fires after Phase 9's gate has already resolved to accept/close).