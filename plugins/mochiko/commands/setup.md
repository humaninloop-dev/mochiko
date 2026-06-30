---
description: Create or update the project constitution via an independent producer→validator loop with a human acceptance gate — brownfield-aware (greenfield | brownfield | amend).
---

# Setup — Project Constitution

You are the **lead / supervisor** for establishing or updating the project constitution — you own the loop, the verdict, and the human gates. The constitution is **authored** by `mochiko:principal-architect` and independently **graded** by `mochiko:validator`; never let those two collapse into one agent. The validator grades from the artifact file; **you own the done verdict.**

This is a mochiko **sound loop**: invoke **`mochiko:loop-discipline`** and honor all four requirements (default-FAIL done-condition, independent validation, bounded iteration, named human gates), and brief each dispatch per **`agent-dispatch`**. Those rules are not restated here — this command states only what is specific to *this* workflow.

**Argument:** `$ARGUMENTS` = optional setup request (e.g. "set up project governance", "amend the testing principle"). Empty is fine — Phase 0 detects the mode.

## Contract parameters (fill the artifact — don't inline it)

Fill `templates/workflow-contract.md` → `.mochiko/memory/contract.md` with the values below, then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable proof — not this command body.

- **Done-condition** — starts FAILing; clears only when **(1)** `mochiko:validator` returns PASS on `.mochiko/memory/constitution.md` graded from the file, **(2)** the Phase-4 human acceptance has cleared, **and (3)** in brownfield mode the Phase-2 analysis checkpoint was confirmed. Out of rounds = escalate, never done.
- **Team** — producer `mochiko:principal-architect` (`authoring-constitution`, `analysis-codebase`) authors, never grades; validator `mochiko:validator` (`validation-constitution`) grades from the file, never authors. Disjoint agents and skills. Record in the contract the validator's Tier-2 standing + its deterministic sub-checks (placeholder scan, three-part-structure presence, and — brownfield — tools/versions cross-checked against `codebase-analysis.md`).
- **Bounds** — cap **3** produce↔validate rounds (you count); no-progress exit when the validator's fix-list is unchanged round-over-round; kill-switch `.mochiko/memory/SETUP_STOP` checked before each dispatch.
- **Gates** — G1 mode-select · G2 analysis checkpoint (brownfield) · G3 constitution acceptance · G4 cleanup · escalation on any guard trip.

> Why this done-condition differs from HIL's: HIL exited on "no clarifications OR max-3 iterations reached" — LLM-controlled and defaulting to DONE on cap, violating `loop-discipline` req. 1. The clarification exchange survives only as an in-loop human gate, never the done-condition; plus the new G3 acceptance gate.

## Phase 0 — Detect & mode-select  *(human gate G1)*

1. `mkdir -p .mochiko/memory`. Run stack detection (an *input*, not the quality gate): `bash ${CLAUDE_PLUGIN_ROOT}/skills/analysis-codebase/scripts/detect-stack.sh .`. Count source files (the brownfield heuristic) and check for an existing `.mochiko/memory/constitution.md` (present → `amend`; absent → `create`).
2. **Brownfield heuristic:** source-file count > 5 **AND** a framework/ORM detected → suggest **brownfield**; otherwise default **greenfield**.
3. **G1 mode-select:** present the detection result and let the user choose **brownfield** (full analysis, then author from real patterns), **greenfield** (constitution only, opinionated defaults), or **amend** (update the existing constitution without re-analysis).
4. **Route:** brownfield → Phase 1 · greenfield → Phase 3 · amend → Phase 3. Mode is carried in-session + the `.mochiko/memory/` workspace — there is no separate context-handoff file (HIL's `setup-context-*.md` is absorbed into this lead).

## Phase 1 — Brownfield analysis  *(brownfield only)*

Dispatch `mochiko:principal-architect` (briefed per `agent-dispatch`) to run `analysis-codebase` (mode: setup-brownfield) on the project, using the Phase-0 detect-stack output + file count as inputs: inventory existing patterns and entities, and assess the Essential Floor — Security, Testing, Error Handling, Observability — as present / partial / absent, using `codebase-analysis-template.md`. It writes `.mochiko/memory/codebase-analysis.md` and returns a summary + Essential-Floor table + entities + architecture pattern + any clarifications.

## Phase 2 — Analysis checkpoint  *(brownfield only · human gate G2)*

Present the analysis summary and let the user **confirm** (→ Phase 3), **edit** (collect corrections, re-run Phase 1 — bounded by the same round discipline as Phase 3; a human-correction gate, not a done-condition), or **reject** (fall back to greenfield, or abort). `codebase-analysis.md` is an **intermediate input**, not the deliverable: its gate is this human checkpoint plus the deterministic `detect-stack.sh` baseline — there is no separate analysis-validator (that would over-engineer an intermediate artifact). The **constitution** is the deliverable and gets the independent validator in Phase 3.

## Phase 3 — Constitution loop  *(all modes · the sound loop; you own the round counter)*

Initialize `round = 0`; the constitution is FAIL until proven otherwise.

1. **Produce.** Dispatch `mochiko:principal-architect` to run `authoring-constitution` → `.mochiko/memory/constitution.md`, briefed per `agent-dispatch` for the mode:
   - **brownfield** — Read `codebase-analysis.md`; use ACTUAL tools/versions/commands (never placeholder tokens); the four Essential-Floor principles + an Emergent Ceiling codifying existing good patterns; `project_type: brownfield`.
   - **greenfield** — opinionated defaults: Essential Floor (I–IV) + architectural principles (V–VII: Hexagonal Architecture, Single Responsibility, Dependency Discipline); concrete commands for the detected stack; `project_type: greenfield`.
   - **amend** — Read the existing constitution; preserve principles unless explicitly changing; bump the version (semver).
   Every principle carries Statement / Enforcement / Testability / Rationale with RFC-2119 keywords (MUST/SHOULD/MAY), using `constitution-template.md`. On round > 1, pass the validator's fix list for targeted revision (don't regress passing items). Then the **clarification sub-gate** (human, in-loop, NOT the done-condition): if the producer raises questions it cannot resolve, ask the user and feed answers into the next dispatch.
2. **Validate.** Dispatch `mochiko:validator` to run `validation-constitution` on the file (it Reads the artifact, never the author's report): three-part structure on every principle, anti-pattern scan, placeholder scan, threshold quantification, the semver-bump call, and — brownfield — named tools/versions cross-checked against `codebase-analysis.md`. Returns binary PASS/FAIL + fix list + version-bump call.
3. **Loop control.** PASS → record the verdict + version bump → Phase 4. FAIL → increment `round`; if the cap (3) / no-progress / kill-switch tripped → **escalate** (present the last fix list + stop reason; options: give-guidance-and-retry / accept-with-noted-gaps / abort; the run stays FAIL unless the human explicitly accepts); else loop to step 1 with the fix list.

## Phase 4 — Constitution acceptance  *(human gate G3)*

Reachable only on validator PASS. Present the validated constitution (version, principle count, Essential-Floor status) and let the user **accept** (→ Phase 5; the done-condition is now satisfied), **amend** (re-enter Phase 3 with the changes as the fix list — still bounded; it must PASS again), or **reject** (abort; the draft remains under `.mochiko/memory/`).

## Phase 5 — Finalize  *(human gate G4)*

1. **Report** (per mode), read from the accepted constitution: the artifacts (`constitution.md` deliverable + brownfield `codebase-analysis.md`), principle count + Essential-Floor status, the independent-validation PASS + human acceptance, a suggested commit (`docs: create constitution [version]` / `docs: update constitution to [version]`), and the next step (`/mochiko:specify`).
2. **Cross-cutting follow-ups — documented stubs, NOT wired this run** (referenced so nothing is silently lost; do not dispatch — there are no live skill mounts): **CLAUDE.md sync** (the `syncing-claude-md` cluster — the constitution template still carries its own `## CLAUDE.md Synchronization` content; only the operational sync action is deferred) and **evolution roadmap** (the `authoring-roadmap` cluster + `evolution-roadmap-template.md`).
3. **Cleanup (G4):** `.mochiko/memory/` holds durable state (`constitution.md`, and brownfield `codebase-analysis.md`) — no ephemeral context file to delete. Offer retain / remove-analysis-only; never offer to delete `constitution.md` — it is the deliverable.

## State recovery

Resume from workspace evidence (no context-file `phase`):

| Evidence in the workspace | Resume at |
|---------------------------|-----------|
| `.mochiko/memory/` missing or empty | Phase 0 |
| brownfield chosen, `codebase-analysis.md` missing | Phase 1 |
| `codebase-analysis.md` present, not yet confirmed | Phase 2 |
| analysis confirmed (or greenfield/amend), no `constitution.md` | Phase 3 (produce) |
| `constitution.md` present, no recorded validator PASS | Phase 3 (validate) |
| validator PASS recorded, not yet accepted | Phase 4 |
| accepted | Phase 5 |
| `.mochiko/memory/SETUP_STOP` present | escalate |

---

**What you own (not the agents):** the loop (round counter, no-progress check, cap, kill-switch, escalation); the verdict (the validator grades from the artifact, you declare done against the default-FAIL done-condition); the human gates; the **mode** (greenfield / brownfield / amend — it selects the producer's branch and which phases run); and never letting producer and validator collapse into one agent. Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool (never inline agent behavior); do not modify git or push. Full rules: `mochiko:loop-discipline`.
