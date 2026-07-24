# Strip notes — `skills/patterns-entity-modeling/`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4;
ratified 2026-07-24).

## [v0.23.0] Per-attribute Sensitivity Details blocks → once-per-document defaults + one row per attribute
- **Disposition:** revised per the wave-2 form ruling (R2's self-containment floor) — the 7-row per-attribute aspect table is deleted from the canonical data-model template (SKILL.md) and `references/DATA-SENSITIVITY.md`; handling-by-level defaults are stated **once per document**, each Confidential+ attribute is **one row** (Level · Retention · Access · Deviations · Compliance)
- **Tier failed:** artifact density: encryption/audit/masking are level-determined — repeating them per attribute restated the level default N times (kinako data-model.md 42k B); retention/access/compliance are the genuinely per-attribute content and are exactly what the row keeps
- **Content:** the `#### <attr> (Level)` + 7-row `| Aspect | Requirement |` block format (moved to the defaults matrix + deviation cells); the `### Classification Levels` legend table inside the artifact template (taxonomy restatement — the skill and DATA-SENSITIVITY.md own it; the once-per-doc handling-defaults matrix replaces its in-artifact function); entity intro blockquote collapsed onto the traceability line. Validation checklists (SKILL + reference) re-pointed at rows/defaults.
- **Consumers assessed:** plan producer + review-plan-artifacts ("Sensitivity details" check rephrased to the row form this wave) + `scripts/validate-model.py` (checked: per-entity `Sensitivity Details` heading regex + Sensitivity-column checks still satisfied by the row form — no script change needed).
