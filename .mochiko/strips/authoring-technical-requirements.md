# Strip notes — `skills/authoring-technical-requirements/`

Entry formats: `strips/README.md`. Wave context: [v0.28.0] entries — skill-succinctness wave 4
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.23.0] entries — workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4/T2;
ratified 2026-07-24).

## [v0.28.0] Reference-copied field tables, homed mistake rows, and excuse-column red flags stripped (body 229 → 135, −41%, in-band)
- **Disposition:** deduped → `references/ARTIFACT-TEMPLATES.md` (Read: every field appears in
  its Field Definitions with extra Format/Rules columns, plus document templates — and the SKILL
  already declared it the home): all five in-body field tables (TR / C / D / IP / NFR) and the
  sign-in decomposition example (richer 4-TR worked table there) · deleted (Tier 1, in-file
  homes): the §4/§5 canonical-home blockquotes (restated the top analysis-vs-downstream router
  blockquote, which stays; the x-integration field list and four-level taxonomy they enumerated
  live only in `patterns-api-contracts` / `patterns-entity-modeling` — one-line canonical-home
  clauses folded into the §4/§5 declaration paragraphs), the Completeness-check line (its six
  checks restate the kept Quality Checklist), the Red Flags trigger bullets + no-exceptions list
  (the six bullets map ~1:1 onto the Common Rationalizations table's Excuse column — five are
  semantic parallels, none verbatim; bullet 4's excuse shifted, "sensitivity is obvious"
  (authoring shortcut) vs the kept row's "classification is a security team concern"
  (delegation excuse) — its substance is homed at §5 + rationalizations row 4. STOP framing
  kept as one paragraph, the table kept whole as the discipline core, vertical-tdd precedent) · **Common Mistakes deleted whole** (all 6 rows homed: transcribing → decomposition
  rule + rationalizations row 1; unmeasurable NFRs → "'Fast' is not a requirement" + the
  reference's NFR-categories table; never-bounded → §4's kept optimistic-maps paragraph;
  preferences-disguised → the reference's Distinguishing table + violation test; unclassified →
  §5 + rationalizations row 4; orphans → Traceability Rules + checklist) · densified: the
  technology-agnostic Wrong/Right table → rule + one pair (constraints-MAY-name-tech exception
  kept), the D-technique blockquote → 2 lines (the boundary is also in the description,
  When-NOT-to-Use, ARTIFACT-TEMPLATES' Part-2 blockquote, and TRACEABILITY-PATTERNS' note —
  width-only, no line delta)
- **Tier failed:** 1 throughout (every cut had a verified richer home, most in the already-
  declared reference) · n/a for the densifications
- **Content:** five field tables, one example sentence, two blockquotes, one 4-row table, six
  mistake subsections, eleven red-flag/no-exception bullets; nothing written to `templates/` —
  dedups run against pre-existing reference content, D4's destination ban not engaged
- **Consumers assessed:** wave-open enumeration — 8 citing files (technical-analyst, plan,
  patterns-entity-modeling ×2, patterns-technical-decisions ×2, artifact-format, mochiko
  router); none links a section anchor. Kept: the three-artifact spine, Traceability Rules
  mandatory links, INT/DS declaration paragraphs, "'Fast' is not a requirement" /
  "constraints are facts" / no-orphan / IP-coverage behavioral lines, three no-exceptions
  lines, Common Rationalizations table, Quality Checklist, the letter/spirit epigraph (R4b:
  anchored to the envelope density rules directly above it). Session ruling: wave-4 batch-2
  ratified 2026-07-25.

## [v0.23.0] Description fields collapsed into the statement line across TR/C/IP/NFR blocks (T2, user-ruled)
- **Disposition:** revised per the wave-2 T2 ruling — the separate `**Description:**` paragraph field is deleted from all block templates (`references/ARTIFACT-TEMPLATES.md`) and field-definition tables (SKILL.md + reference); the ID line's statement IS the description
- **Tier failed:** artifact density (epic D4 extension): kinako's requirements.md 61k B / constraints-and-decisions.md 67k B were dominated by per-item field ceremony (label lines + a Description paragraph restating the statement), re-paid ~10× per feature
- **Content:** per-block forms compressed — TR: `**FR-XXX · MUST** — statement` + Criteria bullets + `**Deps:**` line (was Title/Source/Priority/Description/AC-checkboxes/Dependencies-list); C: type·severity·source on the statement line + one-line Impact (was 6 labeled fields); D: one-to-two-line Context + compact options table + ≤3-line Rationale + one-line Consequences (options/choice/ADR substance kept — `patterns-technical-decisions` owns the technique); IP: same collapse; NFR: statement line + Target/Measured/Applies-to lines (was 6 labeled fields + paragraph Requirement + paragraph Measurement Method). Summary tables kept and designated the **ID index** per `templates/artifact-format.md`. `references/TRACEABILITY-PATTERNS.md` pattern examples aligned to the statement-line form.
- **Consumers assessed:** plan producer (technical-analyst) + review-plan-artifacts (ARTIFACT-CHECKLISTS retargeted this wave) + review-feasibility (reads the artifacts; field-agnostic, checked — no edit needed) + downstream tasks/implement readers (consume IDs + statements, unaffected).

## [v0.23.0] Corrections landed in-wave (not strips)
- **Content:** (1) ARTIFACT-TEMPLATES' constraint Severity value set said `Hard` ("all constraints are hard boundaries by definition") while SKILL.md's field table says `blocking / significant / minor` — aligned to the SKILL (blocking/significant/minor), the pre-existing drift resolved toward the skill body; (2) `Constitution Alignment` field renamed `Governance alignment` (post-dissolution vocabulary, the plan-wave Constitution→Governance precedent), now optional-and-omitted when no principle applies.
