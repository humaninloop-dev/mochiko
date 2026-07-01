---
description: Generate the implementation task breakdown via an independent producer→reviewer loop (task-architect authors the story→cycle mapping then the cycle-based tasks, devils-advocate grades both) with a human acceptance gate on tasks.md — plan-gated, default-FAIL, bounded, kernel-free.
disable-model-invocation: true
---

# Tasks — Implementation Task Breakdown (Mapping → Tasks)

You are the **lead / supervisor** for turning an accepted plan into an implementation task breakdown across two phases — a story→cycle **Mapping** then the cycle-based **Tasks**. You own the loop, the verdict, and the human gates. Both artifacts are **authored** by `mochiko:task-architect` (vertical slicing + TDD discipline) and independently graded by a single reviewer, `mochiko:devils-advocate`, checking task-artifact quality (vertical-slice integrity, TDD test-first ordering, story→cycle→task traceability). Never let the producer grade its own output. The reviewer *recommends* a status; **you own the clearing verdict** — its status is input, never the gate.

This is a mochiko **sound loop**: invoke **`mochiko:loop-discipline`** and honor all four requirements (default-FAIL done-condition, independent validation, bounded iteration, named human gates), and brief each dispatch per **`agent-dispatch`**. Those rules are not restated here — this command states only what is specific to *this* workflow.

**Argument:** `$ARGUMENTS` = optional feature ID or description; else the feature is detected from the workspace. Empty input (the known `@`-reference drop bug) is recovered in Phase 0.

## Contract parameters (fill the artifact — don't inline it)

Fill `templates/workflow-contract.md` → `.mochiko/specs/<feature>/tasks-contract.md` with the values below, then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable proof — not this command body.

- **Done-condition** — starts FAILing; clears only when **(1)** both artifacts exist (`task-mapping.md` · `tasks.md`), **(2)** `devils-advocate` returns `ready` on the Phase-1 mapping **and** on the Phase-2 tasks, each grounded in the files, **(3)** *you* Read the artifacts + the reviewer reports and confirm no blocking gap remains, **and (4)** the Phase-3 human acceptance on `tasks.md` has cleared. Out of rounds = escalate, never done.
- **Team** — producer `mochiko:task-architect` (`patterns-vertical-tdd`) authors both phases, never grades; reviewer `mochiko:devils-advocate` (`validation-task-artifacts`) grades from the files, never authors. Disjoint agents and skills — a single reviewer, never the producer.
- **Bounds** — cap **3** produce↔review rounds per phase (you count); no-progress exit when the reviewer's gap set is unchanged round-over-round; kill-switch `.mochiko/specs/<feature>/TASKS_STOP` checked before each dispatch.
- **Gates** — G1 input recovery · G3 clarification (incl. the "Research this" knowledge-gap branch) · G4 exit-early offering · G5 tasks.md acceptance · escalation on any guard trip. (No G2 — tasks is single-reviewer, so plan's feasibility-rejection slot is intentionally unused.)

> Why this done-condition differs from HIL's: HIL declared "no hard caps" and routed on the advocate's verdict *field* — it could self-declare done at pass 1, violating `loop-discipline` reqs 1 & 3. The reviewer's three-state status survives only as input to your verdict; the deterministic cap and the new G5 acceptance gate close the gates HIL lacked.

## Phase 0 — Prerequisites & entry triage  *(human gate G1)*

1. **Capture** `$ARGUMENTS`; resolve `<feature>` (an explicit ID, else the most recent in-progress feature under `.mochiko/specs/`). If empty (the `@`-reference drop bug), recover via **G1**: ask the user to re-enter, or to confirm the detected feature.
2. **Entry gate — plan-workflow-complete.** The plan must be done: `.mochiko/specs/<feature>/plan.md` present (workspace evidence — there is no context-file `status` to read; mochiko `plan` writes none). Missing → block and point the user to `/mochiko:plan`.
3. **Constitution as governing context.** Read `.mochiko/memory/constitution.md` and carry its principles into the producer's brief. Missing → surface it (offer `/mochiko:setup`); this is governing context, not a blocking gate — do not auto-resolve.
4. **Brownfield-from-plan.** Inherit brownfield context from plan's artifacts; do **not** re-run codebase analysis. (The roadmap track — `evolution-roadmap.md` / `[GAP:XXX]` — is a documented stub, deferred.)
5. **Read plan's design outputs** (`spec.md`, `requirements.md`, `constraints-and-decisions.md`, `nfrs.md`, `data-model.md`, `contracts/api.yaml`) as the producer's inputs — workspace-as-state, no registry field.

## Phase 1 — Mapping loop  *(you own the round counter and the verdict)*

`round = 1`; the mapping is FAIL until proven. The reviewer grades slicing quality **before** the expensive full TDD breakdown — the tasks analogue of plan's feasibility-once gate (cheap rework avoidance).

1. **Produce.** Dispatch `mochiko:task-architect` to author `task-mapping.md` — the freehand story→cycle mapping + vertical-slice rationale, the **source of truth** for slicing decisions (+ `taskarchitect-report.md`), briefed per `agent-dispatch`: plan's design outputs, the constitution as governing context, the brownfield context when present, and — on round > 1 — the reviewer's gap list for targeted revision (fix flagged gaps; don't regress passing slices).
2. **Early mapping review.** Dispatch `mochiko:devils-advocate` to grade the mapping from the file (the `validation-task-artifacts` Mapping checklist — slice quality, foundation separation, story coverage, cycle sizing) → `advocate-report.md` (`ready` / `needs-revision` / `critical-gaps`).
3. **Verdict (you).** Read `task-mapping.md` + the report. `ready` **and** no blocking gap → Phase 2. Otherwise route each gap per `loop-discipline`'s gap-routing (→ **G3**), apply the bounds (increment `round`; cap / no-progress / kill-switch → **G4** / escalate), and loop to step 1.

## Phase 2 — Tasks loop  *(cumulative review; you own the round counter)*

`round = 1`; the tasks are FAIL until proven. Phase 2 reviews `tasks.md` **cumulatively** — a full task review plus a cross-check back to `task-mapping.md`.

1. **Produce.** Dispatch `mochiko:task-architect` to author `tasks.md` (cycle→TDD tasks: foundation cycles sequential, feature cycles `[P]`; a file path per task; `[US#]` story tags; `[EXTEND]`/`[MODIFY]` markers; the Story→Cycle table = a **derived echo** of `task-mapping.md`, not a second source) (+ `taskarchitect-report.md`), briefed per `agent-dispatch` with `task-mapping.md` as the input to expand.
2. **Cumulative review.** Dispatch `mochiko:devils-advocate` in **cumulative mode** — a full review of `tasks.md` plus a cross-check back to `task-mapping.md` (mapping↔tasks alignment, the story→cycle→task chain, cycle/dependency consistency — the `validation-task-artifacts` Cross-Artifact Review); you select the mode and supply **both** artifact sets {`tasks.md`}/{`task-mapping.md`} → `advocate-report.md`.
3. **Verdict (you).** Read `tasks.md` + the report. `ready` and no blocking gap → Phase 3. Otherwise route gaps per `loop-discipline` (→ **G3**), apply the bounds (cap / no-progress / kill-switch → **G4** / escalate), and loop to step 1.

**Mid-loop gates (both phases).** **G3** clarification: present a reviewer's gaps, take answers (logged in-session), and feed them into the next produce — a "Research this" answer routes the knowledge gap to native `Explore` (per `loop-discipline`), never to the user. **G4** exit-early / escalation: on a guard trip or stalled gaps, present the last findings and let the user continue-refining / accept-with-noted-gaps / stop-and-review — the run stays FAIL unless the human explicitly accepts. Neither ends the loop on its own.

## Phase 3 — tasks.md acceptance  *(human gate G5)*

Reachable only after your clearing verdict. Present the validated deliverables (cycle / foundation / feature / `[P]` counts, any noted gaps) and ask the user to **accept** (→ Phase 4; the done-condition is now satisfied), **amend** (re-enter the relevant phase with the changes as the gap list — still bounded; it must clear a verdict again), or **reject** (abort; the drafts remain under `.mochiko/specs/<feature>/`).

## Phase 4 — Finalize

Report the artifacts (`task-mapping.md` · `tasks.md` deliverables + the round reports `taskarchitect-report.md` / `advocate-report.md`), the per-phase round counts, the cycle / foundation / feature / `[P]` counts, a suggested commit (`docs: tasks <feature>`), and the next step (`/mochiko:implement`). Offer a lightweight retain/clean choice for the intermediate round reports; never offer to delete `task-mapping.md` or `tasks.md` — they are the deliverables.

## State recovery

Resume from workspace evidence (there is no context-file `phase`/`status`):

| Evidence in the workspace | Resume at |
|---------------------------|-----------|
| no `.mochiko/specs/<feature>/plan.md` | Phase 0 (entry blocked) |
| `plan.md` present, no `task-mapping.md` | Phase 1 (produce) |
| `task-mapping.md` present, no `advocate-report.md` this round | Phase 1 (review) |
| mapping not `ready`, within the cap | Phase 1 (loop control) |
| mapping cleared, no `tasks.md` | Phase 2 (produce) |
| `tasks.md` present, advocate not `ready`, within the cap | Phase 2 (loop control) |
| both cleared, not yet accepted | Phase 3 |
| accepted | Phase 4 |
| `.mochiko/specs/<feature>/TASKS_STOP` present | escalate (G4) |

---

**What you own (not the agents):** the two-phase sequence (Mapping → Tasks) and the per-phase loop (round counter, no-progress check, cap, kill-switch, escalation); the verdict (the reviewer grades from the files, you Read the artifacts and decide against the default-FAIL done-condition — its status is input); the Phase-2 cumulative-mode selection and supplying both artifact sets to the reviewer; the human gates (G1 / G3 / G4 / G5); the plan-complete entry gate and the constitution / brownfield prerequisites; verifying each dispatch actually wrote its expected files (a missing output → log and ask retry/abort); and never letting the producer grade its own output. Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool (never inline agent behavior); do not modify git or push. Full rules: `mochiko:loop-discipline`.
