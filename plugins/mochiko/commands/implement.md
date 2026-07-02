---
description: Execute an accepted task breakdown into working, verified code via an independent producer→verifier loop (staff-engineer implements each cycle through TDD, qa-engineer verifies independently against real infrastructure) with a confidence-based per-cycle gate and a named final-acceptance gate — tasks-gated, default-FAIL, bounded, kernel-free.
disable-model-invocation: true
---

# Implement — Execute the Task Breakdown (Cycle-by-Cycle, Foundation → Feature)

You are the **lead / supervisor** for turning an accepted `tasks.md` into working, verified code — one cycle at a time, foundation cycles before feature cycles. You own the loop, the verdict, and the human gates. Each cycle is **implemented** by `mochiko:staff-engineer` (`executing-tdd-cycle` + `brownfield-integration`) through red/green/refactor TDD, then **independently verified** by a *different* agent, `mochiko:qa-engineer` (`testing-end-user`), against real infrastructure with captured evidence and quality-gate exit codes. Never mount a verification skill on staff, and never let staff grade its own cycle. qa presents evidence and a checkpoint recommendation; **you own the clearing verdict** — the per-cycle checkpoint and the final validation, decided against a default-FAIL done-condition. qa's status is input, never the gate.

This is a mochiko **sound loop**: invoke **`mochiko:loop-discipline`** and honor all four requirements (default-FAIL done-condition, independent validation, bounded iteration, named human gates), and brief each dispatch per **`agent-dispatch`**. Those rules are not restated here — this command states only what is specific to *this* workflow: the cycle sequence, the execute→verify pairing, the retry / fix-pass bounds, and the two implementation gates.

**Argument:** `$ARGUMENTS` = optional feature ID or description; else the feature is detected from the workspace. Empty input (the known `@`-reference drop bug) is recovered in Phase 0.

## Contract parameters (fill the artifact — don't inline it)

Fill `templates/workflow-contract.md` → `.mochiko/specs/<feature>/implement-contract.md` with the values below, then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable proof — not this command body.

- **Done-condition** — starts FAILing; clears only when **(1)** every cycle in `tasks.md` is complete (all tasks `[ ]` → `[x]`, each with its `cycle-report.md`), **(2)** `qa-engineer` verification passes on every cycle **and** on the final-validation run — real-infrastructure evidence + quality-gate exit codes, grounded in the workspace, **(3)** *you* Read the cycle-reports + verification reports and confirm no blocking gap remains: acceptance criteria met, quality gates pass, `tasks.md` fully `[x]`, traceability to requirements holds, and the implementation aligns with the constitution, **and (4)** the final-acceptance human gate (G5) has cleared. Out of rounds = escalate, never done.
- **Team** — producer `mochiko:staff-engineer` (`executing-tdd-cycle`, `brownfield-integration`) implements each cycle via TDD, never verifies; verifier `mochiko:qa-engineer` (`testing-end-user`) verifies against real infrastructure, never implements. Disjoint agents and skills — the verification skill is **never** mounted on staff.
- **Bounds** — **targeted retry:** on a cycle-checkpoint failure, trace it to its tasks and re-open only those (never rewrite passing code), **max 3 attempts per cycle**. **Fix-pass:** after a final-validation failure, a failure-scoped pass, **max 3 passes**. **Convergence-stall:** the same failure pattern across **2+ rounds** → surface, don't silently continue. Overall no-progress = an unchanged failing set; kill-switch `.mochiko/specs/<feature>/IMPLEMENT_STOP` checked before each dispatch. (You count every round.)
- **Gates** — G1 input recovery · G3 clarification (incl. the "Research this" knowledge-gap branch when staff flags ambiguity) · G4 exit-early / escalation on any guard trip · the **confidence gate** (per cycle: deterministic CLI verifications that 100% pass → auto-approve; GUI / subjective / any-failure → human checkpoint) · **G5** the named final-acceptance gate before "done." (No G2 — implement has a single verifier, so plan's feasibility-rejection slot is intentionally unused.)

> Why this done-condition differs from HIL's: HIL declared "no hard caps," routed on an autonomously-evaluated gate verdict, and had **no** final-acceptance gate — it could churn indefinitely or self-declare done. The deterministic caps, the lead-owned verdict (qa's status is input), and the new G5 acceptance gate close the gates HIL lacked.

## Phase 0 — Prerequisites & entry triage  *(human gate G1)*

1. **Capture** `$ARGUMENTS`; resolve `<feature>` (an explicit ID, else the most recent in-progress feature under `.mochiko/specs/`). If empty (the `@`-reference drop bug), recover via **G1**: ask the user to re-enter, or to confirm the detected feature.
2. **Entry gate — tasks-workflow-complete.** The task breakdown must be done: `.mochiko/specs/<feature>/tasks.md` present and complete (workspace evidence — there is no context-file `status` to read). Missing → block and point the user to `/mochiko:tasks`.
   **Entry variant — slice-scoped run (graduation slices).** If `.mochiko/specs/<feature>/slices.md` exists (accepted), the run is slice-scoped — honor that file's own **Graduation contract** section (the single source of the consumption rules; do not restate it). Resolve `<slice>` (named in `$ARGUMENTS`, else the first slice in Slice-order whose `slices/<slice>/tasks.md` has unchecked tasks) and check the **staleness guard**: the live `spec.md` story-ID set must match the Spec stamp — mismatch → block and point to `/mochiko:slice`. The entry gate and the cycle loop read `slices/<slice>/tasks.md`; the design inputs are the shared feature-root artifacts plus `slices/<slice>/{plan.md, task-mapping.md}`; cycle reports, verification reports, and the filled contract go under `slices/<slice>/`. The quality gates still run the **full repository suite** — earlier slices' tests are the regression net that catches a design amendment breaking shipped behavior, by construction. When this slice is the last in Slice-order and its G5 clears, the *feature* is still **declared, not verified**: `slices.md`'s Feature-Done section executes at feature-close (no workflow owns that pass yet — surface it as the next step; never report feature completion here).
3. **Constitution as governing context.** Read `.mochiko/memory/constitution.md` and carry its principles into every producer/verifier brief. Missing → surface it (offer `/mochiko:setup`); this is governing context, not a blocking gate — do not auto-resolve.
4. **Read the design inputs** (`plan.md`, `task-mapping.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `requirements.md`) under `.mochiko/specs/<feature>/` as the producer's inputs — workspace-as-state, no registry field.
5. **Project scaffolding.** From the detected stack, create any missing ignore-files (`.gitignore` / `.dockerignore` / lint-ignore) — project-relative outputs, one-time before the cycle loop.

## Phase 1 — Cycle loop  *(you own the sequence and each cycle's round counter)*

**Sequencing.** Cycles run in dependency order: **all foundation cycles before feature cycles**; the *current* cycle = the first with unchecked `tasks.md` tasks. **Sequential-only** — one cycle at a time; parallel cycle execution (native-Workflow `parallel()`) is a deliberate `deliberate-shortcut-ledger` deferral pending dogfooding, **not** a capability drop.

For each cycle, `round = 1`; the cycle is FAIL until verified:

1. **Implement.** Dispatch `mochiko:staff-engineer` to execute the cycle's task list through red/green/refactor TDD, briefed per `agent-dispatch`: the cycle's tasks + per-task file paths, the design inputs, the constitution as governing context, `[EXTEND]`/`[MODIFY]` brownfield markers when present, and — on round > 1 — the checkpoint's failed tasks for **targeted retry** (re-open only those; don't regress passing code). Emits `cycle-report.md`.
2. **Verify — same round, never skipped.** Dispatch `mochiko:qa-engineer` to verify the cycle against real infrastructure: execute the cycle's `**TEST:**` tasks, run the quality gates (lint / build / test), capture evidence → verification report + a checkpoint recommendation. Every staff cycle is paired with a qa verification in the same round.
3. **Confidence gate + verdict (you).** Read `cycle-report.md` + the verification report + qa's evidence. Apply the **confidence gate**: qa classifies each verification (the CLI / GUI / SUBJECTIVE classification procedure lives in `testing-end-user`); if every verification is a deterministic CLI check that passed 100%, **auto-approve** and advance to the next cycle; if any is GUI / subjective, or anything failed, **checkpoint to the human**. On a pass verdict → next cycle. On a checkpoint failure → **targeted retry** (step 1, failed tasks only; increment `round`), applying the bounds (max 3/cycle; a 2+-round stall → surface). Route knowledge / preference / scope gaps per `loop-discipline` (→ **G3** / **G4** / escalate).

## Phase 2 — Final validation & fix-pass loop  *(you own the round counter)*

Reachable when every cycle has cleared. `round = 1`; final-validation is FAIL until proven.

1. **Final validation.** Dispatch `mochiko:qa-engineer` for a whole-implementation verification — the full quality gates + the cross-cutting `**TEST:**` verifications against real infrastructure → verification report.
2. **Verdict (you).** Read the report + confirm the done-condition's end state: all `tasks.md` `[x]`, quality gates pass, traceability to requirements holds, constitution alignment. Clear → the **G5 acceptance gate**.
3. **Fix-pass on failure.** Dispatch `mochiko:staff-engineer` in a **fix pass** — scoped strictly to the reported failures, **unconstrained by cycle boundaries** (may touch files from any cycle), briefed with the final-validation failures; then re-verify (step 1). Increment `round`; apply the bounds (max 3 fix passes; a 2+-round stall on the same failure → surface / escalate).

**Mid-loop gates (both phases).** **G3** clarification: when staff flags an ambiguity, collect input (logged in-session) and feed it into the next dispatch — a "Research this" answer routes the knowledge gap to native `Explore` (per `loop-discipline`), never to the user. **G4** exit-early / escalation: on a guard trip (cap / no-progress / kill-switch) or stalled failures, present the last evidence and let the user continue-refining / accept-with-noted-gaps / stop-and-review — the run stays FAIL unless the human explicitly accepts. Neither ends the loop on its own.

## Phase 3 — Final acceptance  *(human gate G5 — the named final-acceptance gate)*

Reachable only after your clearing verdict on final-validation. Present the verified implementation (cycle / task / fix-pass counts, quality-gate results, an evidence summary, any noted gaps) and ask the user to **accept** (→ Phase 4; the done-condition is now satisfied), **amend** (re-enter the relevant cycle or fix-pass with the changes as the failure list — still bounded; it must clear a verdict again), or **reject** (abort; the work remains under `.mochiko/specs/<feature>/` and the working tree).

## Phase 4 — Finalize

Report the outputs (the working code + a `cycle-report.md` per cycle + the final verification report), the per-cycle and fix-pass round counts, the cycle / task / fix-pass counts + quality-gate status, a suggested commit (`feat: implement <feature>`), and the next step. Never modify git or push.

## State recovery

Resume from workspace evidence (there is no context-file `phase`/`status`):

| Evidence in the workspace | Resume at |
|---------------------------|-----------|
| no `.mochiko/specs/<feature>/tasks.md` | Phase 0 (entry blocked) |
| `slices.md` present | slice-scoped: resolve the current slice (the Phase-0 entry variant); the rows below then read `slices/<slice>/tasks.md` and per-slice reports |
| `tasks.md` present, ignore-files absent | Phase 0 (scaffolding) |
| unchecked tasks remain, current cycle has no `cycle-report.md` this round | Phase 1 (implement current cycle) |
| current cycle's `cycle-report.md` present, no verification report this round | Phase 1 (verify current cycle) |
| current cycle not passed, within the cap | Phase 1 (retry / loop control) |
| all `tasks.md` `[x]`, no final verification report | Phase 2 (final validation) |
| final-validation failed, within the cap | Phase 2 (fix-pass / loop control) |
| final-validation cleared, not yet accepted | Phase 3 |
| accepted | Phase 4 |
| `.mochiko/specs/<feature>/IMPLEMENT_STOP` present | escalate (G4) |

---

**What you own (not the agents):** the cycle sequence (foundation before feature; current = first unchecked) and each cycle's loop (round counter, no-progress check, retry cap, kill-switch, escalation); the execute→verify pairing (every staff cycle followed by a qa verification in the same round, never skipped); the verdict (qa grades from real infrastructure, you Read the cycle-reports + verification reports and decide against the default-FAIL done-condition — cycle-checkpoint = criteria-met + gates pass; final-validation = all `[x]` + gates + traceability + constitution alignment; qa's status is input); the fix-pass routing and its max-3 bound; the two implementation gates (the per-cycle **confidence gate** auto-approve-vs-checkpoint placement, the named final-acceptance **G5**) plus G1 / G3 / G4; the tasks-complete entry gate and the constitution / design-input prerequisites; project scaffolding; verifying each dispatch actually wrote its expected files (a missing output → log and ask retry/abort); and never mounting a verification skill on staff or letting staff grade its own cycle. Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool (never inline agent behavior); do not modify git or push. Full rules: `mochiko:loop-discipline`.
