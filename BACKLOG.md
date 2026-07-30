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

- [ ] **loop-discipline all-consumer pass** (2026-07-30; provenance:
  `.mochiko/decisions/2026-07-30-layer-2-mesh-rewrite-executed.md`) — SKILL.md:56 ("The
  lead/referee owns the verdict") literally forbids shape v4's devolved clean branch; needs a
  narrow qualifier pointing at the devolved branch, done-condition verdict staying the lead's.
  ≥3-consumer rule (22 referencing files, 7 commands) escalated it out of the mesh wave.
  Riders: dedup ruling on "status is input, never the gate" (v4 hoisted it into the home;
  restated in all six commands' Contracts) · verdict-ownership triplication recurrences
  (plan/implement/tasks — their own strip notes claim it deduped).
- [ ] **agent-dispatch.md peer-edge briefing field** (2026-07-30; same ADR) — shape v4
  obliges the lead's brief to carry each seat's peer edges AND the gap-list hold, but the
  briefing-field table has no home for either, and field 6 (prior feedback verbatim-paste)
  still assumes the superseded v3 lead-relay. User ruling: ninth field (shared-primitive
  change) vs. shape-sentence-only.
- [ ] **Cold-checkout gate (kinako rule 7)** — warm-machine verification passed a bug six
  cycles (`build/` gitignored; `dart compile exe` missing `-o` parent). Add a fresh-clone
  verification step. **Placement ruling needed:** implement final validation vs the audit
  charter.
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
  evidence only. Standing-seat D4 + model-tiered D6 probe questions ride it.
- [ ] **Wave-3 candidates (candidates, not commitments)** — (1) governance/memory layer
  (intent 26.2k B · ledger 47.4k B · trace summary 12.9k B; densify inside the setup cluster,
  not an artifact wave; revisit: an amend run showing read cost mattering); (2) brainstorm
  `record.md`/`synthesis.md` (record length IS the audit trail; revisit: transcript forensics
  showing record *authoring* as a material pool). Neither enters scope silently.
- [ ] **Standing-seat build items (deferred)** — conditioned checkpoint recycling · respawn
  briefs from artifacts · the Layer-2 transport-vs-lifecycle rewrite (**v4+** — coordinate with
  the team-method mesh rewrite above) · per-seat measurement. Record D1–D4.
- [ ] **Parallelism deferral — live shortcut-ledger candidate** (2026-07-01) — revisit native
  `pipeline()`/`parallel()` or the lightweight kernel question **if dogfooding shows sequential
  cycle execution too slow** on a dependency-rich feature; the concrete drift-trigger the
  code-free bet waits on. Re-evaluation queued in ROADMAP.md *Next*.

## Defects & empirical checks

- [ ] **`validation-command-shape` check-1 setup carve-out** (2026-07-30; provenance:
  `.mochiko/decisions/2026-07-30-layer-2-mesh-rewrite-executed.md`) — check 1 says the five
  KM-carrying commands reference the project copy, "never the module template's path"; setup
  is the scaffolder that must name the template as its scaffold source (setup.md L182). Read
  mechanically the check FAILs a correct file — a latent false-FAIL generator. Carve out the
  scaffold-source naming explicitly (or mark setup's line). Rider: the run-cost element
  carries no `[PARAM]` and no command mentions it — confirm the obligation is reachable by a
  lead who only Reads the home.
- [ ] **`validation-constitution` QUALITY-CHECKLIST post-dissolution drift** (2026-07-23,
  re-confirmed at succinctness wave 4) — its "Structure Quality — universal core" section still
  grades the dissolved `constitution.md` (SYNC IMPACT REPORT, Roman-numeral principles, sync
  mandate, version footer). Retarget at the surface set per
  `templates/governance-surfaces-template.md`; re-audit sibling sections for drift. A latent
  false-FAIL generator.
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
  journey-gate class (Cluster 2) gives the pass its hard gate class; the cold-checkout gate's
  placement ruling may land here.
- [ ] **Feature-close verification has no owning workflow** (2026-07-02) — until audit owns
  it, the section is executable by hand; implement surfaces "declared, not verified" after the
  last slice.
- [ ] **Design track** (2026-06-27, carried; archive REGISTRY disposition) — `ui-designer` +
  `analysis-screenshot` / `authoring-design-system` / `patterns-flow-mapping` /
  `patterns-interface-design`; port as a dedicated cluster (HIL sources listed in the archive).
- [ ] **Frontend catalog shelf (stage 2)** (2026-07-16, user-ruled in-scope) —
  `catalog/frontend.md`: type principles + frontend-appropriate floor examples (universal shelf
  examples are backend-flavored). CLI/library shelves stay mint-driven — do not author
  speculatively.
- [ ] **Mobile/app shelf** (2026-07-21) — flavored clean-architecture-for-apps card selecting
  `layer-rules`. Acceptance: a Flutter setup session deals flavored layered material. Carries
  F6: greenfield app sessions risk backend-flavored paths globs until this lands.
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
