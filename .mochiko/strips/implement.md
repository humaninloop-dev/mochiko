# Strip notes — `commands/implement.md`

Entry formats: `strips/README.md`. Wave context: the implement cluster wave (BACKLOG item 7, the
**fifth and final** one-shot-command wave after specify's v0.13.0, slice's v0.14.0, plan's v0.15.0,
and tasks' v0.16.0). The wave also ran the **D2 conversion assessment** (one-shot → team-form) and
re-checked the **S8 home-revision checkpoint** against implement's needs (a standing producer spanning
the whole cycle sequence + the fix-pass loop, a standing verifier fired once per cycle + a
whole-implementation final validation, and a per-cycle confidence gate that auto-approves
deterministic-CLI-pass cycles — **no new shape gap at that wave, when the shape was v2**, so it made
no template revision and no cross-command re-audit). **Stale as a standing claim:** the shape is now
**v4** (2026-07-30), and its D3 devolution changed exactly that confidence gate — see the v0.31.0
entries below. **Also stale:** the shape is **v5** as of the v0.35.0 wave below, and the
"standing producer / standing verifier" claim is superseded by that wave's seat-recycling binding.

---

## [v0.61.0] Spec-folder package gate + unconditional graduation landing superseded by feature-keyed entry + scope-branched landing with graded folds
- **Disposition:** superseded → the feature-keyed rewrite in the same file: the canonical entry gate (selection scope additionally gating on the accepted package resolved from `.mochiko/features/FEAT-XXX/`; delta scope gating on a delta card confirmed by a delta-scope plan run), product baselines at `.mochiko/product/` + this feature's deltas as design inputs, reports in `.mochiko/features/FEAT-XXX/` (product-lane runs: `.mochiko/product/lane-<slug>/`), the scope-branched Acceptance-landing binding (selection scope = the graduation batch, surviving verbatim; delta scope = the delta fold; every touched baseline folded via the graded fold — three-way diff — checked by the landing verification seat, scope-extended; lane runs add the map-delta boundary check to the same seat, no new seat), the Baseline-touches binding (`baseline-delta.md` authored at discovery; product-lane abort-and-reroute)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-10 "Land feature-sizing & entry-points rulings (D1–D15)"; record `.mochiko/brainstorms/feature-sizing-and-entry-points/record.md`, D7 (the lane map-delta boundary check joins the landing verification seat) / D8-as-amended / D9 / D14 (findings 6 + 7: dispatched-run `baseline-delta.md` at discovery, product-lane files-and-aborts) / D15 (graded folds, appliable delta form) — D9's supersession note retires feature-map D17/D18 (artifact layout inside the spec folder, extend-mode home, cross-spec reach), feature-map D10's plan-artifacts-in-spec-folder clause, and feature-map D19's read mechanics)
- **Content:** header "(cycle cards, under `features/FEAT-XXX/` in the spec folder)" / "empty → resolve the next planned undelivered feature from `.mochiko/specs/` and confirm with the user." · Goal "the acceptance landing's map bookkeeping executed whole" (extended to "— map bookkeeping and every touched baseline's graded fold") · Entry bullet "the accepted package gates the run — the feature's `tasks.md` complete alongside its `plan.md` and `architecture.md` under `features/FEAT-XXX/`; missing or incomplete → block, point to `/mochiko:plan`. A selected feature ordered earlier and not yet `delivered` blocks — one run per feature, in dependency order." · Design-inputs clause "under `features/FEAT-XXX/`, the spec-root `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`, `requirements.md`, `nfrs.md`" · Reports "under `features/FEAT-XXX/` in the spec folder" · Acceptance-landing binding head "**Acceptance landing — map bookkeeping:** at user acceptance, the same landing that folds `ARCHITECTURE.md` executes the map's graduation batch" (the unconditional form — the graduation-batch text itself survives whole under the selection-scope branch)
- **Kept deliberately:** the full graduation batch verbatim under selection scope (status flip dated · marked deltas fold into extent lines · `FEATURES.md` index touch · `ARCHITECTURE.md` In-flight pointer clear · derived specs-index closure · no separate feature-close stage) · TDD cycle discipline and every craft-skill binding (`mochiko:executing-tdd-cycle` with cycle-report as uncertainty carrier, `mochiko:brownfield-integration`, `mochiko:testing-end-user`, the `mochiko:review-code-minimalism` lens with advisory-only `minimalism:` findings) · Bounds/attempt machinery, no-progress stop, graded-head rule, delta-grade scoping, cold-snapshot final validation with the worktrees ignore entry · verification seats, evidence rules, real-infrastructure gates · regression scope (accumulated TEST gates + later-landing seam ownership) · architecture-deviation consent + built-vs-approved diff · escalation cadence · KM-landing dual-target `ARCHITECTURE.md` fold · human approval gates (plain blocking acceptance, never timed) · no-git-mutation lines. Pure additions riding the decision row: the graded-fold not-done state, the repeat-run append rule, the Baseline-touches binding.
- **Consumers assessed:** none mount commands — entry points. Ripple assessed: `plan` (same cluster, same stamp) · the new feature command (dispatches lane work here; the landing verification seat's extended scope lives in this file, referenced not restated there) · `specify`/`setup` (parallel seat this wave) · the router skill (unaffected) · `mochiko:authoring-feature-map` (carries the graduation-batch and delta-fold grammar; the parallel map builder owns it this wave).

## [v0.58.0] Slice scope superseded by per-feature runs; acceptance landing absorbs map bookkeeping
- **Disposition:** superseded → the feature-scoped rewrite in the same file: one run per feature (`$ARGUMENTS` = FEAT-XXX, package under `features/FEAT-XXX/`), the Regression-scope binding (accumulated delivered-feature TEST gates + later-landing-feature seam ownership), the Acceptance-landing map-bookkeeping binding (status flip · delta fold · `FEATURES.md` index touch · In-flight pointer clear · derived specs-index closure), machinery per `mochiko:authoring-feature-map`
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-10 "Feature-map layer ruled (D1–D22)"; record `.mochiko/brainstorms/feature-map-layer/record.md`, D13/D17/D18; D13 dissolves feature-close into this landing)
- **Content:** header Goal line "`$ARGUMENTS` = optional feature ID; empty → resolve from `.mochiko/specs/` and confirm with the user." · Reports binding "under `.mochiko/specs/<feature>/` — or `slices/<slice>/` when slice-scoped" · the whole **Slice scope** binding: "**Slice scope** (the spec's Delivery Slices section holds a decomposition; its Graduation contract governs): the run reads `slices/<slice>/tasks.md`; quality gates still run the full repository suite; when the last slice clears, the feature is declared, not verified — Feature-Done executes at feature-close, surfaced as the next step, never reported complete here." · KM-landing clause "— slice-scoped, the fold is dual-target (feature-root `architecture.md` accumulates the approved delta) … at feature close the In-flight pointer is removed."
- **Kept deliberately:** v0.56.0 Bounds + Escalation-cadence bullets verbatim, all snapshot-isolation machinery (graded-head rule, delta-grade scoping, cold-snapshot final validation) untouched · TDD + real-infrastructure verification with evidence and exit codes · architecture-deviation consent + built-vs-approved diff · the `ARCHITECTURE.md` fold itself (still dual-target — the second target re-keyed feature-root → `features/FEAT-XXX/architecture.md`) · full-repository quality-gate suite (was the slice binding's live half, now in Regression scope) · the In-flight pointer removal (relocated from KM landing into the acceptance-landing batch, same landing per D13) · no-git-mutation + plain-blocking-acceptance lines. Feature-Done's obligations dissolve into this landing per D13 — per-feature SC coverage is the TEST-gate verification; cross-feature seams are owned here by the later-landing feature.
- **Consumers assessed:** none — commands are entry points, nothing mounts them. Ripple noted: plan (same wave) · authoring-feature-map (carries the landing batch grammar, wave 1) · testing-end-user (gate parsing unchanged).

## [v0.49.0] Task-checkbox progress superseded by per-card checkboxes; builder decomposes
- **Disposition:** superseded → Goal "every `tasks.md` cycle card is `[x]`" + decompose-at-build clause; Bindings deliverable/inputs/slice-scope re-keys
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D2+D2.1+D6)
- **Content:** Goal "Every `tasks.md` task is `[x]`" · Not-done "an unchecked task" · Bindings "`tasks.md`'s checkboxes (`T{N}.{X}` namespace) are the progress surface, flipped as tasks complete" · `task-mapping.md` in Design inputs · Slice-scope key "(accepted `slices.md` present)".
- **Kept deliberately:** TDD-built + real-infrastructure verification, cold-snapshot final validation, architecture-deviation consent, KM landing, all acceptance gates — untouched. `spec.md` added to Design inputs (cards cite acceptance-criteria IDs the builder resolves).
- **Consumers assessed:** executing-tdd-cycle + testing-end-user (both re-keyed same wave) · router · ARCHITECTURE.md implement section.

## [v0.48.0] Shape v8 goal+harness rewrite — choreography dies in place
- **Disposition:** superseded → the v8 goal+harness rewrite of this command (whole-file)
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/command-architecture-realignment/record.md` D1–D6; DECISIONS.md 2026-08-02 command-architecture row)
- **Content:** the entire v7-form file superseded — preamble dispatch-brief protocol · Seats & checks table + validation model · team-transport mandate + roster probe (D5: transport-neutral now) · seat lifecycle/recycling · every G-numbered gate, the run-start weight card, floor-gate set, counted bounds/caps/kill-switch, ordering invariants, ground-rules block · run-start declaration + departure trail + per-run contract file · KM-landing command steps · the Recovery section and resume table. Verbatim text below (pre-edit file at the v0.47.0 tree).
- **Kept deliberately:** the Goal's all-tasks-[x], TDD, per-cycle + whole-run real-infrastructure verification with evidence and exit codes, traceability/governance alignment, built-vs-approved diff condition · implementation-verification seat split (Independence line) · diagram-anchored deviation check with user consent, never silently built · package entry gate · governance obligated-read brief line · craft-skill bindings (executing-tdd-cycle with cycle-report as uncertainty carrier, brownfield-integration, testing-end-user) · design-inputs list · cold dependency-cold snapshot verification with the worktrees ignore entry (validator snapshot-isolation intent preserved) · slice full-suite + feature-declared-not-verified rules · KM ARCHITECTURE.md fold · no-git-mutation + plain-blocking-acceptance lines · output-style register pointer
- **Consumers assessed:** none — commands are entry points, nothing mounts them.

<details><summary>Verbatim superseded file (v0.47.0)</summary>

````markdown
---
description: Execute an accepted task breakdown into working, verified code via an independent producer→verifier team loop — a staff-engineer seat implements each cycle through red/green/refactor TDD (foundation cycles before feature cycles) and fix-passes the final validation; a qa-engineer seat verifies every cycle and then the whole implementation against real infrastructure with captured evidence and quality-gate exit codes; the approved architecture is briefed input, guarded by a diagram-anchored deviation self-check at cycle open and close and by a built-vs-approved diff at final validation. A per-cycle checkpoint carries a deterministic-clean devolved branch; a named final-acceptance gate closes the run. Package-gated, cycle-by-cycle, default-FAIL, bounded, kernel-free. Requires agent teams (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS); refuses without them.
disable-model-invocation: true
---

# Implement — Execute the Task Breakdown (Cycle-by-Cycle, Foundation → Feature)

**Goal:** turn an accepted `tasks.md` into working, verified code — one cycle at a time, foundation
cycles before feature cycles, each implemented through red/green/refactor TDD and independently
verified against real infrastructure, until every cycle clears and a whole-implementation final
validation passes. `$ARGUMENTS` = optional feature ID or description; empty or
detected-from-workspace is resolved at G1.

**You are the lead**: you compose the run and own its counters, every verdict, every escalation,
every human gate, and the user-facing conversation — agents produce and review, you adjudicate; the
one exception is the cycle checkpoint's devolved clean branch below. Every dispatch carries its
own brief in the spawn or send prompt — the seat's role and skill (named as a hint, the agent
decides fit), the exact inputs to Read, where the output lands (write vs return), the bar it
must clear, its peer edges and holds, and the independence reminder that matches the seat
(author: never grade your own output; grader: read the artifact itself, default FAIL, quote
evidence) — the seat owns none of this context and gets all of it from you; on a retry, a
peer-routed gap list is pointed at and the round opened, a relayed one pasted verbatim.
This file is self-contained: implement's whole contract lives here.
**First-spawn probe:** the `staff-engineer` producer — foundation cycle 1 is
implemented before anything verifies it.

## Goal

Every `tasks.md` task is `[x]` with its `cycle-report.md`; a verification report per cycle and one for
the whole implementation, each naming its real-infrastructure evidence tree and its quality-gate exit
codes; the built code meets its criteria, holds traceability to requirements, and aligns with the
project's governance; where an approved structural delta existed, its built-vs-approved diff report
exists and any divergence it names was ruled at G5; the KM landing ran; and the user accepted at G5.

**Not done:** an unchecked task, or a cycle with no report · a failing quality gate · a cycle or the
final validation with no real-infrastructure evidence · a warm-only final validation · a surfaced
architecture deviation neither built as approved nor consented as an amendment · an approved delta
with no built-vs-approved diff report · a departure with no trail line · out of rounds · G5
unaccepted.

## Seats & checks

| seat | agent × skill(s) | produces / grades | spawn | peer edges |
|---|---|---|---|---|
| producer | `staff-engineer` × `executing-tdd-cycle`, `brownfield-integration` | implements each cycle through red/green/refactor TDD → `cycle-report.md`; targeted retry of only the failed tasks; the final-validation fix pass, unconstrained by cycle boundaries; reports its architecture deviation self-check; never verifies | standing across the cycle sequence and the fix-pass loop; **probe seat**, foundation cycle 1 | hands each completed cycle straight to the verifier; retries and fix passes are dispatched by you |
| verifier | `qa-engineer` × `testing-end-user` | verifies each cycle, then the whole implementation, against real infrastructure — executes the cycle's `**TEST:**` tasks, runs the quality gates, captures evidence → a verification report naming its evidence tree + a checkpoint recommendation; never implements | cold at the first cycle verification, standing after | peer-edged with the producer for cycle hand-offs; the endgame is lead-routed |
| arch-diff | `principal-architect` × `authoring-architecture`, diff mode | reports built vs. approved — "built as approved", or the divergence | disposable, at final validation, whenever an approved structural delta existed | none — never the verifier seat |
| arch-scribe | `principal-architect` × `authoring-architecture` | folds the resulting system into `ARCHITECTURE.md` | disposable, at finalize, on a built structural change only, per the KM landing | none |

**Validation model:** the loop's bounded in-loop critique — qa's per-cycle verification plus the
final validation, unsized by design. The verification skill is **never** mounted on the producer.
Outside the devolved branch qa's output is **lead-adjudicated input** and the verdict is yours. One
verifier, so implement numbers no G2 — there is no feasibility-rejection gate. No seat ever grades
its own output.

**Team transport:** check `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS` before anything else — unset →
stop and tell the user how to enable it (settings/env; Claude Code ≥ v2.1.178); the first spawn is
the authoritative probe, and there is no teamless fallback. A seat is spawned with **`name:`** — a
nameless spawn is a one-shot subagent, the forbidden transport; every later round is a
`SendMessage` to that same named seat. Verify from the roster: the `members` array in
`~/.claude/teams/<team>/config.json` (`<team>` = `session-` + first eight chars of the session ID)
must carry the seat's `name` — absent ⇒ kill and respawn explicitly requesting an agent team;
failing again stops the run. Teammates don't load `skills:` frontmatter — every spawn prompt names
the skill and role itself. Tell the user up front they can watch or message any teammate; announce
each seat in one line when filled; never narrate or reply to teammate housekeeping. A peer-routed
hand-off is **not a start signal** — the producer revises only when you open the next round, and
your brief carries that hold.

**Seat lifecycle:** the counted unit is the **cycle**; at each gate pause count each standing
seat's completed cycles and recycle at ~≥3 — counted, never observed; the user may order a recycle
at any gate. **Override —** the verifier recycles per
**slice** boundary, its final-validation incarnation additionally briefed from the on-disk
verification reports. A retry or fix-pass respawn carries the failed-task list **and** the
just-failed `cycle-report.md`, relayed at dispatch: the next attempt overwrites that file. A
respawn is a reset: briefed from the on-disk artifact set alone, versioned successor name
(`producer-2`), never the dead seat's bare name. End-of-need shutdown; no ritual sends.

## Constraints

- **G1 entry** — evidence: `$ARGUMENTS` · rules: the user · decides: the resolved `<feature>` (an
  explicit ID, else the most recent in-progress feature under `.mochiko/specs/`, confirmed with the
  user before the run opens).
- **Package gate** — evidence: `tasks.md` present and complete alongside the accepted `plan.md` and
  `architecture.md`, plus the design inputs and `slices.md` (Bindings) · rules: the user · decides:
  whether the run opens. Missing or incomplete → block, pointing the user to `/mochiko:plan`.
- **Run-start weight card** — evidence: your stated read of the four rigor factors against this
  breakdown — **reversibility** (rework cost if the build is wrong) · **blast radius** (how much
  downstream work reads the built code as authoritative) · **precedent** (first-of-kind, or
  mirroring an audit-cleared pattern) · **input confidence** (scored on the artifact under review;
  a user ruling discounts ambiguity risk only, and one introducing new surface raises consistency
  risk) — plus the process you compose from it — the stated default below, or your departures from
  it · rules: the user · decides: the run's composed process. Rigor scales with
  cost-of-being-wrong, never task size; diff size is at most a hint.
- **Governance surface** — evidence: `CLAUDE.md`'s `<!-- mochiko:governance:begin -->` region ·
  rules: the user, when it is absent · decides: proceed on governing context, or run
  `/mochiko:setup` first. Absence is **surfaced, never auto-resolved** — governing context, not a
  blocking gate. Present → each code-touching brief carries the one-line obligated read naming the
  `.claude/rules/mochiko/` files relevant to that cycle's file paths.
- **Cycle checkpoint** — evidence: `cycle-report.md` (deviation self-check, `domain_deps_added`), the
  verification report, qa's classified evidence + recommendation · rules: you, except on the devolved
  branch · decides: the cycle advances, or a targeted retry. It carries this command's **devolved
  branch**, skipped **exactly** when every verification in the cycle is a deterministic CLI check at
  100% pass **and** no deviation is reported **and** `domain_deps_added` is empty **and** both
  reports are clean by the envelope's prose check (`report-format.md`): the cycle then
  clears on qa's PASS-with-evidence, unread by you, counted from its one-line clearance notice.
  Otherwise it fires — any failure, any GUI or subjective verification, any reported deviation, any
  registry addition, any prose on a passing report — and you rule on the reports.
- **Architecture deviation** — evidence: the producer's diagram-anchored self-check, run at cycle
  open **and** cycle close — does this cycle add or remove a box, add, remove or redirect an arrow,
  or move a responsibility across a boundary on the approved diagram? · rules: the user · decides:
  build as approved, or a consented amendment of `architecture.md` before the cycle resumes. A yes
  stops the cycle and you present it — never silently built.
- **G3 clarification** — evidence: an ambiguity or blocker the producer flags · rules: the user ·
  decides: the answer fed forward into the next dispatch, logged in-session. You route each
  finding by judgment: **a genuine judgment call is ruled here**; a gap answerable by
  investigation routes to a native `Explore` pass (the "Research this" branch), never to the
  user; work bigger than the run was framed is G4's.
- **G4 exit-early / escalation** — evidence: a cap trip, a failing set unchanged round-over-round,
  `IMPLEMENT_STOP`, or a scope gap · rules: the user, on the last evidence · decides:
  continue-refining / accept-with-noted-gaps / stop-and-review — the run stays FAIL unless the user
  explicitly accepts. Neither G3 nor G4 ends the loop on its own.
- **G5 final acceptance** — evidence: your clearing verdict on the final validation — the cycle /
  task / fix-pass counts, quality-gate results, an evidence summary, any noted gaps — **and** the
  built-vs-approved architecture result where an approved delta existed · rules: the user · decides:
  accept (done) / amend (the changes become the failure list; re-enter the relevant cycle or fix
  pass, still bounded, and clear a verdict again) / reject (the work remains under
  `.mochiko/specs/<feature>/` and in the working tree).
- **Floor gates:** the run-start weight card · the package gate · the governance surface's absence
  ruling · **G3**'s judgment-call ruling · **G4** · **G5** · the architecture-deviation consent — the
  user's whatever you compose, never departable. G1 and the cycle checkpoint (yours by definition)
  are not. Batch rulings into the fewest checkpoints that respect these gates. **Verification depth
  is floored:** it may thin on a light cycle — quality gates plus spot
  evidence rather than full evidence capture — never to zero. No cycle closes without
  real-infrastructure evidence; *none* is a reviewer-count option, never a verification one. No
  lead-penned surface takes a standing cold grade here: the uncertainty carrier is
  producer-authored — were you to pen a deliverable surface, it would take one cold-seat grade
  non-discretionarily, waivable only by recorded user waiver at the weight card.
- **Bounds:** **targeted retry** — trace a checkpoint failure to its tasks and re-open only those,
  **max 3 attempts per cycle**, never regressing passing code; **fix pass** — failure-scoped after a
  final-validation failure, **max 3 passes**; **convergence stall** — the same failure pattern
  across **2+ rounds** surfaces rather than silently continuing, no-progress being an unchanged
  failing set; kill-switch `.mochiko/specs/<feature>/IMPLEMENT_STOP`, checked before each seat send.
  You count every round. Any bound this run declares — including a declared cost range — has you as
  its named counter, **rises only at a user checkpoint**, and is re-declared only on the record;
  busting a bound escalates, never silently continues.
- **Ordering invariants:** cycles run in dependency order, **all foundation cycles before feature
  cycles**, the current cycle being the first with unchecked tasks. **Sequential-only** — parallel
  cycle execution is a `deliberate-shortcut-ledger` deferral, not a capability drop. Every produced
  cycle is paired with a verification in the same round, never skipped: the hand-off is peer-routed,
  the pairing is yours to enforce. The final validation is lead-routed, never devolved.
- **Cold tree:** the final validation builds and runs the quality gates from a dependency-cold
  snapshot of the **uncommitted working state** — `git ls-files -co --exclude-standard
  :!.claude/worktrees` copied to `.claude/worktrees/mochiko-<purpose>/`, carrying no warm items —
  its results part of G5's evidence.
- **Per-cycle qa isolation:** yours to compose per run — that snapshot plus a declared carry-set of
  warm gitignored items, dependencies **copied or installed, never linked**. Tear **either**
  snapshot down only after its evidence is captured and any snapshot-only failure dispositioned; a
  failed cycle's, after its retry.
- **Scaffolding:** from the detected stack, create any missing ignore files (`.gitignore` /
  `.dockerignore` / lint-ignore) and the `/.claude/worktrees` ignore entry, project-relative, once
  before the cycle loop.
- **Slice scope** *(when an accepted `slices.md` exists)* — that file's **Graduation contract** is
  the single home for slice resolution, the staleness guard, scope, extend-mode, graded amendment,
  and layout; not restated. implement's own bindings on top: the package gate and the cycle loop read
  `slices/<slice>/tasks.md`; the design inputs are the shared feature-root artifacts plus
  `slices/<slice>/{plan.md, task-mapping.md}`; the quality gates still run the **full repository
  suite** — earlier slices' tests are the regression net that catches a design amendment breaking
  shipped behavior; and when the last slice in Slice-order clears G5 the *feature* is **declared, not
  verified** — `slices.md`'s Feature-Done section executes at feature-close, owned by no workflow
  yet: surface it as the next step, never report feature completion here.
- **Ground rules:** kernel-free — no brain code, no capability catalogs, no DAG-mediated
  orchestration. Suggest commits; never run git mutations, never push — the ban's surface is refs,
  index, tracked content, and history, so the ephemeral, self-removed verification snapshot above
  is not a mutation of it. No internal machinery vocabulary in user-facing prose — the
  conversation is yours and the user's, in the mochiko register (`templates/output-style.md`).
  User acceptance is plain blocking text, never a timed prompt. Reports are written as the work
  progresses, never reconstructed at the end.

## Bindings

- **Deliverable:** the **working code**, in `tasks.md`'s cycle / task (`T{N}.{X}`) ID namespace, whose
  checkboxes flip `[ ]` → `[x]` as tasks complete.
- **Reports** under `.mochiko/specs/<feature>/` — or `slices/<slice>/` when slice-scoped, where the
  Goal's artifact set reads them: `cycle-report.md` and a verification report per cycle, the
  final-validation report, the built-vs-approved diff report.
- **Design inputs:** `plan.md`, `architecture.md` (the **anchor** for both architecture mechanisms),
  `task-mapping.md`, `data-model.md`, `contracts/api.yaml`, `constraints-and-decisions.md`,
  `requirements.md`. Workspace-as-state, no registry field.
- **Uncertainty carrier:** producer-authored — `cycle-report.md`'s honest difficulties, deviations,
  `domain_deps_added` and flagged blockers (`mochiko:executing-tdd-cycle`'s format); qa's evidence
  and status live in its verification reports.
- **Fact route:** real infrastructure — executed `**TEST:**` tasks and quality-gate exit codes;
  knowledge gaps go to a native `Explore` pass.
- **Clearing unit + checkpoint keying:** the **cycle**; a surfaced architecture deviation
  **de-devolves** it, and a non-empty `domain_deps_added` **always** forces the escalated human
  checkpoint — never auto-approved, no stamp read.
- **Run-start declaration:** one line at the head of `tasks.md` — the deliverable's progress surface,
  where Recovery already notes the resume stage — for a default run; a run that departs from the
  stated default, or declares non-default bounds, writes a departure record at
  `.mochiko/specs/<feature>/implement-contract.md` beside the reports instead — the
  done-condition and bounds as (re-)declared, departures taken, and the counter state Recovery
  reads on resume. Counted unit: the
  **cycle**, the unit the bounds and the lifecycle cadence already count.
- **Departure trail:** one line per departure from the stated default, appended under that same
  `tasks.md` declaration as it is taken and carried into G5's evidence — never your context alone;
  the trail names the grading that actually ran. Departure is by record, never by silence.
- **KM landing:** `.mochiko/memory/knowledge-management.md` exists → run its ritual + invariants
  under fix-on-sight; a **built** structural change folds the built system into `ARCHITECTURE.md`.
  No copy → skip.

## Recovery

Note the resume stage on the deliverable, with the run's counter state — cycles and rounds
consumed · bounds declared · departures taken. Sessions and teams do not survive `/resume`, and a
shared account limit can throttle the team and the main session together — escalation then has
nowhere to go but pause. Resume from workspace evidence, never a context `phase` field, respawning
only what the stage needs — a respawned producer re-reads the cycle's tasks, the design inputs,
and any failed-task list, and a respawn is cold by design.

| Evidence in the workspace | Resume at |
|---|---|
| no `tasks.md`, or an incomplete package | entry blocked |
| `slices.md` present | resolve the current slice; the rows below then read `slices/<slice>/tasks.md` and per-slice reports |
| `tasks.md` present, ignore files absent | scaffolding |
| unchecked tasks remain, the current cycle has no `cycle-report.md` this round | implement the current cycle |
| the current cycle's `cycle-report.md` present, no verification report this round | verify the current cycle |
| a surfaced deviation unruled | the architecture-deviation gate |
| the current cycle not passed, within the cap | retry / cycle loop control |
| all tasks `[x]`, no final verification report | final validation |
| final validation failed, within the cap | fix pass / loop control |
| final validation cleared, an approved delta existed, no diff report | the built-vs-approved diff |
| final validation cleared, not yet accepted | G5 |
| accepted | finalize — report the code and reports, the per-cycle and fix-pass round counts, the cycle / task / fix-pass counts with quality-gate status, the KM landing, a suggested commit (`feat: implement <feature>`), and the next step |
| `IMPLEMENT_STOP` present | escalate (G4) |
````

</details>

---
## [v0.46.0] Doctrine-purge rewrite — obligated reads out, shape mechanics inlined
- **Disposition:** superseded → the command's own text
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** the preamble's obligated shape/loop-discipline reads and "in the mochiko command shape" framing left; "the shape's devolved branch" re-attributed to "this command's devolved branch" (description + cycle checkpoint); G3's gap-taxonomy vocabulary reworded to plain lead-judgment routing.
- **Kept deliberately:** the devolved branch's four conditions (already inline, unchanged) — plus all gates/bounds/bindings/recovery and the inlined weight-card factors, floor rules, transport, lifecycle, mesh hold, ground rules (with the snapshot ban-reading carve-out), counter-state recovery.
- **Consumers assessed:** none.

---
**Wave context (v0.44.0 — the D7 leakage scrub).** `verbosity-caveman-ops-separation` D7 as
folded at review (S4): **full scrub** of ops leakage from the shipped tree, with no
changelog-worthy detail lost — every removed block is preserved verbatim below. Ruling:
`DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation ruled" row.

**The leak test this wave used, recorded so a future sweep inherits it: *whose artifact does the
pointer name?*** Mochiko's own ops records — `.mochiko/strips/`, `.mochiko/brainstorms/`,
`.mochiko/decisions/`, `.mochiko/archive/` — are leaks: they resolve to nothing in an installed
plugin. Adopter runtime paths (`.mochiko/specs/`, `.mochiko/memory/`) and the KM module's
document contracts are the **user's** artifacts and are untouchable. A prefix-based sweep on
`.mochiko/` would gut the KM module and the brainstorm command; 101 of this tree's 146
`.mochiko/` references were correctly left alone on that test.

## [v0.44.0] Per-cycle isolation rationale pointer
- **Disposition:** superseded → deleted from the shipped file; preserved verbatim here.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim):**
```
Triggers, evidence provenance, the git-dependent-gate fallback,
  and rationale: `.mochiko/brainstorms/validator-worktree-isolation/record.md` (D3–D7).
```
- **Kept deliberately:** every operative clause of the isolation binding — lead-composed per run, the declared carry-set, deps copied-or-installed-never-linked, and the teardown ordering.

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
- **Measured:** `commands/implement.md` **16,046 → 16,021 B** (−25). Derived figures in this note's
  conversion section re-measured accordingly, superseded values kept inline.

# v0.43.0 — the first v6→v7 conversion

**Wave context:** shape **v7** landed at v0.40.0 (`lead-owned-process-flexibility`,
`.mochiko/brainstorms/lead-owned-process-flexibility/record.md`; `DECISIONS.md` 2026-08-01 — the
lead-owned-process-flexibility row plus the shape-v7 wave-close ratification row), with **D4**
ruling **convert-on-touch** and all six commands staying v6-form. implement's conversion was
**deferred by user ruling at the v0.42.0 touch** — that wave was ruled surgical and v6-form, the
**F66** trigger left live and the conversion routed to a dedicated wave. This is that wave; the
F66 deferral is discharged. BACKLOG: "convert-on-touch residuals".

It also carries the **first-conversion ceiling-term obligation**, user-ratified at the v0.40.0 wave
close (2026-08-01) — *the first conversion measures its Constraints/Bindings blocks and lands any
needed check-6 term in the same wave* — which discharges item 4 of the v0.40.0 shape note's
*Deliberately not encoded* list. Both terms landed:
`.mochiko/strips/validation-command-shape.md` [v0.43.0].

**Post-conversion measurement, all blocks, body-only in words** (`## Heading` lines excluded, per
check 6): preamble **114/130** (published as 118 while the 4-word form marker stood;
retired at the wave close) · Goal **145/150** · Seats & checks **324/340** (unchanged) ·
Constraints **1031/1110** · Bindings **263/290** · Recovery **239/242** (unchanged). Term
derivation as check 6 requires: **G = 9** — the eight prior gate lines plus the run-start weight
card, all nine carrying the complete three-part `evidence:`/`rules:`/`decides:` form — so
Constraints is 90·(9+2) = 990 **plus the new +120 P18 term** = 1110. **S = 4** and **R = 13**, both
unchanged. **A = 5**, unchanged from the v0.42.0 reading (working code · `cycle-report.md` · the
per-cycle verification report · the final-validation report · the built-vs-approved diff report),
so Bindings is 90 + 12·5 + 30 (KM) **plus the new +110 P19/P20 term** = 290.

> **One A-term judgment, recorded so the next auditor does not re-derive it.** P19 names
> `implement-contract.md` as a **departing** run's per-run carrier. It is **not counted in A**: it
> is neither a deliverable nor a round report, and it exists only on a departing run. Counting it
> (A = 6) would raise the Bindings ceiling to 302 and so only loosen the check — the conservative
> reading is the one measured here.

## [v0.43.0] The Goal's end state loses its seat choreography and its lead-read clause
- **Disposition:** superseded → rewritten in place as artifact state. The verified evidence the end
  state named **survives as the artifacts that carry it** — a verification report per cycle and one
  for the whole implementation, each naming its real-infrastructure evidence tree and its
  quality-gate exit codes.
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, ratified at **A4**, 2026-08-01: *"Goal
  blocks lose process residue. Done = artifact state + floor compliance + user acceptance"*; graded
  by `validation-command-shape` check 23, v7-form only).
- **Protected content, leaving by ruling and named as such:** the second clause is
  `DECISIONS.md`-traceable — the [v0.31.0] entry below records the team-method **D3** rewrite of
  the Contract done-condition into exactly this text ("the final-validation report plus every
  **escalated** cycle's reports"). It is superseded by a cited ruling, not dropped.
- **Content (v6, verbatim — the two clauses that left):**
  ```
  `qa-engineer` verification passed on every
  cycle **and** on the whole-implementation final validation, on real-infrastructure evidence and
  quality-gate exit codes; you Read the final-validation report and every escalated cycle's reports and
  found no blocking gap —
  ```
- **Kept deliberately:**
  - **All four build-state findings**, in substance verbatim: "criteria met, gates passing,
    traceability to requirements holding, the build aligned with the project's governance" → "the
    built code meets its criteria, holds traceability to requirements, and aligns with the
    project's governance", *gates passing* carried by the quality-gate exit codes named one clause
    earlier. Nothing in the finding set was dropped.
  - **The real-infrastructure evidence and quality-gate exit codes** — moved from *what qa did* to
    *what the reports contain*, so the Goal and the D5 verification floor (now in P18) read the
    same evidence.
  - **The not-re-read consequence of the devolved branch** — untouched at the Cycle-checkpoint
    constraint the v0.35.0 ledger assigned it to: "the cycle then clears on qa's PASS-with-evidence,
    **unread by you**".
  - **"the KM landing ran" and "the user accepted at G5"** — both are explicit end-state elements
    in the shape's own Goal spec (the KM landing under fix-on-sight; user acceptance as part of the
    end state), so neither reads as residue.
- **Consumers assessed:** not a shared primitive. Two cross-file consumers checked: the grader's
  check 23 (this is the text it was written for — `.mochiko/strips/validation-command-shape.md`
  [v0.40.0]) and the five remaining commands, whose Goal blocks **stay exactly as written** — the
  residue clause is v7-form-only and each converts at its own touch (shape note [v0.40.0], *The
  Goal block's process residue left the end state*, Consumers assessed).

## [v0.43.0] The not-done state `a non-clean cycle advanced without your verdict`
- **Disposition:** superseded → deleted from the Goal. The rule it echoed is unchanged at its
  ledgered home, the **Cycle checkpoint** constraint: "Otherwise it fires — any failure, any GUI or
  subjective verification, any reported deviation, any registry addition — and you rule on the
  reports."
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, as above). It made the lead's own
  process step a done-condition element, which is the residue class check 23 fails; and it can
  never be rescued as a floor gate, because the cycle checkpoint reads `rules: you`, not `rules:
  the user`, and check 21's floor-gate test keys on the latter.
- **Protected content, leaving by ruling:** `DECISIONS.md`-traceable to team-method **D3**
  ([v0.31.0] below). The v0.35.0 CS-D8 ledger assigns that row's home to the **Constraints**
  cycle-checkpoint line — "Same line's *Otherwise it fires — …*, all four classes enumerated" — not
  to this Goal state. The protected content stays where the ledger put it; only the Goal echo left.
- **Content (v6, verbatim):** `a non-clean cycle advanced without your verdict`
- **Kept deliberately:** the whole devolved-branch predicate and its four escalation classes, in the
  Cycle-checkpoint constraint; the lead's verdict ownership, in the validation-model line.
- **Consumers assessed:** as above — not a shared primitive; grader check 23 and the five v6-form
  commands, both unaffected.

## [v0.43.0] Two not-done states re-read from process to artifact state
- **Disposition:** superseded → rewritten in place. The same states, named by the artifact that is
  missing rather than by the step that did not run.
- **Tier failed:** n/a — supersession by ruling (**D6(b)**, as above).
- **Content (v6, verbatim → v7):**
  - `a cycle or the final validation unverified` → `a cycle or the final validation with no
    real-infrastructure evidence`
  - `an approved delta whose diff never ran` → `an approved delta with no built-vs-approved diff
    report`
- **Protected content:** the second is `DECISIONS.md`-traceable to **AD-D6.3/R8**, whose v0.35.0
  ledger row names *the Goal's end state and not-done state* among its four resolution homes. Both
  homes survive in the converted Goal — the diff report exists (end state) / an approved delta with
  no diff report (not-done) — so the row is **preserved**, and this entry records a rewording, not
  a supersession of the row. The first state is re-anchored on the D5 verification floor now stated
  in P18, which is what "unverified" always meant here.
- **Kept deliberately:** **`a warm-only final validation`** — untouched, exactly as the [v0.42.0]
  entry below logged it ("Warm-only as a named not-done state — untouched in the Goal block"); and
  every other not-done state, unedited.
- **Consumers assessed:** as above.

*Pure additions this wave, riding the decision row rather than these entries:*

- **The form marker** `<!-- shape-form: v7 -->` in the preamble — check 20's branch key.
- **The run-start weight-card gate line** (P7) — U1-A's standing user stop, in the three-part
  countable form, taking **G from 8 to 9**.
- **`**Floor gates:**`** (P18) — the floor set (the run-start weight card · the package gate · the
  governance surface's absence ruling · **G3**'s preference ruling · **G4** · **G5** · the
  architecture-deviation consent) with the non-floor two named, so the absence is stated rather
  than inferred; the **D5
  verification-depth floor** at its named natural site (`workflow-token-reduction` **D5** as split
  at S5, ruled 2026-07-23, no-softening confirmed — all three limbs carried: depth may thin on a
  light cycle, never to zero, and *none* is a reviewer-count option only); and the
  lead-penned-surface element stated as an **absence**, implement's P11 being producer-authored.
- **`**Run-start declaration:**`** (P19) and **`**Departure trail:**`** (P20) in Bindings — the
  declaration on the deliverable's progress surface for a default run, an instantiated
  `implement-contract.md` for a departing one, and the **cycle** named as the counted unit (check
  22), the same unit the Bounds and the P17 lifecycle line already count.
- **One new not-done state** — `a departure with no trail line`, the honest-trail invariant made
  visible in the Goal as floor compliance.

**Two judgments made here rather than deferred, flagged for the grader.**

1. **The floor-gate set is seven of nine, and the ground is *who rules*, not how heavy the gate
   is.** *(Corrected at the audit's fix round 1 — the governance surface moved into the floor set;
   the original six-of-nine reading is superseded, see the axis note below.)* The two left as
   departable defaults are the two whose ruling was never the user's to lose: **G1 entry** and the
   **cycle checkpoint** (`rules: you`, so check 21's test excludes it by construction). **G3 is
   marked floor on its narrow limb only** — the *preference ruling* is the user's under floor
   invariant 1; *when* it is presented stays the lead's under **D3**'s consolidation authority,
   which is home doctrine and is deliberately not restated in the command.

   **Why G1 clears, stated on the ground that actually carries it.** The first draft argued only
   the explicit-ID case (an explicit `<feature>` in `$ARGUMENTS` leaves nothing to rule), which is
   too narrow — it says nothing about the detected-feature branch. The clearing ground is
   structural: **the package gate is floor and its evidence is the resolved feature's package**,
   deciding "whether the run opens". So a lead that composes out G1's confirm still puts the
   resolved feature in front of the user before the run opens, on the very next gate. G1's confirm
   is a convenience stop, not the invariant's only carrier — which is exactly what makes it safely
   departable and the package gate not.

   **Why the governance surface is floor: the blocking and floor axes are independent, and the
   first draft conflated them.** *Blocking* asks whether the run stops; *floor* asks who rules.
   This gate is **non-blocking AND floor** — its own protected text ("Absence is **surfaced, never
   auto-resolved** — governing context, not a blocking gate") settles the first axis and says
   nothing about the second, while `rules: the user, when it is absent` settles the second
   outright. Excluding it produced a contradiction inside one block: a gate reading `rules: the
   user` sitting in a not-floor list whose own sentence defines floor as never-departable. The
   alternative repair — reading the surfacing as departable — was **not taken**: it would be a
   behavior change to a protected pre-wave line, which no ruling in this wave authorizes.
2. **The declaration and the trail share one surface.** Both land at the head of `tasks.md` — the
   deliverable's progress surface, where Recovery already notes the resume stage, so a resumed lead
   finds declaration, departures and resume state in one place. **`cycle-report.md` was rejected as
   a home:** the P17 lifecycle line records that the next attempt **overwrites** that file, so a
   trail parked there is a trail that can vanish mid-run.

**Recovery left untouched, deliberately.** The shape's counter-state clause is home doctrine (v7
Recovery block); implement's pause line — "Note the resume stage on the deliverable" — does not
contradict it and names the same surface P19 binds, so no edit was owed.

### R21 heavy-site measurement — the first measured v7 conversion

`lead-owned-process-flexibility` **R21** carries a recorded-open obligation: *a measured cost
estimate for declaration + trail + composition on one light and one heavy run* (verify N3, narrowed
by **A3** to the estimate alone). implement is the **heavy site** — the library's densest command.
Figures are `wc`-measured after the last edit (**re-measured at the audit's fix round 1**, which
added 3 w to the P18 binding — the superseded figures were 16,028 B / +1,526 / Constraints 1028,
recorded here so the drift is traceable rather than silently overwritten). **No offsetting saving
is claimed, because none exists:** the Goal strip returned 67 B against 1,611 B of additions.

**File growth.** `commands/implement.md` **14,502 → 16,021 B** (+1,519; words 2,014 → 2,242,
+11.3%). Attribution, each construct measured on its own text:

| construct | bytes | words |
|---|---|---|
| ~~`<!-- shape-form: v7 -->` marker~~ — added here, **retired at the wave close** | ±0 | ±0 |
| run-start weight-card gate line (P7) | +266 | +44 |
| `**Floor gates:**` — floor set + D5 depth floor + P11 absence (P18) | +674 | +100 |
| `**Run-start declaration:**` (P19) | +473 | +61 |
| `**Departure trail:**` (P20) | +173 | +27 |
| Goal block, D6(b) residue strip | −67 | −4 |
| **net** | **+1,519** | **+228** |

**Per-run read cost.** implement.md is an obligated read once per run, so this is **+1,519 B on
every implement run** — 10.5% on top of the command itself. It is not the whole delta a run pays
this wave: the shape-home edits add **+450 B to `command-shape.md`** (31,816 → 32,266 — the
v6-form weight-card clause 236 B, its footer stamp 213 B), and *that* one is paid by **every
team-form run of any command**, converted or not, because the shape home is the shared always-read
floor. Against v7's own doctrine cost (+11,399 B/run, measured at
v0.40.0), conversion is the small half of the bill.

**Run-time cost of declaration + trail — an estimate, and marked as one.** Three components, none
yet observed on a live run:

- **The declaration, every run.** One line on `tasks.md` stating the four-factor read and the
  composed process. At the density this repo's own cards use, ~30–60 words (~200–400 B), produced
  once and re-read on every resume. It is the only one of the three a **default** run pays.
- **The trail, per departure.** ~15–25 words (~100–170 B) a line. A run that takes the stated
  default pays **zero**, and the cost scales with departures — the intended shape: the lead buys
  flexibility by the line.
- **The contract, departing runs only.** `templates/workflow-contract.md` measures **5,572 B**
  today, so a departing run reads 5.6 KB and writes a filled copy of comparable size to
  `.mochiko/specs/<feature>/implement-contract.md`. The largest run-time item by far, and
  **conditional by construction** — no default run touches it.

**The honest read at this site.** A default implement run pays the +1,519 B read plus one
declaration line — ~1.8 KB, on the command carrying the library's largest protected surface. A
departing run adds ~5.6 KB of template plus its fill, plus a line per departure. **The light site
stays unmeasured**, so R21 remains open at half; the next conversion of a light command closes it.

---

## [v0.42.0] Cold checkout's **fresh clone** superseded by the git-semantics filtered snapshot

- **Disposition:** superseded → the `**Cold tree:**` + `**Per-cycle qa isolation:**` constraints
  that replaced it, plus the extended `**Scaffolding:**` constraint and the verifier seat row.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/validator-worktree-isolation/record.md`
  **D3(i)/D4**, with D5–D7 for the option, carry-set and teardown; `DECISIONS.md` row
  2026-08-01 "Validator snapshot-isolation ruled"). Not a minimalism strip: the line was made
  wrong, not verbose — executed against an uncommitted implementation, a clone of HEAD gates a
  tree that does not contain the work under validation (F22–F24).
- **Content:**
  ```
  - **Cold checkout:** the final validation builds and runs the quality gates from a **fresh clone**
    of the repository, never only the warm working tree; that clone's results are part of G5's
    evidence.
  ```
- **Kept deliberately:**
  - **The works-warm-only catch** — the whole ground of the 2026-07-31 ruling. The snapshot drops
    gitignored dirs exactly as a clone does, so the catch is preserved by mechanism, not by
    assertion (record F14/F15, F32/F57c).
  - **G5 evidence status** — "its results part of G5's evidence" survives verbatim in intent.
  - **Warm-only as a named not-done state** — untouched in the Goal block ("a warm-only final
    validation"). See the deviation note below on where the Constraints echo went.
- **Consumers assessed** (the three-carrier set enumerated at D3(i), one disposition each):
  - `plugins/mochiko/commands/implement.md` — **edited** under this entry (the carrier being
    superseded).
  - `.mochiko/decisions/2026-07-31-team-method-escalations-closed.md` — **marker appended**
    ("Superseded-mechanism note (2026-08-01)"); the ADR's existing text is untouched, intent
    standing.
  - `DECISIONS.md`, the 2026-07-31 cold-checkout row — **annotated** in the same landing
    ("cold-checkout *mechanism* superseded 2026-08-01 → the validator snapshot-isolation row;
    intent stands").

**Two ceiling-forced deviations, recorded rather than silent.** implement's Constraints ceiling is
90·(G+2) = **900 w** at G = 8, and the block stood at **817 w** before this wave — 83 w of headroom
against a ruled addition set (mechanism · per-cycle option · carry-set · teardown · fallback ·
evidence provenance) that measured ~136 w at first draft. Both moves below were made to land the
ruling inside the floor rather than ship a check-6 FAIL; neither drops a responsibility.

1. **"never only the warm working tree" is not restated in Constraints.** The prohibition survives
   as the Goal block's not-done state, which this wave did not touch. Dropping the Constraints echo
   removed a duplication; the mandatory phrasing of the replacement ("builds and runs the quality
   gates from a dependency-cold snapshot") carries the same obligation at the point of use.
2. **U7's report-provenance obligation landed in the Seats & checks verifier row**, not Constraints —
   "→ a verification report **naming its evidence tree** + a checkpoint recommendation" (+5 w;
   Seats 319 → 324 against its 340 ceiling). That row covers both reports U7 names, per-cycle and
   final, since the same seat produces both. The variant/carry-set enumeration ("warm, or snapshot
   path + variant + carry-set") is single-sourced to the record and reached from the Constraints
   reference, which names evidence provenance explicitly.

Post-edit measurement, all blocks: preamble 114/130 · Goal 149/150 · Seats & checks 324/340 ·
Constraints **887/900** · Bindings 175/180 · Recovery 239/242. G unchanged at 8 (neither new bullet
carries the three-part `evidence:`/`rules:`/`decides:` form), so the ceiling term is unchanged.

**Two measurement notes, so the next auditor does not re-derive them.**

1. **The Bindings term is A = 5**, not the **A = 6** this file's v0.35.0 entry measured.
   `validation-command-shape` check 6 now bars counting a **KM-landing fold target** — a doc the
   command folds *into* rather than produces — as an own-output, which removes `ARCHITECTURE.md`
   from the set. What remains: working code · `cycle-report.md` · the per-cycle verification
   report · the final-validation report · the built-vs-approved diff report. Ceiling
   90 + 12·5 + 30 = **180**, measured **175**. **This retires the v0.35.0 entry's "At-risk
   measurement" flag** (`Bindings passes at A=6 and A=5 but fails at A=4`): the failing case needed
   the built-vs-approved diff report discounted *as well*, and the skill bars only the fold target,
   so A = 4 is not reachable under the written rule. The flag is answered, not re-argued.
2. **The "preamble 114/130" figure above counts the `# ` title line; strict body-only is 103.**
   Check 6's exclusion is written for a block's `## Heading`, and the preamble has no `##` heading,
   so the margin is unsettled by the letter of the rule. Recorded rather than ruled because both
   readings clear 130 comfortably — nothing in this wave turns on it.

**v7 convert-on-touch deferred at this touch — by user ruling.** The record's Open thread 5 fires
convert-on-touch (F66) at build scoping because this build touches implement; the user ruled this
wave **surgical and v6-form**, with conversion going to a dedicated wave. So implement carries **no**
`<!-- shape-form: v7 -->` marker and no P18–P20 bindings, and is graded on the v6 slot set. **The F66
trigger stays live** — the next touch faces the same decision.

*Pure additions this wave, riding the decision row rather than this note:*

- **The U1 ban clarification** — one sentence in `command-shape.md`'s Layer 1 Ground rules: the
  ban's surface is refs, index, tracked content, and history; an ephemeral self-removed
  verification worktree is not a mutation of it.
- **The `mochiko-` snapshot name prefix** — the snapshot home is
  `.claude/worktrees/mochiko-<purpose>/`, never a bare `<purpose>`. Ground: **F76** measured that
  `git worktree add` refuses a **non-empty existing directory** (`fatal: '…' already exists`), and
  the docs' name-reuse rule keys on directory existence without distinguishing a registered
  worktree from a plain one — so a snapshot parked on a name the harness later wants would block
  worktree creation there. **F77** puts real traffic at that path: background sessions isolate into
  `.claude/worktrees/` as well. The prefix makes the collision impossible by construction rather
  than by convention. The periodic sweep is *not* the hazard here — F72–F75 establish it as
  worktree-registry-scoped, so an unregistered directory is not a target.

---

## [v0.38.0] `RETURNED:` — the seat-recycling binding, re-added as a P17 lifecycle line

- **Evidence:** `.mochiko/brainstorms/team-lead-strategic-compaction/record.md` **TC-D5/TC-D6**
  (`DECISIONS.md` 2026-07-31) + `.mochiko/brainstorms/plan-run-transport-forensics/record.md`
  **R1** (user-ruled 2026-08-01) + the open BACKLOG item "Standing-seat build items — surface
  specified 2026-07-31". Wave note: `.mochiko/strips/command-shape.md` v0.38.0. Not an override
  re-add — every ground the v0.35.0 revert gave is discharged below, by name.

**The three grounds of the v0.35.0 revert, each answered:**

1. *"The wave's contract is translation under true-reductions-only accounting, and this was its
   only line of new behavior."* — **Spent.** That was the goal-shape translation wave's contract;
   this is the build wave the same entry pointed at ("the BACKLOG item remains the build's proper
   home"), and its contract is to build.
2. *"D1's cycle floor is probe-calibrated and the D4 probe is deferred, so the command would carry
   an approximate threshold."* — **Still true of the probe, and answered by where the number now
   lives.** The D4 probe remains deferred, but **TC-D6 ruled the ~≥3 default into Layer 2** as a
   probe-tunable shape value. So implement carries **no threshold at all**: the re-added line
   names its *unit* (the cycle) and its *override*, and inherits the number. When the probe tunes
   the default, one shape edit re-tunes all six commands and implement needs no touch — which is
   the outcome this ground was protecting.
3. *"Standing-seat D3 — the Layer-2 reframe that would give each command a per-seat lifecycle
   `[PARAM]` — is unbuilt … so the invariant was a workaround for a missing shape slot rather
   than a home."* — **Discharged: the slot now exists.** Shape **v6** carries the two-axis Layer 2
   and **P17**, so the re-added text is a slot binding, not a workaround.

**What came back, and it is smaller than what left.** The revert removed an 88-w `Seat recycling`
invariant from Constraints plus "recycled per Constraints" / "recycled per slice" from the two
spawn cells. What returns is a **48-w** `**Seat lifecycle:**` line beneath the Seats & checks
table — **40 w less**, and in a different block, because the doctrine the 88 words carried is now
in the shape home. **Constraints is untouched by this re-add**, measured **784/900** at this
revision — *not* the 796 the v0.35.0 entry records: that figure was correct when written and the
**v0.37.0 `@`-reference supersession took 12 words out of G1**. Re-measured rather than carried,
because a figure quoted from a prior wave is exactly the kind that goes stale unread. The words
land in Seats & checks instead: **271 → 319**, against the ceiling's new `+60` P17 term
(280 → 340).

**What the line binds, all three of implement's genuine differences from the Layer-2 default:**
the **counted unit is the cycle** (implement's Bounds count three different things — retries per
cycle, fix passes, and stall rounds — so the lifecycle denominator is ambiguous without this) ·
the **verifier's per-slice override** with its final-validation incarnation briefed from the
on-disk verification reports (standing-seat **D1**, the ruled asymmetry) · the **retry/fix-pass
respawn** carrying the failed-task list **and** the just-failed `cycle-report.md` **relayed at
dispatch** (D1's S1 fold, on F-g's finding that the next attempt overwrites that file — a later
re-read would hit the wrong report).

**Not re-added, deliberately:** the producer's `~≥3` cycle floor and the gate-pause/cache-warmth
condition — both are now the Layer-2 default and restating them in the command would be the
restatement the shape forbids (TC-D6 as amended, RI-2: "implement conforms on the producer floor
but carries one explicit override"). The two **spawn cells are left as they are** ("standing
across the cycle sequence and the fix-pass loop" · "cold at the first cycle verification,
standing after"): shape v6 states that a roster row reading *standing* describes the **seat**,
not one context, so the cells are accurate under recycling and needed no re-edit — this is the
clause that made the v0.35.0 cell edits unnecessary rather than merely reverted.

## [v0.37.0] `@`-reference recovery superseded — the platform bug it named is resolved
- **Disposition:** superseded → user ruling (2026-08-01). The bug-attributed re-enter workaround retires; the most-recent-feature resolution is relocated into the decides-clause with a confirm.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/decisions/2026-08-01-at-reference-recovery-superseded.md`; `DECISIONS.md` 2026-08-01).
- **Content (superseded, verbatim):** "Empty `$ARGUMENTS` (the known `@`-reference drop bug) → ask the user to re-enter it, or to confirm the detected feature (an explicit ID, else the most recent in-progress feature under `.mochiko/specs/`)."
- **Kept deliberately:** the resolution clause and a confirm — G1 now decides "the resolved `<feature>` (an explicit ID, else the most recent in-progress feature under `.mochiko/specs/`, confirmed with the user before the run opens)". Only the re-enter workaround and the bug attribution left.
- **Consumers assessed:** five-command recovery — see the shared consumer list in the `strips/plan.md` v0.37.0 entry; implement carried the resolution clause `plan` referenced, and keeps it.
- **Protected-set note:** as recorded in the plan entry — record §7's protection premise for this recovery is spent now the bug is resolved; deliberate supersession, not a check-14 re-drop.

# v0.36.0 — the production-only re-key (stage 4)

**Wave context:** the PO narrowing build, stage 4 of 5 — the two commands aligned with the
constitution cluster rewritten earlier in the same wave. Scope ADR:
`.mochiko/decisions/2026-07-30-po-narrowing-build-scope.md`, scoping PO-D1–D7 from
`.mochiko/brainstorms/production-only-focus/record.md`. **One site, in Bindings**; shape stays **v5**
(G = 8, S = 4, blocks unmoved), Bindings 178 → 175 w.

## [v0.36.0] The cycle checkpoint no longer keys on tier
- **Disposition:** superseded → the one universal gate at the asserted floor in
  `authoring-constitution/references/DOMAIN-DEPENDENCIES.md` ("Growth" — human ruling before
  registry entry; the checkpoint MUST NOT auto-approve while `domain_deps_added` is non-empty), with
  the same always-forces reading now in `executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md`'s
  field table
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/decisions/2026-07-30-po-narrowing-build-scope.md`; PO-D2 retired the tier fork)
- **Content:** "the escalated checkpoint keys on tier — `production`/`regulated` (the `CLAUDE.md`
  stamp) forces it for a `domain_deps_added` entry; non-blocking below." → "a non-empty
  `domain_deps_added` **always** forces the escalated human checkpoint — never auto-approved, no
  stamp read."
- **Kept deliberately:** P14 stays bound — the clearing unit (**the cycle**) and the
  architecture-deviation de-devolution are unchanged; only the checkpoint's key moved. Noted while
  editing: the Cycle-checkpoint constraint already required an empty `domain_deps_added` for
  devolution *unconditionally*, so the tier fork had left Bindings and Constraints in latent
  disagreement — this re-key closes it rather than creating consistency that was never there.

---

# v0.35.0 — the goal-shape wave (CS-D10 step 4)

**Wave context:** command goal-shape rebuild, **step 4 of 4** — the five-command wave following the
plan pilot (design: `.mochiko/brainstorms/command-succinctness-strip/record.md`, CS-D3/D4/D5 + D8 +
D10; pilot checkpoint ADR `.mochiko/decisions/2026-07-30-goal-shape-pilot-checkpoint.md`). Authored
against **shape v5** with the obligated `mochiko:loop-discipline` read **retained** — the drop is
deferred to a named live-run trigger, so a v5 command that omits it is non-conformant, not early.
implement declares the **in-loop critique** branch of P6, so it must **not** reference
`templates/sized-end-stage-review.md` (check 1's negative direction) — it does not.

**Measured: 3,230 → 1,868 words (−42.2%), 23,873 → 13,331 B (−44.2%)** — `wc`-measured after the
**seat-recycling revert**, which was the last edit (the pre-revert file measured 1,962 w / 13,959 B;
those figures are superseded here and everywhere in this note, per the pilot's standing
headline-sweep habit). Against the wave's pre-authoring floor row of **1,354 w: +514 w (+38.0%)** —
over, not under, which is CS-D8's safe side. The overage is accounted by block: implement's protected
surface is the densest in the library and three blocks sit at 92–98% of their ceilings after every
trim below. The floor row assumed a G=5/S=3 file; this one lands **G=8, S=4** because two source
decision points were restored to gate form and the architecture dispatch was split into two seat rows
(both below).

Block sizes against the grader's ceilings, **with the term derivation as check 6 requires** —
**G=8** gate lines (bullets matching `^- \*\*` containing `evidence:`: G1 entry · package gate ·
governance surface · cycle checkpoint · architecture deviation · G3 · G4 · G5), **S=4** seat rows,
**A=6** own-outputs (working code · `cycle-report.md` · the per-cycle verification report · the
final-validation report · the built-vs-approved diff report · the `ARCHITECTURE.md` fold), **R=13**
resume rows: preamble 114/130 · Goal 144/150 · **Seats & checks 271/280 (96.8%)** · Constraints
796/900 (88.4%) · Bindings 178/192 (92.7%) · **Recovery 239/242 (98.8%)**.

> **At-risk measurement, flagged rather than resolved (auditor: check this term first).** Bindings
> passes at A=6 (178/192) and at A=5 (178/180) but **fails at A=4 (178/168)**. A=4 requires
> discounting two enumerated own-outputs — the built-vs-approved diff report and the
> `ARCHITECTURE.md` fold — both of which this run's own dispatches produce. The +30 KM term is
> claimed and the KM-landing binding is present. If the grader counts A=4, the fix is a real cut,
> not a re-argued term.

**Three ceiling pressures resolved by relocation or reduction, never by loosening a ceiling** (the
pilot's rule — recalibrating a ceiling to fit a file the author wrote is the forbidden
quota-override, and implement is the file most likely to want it):

1. **Constraints opened at 945/810 (G=7).** Resolved by restoring **two source decision points to
   gate form** — the pre-rewrite Phase 0 carried *step 1* (capture and resolve `<feature>`) and
   *step 2* (the plan-package-complete entry gate) as distinct steps with distinct evidence and
   distinct decisions, and *step 3* (governance) as a third. Collapsing all three into one `G1`
   line, as the first draft did, hid two real gates and cost the ceiling 180 words it was entitled
   to. G=8 → 900. The remaining 45 words came from the reductions in the ledger's *dropped* rows.
   The block finally lands at **796/900 (88%)** once the seat-recycling revert removes its 88 words —
   so the trims that bought its headroom stand, and the block is no longer the file's tightest.
2. **Bindings opened at 279/168.** Two bullets in the colliding draft (below) were
   **Constraints-class content parked in Bindings** — the governance obligated-read brief and
   project scaffolding are obligations, not referents. Both moved to their own class: governance to
   its gate line, scaffolding to an invariant.
3. **`·` separators counted as words** in the mechanical count (they are whitespace-delimited
   tokens, as in the pilot's own measurements). Two Bindings list lines were re-punctuated to commas
   (−9 tokens). **Recorded because it is a formatting change, not a content cut** — the auditor
   should treat those two lines as unreduced.

## [v0.35.0] The phase body and the Contract section retired into the five-block anatomy
- **Disposition:** superseded → the goal-shaped anatomy. `Team-form parameters` → the preamble's
  probe-seat line (the env check and transport mechanics are shape Layer 2, referenced) ·
  `Session constraints` → the package gate, the bounds' kill-switch, and Bindings' deliverable /
  ID-namespace lines · `The seats` → the **Seats & checks** table plus the validation-model line ·
  `Phase 0` → **G1** + the **package gate** + the **governance surface** + Bindings' design inputs +
  the scaffolding and slice-scope invariants · `Phase 1` → the **cycle checkpoint** + the
  **architecture deviation** gate + the ordering invariants · `Phase 2` → the ordering invariants'
  lead-routed final validation, the bounds' fix-pass cap, and Bindings' diff report · `Phase 3` →
  **G5** · `Phase 4` → the KM-landing binding + the Recovery table's `accepted` row · `Contract`'s
  four clauses → **Goal** (done-condition + not-done states), the **Seats & checks** table
  (producer↔validator), **Constraints** (bounds + gates) · `State recovery` → **Recovery**.
- **Tier failed:** n/a — supersession by ruling (**CS-D3** condition-first documents · **CS-D4** the
  connective procedure is deleted and what survives is restructured · **CS-D5** the five-block
  anatomy and the Contract-as-document inversion).
- **Content:** eleven `## Phase`/`## Contract`/`## State recovery`/roster sections, ~2,600 words of
  ordered procedure, appendix, and footer. Not reproduced verbatim — every *rule* inside them is
  resolved individually in the ledger below, and the deleted remainder is connective narration
  (step numbering, `Phase N step M` cross-references, per-phase restatements of the lead's job, and
  the reachability sentences that opened each phase). Recoverable in full at
  `git show 7898d86:plugins/mochiko/commands/implement.md` — the authoritative pre-wave baseline,
  292 lines / 3,230 w / 23,873 B (byte-identical at `c47684d`; the intervening step-1/step-2 commits
  landed shape v5 and the plan pilot without touching this file).
- **Kept deliberately:** every gate, bound, predicate, routing decision, trigger, ordering rule and
  artifact binding — resolved row by row in the CS-D8 ledger.

## [v0.35.0] The `What you own (not the seats)` footer deleted
- **Disposition:** deleted.
- **Tier failed:** 1 — a declared duplicate, and implement carried the longest instance in the
  library (**~190 words**) restating the cycle sequence, the round counters, the execute→verify
  pairing, the verdict ownership, the devolved-branch clearing, the deviation escalation, the diff
  trigger, the fix-pass bound, every gate, the entry and governance prerequisites, scaffolding, and
  the never-mount-verification rule — each of which is now a Constraints line, a Seats cell, or a
  Bindings entry. The v0.17.0 wave already deduped this class once (*Verdict-ownership
  triplication*) and it grew back; the anatomy leaves it nowhere to hide.
- **Kept deliberately:** the one clause with no other home — "verifying each seat actually wrote its
  expected files (a missing output → log and ask retry/abort)" — is **not** dropped as behavior: it
  survives as the Recovery block's evidence-driven resume (a missing report *is* a resume row: two
  rows key on "no `cycle-report.md` this round" and "no verification report this round") plus G4's
  escalation menu. Same disposition the pilot gave the identical clause; the anatomy's Recovery
  block is the structural-prevention claim, and it is checkable in the table.

## [v0.35.0] Seam-N1 narration replaced by two seat rows
- **Disposition:** superseded → structure. The `architecture scribe` roster entry's ~55 words of
  "**two distinct firing conditions**, kept separate (seam N1)" prose, plus the two mid-body
  "seam N1 — distinct from …" reminders, are replaced by **two rows in the Seats table**:
  `arch-diff` (spawn: at final validation, whenever an approved structural delta existed) and
  `arch-scribe` (spawn: at finalize, on a built structural change only).
- **Tier failed:** 1 (altitude) — the seam was being *asserted in prose* three times because the
  single row could not show it. Two rows make the broad/narrow trigger split mechanically visible in
  the parameter the shape already provides for it (P5's spawn column), which is what the v0.32.0
  build note asked the build to resolve.
- **Kept deliberately:** both triggers, in their exact breadth — the diff fires on
  *approved-delta-existed* (independent of what was built, so a silently-descoped approved delta
  cannot escape) and the fold on *built structural change*; the diff's report reaching **G5**; and
  "never the verifier seat", stated once on `arch-diff` and shown by the table for both rows.
- **Deliberate call, flagged:** both rows are `principal-architect` × `authoring-architecture`, so
  the pair appears twice. Check 7 is satisfied — neither row grades an artifact it authored, and the
  two rows touch different artifacts (`architecture.md` + built code for the diff; `ARCHITECTURE.md`
  for the fold) — but the repetition is deliberate and worth the grader's eye.

## [v0.35.0] Skill- and shape-owned content stripped from the command body
- **Disposition:** relocated → the homes that already carry it (no new home written).
- **Tier failed:** 1 (altitude).
- **Content:**
  - `input, never the gate` — stated **three times** (roster, Contract done-condition, footer). Home:
    `command-shape.md` Layer 2 *Clearing*, and it is check 8's keyed marker. Survives once, as the
    validation-model line's "qa's output is **lead-adjudicated input** and the verdict is yours" —
    the pilot's audit-cleared phrasing.
  - "Disjoint agents, disjoint skills, structurally separated" (Contract, Producer ↔ validator).
    Home: Layer 2 *Independence by structure*, which states both phrasings; the table *shows* it.
  - "a verifier respawn is cold by design" (State recovery preamble). Home: Layer 2 *Independence by
    structure* — "a respawn is cold by design". Recovery keeps only what respawning *re-reads*.
  - "Never modify git or push" (Phase 4). Home: Layer 1 *Ground rules*.
  - "out of rounds = escalate, never done" (Contract bounds). Home: the shape's Constraints block.
    **Contested-adjacent:** the audit-PASSed pilot keeps this sentence, so keeping it was permitted;
    it is cut here because it is verbatim shape prose and Constraints needed the words. The
    semantics survive in the Goal's not-done state "out of rounds".
  - "drift caught one cycle deep, never deferred to landing" (Phase 1 / footer) and "the same
    mechanism as plan's design-time return to sign-off". Home: this note's v0.32.0 entry — design
    rationale and a cross-command provenance pointer, both of the class the pilot relocated out of
    Constraints under audit pressure.
  - "Round reports are cleaned by default; never offer to delete a deliverable." **This line never
    existed in implement** — it entered from the colliding draft (below), imported from `plan.md`.
    Removed as an import, not stripped: implement's cycle reports are the audit trail **and** the
    input a recycled producer is briefed from, so a default clean would break the seat-recycling
    binding.

## [v0.35.0] CS-D8 survivor re-grade ledger — every protected line resolved

CS-D8 (extended by user ruling U4) protects `KEPT:`/Tier-2-evidenced lines **and** every line
traceable to a `DECISIONS.md` row. implement carries **no `KEPT:` survivor-provenance entries**; its
protection set is the *Kept deliberately* fields of the four prior entries (v0.17.0 conversion,
v0.31.0 ×2, v0.32.0) plus the DECISIONS row trace, grepped before any cut. **All 26 rows survive
translated; one is superseded with grounds; zero dropped.** Per the pilot's warning that losses hide
in *compressed evidence clauses* rather than deleted sections, the devolved-branch predicate and the
deviation triggers were re-read clause by clause against `git show c47684d` after the last edit.

| protected line | source | resolved |
|---|---|---|
| AD-D6.1 — the approved `architecture.md` is **briefed input**, read at entry and carried in the producer's per-cycle brief | DECISIONS row AD-D6; v0.32.0 | Bindings' design inputs, marked "the **anchor** for both architecture mechanisms"; the package gate's evidence; the producer seat row's deviation self-check |
| AD-D6.2 / R7 — the **diagram-anchored mechanical test**, verbatim: add/remove a box · add/remove/redirect an arrow · move a responsibility across a boundary | DECISIONS row AD-D6; v0.32.0 | The **Architecture deviation** gate's evidence clause, all four triggers intact |
| AD-D6.2 — the self-check runs at **cycle open AND cycle close** | v0.32.0 (emphasised there) | Same gate line: "run at cycle **open** and cycle **close**" — the conjunction kept, not compressed to "each cycle" |
| AD-D6.2 — a surfaced deviation **stops and surfaces**, never silently built; the user re-rules; the target is **amendable mid-implement with consent**, updated before the cycle resumes | DECISIONS row AD-D6 | Same gate line's rules/decides clauses + "never silently built" |
| AD-D6.3 / R8 — the **built-vs-approved diff** fires whenever an **approved structural delta existed** (broad, independent of what was built), at final validation, in **diff mode** | DECISIONS row AD-D6; v0.32.0 | `arch-diff` seat row (skill + diff mode + spawn trigger) · **G5**'s evidence · the Goal's end state and not-done state · a Recovery row |
| AD-D6.3 — the divergence **surfaces at the acceptance gate** | DECISIONS row AD-D6 | **G5** evidence: "**and** the built-vs-approved architecture result where an approved delta existed" |
| Seam N1 — the diff's broad trigger vs the fold's narrow **built-structural-change** trigger, kept separate | v0.32.0 (build-seam resolution) | Two seat rows + the KM-landing binding's "a **built** structural change" (entry above) |
| Team-method D3 — the **cycle** is the clearing unit and the devolved branch applies to it | DECISIONS row; v0.31.0 | Bindings' **Clearing unit + checkpoint keying** (P14) + the cycle-checkpoint gate line |
| Team-method D3 — the predicate, **exactly**: every verification a deterministic CLI check at **100% pass** AND no deviation reported AND `domain_deps_added` empty | v0.31.0 | Cycle checkpoint, with "skipped **exactly** when" and all three conjuncts — the compressed-evidence-clause check's primary target, re-read against the prior text |
| Team-method D3 — a clean cycle clears on qa's **PASS-with-evidence, unread by the lead**, counted from its clearance notice | v0.31.0 | Same line's devolved clause; the Goal's "every **escalated** cycle's reports" carries the not-re-read consequence |
| Team-method D3 — **everything else returns to the lead**: any failure · any GUI or subjective verification · any reported deviation · any registry addition | v0.31.0 | Same line's "Otherwise it fires — …", all four classes enumerated |
| AD-D6 × D3 fold — a surfaced architecture deviation **is** a reported deviation and **de-devolves** the cycle (no parallel gate) | v0.32.0 | Bindings' P14 line ("a surfaced architecture deviation **de-devolves** it") — kept out of the gate line so the fold is stated once |
| Domain-allowlist D2/F2 — the `domain_deps_added` **visibility floor**: disclosed in the cycle report and surfaced at the checkpoint | DECISIONS row (2026-07-21) | Cycle-checkpoint evidence + Bindings' uncertainty carrier (the field named in both) |
| Domain-allowlist — the **confidence-gate hook**: a registry addition at `production`/`regulated` **forces** the human checkpoint regardless of deterministic-CLI pass; lower tiers surface non-blocking | DECISIONS row | **Superseded by v0.36.0** (the entry above): always forces, no stamp read — this row records the v0.35.0-era keying |
| v4 mesh D1/D2 — verifier **cold at the first cycle verification**, standing after; the producer↔verifier **peer edge** declared on the roster | v0.31.0 | Verifier seat row (spawn + peer-edge cells) |
| v4 mesh — the **endgame is lead-routed**; the devolved branch clears cycles, never the final validation | v0.31.0 *Kept deliberately* | Ordering invariants: "The final validation is lead-routed, never devolved" + the verifier row's peer-edge cell |
| v4 mesh — a **retry is lead-dispatched** (a retry follows a failure; the verdict on a non-clean unit is the lead's) | v0.31.0 *Kept deliberately* | Producer seat row: "retries and fix passes are dispatched by you" |
| Delivery is a hand-off, not a start signal — the pairing is the lead's to enforce | v0.31.0 / v4 mesh | Ordering invariants: "the hand-off is peer-routed, the pairing is yours to enforce" |
| Standing-seat **D1/D2** — conditioned checkpoint recycling: cycle floor (~≥3) + gate-pause check, same-name successors, artifact-only respawn briefs, just-failed report relayed at dispatch, verifier per slice | DECISIONS row (2026-07-23) | The **Seat recycling** invariant + the producer/verifier spawn cells. **An addition, not a translation — see the contested call below.** |
| The **`@`-reference recovery** — empty `$ARGUMENTS` has a **named cause** (the `@`-reference drop bug) and a **two-option prompt** (re-enter, or confirm the detected feature) | Pilot fix-round restore; record §7 protected set | **G1**, cause and both options intact, plus the detection rule (explicit ID, else the most recent in-progress feature). The pilot lost this once in a compressed evidence clause; written first here and re-checked last |
| Roadmap-v2 — **sequential implement**; parallel cycle execution is a `deliberate-shortcut-ledger` deferral, **not a capability drop** | DECISIONS row (2026-06-27); BACKLOG parallelism item | Ordering invariants, both clauses |
| Foundation cycles before feature cycles; current cycle = the first with unchecked tasks | current body | Ordering invariants |
| The **execute→verify pairing** — every produced cycle verified in the same round, **never skipped** | current body / v0.17.0 | Ordering invariants |
| The verification skill is **never mounted on staff**, and staff never grades its own cycle | v0.17.0 *Verdict-ownership* strip (deduped to once) | Validation-model line, once: "The verification skill is **never** mounted on the producer" — the second half is shown by the table, not asserted |
| Bounds — targeted retry **max 3/cycle** re-opening only the failed tasks and never regressing passing code · fix pass **max 3**, failure-scoped, **unconstrained by cycle boundaries** · convergence stall at **2+ rounds** · no-progress = an unchanged failing set · `IMPLEMENT_STOP` checked **before each seat send** | v0.17.0 + current body | The **Bounds** line (caps, stall, no-progress, kill-switch) + the producer seat row (the fix pass's cycle-boundary freedom) |
| **No G2** — a single verifier, so no feasibility-rejection gate (the audit-passed reword) | v0.32.0 (preserved there) | Validation-model line, where it now *follows from* the single-verifier fact rather than sitting as a standalone note |
| Slice binding 1 — the package gate and the cycle loop read `slices/<slice>/tasks.md` | v0.17.0 slice strip, *four genuine bindings kept* | Slice-scope constraint |
| Slice binding 2 — design inputs = shared feature-root artifacts **plus** `slices/<slice>/{plan.md, task-mapping.md}` | same | Slice-scope constraint |
| Slice binding 3 — per-slice outputs land under `slices/<slice>/`, and what that does to the artifact set | same | Bindings' **Reports** line ("or `slices/<slice>/` when slice-scoped, where the Goal's artifact set reads them") — moved out of Constraints to the block that owns paths |
| Slice binding 4 — the quality gates run the **full repository suite** (implement's own operationalization of the contract's regression-safety rule) | same | Slice-scope constraint, with the regression-net reason kept |
| Slice binding 5 — at the last slice the **feature is declared, not verified**; Feature-Done executes at feature-close, owned by no workflow; surface it, never report feature completion | same (implement is the pipeline's terminal stage) | Slice-scope constraint's closing clause |
| The **Graduation contract** is the single home; do not restate it | v0.17.0 audit catch (the D1 churn liability) | Slice-scope constraint opens by naming it as the single home for the six rules and restates none — the defect that entry was written about is not reintroduced |
| Vertical-graduation — the slice-scoped entry variant | DECISIONS row (2026-07-02) | The Slice-scope constraint + Bindings' per-slice report layout |
| Governance prerequisite — surface a missing region (offer `/mochiko:setup`), **never auto-resolve**; governing context, not a blocking gate; present → the **one-line obligated read** of the `paths`-relevant `.claude/rules/mochiko/` files in each code-touching brief | current body / setup-cluster rulings | The **Governance surface** gate, all three parts |
| Workspace-as-state, **no registry field** | v0.17.0 (named a genuine survivor there) | Bindings' design-inputs line |
| The producer's **honest** `cycle-report.md` is the producer-authored uncertainty carrier, not confidence marks | current body (P11) | Bindings' uncertainty carrier, with `mochiko:executing-tdd-cycle`'s format referenced as the field owner |
| KM landing under fix-on-sight; implement records **what it builds** | v0.32.0 / KM invariants | The KM-landing binding. **Superseded with grounds:** the "implement records what it builds" half is deduped — `plan.md`'s KM binding already states the division ("Plan records only what plan itself established … implement records what it builds"), so it survives stated once, in plan. The *behavior* (the fold fires on built structural change) is unchanged here |

**One routing correction, recorded rather than folded silently.** The prior body's line "Route
knowledge / preference / scope gaps per `loop-discipline` (→ **G3** / **G4** / escalate)" read as
*preference → G4*, which contradicts `loop-discipline`'s own routing table (knowledge → research;
**preference → the human gate**; scope → halt or split, i.e. the escalation gate). Translated to the
doctrine's mapping: **G3** rules the preference gap and routes knowledge to `Explore`; **G4** takes
the scope gap. Flagged because the compressed original is ambiguous enough that a reader could call
this a behavior change rather than a correction.

## [v0.35.0] `RETURNED:` — the seat-recycling binding stays unbuilt (contested call, user-ruled out)

**Raised as a contested addition, ruled out by the user at wave ratification, and reverted in the
same version.** Recorded in full because the ruling that keeps it unbuilt is the useful artifact —
standing-seat **D1–D4** remains ruled, and this entry is where the next builder finds why the command
does not yet carry it.

**What was raised.** The wave brief listed "producer checkpoint-recycling (cycle floor + gate-pause
check, same-name successors)" among implement's protected, DECISIONS-traceable surface. It **is**
ruled (`DECISIONS.md` 2026-07-23, standing-seat lifecycle **D1–D4**) but it was **never built into
`implement.md`** — a grep of `git show 7898d86:plugins/mochiko/commands/implement.md` for `recycl`
returns zero, and the file instead declared the pre-D1 claim, "one **named standing seat** across the
whole cycle sequence and the Phase-2 fix-pass loop". `BACKLOG.md` carries the build open:
"**Standing-seat build items (deferred)** — conditioned checkpoint recycling · respawn briefs from
artifacts · the Layer-2 transport-vs-lifecycle rewrite (**v4+**) · per-seat measurement."

**Authored in the include direction** (an 88-w **Seat recycling** invariant plus "recycled per
Constraints" / "recycled per slice" in the two spawn cells), on the ground that the pre-rewrite text
was stale against a ruling, and **flagged rather than silently resolved**.

**Ruled out. Grounds, as given at ratification:** the wave's contract is **translation under
true-reductions-only** accounting, and this was its only line of new behavior; D1's cycle floor is
**probe-calibrated and the D4 probe is deferred**, so the command would carry an approximate
threshold; standing-seat **D3** — the Layer-2 *transport vs context-lifecycle* reframe that would
give each command a per-seat lifecycle `[PARAM]` — is **unbuilt** (v4+, open in BACKLOG, and
`.mochiko/strips/command-shape.md` names it as deliberately not combined into the mesh revision), so
the invariant was a workaround for a missing shape slot rather than a home; and the BACKLOG item
remains the build's proper home.

**Reverted, exactly as costed.** The `Seat recycling` invariant deleted (88 w); the producer's spawn
cell back to "standing across the cycle sequence and the fix-pass loop; **probe seat**, foundation
cycle 1"; the verifier's back to "cold at the first cycle verification, standing after". Measured
after: Constraints **796/900** (projected 796 — exact), Seats & checks **271/280**, file **1,868 w /
13,331 B** — 6 w under the ~1,874 projection, because the two spawn cells shed "recycled per
Constraints" / "recycled per slice" on top of the 88-w invariant. **The ruling is acknowledged here
and unbuilt by design** — the
standing-seat claim in the seat rows is therefore known-stale against D1, not an oversight.

**Never added, so the absence is not read as a drop:** D1's user escape hatch (the user may order a
recycle at any gate), D2's artifact-only respawn briefs and their sufficiency watch-item, and the
relay of the just-failed `cycle-report.md` at dispatch. All lifecycle policy with no consumer in the
goal-shaped file; they stay in the record until the D3 reframe lands. **Re-add trigger:** the
standing-seat build items shipping — the D3 Layer-2 reframe first, since it supplies the `[PARAM]`
this content belongs in.

## [v0.35.0] Collision note — an unledgered orphan draft occupied the working tree

**Baseline provenance, stated because it was briefly in doubt: this rewrite and its ledger derive
from HEAD**, not from the working tree. While the wave was in flight a since-terminated seat,
executing a superseded instruction, overwrote the working-tree `implement.md` at ~23:26 with a
**different** goal-shaped draft (1,934 w / 13,919 B, never committed, no strip entry). It was read
in full before being replaced and is snapshotted lead-side; it is an unledgered orphan and carries
no authority — **reference seed at most.** Every row of the CS-D8 ledger above was re-derived
against `git show 7898d86:…`.

It was also **not** conformant, which is why replacing it rather than extending it was the cheaper
path: measured against the grader's floor it failed check 6 on **four of six blocks** (Goal 155/150 ·
Seats & checks 258/235 at S=3 · **Constraints 792/630** at G=5 · **Bindings 279/168**), and check 13
on the architecture deviation, which it carried as a plain bullet with no
`evidence:`/`rules:`/`decides:` triple despite the body relying on the user re-ruling a surfaced
deviation.

**Three elements it shares with this file are independently HEAD-traceable** — verified line by line,
so nothing here rests on the orphan: the `No G2` note (HEAD:247, folded onto the validation-model
line here), "Neither ends the loop on its own" (HEAD:187), and `domain_deps_added` in the cycle
checkpoint (HEAD, 3 occurrences). The orphan influenced phrasing and placement only. **Two of its
calls were rejected:** the imported report-cleaning binding (see the strip entry above) and the
Constraints-class content parked in Bindings.

## [v0.32.0] Build note + shape-v4 re-conform — implement honors the approved architecture (AD-D6; 2026-07-30)

Design record: `.mochiko/brainstorms/architecture-design-primitive/record.md` (AD-D6 with folds R2/R7/R8,
seam note N1). Not a strip — **additions** (recorded in `DECISIONS.md` row AD-D6, lead-owned landing);
logged here with the version stamp for the audit trail and to name the seam-N1 resolution the record left
to build.

> **Version note:** originally stamped **v0.30.0**; while in flight, origin/main released **v0.30.0** and
> **v0.31.0** (the shape-v3→v4 mesh + devolved-cycle rewrite, the two entries below). The merge rebased
> these AD-D6 additions onto v4, so they land at **v0.32.0** and fold into v4's devolved branch (see the
> re-conform bullet).

- **Briefed input (D6.1):** the approved `architecture.md` joins the design inputs read at Phase 0 step 4
  and is added to the producer's per-cycle brief — it is the **anchor** for the two new mechanisms below.
- **Deviation escalation (D6.2 + R7) — the diagram-anchored mechanical test:** "does this cycle add/remove
  a box, add/remove/redirect an arrow, or move a responsibility across a boundary on the approved diagram?"
  — **self-checked by the producer at cycle open AND cycle close**, reported in `cycle-report.md` and
  surfaced at the cycle checkpoint (Phase 1 step 3). The user re-rules and the approved target is
  **amendable mid-implement with consent** (a consented target amendment updating `architecture.md`, the
  same mechanism as plan's design-time return to G3). Drift caught one cycle deep, never deferred to landing.
- **Built-vs-approved landing diff (D6.3 + R8) — new build capability:** at final validation (Phase 2 step
  3), when an **approved structural delta existed** in `architecture.md`, the `authoring-architecture`
  dispatch runs in **diff mode** (approved target + built code → "built as approved" or the divergence). The
  divergence is surfaced at the **G5** acceptance presentation. This is a *new* capability (R8 — the prior
  `authoring-architecture` only wrote prose from built code); assigned to that dispatch as a named build
  item, taking the approved artifact as input.
- **Seam N1 made explicit (the record's carry-forward):** the `authoring-architecture` dispatch now has
  **two distinct firing conditions**, kept separate at build — the **diff** fires on *approved-delta-existed*
  (broad, independent of what was built, so a silently-descoped approved delta cannot escape both mechanisms),
  run at final validation to reach the G5 decision; the **`ARCHITECTURE.md` fold** fires only on a *built
  structural change* (narrow, the KM writer moment), at Finalize. An approved-but-not-built delta triggers the
  diff without forcing a doc update. **Placement resolution (build decision):** the record has the diff "at
  landing" yet its divergence "surfaces at implement's acceptance," and acceptance (G5, Phase 3) precedes the
  Finalize landing (Phase 4) — resolved by running the diff at final validation (Phase 2, end) so its report
  is available at G5, while the doc fold stays at Finalize. Flagged as a build-seam resolution the record
  deferred (N1).
- **Shape-v4 re-conform (the merge work, this task):** the AD-D6 additions were re-applied onto main's
  v4-conformed implement (the two v0.31.0 entries below) rather than the v3 confidence gate they were first
  written against. The fold: v4 replaced the confidence gate with the **per-cycle checkpoint carrying the
  devolved branch** (a deterministic-CLI-100%-pass + no-deviation + empty-`domain_deps_added` cycle clears on
  qa's PASS-with-evidence, unread by the lead). The architecture deviation self-check **integrates as a
  reported deviation**: a surfaced deviation is a `cycle-report.md` deviation, which **de-devolves the cycle**
  (removing it from the clean branch → lead checkpoint + consented-target-amendment decision) — so the
  deviation rides v4's existing "any reported deviation returns to the lead" rule rather than adding a
  parallel gate. The built-vs-approved diff (Phase 2 step 3) sits on the **lead-routed endgame** (the devolved
  branch clears cycles, never the endgame), consistent with v4's "Clearing under the mesh". Verify hand-off is
  peer-routed (producer→verifier) per the mesh.
- **Consequent edits:** Phase 0 entry gate retargeted to `/mochiko:plan` (the package producer) after the
  `/mochiko:tasks` retirement (see `strips/tasks.md` v0.32.0); done-condition gains clauses **(4)** (the diff
  ran when an approved delta existed) and **(5)** (G5 cleared), atop v4's clause (3) (lead reads escalated
  cycles + final validation only); the per-cycle checkpoint predicate, G5 presentation, state-recovery table,
  and the "What you own" footer updated to carry the deviation check + the diff. The audit-passed "No G2"
  reword ("there is no feasibility-rejection gate") is preserved. **No shape gap** — both mechanisms are
  per-workflow gates/steps folded into v4 doctrine, not a shape revision; shape stays **v4**.

## [v0.31.0] Lead-as-switchboard routing superseded by the in-loop mesh (shape v4 conforming edit)
- **Disposition:** superseded → `templates/command-shape.md` v4 (Layer 2 — "Independence by structure" + "In-loop mesh"). Rewritten in place at command altitude: the verifier is still cold-spawned at the first cycle verification (a spawn-timing parameter), the producer↔verifier peer edge is now declared on the roster, and the doctrine stays in the shape.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/team-method-vs-command-shape/record.md` **D1**, scoped by **D2**), not a minimalism strip. Permanent no-contact was the falsified claim; cold *arrival* survives as a property of the stage.
- **Content (superseded, verbatim):**
  - seat roster: "spawned **cold at the first cycle verification**, never in contact with the producer"
  - Phase 1 step 2: "**Verify — same round, never skipped.** Message the verifier to verify the cycle against real infrastructure"
  - Contract, Producer ↔ validator: "(verifier cold-spawned at the first cycle verification, evidence/reports lead-routed, no producer↔verifier contact)"
- **Kept deliberately (not superseded):** Phase 2 step 1's lead-routed final validation — the endgame is the lead's under v4, now stated rather than left implicit; and Phase 1 step 1's lead-dispatched targeted retry — a retry follows a failure, and the verdict on a non-clean unit is the lead's.

## [v0.31.0] The clean-cycle verdict devolves to the producer↔verifier pair (shape v4 conforming edit)
- **Disposition:** superseded → `templates/command-shape.md` v4 (Layer 2 — "Clearing under the mesh"). implement supplies the parameters: the **cycle** is its clearing unit, and the escalated branch's checkpoint keying is a `production`/`regulated`-tier domain-registry addition.
- **Tier failed:** n/a — supersession by ruling (record **D3**), not a minimalism strip.
- **Content (superseded, faithfully compressed):**
  - Phase 1 step 3 header + read: "**Confidence gate + verdict (you).** Read `cycle-report.md` + the verification report + qa's evidence." — the lead read every cycle, clean deterministic ones included; those now clear unread.
  - Phase 1 step 3 branch: "if every verification is a deterministic CLI check that passed 100%, **auto-approve** and advance to the next cycle" — the auto-approve was the lead's act; it is now the pair's, on qa's PASS-with-evidence.
  - Contract done-condition (3): "*you* Read the cycle-reports + verification reports" → the final-validation report plus every **escalated** cycle's reports; "qa's status is input, never the gate" gains "wherever judgment exists".
  - Contract human gates: "the **confidence gate** (per cycle: deterministic CLI verifications that 100% pass → auto-approve; GUI / subjective / any-failure / a `production`+-tier domain-registry addition → human checkpoint)" → restated as the per-cycle checkpoint carrying the **exact skip predicate**, per shape v4's Contract requirement.
  - "What you own": "the verdict against the default-FAIL done-condition (qa grades from real infrastructure, you Read the cycle-reports + verification reports and decide …)"
  - frontmatter `description:`: "with a confidence-based per-cycle gate"

## [v0.17.0] Conversion note (D2/S4 — one-shot → team-form, 2026-07-19)

- **Command-specific rationale (user-ratified):** implement runs a producer↔verifier cycle across a
  **variable-length cycle sequence** (foundation cycles before feature cycles, each execute→verify in
  the same round, targeted retry ≤3/cycle) then a **final-validation + fix-pass loop** (≤3 passes) — the
  **longest producer horizon of any converted command**: not two fixed phases but *N* cycles plus fix
  passes over a **codebase that accumulates as it goes**. The context-retention bet is implement's own
  and is its strongest: a **standing producer seat** (`staff-engineer`) carries (1) the conventions the
  foundation cycles set forward into the feature cycles (the brownfield "follow existing patterns"
  consistency, now *within its own* growing implementation), (2) whole-implementation knowledge into a
  **fix pass that is unconstrained by cycle boundaries** (it may touch any cycle's files — a cold spawn
  would rebuild the entire implementation's mental map from disk), and (3) targeted-retry coherence (it
  re-opens only the failed tasks of code it wrote). The verifier maps to a **standing qa seat**: cold at
  the first cycle verification, then messaged once per cycle and again for the whole-implementation final
  validation — its retained per-cycle context is what makes the final validation informed by what it
  already checked rather than a cold whole-repo read. The verifier never contacts the producer, and the
  verification skill is never mounted on staff — independence stays structural. Transport rides the v3
  fix (`agent-dispatch.md` Seat transport + addressability probe on the producer's first spawn, the
  foundation-cycle-1 implement).
- **Steelman recorded (user-ratified with the conversion):** zero successful team-form runs at
  conversion time (two setup defect runs; specify's, slice's, plan's, and tasks' own checkpoints all
  unfired; brainstorm v2 measured standing seats *more* expensive than dispatches). Implement is
  **two-seat** (nearer tasks'/slice's cost than plan's three-seat load), so its team-form tax is moderate
  if the retention payoff doesn't land. Two honest weak points. First, **implement's producer craft is
  specifically built to reconstruct context from disk**: `brownfield-integration`'s whole discipline is
  "read the full file first, identify its conventions, follow them" — so a cold per-cycle producer is
  *designed to be safe* re-reading the accumulating code, and the retention payoff is narrower than the
  raw cycle count suggests (it is the *authorial judgment* — why a pattern was chosen, what scope
  discipline deliberately left out — which the `cycle-report.md` records as prose but a cold reader must
  re-derive, not the code itself, which is fully on disk). Second, **the qa seat is the weaker team-form
  fit** (implement's analogue of plan's fire-once architect): its verification is **Tier-1 deterministic**
  — real-infra evidence + quality-gate exit codes, re-run fresh each cycle, and the final validation
  re-runs the full suite regardless — so a cold-respawned verifier would reconstruct almost nothing;
  modeled as a standing seat messaged per-cycle for uniform transport, its persistence buys the least of
  the two seats. Ruled team-form anyway per D2's declared default + S4 (no prior dogfood evidence
  required; checkpoint below).
- **Confirm-or-revert checkpoint:** the first post-conversion dogfood run (the open "Dogfood
  `/mochiko:implement`" BACKLOG item, Implement-port follow-ups) confirms the conversion or reverts it to
  one-shot Layer-1 form; a revert is logged as a `RETURNED:` entry here. Team-form named checks: the
  producer probe fires the addressability check (the foundation-cycle-1 implement); the standing producer
  seat is messaged (not respawned) across cycles, across targeted retries, **and across the cycle→fix-pass
  boundary** (whole-implementation knowledge carried into a cross-cycle fix pass); the verifier spawns
  **cold at the first cycle verification**, is messaged once per cycle and for the whole-implementation
  final validation, and **never contacts the producer**.

## [v0.17.0] Sound-loop paragraph + four-requirement enumeration
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, One lead) + the
  `mochiko:loop-discipline` reference
- **Tier failed:** 1
- **Content:** "This is a mochiko **sound loop**: invoke **`mochiko:loop-discipline`** and honor all four
  requirements (default-FAIL done-condition, independent validation, bounded iteration, named human
  gates), and brief each dispatch per **`agent-dispatch`**. Those rules are not restated here — this
  command states only what is specific to *this* workflow: the cycle sequence, the execute→verify
  pairing, the retry / fix-pass bounds, and the two implementation gates." — restated loop-discipline's
  own enumeration; the workflow-specific tail survives as the converted goal + the sections themselves.

## [v0.17.0] Per-run contract fill (`workflow-contract.md` → `implement-contract.md`)
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Contract — the authoring-time-fill
  rule); the per-workflow values survive as the command's authoring-time Contract section (implement's are
  a four-part done-condition, the targeted-retry / fix-pass / convergence-stall bounds, and the confidence
  gate + G5 + G1/G3/G4 + the no-G2 note)
- **Tier failed:** 1 (the shape retired per-run fills whose values are constant at authoring time)
- **Content:** "## Contract parameters (fill the artifact — don't inline it) … Fill
  `templates/workflow-contract.md` → `.mochiko/specs/<feature>/implement-contract.md` with the values
  below, then confirm it against `mochiko:loop-discipline`. The filled artifact is the inspectable proof —
  not this command body."

## [v0.17.0] Verdict-ownership triplication
- **Disposition:** deduped to once (the Contract's Done-condition / Producer↔validator clause; the
  qa's-status-is-input boundary also lives on `qa-engineer`'s persona + REGISTRY's "independent Tier-1
  validator" row). The per-phase Verdict *steps* (Phase 1 step 3, Phase 2 step 2) are workflow mechanics
  and survive.
- **Tier failed:** 1
- **Content:** stated three times pre-wave — the lead framing ("qa presents evidence and a checkpoint
  recommendation; **you own the clearing verdict** … qa's status is input, never the gate"), the Contract
  Team clause ("verifier `mochiko:qa-engineer` … never implements … the verification skill is never
  mounted on staff"), and the footer ("the verdict (qa grades from real infrastructure, you Read the
  cycle-reports + verification reports and decide against the default-FAIL done-condition … qa's status is
  input)").

## [v0.17.0] Footer ground rules + Task-tool transport line
- **Disposition:** kernel-free/git relocated → `templates/command-shape.md` (Layer 1, Ground rules); the
  "always dispatch via the Task tool" line superseded by the team-form conversion (transport now per shape
  Layer 2 + `agent-dispatch.md` Seat transport)
- **Tier failed:** 1
- **Content:** "Stay kernel-free; brief agents per `agent-dispatch`; always dispatch via the Task tool
  (never inline agent behavior); do not modify git or push."

## [v0.17.0] Recovery memory-model parenthetical
- **Disposition:** relocated → `templates/command-shape.md` (Layer 1, Recovery — "never a context `phase`
  field")
- **Tier failed:** 1
- **Content:** "Resume from workspace evidence (there is no context-file `phase`/`status`)" + the
  entry-gate parenthetical "(workspace evidence — there is no context-file `status` to read)". The
  recovery table (evidence → resume-at) is the workflow-specific Recovery PARAM and survives, as does
  Phase 0 step 4's "workspace-as-state, no registry field" (a genuine survivor, as in the siblings).

## [v0.17.0] "Why this done-condition differs from HIL's" blockquote
- **Disposition:** deleted (user-ratified)
- **Tier failed:** 2 (no behavior produced — historical/motivational provenance; preserved in ROADMAP's
  Decision Trail + `.mochiko/transform/implement/`)
- **Content:** "> Why this done-condition differs from HIL's: HIL declared "no hard caps," routed on an
  autonomously-evaluated gate verdict, and had **no** final-acceptance gate — it could churn indefinitely
  or self-declare done. The deterministic caps, the lead-owned verdict (qa's status is input), and the new
  G5 acceptance gate close the gates HIL lacked." — the shape of specify's / plan's / tasks' deleted
  HIL-comparison blockquotes; its rationale is carried by the Contract done-condition (the deterministic
  caps + lead-owned verdict + G5), so no unique behavior is lost.

## [v0.17.0] Slice-scoped entry — de-restated to the Graduation-contract reference
- **Disposition:** relocated → `templates/slices-template.md` (the **Graduation contract** section — the
  single home of the consumption rules); Phase 0 step 6 now *applies* the contract by reference for slice
  resolution, the staleness guard, scope, extend-mode, graded amendment, and artifact layout
- **Tier failed:** 1 (the one-shot entry variant declared the Graduation contract "the single source of
  the consumption rules; do not restate it" and then restated slice-resolution + staleness-guard rules
  beneath that self-declaration — the same D1 churn liability the plan wave's `validation-command-shape`
  audit caught on plan's identical entry and the tasks wave de-restated in-conversion; applied here
  proactively by that prior ruling, **NOT contested**)
- **Content:** the copied rules — slice resolution ("named in `$ARGUMENTS`, else the first slice in
  Slice-order whose `slices/<slice>/tasks.md` has unchecked tasks") and the **staleness guard** ("the live
  `spec.md` story-ID set must match the Spec stamp — mismatch → block and point to `/mochiko:slice`").
  implement's genuine own bindings were **kept**: the entry gate + cycle loop read `slices/<slice>/tasks.md`;
  the design inputs are the shared feature-root artifacts plus `slices/<slice>/{plan.md, task-mapping.md}`;
  per-slice outputs (`cycle-report.md` + verification reports) land under `slices/<slice>/` and what that
  does to the done-condition's artifact set; the **full-repository-suite regression net** (implement is the
  only slice-scoped consumer that runs the quality gates, so "the gates run the full repo suite" is its own
  operationalization of the contract's regression-safety rule, not a restatement); and the
  **feature-declared-not-verified-at-last-slice** surfacing (implement is the pipeline's terminal stage —
  only it reaches the last slice's G5, so the Feature-Done handoff is uniquely its responsibility).
- **Note:** the Graduation contract is on the ≥3-consumer queue (plan/tasks/implement slice-scoped
  variants) — this strip relocates implement's *local restatement* to the contract home; it does not rule
  the shared contract. **implement was the last restating consumer** (per the tasks-wave queue note:
  "plan + tasks are now locally de-restated, and implement.md's entry variant is the remaining restating
  consumer") — with this strip, all three consumers are locally de-restated; only the shared-contract
  ruling remains queued.
