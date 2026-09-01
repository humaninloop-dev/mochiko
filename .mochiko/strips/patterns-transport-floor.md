# Strip notes — `skills/patterns-transport-floor/`

Entry formats: `strips/README.md`. This file's first entry is the [v0.102.0] schema
conversion below (census-patterns J-P2): the skill was born by ruling at v0.71.0
(`teammate-message-races` D1–D7, `DECISIONS.md` 2026-08-14) and no post-birth edit had
removed content, so no strips file existed before the conversion.

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
