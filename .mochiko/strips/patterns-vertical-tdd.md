# Strip notes — `skills/patterns-vertical-tdd/`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction waves 1–2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`, D6a + D4/wave-2 rulings).

## [v0.23.0] task-mapping.md gains its canonical compact shape (wave 2)
- **Disposition:** addition, not a strip — recorded here because it materially changes the authored artifact: `task-mapping.md` had **no prescribed structure anywhere** (the commands called it "the freehand story→cycle mapping"), and kinako's freehand form ran to 45.9k B for one slice
- **Content:** SKILL.md's Mapping-Stories-to-Cycles section now carries the canonical shape — the Story→Cycle table (Case column for Simple/Split/Merge with one-line why) + a per-cycle table (≤ 2-line rationale cells) + an optional Slicing-notes section (≤ 3 lines each, omit when empty), under the `artifact-format.md` envelope. `commands/tasks.md`'s two "freehand" mentions updated to "compact".
- **Consumers assessed:** tasks producer (task-architect — authors it) + review-task-artifacts (mapping checklist grades coverage/slice-quality, shape-agnostic; density note added this wave) + implement's producer (reads it as a design input — a table serves that read better than prose).

## [v0.22.0] TEST: grammar split out of CYCLE-STRUCTURE.md
- **Disposition:** relocated → `references/TEST-GRAMMAR.md` (new; ownership stays with this skill) — the runtime verifier (`testing-end-user`) now loads only the grammar (~5k B) instead of the full CYCLE-STRUCTURE.md (18.6k B)
- **Tier failed:** pure waste (D6a): the verifier's mandated cross-skill read pulled 13k B of cycle-structuring content it never uses
- **Content:** the `## Verification Task Requirements` block (what verification MUST include · Unified TEST: format + runtime-classification table · field definitions · action modifiers · assert patterns · four worked examples · bad-verification examples · why-this-matters · legacy format support) moved verbatim; CYCLE-STRUCTURE.md keeps a pointer section. Retargeted refs: testing-end-user SKILL.md (5) + TASK-PARSING.md (3), patterns-vertical-tdd SKILL.md (2), tasks-template.md (1).
