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
