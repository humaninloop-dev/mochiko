# Strip notes — `skills/patterns-api-contracts/`

Entry formats: `strips/README.md`. Wave context: [v0.27.0] entries — skill-succinctness wave 3
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.23.0] entries — workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` + the wave-2 rulings R1–R4/T3;
ratified 2026-07-24).

<!-- Wave context: the joint hook/delta build, wave 3 (v0.116.0) — contracts and the quickstart are
product baselines edited in place. Rulings for every [v0.116.0] entry below: the delta record
`.mochiko/brainstorms/delta-files-vs-direct-baseline-edits/record.md` D1-delta-files-retire (in place, no per-feature
copies), D2 (the lifecycle marker), D4 (the feature home loses its baseline copies) and D6(b) (epic
copies withdrawn) (`DECISIONS.md` 2026-09-24 row); census rows H5 (no `quickstart.md` in the spec
home) and Q1 (the product `quickstart.md`, bounded per entry),
`.mochiko/decisions/2026-09-29-census-table-ratified.md`, rows in
`.mochiko/brainstorms/hook-enforcement-field-review/reports/w2-census-table.md` (`DECISIONS.md`
2026-09-29 row); the wave-3 lead's ruling on P8's B2. Pre-edit verbatim text:
`git show 5558fd7:plugins/mochiko/skills/patterns-api-contracts/SKILL.md`. -->

## [v0.118.0] Traceability table FR and US cells joined and qualified

- **Disposition:** superseded → `` `<spec-slug>` FR-001-<slug> `` / `` `<spec-slug>` US-1-<slug> `` and the `FR-004` / `US-4` row the same.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/human-readable-ids/record.md` D2, D9, D19 and D21's scope clause — durable IDs are written joined, every mention including the definition, with example IDs and template definition lines joined; wave plan `.mochiko/brainstorms/human-readable-ids/wave2-plugin-minting.md`; the lead's Q-B ruling (`US#n` → `US-<n>-…`, on D9); D11 (S4) for the owner)
- **Content (superseded):** "| /auth/login | POST | FR-001 | US#1 | User login |" and "| /users/me | GET | FR-004 | US#4 | Get current user |".
- **Kept deliberately:** the table shape and the endpoint columns.

## [v0.116.0] OPENAPI-TEMPLATE header — the per-feature placeholders and "Copy and customize" re-keyed

- **Disposition:** superseded → "Seed a missing `api.yaml` from it, the product's or a spec's; a
  run edits an existing file in place", with the placeholders `{Product Name}` and `{product}` (for
  a spec's file, its name and slug).
- **Tier failed:** n/a — supersession by ruling (delta D1, D4 and D6(b); the spec-contracts home
  stays live).
- **Content (superseded text, verbatim):**

  ```
  # Complete template for RESTful API specifications. Copy and customize for your API.
  # Replace placeholders: {Feature Name}, {feature_id}, {description}
    title: "{Feature Name} API"
      API contracts for {feature_id}.
  ```

- **Kept deliberately:** the template body, every section of it, and the `x-integration` example.
- **Consumers assessed:** `scripts/validate-openapi.py` reads no placeholder and is unchanged.

## [v0.116.0] Quality Checklist quickstart line — "(≤ 150 lines …)" and "authored iff a real integration surface exists" re-keyed per entry

- **Disposition:** superseded → "A quickstart entry written iff the run adds or changes a real
  integration surface (within its per-entry bound, cites the contract, never re-documents it);
  otherwise its null path recorded in the run's sufficiency report".
- **Tier failed:** n/a — supersession by ruling (delta D1/D4/D6(b); census H5 and Q1; the lead's
  ruling on P8's B2).
- **Content (superseded text, verbatim):** `- [ ] Quickstart authored iff a real integration surface
  exists (≤ 150 lines, cites the contract, never re-documents it); otherwise its null path recorded
  in the run's sufficiency report`
- **Kept deliberately:** the iff condition, cite-never-re-document, and the null path in the
  sufficiency report (the v0.91.0 home).
- **Consumers assessed:** `review-plan-artifacts`'s quickstart checklist re-keyed to the same
  per-entry check in the same wave.

## [v0.116.0] The Quickstart section — the v0.23.0 T3 whole-document cap and the per-feature framing superseded

- **Disposition:** superseded →
  - the heading "— the product's integration guide";
  - "a run … writes or amends a quickstart entry **only when its work adds or changes a real
    integration surface**", with the null path in the sufficiency report and never a stub entry;
  - "Each entry is **dense and bounded** as the product home prints it". The bound is read from
    the home and never restated here.
- **Tier failed:** n/a — supersession by ruling:
  - delta D1/D4/D6(b) withdraw the per-feature file the T3 cap bounded;
  - census H5 and Q1 give the product `quickstart.md` its bound per entry, as the home declares it;
  - the lead's ruling on P8's B2.
- **Content (superseded text, verbatim):**

  ```
  ## The Quickstart (`quickstart.md`) — conditional, capped

  `quickstart.md` is the human-facing integration guide over the finished contract — and it
  is **conditional**: author it **only when the feature has a real integration surface**
  (external consumers of the API, an external system wrapped via `x-integration`, or a
  non-trivial auth sequence a caller must follow). A feature whose endpoints only serve its
  own UI over standard auth does not need one — record the null path as one line in the run's
  **sufficiency report** ("not applicable — no external integration surface"), never a
  stub file.

  When authored, it is **capped and dense** (deliverable envelope,
  `templates/artifact-format.md`): target ≤ 150 lines —
  ```

- **Kept deliberately:**
  - the integration-surface condition and its three triggers;
  - the null path, one line in the sufficiency report;
  - the four dense sections, verbatim: common flows, auth sequence, error handling, and the
    external-system overview;
  - cite, never re-document;
  - the deliverable-envelope pointer.
  The T3 intent (no unconditional, sprawling quickstart) survives as the condition plus the bound
  per entry.
- **Consumers assessed:**
  - `review-plan-artifacts`'s quickstart checklist, re-keyed in the same wave;
  - `agents/technical-analyst.md:60` names the conditional quickstart without the cap or the
    presence check, so it stands;
  - `templates/artifact-format.md:67` states generic conditional authoring and stands.

## [v0.116.0] Where the Artifact Lives — "whichever scope the run owns" and the spec/epic `quickstart.md` homes superseded

- **Disposition:** superseded →
  - contracts live in the product's `contracts/`, edited in place, or in a spec's;
  - `quickstart.md` is a product baseline, `.mochiko/product/quickstart.md`, edited in place, and
    no other home carries it.
  The same edit adds one pure-addition pointer line: the `**Lifecycle:**` marker and
  `x-lifecycle`, pointing to `impl.baseline-entry-grammar` (S8's Q8).
- **Tier failed:** n/a — supersession by ruling (delta D1/D4/D6(b); census H5; the
  feature-contracts and epic-contracts homes are withdrawn in the log).
- **Content (superseded text, verbatim):** `` `api.yaml` and the per-endpoint contract files are
  declared files in the `contracts/` home under
  whichever scope the run owns — the spec, the feature, the epic or the product baseline — `` … ``
  `quickstart.md` is a declared file one level up, in the spec, the epic or the product home; the
  feature home does not carry it. ``
- **Kept deliberately:**
  - each contracts home's bound as the interface;
  - the obligations to render `mochiko-cli home` before the first write and never copy a sibling;
  - the refused-write paragraph.
- **Budget:** the body goes from 11,877 to 12,278 against a budget of 13,412, and the description
  stays at 486. Both are inside budget.
- **Consumers assessed:** `patterns-entity-modeling` and `patterns-technical-decisions` take the same
  pointer in the same wave.

## [v0.91.0] Ladder blockquote and two "independent plan reviewer" pointers re-keyed to the design phase — plan-stage retirement D1/D5

- **Disposition:** superseded → the design ladder / design-phase package / design-phase proposal
  in the blockquote; "the independent design-phase reviewer" in both self-check disclaimers.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1-plan-command-retires carry-over (the ladder governs
  what the design phase authors) and D5 (the two graders re-scope to the design-phase output)).
  Wording follows the wave lead's 2026-08-26 rulings for the parallel sites — "the plan proposal"
  → "the design-phase proposal" (ruled for `patterns-entity-modeling:12`, whose blockquote is
  this one's twin) and the design-phase re-scope of the review pair.
- **Found by sweep, in an already-allocated file:** the lead's item-6 allocation named :159/:198
  of this file. These three surfaced on the exhaustive final sweep — same file, same ruling
  cluster. Fixed here on the precedent the lead approved for the `authoring-architecture-store`
  in-file finds, rather than left to contradict the sites already re-keyed.
- **Content (superseded fragments, verbatim — three sites):**

  1. Overview blockquote:

     ```
     > **Endpoint and contract necessity answer the plan ladder** (`mochiko:patterns-plan-minimalism`) before they enter the plan — the simplest-execution stops are disclosed in the proposal; this skill designs the contracts that survive it.
     ```
  2. Self-check section: `that substantive review belongs to the independent plan reviewer, not this script.`
  3. `scripts/validate-openapi.py` module docstring: `substantive review is model judgment, owned by the independent plan reviewer.`

- **Kept deliberately:** the ladder obligation (necessity answered **before** entry, stops
  disclosed not re-derived here, this skill designs only the survivors) and — load-bearing — the
  whole deterministic-vs-substantive split: the script checks format and convention and
  explicitly does **not** judge whether the endpoints, schemas, or failure modes are the *right*
  ones. That boundary is what keeps the script advisory rather than kernel-class; only the name
  of the seat holding the substantive judgment changed.
- **Verification:** `python3 -m py_compile` on the edited script passes (docstring-only change).
- **Budget:** body **11,031** against the 13,412 budget; description unchanged at 486 against
  608. Both inside. (This supersedes the 10,992 figure recorded in the entry below — that was
  the count after the quickstart re-key and before these three sites.) `scripts/` files are
  budget-exempt.
- **Consumers assessed:** `mochiko:patterns-entity-modeling`'s twin blockquote was re-keyed
  identically in this wave; `mochiko:review-plan-artifacts` (the "independent reviewer" these
  pointers name) was re-scoped to the design-phase package earlier in the wave, so the pointers
  now match their target's own description.
- **Blockquote re-keyed again at the fix round (V1 ripple):** the wording this entry landed —
  `the simplest-execution stops are disclosed in the design-phase proposal` — named an artifact
  the fix round's V1 ruling then retired (D4 lists plan's proposal approval among the dead
  gates). Now reads "disclosed by the design phase as it authors", matching the ladder skill's
  surviving grammar and its twin in `patterns-entity-modeling`. Body 11,031 → **11,036** against
  the 13,412 budget; description unchanged at 486. The ladder obligation is untouched in both
  passes.

## [v0.91.0] Quickstart null-path home re-pointed from `plan.md` to the sufficiency report — plan-stage retirement D4

- **Disposition:** superseded → the null path is recorded in **the run's sufficiency report**,
  the durable assessment record that replaces `plan.md` as the run's summary surface. (Landed
  wording is the wave lead's, ruled 2026-08-26; an earlier pass of this edit read "the
  sufficiency report" without the possessive and was aligned to the ruling before the audit.)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` **D4-planless-homes-gates**: "`plan.md` (the summary
  artifact) dies — no restatement artifact; the sufficiency verdict lands as a report under
  `templates/report-format.md` in the feature dir and is the durable assessment record — it
  additionally carries the `quickstart.md` null-path record"). Scope for this file was opened by
  the wave lead's second extension ruling of 2026-08-26.
- **Why this could not be left:** `plan.md` was deleted from the tree in this same wave, so both
  sites pointed at a file that no longer exists — and the matching check in
  `review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md` had already been re-pointed to the
  sufficiency report, leaving producer and grader contradicting each other about where the null
  path lives.
- **Content (superseded fragments, verbatim — two sites):**

  1. The Quickstart section:

     ```
     own UI over standard auth does not need one — record the null path as one line in
     `plan.md`'s artifact table ("not applicable — no external integration surface"), never a
     stub file.
     ```
  2. Quality Checklist:

     ```
     - [ ] Quickstart authored iff a real integration surface exists (≤ 150 lines, cites the contract, never re-documents it); otherwise its null path recorded in `plan.md`
     ```

- **Kept deliberately:** the whole conditionality rule — quickstart authored **only** on a real
  integration surface (external consumers, an `x-integration`-wrapped system, or a non-trivial
  auth sequence), the never-a-stub-file rule, the exact null-path wording ("not applicable — no
  external integration surface"), and the ≤150-line cap. Only the artifact that holds the record
  changed.
- **Budget:** body **10,992** against the 13,412 budget; description untouched at 486 against
  608. Both inside. (Figure taken after the ruled-wording alignment, not before it.)
- **Consumers assessed:** `review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md`'s
  Conditionality-honored check was re-pointed to the sufficiency report earlier in this same
  wave — producer and grader now agree again. `implement.md` (P1's rewrite) owns the sufficiency
  report's contents.

## [v0.64.0] Guardrails cut — When-to-Use + Quick-Reference example removed, tables/contract kept; slim description

- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line — When-to-Use bullets restate the description; the pagination Quick-Reference is a worked example whose rule lives in the reference).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md` 2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed — section-level inventory; body 11,175 → 10,729 chars, −446, −4%; description 773 → 486 chars):**
  - **Removed whole:** `## When to Use` — the five-bullet list ("Designing new API endpoints — mapping user actions to HTTP methods and paths" · "Creating OpenAPI specifications (`contracts/` artifacts) with request/response schemas" · "Documenting error responses for an API" · "Documenting integration boundaries for endpoints that wrap external systems" · "Integrating with existing API patterns (brownfield)"). Restates the description; the brownfield firing survives in `## Brownfield Considerations`, the integration-boundary firing in `## Integration Boundaries`.
  - **Removed whole:** the `### Quick Reference` example under `## List Endpoints` — the fenced one-liner `GET /api/users?page=1&limit=20&role=admin&sort=-createdAt`. A worked pagination example; the pagination/filtering/sorting rules live in `references/PAGINATION-PATTERNS.md` (the surviving pointer immediately above it).
  - Old description verbatim: "This skill MUST be invoked when designing the API-contract layer of a feature — mapping user actions to REST endpoints (HTTP method, idempotency, resource naming), defining request/response schemas (mapping conceptual data-model types to OpenAPI types), designing error responses and list pagination, authoring per-endpoint integration boundaries for endpoints that wrap external systems, and assembling the OpenAPI specification at `contracts/api.yaml`. SHOULD also invoke when the design work involves an \"endpoint\", \"API contract\", \"request/response schema\", \"OpenAPI spec\", \"REST API design\", an \"HTTP\" method or status code, or an \"integration boundary\" / \"x-integration\". Produces a traceable OpenAPI contract with documented errors and external-system failure modes."
  - Verbatim removed text survives in: git history of the SKILL.md (pre-v0.64.0); archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (the guardrails keep-set):** `## Overview`, `## When NOT to Use`, `## Endpoint Mapping` (User-Action decision table + Resource Naming Conventions), `## Schema Definition` + the `### Type Mapping from Data Model` table (the entity-modeling↔contracts boundary vocabulary — v0.27.0-noted unique home), `## Error Response Design` + `## List Endpoints` (pointers), `## Brownfield Considerations`, `## Integration Boundaries` (the `x-integration` field-reference + criticality tables — the only home of the field rules, per v0.27.0; the "optimistic integration maps are incomplete" floor), `## OpenAPI Structure`, `## Traceability` (the ID-index table), `## The Quickstart` (the v0.23.0 canonical conditional-quickstart home), `## Validation`, `## Quality Checklist`, `## Common Mistakes`.
- **Protected-content reconciliation (MANDATORY):** the v0.27.0 KEPT set (Type Mapping table, x-integration field-reference + criticality tables, the Quickstart section, the traceability ID index, the pagination quick-ref one-liner) reconciled — all survive **except** the pagination Quick-Reference, which v0.27.0 listed among "Kept: … pagination quick-ref (one line)". That one line is **superseded by this ruling** and recorded above (its rule survives in `references/PAGINATION-PATTERNS.md`). Every other v0.27.0-KEPT element is intact. No other protected line dropped.
- **Consumers assessed:** `agents/technical-analyst.md`, `skills/authoring-technical-requirements/SKILL.md` (+ `references/ARTIFACT-TEMPLATES.md`, `references/TRACEABILITY-PATTERNS.md`), `skills/review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md`, `skills/patterns-entity-modeling/SKILL.md`, `skills/patterns-system-design/SKILL.md`, `skills/patterns-technical-decisions/references/EVALUATION-MATRIX.md`, router `skills/mochiko/SKILL.md`, `templates/artifact-format.md`, `templates/plan-template.md` — all reference the skill by name; none links a removed section anchor. The OpenAPI contract shape reviewers grade against is intact.

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
