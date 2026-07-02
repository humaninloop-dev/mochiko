# `/mochiko:brainstorm` Command — Design Synthesis

**Session**: 2026-07-02 · 5 structured questions, all recommendations accepted without contest
**Method**: `analysis-iterative` (brainstorm mode) — this document is itself the shape the designed command would produce on its plan-free exit

## Problem Statement

The ported `analysis-iterative` skill runs the adaptive questioning well, but leaves two gaps. First, nothing independent ever challenges the session's thinking — the lead is captured by the conversation's momentum, and the skill's inline "light challenge" is the same context talking to itself. Second, the session terminates in a one-size synthesis document whose only wired consumer is specify's enrichment phase, while real brainstorms conclude at different altitudes: some need a full spec, some are design-ready, some are a refactor brief, some are just-do-it work. The fix is a new thin command that wraps the skill in a mochiko sound loop with two adversarial gates and altitude-aware exit ramps.

## Context & Constraints

- **Reviewer doctrine is artifact-grounded**: the `devils-advocate` grades from a file, never from a conversation — so both adversarial gates need something frozen to attack (a decision digest, a drafted synthesis).
- **Thin-command altitude**: commands state only what is workflow-specific (~66–82 line precedent); discipline lives in `loop-discipline` and the skills. Kernel-free is non-negotiable.
- **Pipeline entry gates are strict and asymmetric** (verified in the command sources): `plan` blocks unless `spec.md` is present and accepted; `tasks` blocks on `plan.md` *and* feeds its producer `spec.md` plus all six plan artifacts; `implement` blocks on a cycle-structured `tasks.md`. A brainstorm can honestly fill the spec slot; it can never honestly produce `contracts/api.yaml` or a `**TEST:**`-structured `tasks.md`.
- **Reuse surface**: `analysis-iterative` (unchanged), `devils-advocate` (gains one mount), `spec-template.md` (plan-ramp fill target), `staff-engineer` + `qa-engineer` (implement-ramp mini-loop).

## Key Decisions

| Decision | Choice | Confidence | Rationale |
|----------|--------|------------|-----------|
| DA gate placement | **Two gates**: DA challenges a frozen *decision digest* at convergence; a separate *pressure test* hits the drafted synthesis | Confident | A single post-synthesis gate hardens the artifact but arrives after positions harden — it enriches the doc, not the session. A continuous shadow costs a dispatch per decision and fights the one-question-at-a-time flow. The digest freeze keeps gate 1 artifact-grounded per doctrine. |
| Exit-ramp timing | **Emergent**: after gate 1 resolves, the lead recommends a ramp from what the session revealed; user confirms at a named gate; the synthesis is drafted *after*, shaped for the destination | Confident | The altitude of the work is a *discovery* of the brainstorm, not a premise (rules out upfront declaration). A generic-synthesis-plus-adapter reproduces the exact fits-no-consumer problem being solved. |
| Pressure-test scope | **Substance + readiness, one exit-aware dispatch**: scenario stress, reality-grounding of factual assumptions against the codebase via `Explore`, and an exit-readiness audit per ramp | Confident | Readiness alone can pass a *wrong* finding. Reality-grounding catches the classic "we'll just extend X" where X doesn't work as assumed. The steelman lens is deliberately relocated to gate 1, where reopening a decision costs one question, not a redraft. |
| Handoff mechanism | **Per-ramp honest** (detail below): specify = rich input · plan = synthesis *is* a provenance-marked `spec.md` · tasks = one scoped entry variant · implement = direct staff↔qa mini-loop | Confident | A uniform stand-in is dishonest at depth (fakes six design artifacts); uniform entry-variants tax all three commands when two ramps need zero edits. Mechanism should match what is truthful at each depth. |
| Advocate procedure home | **One new skill, two branches**: `validation-brainstorm` — branch 1 digest challenge, branch 2 finding pressure-test; per-ramp readiness checklists in `references/` | Confident | Briefing-only reuse of `analysis-specifications` smuggles two new procedures through the prompt — the exact failure the skill library exists to prevent. Two separate skills is single-consumer sprawl against the strategy-family precedent. Consolidation-as-branches matches `authoring-constitution` and `analysis-iterative` themselves. |

### Proposed defaults (Assumed — amend freely; never explicitly confirmed in session)

| Decision | Choice | Confidence | Rationale |
|----------|--------|------------|-----------|
| Command name | `/mochiko:brainstorm` | Assumed | Matches the user-invoked verb; alternatives (`think`, `explore`) collide with native tool names. |
| Workspace | `.mochiko/brainstorms/<slug>/` (`digest.md`, `synthesis.md`, advocate reports); a plan-ramp exit writes `spec.md` into `.mochiko/specs/<feature>/`, deriving the feature slug at handoff | Assumed | Brainstorms precede features; the workspace transition happens only when a ramp makes it feature-shaped. This document dogfoods the convention. |
| Loop shape | Cap **2** challenge rounds + **2** pressure-test rounds; no-progress exit; kill-switch `BRAINSTORM_STOP`; gates G1 topic capture · G2 challenge resolution (live questions) · G3 ramp confirmation · G4 synthesis acceptance; escalation on any guard trip | Assumed | Lighter caps than specify's 3 — revision here is cheap conversation, not artifact rework. Structure per `loop-discipline`, filled via `workflow-contract`. |
| Who drafts | The **lead** runs the questioning inline (invoking `analysis-iterative`, as specify's Phase 1 does) and drafts both digest and synthesis; the advocate is the only reviewer dispatch | Assumed | The lead uniquely holds the conversation context. Precedent: `plan.md` is the plan lead's fill-target — a summary over validated content. Independence is preserved because the *grader* is never the drafter. |
| Templates | Plan ramp reuses `spec-template.md` (no new file); one shared handoff-brief template serves the tasks and implement ramps; the digest format is specified inside `validation-brainstorm`, not a standalone template | Assumed | Registry impact stays minimal: 1 command, 1 skill, ≤1 template, 1 advocate mount, 1 tasks entry variant. |
| `analysis-iterative` | Unchanged — the command consumes its brainstorm mode; enrichment mode keeps serving specify | Assumed | The skill survives without the command; the command adds the loop, gates, and ramps on top. |
| Constitution | Carried as governing context when present; **never a blocking gate** | Assumed | Brainstorms legitimately precede governance; unlike plan/tasks, there is no artifact chain to protect at entry. |

## The Composed Flow

```
Phase 0  Topic capture                                              (G1)
Phase 1  Adaptive questioning — lead runs analysis-iterative inline
Phase 2  Convergence signals → lead freezes decision digest to file
         → DA challenge (validation-brainstorm branch 1:
           assumptions, missing dimensions, passive acceptances,
           steelman of rejected roads)
         → surviving challenges return as LIVE questions            (G2)  [≤2 rounds]
Phase 3  Lead recommends exit ramp → user confirms/overrides        (G3)
Phase 4  Lead drafts destination-shaped synthesis
Phase 5  Pressure test (branch 2: scenario stress + reality-
         grounding via Explore + exit-readiness audit)                    [≤2 rounds]
Phase 6  Acceptance + handoff                                       (G4)
         ├─ specify   → packet as rich input to /mochiko:specify   (zero edits)
         ├─ plan      → spec.md w/ provenance header → /mochiko:plan (zero edits)
         ├─ tasks     → stamped packet → /mochiko:tasks entry variant (one edit)
         └─ implement → bounded staff↔qa mini-loop from the brief  (no command)
```

### Handoff mechanism by ramp

- **→ specify** — the packet is a rich feature description; specify's own Phase-0 triage classifies it "rich" and skips enrichment. Works today, zero edits.
- **→ plan** — the destination-shaped synthesis is drafted *against `spec-template.md`* and written as `.mochiko/specs/<feature>/spec.md` with a provenance header (brainstorm-produced; digest DA-challenged; spec-equivalence pressure-tested). `plan` and `tasks` read that slot natively — and the reviewer that blessed it is the *same advocate agent* specify's own loop would have used.
- **→ tasks** — legitimate only for design-light work (refactors, migrations — where plan's six artifacts would be N/A padding). One entry variant on `/mochiko:tasks`, scoped strictly to packets carrying the pressure-test's `tasks-ready` stamp.
- **→ implement** — *not* the implement command (its machinery presumes a cycle-structured `tasks.md`). A bounded `staff-engineer` ↔ `qa-engineer` mini-loop briefed from the packet, for just-do-it work. The pressure-test stamp here means "an engineer could code against this tomorrow."

The per-ramp readiness checklists in `validation-brainstorm/references/` are the single source of truth for what each stamp means — consumed by the pressure test, the tasks entry variant, and the mini-loop alike.

## Risks

- **The plan ramp writes a `spec.md` that never passed authoring-requirements discipline.** Mitigation: the spec-equivalence readiness checklist mirrors the spec template's structure (P1–P3 stories, Given/When/Then, FR-XXX, SC-XXX). If dogfooding shows brainstorm-born specs grade worse inside plan's reviewer loop, revisit — a `deliberate-shortcut-ledger` entry, not a silent assumption.
- **The implement-ramp mini-loop could bloat the command past thin altitude.** Mitigation: it is a dispatch pairing plus bounds, referencing `loop-discipline` — never restated discipline.
- **Gate-1 challenges attack decisions the user personally made.** G2 must present them as questions, not verdicts; a challenge the user overrules carries a `Contested` marker into the synthesis — the skill's own confidence vocabulary already covers this.
- **The tasks entry variant is the only cross-command edit** — scope it tightly to stamped packets so it can never become a general bypass around `/mochiko:plan`.

## Open Questions

- Does the mini-loop stay a final phase of the brainstorm command (proposed), or graduate to its own tiny command if other callers want direct execution later?
- Router (`mochiko` skill) update: the new command needs graded trigger phrases for discoverability — wording deferred to build time.
- Interaction with `audit` (the last unported workflow): none identified, but audit's scoping should check whether it wants the same digest-challenge branch.

## Recommended Next Steps

1. **Record the scoping** — BACKLOG.md entry pointing here (done this session).
2. **Build in workflow-first order**: `validation-brainstorm` skill (both branches + `references/` checklists) → `brainstorm` command (thin by construction, contract filled from `workflow-contract`) → the `/mochiko:tasks` entry variant → router/REGISTRY updates.
3. **On landing, record the structural decisions in ROADMAP.md Key Decisions** — the advocate-mount choice and the per-ramp handoff touch the agent↔skill composition axis, and the operating manual requires axis-touching decisions to be recorded there.
4. **Dogfood with a real design question** — natural candidate: scoping the `audit` cluster, the last unported workflow.
