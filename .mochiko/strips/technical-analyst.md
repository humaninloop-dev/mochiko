# Strip notes — `agents/technical-analyst`

Entry formats: `strips/README.md`. Wave context: the plan cluster wave (v0.15.0). Plan-cluster-only
agent (the analysis+design producer, mounted on `plan`) — strips ruled in-wave (single consumer).

## [v0.15.0] "Skills Available" scope duplication
- **Disposition:** scope enumeration relocated → the four skills themselves
  (`mochiko:authoring-technical-requirements`, `mochiko:patterns-technical-decisions`,
  `mochiko:patterns-entity-modeling`, `mochiko:patterns-api-contracts`), which already single-source
  it — each skill's `description:` carries its scope; the skill **names + a one-line reach-for-it hint
  are kept** (they carry the team-form function — teammates ignore `skills:` frontmatter, so the
  in-body names are how a spawned teammate learns its skills)
- **Tier failed:** 1 (a second home for each skill's scope — the bullets restated what each skill's
  `description:` already single-sources)
- **Content:** the four full scope descriptions in the "Skills Available" bullets — "Guidance on
  writing technical requirements, constraints, non-functional requirements, and data sensitivity
  classifications with proper traceability and measurability standards.", "Evaluate technology
  alternatives and document decisions in ADR format with criteria weighting, trade-offs, and
  consequences.", "DDD-style entity extraction including attributes, relationships, state machines,
  and validation rules.", and "RESTful API design with endpoint mapping, schema definition, error
  handling, and OpenAPI specification."
- **Consumers assessed:** plan only (technical-analyst is plan-only). One instance of the 7-agent
  library-wide "Skills Available" pattern; ruling in-wave is D9-authorized (single consumer) and noted
  so the escalation lands consistently across all seven agent personas — the `devils-advocate` instance
  is raised for the ≥3-consumer escalation, while `task-architect` (v0.14.0, 2 consumers) and
  `principal-architect` (this wave, 2 consumers) were ruled in-wave under the 2-consumer allowance.

## [v0.15.0] KEPT: the "Quality Standards" persona enumeration (Traceable / Measurable / … / Realistic)
- **Tier-2 evidence:** the agent↔skill composition axis blesses persona (what the agent cares about)
  in the agent; "Quality Standards" is the analyst's *taste*, explicitly disclaimed — "this is the
  *taste* you bring, not the format spec. The concrete formats and procedures live in your skills,
  which are the single source of truth" — and closed with the single-source reference to the four
  skills (the IP-XXX format, the ≥2-alternatives ADR evaluation, the entity schema, the OpenAPI
  structure "live in [the four skills] … Consult them there rather than a copy in this persona"). Persona
  altitude, not a behavior-driving procedure restatement — distinct from the "Skills Available" strip
  above, which was a scope catalog pointing at the very homes it copied. Matches the `task-architect`
  "What You Produce" persona keep.
