# Strip notes — `skills/authoring-user-stories/`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4/T1;
ratified 2026-07-24).

## [v0.23.0] Acceptance scenarios compressed to one line each, cap 2-4 → 2-3 (T1, user-ruled)
- **Disposition:** revised per the wave-2 T1 ruling (deleted prose replaced by the dense form; nothing relocated)
- **Tier failed:** artifact density (epic D4 extension): multi-line Given/When/Then prose re-paid ~10× per feature via mandated reads; the G/W/T grammar carries the testability, the line breaks carried nothing
- **Content:** the story-format block's multi-line scenario shape; the "2-4 scenarios" rule (now 2-3 with the compound-story rationale); the multi-line good example (now one line); quality-checklist counts. `references/EXAMPLES.md` rewritten to the dense form (3 examples: journeys ≤ 2 lines, one-line why/test/scenarios — same substance, same story content).
- **Consumers assessed:** spec.md producers (specify) + review-specifications (retargeted this wave: density-is-not-a-gap note) + `scripts/validate-user-stories.py` (checked: numbered-scenario + G/W/T keyword regexes match the one-line form — no script change needed).
