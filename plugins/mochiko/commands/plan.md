---
description: Turn an accepted spec into an accepted implementation package — analysis, a user-signed architecture, detailed design, and the task breakdown.
disable-model-invocation: true
---

# Plan — Implementation Package

**Goal:** turn an accepted `spec.md` into one accepted implementation package — analysis,
architecture, detailed design, and the task breakdown. `$ARGUMENTS` = optional feature ID;
empty → resolve from `.mochiko/specs/` and confirm with the user.

## Goal

The package exists under `.mochiko/specs/<feature>/`: `requirements.md` (FR→TR) ·
`constraints-and-decisions.md` (C-XXX / D-XXX / IP-XXX) · `nfrs.md` (NFR-XXX) ·
`architecture.md` — **signed off by the user, on a rendered diagram, before any detailed
design was built on it** · `data-model.md` · `contracts/api.yaml` · `quickstart.md` when a
real external-integration surface exists (its null path recorded in `plan.md`) · `tasks.md`
as **cycle cards** — per card: stories + slice rationale, foundation/feature type,
dependencies, acceptance criteria by ID, a `**TEST:**` real-infrastructure gate, cycle-level
brownfield exposure; no task lists, no file paths — the builder decomposes at build time ·
`plan.md`, a summary over the validated artifacts, never new design. The package
was independently graded — feasibility and completeness — traces the business requirements
through to the task breakdown, carries no cross-artifact contradiction, conforms to the
signed-off architecture, and — where the spec carries a Screens & Flows manifest — traces
its binding rows into the design: every SCR-XXX's data shown has a serving contract surface,
every FLOW-XXX action a mutation path, and every UX-bearing cycle card's `**TEST:**` gate
names the FLOW-XXX paths it verifies (pixels stay advisory, never traced). The user accepted
it whole. It is `/mochiko:implement`'s
unchanged entry condition.

**Not done — default FAIL:** a missing artifact, or an unrecorded `quickstart.md` null path ·
an unsigned architecture, or a design element contradicting the signed-off target · a package
never graded by anyone but its authors · user acceptance not given.

## Harness

- **You are the lead.** Plan the run and orchestrate it toward the Goal; teammates or
  subagents per seat is your call. The architecture is designed and signed off **before**
  detailed design builds on it; a later contradiction with the signed-off target returns to
  the user for a consented amendment, never designed around silently.
- **Plan approval:** any seat that writes artifacts plans first and works only on a plan you
  approved; grading and fact-finding seats are exempt.
- **Independence:** no output is cleared by its author; grading reads the artifacts
  themselves — never the author's report — default FAIL.
- **Reserved to the user:** architecture sign-off, presented on a rendered diagram (no render
  surface → present source + component table and record it) · a governance conflict — conform,
  or amend/waive through `governance-ledger.md`; feature work never overrules the
  constitution · an infeasible grade — escalated as a business-level scope decision · package
  acceptance (done / amend / reject).
- **Entry:** an absent or unaccepted `spec.md` blocks — point to `/mochiko:specify`. A
  missing governance region is surfaced (offer `/mochiko:setup`), never auto-resolved; on a
  brownfield codebase a missing or stale `.mochiko/memory/codebase-analysis.md` is surfaced
  the same way — offer setup, or proceed greenfield with the warning logged.
- Suggest commits; never run git mutations, never push. User acceptance is plain blocking
  text, never a timed prompt.

## Bindings

- **Artifacts** as listed in the Goal; `plan.md` from `templates/plan-template.md`;
  `tasks.md` from `templates/tasks-template.md` per `mochiko:patterns-vertical-tdd`;
  `architecture.md`'s structure and scope bound are `mochiko:patterns-system-design`'s; the
  structural D-XXX rows live in `constraints-and-decisions.md`'s designated section.
- **UX input** (the spec's Screens & Flows section holds a manifest): an obligated design
  read — its binding rows (screens, data shown, actions) are requirements input to contracts
  and data-model; the `prototype/` app is reference, its pixels advisory. Slice-scoped runs
  consume only their slice's SCR/FLOW rows.
- **Slice scope** (the spec's Delivery Slices section holds a decomposition): its Graduation
  contract is the single home for slice resolution, scope, extend-mode, and layout —
  `plan.md`, `architecture.md`, and `tasks.md` land under `slices/<slice>/`; the architecture
  delta seeds from the accumulated feature-root `architecture.md` / `ARCHITECTURE.md`, never
  per-slice from scratch.
- **Baseline:** repo-root `ARCHITECTURE.md` is the current-state seed; absent → the
  reconstructed baseline is confirmed with the user before a delta is designed on it, and
  lands as the initial `ARCHITECTURE.md` where the KM copy
  (`.mochiko/memory/knowledge-management.md`) exists.
- **In-flight pointer:** at architecture sign-off, add the feature's one-line pointer to
  repo `ARCHITECTURE.md`'s In-flight list per `mochiko:authoring-architecture`.
- **Register:** user-facing prose per `templates/output-style.md`.
- **Next step:** `/mochiko:implement`.
