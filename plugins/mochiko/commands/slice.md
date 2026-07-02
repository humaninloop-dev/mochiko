---
description: Decompose an accepted feature specification into graduation slices via an independent producer→reviewer loop (task-architect authors the slices.md overlay, devils-advocate grades the decomposition) with a human acceptance gate — spec-gated, null-exit-aware, default-FAIL, bounded, kernel-free. Downstream stages then run per slice instead of whole-spec.
disable-model-invocation: true
---

# Slice — Graduation-Slice Decomposition (Spec → Slices)

You are the **lead / supervisor** for decomposing an accepted spec into graduation slices — ordered story groups that graduate through `/mochiko:plan` → `/mochiko:tasks` → `/mochiko:implement` as independent units instead of one whole-spec batch. You own the loop, the verdict, and the human gates. The `slices.md` overlay is **authored** by `mochiko:task-architect` (the story→slice judgment: dependency-closed ordering, foundation designation, extend obligations, Feature-Done declaration) and independently graded by `mochiko:devils-advocate` (coverage, ordering, foundation legitimacy, SC coverage, depth). Never let the producer grade its own output. The reviewer *recommends* a status; **you own the clearing verdict** — its status is input, never the gate.

This is a mochiko **sound loop**: invoke **`mochiko:loop-discipline`** and honor all four requirements (default-FAIL done-condition, independent validation, bounded iteration, named human gates), and brief each dispatch per **`agent-dispatch`**. Those rules are not restated here — this command states only what is specific to *this* workflow: the decomposition loop, the null exit, and the graduation handoff.

**Argument:** `$ARGUMENTS` = optional feature ID or description; else the feature is detected from the workspace. Empty input (the known `@`-reference drop bug) is recovered in Phase 0.

## Contract parameters (fill the artifact — don't inline it)

Fill `templates/workflow-contract.md` → `.mochiko/specs/<feature>/slice-contract.md` with the values below, then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable proof — not this command body.

- **Done-condition** — starts FAILing; clears only when **(1)** EITHER `slices.md` exists conforming to `templates/slices-template.md` (exact spec stamp; every story homed exactly once; Feature-Done section complete) OR the **null exit** was taken — recommend whole-spec, no file written, reasoning disclosed in `slicer-report.md`, **(2)** `devils-advocate` returns `ready` on the decomposition (or, on a null-exit round, on the depth call), grounded in the files, **(3)** *you* Read `spec.md` + the artifacts + the report and confirm no blocking gap remains, **and (4)** the Phase-2 human acceptance (G5) has cleared — accepting either the decomposition or the whole-spec recommendation. Out of rounds = escalate, never done.
- **Team** — producer `mochiko:task-architect` (`authoring-slices`) authors the overlay, never grades; reviewer `mochiko:devils-advocate` (`validation-slices`) grades from the files, never authors. Disjoint agents and skills — a single reviewer, never the producer.
- **Bounds** — cap **3** produce↔review rounds (you count); no-progress exit when the reviewer's gap set is unchanged round-over-round; kill-switch `.mochiko/specs/<feature>/SLICE_STOP` checked before each dispatch.
- **Gates** — G1 input recovery · G3 clarification (incl. the "Research this" knowledge-gap branch) · G4 exit-early offering · G5 decomposition acceptance · escalation on any guard trip. (No G2 — slice is single-reviewer, so plan's feasibility-rejection slot is intentionally unused.)

> Why this workflow is net-new (no HIL ancestor): the pipeline's unit was the whole feature — every story crossed each stage together, so P1 stories could not reach verified code until the entire spec was planned and tasked, and whole-spec artifacts diluted producer and reviewer attention. This command creates the smaller unit; the downstream commands' slice-scoped entries consume it. Design record: `.mochiko/brainstorms/vertical-graduation/synthesis.md`.

## Phase 0 — Prerequisites & entry triage  *(human gate G1)*

1. **Capture** `$ARGUMENTS`; resolve `<feature>` (an explicit ID, else the most recent in-progress feature under `.mochiko/specs/`). If empty (the `@`-reference drop bug), recover via **G1**: ask the user to re-enter, or to confirm the detected feature.
2. **Entry gate — spec accepted.** `.mochiko/specs/<feature>/spec.md` must be present and accepted (workspace evidence — there is no context-file `status` to read). Missing or unaccepted → block and point the user to `/mochiko:specify`.
3. **Already-decomposed guard.** If `slices.md` already exists: no slice has graduated (no `slices/<id>/` stage artifacts) → offer re-decomposition (overwrite) or stop; **any slice already graduated → halt and escalate** — amending a live decomposition is deliberately unsupported for now (a recorded deferral, not an oversight; see `BACKLOG.md`).
4. **Constitution as governing context.** Read `.mochiko/memory/constitution.md` and carry its principles into the producer's brief. Missing → surface it (offer `/mochiko:setup`); this is governing context, not a blocking gate — do not auto-resolve.

## Phase 1 — Decomposition loop  *(you own the round counter and the verdict)*

`round = 1`; the decomposition is FAIL until proven.

1. **Produce.** Dispatch `mochiko:task-architect` to author `slices.md` from `templates/slices-template.md` (+ `slicer-report.md` from its template), briefed per `agent-dispatch`: `spec.md` as the input to Read, the constitution as governing context, and — on round > 1 — the reviewer's gap list for targeted revision (fix flagged gaps; don't regress passing slices). **Null exit:** the producer may instead recommend whole-spec (fewer than two distinct value seams) — no `slices.md` written, reasoning disclosed in `slicer-report.md`; the round proceeds to review either way.
2. **Review.** Dispatch `mochiko:devils-advocate` to grade the decomposition from the files (the `validation-slices` checks — story coverage, dependency closure, foundation legitimacy, sizing, extend-obligation visibility, Feature-Done SC coverage + seams, spec stamp, and the depth second-guess in both directions; on a null-exit round it grades the depth call from `spec.md` + the disclosed reasoning) → `advocate-report.md` (`ready` / `needs-revision` / `critical-gaps`).
3. **Verdict (you).** Read `spec.md`, `slices.md` (when written) + both reports. `ready` **and** no blocking gap → Phase 2. Otherwise route each gap per `loop-discipline`'s gap-routing (→ **G3**; an un-homeable cross-cutting story is a spec-amendment finding — surface it, offer `/mochiko:specify` amendment, don't force a placement), apply the bounds (increment `round`; cap / no-progress / kill-switch → **G4** / escalate), and loop to step 1. A **wrong-depth** finding flips the outcome shape (decompose ↔ whole-spec) — it counts as a round, not a special case.

**Mid-loop gates.** **G3** clarification: present the reviewer's gaps, take answers (logged in-session), and feed them into the next produce — a "Research this" answer routes the knowledge gap to native `Explore` (per `loop-discipline`), never to the user. **G4** exit-early / escalation: on a guard trip or stalled gaps, present the last findings and let the user continue-refining / accept-with-noted-gaps / stop-and-review — the run stays FAIL unless the human explicitly accepts. Neither ends the loop on its own.

## Phase 2 — Decomposition acceptance  *(human gate G5)*

Reachable only after your clearing verdict. Two shapes:

- **Decomposition produced** — present the validated overlay (slice count + names, foundation choice, story coverage, SC coverage, seams, any noted gaps) and ask the user to **accept** (→ Phase 3), **amend** (re-enter Phase 1 with the changes as the gap list — still bounded; it must clear a verdict again), or **reject** (abort; the draft remains under `.mochiko/specs/<feature>/`).
- **Null exit** — present the whole-spec recommendation and its reviewed reasoning; the user **accepts** (→ Phase 3; the pipeline runs whole-spec exactly as before — the done-condition is satisfied with no `slices.md`) or **overrides** (re-enter Phase 1 directed to decompose — still bounded).

## Phase 3 — Finalize

Report the outcome (`slices.md` + the slice / foundation / story counts and SC coverage — or the accepted whole-spec recommendation), the round count, the round reports (`slicer-report.md` / `advocate-report.md`), a suggested commit (`docs: slice <feature>`), and the next step: `/mochiko:plan <feature> --slice <foundation-id>` (or plain `/mochiko:plan <feature>` after a null exit). Note that feature-done is now **declared, not verified** — the Feature-Done section executes at feature-close, once every slice ships. Offer a lightweight retain/clean choice for the intermediate round reports; never offer to delete `slices.md` — it is the deliverable.

## State recovery

Resume from workspace evidence (there is no context-file `phase`/`status`):

| Evidence in the workspace | Resume at |
|---------------------------|-----------|
| no `.mochiko/specs/<feature>/spec.md` (or unaccepted) | Phase 0 (entry blocked) |
| `slices.md` present with graduated slices (`slices/<id>/` stage artifacts exist) | Phase 0 (halt — amend deferred) |
| spec accepted; no `slices.md` and no `slicer-report.md` | Phase 1 (produce) |
| `slices.md` (or a null-exit `slicer-report.md`) present, no `advocate-report.md` this round | Phase 1 (review) |
| reviewer not `ready`, within the cap | Phase 1 (loop control) |
| `ready`, not yet accepted | Phase 2 |
| accepted | Phase 3 |
| `.mochiko/specs/<feature>/SLICE_STOP` present | escalate (G4) |

---

**What you own (not the agents):** the loop (round counter, no-progress check, cap, kill-switch, escalation); the verdict (the reviewer grades from the files, you Read `spec.md` + the artifacts and decide against the default-FAIL done-condition — its status is input); the human gates (G1 / G3 / G4 / G5) including the two-shape acceptance; the spec-accepted entry gate, the already-decomposed guard, and the constitution prerequisite; routing the un-homeable-story spec-amendment finding; verifying each dispatch actually wrote its expected files (a missing output → log and ask retry/abort — a null-exit round expects `slicer-report.md` only); and never letting the producer grade its own output. Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool (never inline agent behavior); do not modify git or push. Full rules: `mochiko:loop-discipline`.
