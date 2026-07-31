---
description: Turn an accepted spec into an accepted implementation package — analysis, an architecture design with its own early sign-off, detailed design, and the task breakdown — via an independent producer→reviewer team loop. A standing technical-analyst seat authors analysis then detailed design; a standing system-architect seat authors the architecture delta artifact first among the design work and stops the run at a rendered-diagram sign-off; a standing task-architect seat structures the mapping then tasks; a cold principal-architect seat grades analysis feasibility then the architecture (topology + governance); a cold devils-advocate seat grades completeness then the task artifacts, peer-edged with the active producer; the user signs off the architecture early and accepts the whole package at a named final gate. Governance-gated, architecture-first, default-FAIL, bounded, kernel-free. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Plan — Implementation Package (Analysis → Architecture → Design → Structuring)

**Goal:** turn an accepted `spec.md` into one accepted implementation package — analysis,
architecture (signed off early), detailed design, and the task breakdown — independently graded
for feasibility, completeness, and task-artifact quality. `$ARGUMENTS` = optional feature ID;
empty or detected-from-workspace is resolved at G1.

**You are the lead** of a team-form command in the mochiko command shape: Read
`${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md` (both layers) and `mochiko:loop-discipline`
before anything else; brief every dispatch per `templates/agent-dispatch.md`. This file carries
only plan's parameters. **First-spawn probe:** the `technical-analyst` producer — analysis
produces before anything reviews.

## Goal

Every artifact in Bindings exists; `principal-architect` returned `feasible` on the analysis
**and** on the architecture pass; `devils-advocate` returned `ready` on the analysis, the
architecture coverage, the detailed design, the mapping **and** the tasks, each grounded in the
files; the architecture sign-off (G3) cleared with no unresolved contradiction; you Read the
artifacts and reports and found no blocking gap; the KM landing ran; and the user accepted the
whole package at G7. The package is `/mochiko:implement`'s unchanged entry condition.

**Not done:** a missing artifact, or an unrecorded `quickstart.md` null path · any reviewer
status short of `feasible`/`ready` · an unsigned or re-opened architecture target · a reviewer
status taken as the gate without your read · out of rounds · G7 unaccepted.

## Seats & checks

| seat | agent × skill(s) | produces / grades | spawn | peer edges |
|---|---|---|---|---|
| producer | `technical-analyst` × `authoring-technical-requirements`, `patterns-technical-decisions`, `patterns-entity-modeling`, `patterns-api-contracts` | authors the analysis set, then the detailed design conforming to the approved `architecture.md`; never grades | standing across analysis + detailed design; **probe seat** | hands finished artifacts to completeness directly; architect concerns arrive via you (G4) |
| system-architect | `system-architect` × `patterns-system-design` | authors `architecture.md` + the structural D-XXX rows; never grades | standing, architecture stage | hands `architecture.md` to completeness; feasibility is lead-fired |
| task-architect | `task-architect` × `patterns-vertical-tdd` | authors `task-mapping.md`, then expands it into `tasks.md`; never grades | standing across mapping + tasks | hands each round's artifact to completeness |
| feasibility | `principal-architect` × `review-feasibility` | grades analysis feasibility, then the architecture pass (topology + governance); never authors, and never grades the detailed design | cold once the analysis is authored; **lead-gated** thereafter | none — its concerns reach producers only through you (G4) |
| completeness | `devils-advocate` × `review-plan-artifacts`, then `review-task-artifacts` | grades coverage / measurability / consistency at every stage, architecture coverage, and conformance to the approved architecture; never authors | cold at first review, standing after | peer-edged with the active producer; grades only when you open the pass |
| architecture scribe | `principal-architect` × `authoring-architecture` | records a bootstrap-confirmed baseline into `ARCHITECTURE.md` | disposable, at finalize, per the KM landing | none — never the feasibility seat |

**Validation model:** the loop's bounded in-loop critique, every round. Each reviewer's output is
**lead-adjudicated input** (the `review-*` family boundary) and every verdict is yours. You select
the completeness reviewer's skill and mode per stage and supply the artifact sets it grades across (incremental for detailed design: {new design} / {prior analysis};
cumulative for tasks: {`tasks.md`} / {`task-mapping.md`}) — a policy call, not a hand-off.

## Constraints

- **G1 entry** — evidence: `$ARGUMENTS`, `spec.md` present and accepted, the governance region
  (`<!-- mochiko:governance:begin -->`), the declared project type in `governance-intent.md`,
  repo-root `ARCHITECTURE.md`, `slices.md` · rules: the user · decides: the resolved `<feature>`
  (an explicit ID, else the most recent in-progress feature under `.mochiko/specs/`, confirmed with
  the user before the run opens). An absent or unaccepted spec blocks to `/mochiko:specify`; a
  missing governance region is
  surfaced (offer `/mochiko:setup`), never auto-resolved; brownfield requires
  `codebase-analysis.md` (missing → offer setup or proceed greenfield with a logged warning; >14d
  stale by mtime → warn); an absent `ARCHITECTURE.md` sets the bootstrap flag for G2.
- **G2 baseline** *(bootstrap only)* — evidence: the architect's reconstructed current-state
  topology, marked reconstructed with its confidence noted · rules: the user · decides: the
  delta's current-state seed, **before any delta is designed on it**. A present `ARCHITECTURE.md`
  skips this gate — its content is the seed. Greenfield with no prior structure degenerates
  cleanly: the baseline is empty, the target is the whole picture.
- **G3 architecture sign-off** *(always-on)* — evidence: `architecture.md` + both reports clear,
  and the **rendered** diagram presented by you via the session's render surface (side-panel
  render, published artifact, IDE preview), never a raw mermaid block · rules: the user ·
  decides: the approved target — detailed design opens only once it clears. A no-delta feature
  still presents the neighborhood-scoped diagram plus its one-line "changes nothing structurally"
  claim, so the judgment is shown, never silently made by the producer. With none of those render
  surfaces in an attended session the gate **degrades with record**: present the diagram source +
  component table and record "presented un-rendered" on the artifact. Plan is never hard-blocked
  by rendering.
- **G4 feasibility / governance rejection** — evidence: architect concerns, or an architecture
  that must break a governance surface · rules: the user · decides: redesign to conform, or an
  amendment/waiver through `governance-ledger.md` — exactly two exits; the feature gate never
  overrules the constitution. An `infeasible` grade escalates as a business-level scope decision,
  not a routine revision.
- **G5 clarification** — evidence: an advocate gap, or a producer question it cannot resolve ·
  rules: the user · decides: the answer fed forward. **A preference gap is ruled here**; a
  knowledge gap routes to a native `Explore` pass (the "Research this" branch), never to the user;
  a scope gap is G6's.
- **G6 exit-early / escalation** — evidence: a cap trip, a gap set unchanged round-over-round,
  `PLAN_STOP`, or a producer designing beyond slice scope · rules: the user · decides:
  continue-refining / accept-with-noted-gaps / abort — the run stays FAIL unless the user
  explicitly accepts.
- **G7 package acceptance** — evidence: every stage's clearing verdict, `plan.md` assembled, and
  the decision / entity / endpoint / cycle counts with any noted limitations · rules: the user ·
  decides: done / amend (re-enter the relevant bounded stage; an architecture amend re-clears G3)
  / reject (drafts remain in place). This is the package's **one** standing acceptance.
- **Bounds:** cap **3** produce↔review rounds **per stage** (analysis · architecture · detailed
  design · mapping · tasks), you count each; no-progress exit on a gap set unchanged
  round-over-round; kill-switch `PLAN_STOP` checked before each seat send; out of rounds =
  escalate, never done.
- **Ordering invariants:** the architecture is the **first** artifact of the design work — nothing
  downstream is authored against an unapproved shape. Feasibility grades **once** per input and
  re-fires only on a structural change (new or changed constraints, expanded requirement scope,
  modified NFR targets); a clarification-only revision returns to the completeness pass alone. The
  mapping is graded **before** the expensive TDD breakdown. A detailed-design contradiction with
  the approved architecture **stops the producer and returns to G3** for a consented target
  amendment — never designed around silently. Delivery is a hand-off, not a start signal: you open
  every round and every review pass. **No devolved branch** — every review here is a judgment
  grade, so no gate is skipped and no unit clears unread.
- **Slice scope** *(when an accepted `slices.md` exists)* — that file's **Graduation contract** is
  the single home for slice resolution, the staleness guard, scope, extend-mode, graded amendment,
  and artifact layout; not restated. plan's own bindings on top: a producer designing beyond scope
  is a scope gap → G6; a `[MODIFY]` graded amendment is surfaced for this round's reviews with its
  migration flagged for the slice's task breakdown; each reviewer is briefed with {this slice's
  extensions + artifacts} / {the prior accumulated artifacts}, so the extension is graded against
  what earlier slices established; the architecture delta seeds from the accumulated feature-root
  `architecture.md`/`ARCHITECTURE.md`, never per-slice from scratch.
- **Optional design checkpoint** — on request in a judgment-heavy run, a look at the design before
  structuring is spent on it; a courtesy, never a standing gate.

## Bindings

- **Artifacts**, under `.mochiko/specs/<feature>/` (slice-scoped: `plan.md`, `architecture.md`,
  the task artifacts and round reports land under `slices/<slice>/`, so the Goal's artifact set
  reads the shared feature-root artifacts extended there plus the per-slice ones):
  `requirements.md` (FR→TR) · `constraints-and-decisions.md` (C-XXX / D-XXX / IP-XXX, with a
  designated **structural-decisions section** the architect owns) · `nfrs.md` (NFR-XXX) ·
  `architecture.md` (its structure and scope bound are `patterns-system-design`'s) ·
  `data-model.md` (entities + sensitivity) · `contracts/api.yaml` (OpenAPI + `x-integration`) ·
  `quickstart.md` **conditional** on a real external-integration surface, its null path recorded
  in `plan.md` · `task-mapping.md` (the slicing source of truth) · `tasks.md` (`[US#]` tags,
  `[EXTEND]`/`[MODIFY]` markers; its Story→Cycle table a **derived echo** of the mapping) ·
  `plan.md`, your fill-target from `templates/plan-template.md` — a summary over validated
  artifacts, never new design.
- **Reports:** `techanalyst-report.md` · `sysarchitect-report.md` · `taskarchitect-report.md` ·
  `feasibility-report.md` · `advocate-report.md`. Cleaned by default at finalize (outcome stamps
  live in `plan.md`); never offer to delete a deliverable.
- **Uncertainty carrier:** producer-authored — each report's Assumptions / Open Questions, not
  confidence marks.
- **Fact route:** the artifacts themselves; a knowledge gap goes to a native `Explore` pass.
- **KM landing:** `.mochiko/memory/knowledge-management.md` exists → run its ritual + invariants
  under fix-on-sight, and mint new domain terms into `GLOSSARY.md`. A **bootstrap-confirmed
  baseline** (G2) lands as the initial `ARCHITECTURE.md` via the scribe seat. Plan records only
  what plan itself established — the confirmed baseline; implement records what it builds. No copy
  → skip.

## Recovery

Note the resume stage on the deliverable; resume from workspace evidence, respawning what the
stage needs — a respawned producer re-reads the artifacts + gap list.

| Evidence in the workspace | Resume at |
|---|---|
| no `spec.md`, or unaccepted | entry blocked (G1) |
| `slices.md` present | resolve the current slice; the rows below then read per-slice artifacts alongside the shared feature root |
| `spec.md` present, no `requirements.md` | analysis (produce) |
| analysis present, no report this round | analysis (review) |
| analysis not `feasible`+`ready`, within the cap | analysis (loop control) |
| analysis cleared, no `architecture.md`, or baseline unconfirmed | architecture (baseline / produce) |
| `architecture.md` present, unsigned | architecture (review / G3) |
| architecture signed off, no `data-model.md` | detailed design (produce) |
| design present, advocate not `ready`, or an open architecture contradiction | detailed design (loop control / return to G3) |
| design cleared, no `task-mapping.md` | mapping (produce) |
| `task-mapping.md` present, not `ready`, within the cap | mapping (loop control) |
| mapping cleared, no `tasks.md` | tasks (produce) |
| `tasks.md` present, not `ready`, within the cap | tasks (loop control) |
| all stages cleared, no `plan.md` | assemble |
| `plan.md` present, unaccepted | G7 |
| accepted | finalize — report artifacts, per-stage round counts, the decision / entity / endpoint / cycle counts, a suggested commit (`docs: plan <feature>`), next step `/mochiko:implement` |
| `PLAN_STOP` present | escalate (G6) |
