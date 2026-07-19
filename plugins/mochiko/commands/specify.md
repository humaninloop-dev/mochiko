---
description: Create a feature specification via an independent author→critic loop (requirements-analyst writes spec.md, devils-advocate stress-tests it) with a human acceptance gate — sparse input is enriched first; default-FAIL, bounded, kernel-free.
disable-model-invocation: true
---

# Specify — Feature Specification

You are the **lead / supervisor** for producing a feature specification — you own the loop, the verdict, and the human gates. The spec is **authored** by `mochiko:requirements-analyst` and independently **stress-tested** by `mochiko:devils-advocate`; never let those two collapse into one agent. The critic *recommends* a status; **you own the clearing verdict** — its status is input, never the gate.

This is a mochiko **sound loop**: invoke **`mochiko:loop-discipline`** and honor all four requirements (default-FAIL done-condition, independent validation, bounded iteration, named human gates), and brief each dispatch per **`agent-dispatch`**. Those rules are not restated here — this command states only what is specific to *this* workflow.

**Argument:** `$ARGUMENTS` = the feature description (e.g. "let users export their data as CSV"). Empty or sparse is handled — Phase 0 enriches before authoring.

## Contract parameters (fill the artifact — don't inline it)

Fill `templates/workflow-contract.md` → `.mochiko/specs/<feature>/contract.md` with the values below, then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable proof — not this command body.

- **Done-condition** — starts FAILing; clears only when **(1)** `devils-advocate` recommends `ready` on `spec.md` grounded in the file, **(2)** *you* Read `spec.md` + the advocate report and confirm no blocking gap remains (the advocate's status is input, not the gate), and **(3)** the Phase-3 human acceptance has cleared. Out of rounds = escalate, never done.
- **Team** — producer `mochiko:requirements-analyst` (`authoring-requirements`, `authoring-user-stories`) authors, never grades; critic `mochiko:devils-advocate` (`review-specifications`) reviews from the file, never authors — it produces lead-adjudicated input, never the authoritative grade (the `review-*` family boundary). Disjoint agents and skills.
- **Bounds** — cap **3** rounds (you count); no-progress exit when the advocate's gap set is unchanged round-over-round; kill-switch `.mochiko/specs/<feature>/SPECIFY_STOP` checked before each dispatch.
- **Gates** — G1 input recovery · G2 clarification + preference decisions · G3 spec acceptance · escalation on any guard trip.

> Why this done-condition differs from HIL's: HIL exited on the State-Analyst's *autonomous* verdict with no human acceptance — it could self-declare done on pass 1, violating `loop-discipline` req. 1. The advocate's three-way status survives as input to your verdict, plus the new G3 acceptance gate.

## Phase 0 — Initialize & input triage  *(human gate G1)*

1. **Capture** `$ARGUMENTS` as the feature description. If empty (the known `@`-reference drop bug), recover via **G1**: ask the user to re-enter it, or to proceed and enrich from scratch.
2. **Governance prerequisite.** Check `CLAUDE.md` for the mochiko governance region (`<!-- mochiko:governance:begin -->`). Present → governance reaches every spawned producer natively (CLAUDE.md loads at spawn); add to the producer's brief the **one-line obligated read** naming the `.claude/rules/mochiko/` files and skills relevant to what it authors (`paths`-scoped rules don't fire for from-scratch authoring). Missing → do **not** silently proceed; surface it (offer `/mochiko:setup` first, or proceed ungoverned for this spec). Never auto-resolve.
3. **Input triage (you own this).** *Rich* (Who / Problem / Value clear from the description and/or the governance region's domain context) → skip to Phase 2. *Sparse* (missing Who/Problem/Value, or a one-line request) → Phase 1. Governance context can substitute for enrichment.

## Phase 1 — Enrichment  *(sparse input only)*

Invoke `Skill(mochiko:analysis-iterative)` to surface Who / Problem / Value and the feature shape interactively, and carry the enriched description forward in-session. Enrichment runs **once, pre-loop** — the loop's own critique drives later rounds; do not re-enrich after round 1. The lead invokes this because enrich-or-not is loop-entry triage; it conditions the input, it does not author or grade.

## Phase 2 — Spec loop  *(the sound loop; you own the round counter and the verdict)*

1. **Workspace.** Derive a kebab-case `<feature>` slug from the (now clear) description, `mkdir -p .mochiko/specs/<feature>`, and seed `spec.md` from `templates/spec-template.md`. Initialize `round = 1`; the spec is FAIL until proven otherwise. (No feature-numbering script — workspace-as-state replaces it.)
2. **Produce.** Dispatch `mochiko:requirements-analyst` to author `spec.md` (+ `analyst-report.md` from its template), briefed per `agent-dispatch`: the feature description (enriched where applicable), the governance obligated-read line (per the prerequisite), the template to fill (prioritized P1/P2/P3 user stories with Given-When-Then, FR-XXX requirements, measurable SC-XXX, edge cases; technology-agnostic; no placeholder tokens), and — on round > 1 — the advocate's gap list for targeted revision (fix the flagged gaps; don't regress sections that already passed). Then the **G2 clarification sub-gate**: if the producer raises questions it cannot resolve, ask the user and feed the answers into the next dispatch — this never ends the loop on its own.
3. **Critique.** Dispatch `mochiko:devils-advocate` to stress-test `spec.md` — it Reads the spec file itself, never the author's report — writing `advocate-report.md` (from its template) with severity-bucketed gaps, clarifying questions, genuine strengths, and a recommended status (`ready` / `needs-revision` / `critical-gaps`).
4. **Verdict (you).** Read `spec.md` + `advocate-report.md` directly. If advocate `ready` **and** you find no unresolved blocking gap → Phase 3. Otherwise classify each gap and route it per `loop-discipline`'s gap-routing (knowledge → native `Explore`; preference → G2; scope → escalate), then apply the bounds: increment `round`; if the cap (3) / no-progress / kill-switch tripped → escalate; else loop to step 2 with the classified-gap list. `critical-gaps` → escalate.
5. **Escalate** (on any guard — never declare done on exhaustion): present the advocate's last gaps and the stop reason to the user (give-guidance-and-retry / accept-with-noted-gaps / abort). The run stays FAIL unless the human explicitly accepts.

## Phase 3 — Spec acceptance  *(human gate G3)*

Reachable only after your clearing verdict. Present the validated spec (user-story and FR counts, outstanding clarifications) and ask the user to **accept** (→ Phase 4; the done-condition is now satisfied), **amend** (re-enter Phase 2 with the requested changes as the gap list — still bounded; it must clear a verdict again), or **reject** (abort; the draft remains under `.mochiko/specs/<feature>/`).

## Phase 4 — Finalize

Report the artifacts (`spec.md` deliverable + `analyst-report.md` + `advocate-report.md`), the round count, the user-story / FR counts, a suggested commit (`docs: specify <feature>`), and the next step (`/mochiko:plan`). Offer a lightweight retain/clean choice for the intermediate round reports; never offer to delete `spec.md` — it is the deliverable.

## State recovery

Resume from workspace evidence (there is no context-file `phase`):

| Evidence in the workspace | Resume at |
|---------------------------|-----------|
| no `.mochiko/specs/<feature>/` | Phase 0 |
| `spec.md` still the bare seeded template | Phase 2 (produce, round 1) |
| `spec.md` authored, no `advocate-report.md` this round | Phase 2 (critique) |
| `advocate-report.md` `needs-revision`/`critical-gaps`, within the cap | Phase 2 (loop control → produce) |
| `advocate-report.md` `ready`, not yet accepted | Phase 3 |
| accepted | Phase 4 |
| `.mochiko/specs/<feature>/SPECIFY_STOP` present | escalate |

---

**What you own (not the agents):** the loop (round counter, no-progress check, cap, kill-switch, escalation); the verdict (the advocate recommends a status *from* the spec, you Read the artifacts and decide against the default-FAIL done-condition); the human gates; input triage; gap classification; and never letting producer and critic collapse into one agent. Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool (never inline agent behavior); do not modify git or push. Full rules: `mochiko:loop-discipline`.
