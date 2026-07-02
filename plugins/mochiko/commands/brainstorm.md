---
description: Run a collaborative brainstorm as a sound loop — adaptive questioning (analysis-iterative) hardened by two independent adversarial gates (devils-advocate challenges the frozen decision digest, then pressure-tests the destination-shaped synthesis) with an emergent exit ramp (specify | plan | tasks | direct execution) and a human acceptance gate — default-FAIL, bounded, kernel-free.
disable-model-invocation: true
---

# Brainstorm — Adversarially-Hardened Thinking, Ramped Into the Pipeline

You are the **lead / supervisor** for a collaborative thinking session — you own the loop, the verdict, and the human gates. You run the questioning **inline** via `mochiko:analysis-iterative` and you draft the session's two artifacts (the decision digest, then the destination-shaped synthesis) as fill-targets over the session's decisions — which is exactly why every clearing step runs through `mochiko:devils-advocate` (`validation-brainstorm`), an independent reviewer who was never in the room and grades **from the files**, never from the conversation. The advocate *recommends* a status; **you own the clearing verdict** — its status is input, never the gate. Never clear your own draft without its grounded review.

This is a mochiko **sound loop**: invoke **`mochiko:loop-discipline`** and honor all four requirements (default-FAIL done-condition, independent validation, bounded iteration, named human gates), and brief each dispatch per **`agent-dispatch`**. Those rules are not restated here — this command states only what is specific to *this* workflow: the two adversarial gates, the emergent destination, and the per-ramp handoff.

**Argument:** `$ARGUMENTS` = the topic to think through. Empty is recovered in Phase 0.

## Contract parameters (fill the artifact — don't inline it)

Fill `templates/workflow-contract.md` → `.mochiko/brainstorms/<slug>/contract.md` with the values below, then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable proof — not this command body.

- **Done-condition** — starts FAILing; clears only when **(1)** `devils-advocate` recommends `ready` on `digest.md` (no outstanding challenge), **(2)** it recommends `ready` on the destination-shaped synthesis — all three pressure-test lenses hunted and every destination-checklist item evidenced — each grounded in the files, **(3)** *you* Read both artifacts + both reports and confirm no blocking gap remains, **and (4)** the Phase-5 human acceptance has cleared — plus, on the direct-execution ramp only, the mini-loop's verification passed and **G5** cleared. Out of rounds = escalate, never done.
- **Team** — producer: **you + the user** (the session co-authors `digest.md` and the synthesis; the lead's fill-target, per the plan.md precedent — a summary over challenged decisions, not free authoring). Reviewer: `mochiko:devils-advocate` (`validation-brainstorm`) grades from the files, never co-authors; disjoint from the lead's skill (`analysis-iterative`). On the direct-execution ramp: producer `mochiko:staff-engineer` (`executing-tdd-cycle`, `brownfield-integration`) implements, never verifies; verifier `mochiko:qa-engineer` (`testing-end-user`) verifies against real infrastructure, never implements.
- **Bounds** — cap **2** challenge rounds + **2** pressure-test rounds (you count; lighter than sibling caps — a revision here is conversation, not artifact rework); mini-loop **max 2** attempts; no-progress exit when a report's challenge/gap set is unchanged round-over-round; kill-switch `.mochiko/brainstorms/<slug>/BRAINSTORM_STOP` checked before each dispatch.
- **Gates** — G1 topic capture · G2 challenge resolution (live questions) · G3 destination confirmation · G4 synthesis acceptance + handoff · G5 direct-execution acceptance (that ramp only) · escalation on any guard trip.

> Why this loop exists (no HIL ancestor — the first net-new mochiko workflow): the bare skill concludes on convergence *feel* — the same context that led the session grades it, and its one-size synthesis has a single consumer. The two artifact-grounded adversarial gates fix the first gap; the emergent destination + per-ramp handoff fix the second. The skill itself is unchanged — it still runs the questioning.

## Phase 0 — Topic & workspace  *(human gate G1)*

1. **Capture** `$ARGUMENTS` as the topic. Empty (the known `@`-reference drop bug) → recover via **G1**: ask what we are thinking through.
2. **Constitution as governing context.** Read `.mochiko/memory/constitution.md` if present and carry its principles into the session and every brief. Missing → proceed; thinking may legitimately precede governance — this is **never** a blocking gate.
3. **Workspace.** Derive a kebab-case `<slug>` from the topic; `mkdir -p .mochiko/brainstorms/<slug>`.

## Phase 1 — Adaptive questioning  *(you, inline — no dispatch)*

Invoke `Skill(mochiko:analysis-iterative)` (general-analysis shape) and run the session yourself: one question per turn, format adapted to the user's state, per that skill's doctrine. You run this inline because the conversation *is* the production surface — it emits no reviewable artifact until you freeze one; the reviewer enters at the freeze points. On its convergence signals (answers turning confirmatory, no new dimensions) → Phase 2. Do **not** draft a synthesis yet.

## Phase 2 — Digest challenge loop  *(gate 1; you own the round counter and the verdict)*

`round = 1`; the digest is FAIL until proven.

1. **Freeze.** Write `digest.md`: every decision with its rationale, confidence mark (the skill's vocabulary — Confident / Assumed / Contested / Unsure / Deferred), the alternatives rejected, and open threads. Self-contained — the advocate sees only this file.
2. **Challenge.** Dispatch `mochiko:devils-advocate` (branch: **digest challenge**), briefed per `agent-dispatch`: Read `digest.md`, hunt the five challenge classes (assumptions, missing dimensions, passive acceptances, rejected-road steelman, inconsistencies), write `challenge-report.md` (advocate-report shape) with a live question per challenge and a recommended status.
3. **Verdict (you).** Read `digest.md` + `challenge-report.md`. Surviving challenges → **G2**: put each live question to the user **in-session** — this enrichment is what the gate exists for; brief Phase-1-style re-questioning is fine; fold the answers back into `digest.md` (an overruled steelman → mark that decision `Contested`). `ready` and no blocking challenge → Phase 3. Otherwise apply the bounds (increment `round`; cap **2** / no-progress / kill-switch → escalate) and loop to step 2 with the updated digest.

## Phase 3 — Destination  *(human gate G3)*

From the challenged decision set, **recommend the shallowest destination the work honestly needs**, with your reasoning: **specify** (requirements are still discovering themselves) · **plan** (spec-level clarity already reached) · **tasks** (design-light — no new entities, endpoints, external integrations, or NFR targets) · **direct execution** (one small bounded change-set). The user confirms or overrides; record the destination in `digest.md`. The synthesis is drafted only after this gate — shaped for its consumer.

## Phase 4 — Synthesis & pressure-test loop  *(gate 2; you own the round counter and the verdict)*

`round = 1`; the synthesis is FAIL until proven.

1. **Draft (you).** Fill the destination shape as `.mochiko/brainstorms/<slug>/synthesis.md` — a summary over the challenged decisions, not new thinking: **specify** → enriched feature description (Who / Problem / Value, out-of-scope, success sketch, decisions carried with their marks); **plan** → the spec shape from `templates/spec-template.md` + a provenance header; **tasks / direct execution** → `templates/handoff-brief-template.md`.
2. **Pressure-test.** Dispatch `mochiko:devils-advocate` (branch: **pressure test**, destination checklist named — specify → `feature-description` · plan → `specification-equivalence` · tasks → `task-derivability` · direct execution → `direct-execution`), briefed per `agent-dispatch`: Read `synthesis.md`, work all three lenses — scenario stress, reality-grounding against the actual codebase, and the named destination checklist (`validation-brainstorm` references) — write `pressure-report.md` with a recommended status.
3. **Verdict (you).** Read `synthesis.md` + `pressure-report.md`. Route findings per `loop-discipline`'s gap-routing: knowledge → native `Explore`; preference → ask the user in-session; **wrong-depth** → back to **G3**, re-shape for the shallower destination (counts as a round). `ready` and no blocking gap → Phase 5. Otherwise bounds (cap **2** / no-progress / kill-switch → escalate) and loop to step 1.

## Phase 5 — Acceptance & handoff  *(human gate G4; G5 on the direct ramp)*

Reachable only after your clearing verdict. Present the synthesis, the destination, and both reports; the user **accepts** (execute the ramp below), **amends** (re-enter the relevant phase with the changes as the gap list — still bounded; it must clear a verdict again), or **rejects** (abort; drafts remain under `.mochiko/brainstorms/<slug>/`).

- **specify** → the deliverable stays `synthesis.md`; next step `/mochiko:specify` with it as the feature description.
- **plan** → derive `<feature>`, `mkdir -p .mochiko/specs/<feature>`, promote `synthesis.md` → `spec.md` there (provenance header intact); next step `/mochiko:plan`.
- **tasks** → promote `synthesis.md` → `.mochiko/specs/<feature>/handoff-brief.md`; next step `/mochiko:tasks` (its design-light entry variant reads the stamp).
- **direct execution** → run the mini-loop here: dispatch `mochiko:staff-engineer` to implement the brief through red/green/refactor TDD (briefed per `agent-dispatch`; never verifies); then `mochiko:qa-engineer` to verify against real infrastructure with captured evidence and quality-gate exit codes. Apply the confidence gate (all-deterministic-CLI 100% pass → auto-approve; GUI / subjective / any failure → human checkpoint). On failure: targeted re-brief, **max 2** attempts, then escalate. Close with **G5**: present the verified change for final acceptance.

## Phase 6 — Finalize

Report the artifacts (`digest.md`, the deliverable — `synthesis.md` or its promoted form — and `challenge-report.md` / `pressure-report.md`), the per-gate round counts, the destination taken, a suggested commit (`docs: brainstorm <slug>`; the direct ramp: `feat: <slug>`), and the next step for the chosen ramp. Offer a retain/clean choice for the intermediate reports; never offer to delete the deliverable.

## State recovery

Resume from workspace evidence (there is no context-file `phase`/`status`):

| Evidence in the workspace | Resume at |
|---------------------------|-----------|
| no `.mochiko/brainstorms/<slug>/` | Phase 0 |
| workspace exists, no `digest.md` | Phase 1 |
| `digest.md` present, no `challenge-report.md` this round | Phase 2 (challenge) |
| challenge report not `ready`, within the cap | Phase 2 (loop control) |
| digest cleared, no destination recorded in `digest.md` | Phase 3 |
| destination recorded, no `synthesis.md` | Phase 4 (draft) |
| `synthesis.md` present, no `pressure-report.md` this round | Phase 4 (pressure-test) |
| pressure report not `ready`, within the cap | Phase 4 (loop control) |
| both cleared, not yet accepted | Phase 5 |
| accepted (deliverable promoted / mini-loop verified) | Phase 6 |
| `.mochiko/brainstorms/<slug>/BRAINSTORM_STOP` present | escalate |

---

**What you own (not the agents):** the session itself — the questioning, the digest freeze, the destination recommendation, the synthesis draft (you are the producer here, which is why nothing clears without the advocate's independent review); both loops (round counters, caps, no-progress checks, kill-switch, escalation); the verdicts (the advocate grades from the files, you Read the artifacts + reports and decide against the default-FAIL done-condition — its status is input); the human gates (G1–G5); the ramp mechanics (promotion paths, provenance headers, the mini-loop pairing — never mount a verification skill on staff, never skip qa, never let staff grade its own change); gap classification per `loop-discipline`; and verifying each dispatch actually wrote its expected files (missing → log and ask retry/abort). Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool (never inline agent behavior); do not modify git or push. Full rules: `mochiko:loop-discipline`.
