# Strip notes — `skills/patterns-api-contracts/`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` + the wave-2 rulings R1–R4/T3;
ratified 2026-07-24).

## [v0.23.0] quickstart.md becomes conditional + capped (T3, user-ruled)
- **Disposition:** ruled change, not a strip — recorded here because it changes the authored artifact set: `quickstart.md` (kinako: 17.5k B, authored unconditionally) is now authored **only when the feature has a real external-integration surface**, capped ≤ 150 lines, citing the contract instead of re-documenting it; the null path is one line in `plan.md`'s artifact table, never a stub file
- **Content:** the SKILL.md gains the Quickstart section (this skill owned the artifact per cross-references but never defined it — the ownership gap closed with the compact definition); endpoint↔FR/US traceability table designated the contract's ID index. Ripples: `commands/plan.md` (goal / deliverables / Phase-2 produce / done-condition read "when applicable"), `templates/plan-template.md` artifact row, `agents/technical-analyst.md` deliverable #6, review-plan-artifacts' quickstart checklist (conditionality check added).
- **Consumers assessed:** plan's done-condition (now reads quickstart-or-null-path) + review-plan-artifacts (checklist retargeted) + slices-template's Graduation-contract artifact-layout line (lists quickstart at the feature root among accumulating artifacts — an absent conditional artifact simply never accumulates; no edit needed).
