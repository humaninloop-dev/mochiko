# Strip notes — `skills/authoring-technical-requirements/`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4/T2;
ratified 2026-07-24).

## [v0.23.0] Description fields collapsed into the statement line across TR/C/IP/NFR blocks (T2, user-ruled)
- **Disposition:** revised per the wave-2 T2 ruling — the separate `**Description:**` paragraph field is deleted from all block templates (`references/ARTIFACT-TEMPLATES.md`) and field-definition tables (SKILL.md + reference); the ID line's statement IS the description
- **Tier failed:** artifact density (epic D4 extension): kinako's requirements.md 61k B / constraints-and-decisions.md 67k B were dominated by per-item field ceremony (label lines + a Description paragraph restating the statement), re-paid ~10× per feature
- **Content:** per-block forms compressed — TR: `**FR-XXX · MUST** — statement` + Criteria bullets + `**Deps:**` line (was Title/Source/Priority/Description/AC-checkboxes/Dependencies-list); C: type·severity·source on the statement line + one-line Impact (was 6 labeled fields); D: one-to-two-line Context + compact options table + ≤3-line Rationale + one-line Consequences (options/choice/ADR substance kept — `patterns-technical-decisions` owns the technique); IP: same collapse; NFR: statement line + Target/Measured/Applies-to lines (was 6 labeled fields + paragraph Requirement + paragraph Measurement Method). Summary tables kept and designated the **ID index** per `templates/artifact-format.md`. `references/TRACEABILITY-PATTERNS.md` pattern examples aligned to the statement-line form.
- **Consumers assessed:** plan producer (technical-analyst) + review-plan-artifacts (ARTIFACT-CHECKLISTS retargeted this wave) + review-feasibility (reads the artifacts; field-agnostic, checked — no edit needed) + downstream tasks/implement readers (consume IDs + statements, unaffected).

## [v0.23.0] Corrections landed in-wave (not strips)
- **Content:** (1) ARTIFACT-TEMPLATES' constraint Severity value set said `Hard` ("all constraints are hard boundaries by definition") while SKILL.md's field table says `blocking / significant / minor` — aligned to the SKILL (blocking/significant/minor), the pre-existing drift resolved toward the skill body; (2) `Constitution Alignment` field renamed `Governance alignment` (post-dissolution vocabulary, the plan-wave Constitution→Governance precedent), now optional-and-omitted when no principle applies.
