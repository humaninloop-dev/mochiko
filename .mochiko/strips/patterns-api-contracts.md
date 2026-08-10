# Strip notes — `skills/patterns-api-contracts/`

Entry formats: `strips/README.md`. Wave context: [v0.27.0] entries — skill-succinctness wave 3
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.23.0] entries — workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` + the wave-2 rulings R1–R4/T3;
ratified 2026-07-24).

## [v0.27.0] Second-copy yaml blocks, zero-consumer doc format, and homed mistake rows stripped (body 361 → 215, −40%, in-band)
- **Disposition:** relocated/deduped → `references/OPENAPI-TEMPLATE.yaml` (879-line complete
  copy-ready template, Read and confirmed to hold each): the Minimal-Structure skeleton, the
  LoginRequest schema example, and — ratified contested — the 26-line worked `x-integration`
  yaml (the template carries an x-integration example on a live operation — two failure modes
  vs the deleted block's three; the 429 rate-limit case did not travel, though the surviving
  in-body field-reference table names rate-limit among the failure kinds; the field-reference
  and criticality tables, the only home of the field *rules*, stay in-body) · relocated →
  `references/ERROR-PATTERNS.md` (Read: all 8 statuses held richer with when-to-use + example
  codes): the error Quick-Reference table · deleted (Tier 2): the Endpoint Documentation Format
  block — a second per-endpoint markdown format whose marker field (`Source Requirements`)
  appears nowhere else in the plugin (zero consumers; the contract artifact is `api.yaml`, the
  ID index is the traceability table) · deleted (Tier 1, homes verified): four Common-Mistakes
  rows — missing-error-responses (checklist + Error Response Design), generic-error-codes
  (ERROR-PATTERNS Naming Rules), optimistic-boundaries (the Integration Boundaries section
  states it twice), skipping-brownfield (Brownfield Considerations) · densified (form-only):
  User-Action + Method-Selection tables merged into one 8-row table with an Idempotent column
  (all rows preserved, PUT/PATCH split kept), four surviving mistakes → one 3-column table,
  When-to-Use 8 → 5 bullets
- **Tier failed:** 1 (relocations/dedups — every block had a verified richer home) · 2 (the
  doc format — zero consumers, no behavior lost) · n/a for the merges
- **Content:** three yaml blocks, the doc-format block, four mistake subsections, the
  8-row status table; nothing written to `templates/` — dedups run against pre-existing
  reference content, D4's destination ban not engaged
- **Consumers assessed:** wave-open enumeration — 11 citing files, none references the stripped
  blocks or a section anchor; `Source Requirements` grep confirmed empty outside this file.
  Kept: Type Mapping table (the entity-modeling↔contracts boundary vocabulary, unique home),
  field-reference + criticality tables (only home of the x-integration rules), the Quickstart
  section (v0.23.0 canonical home, untouched), traceability ID index, pagination quick-ref
  (one line). Session ruling: wave-3 batch-2 ratified 2026-07-25 (A8 ruled strip)

## [v0.23.0] quickstart.md becomes conditional + capped (T3, user-ruled)
- **Disposition:** ruled change, not a strip — recorded here because it changes the authored artifact set: `quickstart.md` (kinako: 17.5k B, authored unconditionally) is now authored **only when the feature has a real external-integration surface**, capped ≤ 150 lines, citing the contract instead of re-documenting it; the null path is one line in `plan.md`'s artifact table, never a stub file
- **Content:** the SKILL.md gains the Quickstart section (this skill owned the artifact per cross-references but never defined it — the ownership gap closed with the compact definition); endpoint↔FR/US traceability table designated the contract's ID index. Ripples: `commands/plan.md` (goal / deliverables / Phase-2 produce / done-condition read "when applicable"), `templates/plan-template.md` artifact row, `agents/technical-analyst.md` deliverable #6, review-plan-artifacts' quickstart checklist (conditionality check added).
- **Consumers assessed:** plan's done-condition (now reads quickstart-or-null-path) + review-plan-artifacts (checklist retargeted) + slices-template's Graduation-contract artifact-layout line (lists quickstart at the feature root among accumulating artifacts — an absent conditional artifact simply never accumulates; no edit needed).
