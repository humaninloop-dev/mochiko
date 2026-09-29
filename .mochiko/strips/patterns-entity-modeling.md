# Strip notes — `skills/patterns-entity-modeling/`

Entry formats: `strips/README.md`. Wave context: [v0.27.0] entries — skill-succinctness wave 3
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.23.0] entries — workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4;
ratified 2026-07-24).

<!-- Wave context: the joint hook/delta build, wave 3 (v0.116.0) — `data-model.md` is a product
baseline edited in place, one `###` entry per entity. Rulings for every [v0.116.0] entry below:
the delta record `.mochiko/brainstorms/delta-files-vs-direct-baseline-edits/record.md` D1 (baselines
edited in place, no per-feature copies) and D2 (a lifecycle marker per entry, keyed by the run's
owner) (`DECISIONS.md` 2026-09-24 row); census row S1 (the product `data-model.md`, one `###`
entry per entity) and H5 (the spec home drops `data-model.md`),
`.mochiko/decisions/2026-09-29-census-table-ratified.md`, rows in
`.mochiko/brainstorms/hook-enforcement-field-review/reports/w2-census-table.md` (`DECISIONS.md`
2026-09-29 row); the wave-3 lead's rulings on S8's Q2 (option A, the linter regex) and on P8's C1
(the linter docstring and message). The v0.27.0 KEPT protected core — the template, the brownfield
status table and the validation-script paragraph — and the v0.64.0 checklist keep-set leave only
as the supersessions below. Pre-edit verbatim text:
`git show 5558fd7:plugins/mochiko/skills/patterns-entity-modeling/SKILL.md`. -->

## [v0.116.0] validate-model.py docstring, format comments and "no entities" message — `## Entity:` and the status tags re-keyed to `### Entity:`

- **Disposition:** superseded → the docstring lists `### Entity: EntityName` and the unchanged
  `### EntityName (simpler format)`; the two comments naming the format read `### Entity:`; the
  message reads `Expected '### Entity: Name' format.`
- **Tier failed:** n/a — supersession by ruling (census S1; delta D2 — the marker replaces the
  status tags; the lead's ruling on P8's C1).
- **Content (superseded text, verbatim):**

  ```
      - ## Entity: EntityName
      - ## Entity: EntityName [NEW]
      - ## Entity: EntityName [EXTENDS EXISTING]
      # Pattern for ## Entity: Name [STATUS] format
      # If no entities found with ## Entity: format, try summary table
          issues.append("No entities found in document. Expected '## Entity: Name' format.")
  ```

- **Kept deliberately:** the script's check logic, untouched, including the optional `[STATUS]`
  capture and the status-keyed skips: an entity with no tag reads `NEW`, so they never fire on the
  new template. Pruning them is outside this row.
- **Consumers assessed:** the SKILL.md Validation Script paragraph and the template, both re-keyed
  in the same wave.

## [v0.116.0] validate-model.py entity regex — `^##\s+Entity:` superseded by `^###\s+Entity:`

- **Disposition:** superseded → `entity_pattern = r"^###\s+Entity:\s+(\w+)(?:\s+\[([^\]]+)\])?"`.
  Only the heading level changes. The self-check now reports a `##` entity, which the gate reads
  as section text rather than an entry.
- **Tier failed:** n/a — supersession by ruling (census S1; the lead's ruling on S8's Q2).
- **Content (superseded text, verbatim):** `entity_pattern = r"^##\s+Entity:\s+(\w+)(?:\s+\[([^\]]+)\])?"`
- **Proof, run on scratch copies before the regex landed:**
  - new template × new regex: `['User', 'Session']`, 8/8, exit 0.
  - new template × old regex: `['User', 'User']`, 7/8, with `required_attributes` failing.
  - old template × new regex: `['User', 'User', 'User', 'Session']` through the summary fallback,
    7/8, exit 1.
  - old template × old regex: `['User', 'Session']`, 8/8.
- **Kept deliberately:** everything else in the script.
- **Consumers assessed:** only this skill ships or runs the script.

## [v0.116.0] RELATIONSHIP-PATTERNS join-entity example — `## Entity: ProjectMember [NEW]` re-levelled

- **Disposition:** superseded → `### Entity: ProjectMember`, then `**Lifecycle:** proposed (<key>)`,
  with `#### Attributes` and `#### Constraints`.
- **Tier failed:** n/a — supersession by ruling (census S1; delta D2).
- **Content (superseded text, verbatim):** `## Entity: ProjectMember [NEW]` · `### Attributes` ·
  `### Constraints`
- **Kept deliberately:** the example's attribute table and its uniqueness constraint.
- **Consumers assessed:** none outside this skill. `## Entity Relationships` (`:156`) is a section
  heading, not an entry, and it stands.

## [v0.116.0] Quality Checklist — "Brownfield status indicated for each entity" re-keyed to the lifecycle marker

- **Disposition:** superseded → `- [ ] Each entity written or amended carries the run's lifecycle marker`.
- **Tier failed:** n/a — supersession by ruling (delta D1/D2; census S1). The line is a member of
  the v0.64.0 keep-set.
- **Content (superseded text, verbatim):** `- [ ] Brownfield status indicated for each entity`
- **Kept deliberately:** the checklist's other nine lines.
- **Consumers assessed:** none outside this skill.

## [v0.116.0] Brownfield Entity Status table — the four tags superseded by the lifecycle marker

- **Disposition:** superseded → `### Brownfield Entities`, which maps the old tags onto the new
  model:
  - `[NEW]` becomes a new entry;
  - `[EXTENDS EXISTING]` becomes the existing entry amended;
  - `[REUSES EXISTING]` becomes no write;
  - the entry's lifecycle marker carries which it is;
  - `[RENAMED]`'s collision rule survives as one line: a new entity whose name would collide takes
    a new name, and its entry records the reason.
- **Tier failed:** n/a — supersession by ruling (delta D1/D2). The table was in the v0.27.0 KEPT
  core.
- **Content (superseded text, verbatim):**

  ```
  ### Brownfield Entity Status

  When modeling in brownfield projects:

  | Status | Meaning | Action |
  |--------|---------|--------|
  | `[NEW]` | Entity doesn't exist | Create full definition |
  | `[EXTENDS EXISTING]` | Adding to existing entity | Document new fields only |
  | `[REUSES EXISTING]` | Using existing as-is | Reference only |
  | `[RENAMED]` | Avoiding collision | Document new name + reason |
  ```

- **Kept deliberately:** each status's meaning, re-expressed as what the run writes, and the
  collision-rename rule with its recorded reason.
- **Consumers assessed:** `brownfield-integration` owns collision detection. It names no tag from
  this table and is unchanged.

## [v0.116.0] Template Entity Summary — the Status column superseded

- **Disposition:** superseded → `| Entity | Attributes | Relationships |`. The example counts now
  agree with the entity tables shown.
- **Tier failed:** n/a — supersession by ruling (delta D2 — the status is each entry's
  `**Lifecycle:**` field).
- **Content (superseded text, verbatim):**

  ```
  | Entity | Attributes | Relationships | Status |
  |--------|------------|---------------|--------|
  | User | 8 | 3 | [EXTENDS EXISTING] |
  | Session | 5 | 1 | [NEW] |
  ```

- **Kept deliberately:** the summary table itself.
- **Consumers assessed:** the linter's summary-table fallback runs only when no entity heading is
  found, and it is unchanged.

## [v0.116.0] Template entity tags and "Existing Attributes (Not Modified)" — superseded by one full entry per entity

- **Disposition:** superseded → the User entry carries its whole attribute table (`id`, `email`,
  `passwordHash`, `lastLoginAt`, `createdAt`, `updatedAt`) under a `**Lifecycle:**` line. A
  product entry holds the entity; the pinned-base diff holds the change. Its lead line no longer
  narrates the change ("A registered account holder").
- **Tier failed:** n/a — supersession by ruling (delta D1 — prior text is never kept beside the
  new; D2).
- **Content (superseded text, verbatim):**

  ```
  ## Entity: User [EXTENDS EXISTING]

  Existing entity extended with authentication fields. **Traceability:** FR-001, FR-002, US#1
  ...
  ### Existing Attributes (Not Modified)

  | Attribute | Type | Sensitivity | Description |
  |-----------|------|-------------|-------------|
  | id | UUID | Internal | Existing primary key |
  | email | Email | Confidential | Existing email field |
  ...
  ## Entity: Session [NEW]
  ```

- **Kept deliberately:** every attribute, its sensitivity class, both Sensitivity Details rows,
  the traceability lines and the Session relationship.
- **Consumers assessed:** the linter's `EXTENDS EXISTING` skip no longer applies. User is checked
  for id and audit fields and passes, per the proof above.

## [v0.116.0] Template re-levelled — `# Data Model: {feature_id}` and `## Entity:` superseded by `# Data Model` and `### Entity:` under `## Entities`

- **Disposition:** superseded → the title `# Data Model`, a `## Entities` section, one
  `### Entity: <Name>` entry per entity with its first line `**Lifecycle:** proposed (<key>)`, and
  the entity's tables under `####`.
- **Tier failed:** n/a — supersession by ruling (census S1 — one `###` entry per entity, bounded
  per entry; delta D1 — one product file, not a per-feature copy; delta D2).
- **Content (superseded text, verbatim):** `# Data Model: {feature_id}` · `## Entity: User [EXTENDS
  EXISTING]` · `## Entity: Session [NEW]` · `### Attributes` · `### Relationships` ·
  `### Sensitivity Details`
- **Kept deliberately:** the v0.27.0-KEPT template's substance: the summary, the handling defaults
  stated once, the per-attribute Sensitivity column, the Details rows, and the trailing
  Relationships / State Machines / Validation Rules sections. Only heading levels, the marker line
  and the tags moved.
- **Consumers assessed:** `patterns-api-contracts` reads entity names, not heading levels.
  `review-plan-artifacts`'s data-model checklist grades content, not levels. Both are unchanged.

## [v0.116.0] Validation Script path — `.mochiko/specs/<feature>/data-model.md` re-pointed

- **Disposition:** superseded → `python scripts/validate-model.py .mochiko/product/data-model.md`.
- **Tier failed:** n/a — supersession by ruling (delta D1; census H5).
- **Content (superseded text, verbatim):** `python scripts/validate-model.py .mochiko/specs/<feature>/data-model.md`
- **Kept deliberately:** the v0.27.0-KEPT validation-script scope paragraph, verbatim.
- **Consumers assessed:** none.

## [v0.116.0] Where the Artifact Lives — "whichever home the run owns — the spec, the feature, the epic or the product baseline" superseded

- **Disposition:** superseded → "`data-model.md` is a product baseline,
  `.mochiko/product/data-model.md`, edited in place: a run writes or amends its entities there, and
  the change is the diff against the run's pinned base." The same edit adds one pure-addition
  pointer line to `impl.baseline-entry-grammar` (S8's Q8).
- **Tier failed:** n/a — supersession by ruling (delta D1 and D4 — the feature home loses its
  baseline copies; D6(b) — an epic writes shared baselines in place; census H5).
- **Content (superseded text, verbatim):** `` `data-model.md` is a declared file in whichever home
  the run owns — the spec, the feature, the epic
  or the product baseline. Render `mochiko-cli home <that path>` for the file you are about to write,
  before the first write, and hold `` …
- **Kept deliberately:** the obligations to render `mochiko-cli home` before the first write, hold
  its file set and bound, and never copy a sibling file, plus the refused-write paragraph, verbatim.
- **Budget:** body 14,355 → 14,592 against 16,835; the description is unchanged at 497. Both are
  inside budget.
- **Consumers assessed:** `patterns-api-contracts` and `patterns-technical-decisions` take the same
  pointer in the same wave.

## [v0.91.0] Ladder blockquote re-keyed: "the plan ladder" / "the plan proposal" → design — plan-stage retirement D1

- **Disposition:** superseded → the same blockquote naming the design ladder and the
  design-phase proposal.
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1 carry-over: "plan-minimalism
  ladder governs what the design phase authors"; wording ruled by the wave lead 2026-08-26).
- **Content (superseded text, verbatim):**

  ```
  > **Entity necessity and shape answer the plan ladder** (`mochiko:patterns-plan-minimalism`) before an entity enters the model — the simplest-execution stops are disclosed in the plan proposal; this skill models the entities that survive it.
  ```

- **Kept deliberately:** the obligation itself — entity necessity and shape answer the ladder
  **before** an entity enters the model, the stops are disclosed rather than re-derived here, and
  this skill models only the survivors. The `mochiko:patterns-plan-minimalism` slug is unchanged
  (the skill kept its name through its own re-scope this wave), so the pointer still resolves.
- **Budget:** body 13,711 → **13,721** against the 16,835 budget; description unchanged at 497
  against 622. Both inside.
- **Consumers assessed:** `mochiko:patterns-plan-minimalism` was re-scoped to the design phase
  earlier in this wave and now uses the same vocabulary; `mochiko:authoring-technical-requirements`
  carried a parallel ladder blockquote, re-keyed identically in the same wave.
- **Re-keyed again at the fix round (V1 ripple):** the wording this entry landed —
  `the simplest-execution stops are disclosed in the design-phase proposal` — named an artifact
  the fix round's V1 ruling then retired (D4 lists plan's proposal approval among the dead gates;
  the design-phase authoring proposal does not survive in any form). Now reads "disclosed by the
  design phase as it authors", matching the ladder skill's surviving disclosure grammar. Body
  13,721 → **13,726** against the 16,835 budget; description unchanged at 497. The obligation is
  untouched in both passes — only the named disclosure site moved, twice.

## [v0.64.0] Guardrails cut — When-to-Use removed, canonical template + taxonomy kept; slim description

- **Disposition:** superseded → Wave 2 editorial guardrails cut (D4 cut line — When-to-Use bullets restate the description).
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail Wave 2, `DECISIONS.md` 2026-08-11 build row Wave 2 residual + user rulings 2026-08-10/11; method warrant: benchmark verdict `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`).
- **Content (faithfully compressed — section-level inventory; body 13,899 → 13,468 chars, −431, −3%; description 852 → 497 chars):**
  - **Removed whole:** `## When to Use` — the five-bullet list ("Creating data-model.md — extracting entities from requirements, user stories, and specifications" · "Defining attributes, types, and constraints for entities" · "Modeling relationships with cardinality, and state machines for stateful entities" · "Classifying the sensitivity of each attribute (Public / Internal / Confidential / Restricted) and its handling requirements" · "Brownfield analysis of existing data models"). Restates the description; the brownfield firing survives in `### Brownfield Entity Status`.
  - **Honest small yield:** this skill is dominated by v0.27.0-KEPT canonical content (see reconciliation) — the only guardrails-droppable section was the description-restating When-to-Use. The real cut here is the description (−355 chars). No forced percentage.
  - Old description verbatim: "This skill MUST be invoked when modeling a feature's domain data — extracting entities from requirements, defining attributes and conceptual types, mapping relationships (cardinality and delete behavior), documenting state machines, and classifying each attribute's data sensitivity (the 4-level Public/Internal/Confidential/Restricted taxonomy) — to author the canonical data-model.md. SHOULD also invoke when the design work involves \"extract entities\", \"define data model\", \"domain model\", \"model relationships\", \"cardinality\", \"state machine\", \"data attributes\", \"classify data sensitivity\", \"DS-XXX\", or per-attribute PII / encryption / retention classification. Authors the data model and its per-attribute sensitivity annotations — conceptual entities and their data, not REST/OpenAPI request/response schemas (those are patterns-api-contracts)."
  - Verbatim removed text survives in: git history of the SKILL.md (pre-v0.64.0); archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (the guardrails keep-set):** `## Overview`, `## When NOT to Use`, `## Entity Extraction` (identification heuristics + entity-vs-attribute rules + brownfield status table), `## Attribute Definition` (standard attributes + conceptual-type vocabulary), `## Data Sensitivity Classification` (the four-level taxonomy, PII mapping, decision tree, the five-step `### Annotating Sensitivity` procedure), `## Relationship Modeling` / `## State Machine Modeling` / `## Validation Rules` (pointers), `## data-model.md Structure` (the canonical template), `## Validation Script` (+ the producer-self-check-vs-independent-review boundary), `## Quality Checklist`, `## Common Mistakes`. The description keeps the MUST trigger, the modeling-outputs gist, the four-level taxonomy, the top trigger phrases, and the api-contracts sibling distinction.
- **Protected-content reconciliation (MANDATORY):** the v0.27.0 `KEPT: the remaining body` survivor ruling enumerated the protected core — the ~100-line data-model.md template, the sensitivity taxonomy + decision tree + PII mapping, **the five-step annotation procedure**, the conceptual-type vocabulary, entity-extraction heuristics and entity-vs-attribute rules, the brownfield status table, and the validation-script scope paragraph. **Every enumerated element survives this cut intact.** The removed `## When to Use` is NOT among the enumerated protected elements (it is a description-restatement, densified 7→5 the same wave but never named a survivor). Nothing protected silently dropped.
- **Consumers assessed:** `agents/technical-analyst.md`, `skills/authoring-technical-requirements/SKILL.md` (+ `references/ARTIFACT-TEMPLATES.md`, `references/TRACEABILITY-PATTERNS.md`), `skills/patterns-api-contracts/SKILL.md` (maps *from* the conceptual-type vocabulary — intact), `skills/patterns-system-design/SKILL.md`, `skills/patterns-technical-decisions/references/EVALUATION-MATRIX.md`, router `skills/mochiko/SKILL.md`, `templates/artifact-format.md`, `scripts/validate-model.py` (regexes check the authored artifact shape, not the SKILL's teaching sections — unaffected) — all reference the skill by name; none links a removed section anchor.

## [v0.27.0] In-file second copies and homed mistake rows stripped; reference drift repaired (body 359 → 297, −17%; DATA-SENSITIVITY.md 118 → 90)
- **Disposition:** deleted as in-file/reference restatements, each home Read before landing:
  the Attribute Format block (the same table format appears twice in the canonical
  data-model.md template 150 lines down — the declared single source; the Annotating-Sensitivity
  step-1 pointer retargeted to the template), the relationship text-diagram (verbatim in
  `references/RELATIONSHIP-PATTERNS.md`, richer — self-ref row + Symbol Reference), the
  When-to-Model-State bullets (identical four in `references/STATE-MACHINES.md`), six
  Common-Mistakes rows (homes: the in-file checklist rows, Conceptual Types section, the
  decision tree's classify-up line, the two references) · **drift repair (reference ledger):**
  `references/DATA-SENSITIVITY.md`'s own header declares "the four-level taxonomy and the
  decision tree live in the SKILL," yet the file duplicated the Classification Levels table,
  the PII paragraph, and the decision tree — all three cut from the reference (−28 lines), the
  SKILL copies untouched as the declared home; external-consumer grep: none (reference cited
  only within this skill) · densified: two surviving mistakes (anemic, orphan — no other home
  for either) → one 3-column table, When-to-Use 7 → 5
- **Tier failed:** 1 throughout (every cut had a verified home; the reference cut is the
  duplication-only-reference case D2 scopes in)
- **Content:** the format block, one text diagram, four bullets, six mistake subsections; the
  reference's levels table + PII paragraph + tree
- **Consumers assessed:** wave-open enumeration — 10 citing files incl.
  `scripts/validate-model.py`: its regexes check `## Relationships` / `## State Machine` /
  attribute-table shapes in the **authored artifact**, not the SKILL's teaching sections — no
  coupling to the stripped blocks

## [v0.27.0] KEPT: the remaining body (under-band survivor ruling, 17% vs 30–70)
- **Tier-2 evidence:** post-v0.23.0 this body is dominated by canonical-home content that has
  no other legal residence: the ~100-line data-model.md template (the single canonical
  template, this wave's untouchable core), the sensitivity taxonomy + decision tree + PII
  mapping (ownership declared by the reference's own header — the wave's drift repair pointed
  *into* the SKILL), the five-step annotation procedure, the conceptual-type vocabulary
  (`patterns-api-contracts`' Type Mapping maps *from* it), entity-extraction heuristics and
  entity-vs-attribute rules, the brownfield status table, and the validation-script scope
  paragraph (names the producer-self-check-vs-independent-review boundary). D1: the band is a
  calibration bar, not a quota. Session ruling: wave-3 batch-2 ratified 2026-07-25.

## [v0.23.0] Per-attribute Sensitivity Details blocks → once-per-document defaults + one row per attribute
- **Disposition:** revised per the wave-2 form ruling (R2's self-containment floor) — the 7-row per-attribute aspect table is deleted from the canonical data-model template (SKILL.md) and `references/DATA-SENSITIVITY.md`; handling-by-level defaults are stated **once per document**, each Confidential+ attribute is **one row** (Level · Retention · Access · Deviations · Compliance)
- **Tier failed:** artifact density: encryption/audit/masking are level-determined — repeating them per attribute restated the level default N times (kinako data-model.md 42k B); retention/access/compliance are the genuinely per-attribute content and are exactly what the row keeps
- **Content:** the `#### <attr> (Level)` + 7-row `| Aspect | Requirement |` block format (moved to the defaults matrix + deviation cells); the `### Classification Levels` legend table inside the artifact template (taxonomy restatement — the skill and DATA-SENSITIVITY.md own it; the once-per-doc handling-defaults matrix replaces its in-artifact function); entity intro blockquote collapsed onto the traceability line. Validation checklists (SKILL + reference) re-pointed at rows/defaults.
- **Consumers assessed:** plan producer + review-plan-artifacts ("Sensitivity details" check rephrased to the row form this wave) + `scripts/validate-model.py` (checked: per-entity `Sensitivity Details` heading regex + Sensitivity-column checks still satisfied by the row form — no script change needed).
