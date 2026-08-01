# Backlog

The **complete open-set detail store**: every open thread, bounded (title · date · provenance ·
resume-cold context), in theme-keyed sections merged on groom. Never curated — the curated scan
surface is [`ROADMAP.md`](ROADMAP.md). Closing an item = the KM landing ritual
(`.mochiko/memory/knowledge-management.md`): decision row + move here → the trail
(`.mochiko/archive/backlog-trail.md`) + touch ROADMAP.md. No `[x]` lives here.

*Last groomed: 2026-08-01 (delivery sweep; shape-v7 wave close) — baseline: 55 open items
(convert-on-touch residuals closed to the trail, post-conversion watches opened in place; 60
pre-sweep: 3 closed to the trail, 2 merged; 52 at the 2026-07-25 migration), per-item bound
≤15 lines.*

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
  `.mochiko/brainstorms/command-altitude/synthesis.md`) — all six commands are team-form by
  ruling, but **no run has yet proven team-form execution**: three defect runs dispatched
  subagents despite the mandate (latest 2026-07-31, transcript in
  `plan-run-transport-forensics/inputs/` — "the team never existed"; probe rebuilt v0.38.0 on
  the team-config roster check, not yet exercised live). Blocks on the team-form
  confirm-or-revert item below. Datapoint history: archive `DECISIONS.md` rows +
  `brainstorm-command-rewrite` / `brainstorm-v2-revision` / `setup-v3-team-defect` /
  `plan-run-transport-forensics` records.
- [ ] **Module-elicitation scaling** (2026-07-17, `setup-operating-docs-scaffolding` record) —
  one interrogation dimension per future module, or a consolidated modules beat? Datapoints:
  `layer-rules` landed as a *beat* (2026-07-21); compliance modules ruled fact-triggered off
  dimension 2, agenda stays ten (PO 2026-07-30); ops SLOs elicited at dimension 8, no new
  dimension (OO-D3 2026-07-31). Three-for-three consolidated — open only for a genuinely new
  *constitution* module. **Revisit: next module design.**
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
  manual until more commands run in-repo (accepted risk, D9 fold). Partial credit (2026-07-31
  mochiko-app *amend* run): injection probe ran at G5, landing ritual observed at 2 of 5
  boundaries (amend + plan) — the fresh-setup half and the seeded cap trip remain.
- [ ] **Brainstorm bookkeeping watch** (2026-07-17, carried) — on a KM project:
  read-index-before-open fires, entries land/update with named landings, close invariants catch
  a seeded defect, the no-module branch stays silent. Honest residual: drift on a session-less
  project is caught only at the next setup/amend re-audit.

## Team-method build items

Rulings D1–D5 (DECISIONS.md 2026-07-25; record `team-method-vs-command-shape` — bare session,
un-reviewed). Build items DONE 2026-07-31 at v0.39.0 → trail; the deferred direction below is
the open remainder.

- [ ] **Deferred direction (recorded, not ruled)** — build-room merge (structuring seat inside
  implement), slice-fold toward a four-command surface, multi-stream implement (frozen seams ·
  single-ownership · wait-fallbacks as task-artifact content). Re-open in a dedicated session;
  design input preserved in the record (R1/D4).

## Architecture-primitive build items

Rulings AD-D1–D9 (DECISIONS.md 2026-07-30; record
`.mochiko/brainstorms/architecture-design-primitive/record.md`). **Built 2026-07-30 at
v0.32.0** together with the plan-absorbs-tasks merge; both independent audits PASS; closed
build item in the trail.

- [ ] **Architecture-primitive dogfood** (2026-07-30) — the first merged-plan run happened
  (2026-07-31 S1): R4 sequence trigger fired (six qualifying flows) · R3 structural D-XXX
  rows landed in the designated section (D-017–D-022) · R5's degrade-with-record fallback
  exercised (un-rendered, G3 stamp). Open: R6a bootstrap (baseline existed, branch never
  entered) · the rendered-diagram primary path · implement's cycle open/close deviation
  self-check + built-vs-approved diff at the acceptance gate (seam-N1) · the
  `ARCHITECTURE.md` fold staying distinct (no implement run yet). Rides the team-form
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
Steps 1–4 DONE → trail (v0.33.0–v0.35.0); the CS-D7-scoped `loop-discipline` all-consumer
pass closed 2026-07-31 (v0.39.0, trail). The read-drop deferral below is the open remainder.

- [ ] **Deferred: the `loop-discipline` read-drop** (2026-07-30, pilot-checkpoint ruling 5; steps 1–4 all DONE → trail, v0.35.0) —
  lands as a one-clause shape edit + delta re-audit when the named trigger fires: **first live
  dogfooded run of a rebuilt command with gates un-rationalized** (CS-D7 replacement guarantee;
  authoring-loop evidence ruled insufficient — ADR `2026-07-30-goal-shape-pilot-checkpoint`).
  Trigger terms re-keyed by the 2026-08-01 flexibility ruling (R16: "gates not rationalized" →
  measured against the stated default + recorded departures; "bounds held" → declared bounds
  under the U1-D counter rule); re-specification landed with shape v7 (v0.40.0 transition
  note). The re-keyed trigger's v7-form constructs (P18 gates · stated-default trail · declared
  bounds) exist on every command since the v0.43.0 conversion wave — the first live run with
  gates ruled and bounds held fires it.

## Lead-owned process flexibility build (shape v7)

Rulings D1–D6-as-amended + A1–A4 (DECISIONS.md 2026-08-01; record
`.mochiko/brainstorms/lead-owned-process-flexibility/record.md` — pair-reviewed 40→39→31,
31/31 dispositioned, verify CLEAN round 3).

- [ ] **Shape-v7 post-conversion watches** (2026-08-01; conversions DONE at v0.43.0 → trail) —
  open remainders of the discharged convert-on-touch item: align the
  `review-brainstorm`/`review-governance-intent` verify-pass trigger phrasing with U1-B at
  those skills' next touch (logged in `.mochiko/strips/sized-end-stage-review.md`) · each
  converted command's first live run is its own checkpoint, and the first run with gates ruled
  and bounds held also fires the `loop-discipline` read-drop trigger (goal-shape section) ·
  R22: the first external dogfood grades the uniform stated default for distributed leads ·
  R21's declaration + trail estimate awaits live-run confirmation (both conversion sites
  measured; regressive fixed-cost finding — near-constant v7 constructs, so lighter commands
  pay a larger share — `.mochiko/strips/command-shape.md`) · token-epic figure: the always-read
  floor now 52,129 B/run (−581 at the marker retirement, the first shrink in three revisions).

## Pipeline dogfood & confirm-or-revert

Every command passed structural verification + the kinako artifact pass on content; the
**team-form half is unproven** (no transcripts). Named per-command checks: archive
`DECISIONS.md` + each command's strip note. Kinako artifact evidence: the 2026-07-24 validation
pass (evidence repo `humaninloop-dev/kinako`).

- [ ] **Team-form confirm-or-revert — instrumented run (residual A)** (2026-07-24) — the
  capture half is DONE: the 2026-07-31 plan S1 run + forensics record hold all three artifacts
  (seat spawn — every seat a subagent, F1 · standing-vs-respawn — 14+ passes, respawn with a
  name-takeover send failure · messaging — zero peer edges across ~44 hand-offs). Open: the
  **verdict** — a run on true team transport (repaired v0.38.0 roster probe, never exercised
  live) or an explicit accept-on-weak-evidence ruling. Blocks the orchestration-substrate
  decision above. A revert logs `RETURNED:` in the command's strip note.
- [ ] **Setup v3 dogfood** (2026-07-18, carried) — live-run residue only (authoring half
  kinako-validated). Partial credit (2026-07-31 mochiko-app *amend*): G3 sizing gate fired ·
  injection-probe G5 offered and ran (residual C met). Open: validator catches a seeded
  missing-companion · downstream producer receives CLAUDE.md governance natively · region
  regenerates idempotently (amend showed non-destructive only) · G3 waiver + delta-pass
  branches · KM dimension-7 + collision beats — and the fresh-setup form itself.
- [ ] **Specify dogfood** (2026-06-27, carried) — loop content kinako-validated (3 rounds to
  `ready`); open: team-form checks (probe fires, standing producer messaged round > 1, critic
  cold).
- [ ] **Plan dogfood (+ tasks, inherited by the merged command)** (2026-07-01, carried) —
  content kinako-validated; the 2026-07-31 S1 run observed the named behaviors (standing
  producer across the phase boundary · incremental advocate · architect once +
  structural-change re-fire) **but on subagent transport** (F1 — the probe false-passed), so
  the team-form qualifier failed on all. Open: the same checks on proven team transport.
- [ ] **Implement dogfood** (2026-07-01, carried) — content kinako-validated (21 cycles, 407
  tests); open: team-form checks (standing producer across cycles/fix-pass, cold qa, confidence
  gate observed live).
- [ ] **Slice dogfood + unexercised sub-paths (residual D)** (2026-07-02, carried) — content
  kinako-validated (4 slices, extend-mode); open: **null exit** (small spec) ·
  **graded-amendment/breaking change** (mid-flight spec edit) · **`infeasible` escalation** ·
  the **"Research this" → Explore** branch · team-form checks.
- [ ] **Brainstorm v2.2 review-machinery dogfood** (2026-07-16) — the sized lens-split review
  has now run in ~10 pair-form sessions; still open formally: the cost re-measure (target pair
  ≈150–170k; carrier retired at v0.33.0 — rides the OTel probe; one whole-session figure
  exists, operating-docs ≈400k out, not pair-isolated) · the argument-cap watch (~0-for-10,
  ripe to rule) · the one-shot-exchange calibration bet (datapoint 2026-08-01: a cross-exam
  message delayed, not lost — "sender receipts prove neither delivery nor timeliness").
- [ ] **Post-fix confirmation run (residual E)** (2026-07-24) — after the Cluster-2
  ratification lands, re-run the pipeline to confirm the surface-task rule / journey gates /
  residue routing actually catch the kinako gaps.

## Kinako follow-up run

One vehicle, several riders (2026-07-24; oversight trace
`.mochiko/specs/mvp-thin-loop/oversight-trace.md` in the evidence repo; validation-pass record
in this repo's git history at `7920ccb` BACKLOG).

- [ ] **Waves 1–2 didn't land — investigate, then dogfood (residual B)** — kinako artifacts
  show the dense forms ABSENT in S4-era entries, design layer ~28% above the 555k baseline, no
  v0.22/0.23 stamp, run-costs 2/~15 rows. A strip-didn't-land bug is on the table — candidate
  mechanism found 2026-07-31: a stale plugin cache (0.7.0/0.28.0/0.36.0 cached vs the labelled
  v0.38.0, forensics record) — confirm the forms apply, then re-run + re-measure. Acceptance checks + calibration watches (envelope size
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
  the five pipeline stages (verification depth never zero, real-infra floor). Gap sharpened
  2026-08-01 (delivery sweep): the v7 flexibility ruling makes stated defaults lead-departable
  while D5's never-zero floor is encoded nowhere — land it as a floor invariant (P18 carrier).
  Partial landing at the v0.43.0 conversion wave: implement's P18 encodes the never-zero /
  real-infra depth floor verbatim in intent; reviewer-count sizing is now lead-composed under
  v7 stated defaults (U4). Remaining: rule whether v7 composition + the weight card discharges
  D5's sizing-gate generalization for the other four stages — take it at the next groom.
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
  frontend → mobile → desktop (map F30: only backend/service seeded; desktop net-new).
  Frontend (`catalog/frontend.md`; 2026-07-16 user-ruled in-scope): type principles +
  frontend-appropriate floor examples (universal shelf examples are backend-flavored);
  CLI/library shelves retire under PO-D1 — never author speculatively. Mobile (2026-07-21):
  flavored clean-architecture-for-apps card selecting `layer-rules`; acceptance: a Flutter
  setup session deals flavored layered material; carries F6 — greenfield app sessions risk
  backend-flavored paths globs until it lands. (Absorbed the two Deferred-tracks shelf items
  at the 2026-08-01 groom.)
- [ ] **IaC / deployment engineering — staged (Tier II)** (2026-07-30, PO-D5) — stage 1:
  release gates + environment discipline asserted; stage 2: infrastructure-code authoring (new
  artifact class, map F23). Own scoping session; data lifecycle + reliability/resilience ride
  Tier II behind it.
- [ ] **PO watches & revisits** (2026-07-30) — D4.1 waiver-expiry revisit (user-flagged
  "I will come to revisit") · D7 waiver-as-normal-state dogfood watch (if young teams live on
  permanent floor waivers, revisit expiry/maturity design). (The non-legal module waivability
  ambiguity was ruled 2026-07-30 — waivable under D4; ADR `po-narrowing-build-scope`.) D7
  first datapoint (2026-07-31 mochiko-app amend): zero waivers, floor asserted clean —
  pointing away from the concern.

## Ergonomics: output, language & run-hygiene

Raw captures (2026-08-01) — to triage/brainstorm; grooming may re-key into finer themes.

- [ ] **Validator snapshot-isolation watches** (2026-08-01; provenance:
  `validator-worktree-isolation` record, U5 + Open threads — build DONE at v0.42.0, audit
  PASS; trail) — standing watches, per U5's never-close-silently ruling: record every
  implement-run interference observation on the run, composed-in or not · measure per-cycle
  snapshot cost at the first composed-in run (F33 UNVERIFIED) · check non-npm dependency
  behavior at the first non-npm composed-in run (F71; kinako/Flutter the live candidate) ·
  (F66 discharged 2026-08-01: the v0.43.0 wave converted implement, first-conversion
  ceiling terms landed.)
- [ ] **Layer-2 working-tree ownership gap** (2026-08-01; provenance:
  `validator-worktree-isolation` record, Open thread 6) — command-shape Layer 2 says nothing
  about seats sharing one working tree (F19: independence is defined context-scoped) while
  three real authoring-side collisions are on record (F48: the implement/specify/setup strip
  notes, all recovered by falling back to HEAD as baseline). Scope a doctrine line — tree
  ownership / write discipline per seat — kept out of the validator-isolation session's
  validator-only scope by ruling (D1).
- [ ] **Reduce mochiko output verbosity — explore "caveman"** (2026-08-01; provenance: capture
  session, to-brainstorm) — mochiko's user-facing output is too verbose; explore the "caveman"
  terse-output style and how to bring it into `setup` and mochiko generally. Brainstorm: where
  terseness helps vs. where detail is load-bearing, global dial vs. per-surface, and the seam
  with the token-reduction epic (which targets inter-agent/report tokens, not user-facing prose).
- [ ] **Plain-language sweep + internal-jargon leak to end users** (2026-08-01; provenance:
  capture session, to-brainstorm) — the language across mochiko is too complex; needs a
  plain-English sweep. Concrete leak: the plugin's end user is shown "Layer -2" (internal
  shape/architecture vocabulary) which means nothing to them. Sweep finding (2026-08-01):
  "Layer -2" appears in no shipped file — the leak is runtime lead prose, so a file sweep
  won't find it; the fix targets the shape's user-facing vocabulary ban (today an enumerated
  three-term list) and interacts with the R5 record-don't-build ruling. Brainstorm: the
  end-user vs. internal vocabulary boundary, and where the ban's term list should grow.

## Defects & empirical checks

- [ ] **Inter-agent message delivery — content emitted as text, not `SendMessage`** (2026-08-01;
  provenance: capture-session observation, to-brainstorm) — observed buggy behavior: a teammate's
  output reaches the lead as plain assistant text instead of a `SendMessage` tool call, so the
  lead never actually receives it. Verbatim symptom: "The map is complete but never reached you —
  it went out as text, not SendMessage. Sending all four sections now." Distinct from the
  team-form transport items (zero-peer-edges, subagents-vs-teams) which ask whether the team
  exists at all — this is a message-fidelity defect *within* attempted hand-offs: the payload is
  composed but never dispatched through the messaging tool, and the sender only notices after the
  fact. Brainstorm: root cause (affordance/prompt gap that lets a seat narrate a hand-off instead
  of calling `SendMessage`), whether a structural nudge or gate can force tool-call dispatch at
  hand-off boundaries, and detection (a hand-off round that produced no `SendMessage` tool call).
  Relates to Pipeline-dogfood residual A + the orchestration-substrate open decision.
- [ ] **Shape-home keying watch: "Out of rounds = escalate, never done."** (2026-07-30,
  shape-audit advisory at the PO wave) — verbatim shape prose (`command-shape.md` bounds line)
  carried by **4 of 6** commands (setup · plan · slice · specify; line-wrapped — flatten
  before counting). The mandated-exemplar disposition is already recorded
  (`.mochiko/strips/validation-command-shape.md` [v0.34.0], re-ratified [v0.40.0]), so no
  check-8 marker fires by design. Remaining trigger only: raise if a command's Constraints
  block ever needs those words back (setup sits at 4 w of Constraints headroom).
- [ ] **Fresh-session description-delivery probe** (2026-07-25, succinctness R1; re-scoped
  2026-08-01) — total-budget hypothesis dead: no description exceeds 1,536 chars (max 1,517),
  the two repaired descriptions deliver complete, yet **five** skills render name-only
  (validation-constitution · review-slices · review-task-artifacts · testing-end-user ·
  testing-governance-injection; observed from a subagent context, not a cold session).
  Re-scope: diagnose the five-skill absent-fire set in a cold-session probe.
- [ ] **HTML-comment stripping in `.claude/rules/` files** (2026-07-18) — docs confirm
  stripping for CLAUDE.md, silent for rules files. If stripped, in-file trace comments become a
  free option. Method: `InstructionsLoaded` hook or `/context` on a comment-bearing rule.
- [ ] **Fresh-session rules-loading test** (2026-07-18) — the negative probe's confound: the
  rule file was created mid-session. Re-run with the file present before session start; either
  result stands (universal principles stay in CLAUDE.md). Note (2026-08-01): both shipped
  rules files are `paths`-scoped — a different mechanism than the original unconditional
  probe; the re-run needs an unconditional marker file.
- [ ] **Trace-check ergonomics watch** (2026-07-16) — v3 manifest grading: watch for false
  FAILs on trace mechanics vs substance; if the fix-list loop thrashes on mechanics, tighten
  templates before touching the check. First datapoint (2026-07-31 amend run): round 2 of 3
  consumed entirely by trace arithmetic — genuine defects, not false FAILs, but PASS came at
  round 3 of 3, zero headroom.
- [ ] **Experimental-API churn watch** (2026-07-04; re-baselined 2026-08-01 at CC v2.1.220) —
  if an update breaks spawn/messaging semantics, re-verify against docs. Fired once and
  discharged clean: the addressability discriminator rotted, probe rebuilt on the documented
  team-config roster check (v0.38.0) — the worked precedent for the next firing. The commands
  name capabilities, not version mechanics, to keep this surface small.
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
- [ ] **Multi-stack / monorepo registries** (2026-07-21) — per-stack registries, paths scopes,
  seeding. **Revisit: first multi-stack setup run.**
- [ ] **`codebase-inventory-schema.json` port** (2026-06-27; provenance:
  `.mochiko/transform/setup/assess-codebase-inventory-schema.md` — the full draft-07 shape
  survives in-repo; HIL submodule removed 2026-07-21) — the collision/spec-plan-mode contract
  for `analysis-codebase`; port with the spec/plan cluster and wire its consumer then.
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
- [ ] **Context handoff document** (idea) — cross-session serialization; a deliberate add-on
  if pursued (setup/specify dissolved the HIL carrier into the lead).
- [ ] **Deliberate shortcut ledger** (idea; defect-backed 2026-08-01) — deferral tracking with
  upgrade triggers; the parallelism entry above is its first live candidate. Delivery sweep:
  `implement.md:102` names `deliberate-shortcut-ledger` as a live mechanism but no carrier
  exists anywhere (a protected, strip-recorded line pointing at nothing), and ≥5
  ledger-shaped entries sit scattered across records — build the carrier or re-point the line.
