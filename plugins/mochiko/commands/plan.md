---
description: Turn an accepted spec into an accepted implementation package — analysis, an architecture design with its own early sign-off, detailed design, and the task breakdown — via an independent producer→reviewer team loop. A standing technical-analyst seat authors analysis then detailed design; a standing system-architect seat authors the architecture delta artifact first among the design work and stops the run at a rendered-diagram sign-off; a standing task-architect seat structures the mapping then tasks; a cold principal-architect seat grades analysis feasibility then the architecture (topology + governance); a cold devils-advocate seat grades completeness then the task artifacts, peer-edged with the active producer; the user signs off the architecture early and accepts the whole package at a named final gate. Governance-gated, architecture-first, default-FAIL, bounded, kernel-free. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Plan — Implementation Package (Analysis → Architecture → Design → Structuring)

**Goal:** turn an accepted `spec.md` into an accepted implementation **package** — the analysis
artifacts (`requirements.md` · `constraints-and-decisions.md` · `nfrs.md`), the **architecture**
artifact (`architecture.md`, the delta system view signed off early), the detailed-design
artifacts (`data-model.md` · `contracts/api.yaml` · `quickstart.md` when the feature has an
external-integration surface), and the task breakdown (`task-mapping.md` · `tasks.md`) — authored
in stages and independently graded for **feasibility** (can these pieces be built *together*, and
can the topology carry the NFRs and honor governance?), **completeness** (is anything missing?),
and **task-artifact quality** (vertical-slice integrity, TDD test-first ordering) before the user
accepts one package. The package is `/mochiko:implement`'s unchanged entry condition. `$ARGUMENTS`
= optional feature ID or description; empty or detected-from-workspace is handled by triage below.

**You are the lead**, and this is a **team-form command in the mochiko command shape**: Read
`${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md` (both layers) before anything else — the
shape's rules bind here and are not restated; this file carries only plan's parameters. You own
the loop (per-stage round counters, verdict, escalation) and every human gate. This is a
`mochiko:loop-discipline` sound loop; the Contract section below is its authoring-time fill.

## Team-form parameters (shape Layer 2)

Hard-require `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` per the shape. The **authoritative
first-spawn probe** is the producer (`technical-analyst`) — always the first seat filled (analysis
produces before anything reviews). Transport mechanics + the addressability check:
`templates/agent-dispatch.md` (Seat transport). The no-fallback bet is the same `Contested`
dogfood-pilot ruling as the other team-form commands.

## Session constraints

- Workspace: resolve `<feature>` (an explicit ID from `$ARGUMENTS`, else the most recent
  in-progress feature under `.mochiko/specs/`); seed `.mochiko/specs/<feature>/`. The deliverables
  live alongside the spec they plan.
- Kill-switch: stop and escalate if `.mochiko/specs/<feature>/PLAN_STOP` exists — check before
  each seat send.
- **Deliverables & IDs:** the artifacts — `requirements.md` (FR→TR) · `constraints-and-decisions.md`
  (C-XXX/D-XXX/IP-XXX, with a designated **structural-decisions section** for the architecture's
  D-XXX rows) · `nfrs.md` (NFR-XXX) · `architecture.md` (the delta system view — its structure is
  owned by `mochiko:patterns-system-design`, not restated here) · `data-model.md` (entities +
  sensitivity) · `contracts/api.yaml` (OpenAPI + `x-integration`) · `quickstart.md` **conditional**
  (authored only when a real external-integration surface exists, per `patterns-api-contracts`; its
  null path recorded in `plan.md`) · `task-mapping.md` (the story→cycle mapping, the slicing source
  of truth) · `tasks.md` (cycle→TDD tasks; `[US#]` tags; `[EXTEND]`/`[MODIFY]` markers; the
  Story→Cycle table is a **derived echo** of `task-mapping.md`) — plus the assembled `plan.md`,
  authored per the producers' skills and `templates/plan-template.md` — no placeholder tokens. The
  producers' self-disclosures are `techanalyst-report.md`, `sysarchitect-report.md`, and
  `taskarchitect-report.md` (from their `templates/*-report-template.md`) — their Assumptions /
  Open Questions are the producer-authored uncertainty carrier (the shape's producer-authored
  branch), not confidence marks. Reviewer output: feasibility findings in `feasibility-report.md`,
  completeness gaps and task-artifact gaps in `advocate-report.md`.

## The seats

Three producers, two reviewers. Each producer authors its own artifacts and never grades; each
reviewer grades from the files and never authors. Independence is structural (shape Layer 2):
disjoint agents, disjoint skills, reviewers arriving **cold at their stage**. In-loop the mesh is
the default — a producer hands its finished artifacts to the completeness reviewer **directly**
(peer-routed), but **delivery is not a start signal**: the reviewer grades, and the producer
revises, only when you open the round. The **feasibility architect is lead-gated** — you fire it
selectively and its concerns reach the producer through you. Every verdict is yours.

- **producer** — `mochiko:technical-analyst` (`authoring-technical-requirements`,
  `patterns-technical-decisions`, `patterns-entity-modeling`, `patterns-api-contracts`), one
  **named standing seat across analysis and detailed design**. Analysis stage: author
  `requirements.md`, `constraints-and-decisions.md` (its C-XXX/IP-XXX and Phase-1 D-XXX rows — **not**
  the structural-decisions section, which is the architect's), `nfrs.md`. Detailed-design stage
  (after the architecture sign-off): author `data-model.md`, `contracts/api.yaml`, `quickstart.md`
  when applicable — **these must conform to the approved `architecture.md`**; a contradiction the
  authoring reveals is a design-time deviation (Phase 3). The standing seat carries the analysis
  rationale forward. Brief it per `agent-dispatch`: `spec.md`, the approved `architecture.md` (in
  detailed design), the governance obligated-read line, the brownfield analysis when present, the
  templates to fill. **Peer-edged with the completeness reviewer:** it hands each stage's finished
  artifacts straight there; the feasibility architect is lead-gated, its concerns reaching the
  producer through you (G4). Round > 1 within a stage reaches the same seat with the completeness
  gap list already in hand (peer-routed) — you rule the round and hold the revision targeted (fix
  the flagged gaps; don't regress passing sections). It never grades.
- **system-architect** — `mochiko:patterns-system-design`, one **named standing seat for the
  architecture stage** (a new seat: topology judgment is distinct from requirements decomposition).
  Authors `architecture.md` — the delta system view (current + proposed target, the change
  highlighted), scoped to the delta neighborhood past the artifact's size threshold (the full-system
  view is linked, never inlined — the same scope bound governs the no-delta presentation) — and
  writes the **structural D-XXX rows** into the designated structural-decisions section of
  `constraints-and-decisions.md` (+ `sysarchitect-report.md`). It consumes `spec.md`, the analysis
  artifacts, the seeded current-state baseline (Phase 2 step 1), and the governance region + relevant
  rules files (layer-rules / domain-dependency registry when attached) — the artifact cites the
  principles that bound the target. Brief it per `agent-dispatch`. **Peer-edged with the completeness
  reviewer** for the coverage grade — it hands `architecture.md` straight there; the feasibility
  **architecture pass is lead-gated** (you fire it). Round > 1 reaches the same seat with the
  completeness gap list in hand (peer-routed); architect / governance concerns arrive through you
  (G4). It never grades; the artifact structure and the D-XXX schema are its skills' homes, not
  restated here. `technical-analyst` consumes the approved architecture downstream.
- **task-architect** — `mochiko:patterns-vertical-tdd`, one **named standing seat across mapping and
  tasks**. Mapping: author `task-mapping.md` (story→cycle mapping + vertical-slice rationale, the
  source of truth). Tasks: expand it into `tasks.md` (+ `taskarchitect-report.md` each round) — the
  standing seat carries its Phase-4 slicing judgment from the mapping into the expansion. Brief it per
  `agent-dispatch`: the accepted design outputs (incl. `architecture.md`), the governance
  obligated-read line, the brownfield context, the templates. **Peer-edged with the completeness
  reviewer** (running `review-task-artifacts`): it hands each round's finished artifact straight
  there. Round > 1 reaches the same seat with the reviewer's gap list already in hand (peer-routed) —
  you rule the round and hold the revision targeted. It never grades.
- **feasibility reviewer** — `mochiko:principal-architect` (`review-feasibility`), spawned **cold
  after the Phase-1 analysis is authored**, **lead-gated thereafter** — you fire it, and its concerns
  reach the producer through you (G4). It grades in two moments, each **once** and re-fired (a
  message to the same seat) **only on a structural change** to its inputs: (1) **analysis feasibility**
  on the Phase-1 artifacts — cross-artifact contradiction / impossibility / buildability, no topology
  needed → `feasibility-report.md`; (2) the **architecture pass** on `architecture.md` in Phase 2 —
  **topology feasibility** (can the proposed topology carry the NFRs and constraints it is graded
  against) **and governance conformance** (layers honored, dependencies within allowlist, NFR-linked
  principles satisfiable by the topology) → `feasibility-report.md`. This is the carve-out of its
  former "never grades past Phase 1" bar: it now grades the architecture too, but **not** the
  detailed-design artifacts (the completeness reviewer carries their cross-artifact consistency and
  their conformance to the approved architecture). Its output is **lead-adjudicated input** (the
  `review-*` family boundary).
- **completeness reviewer** — `mochiko:devils-advocate`, spawned **cold at the first review**,
  **peer-edged with the active producer thereafter**, one **named standing seat across every stage** —
  it runs the skill the stage calls for: `review-plan-artifacts` across analysis, architecture, and
  detailed design, then `review-task-artifacts` across mapping and tasks (the skill is named per
  dispatch, never loaded as frontmatter). **Delivery is not a start signal** — it grades only when you
  open the pass (in the design stages: after the architect, per your sequencing; in structuring: on
  your mode-selecting message). Analysis: completeness / coverage / consistency → `advocate-report.md`.
  Architecture: the extended `review-plan-artifacts` **architecture coverage** checklist (its checks
  are that skill's home — referenced, not restated). Detailed design: incremental mode — a full review
  of the new design artifacts, a consistency check back to the analysis, **and the
  conforms-to-approved-architecture check** over `data-model.md`/`contracts`. Mapping: the
  `review-task-artifacts` Mapping checklist. Tasks: cumulative mode — a full `tasks.md` review plus the
  cross-check back to `task-mapping.md`. Its retained context is what makes each later stage's check
  incremental rather than a cold re-read. Round > 1 within a stage: re-Read the revised files. Its
  output is **lead-adjudicated input**; there is no sized end-stage review — the bounded in-loop
  critique is this workflow's independent validation (declared in the Contract). Single completeness
  reviewer, never a producer.
- **architecture scribe** — `mochiko:principal-architect` (`mochiko:authoring-architecture`), a
  **disposable Finalize dispatch**, fired only per the KM landing — never the feasibility seat, and
  distinct from the architecture stage above (that stage *proposes* a delta; this scribe *records* the
  built or bootstrapped reality into `ARCHITECTURE.md`).

## Phase 0 — Prerequisites & entry triage  *(human gate G1)*

1. **Capture** `$ARGUMENTS`; resolve `<feature>` (an explicit ID, else the most recent in-progress
   feature under `.mochiko/specs/`). If empty (the `@`-reference drop bug), recover via **G1**: ask
   the user to re-enter, or to confirm the detected feature.
2. **Governance prerequisite.** Check `CLAUDE.md` for the mochiko governance region
   (`<!-- mochiko:governance:begin -->`). Present → governance reaches every producer natively at
   spawn; add to each brief the **one-line obligated read** naming the `.claude/rules/mochiko/` files
   and skills relevant to what it authors (`paths`-scoped rules don't fire for from-scratch authoring).
   Missing → do **not** silently proceed; surface it (offer `/mochiko:setup` first). Never auto-resolve.
3. **Entry gate.** The spec must be done: `.mochiko/specs/<feature>/spec.md` present and accepted
   (workspace evidence). Missing → block and point the user to `/mochiko:specify`.
4. **Brownfield check.** Read the declared project type from `.mochiko/memory/governance-intent.md`.
   Brownfield → require `.mochiko/memory/codebase-analysis.md` (missing → offer `/mochiko:setup` or
   proceed greenfield with a logged warning; >14d stale by file mtime → warn); greenfield → bypass.
   Carry the analysis into the producers' briefs when present.
5. **Seed the architecture baseline.** Read repo-root `ARCHITECTURE.md`. **Present** → it seeds the
   current-state half of the Phase-2 delta (no baseline gate). **Absent** → set the bootstrap flag:
   Phase 2 opens with a reconstructed-baseline confirmation (**G2**) before any delta is designed.
6. **Slice-scoped entry (graduation slices).** If `.mochiko/specs/<feature>/slices.md` exists
   (accepted), the run is **slice-scoped** — apply that file's own **Graduation contract** section for
   slice resolution, the staleness guard, scope (= the slice's stories + its extend obligations),
   extend-mode, graded amendment, and artifact layout; do not restate those rules here (the Graduation
   contract is their single home). **plan's own bindings on top:** a producer designing beyond scope is
   a scope gap → **G6**; a `[MODIFY]` graded amendment is surfaced for this round's reviews with its
   migration flagged for this slice's task breakdown; per-slice `plan.md`, `architecture.md`, the task
   artifacts, and round reports land under `slices/<slice>/`, so the done-condition's artifact set reads
   the shared artifacts (extended at the feature root) + `slices/<slice>/`; brief each reviewer with the
   artifact sets {this slice's extensions + its artifacts} / {the prior accumulated artifacts}, so the
   extension is graded against what earlier slices established. The architecture delta seeds from the
   accumulated feature-root `architecture.md`/`ARCHITECTURE.md`, not per-slice from scratch.

## Phase 1 — Analysis loop  *(you own the round counter and the verdict)*

`round = 1`; the analysis is FAIL until proven. The architect grades feasibility **once**, after the
analysis is authored — before the architecture stage spends its work on infeasible requirements.

1. **Produce.** The producer authors `requirements.md` (FR→TR mapping), `constraints-and-decisions.md`
   (its C-XXX / non-structural D-XXX / Part-3 IP-XXX — leaving the structural-decisions section for the
   architect), and `nfrs.md` (+ `techanalyst-report.md`), handing the completed set to the completeness
   reviewer directly (peer-routable delivery); **you sequence when it grades** — the architect first
   (step 2), so an infeasible analysis never buys a completeness pass. On round > 1 the producer already
   holds the completeness gap list for targeted revision (fix flagged gaps; don't regress passing
   sections). The round-1 spawn is the authoritative probe — confirm addressability.
2. **Feasibility (architect, once — lead-gated).** You fire the feasibility reviewer, cold; it grades
   analysis feasibility from the files → `feasibility-report.md`.
3. **Completeness (advocate).** The completeness reviewer, cold at first review, grades completeness /
   coverage / consistency from the files → `advocate-report.md`.
4. **Verdict (you).** Read the artifacts + both reports. `feasible` **and** `ready` **and** no blocking
   gap → Phase 2. Otherwise classify each finding and route per `loop-discipline`'s gap-routing
   (knowledge → native `Explore`, the "Research this" branch in G5; preference → G5; scope → G6):
   architect concerns → **G4**; advocate gaps → **G5**; architect `infeasible` → escalate as a
   business-level scope decision, not a routine revision. Then apply the bounds (increment `round`;
   cap / no-progress / kill-switch → **G6** / escalate) and loop to step 1. **Re-run the architect
   (step 2) only on a structural change** — new/changed constraints, expanded requirement scope, or
   modified NFR targets; a clarification-only revision goes straight back to step 3.

## Phase 2 — Architecture design & sign-off  *(the early gate; you own the round counter)*

The architecture is the **first artifact of the design work**, authored before detailed design and
signed off at its own early gate — so nothing downstream is authored against an unapproved shape (the
cascade the sign-off exists to prevent). `round = 1`; the architecture is FAIL until signed off.

1. **Baseline (bootstrap only, conditional — human gate G2).** If Phase 0 step 5 set the bootstrap flag
   (no `ARCHITECTURE.md`): the architect reconstructs the current-state baseline topology from the code
   (and `codebase-analysis.md` when present), marks it **reconstructed** with its confidence noted, and
   presents it. **G2**: the user confirms the baseline **before any delta is designed on it**. The
   confirmed baseline is the delta's current-state seed and lands as the initial `ARCHITECTURE.md` at
   Finalize. When `ARCHITECTURE.md` is present, skip this step — its content is the seed. (Greenfield
   with no prior structure degenerates cleanly: the baseline is empty; the target is the whole picture.)
2. **Produce architecture.** The system-architect authors `architecture.md` — the delta view (current +
   proposed target, the structural change highlighted), scoped per the artifact's size bound — and
   writes the structural **D-XXX rows** into the structural-decisions section of
   `constraints-and-decisions.md` (+ `sysarchitect-report.md`), handing `architecture.md` to the
   completeness reviewer directly (peer-routable delivery); on round > 1 it already holds the reviewers'
   gap list. Genuine-alternative topology choices get D-XXX rows here (existing ADR discipline); the
   delta summary links each structural change to its D-XXX row, never restating it.
3. **Architecture review.** You fire the feasibility reviewer for its **architecture pass** — topology
   feasibility (against the NFRs/constraints) + governance conformance → `feasibility-report.md`
   (lead-gated). The completeness reviewer, on your open, runs the extended `review-plan-artifacts`
   **architecture coverage** checks → `advocate-report.md`. A proposed architecture that must break a
   governance surface surfaces the conflict here with exactly two exits (redesign to conform, or a
   user-ruled amendment/waiver through the existing `governance-ledger.md` machinery — **G4**); the
   feature gate never overrules the constitution.
4. **Verdict + sign-off (human gate G3).** Read `architecture.md` + both reports. On a clear grade,
   present the **rendered** diagram to the user — via the session's render surface (side-panel file
   render, published artifact, IDE preview), never a raw mermaid block. You (the plan supervisor) are
   the presenter. <!-- shape-exception: D8/R5 — when an attended session has none of those render
   surfaces, the gate degrades with record: present the diagram source + component table and record
   "presented un-rendered" on the artifact (a recorded absence, mirroring waiver discipline). Plan is
   never hard-blocked by rendering. --> The stage is **always-on**: even a no-delta feature presents the
   unchanged (neighborhood-scoped) container diagram plus a one-line "this feature changes nothing
   structurally" claim for approval — the no-delta judgment is shown, never silently made by the
   producer. **G3**: the user approves the **target**. Only after G3 clears does Phase 3 proceed.
   Otherwise route findings (architect/governance → **G4**; advocate → **G5**), apply the bounds
   (increment `round`; cap / no-progress / kill-switch → **G6** / escalate), and loop to step 2.

## Phase 3 — Detailed design loop  *(incremental review; you own the round counter)*

`round = 1`; the detailed design is FAIL until proven. It is authored to **conform to the approved
architecture** — no architect re-review here (the architecture pass already cleared); the advocate
carries cross-artifact consistency **and** architecture conformance in incremental mode.

1. **Produce.** The producer authors `data-model.md` (entities + sensitivity), `contracts/api.yaml`
   (OpenAPI + `x-integration`), and — when the feature has an external-integration surface —
   `quickstart.md` (+ `techanalyst-report.md`), carrying its analysis context forward, conforming to
   the approved `architecture.md`, and handing the design set to the completeness reviewer directly
   (peer-routable delivery).
2. **Design-time deviation (return to sign-off).** If the detailed-design authoring reveals a
   **contradiction with the approved architecture** — the design cannot conform without changing the
   topology — the producer **stops and surfaces it**; you return to the **G3** sign-off for a consented
   target amendment (the same mechanism as the mid-implement and mid-cycle deviation rule), then resume.
   A contradiction is never silently designed around.
3. **Incremental review (advocate).** On your open, the completeness reviewer grades in **incremental
   mode** — a full review of the new design artifacts, a consistency check back to the analysis, and the
   conforms-to-approved-architecture check (you select the mode and supply the {new design}/{prior
   analysis} artifact sets) → `advocate-report.md`.
4. **Verdict (you).** Read the design artifacts + report. `ready`, no blocking gap, and no unresolved
   architecture contradiction → Phase 4. Otherwise route gaps per `loop-discipline` (→ **G5**), apply
   the bounds (cap / no-progress / kill-switch → **G6** / escalate), and loop to step 1.

**Optional design checkpoint (on request).** Before entering Phase 4, in a judgment-heavy run, you may
offer the user a design checkpoint — a look at the design before structuring is spent on it. This is a
sizing-gated courtesy **on request**, never a standing gate; the package's one standing acceptance is G7.

## Phase 4 — Structuring loop (Mapping → Tasks)  *(you own the round counter)*

The design is absorbed into the task breakdown here — the former `/mochiko:tasks` loop, run in the same
room. Two sub-stages, each `round = 1` and FAIL until proven; the reviewer grades the mapping's slicing
quality **before** the expensive full TDD breakdown.

1. **Mapping — produce.** The task-architect authors `task-mapping.md` (+ `taskarchitect-report.md`),
   handing it to the completeness reviewer directly when the round's artifact is complete; on round > 1
   it already holds the reviewer's gap list for targeted revision.
2. **Mapping — review + verdict.** On your open, the completeness reviewer runs the
   `review-task-artifacts` Mapping checklist → `advocate-report.md`. `ready` + no blocking gap →
   sub-stage Tasks. Otherwise route gaps (→ **G5**), apply the bounds (cap / no-progress / kill-switch →
   **G6**), loop to step 1.
3. **Tasks — produce.** The task-architect expands `task-mapping.md` into `tasks.md` (+
   `taskarchitect-report.md`), briefed with the mapping as the input to expand, handing the result across
   peer-routed.
4. **Tasks — review + verdict.** On your open, the completeness reviewer grades in **cumulative mode** —
   a full `tasks.md` review plus the cross-check back to `task-mapping.md` (you supply both artifact
   sets) → `advocate-report.md`. `ready` + no blocking gap → Phase 5. Otherwise route gaps (→ **G5**),
   apply the bounds, loop to step 3.

## Phase 5 — Assemble & accept the package  *(human gate G7)*

1. **Assemble `plan.md`** from `templates/plan-template.md`: the key decisions from
   `constraints-and-decisions.md` (structural rows included), the **Architecture** pointers to
   `architecture.md` (the diagram + component table, not restated), the entity summary from
   `data-model.md`, the endpoint summary from `contracts/api.yaml`. `plan.md` is the lead's fill-target
   — a summary over the validated artifacts, not new design.
2. **Final package acceptance (G7).** Reachable only after every stage's clearing verdict. Present the
   whole package — the design (`plan.md` + the signed-off `architecture.md` + the analysis/design
   artifacts) **plus** the mapping and tasks — as **one package**, with the decision / entity / endpoint /
   cycle counts and any noted limitations. The user **accepts** (→ Phase 6; the done-condition is now
   satisfied — this package is `/mochiko:implement`'s unchanged entry), **amends** (re-enter the relevant
   stage with the changes as the gap list — still bounded; it must clear its verdict again, and an
   architecture amendment re-clears **G3**), or **rejects** (abort; the drafts remain under
   `.mochiko/specs/<feature>/`). The standalone design-acceptance signature is dissolved into this one
   package gate; the architecture **sign-off (G3)** is the one design-time gate that survives, by design.

## Phase 6 — Finalize

Report the artifacts (the deliverables + `plan.md` + the round reports `techanalyst-report.md` /
`sysarchitect-report.md` / `taskarchitect-report.md` / `feasibility-report.md` / `advocate-report.md`),
the per-stage round counts, the decision / entity / endpoint / cycle counts, a suggested commit (`docs:
plan <feature>`), and the next step (`/mochiko:implement`). Intermediate round reports are cleaned by
default (their outcome stamps live in `plan.md`); the user may ask to retain them. Never offer to delete
`plan.md`, `architecture.md`, or the analysis/design/task artifacts — they are the deliverables.
**KM landing** — `.mochiko/memory/knowledge-management.md` exists → run its landing ritual + invariants
under fix-on-sight; a **bootstrap-confirmed baseline** (Phase 2 step 1) lands as the initial
`ARCHITECTURE.md`, and any built-structural-change folds into `ARCHITECTURE.md` via a fresh
`principal-architect` dispatch (`mochiko:authoring-architecture`), never the feasibility seat. (Plan
proposes; implement builds; landing records — plan's landing records only what plan itself established,
the confirmed baseline.) No copy → skip.

## Contract (authoring-time fill — governed by `mochiko:loop-discipline`)

- **Done-condition:** default **FAIL**; clears only when **(1)** the artifacts exist (`requirements.md` ·
  `constraints-and-decisions.md` incl. the structural-decisions section · `nfrs.md` · `architecture.md` ·
  `data-model.md` · `contracts/api.yaml` · `quickstart.md` when applicable, else its null path recorded ·
  `task-mapping.md` · `tasks.md`), **(2)** `principal-architect` returns `feasible` on the analysis **and**
  on the architecture pass, `devils-advocate` returns `ready` on the analysis, the architecture coverage,
  the detailed design, the mapping, **and** the tasks — each grounded in the files, **(3)** the architecture
  **sign-off (G3)** has cleared and no unresolved architecture contradiction remains, **(4)** *you* Read the
  artifacts + reviewer reports and confirm no blocking gap remains (each reviewer's status is input, never
  the gate), **and (5)** the Phase-5 final package acceptance (G7) has cleared. Out of rounds = escalate,
  never done.
- **Producer ↔ validator:** three producers, none grading — `technical-analyst` (authoring-technical-requirements,
  patterns-technical-decisions, patterns-entity-modeling, patterns-api-contracts) authors analysis + detailed
  design; `system-architect` (patterns-system-design) authors the architecture; `task-architect`
  (patterns-vertical-tdd) authors the task breakdown. **Two independent reviewers**, neither a producer —
  `principal-architect` (review-feasibility) grades analysis feasibility + the architecture pass;
  `devils-advocate` (review-plan-artifacts, then review-task-artifacts) grades completeness, architecture
  coverage, detailed-design conformance, and the task artifacts — all from the files, never authoring.
  Disjoint agents, disjoint skills, structurally separated (reviewers cold-spawned at their stage; the
  completeness gap list hands off peer-routed producer↔reviewer per the shape's mesh, the feasibility
  architect lead-gated with its concerns routed through you at G4, every verdict yours). **Validation
  model:** the bounded in-loop critique — every round, unsized by design; no sized end-stage review (the
  shape's in-loop-critique branch).
- **Bounds:** cap **3** produce↔review rounds **per stage** (analysis, architecture, detailed design,
  mapping, tasks — you count each); no-progress exit when a reviewer's gap set is unchanged round-over-round;
  kill-switch `PLAN_STOP` checked before each seat send; a G7 amend re-enters the relevant bounded stage (an
  architecture amend re-clears G3).
- **Human gates:** G1 input recovery + governance / entry / brownfield surface · **G2** architecture
  baseline confirmation (bootstrap only) · **G3** architecture sign-off (always-on; rendered diagram,
  degrade-with-record fallback) · G4 feasibility / governance rejection (incl. the governance two-exit) ·
  G5 clarification (incl. the "Research this" knowledge-gap branch) · G6 exit-early / escalation · **G7**
  final package acceptance · escalation on any guard trip. **No devolved branch** — every review is a
  judgment grade (feasibility, completeness, architecture coverage, task-artifact quality), never
  all-deterministic-CLI, so no gate is skipped and every verdict is yours.

## State recovery

Pause posture (per the shape): note the resume stage on the deliverable. Resume from workspace evidence,
respawning what the stage needs — a respawned producer re-reads the artifacts + the gap list; a reviewer
respawn is cold by design:

| Evidence in the workspace | Resume at |
|---------------------------|-----------|
| no `.mochiko/specs/<feature>/spec.md` | Phase 0 (entry blocked) |
| `slices.md` present | slice-scoped: resolve the current slice (Phase 0 step 6); the rows below then read per-slice artifacts under `slices/<slice>/` alongside the shared feature root |
| `spec.md` present, no `requirements.md` | Phase 1 (produce) |
| analysis artifacts present, no `feasibility-report.md` / `advocate-report.md` this round | Phase 1 (review) |
| analysis not `feasible`+`ready`, within the cap | Phase 1 (loop control) |
| analysis cleared, no `architecture.md` (or bootstrap baseline unconfirmed) | Phase 2 (baseline / produce) |
| `architecture.md` present, not yet signed off (G3) | Phase 2 (review / sign-off) |
| architecture signed off, no `data-model.md` | Phase 3 (produce) |
| design artifacts present, advocate not `ready`, or an open architecture contradiction | Phase 3 (loop control / return to G3) |
| design cleared, no `task-mapping.md` | Phase 4 (mapping produce) |
| `task-mapping.md` present, mapping not `ready`, within the cap | Phase 4 (mapping loop control) |
| mapping cleared, no `tasks.md` | Phase 4 (tasks produce) |
| `tasks.md` present, tasks not `ready`, within the cap | Phase 4 (tasks loop control) |
| all stages cleared, no `plan.md` | Phase 5 (assemble) |
| `plan.md` present, package not yet accepted | Phase 5 (G7) |
| accepted | Phase 6 |
| `.mochiko/specs/<feature>/PLAN_STOP` present | escalate (G6) |

---

**What you own (not the seats):** the stage sequence (Analysis → Architecture → Detailed design →
Structuring) and each stage's loop (round counter, no-progress check, cap, kill-switch, escalation); the
peer-edge sequencing (you open every round and every review pass — delivery is a hand-off, not a start
signal — while routine artifact delivery rides the mesh peer-routed); the verdict against the
default-FAIL done-condition; the architecture-first ordering and its **early sign-off (G3)** before
detailed design; the feasibility-once-per-input-then-re-fire-on-structural-change routing (analysis +
architecture pass, lead-gated); the skip-architect-unless-structural rule for the detailed-design stage;
the completeness reviewer's per-stage skill/mode selection; the design-time deviation return to G3; the
governance two-exit; the human gates (G1–G7); the governance / entry / brownfield prerequisites; `plan.md`
assembly and the single-package acceptance; verifying each seat actually wrote its expected files (a
missing output → log and ask retry/abort); and never letting a producer grade its own output or the two
reviewers collapse into an author. Full rules: `mochiko:loop-discipline`.
