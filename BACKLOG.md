# Backlog

The **complete open-set detail store**: every open thread, bounded (title · date · provenance ·
resume-cold context), in theme-keyed sections merged on groom. Never curated — the curated scan
surface is [`ROADMAP.md`](ROADMAP.md). Closing an item = the KM landing ritual
(`.mochiko/memory/knowledge-management.md`): decision row + move here → the trail
(`.mochiko/archive/backlog-trail.md`) + touch ROADMAP.md. No `[x]` lives here.

*Last groomed: 2026-08-16 (landing groom at the schema-based-template-guidance **build** at
v0.76.0, reconciled at merge with the vertical-TDD test-case-anchor build landed the same day
on `main` at v0.75.0) — baseline: **71 open items**. Both builds closed their build item →
trail and minted one first-live-run watch each (template-schema CLI watch + vertical-TDD
test-case-anchor watch); relative to the 2026-08-06 baseline of 56 that is +15, all growth from
recent session landings, none suspected stale. **Delivery sweep still not run — offered to the
user** per the count-watch delta; all items within the ≤15-line bound. Prior groom narrative:
git history (this line, before this groom).*

---

## Command-content schema build

*(ruled + accepted 2026-08-26, `command-content-schema` D1–D13 as review-amended; provenance:
`.mochiko/brainstorms/command-content-schema/record.md`. Build DELIVERED same day at v0.92.0;
D10 rollout DELIVERED same day at v0.95.0 — all six commands now `.md`+schema pairs)*

- [ ] **First-live-run watch: schema-carried rule delivery** (D10) — now spanning all six
  pairs (implement · feature · specify · architecture · setup · brainstorm): delivery probes
  (schema read? read fully? before first action?) · benefit observations (a schema rule ID
  cited by a strip/audit/DECISIONS row · a `vars:` change replacing a multi-site edit · the
  checker catching a real defect · a cross-command label query answering a real question) ·
  D4 edit observations (pair-touching edits without label churn). One watch item, three
  consumers: D2 absorption, D2 retreat, D4 ceremony graduation.

## Skill-compression eval-primitive build

*(ruled 2026-08-22, `skill-compression-tooling` D1–D8 as amended; provenance:
`.mochiko/brainstorms/skill-compression-tooling/record.md` — build surface + folds R1–R16)*

- [ ] **`.claude/skills/compressing-skills/SKILL.md`** — repo-level compressor (never shipped):
  rule inventory (non-author-reviewed, consumer-side check R10) → pre-arm `KEPT:`/protected
  reconciliation (R2) → Arm A densify → Arm B cut-line → eval dispatch → report → user
  ratification → landing hand-off (strips + audit + budget re-seed R11). Description minimal;
  voluntary author≠grader audit (R16).
- [ ] **`evals/` runner** (top-level, never shipped) — probe run first (R5: `stream-json` for
  `plugin_errors`, write allow rules); minimal per-run plugin dir (R4); 4 arms incl. no-skill
  control (R3); pre-registered ship bar + delivered-chars arithmetic before the grid (R6/R9).
- [ ] **Pilot grid** — `patterns-entity-modeling` end-to-end first (fills `primitive-eval-harness`
  D5's deferred pilot slot), then `mochiko` router (Arm A only) + `review-feasibility` —
  **pilot re-purposed 2026-08-22**: the skill took a user-ruled 90% direct cut at v0.82.0
  ("cut now, eval validates later", verbosity-envelope ADR), so its slot runs as a **post-cut
  regression check** (cut-vs-v0.81.0-baseline rule coverage; a lost load-bearing rule re-adds
  via the strips re-add path) — (AR-D3 seeded-defect method is the follow-on if its checklist
  proves insensitive, R13). **`review-brainstorm` joined the post-cut set 2026-08-26** (user-ruled
  true-deletion cut at v0.83.0, rules from the v0.82.0 baseline; ADR
  `.mochiko/decisions/2026-08-26-review-brainstorm-true-deletion-cut.md`).
  **`review-plan-artifacts` joined the post-cut set 2026-08-26** (user-ruled true-deletion
  cut at v0.87.0; its 113-entry `rules.json` already non-compressor-authored from the
  pre-cut baseline; ADR
  `.mochiko/decisions/2026-08-26-review-plan-artifacts-true-deletion-cut.md`).
  **`review-specifications` joined the post-cut set 2026-08-26** (user-ruled true-deletion
  cut at v0.88.0, 81-entry `rules.json` non-compressor-authored; ADR
  `.mochiko/decisions/2026-08-26-review-specifications-true-deletion-cut.md`).
  **`review-governance-intent` (v0.89.0, 70 rules) and `validation-constitution` (v0.90.0,
  69 rules) joined the post-cut set 2026-08-26** — series close-out: every
  `review-*`/`validation-*` skill is now ruled-cut; further reduction routes to the
  eval-graded pilot path (ADRs
  `.mochiko/decisions/2026-08-26-review-governance-intent-true-deletion-cut.md`,
  `.mochiko/decisions/2026-08-26-validation-constitution-true-deletion-cut.md`).

## Plan-retirement residuals

Ruled 2026-08-26 (`plan-stage-utility` D1–D7; D1 `Contested`, D2 `Assumed`), pair-cold-reviewed
(zero outright kills) + verify CLEAN round 3, accepted. **Built same day at v0.91.0** (build
item + discharged dry-run → trail). Open residual:

- [ ] **Sufficiency-check first-live-run watch, bidirectional falsifier** (2026-08-26;
  provenance: record D5 + Open questions + Addendum) — all-pass (no discrimination) or
  all-fail (the design phase fires almost always — plan reborn inside implement) weakens D1:
  revisit at the record. The pre-wave kinako dry-run (R1-2: 3 sufficient / 6 gap, four
  distinct causes, neither arm tripped) is the first datapoint; live runs are the real test.
  Also watched: design-checkpoint attention (rubber-stamp risk) · landing-time design
  surprises on the zero-gap path · gap-finding fence adequacy after `requirements.md` died
  (D3 re-key) · the V1-A1 audit note — `quickstart.md` has no seeding path (D2's sources
  never grade it absent; the null-path record survives on the sufficiency report, but nothing
  seeds the file when a real external-integration surface exists — repair rides the first
  live run that feels it, never invented earlier).

## Product-architecture store build

Ruled 2026-08-19, pair-cold-reviewed + verify CLEAN, accepted (`product-architecture-schema`
D1–D16). **Stage 1 BUILT same day at v0.81.0** (item → trail); Stage 2 + watch remain.

- [ ] **Stage 2 — frontend/mobile/desktop shelf authoring** (2026-08-19; provenance: record
  D15). Pure data authoring, no pipeline coupling, each shelf shippable alone; frontend
  sample in-record is indicative only.
- [ ] **Product-architecture first-live-run watch** (2026-08-19; provenance: record
  Evidence-honesty falsifier). Two-branch falsifier — (a) baseline shelf walk unbearably
  heavy at greenfield, (b) a real plan run proceeds without consulting the store — plus the
  verify-round-2 rider: landing-time `As-built:`/`Drift:` grading cost priced nowhere; watch
  whether it bites. Reopens the store design on trip.

## QA gap-finding residuals

Ruled AND built 2026-08-19, same day, at v0.79.0 (`qa-gap-finding-verification` D1–D10 as
amended at review). One wave under the sound-loop + transport floors — 2 producer seats on
lead-approved plans with strictly disjoint file ownership (new skill vs
implement/agents/router/strips) + 2 fresh author≠grader validator seats: new
`testing-gap-finding` skill (10,559/709, unbudgeted at birth, desc ruled HOLDS) ·
implement.md final-validation wiring (2 strips) · devils-advocate reseat with both
never-zero lines scoped (2 strips) · qa-engineer fold-back duty · router row; V1 PASS +
producer-applied advisories, V2 FAIL (second never-zero carrier + fence delegation guard) →
fix round → CONFIRMED-PASS; cargo test 11/11; the v0.78.0 strip-intro split repaired
wave-wide (12 files, pure relocation); trail. Open residual:

- [ ] **QA gap-finding first-live-run watch** (2026-08-19; provenance:
  [record](.mochiko/brainstorms/qa-gap-finding-verification/record.md) evidence-honesty
  section — n=0) — watch the first selection-scope or epic implement run at v0.79.0+: the
  blind dispatch fires two-message with the fence held (no code/card/TEST path leaks,
  delegated reads inside the inclusion list) · probe-kit yield (does the pass find real gaps
  the deterministic layer missed, or noise?) · mutation-lens noise level and skip honesty ·
  devils-advocate's runtime-probing fit (the D4 reseat is design-reasoned, unmeasured) ·
  finding-kind adjudication and the gap-rework bound in live use · the first `gates.md`
  fold and its later territory re-execution. A pass that yields only noise or breaks the
  fence reopens at the record. F6 note: the skill's plan-time `gates.md` mint parenthetical
  has no plan-side wiring — deliberate; implement mints on the fold if absent; wire at
  plan's next touch only if a real run wants it.

## Vertical-TDD test-case-anchor residuals

Build DELIVERED 2026-08-16 at v0.75.0, same day as the ruling, in the
`vertical-tdd-brainstorm` worktree (12/12 audits PASS round 1; trail). Open item:

- [ ] **Test-case-anchor first-live-run watch** (2026-08-16; provenance:
  [record](.mochiko/brainstorms/vertical-tdd-complexity-and-qa-role/record.md) open
  questions) — n=0: watch the first plan+implement run under the bundle anchor.
  Directional expectation: kinako-s1-shaped work lands well under 12 cycles. Bundle-grain
  guardrail is the revisit if bundles balloon or re-fragment; `Covers` card-level trace
  convention (D4-adjacent, not a TEST-GRAMMAR field) re-examined if parsing friction
  appears; oracle-semantics clause's first grades checked for bite. **Merge reconciliation
  done** (2026-08-16, mochiko-cli←main merge, v0.76.0): the four overlapping files —
  `patterns-vertical-tdd/SKILL.md`, `review-plan-artifacts/SKILL.md`, `plan.md`, and the
  card-shape surface (`tasks-template.md` folded into `schemas/tasks.yaml`) — carry both
  waves; audited PASS at the merge. **First observation logged** (2026-08-19, user dogfood,
  external project): the run generated cycles AND reified "slice" as a unit noun beside
  them — read as vocabulary leakage from doctrine residue, fixed same day by the
  slice-vocabulary purge (v0.80.0, ADR
  `.mochiko/decisions/2026-08-19-slice-vocabulary-purge.md`). Watch stays open for the
  cycle-count/bundle-grain dimensions. *(The "spec dir" residual from that wave's audit was
  discharged by subtraction at v0.81.0 — the phrase died in the `patterns-system-design`
  transform, closing the third F5 limb; strip Entry B records it.)*

## Template-schema CLI build

Ruled AND built 2026-08-16, same day, at v0.76.0 (`schema-based-template-guidance` D1–D11 as
amended at review). One wave under the sound-loop + transport floors — 6 producer seats with
strictly disjoint file ownership (schemas · crate + CI · template deletions + strips · command
re-points · skill re-points + D7 re-key) + 3 fresh author≠grader validator seats: Rust crate
`crates/mochiko-cli` (mochiko's first non-markdown code) + 8 schema data files + 8 template
supersessions (byte-exact strips) + 14-surface re-points + thin D7 re-key + governance PATCH
2.0.1 activating the AM-1 dormant crate gates; audits V1 crate PASS + delta-confirm, V2 8/8
fidelity + 8/8 strips, V3 13/13 re-points, `authoring-feature-map` budget overage HOLDS, one
fix round (CI `cargo audit --deny`); trail. Open residual:

- [ ] **Template-schema first-live-run watch** (2026-08-16; provenance:
  [record](.mochiko/brainstorms/schema-based-template-guidance/record.md) evidence-honesty
  thread — n=0, the D11 null-road concession on record) — no run yet demonstrates CLI/schema-
  delivered guidance outperforms the old `.md` exemplars. Watch the first live authoring runs on
  the 8 converted templates (specify/plan/feature/setup + the skills that read them): does the
  producer view (schema + example + good/bad) and the `--check` checklist view guide artifact
  quality at least as well as the `.md` baseline, and does the raw-Read fallback stay honest when
  the binary is absent (D8/GI-020)? **M7 rollback trigger:** CLI-delivered guidance
  underperforming the `.md` baseline on artifact quality reverts the 8 supersessions
  (reconstructible from strips, GI-006) and re-points skills back; the crate may survive under
  D11's foundation ruling. **D5 reopen condition:** if a real per-project depth need emerges (the
  governance depth dial and template guidance visibly diverge), the baked-in-norms ruling reopens
  by explicit user ruling only.

## Adopt-first build

Ruled AND built 2026-08-15, same day, at v0.73.0 (`build-vs-off-the-shelf` D1–D6 as amended
at review; wave: 2 producer seats on lead-approved plans with disjoint file ownership + 2
fresh author≠grader validator seats, 13/13 PASS round 1, zero fix rounds; trail). C4
cross-pointer note stands for the queued Tier-I builds: `patterns-adopt-first` shipped first,
so the STACK-TOOLING builds owe the pointer back when they land. Open residual:

- [ ] **Adopt-first first-live-run probe** (2026-08-15; provenance: record D6, n=1 marker) —
  the kinako FEAT-006 re-plan runs under the discipline once built; never blocked on it (if
  the re-plan runs first, the watch moves to the next plan run). Directional expectation:
  superseding D-rows name shelf candidates in alternative sets AND rationale · the D4 gate
  fires on the storage decisions · the D3 valve is exercised against the amended constraint
  line. Failure criterion (C13): a post-build plan run hand-rolling solved-category
  infrastructure without the disclosure firing = the discipline failed, instrument revisited;
  ceremony-only disclosure lines (restating an already-ruled choice) are also a defect
  signal. Side-flag to kinako at the re-plan: their transition record's registry-scope
  overread (record F5). Audit-added watch (v0.73.0 validator, advisory): plan-minimalism's
  "Read before you claim" line stays repo-bounded while widened rung 3 admits outside-repo
  claims — verification is covered via `patterns-adopt-first`'s external-claims binding, but
  check at this probe whether any rung-3 adopt claim ships without its disclosure line; only
  then amend the read-obligation line.

## Epic build

Ruled AND built 2026-08-14, same day, at v0.72.0 (`multi-feature-plan-implement` D1–D13;
wave: 2 producer seats on lead-approved plans with disjoint file ownership + 2 fresh
author≠grader validator seats, 8/8 PASS round 1, zero fix rounds, feature-map 248-char
overage ruled HOLDS; trail). Open residual:

- [ ] **Epic first-live-run watch** (2026-08-14; provenance:
  [multi-feature-plan-implement](.mochiko/brainstorms/multi-feature-plan-implement/record.md)
  D9 n=0 honesty) — watch the first live epic end-to-end (mint → contest → plan → implement →
  landing); every ruling is design-reasoned, no live evidence. D12 rider: an epic dogfood is
  a likely tripper of the "sequential-cycle-too-slow" revisit bet — surface it if felt.

## Teammate-transport floor build

Build DELIVERED 2026-08-14 at v0.71.0 (same-day as the ruling; the wave ran under the floor it
ships — 2 plan-approved producer seats with disjoint file ownership + 2 fresh author≠grader
validator seats, 2/2 PASS round 1, zero fix rounds; trail). Open residual:

- [ ] **Transport-floor first-live-run watch** (2026-08-14; provenance:
  [teammate-message-races](.mochiko/brainstorms/teammate-message-races/record.md) D7 `Assumed`
  + CV4 limitation) — watch the floor's first live multi-seat run (any command composing >1
  seat): trigger lanes fire as split-keyed (message legs on messaging alone, topology legs on
  shared writes) · composition steer consulted at composition time · fan-in confirmation held
  at every convergence · disclosure honest. Scope named on D7's face: the watch verifies the
  floor **fired and was followed** — one clean run is weak evidence for a nondeterministic
  class, absence-of-race is not proven. Revisit trigger: a race despite the floor → reopen at
  the record. The narrated-not-dispatched defect (Defects section) shares leg 7 as detector —
  cross-note there. Version-floor re-verify rides the Experimental-API churn watch, not here.

## Charter ritual-floor build

Build DELIVERED 2026-08-13 at v0.70.0 (same-day as the ruling; the wave ran under the floor it
ships — 2 plan-approved producer seats + 2 fresh author≠grader validator seats, 2/2 PASS round 1;
trail). Open residual:

- [ ] **D5 rules-file leg (first-miss trigger) + first-live-run watch** (2026-08-13; provenance:
  [charter-ritual-balance](.mochiko/brainstorms/charter-ritual-balance/record.md) D5 `Contested` +
  D7) — the path-injected `.claude/rules/mochiko/sound-loop.md` leg (setup-scaffolded, path-scoped
  to the governing surfaces) stays deliberately unbuilt; **the first observed floor miss in live
  use — a judgment-authored governing-surface write that runs without the loop — builds it
  immediately, no new session** (trigger on record). Watch the floor's first live trips (a desk
  visit with a qualifying write · the first ad-hoc governing-surface write): disclosure line
  present, named seats real, review leg run. D7 declined a session probe; this watch is standard
  build hygiene only.

## PM-role & capability-map rebuild

Build DELIVERED 2026-08-13 at v0.68.0 (same-day as the ruling; DM-charter-form wave; trail).
Open residuals:

- [ ] **D11 probe + first-fold watch** (2026-08-13; provenance: record D11) — post-build
  validation gate: kinako map re-derivation probe (directional ~3–4 capabilities + work rows vs
  the current 10 entries; also discriminates D1's undiscriminated causal ranking) + the first
  implement landing that executes a row fold (fold mechanics + the convergence claim's first
  real data). A probe failure re-opens the D1/D2 structural bet, not just the build.

## Guardrails-vs-detail residuals

Ruling: DECISIONS.md rows 2026-08-10 (D1–D8 + benchmark verdict) + build row 2026-08-11;
Wave 1 delivered at v0.63.0 (trail).

- [ ] **First-live-run watches on the v0.63.0/v0.64.0 cuts** (2026-08-11; provenance:
  `.mochiko/brainstorms/validator-scope-and-verbosity/record.md`, Benchmark execution section +
  `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`; extended to the Wave 2
  substrate at v0.64.0) — watch the first live runs on the cut primitives: **F-X1
  review-evidence** — does the independent review leave verdict + dispositions in the
  artifacts (the floor line's job, now in 4 review/validation skills + the equivalent in
  `review-code-minimalism`)? Recurrence re-opens agents ruling (b) (example blocks return to
  the 10 prose-only agent descriptions) · **slim-description fire-rate** — do the 27 slim
  descriptions still fire at their moments? Wave 1's 11 are benchmark-verified, Wave 2's 16
  probe-verified (14/14 blind routing); a live miss on any lands as a probe-evidenced
  RETURNED clause, not a description rewrite · **M2** audit-substrate shrink (terser bodies
  give the preserved-responsibilities check less to grade; two datapoints held — both waves'
  audits graded from strip inventories). M1 retired at v0.64.0 (all near-cap descriptions
  slimmed + probe-verified). Advisory riding: dropped SHOULD-class trigger phrases inventoried
  in the Wave 2 audit reports — restore individually on live-miss evidence only.
- [ ] **`analysis-codebase` dangling-pointer repair** (2026-08-11; provenance: batch-A audit
  ruling, strip `[v0.63.0]` note) — the ruled variant keeps "assess against it using the
  indicators below" but the indicator subsections died; comprehension defect only, no lost
  obligation. One-line reword through the ordinary edit ceremony at the next touch of this
  skill.
- [ ] **Review-evidence floor-line consolidation** (2026-08-11; provenance: Wave 1 user ruling
  option note + Wave 2 spread) — the floor line now lives in 4 skills verbatim
  (`review-specifications`, `review-governance-intent`, `validation-constitution`,
  `review-brainstorm`, plus adapted copies in `review-feasibility`, `review-plan-artifacts`);
  no shared review-contract home exists. Candidate: single-source it when a shared home
  naturally appears — not worth minting a new primitive for.

## Feature-sizing & entry-points residuals

Ruling: DECISIONS.md row 2026-08-10 (D1–D15 as amended); build delivered same day at v0.61.0 (trail).

- [ ] **First live feature-surface run on the v0.61.0 re-key** (2026-08-10; provenance:
  `.mochiko/brainstorms/feature-sizing-and-entry-points/record.md`, open threads 1/3/5/6) —
  watch the first `/mochiko:feature` lane run and first re-keyed plan/implement run: stable-ground
  triage keys honestly (D14, keying audited from the delta) · map-write test + abort-and-reroute
  fire (D7) · graded three-way-diff fold holds at landing (D15) · delta-scope plan genuinely
  collapses (D8). Open threads riding: parent selection semantics (user-reserved — build left
  leaves-only selection, per-child pick on a named parent, no default encoded) · `unrefined`
  stub lifecycle (no expiry rule; grooming surfaces, never auto-retires) · size signal at
  selection · D11 concurrent-fold watch (`Assumed` — first real concurrent fold re-opens it).
  2026-08-13: **re-scoped** — the v0.61.0 surface this watches is superseded by the
  capability-map rebuild (pm-role-and-feature-derivation D6); the watch transfers to the
  rebuilt surface and rides that section's D11 probe + first-fold item from its build on.
- [ ] **Setup baseline-bootstrap hardening** (2026-08-10; provenance: record open thread 4,
  D10 bootstrap clause `Assumed`) — the shipped clause (brownfield reconstructs the five
  product baselines at `.mochiko/product/` from code; greenfield seeds at first plan run) is
  marked `Assumed` in `setup.md`; reconstruction-burden scope unruled; half-reconstructed
  baselines poison extend-mode. Re-opens at the first brownfield setup run on the new surface
  or on friction, whichever first. 2026-08-11: the greenfield arm's enforcement half closed
  at v0.66.0 (dogfood defect — first plan run never seeded; `plan.md` Baseline-seed binding +
  `implement.md` empty-pre-fold clause, ADR
  `2026-08-11-plan-baseline-seed-enforced`); the partial-baseline risk and the brownfield
  reconstruction-burden scope stay open here.

## Cold-review gap-challenge residuals

Ruling: DECISIONS.md row 2026-08-10 (D1–D10); build delivered same day at v0.60.0 (trail).

- [ ] **First-live-run watch, both carriers** (2026-08-10; provenance: D9): watch the first live brainstorm review and first live setup G3 review post-build — does the blind map produce material coverage findings (not generic noise), does reopen routing fire, does the materiality bar hold solo? Feeds the record's I8 padding open thread too.

## Feature-map layer residuals

Ruling: DECISIONS.md row 2026-08-10 (D1–D22); build delivered 2026-08-10 at v0.57.0–v0.59.0 (trail).

- [ ] **Self-application: mochiko's own governance amend for the feature layer** (2026-08-10;
  provenance: `.mochiko/brainstorms/feature-map-layer/record.md`, R12 disposition) — the
  KM-module changes shipped in the plugin template (specs-index agreement invariant ·
  FEATURES.md top-level reservation · R15 map-vs-BACKLOG boundary) reach mochiko's own
  project-pinned `.mochiko/memory/knowledge-management.md` only via a `/mochiko:setup` amend
  run — a governance event (fact-profile/module change) the user rules. Until it runs, the
  dogfood repo's pinned copy predates the feature layer. Note: mochiko itself is a primitive
  library, not a product app — whether it also gets a FEATURES.md is part of the amend ruling.
- [ ] **First live feature-surface pipeline run** (2026-08-10; provenance: the record's open
  threads 1/3/5 + wave audits' advisories) — watch the first specify→plan→implement run on
  the v0.59.0 surface: intent map-read fires (greenfield empty vs brownfield reconstructed) ·
  derivation + filter produce honest rejections · selection card shows deferred SCs ·
  acceptance-time atomic write (map untouched on a rejected spec) · per-feature plan runs
  resolve extend-mode + cross-spec reach · implement landing executes the graduation batch
  whole · D6 entry shape survives real use (Assumed — re-opens on friction) · oversized
  feature (open thread 1) and selection-UX presentation (thread 5) observed. Scale trigger:
  map at ~60 entries (thread 6) — far off.

## UX-prototype-stage watches

Ruling: DECISIONS.md row 2026-08-02 (UX-D1–D9); built same day at v0.50.0.

- [ ] **First live UX-bearing specify run** (2026-08-02; provenance:
  `.mochiko/brainstorms/ux-mocking-in-specify/record.md`, open threads 1–3) — watch the first
  UX-bearing run: intent stage rules UX-bearing sanely on non-obvious features · lockstep
  holds (screens land with their story, not batched after) · design-system discovery
  (`authoring-prototype` step 2) actually finds an existing system · bun/file:// degrade path
  suffices on the user's machine · devils-advocate genuinely serves and walks the prototype
  (Playwright) rather than text-reading the manifest · plan traces SCR/FLOW into
  contracts/data-model · a FLOW-XXX TEST gate executes against the built app at implement.
  Open sub-thread: prototype afterlife (archive-or-keep once the real UI ships) — unruled,
  decide at first feature-close on the new surface.

## Task-granularity / slice-dissolution watches

Ruling: DECISIONS.md row 2026-08-02 (D1–D9); built same day at v0.49.0 (trail).

- [ ] **First live v0.49.0 pipeline run** (2026-08-02; provenance:
  `.mochiko/brainstorms/plan-task-granularity/record.md`; absorbed the slice-dogfood
  survivors at the 2026-08-04 groom) — watch the first specify/plan/implement run on the new
  surface: intent stage fires adaptive (not questionnaire), Delivery Slices co-accepted,
  cycle cards graded by the package reviewers, builder decomposition disclosed in cycle
  reports, slice-scoped runs resolve the Graduation contract from the spec section. Folded
  slice sub-paths (ex-residual D): single-slice null exit (small spec) · spec-amend on a
  mid-flight edit (graded-amendment machinery died with the overlay — watch what replaces
  it) · plan's `infeasible` escalation.

## Adversarial-review ceiling benchmark

Ruling: DECISIONS.md row 2026-08-04 (AR-D1–D6); test deliberately parked for a dedicated session (AR-D5).

- [ ] **Run the seeded-defect benchmark** (2026-08-04; provenance:
  `.mochiko/brainstorms/adversarial-review-generality/record.md`, AR-D3/D4/D6 as amended at
  review) — cold-runnable spec: one real spec artifact (kinako or mochiko-app); independent
  seeder pre-sweeps (labels pre-existing defects) then injects ~10 seeds, ~60/40
  in/off-taxonomy, off-taxonomy seeds **off-list-but-in-jurisdiction only** (wrong domain
  model, mis-modeled actor incentives, missing regulatory/product-legal constraint,
  cross-feature P1-journey interaction; technical shapes banned per F1 — historical dogfood
  misses pass the same filter). Four arms on `review-specifications`: baseline · A floor-line
  · B free-hunt-first · C disjoint lens briefs; ≥2 replicates/arm, hit = strict majority
  (2 replicates = both). Scoring: separate cold scorer holds seed key (never seeder/arm);
  match = location + substance; borderlines escalate to user. Verdict shape (AR-D6):
  off-taxonomy recall primary · in-taxonomy non-regression + precision guards · per-arm
  counts reported · null → no build. A+B composite a build-time option on a B win. Extension
  to other review skills staged on a positive result (review-brainstorm second, only then).

## Open design decisions

- [ ] **Prose vs. gate allocation** (2026-06-27; provenance: unrecoverable —
  `agent-skills-research/synthesis/my-framework.md`, submodule removed 2026-07-21) — which
  behaviors earn graded anti-rationalization prose vs a hard `PreToolUse` hook? Kernel excluded;
  allocate between prose and hook.
- [ ] **Claude-Code portability** (2026-06-27; provenance: unrecoverable — same source) —
  `rfc2119-invocation-trigger` + `disable-model-invocation` are CC-specific: adopt-and-bind or
  abstract? Surfaces when the router evolves.
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
- [ ] **Governance first-live-run watches** (2026-08-06, governance v1.0.0, DECISIONS row) —
  standing amend triggers to observe: public-product transition (GI-002) · CI arrival un-narrows
  FLOOR-SEC scanning (GI-003) · GLOSSARY.md content → scaffold (GI-009) · helper-script waiver
  trigger (GI-008) · evolution-notes/layer-rules remain offerable on amend (GI-013/014, FP-1).
  (Gates 4+5 executed first time at the v0.54.0 bump 2026-08-06 — `marketplace.json` synced
  0.10.0→0.54.0, CHANGELOG entry appended; the obligation is now routine per-bump, GI-012.)
- [ ] **Expansion-heavy-surface watch HIT — ROADMAP stamp line vs CHANGELOG** (2026-08-06;
  provenance: groom-pass watch, `grooming-operating-docs` step 8; recorded re-open trigger for
  the report-writer ruling, OD-D12 + the "Scribe/report-writer closed" standing bet) — the
  ROADMAP last-groomed stamp had grown to a ~700-word per-version build changelog whose every
  clause was a mechanical derivation from DECISIONS rows, trail entries, and strip notes; with
  `CHANGELOG.md` adopted (GI-010, 2026-08-06) the per-bump detail now has a dedicated owner,
  and the 0.53.0 CHANGELOG entry was itself "reconstructed from the ROADMAP stamp line" —
  derivation running in both directions. Logged for the user per the skill's never-act-here
  rule: decide whether this re-opens the report-writer/scribe question or whether the fix is
  purely contractual (stamp = date + baseline figures only; per-bump detail lives in
  CHANGELOG). The 2026-08-06 groom compressed the stamp to contract shape either way.
- [ ] **Brainstorm bookkeeping watch** (2026-07-17, carried) — on a KM project:
  read-index-before-open fires, entries land/update with named landings, close invariants catch
  a seeded defect, the no-module branch stays silent. Honest residual: drift on a session-less
  project is caught only at the next setup/amend re-audit.

## Team-method build items

Rulings D1–D5 (DECISIONS.md 2026-07-25; record `team-method-vs-command-shape` — bare session,
un-reviewed). Build items DONE 2026-07-31 at v0.39.0 → trail; the deferred direction below is
the open remainder.

- [ ] **Deferred direction (recorded, not ruled)** — slice-fold delivered 2026-08-02
  (`/mochiko:slice` dissolved into specify at v0.49.0, commands 6→5); remaining: build-room
  merge (structuring seat inside implement) + multi-stream implement (frozen seams ·
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
  `ARCHITECTURE.md` fold staying distinct (no implement run yet). (Team-form rider dropped
  2026-08-04 groom — transport-neutral ruled 2026-08-02, command-architecture-realignment D5.)

## Pipeline dogfood

Every command passed structural verification + the kinako artifact pass on content. Named
per-command checks: archive `DECISIONS.md` + each command's strip note. Kinako artifact
evidence: the 2026-07-24 validation pass (evidence repo `humaninloop-dev/kinako`). The
team-form qualifier on these checks died 2026-08-04 groom (transport-neutral ruled
2026-08-02, D5) — open checks re-read as behavior checks on whatever transport the lead
composes; the v8 form means each command's next run is also its v8 first-live-run.

- [ ] **Setup v3 dogfood** (2026-07-18, carried) — live-run residue only (authoring half
  kinako-validated). Partial credit (2026-07-31 mochiko-app *amend*): G3 sizing gate fired ·
  injection-probe G5 offered and ran (residual C met). Open: validator catches a seeded
  missing-companion · downstream producer receives CLAUDE.md governance natively · region
  regenerates idempotently (amend showed non-destructive only) · waiver + delta-pass
  branches · KM dimension-7 + collision beats — and the fresh-setup form itself.
- [ ] **Specify dogfood** (2026-06-27, carried) — loop content kinako-validated (3 rounds to
  `ready`); open: standing producer messaged round > 1 · critic cold — plus the v0.49.0/v0.50.0
  first-live-run watches above (intent stage, Delivery Slices, UX-prototype).
- [ ] **Plan dogfood (+ tasks, inherited by the merged command)** (2026-07-01, carried) —
  content kinako-validated; the 2026-07-31 S1 run observed the named behaviors (standing
  producer across the phase boundary · incremental advocate · architect once +
  structural-change re-fire) on subagent transport — now legitimate under D5; open: the same
  behaviors on the v8 cycle-card form (first v8 plan run).
- [ ] **Implement dogfood** (2026-07-01, carried) — content kinako-validated (21 cycles, 407
  tests); open: standing producer across cycles/fix-pass · cold qa · builder decomposition
  disclosed (v0.49.0) · dependency-cold snapshot final validation observed live.
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

- [ ] **Cluster-2 ratification wave** — pressure-tested, awaiting founder ratification; lands
  as one edit wave (author ≠ grader), ruling → DECISIONS.md row. Items (owner · n): 
  surface-task rule (`patterns-vertical-tdd` + `review-plan-artifacts`, which absorbed
  `review-task-artifacts` at v0.49.0; n=2) · journey-gate
  class, graded form (`testing-end-user` + `qa-engineer`; blocks feature-close; n=5) · two
  spec-review hunt classes — display meaningfulness + lifecycle twins (`review-specifications`;
  n=2; hunt-list growth guard n≥2 rides along) · split-gate assert-union invariant
  (formerly `loop-discipline`; owner deleted at the 2026-08-02 doctrine purge — re-home to
  the commands' gate lines or drop at ratification; n=1, zero-cost) · runbook walked-stamp (landing surface ruled at
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
together. Model-tiered D4 explorer build DELIVERED 2026-08-16 at v0.77.0, retargeted post-v8
(`mochiko:explorer` haiku seat + `patterns-model-tiering` floor + six command floor lines; ADR
`2026-08-16-model-tiering-build`); cheap rung RETARGETED 2026-08-19 at v0.78.0 — the
`mochiko:explorer` seat deleted (agent-team teammates cannot spawn plugin-scoped agents),
native `Explore` + explicit `model: haiku` override now the rung (ADR
`2026-08-19-explorer-retarget-native`); D5 seat-tiering stays deferred (Later), D6's three watches
ride the probe below — D6-ii already positively evidenced by the in-session spawn test. Seam note (2026-08-01, `verbosity-caveman-ops-separation` D8): epic D3's
conditional-prose intent was **finished by the output-style build wave 1** (v0.44.0) — never a
wave-3 candidate again; the always-read floor re-baselined at that build (−4,490 B/run on
`command-shape.md`). Epic D5 (sizing-gate generalization) closed 2026-08-04 — superseded at
the v8 rebuild (trail; ADR `2026-08-04-groom-epic-closures`).

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
  it, the other joins. Two-row obligation (adaptive-depth 2026-08-11, #6 fold): every check
  this build ships gets a low/high determination at build time under the D5 retrofit-cost cut
  line; scoping record not reopened.
- [ ] **Ops & observability build (Tier I)** (2026-07-31; rulings OO-D1–D7, record
  `.mochiko/brainstorms/ops-observability-hardening/record.md` — pair-reviewed 25→20 merged,
  20/20 dispositioned, verify CLEAN round 2) — build surface (record: "Build surface", M-folds
  incorporated): fifth floor category **Operations** + FLOOR-OPS card across the nine-file
  enumeration ripple (incl. `principal-architect.md` keystone-checked · both QUALITY-CHECKLIST
  lines · the `plugins/mochiko/schemas/codebase-analysis.yaml` Part 2 Essential Floor Status
  category rollup, ex-`codebase-analysis-template.md:118`) · FLOOR-OBS established-tooling line ·
  dimension-8 SLO beats → region one-liner + `paths`-scoped SLO rules file over
  `.mochiko/specs/**` · RUNBOOK contract in KM + dual-key `implement.md` landing fold (the one
  command edit, shape-v5 audited) · release-gates reference line · `STACK-TOOLING.md` universal
  home (create-or-join) + OTel disambiguation · feasibility SLO row + plan-artifacts coverage
  row · vertical-tdd instrumentation TEST rule + task-artifacts dual-keyed row · known-gap
  validator check · clause-level waiver-key wording · bounded alert canary. Fences:
  release-process assertion · SLO achievement · incident machinery + error budgets (Tier-II
  reliability) · per-kind shelf expressions · dormant IP rows stay dormant. Two-row obligation
  (adaptive-depth 2026-08-11, #6 fold): every check this build ships gets a low/high
  determination at build time under the D5 retrofit-cost cut line; scoping record not
  reopened.
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
- [ ] **Adaptive-depth first-live-run watch** (2026-08-11; provenance:
  `.mochiko/brainstorms/production-floor-adaptive-depth/record.md` + DECISIONS build row
  2026-08-11; build DONE same day at v0.65.0 → trail) — watch the first live setup runs on
  the two-row floor: greenfield run declares a level recommend-then-arbitrate (low
  recommended, user rules, GI-row + ledger state minted) · a low-level run's floor cards
  author the low rows (no high-row leakage, no watcher language surfacing) · the first flip
  (high-mode rerun) surfaces the adherence delta and mints interim transition-delta waivers
  that die at conformance · graders accept a fresh high declaration with open transition
  waivers (not a defect) and FAIL a missing declaration. D6 silent-under-posture risk
  (`Contested`, accepted): note any observed cruise-past-the-flip-moment as evidence for the
  user's future revisit, never as a build change. This repo's own formal GI-row minting rides
  the next amend run (ledger carries the legacy default-high note).

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
- [ ] **Output-style post-build remainders** (2026-08-01; build DONE at v0.44.0 → trail;
  provenance: `verbosity-caveman-ops-separation` record + DECISIONS build row) — was three
  riders; **rider (1), the D6 trio move, closed 2026-08-02 by supersession**: probe re-run
  fresh-session PASSED both halves, but the user ruled delete-outright over the move — trio
  deleted at v0.45.0, probe pair deleted, fourth-consumer edit landed (DECISIONS row + ADR
  `2026-08-02-framework-trio-deleted`). Two riders remain: **(2) Measurement watch** — re-profile
  the first comparable slice-run against the 816,601 B / 79.9%-report-prose baseline
  (F53–F55 method); always-read floor re-baselined: `command-shape.md` 26,745 B (−4,490
  B/run). **(3) Style dogfood** — first setup/amend run writes the switch line + Shape-5
  rules file into a real project; watch the preserve-on-regenerate carve-out and disclose-once
  fire as designed.
- [ ] **Plain-language sweep + internal-jargon leak to end users** (2026-08-01; provenance:
  capture session; amended 2026-08-01 by `verbosity-caveman-ops-separation` D3) — the
  language across mochiko is too complex; needs a plain-English sweep. Concrete leak: the
  plugin's end user is shown "Layer -2" (internal vocabulary); it appears in no shipped file —
  the leak is runtime lead prose, so a file sweep won't find it. **The rule's home is now
  ruled** (D3): the style home carries plain-English-for-end-users + the ban as a principle
  with non-exhaustive examples (lands at the build's wave 2). **Remaining here: the sweep
  work itself** — term hunting across runtime prose, worked examples for the ban's list.

## Defects & empirical checks

- [ ] **ARCHITECTURE.md staleness residue (pre-v0.68.0 debt)** (2026-08-13; provenance: the
  v0.68.0 ripple audit's fix #1 — precise inventory) — line-3 header still stamps
  "(v0.48.0…)" (20 versions stale) · System-overview pipeline line reads "governance → spec →
  slices → plan" and the pipeline mermaid keeps the "per slice when the spec's Delivery
  Slices section decomposes" edge (slices died v0.49.0/v0.57.0) · the Plan section's "Four
  producer stages … architecture is the first design artifact" predates the v0.67.0
  plan-the-plan proposal gate (artifact set is proposal-scaled, architecture conditional) ·
  the Templates row still says "spec — Intent + Delivery Slices sections included". Counts
  were already fixed at the v0.68.0 landing (6 commands / 30 skills, delta-confirmed); the
  Commands-row / Command-form / Feature-section anatomy claims were re-worded to the
  two-anatomy reality at the v0.69.0 charter-extension landing. One grooming pass owns the
  rest; craft home `mochiko:authoring-architecture-store` (predecessor retired v0.81.0).

- [ ] **Teammate hand-off narrated as text, never dispatched via `SendMessage`** (2026-08-04;
  provenance: capture-session observation, to-brainstorm; title line reconstructed at the
  2026-08-05 defect close — the original was clobbered when the orphan-artifacts entry landed
  above it) — observed buggy behavior: a teammate's
  output reaches the lead as plain assistant text instead of a `SendMessage` tool call, so the
  lead never actually receives it. Verbatim symptom: "The map is complete but never reached you —
  it went out as text, not SendMessage. Sending all four sections now." Distinct from the
  team-form transport items (zero-peer-edges, subagents-vs-teams) which ask whether the team
  exists at all — this is a message-fidelity defect *within* attempted hand-offs: the payload is
  composed but never dispatched through the messaging tool, and the sender only notices after the
  fact. Brainstorm: root cause (affordance/prompt gap that lets a seat narrate a hand-off instead
  of calling `SendMessage`), whether a structural nudge or gate can force tool-call dispatch at
  hand-off boundaries, and detection (a hand-off round that produced no `SendMessage` tool call).
  (Its former companions — residual A + the substrate decision — closed 2026-08-04, superseded
  by transport-neutral D5.) Related 2026-08-14 (`teammate-message-races` record, CV3): the
  transport floor's **leg 7 fan-in confirmation is this defect's detector** — a hand-off round
  whose expected deliverable never arrives fails the fan-in confirm regardless of cause
  (narrated-not-dispatched, silent drop, died seat); the brainstorm scope here (root cause +
  structural nudge at hand-off boundaries) stays open.
- [ ] **Fresh-session description-delivery probe** (2026-07-25, succinctness R1; re-scoped
  2026-08-01; set shrunk 2026-08-04 groom) — total-budget hypothesis dead: no description
  exceeds 1,536 chars (max 1,517), the two repaired descriptions deliver complete, yet skills
  render name-only (observed from a subagent context, not a cold session). Affected set now
  **three** — validation-constitution · testing-end-user · testing-governance-injection
  (review-slices + review-task-artifacts deleted at v0.49.0). Re-scope: diagnose the
  absent-fire set in a cold-session probe.
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
  team-config roster check (v0.38.0) — the worked precedent for the next firing. (The roster
  probe itself died with the team mandate at the v8 rebuild, 2026-08-02 D5; the watch
  survives — commands still name spawn/messaging capabilities.) Rider (2026-08-14,
  `teammate-message-races` D6): this watch carries the transport floor's **version-floor
  re-verify** — agent-teams ≥v2.1.224 (below it, sends report success on failed mailbox
  writes) — plus the per-transport semantics the floor cites (teammate delivery
  documented-automatic, ordering undocumented); re-verify both at the next firing, no
  parallel watch.
- [ ] **Two-level "slice" vocabulary watch** (2026-07-02) — graduation slice (spec-level) vs
  vertical slice (cycle-level); if dogfooding shows conflation, sharpen triggers or rename one
  level.

## Deferred tracks & shelves

- [ ] **`audit` workflow scoping** (2026-07-02 charter) — feature-close verification:
  qa-engineer executes `slices.md`'s Feature-Done section (SC coverage + cross-slice seams)
  against real infra; human FEATURE-DONE gate. Scoping decides whole-workflow vs branch; the
  journey-gate class (Cluster 2) gives the pass its hard gate class; the cold-checkout step
  (ruled into implement's final validation 2026-07-31) migrates here if audit takes feature-close.
  Parked rider (AT-D5, 2026-08-04, `architecture-tieback` record): at feature close, before the
  `ARCHITECTURE.md` In-flight pointer is removed, diff shipped code vs the accumulated
  feature-root `architecture.md` — catches descoped/partially-built slices per-slice diffs miss.
  Rider (2026-08-05, ADR `2026-08-05-orphan-plan-artifacts`): runtime NFR verification —
  p95/availability targets from `nfrs.md` re-checked against the built system — joins the
  feature-close verification scope when audit is scoped (no TEST-grammar NFR assert exists;
  implement got the minimal Design-inputs wire only). **NFR rider discharged 2026-08-19**
  (`qa-gap-finding-verification` D9): runtime NFR verification now runs inside the
  gap-finding pass at implement's final validation; it migrates with the pass. **Rider
  (2026-08-19, same record D2/I8):** the gap-finding pass itself homes in implement's final
  validation now and **migrates here if audit takes feature-close** — same clause as the
  cold-checkout step; the journey-gate class stays with this track, unabsorbed by the pass.
- [ ] **Feature-close verification has no owning workflow** (2026-07-02) — until audit owns
  it, the section is executable by hand; implement surfaces "declared, not verified" after the
  last slice.
- [ ] **Design track** (2026-06-27, carried; archive REGISTRY disposition) — `ui-designer` +
  `analysis-screenshot` / `authoring-design-system` / `patterns-flow-mapping` /
  `patterns-interface-design`; port as a dedicated cluster (HIL sources listed in the archive).
- [ ] **Design/UX skills for product engineer** (2026-08-04; provenance: capture session,
  to-brainstorm; re-keyed here from To-triage at the 2026-08-06 groom) — add design + UX
  competence to the product-engineer surface. Scope the brainstorm against what already exists
  so it doesn't duplicate: the **Design track** item above (`ui-designer` cluster, HIL port)
  and the landed **UX-prototype stage** in specify (`authoring-prototype`, UX-D1–D9, v0.50.0).
  Decide: does design/UX ride the product-engineer persona as skills, or a dedicated
  `ui-designer` cluster the persona composes? Persona-carries-judgment /
  skill-carries-procedure (five-axis #4) constrains the shape. Resolve overlap with the Design
  track before authoring.
- [ ] **Quality-control pipeline — deepeval or equivalent** (2026-08-04; provenance: capture
  session, to-brainstorm; re-keyed here from To-triage at the 2026-08-06 groom) — explore
  deepeval (or a comparable eval/QC pipeline) for automated quality control over produced
  artifacts / generated code. Open: what does it grade (spec fidelity · code · agent output),
  how it composes with the existing producer↔validator pairing (five-axis #5) and the
  qa-engineer / testing-* skills, and whether it's a kernel-shaped dependency (constraint
  check — no Python/MCP brain code; must live as skill/agent or native CC). Brainstorm the
  fit before adopting any external harness. (2026-08-19: mutation-tool adoption ruled not
  kernel-class — `qa-gap-finding-verification` D10, GI-019 recorded; that adoption is not
  this item's eval-harness brainstorm; item stays open.) (2026-08-22: `skill-compression-tooling`
  accepted — rules the lowest-level per-skill eval primitive and fills `primitive-eval-harness`
  D5's pilot slot; this item still owns the wider produced-artifact QC question.)
- [ ] **Multi-stack / monorepo registries** (2026-07-21) — per-stack registries, paths scopes,
  seeding. **Revisit: first multi-stack setup run.**
- [ ] **`codebase-inventory-schema.json` port** (2026-06-27; provenance:
  `.mochiko/transform/setup/assess-codebase-inventory-schema.md` — the full draft-07 shape
  survives in git history only, `.mochiko/transform/` deleted 2026-08-04; recover via
  `git log --all -- .mochiko/transform/setup/assess-codebase-inventory-schema.md`; HIL
  submodule removed 2026-07-21) — the collision/spec-plan-mode contract for
  `analysis-codebase`; port with the spec/plan cluster and wire its consumer then.
- [ ] **Phase-A0 codebase-discovery reclaim** (2026-07-01) — `review-plan-artifacts`' parked
  discovery review (documented in its ARTIFACT-CHECKLISTS); the brownfield/discovery track must
  reclaim it.
- [ ] **`/mochiko:graduate` wrapper** (2026-07-02) — thin per-slice sequencer over existing
  commands; build only on demonstrated shepherding pain, never a re-implementation of their
  loops.
- [ ] **Slice spec-amend mode** (2026-07-02, recorded deferral) — re-place changed/new stories
  without touching shipped slices, escalate when a shipped slice's stories changed; design when
  a real mid-flight amendment hits.
- [ ] **Context handoff document** (idea) — cross-session serialization; a deliberate add-on
  if pursued (setup/specify dissolved the HIL carrier into the lead).
- [ ] **Deliberate shortcut ledger** (idea; defect-backed 2026-08-01; defect half discharged
  2026-08-04 groom) — deferral tracking with upgrade triggers; the parallelism entry above is
  its first live candidate. The dangling `implement.md:102` reference died with the v8 rebuild
  (v0.48.0 — no shipped file names the ledger now); remaining: ≥5 ledger-shaped entries sit
  scattered across records — decide whether a carrier is worth building.
