---
description: Turn one capability-batch (a capability's selected work rows) carrying ratified scope into its accepted implementation package — its artifact set scaled to the batch by an approved plan-the-plan proposal (architecture user-signed when the proposal includes it); a delta-scope run collapses to confirming the delta card.
disable-model-invocation: true
---

# Plan — Implementation Package

**Goal:** turn one capability-batch — a capability plus the work rows selected for this run —
carrying ratified scope on its capability's map entry into its accepted
implementation package — the artifacts its approved proposal names, scaled to the batch. One
run per capability-batch, ordered by the selected rows' dependency closure. `$ARGUMENTS` = the
capability ID (`FEAT-XXX`), the run covering its selected work rows;
empty → resolve the next capability with selected rows carrying ratified scope from the map and confirm
with the user.

## Goal

The package exists. **Product baselines live at `.mochiko/product/`** — `data-model.md` ·
`contracts/` · `nfrs.md` · `constraints-and-decisions.md` (C-XXX / D-XXX / IP-XXX) ·
`quickstart.md` when a real external-integration surface exists (its null path recorded in
`plan.md`) — beside repo-root `ARCHITECTURE.md`: they describe what the product HAS, are read
first as design input, and change only through the landing's graded fold — never edited in
place by this run. Absent at run open, **this run seeds them before design input is read**
(Baseline-seed binding).
**Per-capability artifacts land at `.mochiko/features/FEAT-XXX/`** — what the
feature CHANGES; **which land is the approved proposal's call**: `requirements.md` (FR→TR is
per-feature analysis) · the design deltas against
the baselines — `architecture.md`, `data-model.md`, `contracts/`, `nfrs.md`, each a delta
mirroring its baseline's filename; deltas against prose baselines are in appliable form —
exact before/after text — with `architecture.md`, **when the proposal includes it,** signed
off by the user, on a rendered diagram, before any detailed design was built on it · `tasks.md` as **cycle cards** — per
card: stories + feature rationale, foundation/feature type, dependencies, acceptance criteria
by ID, a `**TEST:**` real-infrastructure gate, cycle-level brownfield exposure; no task lists,
no file paths — the builder decomposes at build time · `plan.md`, a summary over the validated
artifacts, never new design. A delta-scope run collapses per the Entry — its deliverable is
the confirmed delta card, not this package. The feature's **map entry was confirmed and
hardened** alongside the design work — architecture link filled when produced, extent
sharpened,
intended-vs-designed drift surfaced to the user, the dependency relations the feature's
design implies asserted onto the entry with provenance (the technical seat asserts; the PM
consumes them downstream); status stays as the scope source set it. The
package was independently graded — feasibility and completeness — traces the business
requirements through to the task breakdown, carries no cross-artifact contradiction, conforms
to the signed-off architecture where the proposal produced one, and — where the spec carries a Screens & Flows manifest —
traces the feature's FEAT-tagged binding rows into the design: every SCR-XXX's data shown has
a serving contract surface, every FLOW-XXX action a mutation path, and every UX-bearing cycle
card's `**TEST:**` gate names the FLOW-XXX paths it verifies (pixels stay advisory, never
traced). The user accepted it whole. It is `/mochiko:implement`'s selection-scope entry
condition.

**Not done — default FAIL:** a proposed artifact missing (an unproposed artifact absent is
correct, not a failure), or an unrecorded `quickstart.md` null path ·
a product baseline absent at close or edited in place, or a delta against a prose baseline
not in appliable before/after form · a proposed architecture left unsigned, or a design
element contradicting the signed-off target · an earlier delivered feature's design broken without its `[MODIFY]` amendment · a
package never graded by anyone but its authors · user acceptance not given.

## Harness

- **You are the lead** — delivery manager of the goal; the artifact set is risk-scaled
  guidance, not a fixed checklist. Plan the run and orchestrate it toward the Goal; teammates
  or subagents per seat is your call. The architecture, when the proposal includes it, is
  designed and signed off **before** detailed design builds on it; a later contradiction with the signed-off target returns to
  the user for a consented amendment, never designed around silently. A design that breaks
  an earlier **delivered** feature is an explicit `[MODIFY]` amendment — named in `plan.md`,
  migration stated, surfaced at architecture sign-off — and it writes the marked delta on
  the affected feature's map entry.
- **Plan approval:** any seat that writes artifacts plans first and works only on a plan you
  approved; grading and fact-finding seats are exempt.
- **Independence:** no output is cleared by its author; grading reads the artifacts
  themselves — never the author's report — default FAIL.
- **Proposal contest (selection scope only):** an independent seat — not the proposing lead —
  reads the proposal cold and files a contest brief — the ladder applied adversarially — that
  lands beside the proposal for the user's ruling. Delta scope keeps its collapse — no contest.
- **Reserved to the user:** the plan-the-plan proposal (selection scope) — the lead's proposed
  artifact list + depth, rung-justified per `mochiko:patterns-plan-minimalism`, approved before
  authoring; the approved list becomes this run's default-FAIL floor · architecture sign-off
  when the proposal includes architecture, presented on a rendered diagram (no render surface →
  present source + component table and record it) · a governance conflict — conform,
  or amend/waive through `governance-ledger.md`; feature work never overrules the
  constitution · an infeasible grade — escalated as a business-level scope decision · a need
  conflicting with an in-flight feature's direction — amend the owning spec or lane run, or
  override · package acceptance (done / amend / reject).
- **Entry:** the run gates on a capability entry with selected work rows carrying ratified
  scope — the scope source is a
  spec's accepted selection, or a feature-command card: growth rows enter as selection scope,
  a bug/improvement delta as delta scope. Neither → block: new
  capability to `/mochiko:specify`, feature-keyed delta to `/mochiko:feature`. **Selection
  scope** (landing is the graduation batch): a capability-batch whose selected rows depend on
  rows not yet `delivered` blocks — batches run in the rows' dependency closure order. **Delta scope** (landing is
  the delta fold): the run collapses to confirming the delta card against the entry — no
  package authoring where no design surface changes. A missing governance region is surfaced
  (offer `/mochiko:setup`), never auto-resolved; on a brownfield codebase a missing or stale
  `.mochiko/memory/codebase-analysis.md` is surfaced the same way — offer setup, or proceed
  greenfield with the warning logged.
- Suggest commits; never run git mutations, never push. User acceptance is plain blocking
  text, never a timed prompt.

## Bindings

- **Artifacts** as listed in the Goal; `plan.md` from `templates/plan-template.md`;
  `tasks.md` from `templates/tasks-template.md` per `mochiko:patterns-vertical-tdd`;
  `architecture.md`'s structure and scope bound are `mochiko:patterns-system-design`'s; the
  structural D-XXX rows land as delta rows against `constraints-and-decisions.md`'s
  designated section.
- **Feature scope:** the capability entry governs scope and order — its ratified scope
  sourced from a spec's accepted selection (selection scope) or a feature-command card (growth
  rows as selection scope, a bug/improvement delta as delta scope); map machinery — entry
  shape, delta grammar, the in-flight fork —
  per `mochiko:authoring-feature-map`, never restated.
- **Repeat runs:** cards and reports append (dated); delta files overwrite only via the
  graded fold.
- **In-flight inputs:** a touched feature in flight (or delta-carrying) is an obligated read
  before design touches it — its design half at that feature's `.mochiko/features/FEAT-XXX/`
  directory (its deltas), its stories with the owning spec: need covered by the planned
  extent → reference it, build against the planned contract · adjacent → a `proposed` delta
  sequenced behind that delivery · conflicting → reserved to the user. No locks; only silent
  contradiction is prohibited.
- **UX input** (the spec's Screens & Flows section holds a manifest): an obligated design
  read — its binding rows (screens, data shown, actions) are requirements input to contracts
  and data-model; the `prototype/` app is reference, its pixels advisory. The run consumes
  its feature's FEAT-tagged SCR/FLOW rows.
- **Baseline:** repo-root `ARCHITECTURE.md` is the current-state seed; absent → the
  reconstructed baseline is confirmed with the user before a delta is designed on it, and
  lands as the initial `ARCHITECTURE.md` where the KM copy
  (`.mochiko/memory/knowledge-management.md`) exists.
- **Baseline-seed:** a baseline file absent at run open is seeded before design reads it —
  no delivered code: empty scaffolds stating so; delivered code exists: reconstructed from
  it and **confirmed with the user** like the `ARCHITECTURE.md` bootstrap above. The seed
  is the baseline write; the feature's design still lands as deltas at acceptance — never
  merged into the seed.
- **In-flight pointer:** at architecture sign-off, add the feature's one-line pointer to
  repo `ARCHITECTURE.md`'s In-flight list per `mochiko:authoring-architecture`. An
  omit-architecture proposal → no sign-off, no pointer, no close-diff owed.
- **Register:** user-facing prose per `templates/output-style.md`.
- **Next step:** `/mochiko:implement`.
