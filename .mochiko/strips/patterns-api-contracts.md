# Strip notes — `skills/patterns-api-contracts/`

Entry formats: `strips/README.md`. Wave context: [v0.27.0] entries — skill-succinctness wave 3
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.23.0] entries — workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` + the wave-2 rulings R1–R4/T3;
ratified 2026-07-24).

## [v0.77.0] `The Quickstart` guidance sourced into a schema with an AUTHORED skeleton (D3 later-ratchet; I1)
- **Disposition:** superseded → `plugins/mochiko/schemas/quickstart.yaml` + `mochiko-cli`
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance **D3 later-ratchet** + user ruling 2026-08-16 (recorded at the v0.76.0 landing); record `.mochiko/brainstorms/schema-based-template-guidance/record.md` D3; `DECISIONS.md` "Template-schema ratchet" row (landed at v0.77.0))
- **Content (superseded, verbatim — the `## The Quickstart` guidance, SKILL.md before the phase-2 body edit):**

````markdown
## The Quickstart (`quickstart.md`) — conditional, capped

`quickstart.md` is the human-facing integration guide over the finished contract — and it
is **conditional**: author it **only when the feature has a real integration surface**
(external consumers of the API, an external system wrapped via `x-integration`, or a
non-trivial auth sequence a caller must follow). A feature whose endpoints only serve its
own UI over standard auth does not need one — record the null path as one line in
`plan.md`'s artifact table ("not applicable — no external integration surface"), never a
stub file.

When authored, it is **capped and dense** (deliverable envelope,
`templates/artifact-format.md`): target ≤ 150 lines —

- **Common flows** — one runnable example per primary flow (request + expected response,
  trimmed to the fields that matter); cite endpoint + schema by name, never re-document
  what `api.yaml` already defines.
- **Auth sequence** — the steps a caller actually performs, compact.
- **Error handling** — the pattern and the top recoverable cases as a table; cite
  ERROR-PATTERNS conventions, don't restate them.
- **External-system overview** — one line per `x-integration` system: name, criticality,
  what the caller observes when it degrades.
````
- **Authored, not lifted (DISCLOSED — I1, under D4/D7):** the source is **prose guidance with no fenced template**, so `quickstart.yaml`'s `skeleton` and its per-section field shape (`Common Flows` / `Auth Sequence` / `Error Handling` / `External-System Overview`, with example fragments) are **NET-NEW authored** under D4/D7 authority — graded by the fidelity validator, not lifted. What IS lifted from the guidance into `overview` / `contract`: the conditional-authoring rule (real integration surface only; null path recorded in `plan.md`, never a stub file), the ≤ 150-line cap, the four content areas, and cite-by-name-never-re-document-`api.yaml`.
- **Kept deliberately:** every rule of the guidance survives (conditional trigger · cap · four content areas · cite-don't-restate). `contracts-api` is **NOT** converted this wave (I1 — its true source `references/OPENAPI-TEMPLATE.yaml` is already raw-readable YAML, does not fit the schema struct); no pointer touches to it. Net-new per-section `check` lines authored under D7. Nothing from the guidance dropped.
- **Phase note:** schema authored at PHASE 1; the atomic SKILL-body edit (add the two-arm pointer at `## The Quickstart`) is the PHASE 2 edit (same P2 seat), gated on V1 fidelity PASS. `contracts-api`: no touch.
- **Consumers assessed:** `patterns-api-contracts/SKILL.md` (owns quickstart — body edit phase 2) · `review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md` Quickstart checklist (additive structure-presence re-key phase 2) · `x-integration` / `api.yaml` references unchanged (contracts-api excluded, I1).

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
