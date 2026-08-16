# Strip notes — `templates/features-index-template.md`

Entry formats: `strips/README.md`. Wave context: the feature-sizing & entry-points build wave
(record: `.mochiko/brainstorms/feature-sizing-and-entry-points/record.md`; `DECISIONS.md` row
2026-08-10). The change is almost wholly additive (nested rows, `unrefined` mark, the
dir-beside-entry layout note); one comment line was superseded.

---

## [v0.76.0] Template retired — superseded by schema-based template guidance (D1/D3/D8)
- **Disposition:** superseded → plugins/mochiko/schemas/features-index.yaml + mochiko-cli template features-index
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D1/D3/D8; record `.mochiko/brainstorms/schema-based-template-guidance/record.md`; `DECISIONS.md` "Template-schema CLI ruled")
- **Content (superseded template, full verbatim below):**

````markdown
<!-- Form: templates/artifact-format.md (the deliverable envelope). FEATURES.md is a
     succinct index, never a monolith: one line per capability, full entries live in
     per-capability files the index points at. Authored and maintained per
     mochiko:authoring-feature-map; delivery writes land at spec acceptance and at
     acceptance landings (implement or lane run) — where delivered work rows fold — never
     mid-run; /mochiko:feature stewardship writes (stub minting, retire, grooming) land
     directly. Register: `full` per artifact-format.md rule 11. -->

# Features

> The system as capabilities — the capability peer of `ARCHITECTURE.md`.
> Entries: `.mochiko/features/FEAT-XXX-<slug>.md` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

<!-- One line per capability: ID (linked to its entry file) · name · status · one-breath
     capability hook. Ordering: newest-relevant first — in-flight and row-carrying entries
     at the top, then delivered, then proposed, then retired. Keep the hook to one breath;
     the entry file carries everything else.
     Work rows: a capability's live/pending work rows show as transient sublines directly
     under its row, name prefixed "↳ ", tagged `live` or `pending` — these are delivery
     increments, not features, and a delivered row folds into the capability's extent and
     leaves the index. There is NO parent/leaf feature nesting.
     Domains: when the soft cap (~9 capabilities) trips, capabilities may be grouped under
     domain headers (mochiko:patterns-map-minimalism) — dormant below the cap; a small map
     stays flat, no headers.
     `unrefined` capability stubs show the mark after the status: `proposed (unrefined)`.
     Layout note: a capability's artifact directory `.mochiko/features/FEAT-XXX/` sits
     BESIDE its entry file `FEAT-XXX-<slug>.md` in the same directory — the entry file
     is not inside the artifact directory. -->

| ID | Capability | Status | Hook |
|----|------------|--------|------|
| [FEAT-001](.mochiko/features/FEAT-001-{{slug}}.md) | {{capability_name}} | {{status}} | {{one_breath_hook}} |
|  | ↳ `live` {{work_row}} | in {{spec-slug}} | {{increment_hook}} |
| [FEAT-002](.mochiko/features/FEAT-002-{{slug}}.md) | {{stub_name}} | proposed (unrefined) | {{one_breath_hook}} |
````
- **Kept deliberately:** Every line of guidance preserved — lifted into `plugins/mochiko/schemas/features-index.yaml` (skeleton / contract / overview / register / density) and rendered by `mochiko-cli template features-index`; the `.yaml` ships in the plugin as the raw-Read first-class degraded path (D8, GI-020, no install regression). Net-new per-section `check` lines were authored under D7 (disclosed, not lifted). Nothing dropped.
- **Consumers assessed:** `commands/specify.md` (re-pointed by P4) · `commands/setup.md` (re-pointed by P4) · `skills/authoring-feature-map/SKILL.md` (re-pointed by P5). V2 fidelity PASS 2026-08-16 (schema graded 8/8 at the M3 gate).

## [v0.68.0] Re-type: capability lines + transient work-row sublines; leaf/parent nesting removed (wave context)

Wave context: the PM-role & feature-derivation build wave (record:
`.mochiko/brainstorms/pm-role-and-feature-derivation/record.md`; `DECISIONS.md` row 2026-08-13).
The index re-types — one line per durable **capability**, with **work rows as transient sublines**
(pending/live) rather than nested leaf features; parent/leaf nesting dies; **domain headers** are
noted as dormant, appearing only at the soft cap (~9 capabilities, `mochiko:patterns-map-minimalism`).
Per the record's D6 exhaustive per-clause inventory. Pure `feature`→`capability` swaps ride the
decision row.

## [v0.68.0] Nesting subline rule superseded → work-row sublines + dormant domain headers
- **Disposition:** superseded → the one-line comment re-typed: work rows show as transient `live`/`pending` sublines under their capability (delivery increments, not features; a delivered row folds and leaves the index); NO parent/leaf nesting; domain headers dormant until the soft cap; the example table re-shaped to a capability row + work-row subline + stub
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-13 pm-role-and-feature-derivation; record D2 work rows + fold, D4b domains-at-cap, D6 — nesting dies, D7 capability-batch)
- **Content (verbatim — superseded lines):**
  - Comment (newlines collapsed): `Nesting: a leaf's row sits directly under its parent's row, name prefixed "↳ " (two levels max — parent then leaves; a parent's status is its roll-up).`
  - Comment (re-keyed "delta-carrying" → "row-carrying"): `Ordering: newest-relevant first — in-flight and delta-carrying entries at the top`
  - Table rows: "| ID | Feature | Status | Capability |" · "| [FEAT-001](.mochiko/features/FEAT-001-{{slug}}.md) | {{parent_name}} | {{status}} | {{one_breath_hook}} |" · "| [FEAT-002](.mochiko/features/FEAT-002-{{slug}}.md) | ↳ {{leaf_name}} | {{status}} | {{one_breath_hook}} |" · "| [FEAT-003](.mochiko/features/FEAT-003-{{slug}}.md) | {{stub_name}} | proposed (unrefined) | {{one_breath_hook}} |"
- **Protected-content reconciliation:** the nesting-row rule (leaf under parent, "↳" prefix, two-level cap, parent-status-is-roll-up) was the feature-sizing D2/D3 ruling (v0.61.0 header entry's "pure additions", carried as index shape) — superseded now by pm-role D6 (nesting dies) and D2 (sublines are transient rows, not sub-features). Not a silent drop.
- **Kept deliberately:** the "↳ " prefix survives, re-purposed for transient work-row sublines; ordering-newest-relevant-first survives (re-keyed to row-carrying); the `unrefined` mark convention and the dir-beside-entry layout note survive verbatim.
- **Consumers assessed:** `authoring-feature-map` (dropped the leaf-lines-under-parent checklist line same wave — its strip) · the feature command writes index lines in this shape (parallel seat).

## [v0.68.0] Header write-timing re-keyed — folds at landings; promotion dropped
- **Disposition:** superseded → the re-typed header comment (one line per **capability**; delivery writes at acceptance landings "where delivered work rows fold"; stewardship writes — stub minting, retire, grooming — land directly)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-13 pm-role-and-feature-derivation; record D2 fold, D6 — retroactive promotion superseded by the growth door)
- **Content (verbatim — superseded clauses):**
  - "one line per feature, full entries live in per-feature files the index points at"
  - "/mochiko:feature stewardship writes (stub minting, promotion, retire, grooming) land directly"
- **Protected-content reconciliation:** the write-timing comment was last ruled at v0.61.0 (feature-sizing G4, "Kept deliberately") — its landing points survive; `promotion` drops from the stewardship list (D6/D8: the growth door replaces retroactive promotion) and the fold is named at the acceptance landings.
- **Kept deliberately:** succinct-index-never-a-monolith, delivery-writes-at-spec-acceptance-and-acceptance-landings, never-mid-run, stewardship-direct — all survive re-worded; both original landing points survive.
- **Consumers assessed:** `authoring-feature-map` invariant 6 superseded in lockstep (its strip) · the feature command writes index lines (parallel seat).

## [v0.61.0] Header write-timing comment superseded — lane landings and stewardship writes added
- **Disposition:** superseded → "delivery writes land at spec acceptance and at acceptance landings (implement or lane run), never mid-run; /mochiko:feature stewardship writes (stub minting, promotion, retire, grooming) land directly."
- **Tier failed:** n/a — supersession by ruling (lead ruling G4, citing record D6/D12; D7/D14 lane-run landings)
- **Content:** "writes land at spec acceptance and at implement's acceptance landing, never mid-run."
- **Kept deliberately:** never-mid-run for delivery writes; the succinct-index-never-a-monolith framing and both original landing points verbatim inside the new wording. Pure additions alongside (no strip owed): the nesting row comment (leaf row under parent row, "↳ " prefix, two levels max, parent status = roll-up) · the `proposed (unrefined)` mark convention · the G2 layout note (artifact dir `.mochiko/features/FEAT-XXX/` sits BESIDE entry file `FEAT-XXX-<slug>.md`, entry not inside the dir) · the two example rows.
- **Consumers assessed:** `authoring-feature-map` SKILL.md invariant 6 superseded in lockstep (its strip note) · the feature command writes index lines in this shape.
