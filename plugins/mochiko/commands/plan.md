---
description: Turn one selected feature of an accepted spec into its accepted implementation package — analysis, a user-signed architecture, detailed design, and the task breakdown.
disable-model-invocation: true
---

# Plan — Implementation Package

**Goal:** turn one selected feature of an accepted spec into its accepted implementation
package — analysis, architecture, detailed design, and the task breakdown. One run per
selected feature, in the map's dependency order. `$ARGUMENTS` = the feature ID (`FEAT-XXX`);
empty → resolve the next undelivered selected feature from the spec's Feature Selection and
confirm with the user.

## Goal

The package exists. **Shared artifacts accumulate at spec root** (`.mochiko/specs/<spec>/`):
`requirements.md` (FR→TR) · `constraints-and-decisions.md` (C-XXX / D-XXX / IP-XXX) ·
`nfrs.md` (NFR-XXX) · `data-model.md` · `contracts/api.yaml` · `quickstart.md` when a real
external-integration surface exists (its null path recorded in `plan.md`) — created scoped
by the selection's first feature; every later feature's run opens them in **extend-mode**:
read first, extend in place, never re-derive a parallel copy. **Per-feature artifacts land
under `features/FEAT-XXX/`**: `architecture.md` (the delta) — **signed off by the user, on a
rendered diagram, before any detailed design was built on it** · `tasks.md` as **cycle
cards** — per card: stories + feature rationale, foundation/feature type, dependencies,
acceptance criteria by ID, a `**TEST:**` real-infrastructure gate, cycle-level brownfield
exposure; no task lists, no file paths — the builder decomposes at build time · `plan.md`, a
summary over the validated artifacts, never new design. The feature's **map entry was
confirmed and hardened** alongside the architecture — architecture link filled, extent
sharpened, intended-vs-designed drift surfaced to the user; status stays as spec acceptance
set it. The package was independently graded — feasibility and completeness — traces the
business requirements through to the task breakdown, carries no cross-artifact
contradiction, conforms to the signed-off architecture, and — where the spec carries a
Screens & Flows manifest — traces the feature's FEAT-tagged binding rows into the design:
every SCR-XXX's data shown has a serving contract surface, every FLOW-XXX action a mutation
path, and every UX-bearing cycle card's `**TEST:**` gate names the FLOW-XXX paths it
verifies (pixels stay advisory, never traced). The user accepted it whole. It is
`/mochiko:implement`'s unchanged entry condition.

**Not done — default FAIL:** a missing artifact, or an unrecorded `quickstart.md` null path ·
a shared artifact re-derived instead of extended · an unsigned architecture, or a design
element contradicting the signed-off target · an earlier delivered feature's design broken
without its `[MODIFY]` amendment · a package never graded by anyone but its authors · user
acceptance not given.

## Harness

- **You are the lead.** Plan the run and orchestrate it toward the Goal; teammates or
  subagents per seat is your call. The architecture is designed and signed off **before**
  detailed design builds on it; a later contradiction with the signed-off target returns to
  the user for a consented amendment, never designed around silently. A design that breaks
  an earlier **delivered** feature is an explicit `[MODIFY]` amendment — named in `plan.md`,
  migration stated, surfaced at architecture sign-off — and it writes the marked delta on
  the affected feature's map entry.
- **Plan approval:** any seat that writes artifacts plans first and works only on a plan you
  approved; grading and fact-finding seats are exempt.
- **Independence:** no output is cleared by its author; grading reads the artifacts
  themselves — never the author's report — default FAIL.
- **Reserved to the user:** architecture sign-off, presented on a rendered diagram (no render
  surface → present source + component table and record it) · a governance conflict — conform,
  or amend/waive through `governance-ledger.md`; feature work never overrules the
  constitution · an infeasible grade — escalated as a business-level scope decision · a need
  conflicting with another spec's in-flight direction — amend the owning spec, or override ·
  package acceptance (done / amend / reject).
- **Entry:** an absent or unaccepted `spec.md` blocks — point to `/mochiko:specify`. A
  feature absent from the spec's Feature Selection, or a selected feature ordered earlier
  and not yet `delivered`, blocks — one run per feature, strictly sequential. A missing
  governance region is surfaced (offer `/mochiko:setup`), never auto-resolved; on a
  brownfield codebase a missing or stale `.mochiko/memory/codebase-analysis.md` is surfaced
  the same way — offer setup, or proceed greenfield with the warning logged.
- Suggest commits; never run git mutations, never push. User acceptance is plain blocking
  text, never a timed prompt.

## Bindings

- **Artifacts** as listed in the Goal; `plan.md` from `templates/plan-template.md`;
  `tasks.md` from `templates/tasks-template.md` per `mochiko:patterns-vertical-tdd`;
  `architecture.md`'s structure and scope bound are `mochiko:patterns-system-design`'s; the
  structural D-XXX rows live in `constraints-and-decisions.md`'s designated section.
- **Feature scope:** the spec's Feature Selection section and the feature's map entry govern
  scope and order; map machinery — entry shape, delta grammar, the in-flight fork — per
  `mochiko:authoring-feature-map`, never restated. **Cross-spec reach:** a run touching a
  feature last shipped by another spec reads that spec's artifacts, found via the entry's
  owning-spec provenance.
- **In-flight inputs:** a touched feature in flight (or delta-carrying) under another spec
  is an obligated read into that spec's artifacts — stories, plan, architecture delta —
  before design touches it: need covered by the planned extent → reference it, build against
  the planned contract · adjacent → a `proposed` delta sequenced behind that delivery ·
  conflicting → reserved to the user. No locks; only silent contradiction is prohibited.
- **UX input** (the spec's Screens & Flows section holds a manifest): an obligated design
  read — its binding rows (screens, data shown, actions) are requirements input to contracts
  and data-model; the `prototype/` app is reference, its pixels advisory. The run consumes
  its feature's FEAT-tagged SCR/FLOW rows.
- **Baseline:** repo-root `ARCHITECTURE.md` is the current-state seed; absent → the
  reconstructed baseline is confirmed with the user before a delta is designed on it, and
  lands as the initial `ARCHITECTURE.md` where the KM copy
  (`.mochiko/memory/knowledge-management.md`) exists.
- **In-flight pointer:** at architecture sign-off, add the feature's one-line pointer to
  repo `ARCHITECTURE.md`'s In-flight list per `mochiko:authoring-architecture`.
- **Register:** user-facing prose per `templates/output-style.md`.
- **Next step:** `/mochiko:implement`.
