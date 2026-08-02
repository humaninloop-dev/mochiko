# Strip notes — `commands/plan.md`

Entry formats: `strips/README.md`. Wave context: the plan cluster wave (BACKLOG item 7, the third
one-shot-command wave after specify's v0.13.0 and slice's v0.14.0). The wave also ran the **D2
conversion assessment** (one-shot → team-form) and re-checked the **S8 home-revision checkpoint**
against plan's needs (a standing producer spanning two phases + two reviewer seats, one of them
fire-once — no new shape gap at that wave, when the shape was v2). **Stale as a standing claim:** the
shape is now **v4** (2026-07-30) — see the v0.31.0 entry below. **Stale again:** the shape reached
**v5** at the v0.34.0 pilot below and **v7** at v0.40.0; plan is **v7-form** as of the v0.43.0
conversion at the top of this note.

---

## [v0.48.0] Shape v8 goal+harness rewrite — choreography dies in place
- **Disposition:** superseded → the v8 goal+harness rewrite of this command (whole-file)
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/command-architecture-realignment/record.md` D1–D6; DECISIONS.md 2026-08-02 command-architecture row)
- **Content:** the entire v7-form file superseded — preamble dispatch-brief protocol · Seats & checks table + validation model · team-transport mandate + roster probe (D5: transport-neutral now) · seat lifecycle/recycling · every G-numbered gate, the run-start weight card, floor-gate set, counted bounds/caps/kill-switch, ordering invariants, ground-rules block · run-start declaration + departure trail + per-run contract file · KM-landing command steps · the Recovery section and resume table. Verbatim text below (pre-edit file at the v0.47.0 tree).
- **Kept deliberately:** the Goal's full artifact enumeration with quickstart null-path recording, architecture-signed-before-detailed-design ordering, independent feasibility+completeness grading, traceability and no-contradiction conditions, plan.md as summary-never-new-design · rendered-diagram sign-off with degrade-with-record · governance two-exit rule (conform or ledger; feature gate never overrules the constitution) · infeasible-as-scope-escalation · contradiction-returns-for-consented-amendment · slice-scope + baseline-seed bindings (bootstrap-confirm + ARCHITECTURE.md landing) · no-git-mutation + plain-blocking-acceptance lines · output-style register pointer
- **Consumers assessed:** none — commands are entry points, nothing mounts them.

<details><summary>Verbatim superseded file (v0.47.0)</summary>

````markdown
---
description: Turn an accepted spec into an accepted implementation package — analysis, an architecture design with its own early sign-off, detailed design, and the task breakdown — via an independent producer→reviewer team loop. A standing technical-analyst seat authors analysis then detailed design; a standing system-architect seat authors the architecture delta artifact first among the design work and stops the run at a rendered-diagram sign-off; a standing task-architect seat structures the mapping then tasks; a cold principal-architect seat grades analysis feasibility then the architecture (topology + governance); a cold devils-advocate seat grades completeness then the task artifacts, peer-edged with the active producer; the user signs off the architecture early and accepts the whole package at a named final gate. Governance-gated, architecture-first, default-FAIL, bounded, kernel-free. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Plan — Implementation Package (Analysis → Architecture → Design → Structuring)

**Goal:** turn an accepted `spec.md` into one accepted implementation package — analysis,
architecture (signed off early), detailed design, and the task breakdown — independently graded
for feasibility, completeness, and task-artifact quality. `$ARGUMENTS` = optional feature ID;
empty or detected-from-workspace is resolved at G1.

**You are the lead**: you compose the run and own its counters, every verdict, every escalation,
every human gate, and the user-facing conversation — agents produce and review, you adjudicate.
Every dispatch carries its own brief in the spawn or send prompt — the seat's role and skill
(named as a hint, the agent decides fit), the exact inputs to Read, where the output lands
(write vs return), the bar it must clear, its peer edges and holds, and the independence
reminder that matches the seat (author: never grade your own output; grader: read the artifact
itself, default FAIL, quote evidence) — the seat owns none of this context and gets all of it
from you; on a retry, a peer-routed gap list is pointed at and the round opened, a relayed one
pasted verbatim. This file is self-contained: plan's whole
contract lives here. **First-spawn probe:** the `technical-analyst` producer — analysis
produces before anything reviews.

## Goal

Every deliverable in Bindings exists, alongside the round reports for the grading that actually
ran; the package traces the business requirements through to the task breakdown, carries no
cross-artifact contradiction, and conforms to an architecture target signed off at G3; `plan.md`
assembles that validated set, never new design; the KM landing ran; and the user accepted the
whole package at G7. The package is `/mochiko:implement`'s unchanged entry condition.

**Not done:** a missing artifact, or an unrecorded `quickstart.md` null path · an unsigned or
re-opened architecture target · a design element contradicting that approved target · a departure
with no trail line · out of rounds · G7 unaccepted.

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
cumulative for tasks: {`tasks.md`} / {`task-mapping.md`}) — a policy call, not a hand-off. No
seat ever grades its own output.

**Team transport:** check `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` before anything else — unset →
stop and tell the user how to enable it (settings/env; Claude Code ≥ v2.1.178); the first spawn
is the authoritative probe, and there is no teamless fallback. A seat is spawned with **`name:`**
— a nameless spawn is a one-shot subagent, the forbidden transport; every later round is a
`SendMessage` to that same named seat. Verify from the roster: the `members` array in
`~/.claude/teams/<team>/config.json` (`<team>` = `session-` + first eight chars of the session
ID) must carry the seat's `name` — absent ⇒ kill and respawn explicitly requesting an agent team;
failing again stops the run. Teammates don't load `skills:` frontmatter — every spawn prompt
names the skill and role itself. Tell the user up front they can watch or message any teammate;
announce each seat in one line when filled; never narrate or reply to teammate housekeeping. A
peer-routed gap list is a **hand-off, not a start signal** — a producer revises only when you
open the next round, and your brief carries that hold.

**Seat lifecycle:** the counted unit is the produce↔review round tallied **cumulatively across
the five stages**, not the per-stage cap below — the completeness seat outlives every stage
boundary and is the longest-lived of the governed seats here. At each gate pause, count each
standing multi-unit seat's completed rounds and recycle at ~≥3 — counted, never observed; the
user may order a recycle at any gate; an early, still-warm pause keeps the seat standing. A
respawn is a reset: briefed from the on-disk artifact set alone, versioned successor name
(`producer-2`), never the dead seat's bare name. End-of-need shutdown; no ritual sends.

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
- **Run-start weight card** — evidence: your stated read of the four rigor factors against this
  feature's accepted spec — **reversibility** (rework cost if the design is wrong) · **blast
  radius** (how much downstream work reads the package as authoritative) · **precedent**
  (first-of-kind, or mirroring an audit-cleared pattern) · **input confidence** (scored on the
  artifact under review; a user ruling discounts ambiguity risk only, and one introducing new
  surface raises consistency risk) — plus the process you compose from it — the stated default
  below, or your departures from it · rules: the user · decides: the run's composed process.
  Rigor scales with cost-of-being-wrong, never task size; diff size is at most a hint.
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
  rules: the user · decides: the answer fed forward. You route each finding by judgment: **a
  genuine judgment call is ruled here**; a gap answerable by investigation routes to a native
  `Explore` pass (the "Research this" branch), never to the user; work bigger than the run was
  framed is G6's.
- **G6 exit-early / escalation** — evidence: a cap trip, a gap set unchanged round-over-round,
  `PLAN_STOP`, or a producer designing beyond slice scope · rules: the user · decides:
  continue-refining / accept-with-noted-gaps / abort — the run stays FAIL unless the user
  explicitly accepts.
- **G7 package acceptance** — evidence: every stage's clearing verdict, `plan.md` assembled, and
  the decision / entity / endpoint / cycle counts with any noted limitations · rules: the user ·
  decides: done / amend (re-enter the relevant bounded stage; an architecture amend re-clears G3)
  / reject (drafts remain in place). This is the package's **one** standing acceptance.
- **Floor gates:** every gate above — **G1**'s feature confirm · the run-start weight card ·
  **G2**'s baseline confirm on its bootstrap limb · **G3** · **G4** · **G5**'s judgment-call
  ruling · **G6** · **G7** — the user's whatever you compose, never departable; plan numbers no
  lead-ruled gate, so the not-floor set is empty. Batch rulings into the fewest checkpoints that
  respect these gates. **`plan.md` is the one lead-penned surface here:** wherever
  a review ran it takes one cold-seat grade before G7, non-discretionarily, never your own read
  in place of one — zero cold reads only by a recorded waiver at the weight card.
- **Bounds:** cap **3** produce↔review rounds **per stage** (analysis · architecture · detailed
  design · mapping · tasks), you count each; no-progress exit on a gap set unchanged
  round-over-round; kill-switch `PLAN_STOP` checked before each seat send; out of rounds =
  escalate, never done. Any bound this run declares — including a declared cost range — has you
  as its named counter, **rises only at a user checkpoint**, and is re-declared only on the
  record; busting a bound escalates, never silently continues.
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
- **Ground rules:** kernel-free — no brain code, no capability catalogs, no DAG-mediated
  orchestration. Suggest commits; never run git mutations, never push. No internal machinery
  vocabulary in user-facing prose — the conversation is yours and the user's, in the mochiko
  register (`templates/output-style.md`). User acceptance is plain blocking text, never a timed
  prompt. Deliverables are written as the work progresses, never reconstructed at the end.

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
- **Run-start declaration:** one line at the head of `plan.md`, opened at run start to carry it
  and assembled at the end — the surface Recovery already notes the resume stage on — for a
  default run; a run that departs from the stated default, or declares non-default bounds,
  writes a departure record at `.mochiko/specs/<feature>/plan-contract.md`
  beside the reports instead — the done-condition and bounds as (re-)declared, departures
  taken, and the counter state Recovery reads on resume. Counted unit: the **produce↔review round** — capped per stage by the
  Bounds, tallied cumulatively across the five stages by the lifecycle line.
- **Departure trail:** one line per departure from the stated default, appended under that same
  `plan.md` declaration as it is taken and carried into G7's evidence — never your context alone;
  the trail names the grading that actually ran. Departure is by record, never by silence.
- **KM landing:** `.mochiko/memory/knowledge-management.md` exists → run its ritual + invariants
  under fix-on-sight, and mint new domain terms into `GLOSSARY.md`. A **bootstrap-confirmed
  baseline** (G2) lands as the initial `ARCHITECTURE.md` via the scribe seat. Plan records only
  what plan itself established — the confirmed baseline; implement records what it builds. No copy
  → skip.

## Recovery

Note the resume stage on the deliverable, with the run's counter state — rounds consumed ·
bounds declared · departures taken. Sessions and teams do not survive `/resume`, and a shared
account limit can throttle the team and the main session together — escalation then has nowhere
to go but pause. Resume from workspace evidence, never a context `phase` field, respawning only
what the stage needs — a respawned producer re-reads the artifacts + gap list, and a respawn is
cold by design.

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
| all stages cleared, `plan.md` unassembled | assemble |
| `plan.md` assembled, unaccepted | G7 |
| accepted | finalize — report artifacts, per-stage round counts, the decision / entity / endpoint / cycle counts, a suggested commit (`docs: plan <feature>`), next step `/mochiko:implement` |
| `PLAN_STOP` present | escalate (G6) |
````

</details>

---
## [v0.46.0] Doctrine-purge rewrite — obligated reads out, shape mechanics inlined
- **Disposition:** superseded → the command's own text
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** the preamble's obligated shape/loop-discipline reads and "in the mochiko command shape" framing left; G5's preference/knowledge/scope-gap taxonomy vocabulary reworded to plain lead-judgment routing.
- **Kept deliberately:** all gates/bounds/bindings/recovery — plus inlined weight-card factors, floor rules, transport, lifecycle cadence, mesh hold, ground rules, counter-state recovery.
- **Consumers assessed:** none.

---
## [v0.43.0] The `<!-- shape-form: v7 -->` marker retired from the preamble
- **Disposition:** superseded → deleted. The marker was added by this same version's conversion
  entry below and retires in the same version, at the wave close.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-01 wave-close
  ratifications row, *shape-form marker retirement when the last command converts*; the trigger
  was written into the marker clause itself). **Ground and full record:**
  `.mochiko/strips/command-shape.md` [v0.43.0 wave close], entry 1 — *The form marker and its
  Conformance bullet retired* — not restated here.
- **Content (verbatim):** `<!-- shape-form: v7 -->`
- **Kept deliberately:** the entire preamble otherwise — goal line, obligated reads, probe seat —
  and every P18–P20 binding the marker used to gate. The slots bind unconditionally now; nothing
  the marker declared was lost, because the marker declared only which grading branch to take, and
  there is one branch.
- **Consumers assessed:** `validation-command-shape` check 20 was the sole grep consumer and its
  form branch retired in the same ceremony. All six commands swept together — a marker left in any
  one of them would be the only file in the library still declaring a form.
- **Measured:** `commands/plan.md` **15,842 → 15,817 B** (−25). Derived figures in this note's
  conversion section re-measured accordingly, superseded values kept inline.

# v0.43.0 — the v6→v7 conversion

**Wave context:** shape **v7** landed at v0.40.0 (`lead-owned-process-flexibility`,
`.mochiko/brainstorms/lead-owned-process-flexibility/record.md`; `DECISIONS.md` 2026-08-01 — the
lead-owned-process-flexibility row plus the shape-v7 wave-close ratification row), with **D4**
ruling **convert-on-touch** and all six commands staying v6-form. This wave converted `implement`
first — audit-cleared, and the precedent this conversion follows — and was **user-widened on
2026-08-01 to all six commands**; plan is one of the five that followed. BACKLOG: "convert-on-touch
residuals".

The **first-conversion ceiling-term obligation** was discharged at implement
(`.mochiko/strips/validation-command-shape.md` [v0.43.0] — the `+120` P18 and `+110` P19/P20 terms).
plan measures its blocks against those terms and **needs no re-key**.

**Post-conversion measurement, all blocks, body-only in words** (`## Heading` lines excluded per
check 6; the `# ` title counts to the preamble, as implement's measurement counted it): preamble
**99/130** (the 4-word form marker retired at the wave close) · Goal **108/150** (was 121) · Seats & checks **350/430**
<!-- Preamble provenance, verified against git rather than assumed, because the wave found this
     convention published both ways across the notes: plan's v0.34.0 figure of 99 was itself
     measured TITLE-INCLUDED — the pilot commit (7898d86) measures 99 incl. title / 87 excl., and
     the note published 99. So the baseline above is like-for-like, not 8–12 w light. plan's `# `
     title is 12 tokens. The ceiling verdict is convention-independent here either way: 99/130
     title-included, 87/130 title-excluded (published as 103/91 while the 4-word form marker
     stood; re-measured at the wave-close sweep) — under 130 on both readings, so no preamble trim is
     owed and no supersession entry arises from it. -->

(unchanged) · Constraints **927/1020** (was 788) · Bindings **337/410** (was 231) · Recovery
**251/298** (unchanged in count; two row keys re-worded, below).

Term derivation as check 6 requires: **G = 8** — the seven prior gate lines plus the run-start
weight card, all eight carrying the complete three-part `evidence:`/`rules:`/`decides:` form — so
Constraints is 90·(8+2) = 900 **plus the new +120 P18 term** = 1020. **S = 6** and **R = 17**, both
unchanged, Seats carrying the P17 `+60` term (370 + 60 = 430). **A = 15**, unchanged: ten
deliverables (`requirements.md` · `constraints-and-decisions.md` · `nfrs.md` · `architecture.md` ·
`data-model.md` · `contracts/api.yaml` · `quickstart.md` · `task-mapping.md` · `tasks.md` ·
`plan.md`) plus five round reports, with `ARCHITECTURE.md` and `GLOSSARY.md` excluded as KM fold
targets — so Bindings is 90 + 12·15 + 30 (KM) **plus the new +110 P19/P20 term** = 410.

> **The A-term judgment implement recorded holds here unchanged.** P19 names `plan-contract.md` as
> a **departing** run's per-run carrier. It is **not counted in A**: neither a deliverable nor a
> round report, and it exists only on a departing run. Counting it would only loosen the check.

> **Datapoint for the `+110` term, recorded so a later re-key is judged and not guessed.** plan's
> P19/P20 pair measures **106 w** against implement's **88 w** — the basis the term was sized on —
> so the second measured v7 binding already exceeds the first by 20 w. It is **not a re-key case**:
> the term is not what binds here, Bindings landing at 337/410 (**17.8%** headroom, wider than
> implement's 9.3%). The pair is larger because plan's declaration carries two homes plus a
> two-clause counted unit (the per-stage cap *and* the cumulative lifecycle tally), where
> implement's counted unit is one word. A conversion that merely fits is not a re-key case.

## [v0.43.0] The Goal's end state loses its six reviewer clearances and its lead-read clause

- **Disposition:** superseded → rewritten in place as artifact state. What the clearances certified
  survives as **properties of the package**: requirements traced through to the task breakdown,
  no cross-artifact contradiction, and conformance to the signed-off architecture target.
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, ratified at **A4**, 2026-08-01: *"Goal
  blocks lose process residue. Done = artifact state + floor compliance + user acceptance"*; graded
  by `validation-command-shape` check 23, v7-form only). The fact map named this exact surface —
  **F56**: plan's Goal "hard-codes **six** distinct reviewer clearances into the done-condition …
  the densest mandated-review-branch surface in the set" — and the v0.40.0 shape wave deferred its
  rewrite to this touch.
- **Content (v6, verbatim — the two spans that left):**
  ```
  `principal-architect` returned `feasible` on the analysis
  **and** on the architecture pass; `devils-advocate` returned `ready` on the analysis, the
  architecture coverage, the detailed design, the mapping **and** the tasks, each grounded in the
  files;
  ```
  ```
  you Read the
  artifacts and reports and found no blocking gap;
  ```
- **Protected content: none of it is in plan's protected set, and this was checked before the cut.**
  The v0.34.0 **CS-D8 survivor re-grade ledger** below resolves all 23 protected rows, and not one
  names the Goal's **end state** as its home: verdict-ownership / no-devolved-branch → the Ordering
  invariants + the validation-model line · feasibility's lead-gated fire-once engagement → the seat
  row + ordering invariant · the mode-selecting message → the validation-model line ·
  **AD-D1/D2/D5/D8** → **G3** and the ordering invariant · **AD-D9** → **G4** · **AD-D3** → **G2**,
  Bindings and the KM binding · **AD-D7** → two seat rows + Bindings. **Every one of those homes is
  untouched by this conversion** — the architecture-primitive content from the v0.32.0 merge is
  carried, not edited.
- **Kept deliberately:**
  - **The `quickstart.md` null path** — the one protected row whose ledger home *is* the Goal
    ("Bindings (conditional + null path) **and** the Goal's not-done states"). Its not-done state is
    untouched, word for word.
  - **The three graded properties, in substance.** FR→TR traceability, cross-artifact consistency
    and conformance-to-the-approved-architecture are what `review-plan-artifacts` grades and
    `review-feasibility` contradicts; they now read as states of the package rather than as statuses
    a seat returned. "with no unresolved contradiction" survives as "carries no cross-artifact
    contradiction".
  - **G3's clearance — floor compliance, not residue.** G3 reads `rules: the user`, so under D6(b)
    its clearance belongs in done exactly as "the user accepted … at G7" does. Reworded only
    ("conforms to an architecture target signed off at G3"); the not-done state "an unsigned or
    re-opened architecture target" is untouched.
  - **"each grounded in the files"** — the evidence-not-say-so bar. Its home is the obligated
    `mochiko:loop-discipline` read, verified by reading it this run: `SKILL.md:58` ("working from
    the artifact itself — not from the producer's say-so") and `:68` ("A PASS is invalid unless the
    evidence was actually Read from the real artifact"). plan's validation-model line keeps its
    lead-adjudicated-input clause unedited.
  - **"the KM landing ran" and "the user accepted the whole package at G7"** — both explicit
    end-state elements in the shape's own Goal spec, so neither reads as residue.
  - **The round reports** — not dropped from the end state, **re-scoped to what ran** ("alongside
    the round reports for the grading that actually ran"). Requiring all five unconditionally would
    re-impose the stated default as an obligation, which is the residue class itself; scoping them
    to the grading that ran is the honest-trail invariant reading the same evidence.
- **Consumers assessed:** not a shared primitive. Two cross-file consumers checked: the grader's
  check 23 (this Goal is text it was written against — `.mochiko/strips/validation-command-shape.md`
  [v0.40.0]) and the four commands still v6-form (specify · slice · setup · brainstorm), whose Goal
  blocks **stay exactly as written** — the residue clause is v7-form-only and each converts at its
  own touch.

## [v0.43.0] Two not-done states superseded — the reviewer-status pair

- **Disposition:** superseded → deleted from the Goal. Both rules stand unchanged at their ledgered
  homes in **Constraints** and the validation-model line.
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, as above).
- **Content (v6, verbatim):**
  - `any reviewer status short of `feasible`/`ready``
  - `a reviewer status taken as the gate without your read`
- **Protected content, leaving by ruling and named as such:** the second is the Goal-side echo of
  the v0.31.0 *Kept deliberately* row "Every verdict stays the lead's; **no devolved branch**",
  whose CS-D8 ledger home is the **Ordering invariants** line ("**No devolved branch** — every
  review here is a judgment grade, so no gate is skipped and no unit clears unread") **plus** the
  validation-model line ("every verdict is yours") — not this Goal state. The protected content
  stays where the ledger put it; only the echo left. Same disposition implement's conversion made
  for "a non-clean cycle advanced without your verdict", on the same ground: it made the lead's own
  process step a done-condition element, and it can never be rescued as a floor gate, because a
  lead's read is not a `rules: the user` line and check 21's floor-gate test keys on that.
- **Kept deliberately:** the whole verdict-ownership and no-devolved-branch surface, unedited, at
  both homes; `infeasible` as a distinct escalating state, at **G4**; and every other not-done
  state, unedited.
- **Consumers assessed:** as above — not a shared primitive; grader check 23 and the four v6-form
  commands, both unaffected.

## [v0.43.0] Recovery's two `plan.md` resume keys re-keyed from presence to assembly

- **Disposition:** superseded → re-worded in place, forced by P19. With the declaration opening
  `plan.md` at run start, *file presence* no longer discriminates the assemble and acceptance
  stages; *assembly* does. Left unfixed, a resumed lead reading "`plan.md` present, unaccepted"
  would jump a run's first minute straight to G7.
- **Tier failed:** n/a — supersession by ruling (**OQ-2 / A2**, 2026-08-01 — the run-start
  declaration's home; this re-key is that ruling's consequence in this file).
- **Content (v6, verbatim → v7):**
  - `| all stages cleared, no `plan.md` | assemble |` → `| all stages cleared, `plan.md` unassembled | assemble |`
  - `| `plan.md` present, unaccepted | G7 |` → `| `plan.md` assembled, unaccepted | G7 |`
- **Kept deliberately:** both rows' resume targets and the table's other fifteen rows, unedited; and
  the pause line ("Note the resume stage on the deliverable"), which names the same surface P19
  binds — so no edit was owed there, matching implement's judgment on its own Recovery block.
- **Consumers assessed:** not a shared primitive. `templates/plan-template.md` **assessed and not
  edited**: the declaration is one lead-written line at the head of the artifact, above the summary
  the template fills, and the template's own head is a metadata line rather than a rigid schema.
  Flagged here rather than silently edited — a template change is its own primitive edit with its
  own audit.

*Pure additions this wave, riding the decision row rather than these entries:*

- **The form marker** `<!-- shape-form: v7 -->` in the preamble — check 20's branch key.
- **The run-start weight-card gate line** (P7) — U1-A's standing user stop, in the three-part
  countable form, taking **G from 7 to 8**.
- **`**Floor gates:**`** (P18) — the floor set (all eight gates, two of them limb-scoped) with the
  empty not-floor set stated rather than inferred, plus the lead-penned surface named (judgment 2
  below).
- **`**Run-start declaration:**`** (P19) and **`**Departure trail:**`** (P20) in Bindings — the
  declaration at the head of `plan.md` for a default run, an instantiated `plan-contract.md` for a
  departing one, and the **produce↔review round** named as the counted unit (check 22), the same
  unit the Bounds cap per stage and the P17 lifecycle line tallies cumulatively.
- **One new not-done state** — `a departure with no trail line`, the honest-trail invariant made
  visible in the Goal as floor compliance.

**Three judgments made here rather than deferred, flagged for the grader.**

1. **The floor-gate set is all eight, and the not-floor set is empty.** The test is *who rules*, and
   every gate in plan reads `rules: the user` — grep-verified this run, and the fact map records the
   same at **F55** for the pre-conversion seven. Where implement kept two gates departable, plan has
   none to keep: it numbers no `rules: you` gate at all, implement's cycle checkpoint having no
   analogue here (the ordering invariants declare **no devolved branch**).

   **Why G1 is floor here although implement's G1 is not.** implement cleared its G1 on a structural
   ground — its **package gate** is floor and its evidence is the resolved feature's package, so a
   lead that composes out G1's confirm still puts the resolved feature in front of the user on the
   very next gate. **plan has no second gate carrying that evidence**: the weight card's evidence is
   the rigor read, not the feature resolution. That leaves only the narrow explicit-ID argument,
   which implement's own audit rejected as too narrow for saying nothing about the detected-feature
   branch. And the confirm is protected: the [v0.37.0] entry below kept it **on its own merit** —
   "an expensive run must not open silently on a guessed feature" — by user ruling. Marking it
   departable would be a behavior change to protected content that no ruling in this wave
   authorizes.

   **Two limbs scoped rather than marked whole**, per the precedent's blocking-vs-floor lesson:
   **G2** is floor *on its bootstrap limb* (a present `ARCHITECTURE.md` skips the gate — its own
   line already says so), and **G5** is floor *on its preference ruling* only (a knowledge gap
   routes to `Explore`, a scope gap to G6 — neither is the user's). *When* each floor gate is
   presented stays the lead's under **D3**'s consolidation authority, which is home doctrine and is
   deliberately not restated in the command.

2. **`plan.md` is named as the lead-penned surface, although P11 is producer-authored.** Check 21(2)
   keys the always-cold-graded naming on P11's lead-penned-*record* branch, which plan does not bind
   — its uncertainty carrier is each report's Assumptions / Open Questions. Reading the check
   narrowly would let plan state an absence and stop, and that reading is wrong on the floor's own
   words: `plan.md` is **your fill-target** (Bindings), so it is a lead-penned **deliverable**, and
   Layer 1 invariant 2 says one ships with zero cold reads *only by a recorded user waiver*. Before
   this conversion `plan.md` went from the lead's pen to G7 with no independent read at all. The
   binding states the floor and nothing beyond it — one cold-seat grade **wherever a review ran**,
   the waiver at the weight card — so a run that composes every review out is not forced into a
   grade; it is forced to have the user say so.

3. **The declaration and the trail share `plan.md`'s head, opened at run start.** plan's other
   artifacts are producer-authored and revised in place; `plan.md` is the package's summary, the
   lead's own surface, and what G7 accepts — the only durable home where the user meets the trail at
   the gate that reads it, which is why P20 carries it into G7's evidence. The cost is a `plan.md`
   that exists from run start carrying only its declaration, which is what forced the Recovery
   re-key above. **`requirements.md` was rejected as a home:** it is producer-authored and rewritten
   across revision rounds, so a trail parked there is a trail a producer can overwrite — the same
   hazard that ruled out `cycle-report.md` at implement.

**On the revived `plan-contract.md` name — not a re-add, and carrying no `RETURNED:` entry.** P19's
departing-run carrier re-uses the filename retired at [v0.15.0] below ("Per-run contract fill").
What was stripped there was an *unconditional, authoring-time* fill whose values were constant ("a
per-run form whose values are constant at authoring time is ritual, not proof"); what **A2** revives
is a *conditional, per-run* carrier instantiated **only** when a run departs from the stated default
or declares non-default bounds — values D1-as-amended makes genuinely vary per run. Same file name,
different rule, different ruling; the stripped text does not return.

**File growth.** `commands/plan.md` **14,344 → 15,817 B** (+1,473; words 1,986 → 2,218, +11.7%).
Attribution, each construct measured on its own text, and reconciling exactly to the file delta:

| construct | bytes | words |
|---|---|---|
| ~~`<!-- shape-form: v7 -->` marker~~ — added here, **retired at the wave close** | ±0 | ±0 |
| run-start weight-card gate line (P7) | +280 | +46 |
| `**Floor gates:**` — the eight-gate floor set + the lead-penned surface (P18) | +559 | +93 |
| `**Run-start declaration:**` (P19) | +558 | +79 |
| `**Departure trail:**` (P20) | +172 | +27 |
| Goal block, D6(b) residue strip | −107 | −13 |
| Recovery, two resume keys re-worded | +11 | 0 |
| **net** | **+1,473** | **+232** |

**R21 — still open at half, and this conversion does not close it.** implement measured the heavy
site; `lead-owned-process-flexibility` **R21** (narrowed by **A3** to the estimate alone) asks for
one light run as well. plan is the library's *second*-heaviest command, not a light one, so the
light-site residue stands. What plan adds to the record is a second heavy-ish datapoint: **+1,498 B
on every plan run** (10.4% on top of the command itself), plus one declaration line on a default
run, plus ~15–25 w per departure, plus — departing runs only — `templates/workflow-contract.md`'s
5,572 B and a filled copy of comparable size at `plan-contract.md`.

## [v0.37.0] `@`-reference recovery superseded — the platform bug it named is resolved
- **Disposition:** superseded → user ruling (2026-08-01). The bug-attributed re-enter workaround retires; the detected-feature resolution survives with a confirm (see *Kept deliberately*).
- **Tier failed:** n/a — supersession by ruling (`.mochiko/decisions/2026-08-01-at-reference-recovery-superseded.md`; `DECISIONS.md` 2026-08-01).
- **Content (superseded, verbatim):** G1's decides-clause tail — "Empty `$ARGUMENTS` (the known `@`-reference drop bug) → ask the user to re-enter it, or to confirm the detected feature."
- **Kept deliberately:** the *confirm the detected feature* half, retained on its own merit (an expensive run must not open silently on a guessed feature) and strengthened into the resolution rule — G1 now decides "the resolved `<feature>` (an explicit ID, else the most recent in-progress feature under `.mochiko/specs/`, confirmed with the user before the run opens)". Only the re-enter workaround and the bug attribution left.
- **Consumers assessed:** the recovery spanned five commands — `specify` · `plan` · `implement` · `slice` · `brainstorm`; each carries its own v0.37.0 entry. plan/implement/slice keep the detected-feature confirm; specify/brainstorm keep their empty-args ask (attribution only removed).
- **Protected-set note:** `command-succinctness-strip` record §7 named the `@`-reference recovery among the hard-won fixes verbosity encodes, and this pilot (v0.34.0) restored it under independent-audit check 14 after a first draft dropped it (see that entry's ledger row below). That protection premise — "a platform bug silently corrupts `$ARGUMENTS`" — is spent now the bug is resolved; this entry is the deliberate supersession a future check-14 pass should read, not a re-drop.

# v0.34.0 — the goal-shape pilot (CS-D10 step 2)

**Wave context:** command goal-shape rebuild, **step 2 of 4** — the pilot (design:
`.mochiko/brainstorms/command-succinctness-strip/record.md`, CS-D3/D4/D5 + D8 + D10;
`DECISIONS.md` 2026-07-30). plan was chosen as the pilot because it is the heaviest file and
carries every content class: 6 seats, 7 gates, slice-scoping, the architecture stage. Authored
against **shape v5** (`.mochiko/strips/command-shape.md` v0.33.0) with the obligated
`loop-discipline` read **retained** — the drop is step 4 and checkpoint-gated, so a v5 command
that omits it is non-conformant, not early. This file's rewrite is the **first live run** of the
revised `validation-command-shape`, including the negative direction of check 1 (plan declares the
in-loop branch, so it must **not** reference `sized-end-stage-review.md` — it does not).

**Measured: 4,439 → 1,950 words (−56.1%), 33,833 → 14,084 B (−58.4%)** — `wc`-measured after the
fix round, superseding this headline's pre-fix figures (1,940 w / 14,053 B / −56.3% / −58.5%).
Against the pre-pilot measured floor of 1,791 w: **+159 w (+8.9%)** — over, not under, which is the
safe side of CS-D8 (landing materially *under* a floor row would signal dropped content). The
overage is accounted line by line: the completeness reviewer's mode-selection binding (~45 w, a
v0.31.0 *Kept deliberately* item the floor draft compressed too far), the G3 render fallback
promoted from an HTML comment to visible body text (~50 w), G2's greenfield-degeneration case
(~20 w), `GLOSSARY.md` minting in the KM binding (~8 w), and the fix round's two audit-mandated
restores (~31 w — the `@`-reference recovery and the `preference → G5` class), less the 21 w of G7
provenance relocated to make room under the Constraints ceiling. Run-level: the file drops
19,749 B while the v5 shared read floor adds 3,225 B → **−16,524 B per plan run**, against the
−17,430 B the floor projected.

> **Standing habit adopted (auditor-suggested, 2026-07-30):** re-run `wc` and sweep every headline
> figure after **each** fix round, not only at first delivery. This was the **third** stale-headline
> instance in this build (the grader note's 180/1,861/12,441 at the v0.33.0 delta, this note's
> pre-fix figures here, and the interim Constraints 782/791 correction) — always the same cause: a
> summary written before the last edit landed. Carried into the step-4 wave briefing material so
> five commands do not reproduce it five times.

Block sizes against the grader's ceilings (terms as the grader counts them — **G=7** gate lines,
S=6 seat rows, A=15 artifacts, R=17 resume rows): preamble 99/130 · Goal 123/150 · Seats & checks
317/370 · **Constraints 791/810 (97.7%)** · Bindings 233/300 · Recovery 253/298.

**The ceiling was genuinely tested, and it held.** The fix round's two mandated restores took
Constraints to **812/810 — two words OVER**, a real floor FAIL on check 6. The restored content is
protected (check 14 demanded it), so it could not go; and loosening a ceiling I calibrated myself,
to fit a file I authored myself, is precisely the quota-override the sibling wave's D1 forbids —
in the opposite direction. Resolved instead by relocating the one piece of **pure provenance** in
the block: G7's sentence naming which two signatures dissolved into it (21 w), whose home is this
note's v0.32.0 entry. Constraints lands at **791/810** with 19 words spare.

Datapoint for the checkpoint: plan is the heaviest command, so if any Constraints block cannot fit
90·(G+2), it is this one — and it fits, but only after provenance was moved out under audit
pressure. **Recommend confirming the ceiling, not loosening it**, with the caveat that it leaves
little room for a command that wants narrative provenance in Constraints. That is arguably the
ceiling working as designed: provenance belongs in a non-loaded note, not in a file paid on every
run.

## [v0.35.0] Ceremony polish — the pilot's run-level figure corrected to the live shared floor

- **Disposition:** corrected in place (correction class)
- **Tier failed:** n/a — figure correction at the wave ceremony
- **Cause:** the **stale-summary failure mode** this note's own standing-habit block quote names,
  arriving one more time: the run-level line was keyed to a shared-floor delta of +2,895 B, which
  was correct when measured and went stale when `command-shape.md` grew to 16,735 B.
- **Content:** the v5 shared read floor adds **+3,225 B**, not +2,895, so the pilot's run-level
  result is **−16,524 B per plan run**, not −16,854 (the file's own 19,749 B drop is unchanged and
  re-verified). Against the −17,430 B the floor projected, the pilot still lands inside its
  projection, so no conclusion in this note reverses — only the number.
- **Kept deliberately:** the 19,749 B file drop, the accounted-overage breakdown, and the
  standing-habit block quote, which now has a fourth instance to its name.

## [v0.34.0] The phase body and the Contract section retired into the five-block anatomy
- **Disposition:** superseded → the goal-shaped anatomy. `Phase 0`→**G1** + the Slice-scope
  constraint · `Phase 1`→ the seat rows + G4/G5/G6 + the ordering invariants (its step-4 verdict
  narration is the record's D5 fold (a) graded exemplar, distilled to exactly the three ruled
  constraint lines) · `Phase 2`→**G2**/**G3** + ordering invariants · `Phase 3`→ the
  design-contradiction-returns-to-G3 invariant · `Phase 4`→ the mapping-before-tasks invariant +
  the mode-selection binding · `Phase 5`→ `plan.md` in Bindings + **G7** · `Phase 6`→ the KM
  binding + the Recovery table's accepted row. The `Contract` section's four clauses →
  **Goal** (done-condition + not-done states), the **Seats & checks** table (producer↔validator),
  **Constraints** (bounds + gates).
- **Tier failed:** n/a — supersession by ruling (**CS-D3** condition-first documents · **CS-D4**
  "the connective procedure is deleted, and what survives is *restructured*" · **CS-D5** the
  five-block anatomy and the Contract-as-document inversion).
- **Content:** ten `## Phase`/`## Contract`/`## State recovery` sections, 2,873 words of ordered
  procedure and appendix. Not reproduced verbatim here — every *rule* inside them survives in the
  ledger below, and the deleted remainder is connective narration ("Then apply the bounds…", "loop
  to step 1", step numbering, and the lead's job description restated per phase). Recoverable in
  full at `git show c47684d:plugins/mochiko/commands/plan.md`.
- **Kept deliberately:** every gate, bound, routing decision, trigger, ordering rule and artifact
  binding — see the CS-D8 ledger below, which resolves each one individually.

## [v0.34.0] The `What you own (not the seats)` footer deleted
- **Disposition:** deleted.
- **Tier failed:** 1 — a declared duplicate. 157 words restating the gate list, the counter
  ownership, the verdict ownership, the peer-edge sequencing, the feasibility routing, the
  skip-architect rule, the deviation return, the governance two-exit, and the collapse
  prohibition — every one of which is now a Constraints line or a Seats-table cell. The checker
  map recorded this footer class as already-deduped-once at v0.13.0–v0.17.0 and still surviving
  (record §9.4); the anatomy leaves it nowhere to hide.
- **Kept deliberately:** nothing was unique to it. The one clause with no other home — "verifying
  each seat actually wrote its expected files (a missing output → log and ask retry/abort)" — is
  **not** dropped: it is the lead's dispatch hygiene, and it survives as the Recovery block's
  evidence-driven resume (a missing artifact *is* a resume row) plus G6's escalation menu.

## [v0.34.0] The `shape-exception` marker retired — its ground dissolved at v5
- **Disposition:** superseded → the AD-D8/R5 degrade-with-record fallback survives as **visible
  Constraints content** on the G3 line; the `<!-- shape-exception: ... -->` marker around it is
  retired. plan now carries **zero** exception markers.
- **Tier failed:** n/a — supersession by ruling (**CS-D8** re-grade + the checkpoint's
  re-justify-or-supersede instruction).
- **Content (the retired marker, verbatim):** `<!-- shape-exception: D8/R5 — when an attended
  session has none of those render surfaces, the gate degrades with record: present the diagram
  source + component table and record "presented un-rendered" on the artifact (a recorded absence,
  mirroring waiver discipline). Plan is never hard-blocked by rendering. -->`
- **Grounds for retirement, stated plainly because this is the pilot's one contestable call:** the
  marker existed because the fallback *mirrored the shape's waiver discipline* — recorded absence
  rather than silent degradation. At v5 that discipline no longer lives anywhere plan reads: it
  left Layer 1 with the sized-end-stage-review block, into a conditional home
  (`templates/sized-end-stage-review.md`) that plan is **forbidden** to load (it declares the
  in-loop branch; check 1 enforces the negative direction). A marker whose cited restatement target
  is unreachable from the graded file points a future auditor at content they cannot find — the
  exact false-positive class the v0.33.0 grader fix pass named for check 8's homeless markers. The
  fallback restates nothing in plan's v5 read set, so it is plain P7 content.
- **Kept deliberately:** the fallback's every element — the trigger (no render surface in an
  attended session), the degraded presentation (diagram source + component table), the recorded
  stamp ("presented un-rendered" on the artifact), and the never-hard-blocked guarantee. It is now
  *more* visible than at v4, where the whole rule sat inside an HTML comment.
- **Consequence for the audit:** plan's contribution to the surface's `shape-exception` inventory
  goes 1 → 0; `setup.md:100–101` remains the only live marker, unexamined here and due at step 4.

## [v0.34.0] Skill-owned content stripped from the command body
- **Disposition:** relocated → the skills that already own it (no new home written; verified by
  reading each skill's declaration this run).
- **Tier failed:** 1 (altitude).
- **Content:**
  - The architecture artifact's **scope bound** — "scoped to the delta neighborhood past the
    artifact's size threshold (the full-system view is linked, never inlined — the same scope bound
    governs the no-delta presentation)". Home: `mochiko:patterns-system-design`, which states
    "scopes the diagram to the delta neighborhood (changed components + direct collaborators; past
    a threshold the full view is linked)". **Kept deliberately:** plan's G3 line still says the
    no-delta case presents the *neighborhood-scoped* diagram — the binding survives as a reference,
    the rule's statement does not.
  - The **ADR discipline** for topology alternatives — "Genuine-alternative topology choices get
    D-XXX rows here (existing ADR discipline); the delta summary links each structural change to
    its D-XXX row, never restating it". Home: `mochiko:patterns-technical-decisions` (the
    decision/ADR technique) + `patterns-system-design` (the delta-summary→D-XXX link).
    **Kept deliberately:** the *designated structural-decisions section* and its architect
    ownership stay in plan's Bindings — that is plan's own artifact binding, not ADR technique.
  - The standing seats' **retention rationale** — e.g. "Its retained context is what makes each
    later stage's check incremental rather than a cold re-read". Home: this note's v0.15.0
    conversion entry, which records the retention bet in full. The Seats table carries the
    operative fact (standing, and across which stages).

## [v0.34.0] CS-D8 survivor re-grade ledger — every protected line resolved

CS-D8 (extended by user ruling U4) protects two sets: `KEPT:`/Tier-2-evidenced lines, **and** every
line traceable to a `DECISIONS.md` row. plan carries **no `KEPT:` survivor-provenance entries**;
its protection set is the *Kept deliberately* fields of the two prior supersessions plus the
DECISIONS row trace. Grepped before any cut, per D8's enumeration procedure. **All 23 rows survive
translated — zero superseded, zero dropped.**

**Two rows were restored at the pilot fix round, not found by the author.** The independent audit
FAILed the pilot on check 14 (preserved responsibilities) for the `@`-reference recovery and the
`preference → G5` routing class — both genuinely dropped in the first draft, both restored below
and marked. Recorded here rather than silently folded, because the pattern matters for step 4: the
losses were in *compressed evidence clauses*, not in deleted sections — G1's evidence list and
G5's routing enumeration each lost a clause while the surrounding gate line still read as
complete. The five-command wave should grep the routing classes and the named-cause recoveries
per command rather than trusting a gate line that looks whole.

| protected line | source | resolved |
|---|---|---|
| Every verdict stays the lead's; **no devolved branch** (plan has no deterministic-CLI verification, so shape D3's branch cannot apply — declared, not left implicit) | v0.31.0 *Kept deliberately* | Ordering invariants: "**No devolved branch** — every review here is a judgment grade, so no gate is skipped and no unit clears unread" + the validation-model line's "every verdict is yours" |
| Feasibility architect **lead-gated**, fires once, re-fires only on structural change | v0.31.0 *Kept deliberately* + second audit round | Seat row (spawn + peer-edge cells) **and** the ordering invariant naming the three structural triggers and the clarification-only exception |
| The completeness reviewer's **mode-selecting message** is the lead's policy call | v0.31.0 *Kept deliberately* | Validation-model line: you select skill + mode per stage and supply the artifact sets (incremental / cumulative, with both named) |
| **Delivery is not a start signal** — it grades only when you open the pass | v0.31.0 in-wave addition (plan-specific: two-reviewer ordering + mode-selected stage) | Seat row ("grades only when you open the pass") + the ordering invariant |
| Slice binding 1 — a producer designing beyond scope is a scope gap → G6 | v0.15.0 slice-scoped entry, *four genuine bindings kept* | Slice-scope constraint, first binding |
| Slice binding 2 — a `[MODIFY]` graded amendment surfaced for this round's reviews, migration flagged | same | Slice-scope constraint, second binding |
| Slice binding 3 — per-slice outputs → the done-condition's artifact set | same | Bindings' artifact preamble (per-slice layout → the Goal's artifact set) |
| Slice binding 4 — the reviewer briefing sets {this slice + extensions} / {prior accumulated} | same | Slice-scope constraint, third binding |
| Graduation contract is the single home; do not restate | v0.15.0 audit catch (the D1 churn liability) | Slice-scope constraint opens by naming it as the single home for the six rules, and restates none of them — the defect that entry was written about is not reintroduced |
| AD-D1 · AD-D2 — design-time architecture, first design artifact, own early sign-off | DECISIONS rows | Ordering invariant ("the **first** artifact of the design work") + **G3** |
| AD-D3 — delta model, baseline bootstrap, landing fold | DECISIONS row | **G2** (bootstrap + confirm-before-delta) · Bindings' `architecture.md` · the KM-landing binding (baseline → `ARCHITECTURE.md` via the scribe) |
| AD-D4 — artifact contents (C4 diagram, sequence, component table, deployment view) | DECISIONS row | Referenced, never restated: Bindings names `patterns-system-design` as the owner of structure **and** scope bound |
| AD-D5 — always-on, no-delta included | DECISIONS row | G3: "*(always-on)*" + the no-delta presentation with its one-line claim, "the judgment is shown, never silently made by the producer" |
| AD-D7 — `system-architect` × `patterns-system-design`; feasibility gains the architecture pass; structural D-XXX architect-authored | DECISIONS row (`Contested`) | Two seat rows + Bindings' designated structural-decisions section |
| AD-D8 / R5 — rendered-diagram sign-off, plan supervisor presents, degrade-with-record | DECISIONS row | G3, in full — presenter named, render surfaces enumerated, raw-mermaid prohibition, and the fallback now visible (marker retired above) |
| AD-D9 — governance binds the design; conflicts route to amendment/waiver, never overruled at a feature gate | DECISIONS row | **G4**, the two-exit with "the feature gate never overrules the constitution" |
| Team-method D4/D5 — plan absorbs tasks; **one** package acceptance | DECISIONS row | The mapping and tasks stages in the seat table + the mapping-before-tasks ordering invariant; **G7** declared as "the package's **one** standing acceptance". The *provenance* of the merge — which two signatures dissolved into G7 — is **not** in the command: it is history, already single-sourced in this note's v0.32.0 gate-renumber entry. Relocated at the fix round (see the ceiling note below); the ruling is encoded by the file's structure, not by narrating what the file used to be. |
| Vertical-graduation — slice-scoped entry variant | DECISIONS row | The Slice-scope constraint + Bindings' per-slice layout |
| The **`@`-reference recovery** — empty `$ARGUMENTS` has a *named cause* (the `@`-reference drop bug) and a two-option prompt (re-enter, or confirm the detected feature) | record §7 protected set (the `command-altitude` retrofit-regression warning names the `@`-reference recovery among the hard-won fixes verbosity encodes); still carried by `implement.md` | **G1** decides-clause. **Restored at the pilot fix round** — the first draft compressed G1's evidence list and lost both the cause and the prompt, leaving "empty is resolved at G1" with no recovery behavior. Exactly the retrofit-regression class §7 warned about, caught by the audit's check 14. |
| All three of the exemplar's **gap-routing classes** — knowledge → `Explore` / the research branch · **preference → G5** · scope → G6 | record D5 fold (a) graded exemplar, line 1; `loop-discipline` gap routing | **G5** names the preference class and the knowledge branch and points scope at G6; **G6**'s evidence carries the scope trigger. **Preference restored at the pilot fix round** — the first draft carried knowledge and scope but dropped preference, so the exemplar's own preservation standard was not met on the line it was drawn from. |
| The `quickstart.md` **null path recorded** in `plan.md` | current body (conditional artifact) | Bindings (conditional + null path) **and** the Goal's not-done states |
| `plan.md` is a summary over validated artifacts, **never new design** | current body | Bindings, on the `plan.md` entry |
| Round reports cleaned by default; never offer to delete a deliverable | current body | Bindings' Reports entry |

## [v0.32.0] Build note + shape-v4 re-conform — merged design-room command: absorbs `/mochiko:tasks` + gains the architecture stage (2026-07-30)

Design records: `.mochiko/brainstorms/team-method-vs-command-shape/record.md` (D4/D5 — plan absorbs
tasks) + `.mochiko/brainstorms/architecture-design-primitive/record.md` (AD-D1–D9 with folds R1–R10,
seam notes N1–N3). Not a strip wave — a feature build; the architecture-stage **additions** are recorded
in the `DECISIONS.md` rows AD-D1–D9 (lead-owned landing), not here (Job-4 rule: pure additions ride the
decision row, the v3 run-cost precedent). This note logs the version stamp, the **relocation** (tasks'
structuring loop moved *into* plan), the consequent cross-reference change, and the **shape-v4
re-conform** the merge required. Overall command surface 7 → 6 — see the tasks retirement note
(`strips/tasks.md` v0.32.0).

> **Version note:** this build was originally stamped **v0.30.0**; while it was in flight, origin/main
> released **v0.30.0** and **v0.31.0** (the shape-v3→v4 mesh rewrite + the six-command re-conform,
> below). The merge rebased this build onto v4, so it lands at **v0.32.0** and is re-stamped throughout.

- **Relocation IN (from `commands/tasks.md`, now retired):** the entire Mapping → Tasks structuring loop
  — the standing `task-architect` (`patterns-vertical-tdd`) producer seat, the `devils-advocate`
  (`review-task-artifacts`) reviewer in its early-mapping-then-cumulative modes, the two-sub-stage round
  loop, and the task-artifact deliverables (`task-mapping.md` · `tasks.md`) — relocated into plan's
  **Phase 4**. tasks' standalone `tasks.md`-acceptance gate (its G5) **dissolves** into plan's single
  final **package acceptance (G7)** per team-method D5 (the standalone signature was load-bearing only
  while a command boundary sat there). The `review-task-artifacts` validator is **unchanged** in
  structure — same agent, same skill, same checklists; only its caller moved. The completeness reviewer
  is now **one standing `devils-advocate` seat** that runs `review-plan-artifacts` across the design
  stages and `review-task-artifacts` across structuring (the skill is named per dispatch, never loaded as
  frontmatter — shape Layer 2), rather than two separately-spawned reviewers across two commands.
- **Addition — the architecture stage (AD-D1–D9; recorded in DECISIONS, summarized here for the trail):**
  a new **Phase 2** between Analysis and Detailed design, authored by a **new standing `system-architect`
  seat** (`mochiko:patterns-system-design`) — the delta `architecture.md` artifact + the structural D-XXX
  rows into `constraints-and-decisions.md`, its own **early sign-off gate (G3)** presenting the *rendered*
  diagram (degrade-with-record fallback, D8/R5, marked as a shape-exception in the command), always-on
  incl. the no-delta form (D5), and a bootstrap **baseline-confirmation gate (G2)** when no
  `ARCHITECTURE.md` exists (R6a). The `principal-architect` feasibility seat gains an **architecture pass**
  (topology feasibility + governance conformance) — the carve-out of its former "never grades past Phase 1"
  bar (R1, named build work); `review-plan-artifacts` gains architecture-coverage + conforms-to-architecture
  checks (referenced, the skill owns them). Detailed design (former Phase 2, now **Phase 3**) must conform
  to the approved architecture; a contradiction found in authoring **returns to G3** for a consented target
  amendment (R2).
- **Gate renumber (consequent):** the architecture gates insert early, so plan's gates renumber —
  G1 (entry) · **G2** baseline-confirm (bootstrap) · **G3** architecture sign-off · G4 feasibility/governance
  rejection (was G2, now also carrying the governance two-exit, D9.3) · G5 clarification (was G3) · G6
  exit-early (was G4) · **G7** final package acceptance (was plan's G5 *and* tasks' G5, merged). Note the
  renumber against main's v0.31.0 entry below: that entry conformed the *two-phase* plan, where
  feasibility-rejection was **G2**; in the merged command it is **G4** (its "G2" references are frozen
  two-phase history).
- **Shape-v4 re-conform (the merge work, this task):** the merged command was re-authored against
  `command-shape.md` **v4** (main's v0.31.0 bumped it from v3). The v4 idiom adopted: (a) the **in-loop
  mesh** — each producer is **peer-edged with the completeness reviewer**, handing finished artifacts
  directly (peer-routable delivery), while **delivery is not a start signal** (the lead opens every round
  and every review pass); (b) the **feasibility architect stays lead-gated** — fired selectively, its
  concerns routed through the lead at **G4** (not peer-edged, matching main's v0.31.0 narrowing on the
  two-phase plan); (c) the roster **names each seat's peer edges** per the v4 seat-roster PARAM; (d) the
  Contract states **"No devolved branch"** — every plan review is a judgment grade (feasibility,
  completeness, architecture coverage, task-artifact quality), never all-deterministic-CLI, so no gate is
  skipped and every verdict is the lead's; (e) "no producer↔reviewer contact" is dropped from the Contract
  (independence now rides disjoint agents/skills + cold *arrival*, not routing). The architecture stage's
  own peer edge: `system-architect` is peer-edged with the completeness reviewer for the coverage grade;
  the architecture *feasibility pass* is lead-gated like the analysis pass.
- **Cross-reference change:** Phase 5's next-step pointer `→ /mochiko:tasks` is superseded; the merged
  command produces the whole package and points `→ /mochiko:implement`. `templates/plan-template.md` gained
  an **Architecture** section (pointers to `architecture.md`, per the summary-not-restatement rule) and now
  lists `architecture.md` / `task-mapping.md` / `tasks.md` in its Artifacts manifest.
- **Producer report added:** `templates/sysarchitect-report-template.md` — the `system-architect`'s
  self-disclosure carrier (report: disclosure, per `report-format.md`), parallel to the techanalyst /
  taskarchitect report templates.
- **Conversion re-assessment:** the merge does not re-open the team-form ruling — all three producers
  (technical-analyst, system-architect, task-architect) and both reviewers stay standing/cold seats per the
  existing conversion assessments below and tasks' (retired) assessment. **S8 home-revision checkpoint
  re-checked:** the merged command is a larger team (3 producers + 2 reviewers, 7 gates) but rides the
  existing shape — Layer 1 as-you-go artifact + producer-authored uncertainty branch, Layer 2 mesh
  peer-edges + independence-by-cold-arrival — with **no new shape gap** (the rendered-diagram gate is a
  per-workflow gate, not shape doctrine; marked shape-exception where a line would otherwise restate
  shape). Shape stays **v4** (this build conforms to it, does not revise it). The first-dogfood
  confirm-or-revert checkpoint carries forward: the open "Dogfood `/mochiko:plan`" item now exercises the
  merged, architecture-first command.

## [v0.31.0] Lead-relayed gap lists superseded by the in-loop mesh (shape v4 conforming edit)
- **Disposition:** superseded → `templates/command-shape.md` v4 (Layer 2 — "Independence by structure" + "In-loop mesh"). Rewritten in place: both reviewers are still cold-spawned at their own stage (a spawn-timing parameter), and the producer↔reviewer peer edges are declared on the roster.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/team-method-vs-command-shape/record.md` **D1**, scoped by **D2**), not a minimalism strip. Permanent no-contact was the falsified claim; cold *arrival* survives as a property of the stage.
- **Content (superseded, verbatim):**
  - producer seat: "Round > 1 within a phase is a message to the same seat carrying the reviewers' gap list verbatim"
  - feasibility reviewer: "spawned **cold after the Phase-1 analysis is authored**, never in contact with the producer"
  - completeness reviewer: "spawned **cold at the first completeness review**, never in contact with the producer"
  - Phase 1 step 1: "on round > 1 the message carries the reviewers' gap list for targeted revision"
  - Contract, Producer ↔ validator: "(both reviewers cold-spawned, gap lists lead-routed, no producer↔reviewer contact)"
- **Kept deliberately (not superseded):** every verdict stays the lead's — plan has no deterministic-CLI verification, so **D3's devolved branch cannot apply here**; the Contract now declares that absence rather than leaving it implicit. Also kept lead-gated: the **feasibility architect's engagement** (fired once, re-fired only on a structural change) and **Phase 2's mode-selecting message** — both policy calls under the traffic classes, not hand-offs.
- **In-wave correction (audit round, 2026-07-30):** the peer edge this wave first wrote was **blanket** — "**Peer-edged with both reviewers:** it hands each round's finished artifacts straight to them" — and its Phase 1 step 1 counterpart "handing them to the reviewers directly when the round's set is complete". The audit caught that this silently peer-routed the *architect*, whose fire-once/re-fire-on-structural-change engagement is a lead-gated policy call, and that it read as licensing a completeness pass before the feasibility gate. Narrowed to the **completeness reviewer only**, with the lead sequencing when it grades. Logged as an in-wave correction, not a separate version: the superseded text never shipped outside this wave.
  - **Second audit round (same wave):** the narrowing was applied to the roster bullet and Phase 1 step 1 but **not propagated**, leaving three sites still asserting the blanket edge. Substance was upheld; only propagation failed. Also superseded, same correction: the feasibility-reviewer bullet's "peer-edged with the producer thereafter" → "**lead-gated thereafter** — you fire it, and its concerns reach the producer through you (G2)"; and the Contract's "gap-list hand-offs peer-routed producer↔reviewer per the shape's mesh, with every verdict yours" → the completeness list peer-routed, "the architect's routes through you at G2". Added in the same pass (not a supersession): the completeness reviewer's **verifying-side hold** — "Delivery is not a start signal — it grades only when you open the pass (Phase 1: after the architect; Phase 2: on your mode-selecting message)". The producer-side hold is universal and lives in the shape home; this one is plan-specific — two-reviewer ordering plus a mode-selected Phase 2 — so it binds at the seat and makes Phase 1 step 1's "you sequence when it grades" a reference to a bound rule rather than a bare assertion.

## [v0.15.0] Conversion note (D2/S4 — one-shot → team-form, 2026-07-19)

- **Command-specific rationale (user-ratified):** plan runs a producer↔two-reviewer cycle (≤3 rounds
  per phase, gap-list-driven revision, cold reviewers) across **two phases** (Analysis → Design) whose
  context-retention bet is plan's own — the longest horizon of any converted command: a **standing
  producer seat** holds (1) the Phase-1 analysis rationale carried into the Phase-2 design across six
  artifacts (why a decision beat its alternatives, which constraint shaped it, what NFR targets bind —
  authored from lived context, not reconstructed from files), and (2) the C↔D dependency web so a
  targeted revision after a feasibility rejection stays coherent. The two reviewers map to: a
  **standing completeness advocate** (`devils-advocate`, cold at first spawn, spans both phases — its
  retained Phase-1 context is what makes the Phase-2 incremental consistency check a spot-check, not a
  full re-read) and a **cold fire-once feasibility architect** (`principal-architect`, grades once
  post-Phase-1-produce, re-fires only on a structural change, never grades Phase 2). Neither reviewer
  contacts the producer — independence stays structural. Transport rides the v3 fix
  (`agent-dispatch.md` Seat transport + addressability probe on the producer's first spawn).
- **Steelman recorded (user-ratified with the conversion):** zero successful team-form runs at
  conversion time (two setup defect runs; specify's + slice's own checkpoints unfired; brainstorm v2
  measured standing seats *more* expensive than dispatches). Plan is the **most expensive command to
  run as a standing team** — three seats across two phases vs specify/slice's two — so it pays the
  largest team-form tax if the retention payoff doesn't land. The **fire-once architect is the weakest
  team-form fit**: it usually fires once and sits dormant, getting little from persistence (modeled as
  a standing seat messaged sparsely — uniform transport, and it keeps its Phase-1 read on a
  structural-change re-fire; the honest steelman is that the architect alone would be fine as a
  bounded one-shot subagent). And the design artifacts **reconstruct relatively cheaply from disk** —
  the six artifacts are richly ID'd and the FR→TR→entity→schema traceability is written *in the files*,
  so the retention payoff, while real, is smaller than "six artifacts / two phases" suggests. Ruled
  team-form anyway per D2's declared default + S4 (no prior dogfood evidence required; checkpoint
  below).
- **Confirm-or-revert checkpoint:** the first post-conversion dogfood run (the open "Dogfood
  `/mochiko:plan`" BACKLOG item, Plan-port follow-ups) confirms the conversion or reverts it to
  one-shot Layer-1 form; a revert is logged as a `RETURNED:` entry here. Team-form named checks: the
  producer probe fires the addressability check; the standing producer seat is messaged (not
  respawned) across rounds and across the phase boundary; the completeness advocate spawns cold and is
  messaged in Phase 2 for incremental mode; the feasibility architect fires once and re-fires only on
  a structural change; neither reviewer contacts the producer.

## [v0.15.0] Sound-loop paragraph + four-requirement enumeration
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, One lead) + the
  `mochiko:loop-discipline` reference
- **Tier failed:** 1
- **Content:** "This is a mochiko **sound loop**: invoke **`mochiko:loop-discipline`** and honor all
  four requirements (default-FAIL done-condition, independent validation, bounded iteration, named
  human gates), and brief each dispatch per **`agent-dispatch`**. Those rules are not restated here…"
  — restated loop-discipline's own enumeration.

## [v0.15.0] Per-run contract fill (`workflow-contract.md` → `plan-contract.md`)
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Contract — the
  authoring-time-fill rule); the per-workflow values survive as the command's authoring-time Contract
  section (plan's are richer — a per-phase round cap and five gates)
- **Tier failed:** 1 (the shape retired per-run fills whose values are constant at authoring time)
- **Content:** "## Contract parameters (fill the artifact — don't inline it) … Fill
  `templates/workflow-contract.md` → `.mochiko/specs/<feature>/plan-contract.md` with the values
  below, then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable
  proof — not this command body."

## [v0.15.0] Verdict-ownership triplication
- **Disposition:** deduped to once (the Contract's Done-condition / Producer↔validator clause; the
  `review-*` family boundary also lives in `review-feasibility` + `review-plan-artifacts` descriptions
  + REGISTRY). The per-phase Verdict *steps* (Phase 1 step 4, Phase 2 step 3) are workflow mechanics
  and survive.
- **Tier failed:** 1
- **Content:** stated at the lead framing ("Each reviewer *recommends* a status; **you own the
  clearing verdict** — their status is input, never the gate") and again in the footer ("the verdict
  (each reviewer grades from the files, you Read the artifacts and decide against the default-FAIL
  done-condition — their status is input)").

## [v0.15.0] Footer ground rules + Task-tool transport line
- **Disposition:** kernel-free/git relocated → `templates/command-shape.md` (Layer 1, Ground rules);
  the Task-tool line superseded by the team-form conversion (transport now per shape Layer 2 +
  `agent-dispatch.md` Seat transport)
- **Tier failed:** 1
- **Content:** "Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool
  (never inline agent behavior); do not modify git or push."

## [v0.15.0] Recovery memory-model parenthetical
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Recovery — "never a context
  `phase` field")
- **Tier failed:** 1
- **Content:** "Resume from workspace evidence (there is no context-file `phase`/`status`):" + the
  entry-gate parenthetical "(workspace evidence — there is no context-file `status` to read)". The
  recovery table (evidence → resume-at) is the workflow-specific Recovery PARAM and survives.

## [v0.15.0] "Why this done-condition differs from HIL's" blockquote
- **Disposition:** deleted (user-ratified)
- **Tier failed:** 2 (no behavior produced — historical/motivational provenance; preserved in
  ROADMAP's Decision Trail + `.mochiko/transform/plan/`)
- **Content:** "> Why this done-condition differs from HIL's: HIL declared "no hard caps" and routed on
  each agent's verdict *field* — it could self-declare done at pass 1, violating `loop-discipline`
  reqs 1 & 3. The two reviewers' three-state statuses survive only as input to your verdict; the
  deterministic cap and the new G5 acceptance gate close the gates HIL lacked." — the shape of
  specify's deleted HIL-comparison blockquote; its rationale is carried by the Contract done-condition
  + `review-feasibility`'s "Preserve `infeasible` as a distinct state" doctrine, so no unique behavior
  is lost.

## [v0.15.0] Slice-scoped entry — restated Graduation-contract rules (audit catch)
- **Disposition:** relocated → `templates/slices-template.md` (the **Graduation contract** section —
  the single home of the consumption rules); Phase 0 step 5 now *applies* the contract by reference
  for slice resolution, the staleness guard, scope, extend-mode, graded amendment, and artifact layout
- **Tier failed:** 1 (the step declared "the single source … do not restate it" and then restated most
  of it — the D1 churn liability)
- **Content:** the copied rules — slice resolution ("named in `$ARGUMENTS`, else the first slice in
  Slice-order lacking `slices/<slice>/plan.md`"), the staleness guard ("the live `spec.md` story-ID
  set must match the Spec stamp — mismatch → block and point to `/mochiko:slice`"), extend-mode ("the
  shared feature-root artifacts are brownfield input the producer extends in place — never re-derives,
  never forks per-slice copies"), and the graded-amendment definition ("a **breaking** change … is a
  graded amendment … never a silent rewrite"). The four genuine plan bindings were **kept**: G4 on
  over-scope, the `[MODIFY]`-surfaced-for-this-round's-reviews behavior, the per-slice-output →
  done-condition mapping, and the reviewer briefing sets.
- **Note:** caught by the `validation-command-shape` audit — the assessment had passed this entry as
  at-altitude on its "do not restate it" self-declaration; the audit found the restatement beneath it.
  Fixed in-wave, no version bump. The Graduation contract is on the ≥3-consumer queue (plan/tasks/
  implement slice-scoped variants) — this strip relocates plan's *local restatement* to the contract
  home; it does not rule the shared contract.
