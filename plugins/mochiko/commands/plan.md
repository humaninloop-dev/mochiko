---
description: Create the analysis→design implementation plan via an independent producer→two-reviewer loop (technical-analyst authors; principal-architect grades feasibility, devils-advocate grades completeness) with a human acceptance gate on plan.md — constitution-gated, default-FAIL, bounded, kernel-free.
disable-model-invocation: true
---

# Plan — Implementation Plan (Analysis → Design)

You are the **lead / supervisor** for producing a feature's implementation plan across two phases — Analysis then Design. You own the loop, the verdict, and the human gates. The six analysis+design artifacts are **authored** by `mochiko:technical-analyst` and independently graded by **two** reviewers: `mochiko:principal-architect` checks cross-artifact **feasibility** (can these pieces be built *together*?), `mochiko:devils-advocate` checks **completeness** (is anything missing?). Never let the producer grade its own output, and never collapse the two reviewers into the author. Each reviewer *recommends* a status; **you own the clearing verdict** — their status is input, never the gate.

This is a mochiko **sound loop**: invoke **`mochiko:loop-discipline`** and honor all four requirements (default-FAIL done-condition, independent validation, bounded iteration, named human gates), and brief each dispatch per **`agent-dispatch`**. Those rules are not restated here — this command states only what is specific to *this* workflow.

**Argument:** `$ARGUMENTS` = optional feature ID or description; else the feature is detected from the workspace. Empty input (the known `@`-reference drop bug) is recovered in Phase 0.

## Contract parameters (fill the artifact — don't inline it)

Fill `templates/workflow-contract.md` → `.mochiko/specs/<feature>/plan-contract.md` with the values below, then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable proof — not this command body.

- **Done-condition** — starts FAILing; clears only when **(1)** all six artifacts exist (`requirements.md` · `constraints-and-decisions.md` · `nfrs.md` · `data-model.md` · `contracts/api.yaml` · `quickstart.md`), **(2)** `principal-architect` returns `feasible` on the Phase-1 analysis **and** `devils-advocate` returns `ready` on both phases, each grounded in the files, **(3)** *you* Read the artifacts + both reviewer reports and confirm no blocking gap remains, **and (4)** the Phase-4 human acceptance on `plan.md` has cleared. Out of rounds = escalate, never done.
- **Team** — producer `mochiko:technical-analyst` (`authoring-technical-requirements`, `patterns-technical-decisions`, `patterns-entity-modeling`, `patterns-api-contracts`) authors both phases, never grades; feasibility reviewer `mochiko:principal-architect` (`validation-feasibility`) and completeness reviewer `mochiko:devils-advocate` (`validation-plan-artifacts`) grade from the files, never author. Disjoint agents and skills — two independent reviewers, neither the producer.
- **Bounds** — cap **3** produce↔review rounds per phase (you count); no-progress exit when a reviewer's gap set is unchanged round-over-round; kill-switch `.mochiko/specs/<feature>/PLAN_STOP` checked before each dispatch.
- **Gates** — G1 input recovery · G2 feasibility-rejection · G3 clarification (incl. the "Research this" knowledge-gap branch) · G4 exit-early offering · G5 plan.md acceptance · escalation on any guard trip.

> Why this done-condition differs from HIL's: HIL declared "no hard caps" and routed on each agent's verdict *field* — it could self-declare done at pass 1, violating `loop-discipline` reqs 1 & 3. The two reviewers' three-state statuses survive only as input to your verdict; the deterministic cap and the new G5 acceptance gate close the gates HIL lacked.

## Phase 0 — Prerequisites & entry triage  *(human gate G1)*

1. **Capture** `$ARGUMENTS`; resolve `<feature>` (an explicit ID, else the most recent in-progress feature under `.mochiko/specs/`). If empty (the `@`-reference drop bug), recover via **G1**: ask the user to re-enter, or to confirm the detected feature.
2. **Constitution prerequisite.** Read `.mochiko/memory/constitution.md`. Present → carry its principles into the producer's brief as governing context. Missing → do **not** silently proceed; surface it (offer `/mochiko:setup` first). Never auto-resolve.
3. **Entry gate.** The spec must be done: `.mochiko/specs/<feature>/spec.md` present and accepted (workspace evidence — there is no context-file `status` to read). Missing → block and point the user to `/mochiko:specify`.
4. **Brownfield check.** Read the constitution's `project_type`. Brownfield → require `.mochiko/memory/codebase-analysis.md` (missing → offer `/mochiko:setup` or proceed greenfield with a logged warning; >14d stale by file mtime → warn); greenfield → bypass. Carry the analysis into the producer's brief when present.

## Phase 1 — Analysis loop  *(you own the round counter and the verdict)*

Seed `.mochiko/specs/<feature>/`; `round = 1`; the analysis is FAIL until proven. The architect grades feasibility **once**, after the analysis is authored — not in Phase 2 (don't spend a completeness pass on infeasible requirements).

1. **Produce.** Dispatch `mochiko:technical-analyst` to author `requirements.md` (FR→TR mapping), `constraints-and-decisions.md` (Part 3 = IP-XXX infrastructure), and `nfrs.md` (+ `techanalyst-report.md`), briefed per `agent-dispatch`: `spec.md`, the constitution as governing context, the brownfield analysis when present, and — on round > 1 — the reviewers' gap list for targeted revision (fix flagged gaps; don't regress passing sections).
2. **Feasibility (architect, once).** Dispatch `mochiko:principal-architect` to grade cross-artifact feasibility from the files → `feasibility-report.md` (three-state `feasible` / `needs-revision` / `infeasible`).
3. **Completeness (advocate).** Dispatch `mochiko:devils-advocate` to grade completeness / coverage / consistency from the files → `advocate-report.md` (`ready` / `needs-revision` / `critical-gaps`).
4. **Verdict (you).** Read the artifacts + both reports. `feasible` **and** `ready` **and** you find no blocking gap → Phase 2. Otherwise classify each finding and route it per `loop-discipline`'s gap-routing (knowledge → native `Explore`, the "Research this" branch in G3; preference → G2/G3; scope → G4): architect concerns → **G2**; advocate gaps → **G3**; architect `infeasible` → escalate as a business-level scope decision, not a routine revision. Then apply the bounds (increment `round`; cap / no-progress / kill-switch → **G4** / escalate) and loop to step 1. **Re-run the architect (step 2) only on a structural change** — new/changed constraints, expanded requirement scope, or modified NFR targets; a clarification-only revision goes straight back to step 3.

## Phase 2 — Design loop  *(incremental review; you own the round counter)*

`round = 1`; the design is FAIL until proven. No architect re-review here — the advocate carries cross-artifact consistency in incremental mode.

1. **Produce.** Dispatch `mochiko:technical-analyst` to author `data-model.md` (entities + sensitivity annotations), `contracts/api.yaml` (OpenAPI + `x-integration` boundaries), and `quickstart.md` (+ `techanalyst-report.md`), briefed per `agent-dispatch` with the Phase-1 analysis as input and the design-phase skills (`patterns-entity-modeling`, `patterns-api-contracts`).
2. **Incremental review (advocate).** Dispatch `mochiko:devils-advocate` in **incremental mode** — a full review of the new design artifacts plus a brief consistency check back to the Phase-1 analysis (the `validation-plan-artifacts` incremental procedure); you select the mode and supply the {new design}/{prior analysis} artifact sets → `advocate-report.md`.
3. **Verdict (you).** Read the design artifacts + report. `ready` and no blocking gap → Phase 3. Otherwise route gaps per `loop-discipline` (→ **G3**), apply the bounds (cap / no-progress / kill-switch → **G4** / escalate), and loop to step 1.

**Mid-loop gates (both phases).** **G2** feasibility-rejection: present the architect's concerns and let the user accept-resolution / relax / keep-as-is / give-direction. **G3** clarification: present a reviewer's gaps, take answers (logged in-session), and feed them into the next produce — a "Research this" answer routes the knowledge gap to native `Explore` (per `loop-discipline`), never to the user. **G4** exit-early / escalation: on a guard trip or stalled gaps, present the last findings and let the user continue-refining / accept-with-noted-gaps / stop-and-review — the run stays FAIL unless the human explicitly accepts. None of these ends the loop on its own.

## Phase 3 — Assemble plan.md

After both verdicts clear, assemble the `plan.md` deliverable from `templates/plan-template.md`: extract the key decisions from `constraints-and-decisions.md`, the entity summary (with sensitivity) from `data-model.md`, and the endpoint summary (with integrations) from `contracts/api.yaml`. `plan.md` is the lead's fill-target — a summary over the validated artifacts, not new design.

## Phase 4 — plan.md acceptance  *(human gate G5)*

Reachable only after your clearing verdict. Present the validated plan (decision / entity / endpoint counts, any noted limitations) and ask the user to **accept** (→ Phase 5; the done-condition is now satisfied), **amend** (re-enter the relevant phase with the changes as the gap list — still bounded; it must clear a verdict again), or **reject** (abort; the drafts remain under `.mochiko/specs/<feature>/`).

## Phase 5 — Finalize

Report the artifacts (the six deliverables + `plan.md` + the three round reports `techanalyst-report.md` / `feasibility-report.md` / `advocate-report.md`), the per-phase round counts, the decision / entity / endpoint counts, a suggested commit (`docs: plan <feature>`), and the next step (`/mochiko:tasks`). Offer a lightweight retain/clean choice for the intermediate round reports; never offer to delete `plan.md` or the six artifacts — they are the deliverables.

## State recovery

Resume from workspace evidence (there is no context-file `phase`/`status`):

| Evidence in the workspace | Resume at |
|---------------------------|-----------|
| no `.mochiko/specs/<feature>/spec.md` | Phase 0 (entry blocked) |
| `spec.md` present, no `requirements.md` | Phase 1 (produce) |
| analysis artifacts present, no `feasibility-report.md` / `advocate-report.md` this round | Phase 1 (review) |
| analysis reviews not `feasible`+`ready`, within the cap | Phase 1 (loop control) |
| analysis cleared, no `data-model.md` | Phase 2 (produce) |
| design artifacts present, advocate not `ready`, within the cap | Phase 2 (loop control) |
| both phases cleared, no `plan.md` | Phase 3 |
| `plan.md` present, not yet accepted | Phase 4 |
| accepted | Phase 5 |
| `.mochiko/specs/<feature>/PLAN_STOP` present | escalate (G4) |

---

**What you own (not the agents):** the two-phase sequence (Analysis → Design) and the per-phase loop (round counter, no-progress check, cap, kill-switch, escalation); the verdict (each reviewer grades from the files, you Read the artifacts and decide against the default-FAIL done-condition — their status is input); the architect-feasibility-once-then-advocate ordering and the skip-architect-unless-structural routing; the Phase-2 incremental mode selection; the human gates (G1–G5); the constitution / entry / brownfield prerequisites; `plan.md` assembly; verifying each dispatch actually wrote its expected files (a missing output → log and ask retry/abort); and never letting the producer grade its own output or the two reviewers collapse into the author. Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool (never inline agent behavior); do not modify git or push. Full rules: `mochiko:loop-discipline`.
