<!-- Form: templates/artifact-format.md (the deliverable envelope). FEATURES.md is a
     succinct index, never a monolith: one line per feature, full entries live in
     per-feature files the index points at. Authored and maintained per
     mochiko:authoring-feature-map; writes land at spec acceptance and at implement's
     acceptance landing, never mid-run.
     Register: `full` per artifact-format.md rule 11. -->

# Features

> The system as capabilities — the capability peer of `ARCHITECTURE.md`.
> Entries: `.mochiko/features/FEAT-XXX-<slug>.md` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

<!-- One line per feature: ID (linked to its entry file) · name · status · one-breath
     capability hook. Ordering: newest-relevant first — in-flight and delta-carrying
     entries at the top, then delivered, then proposed, then retired. Keep the hook to
     one breath; the entry file carries everything else. -->

| ID | Feature | Status | Capability |
|----|---------|--------|------------|
| [FEAT-001](.mochiko/features/FEAT-001-{{slug}}.md) | {{name}} | {{status}} | {{one_breath_hook}} |
