# Changelog

All notable changes to the mochiko plugin. One entry per `plugin.json` version bump —
appending here is release gate 4 (`.mochiko/memory/governance-ledger.md`, GI-010/GI-012).
Entries before 0.53.0 predate this file; their history lives in `ROADMAP.md` stamp lines,
`DECISIONS.md`, and git log.

## [0.62.0] — 2026-08-10

- PM requirements-stacking build (ruling: `pm-requirements-stacking` record D1–D4 + D2a/D3a
  as amended at review). `authoring-feature-map` skill: three-phasing-forms paragraph — an
  extensive feature's phases reuse shipped machinery (within-run = vertical-slice cycles,
  oversize-at-derivation = parent-minting), across-selection-round phasing = leaves under one
  parent with the independently-useful phase-leaf bar (D1, F-6); capability-stub section
  superseded to two-seat minting — specify's derivation may park uncertain remainder as
  `unrefined` stubs with story-trace provenance, selectability and maturation stay
  specify-derivation-only, `/mochiko:feature` stewards-never-matures (D2/D2a; strip
  [v0.62.0]); confidence-keyed cut paragraph (D2); per-parent completeness ledger on the
  selection card + two-site re-surfacing obligation (territory-touching specs +
  stewardship touches, D3/D3a); dependency-triggered escalation split by carrier — leaf via
  technically asserted map relation, shapeless stub via flagged-unverified PM judgment, both
  recommendations for the user's ruling, never a PM-forced cut (D3/D4, F-4/F-7); checklist +
  red-flag reinforcement. Command line-edits (pure additions): `specify.md` selection card
  carries the ledger · `feature.md` stewardship touches re-surface the touched parent's
  parked stubs and undelivered leaves · `plan.md` architecture stage asserts dependency
  relations onto the entry with provenance (technical seat asserts, PM consumes).
  Author≠grader audit: all four surfaces PASS round 1.

## [0.61.0] — 2026-08-10

- Feature-sizing & entry-points build — **breaking change, no migration** (ruling:
  `feature-sizing-and-entry-points` record D1–D15 as amended; D10: the v0.57–v0.59
  spec-folder layout is not read). Plan/implement re-key from spec to feature (D9): entry
  gates on a feature entry carrying ratified scope (a spec's accepted Feature Selection or
  a feature-command delta card); per-feature artifacts re-home to `.mochiko/features/FEAT-XXX/`
  incl. per-feature `requirements.md`; two-altitude design surface — product baselines at
  `.mochiko/product/` (`data-model.md` · `contracts/` · `nfrs.md` ·
  `constraints-and-decisions.md` · `quickstart.md`, `ARCHITECTURE.md` at repo root) + appliable
  before/after per-feature deltas, graded three-way-diff folds at the acceptance landing
  checked by the existing verification seat (D15); cross-spec reach and extend-mode-at-spec-root
  die; spec becomes a pure delivery-event record. New `/mochiko:feature` command (D5–D8 as
  amended): map steward (view/query, `unrefined` stub parking per D12, retroactive promotion,
  retire, integrity grooming) + stable-ground lane triage (D14: feature lane on delivered
  entries, product lane single-flight for cross-cutting defects, in-flight findings file to
  the owning run) + delta-card authoring + dispatch to the re-keyed pipeline — discipline
  floor bound by reference, never restated; lane boundary is the map-write test with
  abort-and-reroute (D7). Map gains two-level nesting (D2–D4): parent capability + leaf
  deliverable, leaf = pipeline unit, hard two-level cap, sticky-delivered roll-up, parents
  minted both directions + retroactive promotion; R5 invariant re-worded (open spec or live
  lane run). Setup bootstraps product baselines (`Assumed`, open thread 4); router + KM
  module updated (lane acceptance is a landing event, D13). Supersessions recorded in strips
  ([v0.61.0]: plan · implement · authoring-feature-map · feature-entry-template ·
  features-index-template · mochiko router), covering feature-map D10/D17/D18/D19/R5 clauses
  + this record's D8 inline-harness architecture (never shipped, no strip owed). Author≠grader
  audits: five commands PASS round 1; map cluster FAIL round 1 (one uncovered reword in the
  skill's strip) → fix → bounded re-audit PASS.

## [0.60.0] — 2026-08-10

- Cold-review gap-challenge build — blind angle map + coverage findings + reopen routing,
  both review clusters (ruling: `cold-review-gap-challenge` record D1–D10, D2/D4 as amended,
  D8 + I4 rider). `review-brainstorm`: new Phase 0 — the reviewer maps expected coverage
  from the topic + free repo grounding (session artifacts excluded) before ever seeing the
  record; map-vs-record diff mints coverage findings, first-class beside the hunt classes,
  admitted by a materiality argument (exempt from the concrete-failure-scenario bar), severity
  by plausibility of ruling change, rejected-roads-checked; hunt class 2 narrowed to
  intra-decision scope (supersession strip `[v0.60.0]` in `.mochiko/strips/review-brainstorm.md`);
  verify pass extends to reopen-born decisions (lighter-review trade-off ruled, one-level
  recursion stop); verdict table admits Critical coverage gaps; description re-fitted at
  1,531/1,536. `brainstorm.md`: two-message reviewer dispatch (topic-only spawn, map back,
  then the record path — blindness lead-enforced), coverage survivors presented per gap with
  the user ruling explore-now / rule-inline / defer (reopens re-enter `analysis-iterative`,
  same D-namespace), reopen-born verify jurisdiction. `review-governance-intent` + `setup.md`:
  same doctrine adapted — the ten-dimension agenda-diff stays primary, the blind map augments
  beyond the agenda, agenda governs on overlap; re-elicited intents land in GI-XXX and ride
  the verify/delta pass. Author≠grader audits: both clusters FAIL round 1 (evidence-bar
  carve-out missing on the setup skill; description 9 over cap; lead-subject wording on the
  reopen ruling in both commands) → fix round → both PASS. First-live-run watch on both
  carriers in BACKLOG (D9).

## [0.59.0] — 2026-08-10

- Feature-map layer wave 4 — brownfield bootstrap + KM wiring (ruling: `feature-map-layer`
  record D11/D12/D16-as-amended/R7/R14/R15; all edits purely additive, no strip notes
  owed). `setup.md`: brownfield runs reconstruct the initial feature map from code (routes,
  UI surfaces, services), confirmed by the user entry by entry, landing as `FEATURES.md` +
  `.mochiko/features/` entries with `delivered` status and the reconstructed-from-code
  mark (first-touch re-verify obligation carried by `authoring-feature-map`); greenfield
  runs scaffold the empty index; the never-overwrite floor covers both writes. KM module
  template: specs-index agreement invariant (open/close contract, rows never contradict
  the map) + FEATURES.md joins the top-level living-doc set marked pipeline-core (never
  scaffolded or declined with the module; map-integrity invariants stay pipeline-side per
  R7) + the R15 boundary line (capabilities on the map, defects/tooling/process in
  BACKLOG). `analysis-codebase`: capability signals seed the reconstruction (one pointer).
  Lead-dispatched author≠grader audit PASS round 1, all three artifacts (the producer's
  self-dispatched audit was not accepted as the ceremony audit). Completes the
  feature-map-layer build: D1–D22 all carried.

## [0.58.0] — 2026-08-10

- Feature-map layer waves 2+3 — the slices→features conversion (ruling: `feature-map-layer`
  record D1–D22). `specify.md` rebuilt: the feature map is an obligated intent-stage read
  (missing map surfaced, never tolerated); after stories, the product-manager seat derives
  features and runs the story filter (rejections recorded, never silent); a user-ruled
  **Feature Selection** replaces the Delivery Slices section (deferred SCs visible at the
  moment of choice); the spec workspace restructures (specs `index.md`, `stories/US-*.md`
  files, map owns status); map writes land only at spec acceptance as one atomic batch;
  migration stance: existing slice-form specs frozen valid, new runs new-form.
  `spec-template.md`: Feature Selection section in, Delivery Slices out, stories section
  becomes an index, header re-keyed to spec vocabulary (`{{spec_title}}`/`{{spec_id}}`).
  `authoring-slices` skill deleted — supersession by ruling, full verbatim preservation,
  per-invariant re-key mapping into `authoring-feature-map`. `plan.md` re-keyed: one run
  per selected feature (FEAT-XXX) in dependency order; the Graduation contract re-keys
  verbatim (shared artifacts extend-in-place at spec root, `[MODIFY]` breaking amendments,
  cross-spec extend reach via owning-spec provenance); in-flight features are readable
  inputs (three-fork resolution, no locks). `implement.md`: the acceptance landing absorbs
  map bookkeeping (status→delivered, delta folds, index touches, in-flight pointer clear,
  derived spec-close) — no separate feature-close stage; regression scope adds accumulated
  delivered-feature gates + later-landing seam ownership; v0.56.0 bounds and snapshot
  isolation preserved byte-identical (audit-verified). `review-specifications`: 10-check
  feature-layer table replaces the Delivery-Slices grade. `authoring-prototype`: FEAT-tag
  re-tag pass at derivation, rejected screens kept greyed. D15 boundary notes on
  `authoring-user-stories`/`authoring-requirements`. Router + `artifact-format.md`
  re-keyed. Strip notes `[v0.58.0]` ×10. Audits: skills PASS round 1 · plan/implement PASS
  round 1 · specify cluster FAIL (unrecorded clause drop, two clobbered prior strip
  headings, template header vocabulary) → fix → delta re-audit PASS.

## [0.57.0] — 2026-08-10

- Feature-map layer wave 1 (pure additions; ruling: `feature-map-layer` record D1–D22,
  DECISIONS.md 2026-08-10). New `product-manager` agent — the product-layer producer: owns
  *which* (feature derivation, the story filter, map writes, selection advice); the
  requirements-analyst owns *how well* under the PM's frame; selection is always the user's
  ruling (D14/D15). New `authoring-feature-map` skill — one living repo-level feature map:
  intent-stage map-read agenda, stories-first derivation with recorded filter rejections,
  FEAT-XXX entry authoring, D8 delta grammar, acceptance-time atomic map writes including
  the specs-index row, map-integrity invariants (incl. in-flight-agreement), foundation as
  an ordering role. New templates `features-index-template.md` (repo-root FEATURES.md — a
  succinct index) and `feature-entry-template.md` (full D6-as-amended entry shape:
  capability, extent, relations, architecture link, story trace, obligations, deltas;
  statuses proposed/in-flight/delivered/retired). Router: skills and agents tables gain
  both rows. plugin.json: agents 9→10. Author≠grader audits: PASS round 1, all four
  artifacts (two advisories logged: specify's wave-2 brief routes spec-index stewardship;
  the skill's split-Process form is record-driven per D5, noted against future form
  audits). Slice machinery untouched this wave — retirement lands with the wave-2 specify
  cluster rebuild.

## [0.56.0] — 2026-08-07

- `implement.md` gains an enforceable bounds contract (pure additions to Harness). New
  **Bounds** bullet: every grading round consumes an attempt whatever its label (default 3
  per cycle, redeclarable at run open); exempting a round is reserved to the user; two
  unchanged-findings rounds is a no-progress stop; test-/records-only changes get a
  delta-grade (no gate re-sweep, prior gate evidence stands); the graded object is the code
  tree, so records-only commits don't move the graded head; round/seat cost surfaced per
  checkpoint. New **Escalation cadence** bullet: reserved-to-user questions batch at the
  cycle checkpoint (build-blockers excepted); Minor advisory findings default to a
  `BACKLOG.md` booking, Important-or-above advisory findings block; gate failures are never
  severity-triaged; `minimalism:` findings stay advisory. Provenance: the mochiko-app
  author-navigate S1 run (R27/R28/R31/R32 "no attempt is consumed" reclassifications) +
  the 2026-08-07 command-text audit. Author≠grader audit FAIL→fix→PASS.

## [0.55.0] — 2026-08-07

- Two native output styles shipped in `output-styles/` (new plugin surface): **Caveman**
  (terse register, baked full level) and **Caveman BLUF** (answer-first BLUF structure +
  caveman diction, BLUF-wins conflict rule). Both `keep-coding-instructions: true`, no
  `force-for-plugin` — user-selectable via `/config` → Output style; main conversation only,
  pipeline reports/artifacts untouched (`templates/output-style.md` still governs those).
- Router skill gains an Output-styles discoverability section (pure addition).
- Design: `.mochiko/brainstorms/plugin-output-styles-delivery/record.md` (D1–D6,
  solo-cold-reviewed 9/9 dispositioned). Author≠grader audit PASS round 1, all three
  artifacts.

## [0.54.0] — 2026-08-06

- `specify.md` gains its missing KM landing Bindings line (governance v1.0.0 validator
  finding — the pin named specify landings but the command carried no reference; pure
  addition, author≠grader audit PASS round 1; pin deviation line struck).
- `marketplace.json` synced 0.10.0 → 0.54.0 — first execution of release gate 5 (GI-016).
- Governance surface set v1.0.0 ratified via first in-repo `/mochiko:setup` run (brownfield):
  CLAUDE.md governance region · governance ledger · KM pin ratified into ruled core ·
  release gates adopted · CHANGELOG elective adopted (this file) ·
  `.claude/settings.local.json` gitignored (GI-015 fix).

## [0.53.0] — 2026-08-05

- Code-minimalism ladder + review lens: `patterns-code-minimalism` + `review-code-minimalism`
  skills minted (26→28); staff/qa personas widened; implement lens wiring. (Pre-CHANGELOG
  entry, reconstructed from the ROADMAP stamp line.)
