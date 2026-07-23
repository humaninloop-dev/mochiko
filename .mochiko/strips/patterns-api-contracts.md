# Strip notes — `skills/patterns-api-contracts`

Entry formats: `strips/README.md`. Plan-cluster-only skill (the API-contract authoring layer, mounted on
the `technical-analyst` producer — sole consumers: `agents/technical-analyst.md` + `commands/plan.md`,
verified by grep). Single-consumer; in-wave rulings D9-authorized.

## [v0.19.0] Wave context — token-reduction build, wave 1 (D4 reference-by-ID)

Token-reduction epic **D4** (reference-by-ID; generalizes `authoring-slices` invariant 6). **Additions
only — zero strips this wave:**
- SKILL.md Traceability section gained the "cite upstream by ID, never re-quote it" rule, naming the
  endpoint→requirement table as the contract's ID index / coverage surface the plan reviewer grades
  endpoint coverage against.
- The Quality Checklist gained two items: cite FR-/US- by ID (one-line description max), and keep schema
  descriptions compact — data-model attribute definitions referenced (types map by reference to
  `data-model.md`), not restated.

Both are **additions, not strips** — no existing content left the primitive (the type-mapping table
already referenced the data model rather than redefining it). No `command-shape.md` touch this wave →
no cross-command re-audit.
