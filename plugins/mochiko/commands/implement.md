---
description: Execute an accepted task breakdown into working code, TDD-built and independently verified against real infrastructure.
disable-model-invocation: true
---

# Implement — Execute the Task Breakdown

**Goal:** turn one feature's accepted `tasks.md` (cycle cards, resolved from
`.mochiko/features/FEAT-XXX/`) into working, verified code — TDD-built, foundation cycles
before feature cycles, verified against real infrastructure. `$ARGUMENTS` = the feature ID
(`FEAT-XXX`); empty → resolve the next planned undelivered feature from the map and confirm
with the user.

## Goal

Every `tasks.md` cycle card is `[x]`; each card was decomposed into concrete tasks by its
builder at build time — the decomposition disclosed in the cycle report, never pre-written —
and the built code was implemented test-first (red/green/refactor) and independently
verified — executed `**TEST:**` gates, quality gates
with exit codes, captured real-infrastructure evidence — per cycle and once for the whole
implementation; the feature's verification also ran the **accumulated TEST gates of
previously delivered features in its territory**, and any seam against an earlier-delivered
feature was exercised here, against the real delivered side; the code meets its criteria,
holds traceability to requirements, and aligns
with the project's governance; where a structural delta was approved at plan time, a
built-vs-approved diff report exists and any divergence it names was ruled by the user; the
acceptance landing executed whole — map bookkeeping and every touched baseline's graded
fold; and the user accepted the implementation.

**Not done — default FAIL:** an unchecked cycle card · a failing quality gate · verification
without real-infrastructure evidence · a regression in a previously delivered feature's
gates · a surfaced architecture deviation neither built as
approved nor consented as an amendment · a touched baseline accepted without its graded
fold · user acceptance not given.

## Harness

- **You are the lead.** Plan the run and orchestrate it toward the Goal; teammates or
  subagents per seat is your call.
- **Plan approval:** any seat that writes code or artifacts plans first and works only on a
  plan you approved; grading, verification, and fact-finding seats are exempt.
- **Independence:** no output is cleared by its author — implementation and verification are
  never the same seat; verification executes against real infrastructure and reads the code
  and its evidence, default FAIL.
- **Bounds:** a cycle consumes an **attempt** every time a verification seat grades it —
  whatever the round is called (rework, completion, targeted fix, re-grade); default 3
  attempts per cycle, redeclarable at run open. Exempting a round from the count is reserved
  to the user, never lead discretion. Two consecutive rounds with unchanged findings is a
  no-progress stop: halt the cycle, present state. Re-verification is scoped to the delta —
  a test-only or records-only change gets a delta-grade of the changed surface, never a full
  gate re-sweep; a delta round re-runs no quality gates, the prior gate evidence standing
  while the graded head is unmoved — and the graded object is the code tree (`git rev-parse
  HEAD:<code-dir>`), so a records-only commit does not move the graded head. Rounds consumed
  and seats spawned are surfaced to the user at each checkpoint.
- **Reserved to the user:** architecture-deviation consent — a cycle that adds or removes a
  box, adds, removes, or redirects an arrow, or moves a responsibility across a boundary on
  the approved diagram stops and is presented: build as approved, or amend `architecture.md`
  first · requirement ambiguity or a judgment call a producer flags — answered by the user,
  investigable gaps excepted · scope escalation (work bigger than the run was framed; the
  run stays FAIL unless the user explicitly accepts) · exempting a grading round from the
  attempt count (Bounds) · final acceptance (accept / amend / reject).
- **Escalation cadence:** reserved-to-user questions accumulate and land as one batch at the
  cycle checkpoint; only a question the build cannot proceed without interrupts mid-cycle.
  Advisory verifier findings ride the same rule — a Minor advisory finding defaults to a
  `BACKLOG.md` booking, never an in-cycle fix; an Important-or-above advisory finding blocks
  the cycle and enters the checkpoint batch. A failed `**TEST:**` gate or quality gate is
  never severity-triaged — it fails the cycle per the Goal; `minimalism:` findings stay
  advisory at any severity, per Bindings.
- **Entry:** the run gates on a feature entry carrying ratified scope — the scope source is
  a spec's accepted Feature Selection or a feature-command delta card. Neither → block: new
  capability to `/mochiko:specify`, feature-keyed delta to `/mochiko:feature`. **Selection
  scope** additionally gates on the accepted package — the feature's `tasks.md` complete
  alongside its `plan.md` and `architecture.md` at `.mochiko/features/FEAT-XXX/`; missing or
  incomplete → block, point to `/mochiko:plan`; a selected feature ordered earlier and not
  yet `delivered` blocks — one run per feature, in dependency order. **Delta scope** gates
  on the delta card confirmed by a delta-scope plan run; the card's acceptance criteria (a
  bug's reproduction-failing-test, or 1–3 criteria on the delta) are the cycle's criteria. A
  missing governance region is surfaced, never auto-resolved; present → each code-touching
  brief names the relevant `.claude/rules/mochiko/` files as an obligated read.
- Suggest commits; never run git mutations, never push — an ephemeral, self-removed
  verification snapshot is not a mutation of refs, index, tracked content, or history.
  User acceptance is plain blocking text, never a timed prompt.

## Bindings

- **Deliverable:** the working code; `tasks.md`'s per-card checkboxes are the progress
  surface, flipped as cycles complete.
- **Craft skills:** card decomposition + TDD via `mochiko:executing-tdd-cycle` (its
  `cycle-report.md` format — the disclosed decomposition, honest difficulties, deviations,
  `domain_deps_added` — is the uncertainty carrier; brownfield touches ride
  `mochiko:brownfield-integration`; the pre-code ladder rides
  `mochiko:patterns-code-minimalism` at decomposition, rungs disclosed) · verification via
  `mochiko:testing-end-user` — evidence captured, never assumed — plus the per-cycle
  code-minimalism lens via `mochiko:review-code-minimalism`: the verification seat reads
  the cycle's diff, `cycle-report.md`, and the codebase around the diff (reuse claims
  never on trust); `minimalism:` findings are advisory to the checkpoint verdict, never a
  cycle-failing gate.
- **Design inputs:** the feature's `plan.md` and `architecture.md` (the anchor for the
  deviation check and the built-vs-approved diff) with the feature's other deltas at
  `.mochiko/features/FEAT-XXX/`, plus its `requirements.md` there; the product baselines at
  `.mochiko/product/` — `data-model.md`, `contracts/`, `constraints-and-decisions.md`,
  `nfrs.md` for the numeric quality targets the built code must respect — and `spec.md` for
  the cards' cited acceptance criteria.
- **Reports** land in `.mochiko/features/FEAT-XXX/` (product-lane runs:
  `.mochiko/product/lane-<slug>/`): cycle reports, verification reports, the
  final-validation report, the built-vs-approved diff report. Repeat runs append (dated);
  delta files overwrite only via the graded fold.
- **Regression scope:** quality gates run the full repository suite; the final validation
  additionally executes the accumulated `**TEST:**` gates of previously delivered features
  in this feature's territory, and this feature's gates exercise any seam whose earlier side
  is already delivered — seam ownership sits with the later-landing feature, per
  `mochiko:authoring-feature-map`.
- **Cold verification:** the final validation builds and runs the quality gates from a
  dependency-cold snapshot of the uncommitted working state
  (`git ls-files -co --exclude-standard :!.claude/worktrees` copied to
  `.claude/worktrees/mochiko-<purpose>/`), its results part of the acceptance evidence;
  ensure the `/.claude/worktrees` ignore entry exists first.
- **KM landing:** where `.mochiko/memory/knowledge-management.md` exists, a built structural
  change folds into `ARCHITECTURE.md` — the fold is dual-target (the feature's
  `architecture.md` accumulates the approved delta) per `mochiko:authoring-architecture`.
- **Baseline touches:** mid-fix discovery that the work touches a product baseline → the
  dispatched run authors `baseline-delta.md` in its feature dir at discovery — a minimal
  enumerated delta in appliable form. A product-lane run discovering it stands on an
  in-flight feature's territory files the finding to that run and aborts — the lane never
  widens in place.
- **Acceptance landing:** at user acceptance, one landing executes whole, branched by scope
  type. **Selection scope** — the same landing that folds `ARCHITECTURE.md` executes the
  map's graduation batch per `mochiko:authoring-feature-map`: the feature's status flips to
  `delivered` (dated) · this feature's marked deltas fold into its extent lines · the
  `FEATURES.md` index line updates · the `ARCHITECTURE.md` In-flight pointer is cleared ·
  the specs-index row is touched — the spec reads closed exactly when all its selected
  FEAT-IDs read `delivered` (derived, never asserted). No separate feature-close stage
  exists. **Delta scope** — the entry's marked delta folds per `mochiko:authoring-feature-map`'s
  delta fold. **Both scopes:** every touched baseline folds via a graded fold — three-way
  diff: pre-fold baseline + delta vs folded result; delta applied whole, nothing else
  changed — checked by the landing verification seat, scope-extended; lane runs add the
  map-delta boundary check (the accepted work made no map write beyond the marked delta) to
  the same seat. A delta whose baseline file is absent at fold time folds into a fresh
  `.mochiko/product/` file (empty pre-fold side), the absence surfaced to the user as a
  seeding gap.
- **Register:** user-facing prose per `templates/output-style.md`.
