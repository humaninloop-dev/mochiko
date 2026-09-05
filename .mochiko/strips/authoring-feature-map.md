# Strip notes — `skills/authoring-feature-map/SKILL.md`

Entry formats: `strips/README.md`. Wave context: the feature-sizing & entry-points build wave
(record: `.mochiko/brainstorms/feature-sizing-and-entry-points/record.md`; `DECISIONS.md` row
2026-08-10 "Feature sizing & entry points ruled (D1–D15 as amended at review)"). The skill gains
nesting (parent/leaf, two-level cap, sticky-delivered roll-up), parent minting three ways,
`unrefined` capability stubs, and the lane-run vocabulary; the entries below record every line
the same wave superseded.

---

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the authoring family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/authoring-feature-map/SKILL.md`. -->

<!-- Wave context: wave 6 of the CLI schema-delivery build (v0.107.0) — the end state. No schema
file ships in the plugin: the 20 files under `plugins/mochiko/schemas/` and the 30
`skills/*/schema.yaml` were deleted, and every delivery they served now has a CLI form. Ruling for
the [v0.107.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D9 wave 6, with
the `DECISIONS.md` 2026-09-05 row and that session's `wave6-plan.md`. Pre-edit verbatim text:
`git show 62aa99d:plugins/mochiko/skills/authoring-feature-map/SKILL.md`. -->

## [v0.107.0] the Related row's "in the schema's artifact section"

- **Disposition:** superseded → "delivered by `mochiko-cli` in the artifact section"
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/cli-schema-delivery/record.md`
  D9 wave 6; `DECISIONS.md` 2026-09-05)
- **Content:** "(bindings in the schema's artifact section)"
- **Kept deliberately:** the row itself — that the `features-index` and `feature-entry` schemas own
  the repo-root `FEATURES.md` index shape and the per-capability entry shape this skill fills — and
  the bindings' location in the artifact section. Only the delivery noun moved.

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action at invoke, before any derivation or map write: **Read `schema.yaml`
  (this skill's own directory) and `../../schemas/skill-authoring-common.yaml` raw, in
  full, in the same first action.** The schema is the source of truth for this skill's
  binding rules; this body carries identity and teaching only. Its rules are nested in six
  sections, each addressable by its section ID: `authoring-feature-map.sec.independence`
  (who grades the produced artifacts) · `authoring-feature-map.sec.scope` (jurisdiction and
  routing) · `authoring-feature-map.sec.inputs` (read duties before deriving) ·
  `authoring-feature-map.sec.artifact` (the map's binding grammar, invariants, and write
  mechanics) · `authoring-feature-map.sec.output` (the acceptance batch and the selection
  card) · `authoring-feature-map.sec.reserved` (decisions reserved to the user).

  Read the rule grammar along with the rules: a rule's `kind:` names what it is, and an
  absent `kind:` reads `constraint`. Where a rule carries `extends: authoring-common.<slug>`,
  the stub inherits `text` / `labels` / `pointer` only from `skill-authoring-common.yaml` —
  `class` and `kind` are always this schema's own, and the stub's `authoring-feature-map.*`
  ID stays the citable ID. `${var}` placeholders substitute from this schema's `vars:` at
  read time. Labels come from `../../schemas/skill-labels.yaml`. A `pointer:` rule binds you
  to that file's or skill's content, referenced never restated.

  The schema carries **the 16 rules of `class: floor`**. State the floor count back before
  the first procedural step; a skipped or partial schema read is a halt-and-surface, never a
  silent continue.
  ```

- **Kept deliberately:** everything outside this section, byte for byte — the title, the opening
  paragraph, and every procedural section. The block's substance survives in the render: the six
  section IDs are now the six `--section` arguments, and the interpretation grammar the block
  taught is printed as the preamble's `legend` with every fire. The per-section glosses this
  block carried are covered by the render, whose `sections` line prints a title per section and
  whose empty sections carry a `note:` giving the reason. The "this body carries identity and
  teaching only" clause is dropped: it stated a scope rather than an obligation, and the new
  halt paragraph states the same split. The `extends:` stub resolution and the family
  common-file co-Read are discharged by the render, which resolves every stub before the model
  sees it.
- **Consumers assessed:** the family common file
  `plugins/mochiko/schemas/skill-authoring-common.yaml` is unchanged and still bound by every
  unconverted consumer; nothing shared leaves. The block was this skill's own text.

## [v0.106.0] the hand-pinned `class: floor` count

- **Disposition:** superseded → the CLI-printed pin. The count is the `- class: floor · N rules`
  line under `pins` in the preamble block, and the `floors:` line beneath it lists the ids; the
  read-back sentence now cites both rather than carrying a number of its own.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3, "the counts are computed
  and printed by the CLI, never hand-pinned"; the wave-4 re-key ruling adding the `floors:` index)
- **Content:** verbatim —

  ```
  The schema carries **the 16 rules of `class: floor`**. State the floor count back before
  the first procedural step; a skipped or partial schema read is a halt-and-surface, never a
  silent continue.
  ```

- **Kept deliberately:** the read-back obligation itself (skill-content-schema D6 as amended),
  re-keyed from a hand-pinned number to the printed pin and its id list — the skill still states
  the floor set back before its first procedural step, and a blank or partial read-back still
  halts. The independent second number the `.md` used to hold is booked as a loss, per D3's own
  rationale; the contract suite's `assert_floor_delivery` now carries what it used to check.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` skill-pair criterion 3, which
  demanded the hand-pinned count — amended in this same wave to branch on a converted skill.

## [v0.101.0] Schema conversion — census-row → minted-ID map (skill-content-schema wave 2A, authoring family)

Ruling for every [v0.101.0] entry below: skill-content-schema D3 (three-home boundary) /
D8/C4 (protected transfers), `DECISIONS.md` 2026-09-01 rows (record + wave-2 family-door
rulings); census: `.mochiko/brainstorms/skill-content-schema/census-authoring.md` §A/§B (AFM).
Schema home: `plugins/mochiko/skills/authoring-feature-map/schema.yaml`. Minted IDs carry the
`authoring-feature-map.` prefix (omitted below). Map — census row → minted ID:
1 `letter-is-spirit` (C-A1 stub) · 2a `capability-work-row` · 2b `work-rows-transient` ·
2c `stories-inform-never-define` · 3 + 9b **merged** → `discipline-single-home` (recorded
deviation: the single-home binding and the When-NOT routing limb are one obligation at D12
grain — near-identical within one member) · 4 `frame-first` · 5a `one-living-map` ·
5b `features-index-two-arm` (C-A3 stub, `${template}` = features-index) **+**
`feature-entry-two-arm` (LOCAL twin — lead-ruled: schema-level `vars:` binds one value per
name, so the twice-bound block splits stub + local twin; keep-distinct allowlist edge queued) ·
6 `four-touchpoints` · 7 `map-side-altitude` · 8 → `envelope-binding` (C-A4 stub) **+**
`map-density` (residue split per the wave lead's residue rule — the AFM density tail mints an
adjacent LOCAL rule, never stretched inherited text) · 9a `independent-grade` (C-A2 stub) ·
9c `stories-routing` · 9d `architecture-routing` · 9e `selection-user-ruling` ·
9f `backlog-boundary` · 10 `one-home` · 11 `complete-disposition` · 12 `row-level-closure` ·
13 `map-owns-status` · 14a `delivered-sticky-rows-fold` · 14b `retired-terminal` ·
15a `capability-writes-sacred` · 15b `acceptance-batch` · 16 `integrity-fix-on-sight` ·
17 `entries-index-never-rewrite` · 18 `growth-rides-work-row` · 19 `in-flight-territory-read` ·
20 `pre-acceptance-workspace` · 21 `phase-row-stands-alone` · 22a `stubs-are-hypotheses` ·
22b `selectability-specify-only` · 23a `stub-dependency-unverified` ·
23b `escalation-never-forced-cut` · 24 `reconstructed-reverify` · 25 `sc-mapping` ·
26 `acceptance-batch-specs-index` · 27 `selection-card` · 28 `story-trace-provenance`.
**Rule count 41** (16 floor · 25 must): census §B table grain sums to 40, not the headline 39
(census arithmetic, the wave-1 VC idiom — landing annotation owed); −1 for the 3+9b merge,
+1 for the 5b twin, +1 for the 8a/8b residue split.
**Conditions divergence (ruled, not a miss):** the census listed KM-exists and epic-context
dims; the pair ships **no `conditions:` block** — 9f keeps both arms in text (the non-KM
degrade path; the wave-1 RPA row-30 both-arms-in-text precedent), and the epic-marker clauses
ride `retired-terminal` / `sc-mapping` text. Lead-confirmed at plan approval (Q5).
**Body relocation inventory:** the epigraph line → the C-A1 stub; Overview obligation
sentences → rules 2a/2b/2c/4/5a/5b/6/7/8; `## When NOT to Use` → the scope/independence/
reserved rules; `## The invariants (hard rules)` (all 8) → the floor set; `## Red Flags` rows →
rule sources (4 · 8 · 9e · 14a · 18 · 19 · 20 · 21 · 22a · 22b · 23a · 23b · 24) or
mirror-dedups (story-mirror ban → `discipline-single-home` + 2a · parent-grouping → the
discipline's domains · widen-silently → 18); `## Quality checklist` rows → rule sources
(4 · 11 · 12 · 19 · 22a · 24 · 25 · 26 · 27 · 21 · 28 · 15b) or mirror-dedups; `## Related`
two-arm mechanics → the 5b pair. The Overview teaching, Vocabulary table (incl. the protected
capability-batch line, body-side, untouched), and Common Rationalizations stay prose per the
census's teaching dispositions.
**Accounting:** body 15,975 → 5,505 (−10,470) + schema 16,131 = **payload 21,636** (the C1
delivered-at-invoke quantity; census est. ~36,100) — ×1.35 vs the pre-conversion body, the
delta structural overhead (IDs, keys, section scaffolding, reading grammar), no content growth
claimed.
**J-6 — the standing +562 ruled overage dissolves at re-seed:** the +562 declared overage
(mechanic-(e) zero-gap branch, ruled **HOLDS** by the v0.91.0 wave audit V2, byte-reconciled to
the ruled obligation) dissolves into the C1 conversion re-seed of the budget row. Named here so
its ruling trail survives the budget row's supersession (GI-006 reconstruction); the zero-gap
clause itself transfers verbatim-in-meaning to `four-touchpoints` (entry below).

## [v0.101.0] The 8 hard invariants — protection transfers (pm-role D2/D6/D7/D8 + feature-sizing G4)

- **Disposition:** superseded — protection transfers to the schema floor set: `one-home` ·
  `complete-disposition` · `row-level-closure` · `map-owns-status` ·
  `delivered-sticky-rows-fold` + `retired-terminal` (invariant 5 split a/b) ·
  `capability-writes-sacred` + `acceptance-batch` (invariant 6 split a/b) ·
  `integrity-fix-on-sight` · `entries-index-never-rewrite`, per D8/C4; the provenance sidecar
  carries the protected status on each ID.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 skill-content-schema
  rows; original protections: 2026-08-13 pm-role-and-feature-derivation D2/D6/D7/D8 ·
  2026-08-10 feature-sizing-and-entry-points G4 · 2026-08-14 multi-feature-plan-implement for
  the `[EPIC-XXX]` marker clause).
- **Content:** the eight `## The invariants (hard rules)` entries, verbatim-in-meaning in their
  rules (one-home · complete disposition · row-level closure · map-owns-status · sticky/fold +
  retired-terminal · sacred/acceptance-batch/stewardship-direct · integrity-fix-on-sight ·
  index-never-rewrite).
- **Kept deliberately:** the Vocabulary table and Common Rationalizations stay body-side
  teaching; no invariant clause is weakened or dropped.
- **Consumers assessed:** the five consuming commands dispatch the skill by name and rely on
  the invariants, now schema-carried and ID-citable; `mochiko:review-specifications` grades map
  integrity by reference — contracts intact.

## [v0.101.0] Re-type core + frame-first + story-wins — protection transfers (pm-role D1/D2/D5)

- **Disposition:** superseded — protection transfers to `capability-work-row` ·
  `work-rows-transient` · `stories-inform-never-define` · `frame-first` (which carries the
  story-wins clause), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows; original
  protection: 2026-08-13 pm-role-and-feature-derivation D1/D2/D5).
- **Content:** the Overview's two-row-type sentences, "Stories inform *which* capabilities
  exist and sharpen their extents; they never define them", and the frame-first paragraph incl.
  "where a story and the frame conflict, the **story wins** and the frame adjusts".
- **Consumers assessed:** the product-manager agent applies the derivation behavior through
  this skill (decoupling holds); no persona restatement found at the v0.68.0 wave, unchanged
  since.

## [v0.101.0] Four touchpoints incl. the zero-gap branch — protection transfers (plan-stage mechanic (e))

- **Disposition:** superseded — protection transfers to `four-touchpoints` (class: must), per
  D8/C4; the zero-gap branch ("on a zero-gap verdict, where no design phase runs, the
  card-authoring seat performs the dependency and extent assertion instead") carried
  verbatim-in-meaning. The +562 HOLDS overage this clause anchored dissolves at the re-seed —
  see the J-6 clause in the map entry above.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows; original
  protection: 2026-08-26 plan-stage-utility D1 mechanic (e), V2 B1 wording).
- **Content:** the Overview four-touchpoints sentence, all four duties and the zero-gap branch.
- **Consumers assessed:** `mochiko:patterns-vertical-tdd` carries the card-authoring seat the
  branch names; `implement.md` owns the design phase — both untouched by this relocation.

## [v0.101.0] Two-arm features-index / feature-entry bindings — protection transfers (template-schema D8, GI-020)

- **Disposition:** superseded — protection transfers to `features-index-two-arm`
  (`extends: authoring-common.two-arm-template`, `${template}` = features-index) and
  `feature-entry-two-arm` (LOCAL twin), per D8/C4. Both arms preserved at both sites — CLI
  invoke + first-class raw Read (GI-020).
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows; original
  protection: 2026-08-16 schema-based-template-guidance D1/D8, the v0.76.0 entry below).
- **Content:** the Overview two-arm parenthetical and the two `## Related` two-arm lines.
- **Kept deliberately:** the Related list keeps both schema names as navigation pointers,
  mechanics now schema-carried.
- **Consumers assessed:** n/a (single-writer skill; the schemas named are P-side data files,
  untouched).

## [v0.101.0] Stub discipline, BACKLOG boundary, and reservations — protection transfers (feature-sizing + pm-requirements-stacking)

- **Disposition:** superseded — protection transfers to `stubs-are-hypotheses` ·
  `selectability-specify-only` (floor) · `stub-dependency-unverified` ·
  `escalation-never-forced-cut` (floor, reservation) · `backlog-boundary` ·
  `selection-user-ruling` (floor, reservation), per D8/C4.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-09-01 rows; original
  protections: 2026-08-10 feature-sizing-and-entry-points D2a/D12/D13 · 2026-08-10
  pm-requirements-stacking D2/D2a/D4 + the v0.62.0 entry below).
- **Content:** the red-flag and checklist rows carrying the stub/selectability/escalation
  discipline and the When-NOT BACKLOG boundary line (KM degrade path and extent-growth
  exception intact in `backlog-boundary`).
- **Consumers assessed:** `/mochiko:feature` remains the stewardship stub-minter;
  `mochiko:review-specifications` grades derivation output — both bind by reference, contracts
  intact.

## [v0.91.0] Touchpoint line, baseline-surface pointer, and capability-batch line re-keyed — plan-stage retirement D1 (e) — V2 B1

- **Disposition:** superseded → the design phase confirms and hardens (with the zero-gap branch
  naming the card-authoring seat); "implement's surface"; "one implement run".
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1 mechanic **(e)**, the map-entry
  hardening mechanic: "the design phase, when it runs, asserts the design-implied dependency
  relations and sharpened extent onto the feature's map entry with provenance … on the zero-gap
  path the card-authoring seat performs the dependency/extent assertion at card authoring").
  Raised by the v0.91.0 wave audit as **V2 B1**; wording ruled by the wave lead 2026-08-26.
- **Content (superseded fragments, verbatim — three sites):**

  1. Four-touchpoints line: `**plan confirms and hardens** alongside architecture`
  2. Baseline-altitude line: `(the appliable before/after form) are a different altitude — plan and implement's surface, untouched here.`
  3. Capability-batch line: `A **capability-batch** is the pipeline unit: each capability with selected work rows gets one plan/implement run covering exactly those rows.`

- **Kept deliberately:** all four touchpoints as four (specify proposes · the design phase
  confirms and hardens · implement's acceptance landing folds · `/mochiko:feature` stewards) —
  the touchpoint *count* and each one's duty are unchanged; the frame-first derivation rule and
  the story-wins conflict rule; the one-living-map / no-per-spec-copy invariant; the
  bookkeeping-inside-the-landing-never-a-separate-close-stage clause; and the capability-batch
  definition itself (a capability plus its selected rows, one run, exactly those rows).
- **Budget — OVERAGE DECLARED, widened.** Body **15,975 against the 15,413 budget (+562)** —
  **+128 this wave**, on top of the +434 standing since v0.81.0. Description unchanged at 598
  against 619. The +128 is mechanic (e)'s **zero-gap branch** and nothing else: without it the
  zero-gap path leaves the map-entry hardening duty unowned, which is precisely the defect the
  record's verify round 1 caught as V1 and repaired — so dropping the clause to save chars would
  re-open a closed defect. Two same-line simplifications partly offset it (−12 and −9). No prose
  added or restored. Recorded in `.mochiko/memory/primitive-cost-budgets.md` for the audit, and
  **ruled HOLDS by the v0.91.0 wave audit (V2)** — byte-reconciled to the ruled obligation
  exactly.
- **Consumers assessed:** `mochiko:patterns-vertical-tdd` carries the card-authoring seat this
  entry's zero-gap branch names (re-keyed earlier this wave); `implement.md` (P1's rewrite) owns
  the design phase and the card-confirm checkpoint; the router's `authoring-feature-map` row
  describes the map's own grammar and never named the plan stage — no re-key owed there.

## [v0.81.0] Three architecture pointers re-keyed to the store — product-architecture-schema D3/D4/D7

- **Disposition:** superseded → the architecture store and its spine. The map's peer view is no
  longer a prose `ARCHITECTURE.md` but the store (`.mochiko/product/architecture/`), of which the
  root doc is a derived index; the retired `authoring-architecture` skill is replaced by
  `authoring-architecture-store` in the Related list.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D3/D4 (one store, derived index) ·
  D7 (crew — `authoring-architecture` retired); `DECISIONS.md` 2026-08-19).
- **Content (superseded, verbatim — three fragments):**

  ```
  the primary capability lens on the product, the way `ARCHITECTURE.md` is the system viewed as components; together the two are the central source of truth.
  ```

  ```
  - **Authoring architecture** — the entry links to `ARCHITECTURE.md` components; it never restates the component view
  ```

  ```
  - `mochiko:authoring-architecture` — the peer view: components that realize capabilities; the entry's architecture link points there
  ```
- **Kept deliberately:** the peer-view relationship itself — map and architecture as the two
  central sources of truth, the entry linking out and never restating the component view. Only
  the peer's name and home moved.
- **Budget note (declared for the audit):** body 15,846 → 15,855 (**+9**) against the 15,413
  budget, widening the standing, already-ruled overage from +433 to +442. The delta is
  arithmetically forced by the longer skill slug (`authoring-architecture` →
  `authoring-architecture-store`, +6) plus the re-keyed peer-view phrasing (+3); **no prose was
  added or restored**, and two of the three fragments shrank. No offsetting trim was taken —
  cutting unrelated protected content to fund a rename would be the worse move.
- **Consumers assessed:** shared-vocabulary file. `mochiko:patterns-map-minimalism` carries the
  matching domain-mapping pointer, re-keyed in the same edit set by this seat;
  `mochiko:review-specifications` grades the map delta but names no architecture surface (grep
  clean); the `feature-entry` schema's component pointers are P1's in this wave.

## [v0.80.0] Vocabulary-table term "Vertical slice (cycle)" → "Cycle" — slice-vocabulary purge

- **Disposition:** superseded → the same table row keyed on the unit's live name.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/decisions/2026-08-19-slice-vocabulary-purge.md`).
- **Content (verbatim, the superseded cell):**

  ```
  | **Vertical slice (cycle)** | Implementation, within one capability-batch run |
  ```

  Replaced by:

  ```
  | **Cycle** | Implementation, within one capability-batch run |
  ```

- **Kept deliberately:** the row's three other cells verbatim — the level ("Implementation,
  within one capability-batch run"), the definition ("A test-first increment delivering one
  observable behavior"), and the owner pointer (`mochiko:patterns-vertical-tdd` (downstream)).
  The row exists to mark the boundary between the map's units and the implementation unit
  below them, and that job is unchanged; only the term is corrected. The three sibling rows
  (Capability, Work row, User story) are untouched.
- **Consumers assessed:** vocabulary table, read by the map author. The owner it points at,
  `patterns-vertical-tdd`, was re-worded off the slice unit noun in the same wave by the seat
  that owns it, so the pointer stays accurate. No other surface cites this term.
- **Char budget:** body 15,863 → 15,846 (−17). The pre-existing +450 overage against the
  15,413 budget — declared and ruled HOLDS at the v0.76.0 V3 audit — narrows to +433. This edit
  shrinks the body; it adds no obligation and needs no fresh overage justification.

## [v0.76.0] `features-index-template.md` / `feature-entry-template.md` read-pointers → schemas (two-arm) — schema-based-template-guidance D1/D8
- **Disposition:** superseded → `mochiko-cli template features-index` / `template feature-entry`, or Read `plugins/mochiko/schemas/features-index.yaml` / `plugins/mochiko/schemas/feature-entry.yaml` raw (D8-first-class). Three sites (four pointer instances): the Overview one-living-map sentence (both templates) + the two Related-section pointers.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/schema-based-template-guidance/record.md` D1/D3/D8; `DECISIONS.md` "Template-schema CLI ruled").
- **Content (superseded, verbatim):**
  - `a succinct repo-root [\`FEATURES.md\`](../../templates/features-index-template.md) index (one line per capability, work rows as sublines) over per-capability entry files (\`.mochiko/features/FEAT-XXX-<slug>.md\`, shaped by [\`feature-entry-template.md\`](../../templates/feature-entry-template.md)).`
  - `- [\`features-index-template.md\`](../../templates/features-index-template.md) — owns the repo-root \`FEATURES.md\` index shape`
  - `- [\`feature-entry-template.md\`](../../templates/feature-entry-template.md) — owns the per-capability entry shape this skill fills`
- **Kept deliberately:** the `artifact-format.md` pointer (line 16) — not in-scope, stays `.md`; all descriptive text. **Body-budget note:** the two-arm re-point adds ~202 chars over the pre-existing v0.72.0 overage (15,661 → 15,863 vs the 15,413 budget). The overage is D1/D8-mandated — the schema two-arm (CLI invoke + first-class raw-Read fallback) is structurally longer than the deleted markdown link, and no mandatory arm is cuttable — and is declared to the author≠grader audit for a HOLDS ruling.
- **Consumers assessed:** n/a (single-writer skill).

## [v0.68.0] Re-type: parent/leaf feature model → durable capabilities + transient work rows (wave context)

Wave context: the PM-role & feature-derivation build wave (record:
`.mochiko/brainstorms/pm-role-and-feature-derivation/record.md`; `DECISIONS.md` row 2026-08-13
"PM role & feature derivation ruled (D1–D12 as amended at review)"). The map re-types — "feature"
now reserved for durable **capabilities**; the old "leaf" becomes a transient **work row** that
folds into its capability's extent at delivery and vanishes; parent/leaf nesting dies (re-typing +
transience, per the record's D6 exhaustive per-clause inventory). The v0.68.0 entries below record
every clause that ruling superseded in this skill. Pure `feature`→`capability` / `leaf`→`work row`
vocabulary swaps that preserve a rule verbatim-in-meaning ride the decision row and are not
separately stripped; every clause whose meaning, structure, or membership changed is entered below.

Several superseded clauses were themselves feature-sizing & entry-points survivors (`DECISIONS.md`
2026-08-10; the v0.61.0/v0.63.0 entries further down) marked "Kept deliberately" — the nesting,
parent-roll-up, and sizing-bar rulings. Superseding them now is the recorded ruling D6 requires;
each entry names the prior-survivor lineage so the preserved-responsibilities audit reads a ruling,
not a silent drop.

## [v0.68.0] Overview reframed — feature-is-built-thing + four-touchpoint "graduates" superseded
- **Disposition:** superseded → the capability/work-row Overview (two-row-type model; frame-first derivation; fold-at-landing; the `mochiko:patterns-map-minimalism` discipline reference; the map-side-vs-product-baseline altitude line)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-13 pm-role-and-feature-derivation; record D1 published-surface reframe, D2 two-row-type model, D5 frame-first, D6 re-type inventory)
- **Content (verbatim — the superseded Overview clauses):**
  - "A **feature is the built thing**: a capability of the system described in the system's own language — not a cluster or regrouping of user stories. Stories inform *which* features get built and sharpen their extents; they never define them."
  - "The map has four touchpoints: **specify proposes** (this skill's core work), **plan confirms and hardens** alongside architecture, **implement's acceptance landing graduates** — status flips, delta folds, and index touches are bookkeeping edits inside that landing, never a separate close stage — and **`/mochiko:feature` stewards**: stub minting, retroactive promotion, retire, integrity grooming, plus lane intake for small feature-keyed work."
  - "This skill is the map judgment plus the entry authoring — the map-read agenda at intent, the derivation and filter after stories, entry and delta authoring, and the write rules."
- **Kept deliberately:** the `FEATURES.md`-index / one-living-map / per-entry-file framing; the peer-of-`ARCHITECTURE.md` framing; independent-delivery-axes; the density envelope — all survive re-worded. `plan confirms and hardens` and the never-a-separate-close-stage clause survive; `retroactive promotion` is dropped from stewardship (superseded — the D8 growth door cuts work rows in its place).
- **Consumers assessed (shared skill — 5 consuming commands):** setup/specify/plan/implement/feature dispatch by name and rely on the invariants + derivation behavior, not on Overview anchors; the re-type lands across their seats in the same wave.

## [v0.68.0] Vocabulary table parent/leaf rows superseded → Capability + Work row
- **Disposition:** superseded → two rows — **Capability** (durable, the only "feature") and **Work row** (transient increment; `pending`/`live`; folds into extent at landing) — plus the capability-batch pipeline-unit note replacing "leaf = pipeline unit"
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-13 pm-role-and-feature-derivation; record D2, D6, D7 capability-batch)
- **Content (verbatim — superseded rows):**
  - "| **Feature — parent** | Product | The capability a product person names in one breath; navigation + status roll-up over its leaves, never built directly | **this skill** (map entry) |"
  - "| **Feature — leaf** | Pipeline unit | A deliverable built capability; graduates through plan/implement as its own unit (a flat entry is a leaf) | **this skill** (map entry) |"
  - vertical-slice row cell "Implementation, within one leaf" (re-keyed → "within one capability-batch run")
- **Protected-content reconciliation:** these two rows were the "Kept deliberately" survivors of the v0.63.0 guardrails cut and trace to feature-sizing D2 (nested entries) / D3 (two-level cap) — superseded now by pm-role D6 (nesting dies as re-typing + transience). Not a silent drop.
- **Kept deliberately:** the user-story row unchanged; the vertical-slice row survives, re-keyed leaf→capability-batch run.
- **Consumers assessed:** `mochiko:patterns-vertical-tdd` (Related line re-keyed same edit — cuts one capability-batch run now) · plan/implement re-key to capability-batch in parallel seats.

## [v0.68.0] Invariant 5 sticky-delivered parent-roll-up clause superseded → capability stickiness + fold
- **Disposition:** superseded → "**Delivered is sticky; delivered rows fold.**" — a `live` row folds into extent at landing and vanishes; a `pending` row persists as open obligation; a `delivered` capability keeps status while carrying live rows, the change riding on the row until its fold; status never regresses
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-13 pm-role-and-feature-derivation; record D2 fold + pending/live, D6 — parent-roll-up superseded by the fold)
- **Content (verbatim):** "5. **Delivered is sticky.** A later spec or lane run touching a `delivered` feature never regresses its status; the change rides as a marked delta until that work's landing folds it. Roll-up yields to stickiness: a delivered parent gaining an in-flight child keeps `delivered`, the child riding as a delta. `retired` is terminal: entry kept, dated, provenance intact — never deleted."
- **Protected-content reconciliation:** the parent-roll-up sentence ("a delivered parent gaining an in-flight child keeps `delivered`") was a feature-sizing D2 survivor (v0.61.0 delta-grammar entry, "Kept deliberately"); superseded now — the fold replaces roll-up (D6). The delta-carry re-types onto the work row.
- **Kept deliberately:** no-status-regression survives (now capability-level); `retired`-terminal survives verbatim; sticky-delivered itself survives as the capability + live-rows rule.
- **Consumers assessed:** review-specifications carries map-integrity by reference; implement's landing seat executes the fold (parallel seat, same wave).

## [v0.68.0] Invariant 6 write-timing re-keyed + capability-write sacredness added; retroactive promotion dropped
- **Disposition:** superseded → "**Capability writes are sacred; delivery writes land at acceptance; stewardship writes are direct.**" — capability mint/merge/retire/status is the sacred layer (specify or a user grooming ruling only); cutting work rows is desk bookkeeping; proposed entries + cut rows stage in the workspace, the delivery-status write (in-flight flips, row folds) batches at the acceptance landing; stewardship writes (stub minting, retire, grooming) land directly
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-13 pm-role-and-feature-derivation; record D8 capability-write test, D6 — retroactive promotion superseded by the growth door)
- **Content (verbatim):** "6. **Delivery writes land at acceptance; stewardship writes are direct.** During a run, proposed entries and deltas live in the spec workspace; the map write is one atomic batch at spec acceptance, and a rejected spec never touched the map. `/mochiko:feature` stewardship writes — stub minting, retroactive promotion, retire, grooming fixes — land directly, outside spec acceptance. Delivery-status writes (in-flight flips, graduations, delta folds) land only at acceptance landings. Reads happen any time."
- **Protected-content reconciliation:** this invariant was itself the v0.61.0 split-write ruling (feature-sizing G4, "Kept deliberately"); its clauses survive re-worded, with two ruled changes — `retroactive promotion` removed from the stewardship list (D6/D8: the growth door cutting work rows replaces it) and `graduations, delta folds` → `row folds`. Capability-write sacredness (D8) is the new obligation added.
- **Kept deliberately:** workspace staging, the atomic acceptance batch, rejected-spec-never-touched, reads-any-time, stewardship-direct — all survive; checklist "stewardship writes per invariant 6 excepted" still holds.
- **Consumers assessed:** `/mochiko:feature` (the sacred-layer boundary re-keys to the capability-write test in its own charter rewrite, parallel seat) · review-specifications grades delivery writes against the workspace baseline.

## [v0.68.0] Invariant 7 integrity re-keyed — parent-roll-up-defect + orphaned-delta clauses superseded
- **Disposition:** superseded → the work-row integrity form: every work row names its run; every `live` row / `in-flight` status points at an open run; a row whose run ended without folding is a defect; closed-spec-pointed-at and specs-index-contradiction defects survive
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-13 pm-role-and-feature-derivation; record D6 — the fold removes parent roll-up; delta re-types to work row)
- **Content (verbatim):** "7. **Map integrity — fix on sight.** No dangling FEAT-IDs; index lines and entry files agree on status; no orphaned deltas; every delta names its spec or lane run; every `in-flight` status or delta points at an open spec or a live lane run — live from dispatch until its acceptance landing; a delta whose lane run ended without folding is a defect, fix-on-sight. A closed spec still pointed at is a defect; a parent whose status contradicts its children's roll-up is a defect; a specs-index row contradicting the map is a defect."
- **Protected-content reconciliation:** the "parent whose status contradicts its children's roll-up is a defect" clause was a feature-sizing D2 survivor (v0.61.0 invariant-7 entry, "Kept deliberately") — superseded now (no parents; the fold replaces roll-up). The delta-integrity clauses re-type to work rows verbatim-in-meaning.
- **Kept deliberately:** dangling-FEAT-ID, index/entry agreement, closed-spec-defect, specs-index-contradiction — all survive; the delta-names-its-run and live-until-landing rules survive re-typed onto work rows.
- **Consumers assessed:** review-specifications (pipeline-core map-integrity invariants by reference) · the lane run's verification seat (parallel seat).

## [v0.68.0] Invariants 3–4 re-keyed to the work-row unit
- **Disposition:** superseded → dependency closure runs at the **row** level (a row depending on another capability's row orders the two capability-batch runs); the status invariant scopes to **capability** status + **work-row state** (`pending`/`live`)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-13 pm-role-and-feature-derivation; record D7 + M12 row-level ordering, D2 pending/live states)
- **Content (verbatim):**
  - "3. **Dependency closure.** A selected feature must be buildable given only the features ordered before it, per the map's relations. No forward dependencies."
  - "4. **The map owns status.** `proposed / in-flight / delivered / retired` — one home, no copies. Story files derive status by following their FEAT-ID; the only story-native status is `rejected`."
- **Kept deliberately:** no-forward-dependencies, one-home-no-copies, story-derives-status-via-FEAT-ID, story-native-`rejected` — all verbatim; the closure and status rules are extended to the row unit, not weakened.
- **Consumers assessed:** review-specifications carries closure + status invariants by reference; plan/implement honor row-level ordering in parallel seats.

## [v0.68.0] Red-flags + checklist nesting/sizing/per-parent cluster superseded (section-level inventory)
- **Disposition:** superseded → the capability/work-row red flags and checklist (flat capabilities · domains-at-cap only · oversize = two capabilities or work rows · frame-as-hypothesis · fold-and-vanish · per-capability completeness · row-level closure · work-row sublines)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-13 pm-role-and-feature-derivation; record D2/D4/D5/D6/D7)
- **Content (verbatim — superseded lines):**
  - Red flag: `"This capability wants a third level" — it doesn't get one; split the parent into two parents`
  - Red flag: `"The parent should go back to in-flight now that a new child arrived" — delivered is sticky; the child rides as a marked delta and folds when it delivers`
  - Red flag (remedy re-keyed): `"This entry needs eight extent lines to be honest" — then it is not one leaf; split it, or mint a parent`
  - Red flag (re-keyed leaf→row): `"This leaf is just a phase, a thin layer is fine" — a phase-leaf must stand alone as a working increment, never a horizontal layer`
  - Red flag (re-keyed feature→capability): `"Every story maps to its own new feature" — the filter never fired; features are capabilities, not story mirrors`
  - Rationalization cell (re-keyed delta→row): `| "The delta will obviously fold, no need to name the spec" | An unnamed delta is unauditable; a delta whose spec or lane run died is invisible rot. The grammar exists to make both checkable. |`
  - Checklist: "Nesting honest: two levels max; leaves the only pipeline units; every parent's status agrees with its children's roll-up (or carries the sticky-delivered delta); single-leaf parents deliberate"
  - Checklist: "Every entry within the sizing bars — one-breath name at parent/flat, extent ≤ ~3 lines at leaf — or split / parent minted"
  - Checklist: "Index lines agree with entry files on status and name; leaf lines under their parent; no dangling FEAT-IDs introduced"
  - Checklist: "Selection card prepared with recommendation, deferred-SC list, per-parent completeness ledger line (delivered/undelivered leaves · stubs · kills), and ordering — the ruling left to the user"
  - Checklist: "Territory-touching parents' parked stubs and undelivered leaves re-surfaced on the selection card; any dependency-blocked leaf/stub escalated as a recommendation (leaf via asserted map relation, stub via flagged-unverified judgment), never a forced cut"
  - Checklist: "Any leaf cut as an across-round phase is independently useful — a working increment, not a horizontal layer"
- **Protected-content reconciliation:** the nesting red flags, the two-level/sizing-bar checklist lines, and the per-parent completeness-ledger line were feature-sizing D1–D4 survivors ("Kept deliberately" across the v0.61.0/v0.63.0 entries) — superseded now by pm-role D6 (no nesting) / D2 (fold + pending rows) / D7 (capability-batch). The escalation-never-forced-cut, re-verify, phase-stands-alone, and story-mirror-ban rules survive re-keyed, not dropped.
- **Kept deliberately:** the extend-beats-mint mirror ban (re-keyed to capability), the phase-stands-alone rule (re-keyed to work row), escalation-is-recommendation-never-forced-cut, first-touch-re-verify, stub-is-hypothesis, selectability-specify-only, pseudo-capability ban — all survive; per-parent completeness re-homes as per-capability completeness (pending rows · folded rows · deferred SCs).
- **Consumers assessed:** product-manager agent applies this skill's guardrails at derivation — no restatement in its persona (decoupling holds); review-specifications grades the derivation output.

## [v0.68.0] Description frontmatter re-keyed for the capability/work-row model
- **Disposition:** superseded → the rewritten `description:` (names durable capabilities + transient work rows; adds triggers 'capability', 'work row', 'extend beats mint'; adds the `mochiko:patterns-map-minimalism` discipline boundary; measured 598 chars, under the 619 budget and the 1,536 delivery cap)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-13 pm-role-and-feature-derivation; record D2/D4c/D5)
- **Content (verbatim old, 495 chars):** "This skill MUST be invoked when deriving or updating the repo-level feature map — the living FEATURES.md index plus per-feature FEAT-XXX entry files — during a specify run or a /mochiko:feature stewardship touch. SHOULD also invoke on 'feature map', 'FEATURES.md', 'FEAT-XXX', 'propose features', 'feature derivation', or 'map delta'. Boundary: authors and maintains the MAP — NOT user stories (mochiko:authoring-user-stories), NOT architecture views, NOT selection. Never grades its own output."
- **Kept deliberately:** the MUST/SHOULD grading, the FEATURES.md/FEAT-XXX/'feature map'/'feature derivation'/'map delta' triggers ('map delta' retained by ruling — old muscle memory fires, the skill teaches the new vocabulary), the three NOT-boundaries, and the author≠grader closing survive; 'propose features' dropped to hold the budget.
- **Consumers assessed:** description is the model-invocation surface only; no primitive quotes it.

## [v0.63.0] Guardrails cut — procedure/walkthrough sections removed, invariants + red-flags + checklist kept; slim description

- **Disposition:** superseded → benchmark-ruled guardrails body + slim description (`variants/body/authoring-feature-map/`, `variants/descriptions/authoring-feature-map/`), one merged edit.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark verdict; `DECISIONS.md` 2026-08-10 benchmark-verdict row; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `report/final-verdict.md`)
- **Content (faithfully compressed — section-level inventory, body 26,976 → 12,330 chars, −54%):**
  - **Removed whole:** `## When to Use` · `## The intent-stage map-read agenda` (the agenda walkthrough) · `## Derivation and the filter — stories first, features derived` (the derivation + story-filter walkthrough, the largest section) · `## Entry authoring` · `## Ordering and the foundation role` · `## Write rules` · `## Nesting — parent and leaf` (the nesting walkthrough) · `## Capability stubs — parking, never a bypass` (the stub walkthrough).
  - **Shortened:** `## Vocabulary — feature vs the units around it` (1,014 → 787 — the parent/leaf/story/vertical-slice rows kept, prose trimmed).
  - **Kept intact:** `# Authoring the Feature Map` (with the letter/spirit line), `## Overview` (the four-touchpoint sentence), `## When NOT to Use`, `## The invariants (hard rules)` (all 8), `## Red Flags — STOP and re-derive`, `## Common Rationalizations`, `## Quality checklist`, `## Related`.
  - Old description (new slim form is 495 chars; **old verbatim, 1,485 chars**): "This skill MUST be invoked when deriving or updating the repo-level feature map — the living FEATURES.md index plus per-feature entry files at `.mochiko/features/FEAT-XXX-<slug>.md` — during a specify run or a /mochiko:feature stewardship touch: the intent-stage map-read agenda, deriving proposed features from drafted user stories (stories first, features derived), running the story filter, authoring or amending FEAT-XXX entries, nesting parent/leaf entries (leaf = pipeline unit, two-level hard cap, sticky-delivered roll-up), minting parents at derivation or by retroactive promotion, minting `unrefined` capability stubs, attaching marked deltas to delivered features, and staging map writes. SHOULD also invoke on "feature map", "FEATURES.md", "FEAT-XXX", "parent feature", "leaf feature", "promote to parent", "capability stub", "unrefined", "propose features", "feature derivation", "feature selection", "map delta", "in-flight feature", or verifying a reconstructed-from-code entry on first touch. Boundary: this skill authors and maintains the MAP — NOT user stories (mochiko:authoring-user-stories), NOT architecture views (links to ARCHITECTURE.md, never restated), NOT selection: which features build now is the user's ruling — this skill recommends. Capabilities and extent-growth ideas live on the map; defects, tooling, and process items stay in BACKLOG.md. Never grades its own output — graded with the spec by mochiko:review-specifications, an independent reviewer."
  - Verbatim removed text survives in: git history; the before/after pair in `variants/`; archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (guardrails keep-set):** the goal/output contract (Overview + the one-living-map / four-touchpoints framing + Vocabulary), the eight hard-rule invariants, the anti-patterns (`## Red Flags`, `## Common Rationalizations`), the quality checklist, and the template/skill pointers in `## Related`.
- **Protected-content reconciliation (MANDATORY):** the existing strip carries a stack of v0.61.0/v0.62.0 supersession entries (the feature-sizing & entry-points wave, `DECISIONS.md` 2026-08-10) whose "Kept deliberately" survivors were the recently-added nesting, capability-stub, split-write, lane-run, and parent/leaf rulings — several of which lived in the walkthrough sections this cut removes. Reconciled: **the guardrails cut removes the prose *homes* of those rulings but preserves every DECISIONS-traceable behavioral rule, folded into the kept invariants / red-flags / rationalizations / checklist / vocabulary.** No protected rule is silently dropped. Specifically:
  - **Nesting (D2 nested entries, D3 two-level cap, sticky-delivered roll-up)** — the `## Nesting` prose is removed; the rules survive: the Vocabulary parent/leaf rows, invariant 5 (roll-up yields to stickiness; a delivered parent + in-flight child), red flags ("wants a third level — split the parent into two parents"; "parent should go back to in-flight — delivered is sticky"), and checklist "Nesting honest: two levels max; leaves the only pipeline units."
  - **Capability stubs (D2a/D12/D13, the v0.62.0 minting ruling)** — the `## Capability stubs` prose is removed; the rules survive: red flags ("stubs are unratified hypotheses; … let a match be confirmation"; "selectability is specify-derivation-only; `/mochiko:feature` parks and grooms, never matures"), the rationalization ("A stub is name + hook only … a pre-filled stub fakes ratification"), and checklist lines ("stubs noted as hypotheses only", "matching stubs confirmed and filled, not duplicated", "Derivation-minted stubs carry story-trace provenance; no stub matured or made selectable outside specify's derivation").
  - **Split write rule (v0.61.0 Invariant 6, lead ruling G4)** — survives verbatim as invariant 6.
  - **Invariant 7 R5 / lane-run + delta grammar (v0.61.0)** — survives as invariants 5 and 7 plus the rationalization "a delta whose spec or lane run died is invisible rot"; the in-flight-territory read (former agenda item 3) survives in the checklist and the "read the owning spec's artifacts" red flag.
  - **Granularity oversize remedy (v0.61.0)** — survives: the red flag "not one leaf; split it, or mint a parent" and the checklist "one-breath name … extent ≤ ~3 lines … or split / parent minted".
  - **BACKLOG boundary + pseudo-feature ban (v0.61.0 / D13)** — survive: the `## When NOT to Use` BACKLOG line (verbatim) and the rationalization "A pseudo-feature poisons the map permanently."
  - **The delta-grammar literal template string** (`extent grows by <X> — in-flight, <spec-slug or lane-run>`) — the literal string is dropped from the body; its component obligations survive in the checklist ("what grows, in-flight mark, named spec or lane run") and its authoritative shape lives in `feature-entry-template.md` (linked in `## Related`). Not a silent drop.
- **Consumers assessed (shared skill — 5 consuming commands):** `commands/setup.md`, `commands/specify.md`, `commands/plan.md`, `commands/implement.md`, `commands/feature.md`. Each dispatches the skill by name and relies on its invariants and its derivation/delta behavior; none quotes a removed section anchor. The kept invariants, red-flags, and checklist preserve the map-integrity and derivation contract each relies on. Contracts intact.


## [v0.62.0] Capability-stub minting no longer exclusive to /mochiko:feature — derivation may mint stubs
- **Disposition:** superseded → the rewritten "Capability stubs — parking, never a bypass" section: two seats mint `unrefined` stubs (`/mochiko:feature` idea-parking AND specify's derivation parking uncertain remainder with story-trace provenance); selectability/maturation stays specify-derivation-only, `/mochiko:feature` stewards but never matures or dispatches unratified scope; the When-to-Use stub line harmonized in the same touch
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-10 pm-requirements-stacking; record D2 as corrected at review, D2a, F-3 sustained)
- **Content:** "`/mochiko:feature` can add `proposed` entries only as **capability stubs**: name + one-breath hook, marked `unrefined`. Only specify's derivation fills extent and relations and makes an entry selectable for delivery." · When-to-Use line "Minting or grooming `unrefined` capability stubs (`/mochiko:feature` stewardship)"
- **Kept deliberately:** the anti-spec-bypass intent survives in force — the loosening extends *minting* only, never the selectability gate ("selectability stays behind specify's derivation"); "parking, never a spec-bypass", the `unrefined`-mark auditability, and the "unratified hypotheses, never extension anchors" / ignore-stub-text / match-is-confirmation rules survive re-worded
- **Consumers assessed:** `/mochiko:feature` (feature.md) remains the stewardship stub-minter — no exclusivity it relied on; specify.md binds the craft by reference (no command edit needed for minting); its selection card gained the ledger line in the same wave; `mochiko:review-specifications` grades derivation output including derivation-minted stubs

## [v0.61.0] Description frontmatter re-fit for nesting, stubs, and feature-command triggers
- **Disposition:** superseded → the rewritten `description:` (adds nesting/stub/promotion trigger phrases, the `/mochiko:feature` stewardship touchpoint, the D13-scoped boundary line; measured 1,485 chars / 1,495 bytes, under the 1,536 delivery truncation)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-10 "Feature sizing & entry points ruled (D1–D15)"; record D2–D4, D6, D12, D13)
- **Content:** prior description named only the specify-run triggers ("during a specify run: the intent-stage map-read agenda … staging the map write that lands at spec acceptance"), carried no nesting/stub/promotion trigger phrases, and closed the boundary with "Product capabilities live on the map; defects, tooling, and process items stay in BACKLOG.md."
- **Kept deliberately:** the MUST/SHOULD grading, the stories-first framing, the three NOT-boundaries (stories · architecture · selection), and the author≠grader closing sentence survive re-worded.
- **Consumers assessed:** description is the model-invocation surface only; no primitive quotes it.

## [v0.61.0] Three-touchpoints sentence superseded by four (feature command added)
- **Disposition:** superseded → the four-touchpoint sentence: specify proposes · plan confirms · implement's landing graduates · `/mochiko:feature` stewards (stubs, promotion, retire, grooming, lane intake)
- **Tier failed:** n/a — supersession by ruling (record D5/D6 — the feature-management command; lead ruling G4 on stewardship writes)
- **Content:** "The map has three touchpoints: **specify proposes** (this skill's core work), **plan confirms and hardens** alongside architecture, and **implement's acceptance landing graduates** — status flips, delta folds, and index touches are bookkeeping edits inside that landing, never a separate close stage."
- **Kept deliberately:** all three original touchpoints and the never-a-separate-close-stage clause survive verbatim inside the extended sentence.
- **Consumers assessed:** specify/plan/implement re-keyed in parallel seats this wave; the new feature command binds this skill by reference.

## [v0.61.0] Vocabulary table: single Feature row superseded by parent/leaf rows
- **Disposition:** superseded → two rows — **Feature — parent** (capability, roll-up, never built directly) and **Feature — leaf** (deliverable, the pipeline unit; a flat entry is a leaf) — plus the mint-a-parent clause on the oversize sentence below the table
- **Tier failed:** n/a — supersession by ruling (record D2 — nested entries, leaf = pipeline unit; D3 two-level cap)
- **Content:** "| **Feature** | Product / pipeline unit | A built capability on the map; graduates through plan/implement as its own unit | **this skill** (map entry) |" · following sentence "A feature too large to land in one breath of implement is cut into vertical-slice cycles downstream — never into pseudo-features minted for pipeline convenience."
- **Kept deliberately:** the story and vertical-slice rows unchanged; the pseudo-feature ban survives verbatim, re-keyed feature→leaf; vertical-slice cycles remain the downstream cut for an oversize *at implement* — parent minting is the remedy only *at derivation*.
- **Consumers assessed:** `mochiko:patterns-vertical-tdd` (cuts one leaf now — Related line updated in the same edit) · plan/implement re-keyed to leaves in parallel seats.

## [v0.61.0] Invariant 6 "Writes land at acceptance" superseded by the split write rule
- **Disposition:** superseded → "**Delivery writes land at acceptance; stewardship writes are direct.**" — `/mochiko:feature` stewardship writes (stub minting, retroactive promotion, retire, grooming fixes) land directly outside spec acceptance; delivery-status writes (in-flight flips, graduations, delta folds) land only at acceptance landings; Write rules section gains the matching closing sentence
- **Tier failed:** n/a — supersession by ruling (lead ruling G4, citing record D6 stewardship remit + D12 stub minting)
- **Content:** "**Writes land at acceptance.** During a run, proposed entries and deltas live in the spec workspace. The map write is one atomic batch at spec acceptance. Reads happen any time; a rejected spec never touched the map."
- **Kept deliberately:** every original clause survives inside the new wording — workspace staging, the atomic acceptance batch, reads-any-time, rejected-spec-never-touched. The stewardship path is carved out, never a status flip or delta fold. Checklist line "All writes staged in the spec workspace" amended to match ("stewardship writes per invariant 6 excepted").
- **Consumers assessed:** the feature command (same wave) is the only stewardship writer; review-specifications still grades delivery writes against the workspace baseline.

## [v0.61.0] Invariant 7 R5 wording superseded — open spec OR live lane run
- **Disposition:** superseded → "every delta names its spec or lane run; every `in-flight` status or delta points at an open spec or a live lane run — live from dispatch until its acceptance landing; a delta whose lane run ended without folding is a defect, fix-on-sight." — plus the new parent-roll-up defect clause ("a parent whose status contradicts its children's roll-up is a defect")
- **Tier failed:** n/a — supersession by ruling (record D7 invariant amendment, review finding 4; D14 lane runs; D2 parent/child integrity extension)
- **Content:** "every delta names its spec; every `in-flight` status or delta points at an open spec — a closed spec still pointed at is a defect;" · agenda item 3 "**In-flight territory:** an `in-flight` or delta-carrying entry obligates a read into the owning spec's artifacts — its stories, plan, and architecture delta — so this run knows what the feature is *becoming*, not just that it is busy." (re-worded "owning spec's" → "owning work's" — lane runs also own in-flight deltas) · checklist line "Map read completed at intent; in-flight territory read into owning specs; reconstructed entries flagged for re-verify" (re-worded to "read into the owning work's artifacts (spec or lane run)", harmonized with the agenda in the same touch)
- **Kept deliberately:** the closed-spec defect clause survives verbatim; dangling-FEAT-ID, index/entry agreement, orphaned-delta, and specs-index-contradiction clauses untouched; agenda item 3's becoming-not-just-busy clause and artifact list survive verbatim.
- **Consumers assessed:** review-specifications carries the map-integrity invariants by reference (feature-map R7 — pipeline-core); the lane run's verification seat gains the boundary check in the implement cluster (parallel seat).

## [v0.61.0] Granularity guide oversize remedy superseded — split or mint a parent
- **Disposition:** superseded → "An extent that cannot be stated in ~3 lines is not one leaf — split it into two features, or mint a parent whose leaves each pass the bar." — plus the two-bars clause (one-breath polices the parent/flat name; ~3-line extent polices the leaf) and the matching red-flag/checklist rewordings
- **Tier failed:** n/a — supersession by ruling (record D1 — rounding-up happens because compositional structure is missing; D2/D4 — parent minting as the remedy)
- **Content:** "An extent that cannot be stated in ~3 lines is two features." · red flag "'This entry needs eight extent lines to be honest' — then it is two features; split" · checklist line "Every entry within the granularity guide — one-breath capability, extent ≤ ~3 lines — or split"
- **Kept deliberately:** the one-breath definition ("bigger than a story, smaller than a product area") verbatim; split remains a valid remedy alongside minting.
- **Consumers assessed:** product-manager agent applies this skill's guide at derivation — no restatement found in its persona (decoupling holds).

## [v0.61.0] Delta grammar superseded — names its spec or lane run; parent child-delta form added
- **Disposition:** superseded → "a delta on a `delivered` entry reads `extent grows by <X> — in-flight, <spec-slug or lane-run>` (on a parent carrying a late child: `new child FEAT-YYY — in-flight, <spec-slug or lane-run>`); it names its spec or lane run … A delta whose spec closed — or whose lane run ended — without folding is an integrity defect." Invariant 5 gains the matching lane-run and roll-up-yields-to-stickiness clauses; the rationalization-table row re-worded ("whose spec or lane run died").
- **Tier failed:** n/a — supersession by ruling (record D7/D14 — lane runs as first-class delta owners; D2 amended — sticky-delivered parent carries a late child as a marked delta)
- **Content:** "a delta on a `delivered` entry reads `extent grows by <X> — in-flight, <spec-slug>`; it names its spec, lives under the entry's Deltas heading, and folds into the extent lines at the owning work's acceptance landing. A delta whose spec closed without folding is an integrity defect." · invariant 5 "A later spec touching a `delivered` feature never regresses its status; the change rides as a marked delta until that work's landing folds it."
- **Kept deliberately:** the Deltas-heading home, the fold-at-acceptance timing, and stickiness itself unchanged; `retired`-is-terminal clause verbatim.
- **Consumers assessed:** `feature-entry-template.md` Deltas comment superseded in lockstep (see its strip note); the feature command authors lane deltas in this grammar.

## [v0.61.0] BACKLOG boundary line superseded — KM-scoped, extent-growth exception (D13)
- **Disposition:** superseded → "**Tracking defects, tooling, or process work** — those live in `BACKLOG.md` where KM exists (a non-KM product has no queue; lane runs accept direct requests — the stated degrade path, never silently assumed away). Extent-growth improvement ideas are the exception: they ride the map as `proposed` deltas or obligation lines — the map is the capability backlog." Rationalization row extended to match.
- **Tier failed:** n/a — supersession by ruling (record D13, review finding 12 — R15 boundary scoped to KM-adopting repos; extent-growth ideas ride the map per feature-map D9)
- **Content:** "**Tracking defects, tooling, or process work** — those live in `BACKLOG.md`; the map carries product capabilities only" · rationalization reality cell "The map states what the product does, not what needs fixing. Defects, tooling, and process live in `BACKLOG.md`."
- **Kept deliberately:** defects/tooling/process still never become entries; the feature-map D22 pseudo-feature ban untouched.
- **Consumers assessed:** the feature command's triage (D13/D14) is the runtime consumer — bound by reference in the parallel seat.
