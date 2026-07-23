# Strip notes — `skills/patterns-entity-modeling`

Entry formats: `strips/README.md`. Plan-cluster-only skill (the data-model authoring layer, mounted on
the `technical-analyst` producer — sole consumers: `agents/technical-analyst.md` + `commands/plan.md`,
verified by grep). Single-consumer; in-wave rulings D9-authorized.

## [v0.19.0] Wave context — token-reduction build, wave 1 (D4 reference-by-ID)

Token-reduction epic **D4** (reference-by-ID; generalizes `authoring-slices` invariant 6). **Additions
only — zero strips this wave:**
- SKILL.md `data-model.md` structure preamble gained the "cite upstream by ID, don't re-quote it" rule,
  naming the **Data Sensitivity Summary** + **Entity Summary** tables as the ID index / coverage surface
  the plan reviewer grades entity→requirement traceability against.
- The Quality Checklist gained two items: cite traceability by ID (one-line gloss max), and keep the
  verbose **Sensitivity Details** blocks to per-attribute handling only (the level definitions and DS-XXX
  text referenced, not restated per block).

Both are **additions, not strips** — no existing content left the primitive (the skill's examples were
already ID-first: entity `Traceability:` lines cite FR-/US- IDs, not requirement text). No
`command-shape.md` touch this wave → no cross-command re-audit.
