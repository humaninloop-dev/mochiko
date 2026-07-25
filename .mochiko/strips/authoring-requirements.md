# Strip notes — `skills/authoring-requirements/`

Entry formats: `strips/README.md`. Wave context: [v0.28.0] entries — skill-succinctness wave 4
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.23.0] entries — workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4;
ratified 2026-07-24).

## [v0.28.0] Reference restatements and homed mistake rows stripped (body 201 → 125, −38%, in-band)
- **Disposition:** deduped → verified pre-existing homes, each Read before landing (nothing
  written to any reference this wave): the RFC 2119
  table → one-line keyword enumeration + `references/RFC-2119-KEYWORDS.md` pointer (batch-1
  precedent), the Edge Cases fenced five-category block + the categories table → one-line
  category enumeration + `references/EDGE-CASES.md` pointer (the reference holds all five with
  patterns, examples, and documentation formats — richer; the "3-5 boundary conditions" rule
  kept) · **Common Mistakes deleted whole** (all 7 rows homed: tech leakage + implementation
  details → the kept in-file tech-agnostic pair + the reference's "Implementation in
  Requirements" mistake; unmeasurable → SC rules; missing keywords → the FR format block (every
  line shows a keyword) + the script's keyword check; technical-metrics-as-SC → SC rule 1
  ("No API metrics, database stats, or code coverage"); FR-vs-story → the When-NOT-to-Use
  dispatch bullet; edge-case scope creep → the "3-5" rule + checklist + EDGE-CASES' Prioritize
  step) · densified: FR numbering 4 rules → 1 line, FR tech-agnostic good/bad (2+2) → one
  contrast pair, SC good/bad (3+3) → one contrast pair, script-checks bullets → a parenthetical
  on the intro line (command kept)
- **Tier failed:** 1 throughout (verified homes) · n/a for the densifications
- **Content:** two tables, one fenced category block, four bullet lists, seven mistake
  subsections
- **Consumers assessed:** wave-open enumeration — 6 citing files (requirements-analyst, specify,
  authoring-technical-requirements, review-specifications, artifact-format, mochiko router);
  none links a section anchor. Kept: the **FR and SC fenced format blocks** — checked
  `templates/spec-template.md` as a dedup candidate and it is placeholder-only
  (`{{functional_requirements}}`), so these blocks are the only explicit homes of the line
  shapes; the Key Entities compact block ([v0.23.0] ruled form, untouchable); SC rules; entity-
  description rules; Quality Checklist. Session ruling: wave-4 batch-2 ratified 2026-07-25.

## [v0.23.0] Key-Entities example compressed to the one-line-purpose form
- **Disposition:** revised per the wave-2 form ruling (nothing relocated)
- **Tier failed:** artifact density: the multi-line per-entity block (purpose sentence + labeled Attributes/Relationships bullet lists) taught a shape that inflated every spec's Key Entities section; the conceptual content fits a compact form
- **Content:** the example block rewritten (purpose one line; attributes/relationships as inline `·`-separated concept lists); entity-description rules gain "one line" + the data-model.md downstream pointer. FR/SC/edge-case prescriptions were already one-line — unchanged. The envelope reference + density note added to the Overview.
- **Consumers assessed:** specify producer + review-specifications (density note added this wave) + `scripts/validate-requirements.py` (checked: FR/SC format checks are line-regex based, unaffected by the entity form).
