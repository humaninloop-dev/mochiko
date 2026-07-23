# Strip notes — `skills/patterns-vertical-tdd/`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`, D6a; ratified 2026-07-23).

## [v0.22.0] TEST: grammar split out of CYCLE-STRUCTURE.md
- **Disposition:** relocated → `references/TEST-GRAMMAR.md` (new; ownership stays with this skill) — the runtime verifier (`testing-end-user`) now loads only the grammar (~5k B) instead of the full CYCLE-STRUCTURE.md (18.6k B)
- **Tier failed:** pure waste (D6a): the verifier's mandated cross-skill read pulled 13k B of cycle-structuring content it never uses
- **Content:** the `## Verification Task Requirements` block (what verification MUST include · Unified TEST: format + runtime-classification table · field definitions · action modifiers · assert patterns · four worked examples · bad-verification examples · why-this-matters · legacy format support) moved verbatim; CYCLE-STRUCTURE.md keeps a pointer section. Retargeted refs: testing-end-user SKILL.md (5) + TASK-PARSING.md (3), patterns-vertical-tdd SKILL.md (2), tasks-template.md (1).
