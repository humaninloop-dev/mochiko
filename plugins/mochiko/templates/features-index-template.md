<!-- Form: templates/artifact-format.md (the deliverable envelope). FEATURES.md is a
     succinct index, never a monolith: one line per feature, full entries live in
     per-feature files the index points at. Authored and maintained per
     mochiko:authoring-feature-map; delivery writes land at spec acceptance and at
     acceptance landings (implement or lane run), never mid-run; /mochiko:feature
     stewardship writes (stub minting, promotion, retire, grooming) land directly.
     Register: `full` per artifact-format.md rule 11. -->

# Features

> The system as capabilities — the capability peer of `ARCHITECTURE.md`.
> Entries: `.mochiko/features/FEAT-XXX-<slug>.md` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

<!-- One line per feature: ID (linked to its entry file) · name · status · one-breath
     capability hook. Ordering: newest-relevant first — in-flight and delta-carrying
     entries at the top, then delivered, then proposed, then retired. Keep the hook to
     one breath; the entry file carries everything else.
     Nesting: a leaf's row sits directly under its parent's row, name prefixed "↳ "
     (two levels max — parent then leaves; a parent's status is its roll-up).
     `unrefined` capability stubs show the mark after the status: `proposed (unrefined)`.
     Layout note: a feature's artifact directory `.mochiko/features/FEAT-XXX/` sits
     BESIDE its entry file `FEAT-XXX-<slug>.md` in the same directory — the entry file
     is not inside the artifact directory. -->

| ID | Feature | Status | Capability |
|----|---------|--------|------------|
| [FEAT-001](.mochiko/features/FEAT-001-{{slug}}.md) | {{parent_name}} | {{status}} | {{one_breath_hook}} |
| [FEAT-002](.mochiko/features/FEAT-002-{{slug}}.md) | ↳ {{leaf_name}} | {{status}} | {{one_breath_hook}} |
| [FEAT-003](.mochiko/features/FEAT-003-{{slug}}.md) | {{stub_name}} | proposed (unrefined) | {{one_breath_hook}} |
