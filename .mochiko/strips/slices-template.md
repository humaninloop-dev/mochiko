# Strip notes — `templates/slices-template.md`

Entry formats: `strips/README.md`.

## [v0.49.0] Template retired — `slices.md` becomes the spec's Delivery Slices section
- **Disposition:** superseded → `spec-template.md` (Delivery Slices section: slice table · extend obligations · Feature-Done · Graduation contract; single-slice null form one line); file deleted
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D6)
- **Content:** the 135-line template — Spec stamp header, Slice order table, per-slice blocks, Cross-cutting placements, Feature-Done, Graduation contract, 7 usage notes. Full text: git history at v0.48.0.
- **Kept deliberately:** the Graduation contract relocated near-verbatim (slice-scoped runs · artifact layout minus `task-mapping.md` · extend-mode · graded amendment with migration now carried as cycle cards · regression safety). **Dropped by ruling:** the Spec stamp + staleness guard (a section cannot drift from its own document) and the overlay-purity notes (no overlay exists). "Absence of the file is the whole-spec state" inverted: the single-slice line is now the explicit, graded record of the depth call.
- **Consumers assessed:** plan/implement (Graduation-contract pointer re-keyed to the spec section) · router · authoring-slices (fills the section now).
