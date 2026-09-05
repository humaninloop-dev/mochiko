# Strip notes — `skills/patterns-transport-floor/`

Entry formats: `strips/README.md`. This file's first entry is the [v0.102.0] schema
conversion below (census-patterns J-P2): the skill was born by ruling at v0.71.0
(`teammate-message-races` D1–D7, `DECISIONS.md` 2026-08-14) and no post-birth edit had
removed content, so no strips file existed before the conversion.

<!-- Wave context: wave 5 of the CLI schema-delivery build (v0.106.0) — the patterns family
converts: each member's rules are rendered at fire by `mochiko-cli` from the migration log the
plugin carries at `plugins/mochiko/migrations/`, and the skill reads no schema file. Ruling for
every [v0.106.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D3 as amended
(the skill-side form — `!` runs in `SKILL.md` and at subagent preload), D7 (the `PreToolUse`
`Skill` limb), D9 (families in the arc's order), and D10 clause 6 (the budgeted quantity re-keys
to body + rendered output), with the wave-open rulings in that session's `wave5-plan.md` and the
`DECISIONS.md` 2026-09-04 row. Pre-edit verbatim text:
`git show 7d098b9:plugins/mochiko/skills/patterns-transport-floor/SKILL.md`. -->

## [v0.106.0] the Rules block — raw schema Read superseded by CLI delivery

- **Disposition:** superseded → `## Rules — delivered by mochiko-cli`: the positive-confirmation
  halt clause plus seven `!` lines, one per rendered block (the preamble and the six sections),
  and the read-back sentence.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3 as amended, the
  skill-side form)
- **Content:** verbatim —

  ```
  ## Rules — load the schema first

  Your first action, before composing or opening any multi-seat work: **Read `schema.yaml`
  (this skill's own directory) raw, in full** — the patterns family ships no common file, so
  the pair's own schema is the whole first action. The schema is the source of truth for this
  floor's binding rules, nested in six sections, each addressable by its section ID:
  `patterns-transport-floor.sec.trigger` · `patterns-transport-floor.sec.scope` ·
  `patterns-transport-floor.sec.discipline` · `patterns-transport-floor.sec.inputs` ·
  `patterns-transport-floor.sec.disclosure` · `patterns-transport-floor.sec.reserved`.
  Interpret it live: a rule's `kind:` names what it is, and an absent `kind:` reads
  `constraint`; a rule's `when:` resolves against the schema's declared `conditions:` — the
  two lanes, `messaging` and `shared_write_surface` — and gates when the obligation applies,
  never whether it is delivered; a rule of `class: floor` is always read and always delivered
  whatever its `when:`; a `pointer:` rule binds you to that file's or skill's procedure,
  referenced never restated; labels come from `plugins/mochiko/schemas/skill-labels.yaml`.
  The floor pin: the 11 rules of `class: floor` are non-waivable. Before the first
  composition or messaging step, state the floor count back — a skipped or partial read
  leaves that count blank: halt and surface it, and halt likewise if the schema's
  `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** everything outside this section, byte for byte — the title, the opening
  paragraph, and every procedural section. The block's substance survives in the render: the six
  section IDs are now the six `--section` arguments, and the interpretation grammar the block
  taught is printed as the preamble's `legend` with every fire. The block's inline naming of
  this schema's own `conditions:` is covered by the render, whose preamble prints the whole
  `conditions` block with each value and its resolution note — strictly more than the sentence
  carried.
- **Consumers assessed:** none shared — the block was this skill's own text, and this family
  ships no common file.

## [v0.106.0] the hand-pinned `class: floor` count

- **Disposition:** superseded → the CLI-printed pin. The count is the `- class: floor · N rules`
  line under `pins` in the preamble block, and the `floors:` line beneath it lists the ids; the
  read-back sentence now cites both rather than carrying a number of its own.
- **Tier failed:** n/a — supersession by ruling (`cli-schema-delivery` D3, "the counts are computed
  and printed by the CLI, never hand-pinned"; the wave-4 re-key ruling adding the `floors:` index)
- **Content:** verbatim —

  ```
  The floor pin: the 11 rules of `class: floor` are non-waivable. Before the first
  composition or messaging step, state the floor count back — a skipped or partial read
  leaves that count blank: halt and surface it, and halt likewise if the schema's
  `class: floor` count disagrees with the pin.
  ```

- **Kept deliberately:** the read-back obligation itself (skill-content-schema D6 as amended),
  re-keyed from a hand-pinned number to the printed pin and its id list — the skill still states
  the floor set back before its first procedural step, and a blank or partial read-back still
  halts. The independent second number the `.md` used to hold is booked as a loss, per D3's own
  rationale; the contract suite's `assert_floor_delivery` now carries what it used to check.
- **Consumers assessed:** `.claude/rules/mochiko/primitive-edits.md` skill-pair criterion 3, which
  demanded the hand-pinned count — amended in this same wave to branch on a converted skill.

## [v0.102.0] Converted to the `.md` + schema pair form — rule content relocated to `schema.yaml` (wave 2B, patterns family)

- **Disposition:** superseded → the pair form: obligation content relocated into
  `plugins/mochiko/skills/patterns-transport-floor/schema.yaml` (14 rules — 11 floor ·
  2 must · 1 advisory — under the patterns six-section set), the `SKILL.md` body keeping
  the epigraph, the Overview hazard teaching, the new "Rules — load the schema first"
  block (floor pin 11 + read-back), and the Sibling paragraph. The frontmatter
  `description:` is byte-untouched.
- **Tier failed:** n/a — supersession by ruling (`skill-content-schema` D1–D9 as amended,
  `DECISIONS.md` 2026-09-01, D8/C4 supersession-transfer; the wave-2 patterns family-door
  ruling, same date — 9 carriers convert on the secondary drivers, section set
  `trigger · scope · discipline · inputs · disclosure · reserved`, no common file).
  Every relocated rule below is birth-ruling machinery; each move cites the
  `DECISIONS.md` 2026-08-14 `teammate-message-races` row (D1–D7), the R-c ceremony class
  for birth-by-ruling bodies. Protection transfers to the rule IDs via
  `.mochiko/provenance.yaml` (D8/C4) — all 14 rules anchored
  "2026-08-14 teammate-message-races".
- **Content (superseded body sections, section-level inventory with the relocation map;
  verbatim text survives in git history pre-v0.102.0 and verbatim-in-substance in the
  named schema rules):**
  - Overview neutrality sentence — `Transport *choice* stays neutral — a seat may be a
    teammate or a subagent, the lead's per-seat call (realignment D5). Transport *use* is
    what this floor disciplines` → `patterns-transport-floor.governs-use-never-choice`.
  - `## When NOT to Use` (four bullets: solo run · single seat no messaging · transport-
    choice questions · sizing questions) → the solo/single-seat bullets fold into the
    chartered-freedom clause of `patterns-transport-floor.neither-lane-waivable`; the
    transport-choice bullet and the sizing bullet (`whether an artifact should exist or
    how small it should be belongs to the three minimalism siblings, another axis`) fold
    into `patterns-transport-floor.governs-use-never-choice` (census J-P10: the sizing
    route folds rather than minting a 15th rule — lead-approved landing annotation).
  - `## The trigger — two lanes, each non-waivable when it fires` (message legs fire on
    any multi-seat run with cross-seat or lead-relayed messaging; topology legs fire on a
    shared write surface; `Neither lane is waivable once fired — a lead cannot legally
    depart the floor mid-crisis, which is exactly the shape a waivable floor would
    sanction.`) → `patterns-transport-floor.message-lane-trigger` ·
    `patterns-transport-floor.topology-lane-trigger` ·
    `patterns-transport-floor.neither-lane-waivable`, the lanes declared as the
    `conditions:` dimensions `messaging` / `shared_write_surface` (surface-presence).
  - `## The seven legs` (numbered list 1–7, each with its lane tag) →
    `patterns-transport-floor.composition-steer` (leg 1, teammate-message-races D4) ·
    `patterns-transport-floor.single-writer-per-surface` (leg 2) ·
    `patterns-transport-floor.mesh-hold` (leg 3) ·
    `patterns-transport-floor.content-pinned-supersession` (leg 4) ·
    `patterns-transport-floor.quiesce-before-cold-grade` (leg 5) ·
    `patterns-transport-floor.no-ritual-sends` (leg 6) ·
    `patterns-transport-floor.fan-in-confirmation` (leg 7) — all `class: floor`, each
    carrying its lane as `when:`; each move cites the 2026-08-14 `teammate-message-races`
    row.
  - `## The platform floor — version and transport facts` → the version bullet
    (`Version floor ≥ v2.1.224` with the masked-failure rationale) to
    `patterns-transport-floor.version-floor` (`class: floor`, lead-ruled at plan
    approval, resolving the census §B table/detail-line disagreement toward the
    row-grain enumeration); the teammate-transport bullet (delivery documented-automatic,
    ordering undocumented, the doc-anchor ownership-split quote) to
    `patterns-transport-floor.ordering-undocumented`; the cross-session bullet to
    `patterns-transport-floor.cross-session-cited-in-scope-only`.
  - `## Quality Checklist` (eight rows restating the lanes, legs, and version floor) —
    deleted as a body surface; every row's substance is a schema rule above (the
    checklist was not KEPT-protected: this skill had no strips file and no survivor
    ruling, protection basis DECISIONS-traceability only, census §A).
- **Kept deliberately:** the epigraph (`The message arrives; the work does not start
  until the lead opens it.`) · the Overview hazard teaching (message races and write
  collisions made concrete) · the `## Sibling` paragraph with the sound-loop neutrality
  cross-pointer · the `description:` byte-identical (450 chars), including its
  non-waivable-lane and governs-use-never-choice clauses — the discovery surface never
  moves (skill-content-schema D3).
- **Consumers assessed:** `patterns-sound-loop` (its neutrality line points here —
  boundary intact, wording untouched on its side) · `authoring-epic` (two
  `mochiko:patterns-transport-floor` pointers in its schema — name-shaped, unaffected) ·
  the DM-chartered commands' floor lines and `implement.md`/`feature.md` transport
  references (skill-name references, no section-anchor links) · the router
  `skills/mochiko/SKILL.md` row (names the skill, unaffected). No shipped surface links a
  removed section anchor of this SKILL.md.
- **Budget:** unbudgeted at birth (hard-cap-only, body 5,398 / desc 450 at v0.71.0,
  `primitive-cost-budgets.md`); the conversion seeds its first budget row via the third
  seeding path ("a ruled schema conversion") in the wave-2B ledger update — measured
  delivered-at-invoke payload **10,556 (body 2,412 + schema 8,144)**, canonical-snippet
  counts, no +25% headroom; description unchanged at 450.
- **Checker pre-pass:** `check-skill-schema.py --skill patterns-transport-floor` — PASS,
  0 findings, 2 warnings. Both warnings are the declared-value coverage class
  (`conditions.messaging: value 'absent'` · `conditions.shared_write_surface: value
  'absent'` named by no rule's `when:`) and are correct, not a hole: both lane triggers
  and all seven legs are `class: floor`, and the checker excludes floors from coverage
  claims (a floor is always delivered whatever its `when:` — the C4 semantics), so both
  lanes report "present: (no rule activates)" and the `absent` poles are naturally
  unnamed. The same class ships on the landed tree (`review-feasibility`'s
  `store_delta`).
