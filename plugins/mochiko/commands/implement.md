---
description: Execute an accepted task breakdown into working, verified code via an independent producer→verifier team loop — a standing staff-engineer seat implements each cycle through red/green/refactor TDD (foundation cycles before feature cycles) then fix-passes a final validation, a standing qa-engineer seat verifies each cycle and the whole implementation independently against real infrastructure, with a confidence-based per-cycle gate and a named final-acceptance gate; tasks-gated, cycle-by-cycle, default-FAIL, bounded, kernel-free. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Implement — Execute the Task Breakdown (Cycle-by-Cycle, Foundation → Feature)

**Goal:** turn an accepted `tasks.md` into working, verified code — one cycle at a time,
foundation cycles before feature cycles, each **implemented** through red/green/refactor TDD
then **independently verified** against real infrastructure with captured evidence and
quality-gate exit codes, until every cycle clears and a whole-implementation final validation
passes. `$ARGUMENTS` = optional feature ID or description; empty or detected-from-workspace is
handled by triage below.

**You are the lead**, and this is a **team-form command in the mochiko command shape**: Read
`${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md` (both layers) before anything else — the
shape's rules bind here and are not restated; this file carries only implement's parameters. You
own the loop (the cycle sequence, each cycle's round counter, the fix-pass round counter, verdict,
escalation) and every human gate. This is a `mochiko:loop-discipline` sound loop; the Contract
section below is its authoring-time fill.

## Team-form parameters (shape Layer 2)

Hard-require `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` per the shape. The **authoritative first-spawn
probe** is the producer (`staff-engineer`) — always the first seat filled (foundation cycle 1 is
implemented before it is verified). Transport mechanics + the addressability check:
`templates/agent-dispatch.md` (Seat transport). The no-fallback bet is the same `Contested`
dogfood-pilot ruling as the other team-form commands.

## Session constraints

- Workspace: resolve `<feature>` (an explicit ID from `$ARGUMENTS`, else the most recent
  in-progress feature under `.mochiko/specs/`); the working code + reports live alongside the
  `tasks.md` they build from.
- Kill-switch: stop and escalate if `.mochiko/specs/<feature>/IMPLEMENT_STOP` exists — check before
  each seat send.
- **Deliverables & IDs:** the **working code** (the deliverable) + a `cycle-report.md` per cycle +
  per-cycle verification reports + the final-validation verification report, in the cycle / task
  (`T{N}.{X}`) ID namespace of `tasks.md` — `tasks.md`'s checkboxes flip `[ ]` → `[x]` as tasks
  complete. The producer's honest `cycle-report.md` (from `mochiko:executing-tdd-cycle`'s format —
  difficulties, deviations, flagged blockers) is the producer-authored uncertainty carrier (the
  shape's producer-authored branch), not confidence marks. qa's evidence + status live in the
  verification reports.

## The seats

- **producer** — `mochiko:staff-engineer` (`executing-tdd-cycle`, `brownfield-integration`), one
  **named standing seat across the whole cycle sequence and the Phase-2 fix-pass loop**. Each cycle:
  execute the cycle's task list through red/green/refactor TDD → `cycle-report.md`; on a
  cycle-checkpoint failure, targeted retry of only the failed tasks (same seat, never rewriting
  passing code); in Phase 2, a fix pass scoped to the final-validation failures, unconstrained by
  cycle boundaries. The standing seat carries the **accumulating implementation** forward — the
  conventions the foundation cycles set flow into the feature cycles, and a fix pass that may touch
  any cycle's files draws on whole-implementation knowledge rather than re-reading the growing
  codebase cold each cycle. Brief it per `agent-dispatch`: the cycle's tasks + per-task file paths,
  the design inputs, the governance obligated-read line (per the prerequisite), `[EXTEND]`/`[MODIFY]`
  brownfield markers when present; round > 1 within a cycle carries the checkpoint's failed tasks for
  targeted retry (fix the flagged tasks; don't regress passing code). It never verifies.
- **verifier** — `mochiko:qa-engineer` (`testing-end-user`), spawned **cold at the first cycle
  verification**, never in contact with the producer, one **named standing seat across all cycles and
  the final validation**. Each cycle: verify against real infrastructure — execute the cycle's
  `**TEST:**` tasks, run the quality gates (lint / build / test), capture evidence → verification
  report + a checkpoint recommendation. Phase 2: a whole-implementation final validation (full
  quality gates + the cross-cutting `**TEST:**` verifications). Its retained per-cycle context is what
  makes the final validation informed by what it already verified rather than a cold whole-repo read.
  Its output is **lead-adjudicated input** — qa's status is input, never the gate. The verification
  skill is **never** mounted on the producer, and staff never grades its own cycle.

## Phase 0 — Prerequisites & entry triage  *(human gate G1)*

1. **Capture** `$ARGUMENTS`; resolve `<feature>` (an explicit ID, else the most recent in-progress
   feature under `.mochiko/specs/`). If empty (the `@`-reference drop bug), recover via **G1**: ask
   the user to re-enter, or to confirm the detected feature.
2. **Entry gate — tasks-workflow-complete.** The task breakdown must be done:
   `.mochiko/specs/<feature>/tasks.md` present and complete (workspace evidence). Missing → block and
   point the user to `/mochiko:tasks`.
3. **Governance prerequisite.** Check `CLAUDE.md` for the mochiko governance region
   (`<!-- mochiko:governance:begin -->`). Present → governance reaches every spawned seat natively;
   add to each producer/verifier brief the **one-line obligated read** naming the
   `.claude/rules/mochiko/` files relevant to the cycle's file paths (code-touching seats also trigger
   `paths`-scoped rules by reading). Missing → do **not** silently proceed; surface it (offer
   `/mochiko:setup`). Governing context, not a blocking gate — never auto-resolve.
4. **Read the design inputs** (`plan.md`, `task-mapping.md`, `data-model.md`, `contracts/api.yaml`,
   `constraints-and-decisions.md`, `requirements.md`) under `.mochiko/specs/<feature>/` as the
   producer's inputs — workspace-as-state, no registry field.
5. **Project scaffolding.** From the detected stack, create any missing ignore-files
   (`.gitignore` / `.dockerignore` / lint-ignore) — project-relative outputs, one-time before the
   cycle loop.
6. **Slice-scoped entry (graduation slices).** If `.mochiko/specs/<feature>/slices.md` exists
   (accepted), the run is **slice-scoped** — apply that file's own **Graduation contract** section for
   slice resolution, the staleness guard, scope, extend-mode, graded amendment, and artifact layout;
   do not restate those rules here (the Graduation contract is their single home). **implement's own
   bindings on top:** the entry gate (step 2) and the cycle loop read `slices/<slice>/tasks.md`; the
   design inputs are the shared feature-root artifacts plus `slices/<slice>/{plan.md, task-mapping.md}`;
   per-slice outputs (`cycle-report.md` · the verification reports) land under `slices/<slice>/`, so the
   done-condition's artifact set reads them there. The quality gates still run the **full repository
   suite** — earlier slices' tests are the regression net that catches a design amendment breaking
   shipped behavior. When this slice is the last in Slice-order and its G5 clears, the *feature* is
   still **declared, not verified**: `slices.md`'s Feature-Done section executes at feature-close (no
   workflow owns that pass yet — surface it as the next step; never report feature completion here).

## Phase 1 — Cycle loop  *(you own the sequence and each cycle's round counter)*

**Sequencing.** Cycles run in dependency order: **all foundation cycles before feature cycles**; the
*current* cycle = the first with unchecked `tasks.md` tasks. **Sequential-only** — one cycle at a
time; parallel cycle execution (native-Workflow `parallel()`) is a deliberate
`deliberate-shortcut-ledger` deferral pending dogfooding, **not** a capability drop.

For each cycle, `round = 1`; the cycle is FAIL until verified:

1. **Implement.** The producer executes the current cycle's task list through red/green/refactor TDD
   (`cycle-report.md`); on round > 1 the message carries the checkpoint's failed tasks for **targeted
   retry** — re-open only those, don't regress passing code. The round-1 (foundation cycle 1) spawn is
   the authoritative probe — confirm addressability.
2. **Verify — same round, never skipped.** Message the verifier to verify the cycle against real
   infrastructure: execute the cycle's `**TEST:**` tasks, run the quality gates, capture evidence →
   verification report + checkpoint recommendation. Every produced cycle is paired with a qa
   verification in the same round (the first verification cold-spawns the qa seat).
3. **Confidence gate + verdict (you).** Read `cycle-report.md` + the verification report + qa's
   evidence. Apply the **confidence gate**: qa classifies each verification (the CLI / GUI / SUBJECTIVE
   classification procedure lives in `testing-end-user`); if every verification is a deterministic CLI
   check that passed 100%, **auto-approve** and advance to the next cycle; if any is GUI / subjective,
   or anything failed, **checkpoint to the human**. A non-empty `domain_deps_added` in the cycle
   report at `production`/`regulated` tier (tier: the CLAUDE.md governance region stamp) also forces
   the human checkpoint — a domain-registry addition is never auto-approved there; at lower tiers
   surface the additions in your verdict, non-blocking. On a pass verdict → next cycle. On a checkpoint
   failure → **targeted retry** (step 1, failed tasks only; increment `round`), applying the bounds
   (max 3/cycle; a 2+-round stall → surface). Route knowledge / preference / scope gaps per
   `loop-discipline` (→ **G3** / **G4** / escalate).

## Phase 2 — Final validation & fix-pass loop  *(you own the round counter)*

Reachable when every cycle has cleared. `round = 1`; final-validation is FAIL until proven.

1. **Final validation.** Message the verifier for a whole-implementation verification — the full
   quality gates + the cross-cutting `**TEST:**` verifications against real infrastructure →
   verification report.
2. **Verdict (you).** Read the report + confirm the done-condition's end state: all `tasks.md` `[x]`,
   quality gates pass, traceability to requirements holds, governance alignment. Clear → the **G5
   acceptance gate**.
3. **Fix-pass on failure.** Message the producer for a **fix pass** — scoped strictly to the reported
   failures, **unconstrained by cycle boundaries** (may touch files from any cycle), briefed with the
   final-validation failures; then re-verify (step 1). Increment `round`; apply the bounds (max 3 fix
   passes; a 2+-round stall on the same failure → surface / escalate).

**Mid-loop gates (both phases).** **G3** clarification: when staff flags an ambiguity, collect input
(logged in-session) and feed it into the next dispatch — a "Research this" answer routes the knowledge
gap to native `Explore` (per `loop-discipline`), never to the user. **G4** exit-early / escalation: on
a guard trip (cap / no-progress / kill-switch) or stalled failures, present the last evidence and let
the user continue-refining / accept-with-noted-gaps / stop-and-review — the run stays FAIL unless the
human explicitly accepts. Neither ends the loop on its own.

## Phase 3 — Final acceptance  *(human gate G5 — the named final-acceptance gate)*

Reachable only after your clearing verdict on final-validation. Present the verified implementation
(cycle / task / fix-pass counts, quality-gate results, an evidence summary, any noted gaps) and ask
the user to **accept** (→ Phase 4; the done-condition is now satisfied), **amend** (re-enter the
relevant cycle or fix-pass with the changes as the failure list — still bounded; it must clear a
verdict again), or **reject** (abort; the work remains under `.mochiko/specs/<feature>/` and the
working tree).

## Phase 4 — Finalize

Report the outputs (the working code + a `cycle-report.md` per cycle + the final verification report),
the per-cycle and fix-pass round counts, the cycle / task / fix-pass counts + quality-gate status, a
suggested commit (`feat: implement <feature>`), and the next step. Never modify git or push.

## Contract (authoring-time fill — governed by `mochiko:loop-discipline`)

- **Done-condition:** default **FAIL**; clears only when **(1)** every cycle in `tasks.md` is complete
  (all tasks `[ ]` → `[x]`, each with its `cycle-report.md`), **(2)** `qa-engineer` verification passes
  on every cycle **and** on the final-validation run — real-infrastructure evidence + quality-gate exit
  codes, grounded in the workspace, **(3)** *you* Read the cycle-reports + verification reports and
  confirm no blocking gap remains: acceptance criteria met, quality gates pass, `tasks.md` fully `[x]`,
  traceability to requirements holds, and the implementation aligns with the project's governance (the
  CLAUDE.md governance region + its rules files) — qa's status is input, never the gate — **and (4)**
  the Phase-3 final-acceptance human gate (G5) has cleared. Out of rounds = escalate, never done.
- **Producer ↔ validator:** `staff-engineer` (executing-tdd-cycle, brownfield-integration) implements
  each cycle via TDD and fix-passes final validation, never verifies; a **single independent verifier**,
  not the producer — `qa-engineer` (testing-end-user) verifies against real infrastructure, never
  implements. Disjoint agents, disjoint skills, structurally separated (verifier cold-spawned at the
  first cycle verification, evidence/reports lead-routed, no producer↔verifier contact); the
  verification skill is **never** mounted on staff. **Validation model:** the bounded in-loop critique —
  qa's per-cycle verification + the final validation, unsized by design; no sized end-stage review (the
  shape's in-loop-critique branch).
- **Bounds:** **targeted retry** — on a cycle-checkpoint failure, trace it to its tasks and re-open only
  those, **max 3 attempts per cycle**. **Fix-pass** — after a final-validation failure, a
  failure-scoped pass, **max 3 passes**. **Convergence-stall** — the same failure pattern across **2+
  rounds** → surface, don't silently continue; no-progress = an unchanged failing set; kill-switch
  `IMPLEMENT_STOP` checked before each seat send. You count every round.
- **Human gates:** G1 input recovery + governance / entry surface · G3 clarification (incl. the
  "Research this" knowledge-gap branch when staff flags ambiguity) · G4 exit-early / escalation on any
  guard trip · the **confidence gate** (per cycle: deterministic CLI verifications that 100% pass →
  auto-approve; GUI / subjective / any-failure / a `production`+-tier domain-registry addition →
  human checkpoint) · **G5** the named final-acceptance
  gate before "done." **No G2** — implement has a single verifier, so plan's feasibility-rejection slot
  is intentionally unused.

## State recovery

Pause posture (per the shape): note the resume stage on the deliverable. Resume from workspace
evidence, respawning what the stage needs — a respawned producer
re-reads the cycle's tasks + design inputs (+ any failed-task list); a verifier respawn is cold by
design:

| Evidence in the workspace | Resume at |
|---------------------------|-----------|
| no `.mochiko/specs/<feature>/tasks.md` | Phase 0 (entry blocked) |
| `slices.md` present | slice-scoped: resolve the current slice (Phase 0 step 6); the rows below then read `slices/<slice>/tasks.md` and per-slice reports |
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

**What you own (not the seats):** the cycle sequence (foundation before feature; current = first
unchecked) and each cycle's loop (round counter, no-progress check, retry cap, kill-switch,
escalation); the execute→verify pairing (every produced cycle followed by a qa verification in the
same round, never skipped); the verdict against the default-FAIL done-condition (qa grades from real
infrastructure, you Read the cycle-reports + verification reports and decide — cycle-checkpoint =
criteria-met + gates pass; final-validation = all `[x]` + gates + traceability + governance alignment;
qa's status is input); the fix-pass routing and its max-3 bound; the two implementation gates (the
per-cycle **confidence gate** auto-approve-vs-checkpoint placement, the named final-acceptance **G5**)
plus G1 / G3 / G4; the tasks-complete entry gate and the governance / design-input prerequisites;
project scaffolding; verifying each seat actually wrote its expected files (a missing output → log and
ask retry/abort); and never mounting the verification skill on staff or letting staff grade its own
cycle. Full rules: `mochiko:loop-discipline`.
