# Backlog

The **complete open-set detail store**: every open thread, bounded (title · date · provenance ·
resume-cold context), in theme-keyed sections merged on groom. Never curated — the curated scan
surface is [`ROADMAP.md`](ROADMAP.md). Closing an item = the KM landing ritual
(`.mochiko/memory/knowledge-management.md`): decision row + move here → the trail
(`.mochiko/archive/backlog-trail.md`) + touch ROADMAP.md. No `[x]` lives here.

*Last groomed: 2026-07-25 (migration) — baseline: 52 open items, per-item bound ≤15 lines.*

---

## Open design decisions

- [ ] **Prose vs. gate allocation** (2026-06-27; provenance: unrecoverable —
  `agent-skills-research/synthesis/my-framework.md`, submodule removed 2026-07-21) — which
  behaviors earn graded anti-rationalization prose vs a hard `PreToolUse` hook? Kernel excluded;
  allocate between prose and hook.
- [ ] **Claude-Code portability** (2026-06-27; provenance: unrecoverable — same source) —
  `rfc2119-invocation-trigger` + `disable-model-invocation` are CC-specific: adopt-and-bind or
  abstract? Surfaces when the router evolves.
- [ ] **Intensity modes** (2026-06-27; provenance: unrecoverable — same source) — global
  `lite/full/ultra/off` dial vs per-rule. Defer until the pattern is clear from real runs.
- [ ] **Command orchestration substrate — teams vs `Task`-subagents** (2026-06-30,
  `.mochiko/brainstorms/command-altitude/synthesis.md`) — all seven commands are team-form by
  ruling, but **no run has yet proven team-form execution**: two defect runs dispatched
  subagents despite the mandate (root cause + fix: `setup-v3-team-defect` record,
  `agent-dispatch.md` v3 + addressability check), and the kinako artifacts can't distinguish
  transport (no transcripts). Blocks on the team-form confirm-or-revert item below. Datapoint
  history: archive `DECISIONS.md` rows + `brainstorm-command-rewrite` / `brainstorm-v2-revision`
  records.
- [ ] **Module-elicitation scaling** (2026-07-17, `setup-operating-docs-scaffolding` record) —
  one interrogation dimension per future module, or a consolidated modules beat? Datapoint
  (2026-07-21): `layer-rules` landed as a *beat*, agenda stayed ten. **Revisit: next module
  design.**
- [ ] **D9 catalog-graduation seam** (2026-07-16, `setup-constitution-flexibility` record) —
  after real sessions mint principles, harvest candidates from trace stamps and design the
  graduation pass (curation authority, admission bar, versioning). From artifacts, not
  speculation.

## Operating-docs / KM module

Ruling: DECISIONS.md OD-D1–D12 (2026-07-25); record
`.mochiko/brainstorms/operating-docs-maintenance/record.md`. Built 2026-07-25 at v0.29.0;
mochiko's own migration executed same day (this file's shape is its result).

- [ ] **Redesigned-module dogfood** (2026-07-25) — a fresh setup run: dimension-7 fires
  core-whole + per-doc electives; G5 scaffolds the core AND the enforcement surfaces
  (project-pinned copy · rules file · CLAUDE.md pointers) never-overwriting; **module dogfood
  gates on the injection probe**; landing ritual + invariants fire at the five command
  boundaries; a seeded cap trip invokes `grooming-operating-docs`. Mochiko's own compliance is
  manual until more commands run in-repo (accepted risk, D9 fold).
- [ ] **Brainstorm bookkeeping watch** (2026-07-17, carried) — on a KM project:
  read-index-before-open fires, entries land/update with named landings, close invariants catch
  a seeded defect, the no-module branch stays silent. Honest residual: drift on a session-less
  project is caught only at the next setup/amend re-audit.

## Team-method build items

Rulings D1–D5 (DECISIONS.md 2026-07-25; record `team-method-vs-command-shape` — bare session,
un-reviewed). Land via `command-architect` + independent `validation-command-shape` audit.

- [ ] **Deferred direction (recorded, not ruled)** — build-room merge (structuring seat inside
  implement), slice-fold toward a four-command surface, multi-stream implement (frozen seams ·
  single-ownership · wait-fallbacks as task-artifact content). Re-open in a dedicated session;
  design input preserved in the record (R1/D4).

## Architecture-primitive build items

Rulings AD-D1–D9 (DECISIONS.md 2026-07-30; record
`.mochiko/brainstorms/architecture-design-primitive/record.md`). **Built 2026-07-30 at
v0.32.0** together with the plan-absorbs-tasks merge; both independent audits PASS; closed
build item in the trail.

- [ ] **Architecture-primitive dogfood** (2026-07-30) — first merged-plan run with the
  architecture stage: watch the R6a bootstrap + baseline-confirm gate, the rendered-diagram
  sign-off + degrade-with-record fallback (R5), the qualifying-flow sequence trigger (R4),
  structural D-XXX rows landing in the designated `constraints-and-decisions.md` section
  (R3), implement's cycle open/close deviation self-check + built-vs-approved diff reaching
  the acceptance gate (seam-N1 placement), and the `ARCHITECTURE.md` fold staying distinct.
  Rides the merged-command dogfood (plan+tasks item below) and the team-form
  confirm-or-revert.
- [ ] **Per-slice `architecture.md` ↔ repo `ARCHITECTURE.md` tie-back** (2026-08-01;
  provenance: mochiko-app dogfood observation, to-brainstorm) — in the mochiko-app dogfood the
  design-time `architecture.md` was written into a spec *slice* subfolder; open question is how
  a per-slice/per-feature `architecture.md` (`patterns-system-design`) folds up to / ties into
  the repo-level `ARCHITECTURE.md` (`authoring-architecture`). Brainstorm: expected placement
  (feature-level vs slice-level), the fold seam between the two, and whether per-slice nesting is
  a placement anomaly or intended granularity.

## Command goal-shape rebuild

Rulings CS-D1–D10 (DECISIONS.md 2026-07-30; record
`.mochiko/brainstorms/command-succinctness-strip/record.md` — pair-reviewed, verify CLEAN).
Execution pilot-first per CS-D10; the anatomy is unprecedented, so nothing fans out before the
pilot checkpoint. Interaction note: CS-D7 scopes skill edits for *this delivery* past the
`loop-discipline` all-consumer pass (team-method section) — that pass stays open for the
skill's own sizing.

- [ ] **Deferred: the `loop-discipline` read-drop** (2026-07-30, pilot-checkpoint ruling 5; steps 1–4 all DONE → trail, v0.35.0) —
  lands as a one-clause shape edit + delta re-audit when the named trigger fires: **first live
  dogfooded run of a rebuilt command with gates un-rationalized** (CS-D7 replacement guarantee;
  authoring-loop evidence ruled insufficient — ADR `2026-07-30-goal-shape-pilot-checkpoint`).
  Trigger terms re-keyed by the 2026-08-01 flexibility ruling (R16: "gates not rationalized" →
  measured against the stated default + recorded departures; "bounds held" → declared bounds
  under the U1-D counter rule); re-specification landed with shape v7 (v0.40.0 transition
  note) — only the trigger's firing remains open.

## Lead-owned process flexibility build (shape v7)

Rulings D1–D6-as-amended + A1–A4 (DECISIONS.md 2026-08-01; record
`.mochiko/brainstorms/lead-owned-process-flexibility/record.md` — pair-reviewed 40→39→31,
31/31 dispositioned, verify CLEAN round 3).

- [ ] **Convert-on-touch residuals** (2026-08-01; build DONE at v0.40.0 → trail) — owed at each
  command's conversion touch: measure the v7-form Constraints/Bindings blocks and land any
  check-6 ceiling term in the same wave (check 20 names the owed work; over-ceiling stays a
  floor FAIL meanwhile — beyond-record, user-ratified at wave close 2026-08-01 together with
  the marker-retirement clause) · bind
  P18–P20 · state the v6-form run's weight-card home (shape-audit obs). Next touch of
  `review-brainstorm`/`review-governance-intent`: align the verify-pass trigger phrasing with
  U1-B (logged in `.mochiko/strips/sized-end-stage-review.md`). R21 (recorded-open): measure
  declaration + trail + composition cost on the first light and the first heavy conversion —
  the natural sites; detail in `.mochiko/strips/command-shape.md` [v0.40.0]. Watches: first
  external dogfood (R22) · each converted command's first live run · floor-read growth
  +11,399 B/run (token epic, no offset claimed; static read-cost — distinct from R21's per-run
  composition overhead).

## Pipeline dogfood & confirm-or-revert

Every command passed structural verification + the kinako artifact pass on content; the
**team-form half is unproven** (no transcripts). Named per-command checks: archive
`DECISIONS.md` + each command's strip note. Kinako artifact evidence: the 2026-07-24 validation
pass (evidence repo `humaninloop-dev/kinako`).

- [ ] **Team-form confirm-or-revert — instrumented run (residual A)** (2026-07-24) — capture
  seat spawn / standing-vs-respawn / messaging, or make an explicit accept-on-weak-evidence
  ruling. Blocks the orchestration-substrate decision above. A revert logs `RETURNED:` in the
  command's strip note.
- [ ] **Setup v3 dogfood** (2026-07-18, carried) — live-run residue only (authoring half
  kinako-validated): the runtime named checks (validator catches a seeded missing-companion ·
  downstream producer receives CLAUDE.md governance natively · region regenerates idempotently),
  the G3 sizing-gate/waiver/delta-pass firing, the KM dimension-7 + collision beats, the
  injection-probe G5 offer (residual C).
- [ ] **Specify dogfood** (2026-06-27, carried) — loop content kinako-validated (3 rounds to
  `ready`); open: team-form checks (probe fires, standing producer messaged round > 1, critic
  cold).
- [ ] **Plan dogfood (+ tasks, inherited by the merged command)** (2026-07-01, carried) —
  content kinako-validated; open: team-form checks (standing producer across the phase
  boundary, incremental advocate mode, architect fires once + structural-change re-fire).
- [ ] **Implement dogfood** (2026-07-01, carried) — content kinako-validated (21 cycles, 407
  tests); open: team-form checks (standing producer across cycles/fix-pass, cold qa, confidence
  gate observed live).
- [ ] **Slice dogfood + unexercised sub-paths (residual D)** (2026-07-02, carried) — content
  kinako-validated (4 slices, extend-mode); open: **null exit** (small spec) ·
  **graded-amendment/breaking change** (mid-flight spec edit) · **`infeasible` escalation** ·
  the **"Research this" → Explore** branch · team-form checks.
- [ ] **Brainstorm v2.2 review-machinery dogfood** (2026-07-16) — the sized lens-split review
  has since run in five pair-form sessions (operating-docs the latest: 35→29→CLEAN); still
  open formally: the cost re-measure from transcripts (target pair ≈150–170k), the argument-cap
  watch (0-for-2), the one-shot-exchange calibration bet.
- [ ] **Post-fix confirmation run (residual E)** (2026-07-24) — after the Cluster-2
  ratification lands, re-run the pipeline to confirm the surface-task rule / journey gates /
  residue routing actually catch the kinako gaps.

## Kinako follow-up run

One vehicle, several riders (2026-07-24; oversight trace
`.mochiko/specs/mvp-thin-loop/oversight-trace.md` in the evidence repo; validation-pass record
in this repo's git history at `7920ccb` BACKLOG).

- [ ] **Waves 1–2 didn't land — investigate, then dogfood (residual B)** — kinako artifacts
  show the dense forms ABSENT in S4-era entries, design layer ~28% above the 555k baseline, no
  v0.22/0.23 stamp, run-costs 2/~15 rows. A strip-didn't-land bug is on the table: confirm the
  forms apply, then re-run + re-measure. Acceptance checks + calibration watches (envelope size
  defaults; T3's two branches): archive ROADMAP wave-1/wave-2 trail entries.
- [ ] **Cluster-2 ratification wave** — pressure-tested, awaiting founder ratification; lands
  as one edit wave (author ≠ grader), ruling → DECISIONS.md row. Items (owner · n): 
  surface-task rule (`patterns-vertical-tdd` + `review-task-artifacts`; n=2) · journey-gate
  class, graded form (`testing-end-user` + `qa-engineer`; blocks feature-close; n=5) · two
  spec-review hunt classes — display meaningfulness + lifecycle twins (`review-specifications`;
  n=2; hunt-list growth guard n≥2 rides along) · split-gate assert-union invariant
  (`loop-discipline`; n=1, zero-cost) · runbook walked-stamp (landing surface ruled at
  ratification; n=1 doc) · residues route to a tracked surface (`executing-tdd-cycle` +
  `authoring-slices`; n=1–2) · plan designs its surfaces (plan cluster; n≈4 paths) ·
  watch-lines (scaffolding→production promotions; out-of-pipeline integration steps —
  upgrade on a second instance). **Meta-rule riding:** single-project-retro changes default to
  checklist edits; new artifacts/stages/gates need n≥2. Full text: git history (`7920ccb`).
- [ ] **D2 baseline capture defect** — `run-costs.md` caught 2 of ~15 stage-runs; decide
  acceptable-partial vs a per-stage prompt in the protocol.
- [ ] **Artifact-filename collisions across stages** — same-named advocate reports overwrite
  across plan/tasks phases and slices; an earlier stage's review is silently lost. Decide a
  stage-qualified filename convention. n=1 but silent.
- [ ] **Domain-allowlist amend confirm** (2026-07-21) — the kinako amend landed (7-row seed
  registry, v1.2.0 governance) but the beat/minted-path *firing* needs a transcript.

## Token-reduction epic

Epic + angles: DECISIONS.md rows 2026-07-23/24; records `workflow-token-reduction`,
`standing-seat-lifecycle`, `model-tiered-seats`. Build scoping re-opens from the three records
together.

- [ ] **D5 — review sizing gates generalized + floored verification depth** — command texts of
  the five pipeline stages (verification depth never zero, real-infra floor).
- [ ] **D2 upgrade — the one-shot OTel probe** — enable documented config in a dogfood run;
  observe console/per-run aggregation/teammate attribution; automation graduates on probe
  evidence only. Standing-seat D4 + model-tiered D6 probe questions ride it, plus TC's three
  recorded-open (2026-07-31): `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` teammate applicability ·
  `subagentStatusLine` teammate coverage + `tokenCount` semantics · cost-one-recycle.
- [ ] **Wave-3 candidates (candidates, not commitments)** — (1) governance/memory layer
  (intent 26.2k B · ledger 47.4k B · trace summary 12.9k B; densify inside the setup cluster,
  not an artifact wave; revisit: an amend run showing read cost mattering); (2) brainstorm
  `record.md`/`synthesis.md` (record length IS the audit trail; revisit: transcript forensics
  showing record *authoring* as a material pool). Neither enters scope silently.
- [ ] **Parallelism deferral — live shortcut-ledger candidate** (2026-07-01) — revisit native
  `pipeline()`/`parallel()` or the lightweight kernel question **if dogfooding shows sequential
  cycle execution too slow** on a dependency-rich feature; the concrete drift-trigger the
  code-free bet waits on. Re-evaluation queued in ROADMAP.md *Next*.

## Production-only narrowing

Rulings PO-D1–D7 (DECISIONS.md 2026-07-30); record
`.mochiko/brainstorms/production-only-focus/record.md` (pair-reviewed, verify CLEAN). Identity
landed in ROADMAP thesis + CLAUDE.md same day; **narrowing build DONE 2026-07-30 at v0.36.0 —
trail** (scope ADR `2026-07-30-po-narrowing-build-scope`). Tier-I depth items open below.

- [ ] **Security-depth build (Tier I)** (2026-07-30; rulings SD-D1–D6, record
  `.mochiko/brainstorms/security-depth-scoping/record.md` — pair-reviewed, verify CLEAN round
  3) — build surface (record: "Build surface" + its V4 extension): `patterns-threat-modeling`
  skill + `SECURITY-AREAS.md` registry · trust-boundary diagram convention · SEC-XXX full
  plumbing + DS-XXX close (F95) · FLOOR-SEC row edits (SAST high/critical · no-hand-rolled
  line · tooling map + enforcement clause relocated universal) · semgrep floor rulesets
  (governance-protected) · contracts checks (F96/F97) · quality-gates producer (F94) ·
  coverage ledger + `validation-constitution` check · persona edits (keystone-checked) · spec
  producer prompt · gate canaries · F19 fix as prerequisite · one command edit (`plan.md`
  Bindings, shape-v5 audited). Fences: DAST/pentest · per-regime content · type-specific
  expressions (shelves) · runtime ops. Tooling-map relocation target named 2026-07-31 (OO-M4):
  `authoring-constitution/references/STACK-TOOLING.md` — whichever build lands first creates
  it, the other joins.
- [ ] **Ops & observability build (Tier I)** (2026-07-31; rulings OO-D1–D7, record
  `.mochiko/brainstorms/ops-observability-hardening/record.md` — pair-reviewed 25→20 merged,
  20/20 dispositioned, verify CLEAN round 2) — build surface (record: "Build surface", M-folds
  incorporated): fifth floor category **Operations** + FLOOR-OPS card across the nine-file
  enumeration ripple (incl. `principal-architect.md` keystone-checked · both QUALITY-CHECKLIST
  lines · `codebase-analysis-template.md:118` rollup) · FLOOR-OBS established-tooling line ·
  dimension-8 SLO beats → region one-liner + `paths`-scoped SLO rules file over
  `.mochiko/specs/**` · RUNBOOK contract in KM + dual-key `implement.md` landing fold (the one
  command edit, shape-v5 audited) · release-gates reference line · `STACK-TOOLING.md` universal
  home (create-or-join) + OTel disambiguation · feasibility SLO row + plan-artifacts coverage
  row · vertical-tdd instrumentation TEST rule + task-artifacts dual-keyed row · known-gap
  validator check · clause-level waiver-key wording · bounded alert canary. Fences:
  release-process assertion · SLO achievement · incident machinery + error budgets (Tier-II
  reliability) · per-kind shelf expressions · dormant IP rows stay dormant.
- [ ] **Shelf builds — the translation tables (Tier I)** (2026-07-30, PO-D5) — order:
  frontend → mobile → desktop (map F30: only backend/service seeded; desktop net-new). Absorbs
  the frontend + mobile shelf items in Deferred tracks below.
- [ ] **IaC / deployment engineering — staged (Tier II)** (2026-07-30, PO-D5) — stage 1:
  release gates + environment discipline asserted; stage 2: infrastructure-code authoring (new
  artifact class, map F23). Own scoping session; data lifecycle + reliability/resilience ride
  Tier II behind it.
- [ ] **PO watches & revisits** (2026-07-30) — D4.1 waiver-expiry revisit (user-flagged
  "I will come to revisit") · D7 waiver-as-normal-state dogfood watch (if young teams live on
  permanent floor waivers, revisit expiry/maturity design). (The non-legal module waivability
  ambiguity was ruled 2026-07-30 — waivable under D4; ADR `po-narrowing-build-scope`.)

## Ergonomics: output, language & run-hygiene

Raw captures (2026-08-01) — to triage/brainstorm; grooming may re-key into finer themes.

- [ ] **Validator worktree isolation in implement** (2026-08-01; provenance: capture session,
  to-brainstorm) — give the implement command's validation/QA step the option to run in a git
  worktree for cleaner separation from the producer's working tree (isolate the grader from
  uncommitted producer state / avoid cross-contamination). Brainstorm: which validation gates
  warrant it, per-gate worktree setup cost, and interaction with the cold-checkout step already
  ruled into implement's final validation (2026-07-31).
- [ ] **Reduce mochiko output verbosity — explore "caveman"** (2026-08-01; provenance: capture
  session, to-brainstorm) — mochiko's user-facing output is too verbose; explore the "caveman"
  terse-output style and how to bring it into `setup` and mochiko generally. Brainstorm: where
  terseness helps vs. where detail is load-bearing, global dial vs. per-surface, and the seam
  with the token-reduction epic (which targets inter-agent/report tokens, not user-facing prose).
- [ ] **Plain-language sweep + internal-jargon leak to end users** (2026-08-01; provenance:
  capture session, to-brainstorm) — the language across mochiko is too complex; needs a
  plain-English sweep. Concrete leak: the plugin's end user is shown "Layer -2" (internal
  shape/architecture vocabulary) which means nothing to them. Brainstorm: audit user-facing
  surfaces for internal jargon, draw an end-user vs. internal vocabulary boundary, and sweep
  command/skill output.

## Defects & empirical checks

- [ ] **Shape-home keying watch: "Out of rounds = escalate, never done."** (2026-07-30,
  shape-audit advisory at the PO wave) — the sentence is verbatim shape prose
  (`command-shape.md` bounds line) carried by 3 of 6 commands (setup · plan · specify); not a
  check-8 keyed marker today, so it passes audits. Raise if a command's Constraints block ever
  needs those words back (setup sits at 4 w of Constraints headroom) — either key a marker and
  strip the three, or record it as a mandated-exemplar phrase like the bounds set.
- [ ] **Fresh-session description-delivery probe** (2026-07-25, succinctness R1) — re-run the
  listing probe in a NEW session: do the two repaired descriptions deliver un-truncated, and
  does `validation-constitution`'s description reappear at the reduced total (confirm-or-kill
  the total-budget hypothesis)? If still absent: wave-pass description-mass cuts.
- [ ] **HTML-comment stripping in `.claude/rules/` files** (2026-07-18) — docs confirm
  stripping for CLAUDE.md, silent for rules files. If stripped, in-file trace comments become a
  free option. Method: `InstructionsLoaded` hook or `/context` on a comment-bearing rule.
- [ ] **Fresh-session rules-loading test** (2026-07-18) — the negative probe's confound: the
  rule file was created mid-session. Re-run with the file present before session start; either
  result stands (universal principles stay in CLAUDE.md).
- [ ] **Trace-check ergonomics watch** (2026-07-16) — v3 manifest grading: watch for false
  FAILs on trace mechanics vs substance; if the fix-list loop thrashes on mechanics, tighten
  templates before touching the check.
- [ ] **Strip-pass residual: brainstorm/setup light pass** (2026-07-19) — reads as discharged
  by the v0.11.0 pre-shrink; confirm and close. Re-add protocol stands
  (`.mochiko/strips/README.md`): evidence-linked re-adds, marked overrides, audit on re-add
  bumps.
- [ ] **Experimental-API churn watch** (2026-07-04) — agent-teams behavior changed across five
  point releases; if an update breaks spawn/messaging semantics, re-verify against docs. The
  commands name capabilities, not version mechanics, to keep this surface small.
- [ ] **Two-level "slice" vocabulary watch** (2026-07-02) — graduation slice (spec-level) vs
  vertical slice (cycle-level); if dogfooding shows conflation, sharpen triggers or rename one
  level.

## Deferred tracks & shelves

- [ ] **`audit` workflow scoping** (2026-07-02 charter) — feature-close verification:
  qa-engineer executes `slices.md`'s Feature-Done section (SC coverage + cross-slice seams)
  against real infra; human FEATURE-DONE gate. Scoping decides whole-workflow vs branch; the
  journey-gate class (Cluster 2) gives the pass its hard gate class; the cold-checkout step
  (ruled into implement's final validation 2026-07-31) migrates here if audit takes feature-close.
- [ ] **Feature-close verification has no owning workflow** (2026-07-02) — until audit owns
  it, the section is executable by hand; implement surfaces "declared, not verified" after the
  last slice.
- [ ] **Design track** (2026-06-27, carried; archive REGISTRY disposition) — `ui-designer` +
  `analysis-screenshot` / `authoring-design-system` / `patterns-flow-mapping` /
  `patterns-interface-design`; port as a dedicated cluster (HIL sources listed in the archive).
- [ ] **Frontend catalog shelf (stage 2)** (2026-07-16, user-ruled in-scope; **now Tier-I of
  the production-only narrowing, PO-D5 2026-07-30 — first among the shelf builds**) —
  `catalog/frontend.md`: type principles + frontend-appropriate floor examples (universal shelf
  examples are backend-flavored). CLI/library shelves retire under PO-D1's deferral — do not
  author speculatively.
- [ ] **Mobile/app shelf** (2026-07-21; **now Tier-I of the production-only narrowing, PO-D5
  2026-07-30 — after frontend, before the net-new desktop shelf**) — flavored
  clean-architecture-for-apps card selecting `layer-rules`. Acceptance: a Flutter setup session
  deals flavored layered material. Carries F6: greenfield app sessions risk backend-flavored
  paths globs until this lands.
- [ ] **Multi-stack / monorepo registries** (2026-07-21) — per-stack registries, paths scopes,
  seeding. **Revisit: first multi-stack setup run.**
- [ ] **`codebase-inventory-schema.json` port** (2026-06-27; provenance: unrecoverable — HIL
  submodule removed 2026-07-21) — the collision/spec-plan-mode contract for
  `analysis-codebase`; port with the spec/plan cluster and wire its consumer then.
- [ ] **Phase-A0 codebase-discovery reclaim** (2026-07-01) — `review-plan-artifacts`' parked
  discovery review (documented in its ARTIFACT-CHECKLISTS); the brownfield/discovery track must
  reclaim it.
- [ ] **`qa-engineer` audit affinity** (2026-07-01) — when audit is scoped, confirm the agent
  (or its verification skill) serves it without a produce+grade leak.
- [ ] **`/mochiko:graduate` wrapper** (2026-07-02) — thin per-slice sequencer over existing
  commands; build only on demonstrated shepherding pain, never a re-implementation of their
  loops.
- [ ] **Slice spec-amend mode** (2026-07-02, recorded deferral) — re-place changed/new stories
  without touching shipped slices, escalate when a shipped slice's stories changed; design when
  a real mid-flight amendment hits.
- [ ] **Brownfield onboarding path** (idea) — a distinct entry path into setup; HIL had
  `brownfield-constitution` + `brownfield-integration` craft.
- [ ] **Context handoff document** (idea) — cross-session serialization; a deliberate add-on
  if pursued (setup/specify dissolved the HIL carrier into the lead).
- [ ] **Deliberate shortcut ledger** (idea) — deferral tracking with upgrade triggers; the
  parallelism entry above is its first live candidate.
