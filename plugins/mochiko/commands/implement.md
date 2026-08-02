---
description: Execute an accepted task breakdown into working code, TDD-built and independently verified against real infrastructure.
disable-model-invocation: true
---

# Implement — Execute the Task Breakdown

**Goal:** turn an accepted `tasks.md` into working, verified code — TDD-built, foundation
cycles before feature cycles, verified against real infrastructure. `$ARGUMENTS` = optional
feature ID; empty → resolve from `.mochiko/specs/` and confirm with the user.

## Goal

Every `tasks.md` task is `[x]`; the built code was implemented test-first
(red/green/refactor) and independently verified — executed `**TEST:**` tasks, quality gates
with exit codes, captured real-infrastructure evidence — per cycle and once for the whole
implementation; the code meets its criteria, holds traceability to requirements, and aligns
with the project's governance; where a structural delta was approved at plan time, a
built-vs-approved diff report exists and any divergence it names was ruled by the user; and
the user accepted the implementation.

**Not done — default FAIL:** an unchecked task · a failing quality gate · verification
without real-infrastructure evidence · a surfaced architecture deviation neither built as
approved nor consented as an amendment · user acceptance not given.

## Harness

- **You are the lead.** Plan the run and orchestrate it toward the Goal; teammates or
  subagents per seat is your call.
- **Plan approval:** any seat that writes code or artifacts plans first and works only on a
  plan you approved; grading, verification, and fact-finding seats are exempt.
- **Independence:** no output is cleared by its author — implementation and verification are
  never the same seat; verification executes against real infrastructure and reads the code
  and its evidence, default FAIL.
- **Reserved to the user:** architecture-deviation consent — a cycle that adds or removes a
  box, adds, removes, or redirects an arrow, or moves a responsibility across a boundary on
  the approved diagram stops and is presented: build as approved, or amend `architecture.md`
  first · requirement ambiguity or a judgment call a producer flags — answered by the user,
  investigable gaps excepted · scope escalation (work bigger than the run was framed; the
  run stays FAIL unless the user explicitly accepts) · final acceptance (accept / amend /
  reject).
- **Entry:** the accepted package gates the run — `tasks.md` complete alongside `plan.md`
  and `architecture.md`; missing or incomplete → block, point to `/mochiko:plan`. A missing
  governance region is surfaced, never auto-resolved; present → each code-touching brief
  names the relevant `.claude/rules/mochiko/` files as an obligated read.
- Suggest commits; never run git mutations, never push — an ephemeral, self-removed
  verification snapshot is not a mutation of refs, index, tracked content, or history.
  User acceptance is plain blocking text, never a timed prompt.

## Bindings

- **Deliverable:** the working code; `tasks.md`'s checkboxes (`T{N}.{X}` namespace) are the
  progress surface, flipped as tasks complete.
- **Craft skills:** TDD via `mochiko:executing-tdd-cycle` (its `cycle-report.md` format —
  honest difficulties, deviations, `domain_deps_added` — is the uncertainty carrier;
  brownfield touches ride `mochiko:brownfield-integration`) · verification via
  `mochiko:testing-end-user` — evidence captured, never assumed.
- **Design inputs:** `plan.md`, `architecture.md` (the anchor for the deviation check and
  the built-vs-approved diff), `task-mapping.md`, `data-model.md`, `contracts/api.yaml`,
  `constraints-and-decisions.md`, `requirements.md`.
- **Reports** under `.mochiko/specs/<feature>/` — or `slices/<slice>/` when slice-scoped:
  cycle reports, verification reports, the final-validation report, the built-vs-approved
  diff report.
- **Slice scope** (accepted `slices.md` present): the run reads `slices/<slice>/tasks.md`;
  quality gates still run the full repository suite; when the last slice clears, the feature
  is declared, not verified — Feature-Done executes at feature-close, surfaced as the next
  step, never reported complete here.
- **Cold verification:** the final validation builds and runs the quality gates from a
  dependency-cold snapshot of the uncommitted working state
  (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to
  `.claude/worktrees/mochiko-<purpose>/`), its results part of the acceptance evidence;
  ensure the `/.claude/worktrees` ignore entry exists first.
- **KM landing:** where `.mochiko/memory/knowledge-management.md` exists, a built structural
  change folds into `ARCHITECTURE.md`.
- **Register:** user-facing prose per `templates/output-style.md`.
