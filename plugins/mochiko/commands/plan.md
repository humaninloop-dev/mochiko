---
description: Create the analysis→design implementation plan via an independent producer→two-reviewer team loop — a standing technical-analyst seat authors the six analysis+design artifacts across two phases (Analysis then Design), a cold principal-architect seat grades cross-artifact feasibility once, a standing devils-advocate seat grades completeness across both phases, the user accepts plan.md at a named gate; governance-gated, two-phase, default-FAIL, bounded, kernel-free. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Plan — Implementation Plan (Analysis → Design)

**Goal:** turn an accepted `spec.md` into an accepted `plan.md` — the six analysis+design
artifacts (`requirements.md` · `constraints-and-decisions.md` · `nfrs.md` · `data-model.md` ·
`contracts/api.yaml` · `quickstart.md`), authored across two phases and independently graded for
**feasibility** (can these pieces be built *together*?) and **completeness** (is anything
missing?) before the user accepts the assembled plan. `$ARGUMENTS` = optional feature ID or
description; empty or detected-from-workspace is handled by triage below.

**You are the lead**, and this is a **team-form command in the mochiko command shape**: Read
`${CLAUDE_PLUGIN_ROOT}/templates/command-shape.md` (both layers) before anything else — the
shape's rules bind here and are not restated; this file carries only plan's parameters. You own
the loop (per-phase round counters, verdict, escalation) and every human gate. This is a
`mochiko:loop-discipline` sound loop; the Contract section below is its authoring-time fill.

## Team-form parameters (shape Layer 2)

Hard-require `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` per the shape. The **authoritative
first-spawn probe** is the producer — always the first seat filled (Phase 1 produces before it
reviews). Transport mechanics + the addressability check: `templates/agent-dispatch.md` (Seat
transport). The no-fallback bet is the same `Contested` dogfood-pilot ruling as the other
team-form commands.

## Session constraints

- Workspace: resolve `<feature>` (an explicit ID from `$ARGUMENTS`, else the most recent
  in-progress feature under `.mochiko/specs/`); seed `.mochiko/specs/<feature>/`. The deliverables
  live alongside the spec they plan.
- Kill-switch: stop and escalate if `.mochiko/specs/<feature>/PLAN_STOP` exists — check before
  each seat send.
- **Deliverables & IDs:** the six artifacts (`requirements.md` FR→TR · `constraints-and-decisions.md`
  C-XXX/D-XXX/IP-XXX · `nfrs.md` NFR-XXX · `data-model.md` entities + sensitivity ·
  `contracts/api.yaml` OpenAPI + `x-integration` · `quickstart.md`) + the assembled `plan.md`,
  authored per the producer's skills and `templates/plan-template.md` — no placeholder tokens. The
  producer's self-disclosure is `techanalyst-report.md` (from
  `templates/techanalyst-report-template.md`) — its Assumptions / Open Questions are the
  producer-authored uncertainty carrier (the shape's producer-authored branch), not confidence
  marks. Feasibility findings live in `feasibility-report.md`, completeness gaps in
  `advocate-report.md`.

## The seats

- **producer** — `mochiko:technical-analyst` (`authoring-technical-requirements`,
  `patterns-technical-decisions`, `patterns-entity-modeling`, `patterns-api-contracts`), one
  **named standing seat across both phases**. Phase 1: author `requirements.md`,
  `constraints-and-decisions.md`, `nfrs.md`; Phase 2: author `data-model.md`, `contracts/api.yaml`,
  `quickstart.md` (+ `techanalyst-report.md` each round) — the standing seat carries the Phase-1
  analysis rationale into the Phase-2 design rather than reconstructing it from the files. Brief it
  per `agent-dispatch`: `spec.md`, the governance obligated-read line (per the prerequisite), the
  brownfield analysis when present, the templates to fill per its skills. Round > 1 within a phase
  is a message to the same seat carrying the reviewers' gap list verbatim (fix the flagged gaps;
  don't regress passing sections). It never grades.
- **feasibility reviewer** — `mochiko:principal-architect` (`review-feasibility`), seated **only when
  the G1 sizing ruling includes it** (dropped under completeness-only / none), spawned **cold
  after the Phase-1 analysis is authored**, never in contact with the producer. Grade cross-artifact
  feasibility from the files → `feasibility-report.md` (three-state `feasible` / `needs-revision` /
  `infeasible`). It grades **once**; re-fire it (a message to the same seat) **only on a structural
  change** — new/changed constraints, expanded requirement scope, or modified NFR targets. It never
  grades Phase 2 (the completeness reviewer carries cross-artifact consistency there). Its output is
  **lead-adjudicated input** (the `review-*` family boundary).
- **completeness reviewer** — `mochiko:devils-advocate` (`review-plan-artifacts`), seated **unless the
  G1 sizing ruling waives it** (kept under both / completeness-only; dropped only under none),
  spawned **cold at the first completeness review**, never in contact with the producer, one
  **named standing seat across both phases**. Phase 1: grade completeness / coverage / consistency from the files →
  `advocate-report.md` (`ready` / `needs-revision` / `critical-gaps`). Phase 2: a message to the
  same seat in **incremental mode** — a full review of the new design artifacts plus a brief
  consistency check back to the Phase-1 analysis (the `review-plan-artifacts` incremental procedure);
  you select the mode and supply the {new design}/{prior analysis} artifact sets. Its retained
  Phase-1 context is what makes the Phase-2 consistency check incremental rather than a full re-read.
  Round > 1 within a phase is a message to the same seat: re-Read the revised files. Its output is
  **lead-adjudicated input**; there is no sized end-stage review — the bounded in-loop critique is
  this workflow's independent validation (declared in the Contract).

## Phase 0 — Prerequisites & entry triage  *(human gate G1)*

1. **Capture** `$ARGUMENTS`; resolve `<feature>` (an explicit ID, else the most recent in-progress
   feature under `.mochiko/specs/`). If empty (the `@`-reference drop bug), recover via **G1**: ask
   the user to re-enter, or to confirm the detected feature.
2. **Governance prerequisite.** Check `CLAUDE.md` for the mochiko governance region
   (`<!-- mochiko:governance:begin -->`). Present → governance reaches the producer natively at
   spawn; add to its brief the **one-line obligated read** naming the `.claude/rules/mochiko/` files
   and skills relevant to what it authors (`paths`-scoped rules don't fire for from-scratch
   authoring). Missing → do **not** silently proceed; surface it (offer `/mochiko:setup` first).
   Never auto-resolve.
3. **Entry gate.** The spec must be done: `.mochiko/specs/<feature>/spec.md` present and accepted
   (workspace evidence). Missing → block and point the user to `/mochiko:specify`.
4. **Brownfield check.** Read the declared project type from `.mochiko/memory/governance-intent.md`.
   Brownfield → require `.mochiko/memory/codebase-analysis.md` (missing → offer `/mochiko:setup` or
   proceed greenfield with a logged warning; >14d stale by file mtime → warn); greenfield → bypass.
   Carry the analysis into the producer's brief when present.
5. **Slice-scoped entry (graduation slices).** If `.mochiko/specs/<feature>/slices.md` exists
   (accepted), the run is **slice-scoped** — apply that file's own **Graduation contract** section for
   slice resolution, the staleness guard, scope (= the slice's stories + its extend obligations),
   extend-mode, graded amendment, and artifact layout; do not restate those rules here (the Graduation
   contract is their single home). **plan's own bindings on top:** a producer designing beyond scope is
   a scope gap → **G4**; a `[MODIFY]` graded amendment is surfaced for this round's reviews with its
   migration flagged for this slice's task breakdown; per-slice `plan.md` + round reports land under
   `slices/<slice>/`, so the done-condition's artifact set reads the six shared artifacts (extended at
   the feature root) + `slices/<slice>/plan.md`; brief each reviewer with the artifact sets {this
   slice's extensions + its `plan.md`} / {the prior accumulated artifacts}, so the extension is graded
   against what earlier slices established.
6. **Review sizing (part of G1).** Run the shape's sized review with these bindings, applied to the
   **in-loop reviewer roster** — plan reviews every round, so the gate sizes the roster's *count* at
   entry, not an end-stage pass. **Weight statement** = story count (the slice's when slice-scoped),
   whole-spec vs slice-scoped, governance tier. **Default keying** = `production`/`regulated` tier or
   a foundation slice → **both** reviewers; `poc`/`internal` light features → **completeness-only**
   (the feasibility reviewer drops — its named risk: cross-artifact contradictions then surface only
   through the completeness reviewer's consistency checks and your verdict, with no dedicated
   feasibility pass); **none** → recorded waiver. **None branch** = the waiver lands in `plan.md`'s
   Review section; your clause-(3) Read of every artifact stays the validation floor and never thins.
   The ruling holds for the run — carried in-session until `plan.md` assembly.

## Phase 1 — Analysis loop  *(you own the round counter and the verdict)*

`round = 1`; the analysis is FAIL until proven. The architect grades feasibility **once**, after the
analysis is authored — not in Phase 2 (don't spend a completeness pass on infeasible requirements).

1. **Produce.** The producer authors `requirements.md` (FR→TR mapping), `constraints-and-decisions.md`
   (Part 3 = IP-XXX infrastructure), and `nfrs.md` (+ `techanalyst-report.md`); on round > 1 the
   message carries the reviewers' gap list for targeted revision (fix flagged gaps; don't regress
   passing sections). The round-1 spawn is the authoritative probe — confirm addressability.
2. **Feasibility (architect, once).** The feasibility reviewer, cold, grades cross-artifact
   feasibility from the files → `feasibility-report.md`.
3. **Completeness (advocate).** The completeness reviewer, cold at first review, grades completeness /
   coverage / consistency from the files → `advocate-report.md`.
4. **Verdict (you).** Read the artifacts + both reports. `feasible` **and** `ready` **and** you find
   no blocking gap → Phase 2. Otherwise classify each finding and route it per `loop-discipline`'s
   gap-routing (knowledge → native `Explore`, the "Research this" branch in G3; preference → G2/G3;
   scope → G4): architect concerns → **G2**; advocate gaps → **G3**; architect `infeasible` →
   escalate as a business-level scope decision, not a routine revision. Then apply the bounds
   (increment `round`; cap / no-progress / kill-switch → **G4** / escalate) and loop to step 1.
   **Re-run the architect (step 2) only on a structural change** — new/changed constraints, expanded
   requirement scope, or modified NFR targets; a clarification-only revision goes straight back to
   step 3.

## Phase 2 — Design loop  *(incremental review; you own the round counter)*

`round = 1`; the design is FAIL until proven. No architect re-review here — the advocate carries
cross-artifact consistency in incremental mode.

1. **Produce.** The producer authors `data-model.md` (entities + sensitivity annotations),
   `contracts/api.yaml` (OpenAPI + `x-integration` boundaries), and `quickstart.md` (+
   `techanalyst-report.md`), carrying its Phase-1 analysis context forward.
2. **Incremental review (advocate).** Message the completeness reviewer in **incremental mode** — a
   full review of the new design artifacts plus a brief consistency check back to the Phase-1
   analysis (the `review-plan-artifacts` incremental procedure); you select the mode and supply the
   {new design}/{prior analysis} artifact sets → `advocate-report.md`.
3. **Verdict (you).** Read the design artifacts + report. `ready` and no blocking gap → Phase 3.
   Otherwise route gaps per `loop-discipline` (→ **G3**), apply the bounds (cap / no-progress /
   kill-switch → **G4** / escalate), and loop to step 1.

**Mid-loop gates (both phases).** **G2** feasibility-rejection: present the architect's concerns and
let the user accept-resolution / relax / keep-as-is / give-direction. **G3** clarification: present a
reviewer's gaps, take answers (logged in-session), and feed them into the next produce — a "Research
this" answer routes the knowledge gap to native `Explore` (per `loop-discipline`), never to the user.
**G4** exit-early / escalation: on a guard trip or stalled gaps, present the last findings and let the
user continue-refining / accept-with-noted-gaps / stop-and-review — the run stays FAIL unless the
human explicitly accepts. None of these ends the loop on its own.

## Phase 3 — Assemble plan.md

After both verdicts clear, assemble the `plan.md` deliverable from `templates/plan-template.md`:
extract the key decisions from `constraints-and-decisions.md`, the entity summary (with sensitivity)
from `data-model.md`, and the endpoint summary (with integrations) from `contracts/api.yaml`.
`plan.md` is the lead's fill-target — a summary over the validated artifacts, not new design.

## Phase 4 — plan.md acceptance  *(human gate G5)*

Reachable only after your clearing verdict. Present the validated plan (decision / entity / endpoint
counts, any noted limitations) and ask the user to **accept** (→ Phase 5; the done-condition is now
satisfied), **amend** (re-enter the relevant phase with the changes as the gap list — still bounded;
it must clear a verdict again), or **reject** (abort; the drafts remain under
`.mochiko/specs/<feature>/`).

## Phase 5 — Finalize

Report the artifacts (the six deliverables + `plan.md` + the round reports the sized roster produced
— `techanalyst-report.md`, plus `feasibility-report.md` / `advocate-report.md` per the G1 roster),
the per-phase round counts, the decision / entity / endpoint counts, a suggested commit
(`docs: plan <feature>`), and the next step (`/mochiko:tasks`). **Clean the intermediate round reports
by default** (retain only on request); never touch `plan.md` or the six artifacts — they are the
deliverables.

**Cost entry.** Append a row to `.mochiko/specs/<feature>/run-costs.md` per
`templates/run-costs-template.md`: ask the user for the `/usage` figure (record `not captured` if
unavailable — never block), plus the run shape you observed — seats spawned, per-phase rounds, the G1
sizing ruling, date / workflow / slice. Manual baseline (the epic-ruled protocol); deeper per-seat
forensics need a transcript parse and stay on-demand.

## Contract (authoring-time fill — governed by `mochiko:loop-discipline`)

- **Done-condition:** default **FAIL**; clears only when **(1)** all six artifacts exist
  (`requirements.md` · `constraints-and-decisions.md` · `nfrs.md` · `data-model.md` ·
  `contracts/api.yaml` · `quickstart.md`), **(2)** each reviewer **on the G1-sized roster** returns
  its clearing status grounded in the files — `principal-architect` `feasible` on the Phase-1
  analysis, `devils-advocate` `ready` on both phases — a reviewer the ruling dropped contributing its
  **recorded waiver** in place of a status, never a silent skip, **(3)** *you* Read the artifacts +
  the roster's reviewer reports and confirm no blocking gap remains (each reviewer's status is input,
  never the gate; your Read of every artifact never thins, whatever the roster), **and (4)** the
  Phase-4 human acceptance on `plan.md` has cleared. Out of rounds = escalate, never done.
- **Producer ↔ validator:** `technical-analyst` (authoring-technical-requirements,
  patterns-technical-decisions, patterns-entity-modeling, patterns-api-contracts) authors both
  phases, never grades; the validators are the **reviewers on the G1-sized roster**, neither the
  producer — `principal-architect` (review-feasibility) grades feasibility, `devils-advocate`
  (review-plan-artifacts) grades completeness — from the files, never authoring. Disjoint agents,
  disjoint skills, structurally separated (reviewers cold-spawned, gap lists lead-routed, no
  producer↔reviewer contact). Under a **none** waiver you + the user are the validator — recorded,
  never silent (your clause-(3) Read never thins).
  **Validation model:** the bounded in-loop critique — every round, at the **G1-sized roster**
  (both / completeness-only / none-with-recorded-waiver, per Phase 0); no sized *end-stage* review
  (the shape's in-loop-critique branch, with the roster count sized at G1).
- **Bounds:** cap **3** produce↔review rounds **per phase** (you count); no-progress exit when a
  reviewer's gap set is unchanged round-over-round; kill-switch `PLAN_STOP` checked before each seat
  send; a G5 amend re-enters the relevant bounded phase.
- **Human gates:** G1 input recovery + governance / entry / brownfield surface + **review sizing
  (both / completeness-only / none-with-waiver)** · G2 feasibility-rejection · G3 clarification
  (incl. the "Research this" knowledge-gap branch) · G4 exit-early / escalation · G5 `plan.md`
  acceptance · escalation on any guard trip.

## State recovery

Pause posture (per the shape): note the resume stage on the deliverable. Resume from workspace
evidence, respawning what the stage needs — a respawned producer re-reads the artifacts + the gap
list; a reviewer respawn is cold by design. The reviewer roster on resume follows the **G1 sizing
ruling**; recover it from the reviewer evidence on disk (a `feasibility-report.md` on disk means the
feasibility reviewer was seated) — or, resumed before any reviewer wrote and with `plan.md` not yet
assembled to carry it, **re-confirm the ruling in one question** before respawning:

| Evidence in the workspace | Resume at |
|---------------------------|-----------|
| no `.mochiko/specs/<feature>/spec.md` | Phase 0 (entry blocked) |
| `slices.md` present | slice-scoped: resolve the current slice (Phase 0 step 5); the rows below then read per-slice artifacts under `slices/<slice>/` alongside the shared feature root |
| `spec.md` present, no `requirements.md` | Phase 1 (produce) |
| analysis artifacts present, the G1 roster's reviewer reports not yet written this round | Phase 1 (review) |
| analysis reviews not clearing on the G1 roster (`feasible`+`ready`, waived seats excepted), within the cap | Phase 1 (loop control) |
| analysis cleared, no `data-model.md` | Phase 2 (produce) |
| design artifacts present, the sized roster not `ready` (or waived), within the cap | Phase 2 (loop control) |
| both phases cleared, no `plan.md` | Phase 3 |
| `plan.md` present, not yet accepted | Phase 4 |
| accepted | Phase 5 |
| `.mochiko/specs/<feature>/PLAN_STOP` present | escalate (G4) |

---

**What you own (not the seats):** the two-phase sequence (Analysis → Design) and the per-phase loop
(round counter, no-progress check, cap, kill-switch, escalation); the verdict against the
default-FAIL done-condition; the architect-feasibility-once-then-advocate ordering and the
skip-architect-unless-structural routing; the Phase-2 incremental mode selection; the human gates
(G1–G5); the governance / entry / brownfield prerequisites; `plan.md` assembly; verifying each seat
actually wrote its expected files (a missing output → log and ask retry/abort); and never letting the
producer grade its own output or the two reviewers collapse into the author. Full rules:
`mochiko:loop-discipline`.
