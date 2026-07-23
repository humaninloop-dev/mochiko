---
description: Generate the implementation task breakdown via an independent producer→reviewer team loop — a standing task-architect seat authors the story→cycle mapping then the cycle-based tasks across two phases (Mapping then Tasks), a single standing devils-advocate seat grades the mapping early then the tasks cumulatively, the user accepts tasks.md at a named gate; plan-gated, two-phase, default-FAIL, bounded, kernel-free. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Tasks — Implementation Task Breakdown (Mapping → Tasks)

**Goal:** turn an accepted `plan.md` into an accepted `tasks.md` — a story→cycle **Mapping**
(`task-mapping.md`, the slicing source of truth) then the cycle-based **Tasks** (`tasks.md`, the
TDD task list the mapping expands into), authored across two phases and independently graded for
task-artifact quality (vertical-slice integrity, TDD test-first ordering, story→cycle→task
traceability) before the user accepts the breakdown. `$ARGUMENTS` = optional feature ID or
description; empty or detected-from-workspace is handled by triage below.

**You are the lead**, and this is a **team-form command in the mochiko command shape**: Read
`${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md` (both layers) before anything else — the
shape's rules bind here and are not restated; this file carries only tasks' parameters. You own
the loop (per-phase round counters, verdict, escalation) and every human gate. This is a
`mochiko:loop-discipline` sound loop; the Contract section below is its authoring-time fill.

## Team-form parameters (shape Layer 2)

Hard-require `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` per the shape. The **authoritative
first-spawn probe** is the producer — always the first seat filled (Phase 1 produces the mapping
before it is reviewed). Transport mechanics + the addressability check:
`templates/agent-dispatch.md` (Seat transport). The no-fallback bet is the same `Contested`
dogfood-pilot ruling as the other team-form commands.

## Session constraints

- Workspace: resolve `<feature>` (an explicit ID from `$ARGUMENTS`, else the most recent
  in-progress feature under `.mochiko/specs/`); the deliverables live alongside the plan they
  break down.
- Kill-switch: stop and escalate if `.mochiko/specs/<feature>/TASKS_STOP` exists — check before
  each seat send.
- **Deliverables & IDs:** `task-mapping.md` (the freehand story→cycle mapping + vertical-slice
  rationale — the **source of truth** for slicing decisions) and `tasks.md` (cycle→TDD tasks:
  foundation cycles sequential, feature cycles `[P]`; a file path per task; `[US#]` story tags;
  `[EXTEND]`/`[MODIFY]` markers; the Story→Cycle table = a **derived echo** of `task-mapping.md`,
  not a second source), authored per the producer's skill and `templates/tasks-template.md` — no
  placeholder tokens. The producer's self-disclosure is `taskarchitect-report.md` (from
  `templates/taskarchitect-report-template.md`) — its Open Questions are the producer-authored
  uncertainty carrier (the shape's producer-authored branch), not confidence marks. The reviewer's
  gap IDs live in `advocate-report.md`.

## The seats

- **producer** — `mochiko:task-architect` (`patterns-vertical-tdd`), one **named standing seat
  across both phases**. Phase 1: author `task-mapping.md`; Phase 2: expand it into `tasks.md`
  (+ `taskarchitect-report.md` each round) — the standing seat carries its own Phase-1 slicing
  judgment forward, so Phase 2 expands the mapping it just authored rather than reconstructing the
  rationale from the file. Brief it per `agent-dispatch`: plan's design outputs, the governance
  obligated-read line (per the prerequisite), the brownfield context when present, the templates to
  fill per its skill. Round > 1 within a phase is a message to the same seat carrying the reviewer's
  gap list verbatim (fix the flagged gaps; don't regress passing slices). It never grades.
- **reviewer** — `mochiko:devils-advocate` (`review-task-artifacts`), seated **unless the G1 sizing
  ruling waives it** (single is the default roster; dropped only under a recorded **none** waiver),
  spawned **cold at the first (mapping) review**, never in contact with the producer, one **named
  standing seat across both phases**. Phase 1: grade the mapping from the file (the
  `review-task-artifacts` Mapping checklist — slice quality, foundation separation, story coverage,
  cycle sizing) → `advocate-report.md`
  (`ready` / `needs-revision` / `critical-gaps`). Phase 2: a message to the same seat in **cumulative
  mode** — a full review of `tasks.md` plus a cross-check back to `task-mapping.md` (mapping↔tasks
  alignment, the story→cycle→task chain, cycle/dependency consistency — the `review-task-artifacts`
  Cross-Artifact Review); you select the mode and supply **both** artifact sets
  {`tasks.md`}/{`task-mapping.md`}. Its retained Phase-1 context is what makes the Phase-2 cross-check
  a cross-check rather than a fresh read of the mapping. Round > 1 within a phase is a message to the
  same seat: re-Read the revised files. Its output is **lead-adjudicated input** (the `review-*`
  family boundary); there is no sized end-stage review — the bounded in-loop critique is this
  workflow's independent validation (declared in the Contract). Single reviewer, never the producer.

## Phase 0 — Prerequisites & entry triage  *(human gate G1)*

1. **Capture** `$ARGUMENTS`; resolve `<feature>` (an explicit ID, else the most recent in-progress
   feature under `.mochiko/specs/`). If empty (the `@`-reference drop bug), recover via **G1**: ask
   the user to re-enter, or to confirm the detected feature.
2. **Entry gate — plan-workflow-complete.** The plan must be done: `.mochiko/specs/<feature>/plan.md`
   present and accepted (workspace evidence). Missing → block and point the user to `/mochiko:plan`.
3. **Governance prerequisite.** Check `CLAUDE.md` for the mochiko governance region
   (`<!-- mochiko:governance:begin -->`). Present → governance reaches the producer natively at
   spawn; add to its brief the **one-line obligated read** naming the `.claude/rules/mochiko/` files
   and skills relevant to what it authors. Missing → do **not** silently proceed; surface it (offer
   `/mochiko:setup` first). Never auto-resolve.
4. **Brownfield-from-plan.** Inherit brownfield context from plan's artifacts; do **not** re-run
   codebase analysis. (The roadmap track — `evolution-roadmap.md` / `[GAP:XXX]` — is a documented
   stub, deferred.)
5. **Read plan's design outputs** (`spec.md`, `requirements.md`, `constraints-and-decisions.md`,
   `nfrs.md`, `data-model.md`, `contracts/api.yaml`) as the producer's inputs — workspace-as-state,
   no registry field.
6. **Slice-scoped entry (graduation slices).** If `.mochiko/specs/<feature>/slices.md` exists
   (accepted), the run is **slice-scoped** — apply that file's own **Graduation contract** section for
   slice resolution, the staleness guard, scope (= the slice's stories + its extend obligations),
   extend-mode, graded amendment, and artifact layout; do not restate those rules here (the Graduation
   contract is their single home). **tasks' own bindings on top:** a cycle serving any story outside
   scope is a scope gap → **G4**; the plan-complete entry gate (step 2) reads `slices/<slice>/plan.md`
   and the producer's inputs are the shared feature-root design artifacts plus that slice plan;
   per-slice outputs (`task-mapping.md` · `tasks.md` · the round reports) land under `slices/<slice>/`,
   so the done-condition's artifact set reads them there; brief the reviewer with the artifact sets
   {this slice's `task-mapping.md`/`tasks.md`} / {the accumulated feature-root plan artifacts}, so the
   breakdown is graded against what earlier slices established.
7. **Review sizing (part of G1).** Run the shape's sized review with these bindings, applied to the
   **in-loop reviewer roster** — tasks reviews every round, so the gate sizes the roster at entry, not
   an end-stage pass. **Weight statement** = story count (the slice's when slice-scoped), whole-spec vs
   slice-scoped, governance tier, and the expected cycle count where the mapping inputs make it evident.
   **Default = single at every tier** (the reviewer seated); at `production`/`regulated` tier or a
   foundation slice, state that single is **strongly recommended** — dropping the only independent
   reviewer is a larger call than plan's dropping one of two, and its named risk under **none** is that
   vertical-slice integrity, TDD test-first ordering, and mapping↔tasks alignment then surface only
   through your clause-(3) Read. **None branch** = the waiver lands in `tasks.md`'s Review section; your
   clause-(3) Read of both artifacts stays the validation floor and never thins. The ruling holds for
   the run — carried in-session until the lead fills that Review section.

## Phase 1 — Mapping loop  *(you own the round counter and the verdict)*

`round = 1`; the mapping is FAIL until proven. The reviewer grades slicing quality **before** the
expensive full TDD breakdown — the tasks analogue of plan's feasibility-once gate (cheap rework
avoidance).

1. **Produce.** The producer authors `task-mapping.md` — the freehand story→cycle mapping +
   vertical-slice rationale, the **source of truth** for slicing decisions (+ `taskarchitect-report.md`);
   on round > 1 the message carries the reviewer's gap list for targeted revision (fix flagged gaps;
   don't regress passing slices). The round-1 spawn is the authoritative probe — confirm addressability.
2. **Early mapping review.** The reviewer, cold, grades the mapping from the file (the
   `review-task-artifacts` Mapping checklist — slice quality, foundation separation, story coverage,
   cycle sizing) → `advocate-report.md`.
3. **Verdict (you).** Read `task-mapping.md` + the report. `ready` **and** no blocking gap → Phase 2.
   Otherwise route each gap per `loop-discipline`'s gap-routing (knowledge → native `Explore`, the
   "Research this" branch in G3; preference → G3) → **G3**, apply the bounds (increment `round`;
   cap / no-progress / kill-switch → **G4** / escalate), and loop to step 1.

## Phase 2 — Tasks loop  *(cumulative review; you own the round counter)*

`round = 1`; the tasks are FAIL until proven. No re-review of the mapping here — the reviewer carries
mapping↔tasks alignment in cumulative mode.

1. **Produce.** The producer expands `task-mapping.md` into `tasks.md` (+ `taskarchitect-report.md`),
   briefed with `task-mapping.md` as the input to expand — carrying its Phase-1 slicing judgment forward.
2. **Cumulative review.** Message the reviewer in **cumulative mode** — a full review of `tasks.md`
   plus a cross-check back to `task-mapping.md` (the `review-task-artifacts` Cross-Artifact Review);
   you select the mode and supply **both** artifact sets {`tasks.md`}/{`task-mapping.md`} →
   `advocate-report.md`.
3. **Verdict (you).** Read `tasks.md` + the report. `ready` and no blocking gap → Phase 3. Otherwise
   route gaps per `loop-discipline` (→ **G3**), apply the bounds (cap / no-progress / kill-switch →
   **G4** / escalate), and loop to step 1.

**Mid-loop gates (both phases).** **G3** clarification: present a reviewer's gaps, take answers
(logged in-session), and feed them into the next produce — a "Research this" answer routes the
knowledge gap to native `Explore` (per `loop-discipline`), never to the user. **G4** exit-early /
escalation: on a guard trip or stalled gaps, present the last findings and let the user
continue-refining / accept-with-noted-gaps / stop-and-review — the run stays FAIL unless the human
explicitly accepts. Neither ends the loop on its own.

## Phase 3 — tasks.md acceptance  *(human gate G5)*

Reachable only after your clearing verdict. **Before presenting, fill `tasks.md`'s `## Review`
section** — the G1 sizing ruling, the review trail, and any waiver (the template's lead-filled
section, not the producer's to author). Present the validated deliverables (cycle / foundation /
feature / `[P]` counts, any noted gaps) and ask the user to **accept** (→ Phase 4; the done-condition
is now satisfied), **amend** (re-enter the relevant phase with the changes as the gap list — still
bounded; it must clear a verdict again), or **reject** (abort; the drafts remain under
`.mochiko/specs/<feature>/`).

## Phase 4 — Finalize

Report the artifacts (`task-mapping.md` · `tasks.md` deliverables + the round reports the sized roster
produced — `taskarchitect-report.md`, plus `advocate-report.md` per the G1 roster), the per-phase
round counts, the cycle / foundation / feature / `[P]` counts, a suggested commit
(`docs: tasks <feature>`), and the next step (`/mochiko:implement`). **Clean the intermediate round
reports by default** (retain only on request); never touch `task-mapping.md` or `tasks.md` — they are
the deliverables.

**Cost entry.** Append a row to `.mochiko/specs/<feature>/run-costs.md` per
`templates/run-costs-template.md`: ask the user for the `/usage` figure (record `not captured` if
unavailable — never block), plus the run shape you observed — seats spawned, per-phase rounds, the G1
sizing ruling, date / workflow / slice. Manual baseline (the epic-ruled protocol); deeper per-seat
forensics need a transcript parse and stay on-demand.

## Contract (authoring-time fill — governed by `mochiko:loop-discipline`)

- **Done-condition:** default **FAIL**; clears only when **(1)** both artifacts exist
  (`task-mapping.md` · `tasks.md`), **(2)** the reviewer **on the G1-sized roster** returns `ready` on
  the Phase-1 mapping **and** on the Phase-2 tasks, each grounded in the files — a reviewer the ruling
  waived contributing its **recorded waiver** in place of a status, never a silent skip, **(3)** *you*
  Read the artifacts + the roster's reviewer reports and confirm no blocking gap remains (the
  reviewer's status is input, never the gate; your Read of both artifacts never thins, whatever the
  roster), **and (4)** the Phase-3 human acceptance on `tasks.md` has cleared. Out of rounds =
  escalate, never done.
- **Producer ↔ validator:** `task-architect` (patterns-vertical-tdd) authors both phases, never
  grades; the validator is the **reviewer on the G1-sized roster**, not the producer —
  `devils-advocate` (review-task-artifacts) grades the task artifacts from the files, never authoring.
  Disjoint agents, disjoint skills, structurally separated (reviewer cold-spawned, gap lists
  lead-routed, no producer↔reviewer contact). Under a **none** waiver you + the user are the
  validator — recorded, never silent (your clause-(3) Read never thins). **Validation model:** the
  bounded in-loop critique — every round, at the **G1-sized roster** (single /
  none-with-recorded-waiver, per Phase 0); no sized *end-stage* review (the shape's in-loop-critique
  branch, with the roster sized at G1).
- **Bounds:** cap **3** produce↔review rounds **per phase** (you count); no-progress exit when the
  reviewer's gap set is unchanged round-over-round; kill-switch `TASKS_STOP` checked before each seat
  send; a G5 amend re-enters the relevant bounded phase.
- **Human gates:** G1 input recovery + governance / entry / brownfield surface + **review sizing
  (single / none-with-waiver)** · G3 clarification (incl. the "Research this" knowledge-gap branch) ·
  G4 exit-early / escalation · G5 `tasks.md` acceptance · escalation on any guard trip. **No G2** —
  tasks is single-reviewer, so plan's feasibility-rejection slot is intentionally unused.

## State recovery

Pause posture (per the shape): note the resume stage on the deliverable. Resume from workspace
evidence, respawning what the stage needs — a respawned producer re-reads the artifacts + the gap
list; a reviewer respawn is cold by design. The reviewer roster on resume follows the **G1 sizing
ruling**; recover it from the reviewer evidence on disk (an `advocate-report.md` on disk means the
reviewer was seated) — or, resumed before the reviewer wrote and with `tasks.md`'s Review section not
yet filled to carry it, **re-confirm the ruling in one question** before respawning:

| Evidence in the workspace | Resume at |
|---------------------------|-----------|
| no `.mochiko/specs/<feature>/plan.md` | Phase 0 (entry blocked) |
| `slices.md` present | slice-scoped: resolve the current slice (Phase 0 step 6); the rows below then read per-slice artifacts under `slices/<slice>/` |
| `plan.md` present, no `task-mapping.md` | Phase 1 (produce) |
| `task-mapping.md` present, the G1 roster's `advocate-report.md` not yet written this round | Phase 1 (review) |
| mapping not `ready` on the G1 roster (or waived), within the cap | Phase 1 (loop control) |
| mapping cleared, no `tasks.md` | Phase 2 (produce) |
| `tasks.md` present, advocate not `ready` (or waived) on the G1 roster, within the cap | Phase 2 (loop control) |
| both cleared, not yet accepted | Phase 3 |
| accepted | Phase 4 |
| `.mochiko/specs/<feature>/TASKS_STOP` present | escalate (G4) |

---

**What you own (not the seats):** the two-phase sequence (Mapping → Tasks) and the per-phase loop
(round counter, no-progress check, cap, kill-switch, escalation); the verdict against the
default-FAIL done-condition (the reviewer grades from the files, you Read the artifacts and decide —
its status is input); the early-mapping-review-then-cumulative-tasks ordering and the Phase-2
cumulative-mode selection (supplying both artifact sets to the reviewer); the human gates (G1 / G3 /
G4 / G5); the plan-complete entry gate and the governance / brownfield prerequisites; verifying each
seat actually wrote its expected files (a missing output → log and ask retry/abort); and never
letting the producer grade its own output or the reviewer collapse into the author. Full rules:
`mochiko:loop-discipline`.
