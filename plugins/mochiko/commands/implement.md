---
description: Execute an accepted task breakdown into working, verified code via an independent producer→verifier team loop — a staff-engineer seat implements each cycle through red/green/refactor TDD (foundation cycles before feature cycles) and fix-passes the final validation; a qa-engineer seat verifies every cycle and then the whole implementation against real infrastructure with captured evidence and quality-gate exit codes; the approved architecture is briefed input, guarded by a diagram-anchored deviation self-check at cycle open and close and by a built-vs-approved diff at final validation. A per-cycle checkpoint carries the shape's deterministic-clean devolved branch; a named final-acceptance gate closes the run. Package-gated, cycle-by-cycle, default-FAIL, bounded, kernel-free. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Implement — Execute the Task Breakdown (Cycle-by-Cycle, Foundation → Feature)

**Goal:** turn an accepted `tasks.md` into working, verified code — one cycle at a time, foundation
cycles before feature cycles, each implemented through red/green/refactor TDD and independently
verified against real infrastructure, until every cycle clears and a whole-implementation final
validation passes. `$ARGUMENTS` = optional feature ID or description; empty or
detected-from-workspace is resolved at G1.

**You are the lead** of a team-form command in the mochiko command shape: Read
`${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md` (both layers) and `mochiko:loop-discipline`
before anything else; brief every dispatch per `templates/agent-dispatch.md`. This file carries only
implement's parameters. **First-spawn probe:** the `staff-engineer` producer — foundation cycle 1 is
implemented before anything verifies it.

## Goal

Every `tasks.md` task is `[x]` with its `cycle-report.md`; `qa-engineer` verification passed on every
cycle **and** on the whole-implementation final validation, on real-infrastructure evidence and
quality-gate exit codes; you Read the final-validation report and every escalated cycle's reports and
found no blocking gap — criteria met, gates passing, traceability to requirements holding, the build
aligned with the project's governance; where an approved structural delta existed, the
built-vs-approved diff ran and any divergence reached G5; the KM landing ran; and the user accepted
at G5.

**Not done:** an unchecked task, or a cycle with no report · a failing quality gate · a cycle or the
final validation unverified · a warm-only final validation · a non-clean cycle advanced without your
verdict · a surfaced architecture deviation neither built as approved nor consented as an
amendment · an approved delta whose diff never ran · out of rounds · G5 unaccepted.

## Seats & checks

| seat | agent × skill(s) | produces / grades | spawn | peer edges |
|---|---|---|---|---|
| producer | `staff-engineer` × `executing-tdd-cycle`, `brownfield-integration` | implements each cycle through red/green/refactor TDD → `cycle-report.md`; targeted retry of only the failed tasks; the final-validation fix pass, unconstrained by cycle boundaries; reports its architecture deviation self-check; never verifies | standing across the cycle sequence and the fix-pass loop; **probe seat**, foundation cycle 1 | hands each completed cycle straight to the verifier; retries and fix passes are dispatched by you |
| verifier | `qa-engineer` × `testing-end-user` | verifies each cycle, then the whole implementation, against real infrastructure — executes the cycle's `**TEST:**` tasks, runs the quality gates, captures evidence → a verification report naming its evidence tree + a checkpoint recommendation; never implements | cold at the first cycle verification, standing after | peer-edged with the producer for cycle hand-offs; the endgame is lead-routed |
| arch-diff | `principal-architect` × `authoring-architecture`, diff mode | reports built vs. approved — "built as approved", or the divergence | disposable, at final validation, whenever an approved structural delta existed | none — never the verifier seat |
| arch-scribe | `principal-architect` × `authoring-architecture` | folds the resulting system into `ARCHITECTURE.md` | disposable, at finalize, on a built structural change only, per the KM landing | none |

**Validation model:** the loop's bounded in-loop critique — qa's per-cycle verification plus the
final validation, unsized by design. The verification skill is **never** mounted on the producer.
Outside the devolved branch qa's output is **lead-adjudicated input** and the verdict is yours. One
verifier, so implement numbers no G2 — there is no feasibility-rejection gate.

**Seat lifecycle:** the counted unit is the **cycle**. **Override —** the verifier recycles per
**slice** boundary, its final-validation incarnation additionally briefed from the on-disk
verification reports. A retry or fix-pass respawn carries the failed-task list **and** the
just-failed `cycle-report.md`, relayed at dispatch: the next attempt overwrites that file.

## Constraints

- **G1 entry** — evidence: `$ARGUMENTS` · rules: the user · decides: the resolved `<feature>` (an
  explicit ID, else the most recent in-progress feature under `.mochiko/specs/`, confirmed with the
  user before the run opens).
- **Package gate** — evidence: `tasks.md` present and complete alongside the accepted `plan.md` and
  `architecture.md`, plus the design inputs and `slices.md` (Bindings) · rules: the user · decides:
  whether the run opens. Missing or incomplete → block, pointing the user to `/mochiko:plan`.
- **Governance surface** — evidence: `CLAUDE.md`'s `<!-- mochiko:governance:begin -->` region ·
  rules: the user, when it is absent · decides: proceed on governing context, or run
  `/mochiko:setup` first. Absence is **surfaced, never auto-resolved** — governing context, not a
  blocking gate. Present → each code-touching brief carries the one-line obligated read naming the
  `.claude/rules/mochiko/` files relevant to that cycle's file paths.
- **Cycle checkpoint** — evidence: `cycle-report.md` (deviation self-check, `domain_deps_added`), the
  verification report, qa's classified evidence + recommendation · rules: you, except on the devolved
  branch · decides: the cycle advances, or a targeted retry. It carries the shape's **devolved
  branch**, skipped **exactly** when every verification in the cycle is a deterministic CLI check at
  100% pass **and** no deviation is reported **and** `domain_deps_added` is empty: the cycle then
  clears on qa's PASS-with-evidence, unread by you, counted from its one-line clearance notice.
  Otherwise it fires — any failure, any GUI or subjective verification, any reported deviation, any
  registry addition — and you rule on the reports.
- **Architecture deviation** — evidence: the producer's diagram-anchored self-check, run at cycle
  open **and** cycle close — does this cycle add or remove a box, add, remove or redirect an arrow,
  or move a responsibility across a boundary on the approved diagram? · rules: the user · decides:
  build as approved, or a consented amendment of `architecture.md` before the cycle resumes. A yes
  stops the cycle and you present it — never silently built.
- **G3 clarification** — evidence: an ambiguity or blocker the producer flags · rules: the user ·
  decides: the answer fed forward into the next dispatch, logged in-session. **A preference gap is
  ruled here**; a knowledge gap routes to a native `Explore` pass (the "Research this" branch),
  never to the user; a scope gap is G4's.
- **G4 exit-early / escalation** — evidence: a cap trip, a failing set unchanged round-over-round,
  `IMPLEMENT_STOP`, or a scope gap · rules: the user, on the last evidence · decides:
  continue-refining / accept-with-noted-gaps / stop-and-review — the run stays FAIL unless the user
  explicitly accepts. Neither G3 nor G4 ends the loop on its own.
- **G5 final acceptance** — evidence: your clearing verdict on the final validation — the cycle /
  task / fix-pass counts, quality-gate results, an evidence summary, any noted gaps — **and** the
  built-vs-approved architecture result where an approved delta existed · rules: the user · decides:
  accept (done) / amend (the changes become the failure list; re-enter the relevant cycle or fix
  pass, still bounded, and clear a verdict again) / reject (the work remains under
  `.mochiko/specs/<feature>/` and in the working tree).
- **Bounds:** **targeted retry** — trace a checkpoint failure to its tasks and re-open only those,
  **max 3 attempts per cycle**, never regressing passing code; **fix pass** — failure-scoped after a
  final-validation failure, **max 3 passes**; **convergence stall** — the same failure pattern
  across **2+ rounds** surfaces rather than silently continuing, no-progress being an unchanged
  failing set; kill-switch `.mochiko/specs/<feature>/IMPLEMENT_STOP`, checked before each seat send.
  You count every round.
- **Ordering invariants:** cycles run in dependency order, **all foundation cycles before feature
  cycles**, the current cycle being the first with unchecked tasks. **Sequential-only** — parallel
  cycle execution is a `deliberate-shortcut-ledger` deferral, not a capability drop. Every produced
  cycle is paired with a verification in the same round, never skipped: the hand-off is peer-routed,
  the pairing is yours to enforce. The final validation is lead-routed, never devolved.
- **Cold tree:** the final validation builds and runs the quality gates from a dependency-cold
  snapshot of the **uncommitted working state** — `git ls-files -co --exclude-standard
  :!.claude/worktrees` copied to `.claude/worktrees/mochiko-<purpose>/`, carrying no warm items —
  its results part of G5's evidence.
- **Per-cycle qa isolation:** yours to compose per run — that snapshot plus a declared carry-set of
  warm gitignored items, dependencies **copied or installed, never linked**. Tear **either**
  snapshot down only after its evidence is captured and any snapshot-only failure dispositioned; a
  failed cycle's, after its retry. Triggers, evidence provenance, the git-dependent-gate fallback,
  and rationale: `.mochiko/brainstorms/validator-worktree-isolation/record.md` (D3–D7).
- **Scaffolding:** from the detected stack, create any missing ignore files (`.gitignore` /
  `.dockerignore` / lint-ignore) and the `/.claude/worktrees` ignore entry, project-relative, once
  before the cycle loop.
- **Slice scope** *(when an accepted `slices.md` exists)* — that file's **Graduation contract** is
  the single home for slice resolution, the staleness guard, scope, extend-mode, graded amendment,
  and layout; not restated. implement's own bindings on top: the package gate and the cycle loop read
  `slices/<slice>/tasks.md`; the design inputs are the shared feature-root artifacts plus
  `slices/<slice>/{plan.md, task-mapping.md}`; the quality gates still run the **full repository
  suite** — earlier slices' tests are the regression net that catches a design amendment breaking
  shipped behavior; and when the last slice in Slice-order clears G5 the *feature* is **declared, not
  verified** — `slices.md`'s Feature-Done section executes at feature-close, owned by no workflow
  yet: surface it as the next step, never report feature completion here.

## Bindings

- **Deliverable:** the **working code**, in `tasks.md`'s cycle / task (`T{N}.{X}`) ID namespace, whose
  checkboxes flip `[ ]` → `[x]` as tasks complete.
- **Reports** under `.mochiko/specs/<feature>/` — or `slices/<slice>/` when slice-scoped, where the
  Goal's artifact set reads them: `cycle-report.md` and a verification report per cycle, the
  final-validation report, the built-vs-approved diff report.
- **Design inputs:** `plan.md`, `architecture.md` (the **anchor** for both architecture mechanisms),
  `task-mapping.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`,
  `requirements.md`. Workspace-as-state, no registry field.
- **Uncertainty carrier:** producer-authored — `cycle-report.md`'s honest difficulties, deviations,
  `domain_deps_added` and flagged blockers (`mochiko:executing-tdd-cycle`'s format); qa's evidence
  and status live in its verification reports.
- **Fact route:** real infrastructure — executed `**TEST:**` tasks and quality-gate exit codes;
  knowledge gaps go to a native `Explore` pass.
- **Clearing unit + checkpoint keying:** the **cycle**; a surfaced architecture deviation
  **de-devolves** it, and a non-empty `domain_deps_added` **always** forces the escalated human
  checkpoint — never auto-approved, no stamp read.
- **KM landing:** `.mochiko/memory/knowledge-management.md` exists → run its ritual + invariants
  under fix-on-sight; a **built** structural change folds the built system into `ARCHITECTURE.md`.
  No copy → skip.

## Recovery

Note the resume stage on the deliverable; resume from workspace evidence, respawning what the stage
needs — a respawned producer re-reads the cycle's tasks, the design inputs, and any failed-task list.

| Evidence in the workspace | Resume at |
|---|---|
| no `tasks.md`, or an incomplete package | entry blocked |
| `slices.md` present | resolve the current slice; the rows below then read `slices/<slice>/tasks.md` and per-slice reports |
| `tasks.md` present, ignore files absent | scaffolding |
| unchecked tasks remain, the current cycle has no `cycle-report.md` this round | implement the current cycle |
| the current cycle's `cycle-report.md` present, no verification report this round | verify the current cycle |
| a surfaced deviation unruled | the architecture-deviation gate |
| the current cycle not passed, within the cap | retry / cycle loop control |
| all tasks `[x]`, no final verification report | final validation |
| final validation failed, within the cap | fix pass / loop control |
| final validation cleared, an approved delta existed, no diff report | the built-vs-approved diff |
| final validation cleared, not yet accepted | G5 |
| accepted | finalize — report the code and reports, the per-cycle and fix-pass round counts, the cycle / task / fix-pass counts with quality-gate status, the KM landing, a suggested commit (`feat: implement <feature>`), and the next step |
| `IMPLEMENT_STOP` present | escalate (G4) |
