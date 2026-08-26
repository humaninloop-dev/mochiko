# Strip notes — `skills/authoring-requirements/`

Entry formats: `strips/README.md`. Wave context: [v0.28.0] entries — skill-succinctness wave 4
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.23.0] entries — workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4;
ratified 2026-07-24).

## [v0.91.0] Fix round — two When-NOT-to-Use carve-outs: "the design/plan track" → "the design track" (advisory)

- **Disposition:** superseded → "the design track" at both sites.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1). Raised as an **advisory** by the
  v0.91.0 wave audit: with the plan stage retired there is one downstream track, so the
  "design/plan" pair named a distinction that no longer exists.
- **Content (superseded fragments, verbatim — two sites):**

  1. `- **API endpoint specifications** - These belong to the design/plan track, not business requirements; keep concrete endpoint contracts out of FRs`
  2. `- **Data model design** - This belongs to the design/plan track; describe entities only conceptually here (see Key Entities), not as schemas`

- **Kept deliberately:** both carve-outs entire — concrete endpoint contracts stay **out of
  FRs**, and entities are described **only conceptually** here (Key Entities), never as schemas.
  The boundary this skill draws between business requirements and downstream design is unchanged;
  only the downstream track's name lost its retired half.
- **Budget:** body 4,423 → **4,413** (the shorter name shrinks it) against the 5,127 budget;
  description unchanged at 379 against 474. Both inside.
- **Consumers assessed:** `mochiko:authoring-user-stories` carried the identical "design/plan
  track" phrasing at two sites and was re-keyed in the same round;
  `mochiko:patterns-entity-modeling` and `mochiko:patterns-api-contracts` are the downstream
  homes these carve-outs point at — both untouched by this edit and still correctly named.

## [v0.91.0] Constraint-vs-posture rule: "a plan-time choice" → "a design-time choice" — plan-stage retirement D1

- **Disposition:** superseded → the same rule, naming a design-time choice.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1; wording ruled by the wave lead
  2026-08-26). The lowest-stakes site of the wave's residue batch — "plan-time" reads as ordinary
  English here rather than as a stage pointer — re-keyed for vocabulary consistency across the
  library on the lead's ruling, with the stakes noted honestly rather than overstated.
- **Content (superseded fragment, verbatim):**

  ```
  it names no product and passes the leakage check, yet freezes a plan-time choice downstream seats then read as ratified.
  ```

- **Kept deliberately:** the rule entire — **a constraint states a capability, never a posture**,
  its worked example ("The corpus is files on disk" vs "retrieval is locally computable"), the
  names-no-product/passes-the-leakage-check observation that makes the example bite, and the
  downstream-seats-read-it-as-ratified consequence.
- **Budget:** body 4,421 → **4,423** against the 5,127 budget; description unchanged at 379
  against 474. Both inside.
- **Consumers assessed:** none — this is a self-contained authoring rule in the spec layer with
  no cross-skill pointer. Specify's FR/SC layer is untouched by D3, which explicitly declined to
  move the technical layer into specify.

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
