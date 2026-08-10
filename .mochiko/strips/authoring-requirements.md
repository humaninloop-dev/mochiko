# Strip notes — `skills/authoring-requirements/`

Entry formats: `strips/README.md`. Wave context: [v0.28.0] entries — skill-succinctness wave 4
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.23.0] entries — workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4;
ratified 2026-07-24).

## [v0.63.0] Guardrails cut — body deletions + slim description (benchmark verdict)
- **Disposition:** superseded → benchmark-ruled guardrails body + slim description (`.mochiko/benchmarks/guardrails-vs-detail/variants/`)
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark verdict — `DECISIONS.md` 2026-08-10 benchmark-verdict row; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`)
- **Content (faithfully compressed).** Body 5,071 → 4,101 chars (−970, −19%). Description 556 → 379 chars. Sections removed or shortened:
  - **## When to Use** (the six-bullet list) — removed; When NOT to Use kept.
  - **### Writing Technology-Agnostic Requirements** — the "notify users … never send email via SendGrid" WHAT/WHY-not-HOW contrast pair.
  - **## Key Entities** worked example including **### RecurringPattern** — the fenced RecurringPattern example block; the ## Key Entities (Optional) intro and ### Entity Description Rules are kept.
  - The Success-Criteria technical-metric contrast line ("Users complete the workflow … never API responds in under 200ms").
  - Old description verbatim: "This skill MUST be invoked when authoring the functional-requirements layer of a feature specification — writing technology-agnostic functional requirements in FR-XXX format with RFC 2119 keywords (MUST/SHOULD/MAY), identifying edge cases, and defining measurable success criteria in SC-XXX format. SHOULD also invoke when the authoring work involves "functional requirements", "FR-", "success criteria", "SC-", "RFC 2119", "MUST SHOULD MAY", or "edge cases". Produces technology-agnostic requirements in FR-XXX format with measurable success criteria."
  - Verbatim removed text survives in three places: (a) git history of the original `plugins/mochiko/skills/authoring-requirements/SKILL.md`; (b) the before/after pair in this tree — `.mochiko/benchmarks/guardrails-vs-detail/variants/body/authoring-requirements/SKILL.md` (after) and the pre-edit original (before, in git); (c) archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (the guardrails keep-set):** goal/output contract (Overview + the FR and SC fenced format blocks — per `[v0.28.0]` the only explicit homes of the line shapes); the `references/RFC-2119-KEYWORDS.md`, `references/EDGE-CASES.md`, and `templates/artifact-format.md` pointers; the PM-frame boundary; the SC rules; the Entity Description Rules (the [v0.23.0] ruled compact form); the Validation Script command; the Quality Checklist. No floor line added to this skill (not a floor-line home).
- **Protected-content reconciliation.**
  - `[v0.23.0]` / `[v0.28.0]` Key-Entities compact form (the "untouchable" ruled form): the ## Key Entities (Optional) intro and ### Entity Description Rules — the compact-form *rules* — survive; the cut removes only the fenced RecurringPattern *worked example* that illustrated the form. The example removal is recorded here as superseded-by-this-ruling; the ruled compact form itself is intact.
  - `[v0.28.0]` KEPT set (the FR/SC fenced format blocks, SC rules, entity-description rules, Quality Checklist) — all survive the cut. Intact.
  No prior KEPT / protected line is silently dropped.
- **Consumers assessed:** no command references this skill (grep `plugins/mochiko/commands/` clean). `agents/requirements-analyst.md` declares it in `skills:`; the kept goal/format-blocks/`references` pointers leave that composition intact.

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
