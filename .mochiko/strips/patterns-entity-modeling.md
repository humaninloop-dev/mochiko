# Strip notes — `skills/patterns-entity-modeling/`

Entry formats: `strips/README.md`. Wave context: [v0.27.0] entries — skill-succinctness wave 3
(design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`, ratified 2026-07-25);
[v0.23.0] entries — workflow-token-reduction wave 2 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md` D4 + the wave-2 rulings R1–R4;
ratified 2026-07-24).

## [v0.77.0] `data-model.md Structure` template extracted to a schema — clean lift (D3 later-ratchet)
- **Disposition:** superseded → `plugins/mochiko/schemas/data-model.yaml` + `mochiko-cli`
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance **D3 later-ratchet** + user ruling 2026-08-16 (recorded at the v0.76.0 landing); record `.mochiko/brainstorms/schema-based-template-guidance/record.md` D3; `DECISIONS.md` "Template-schema ratchet" row (landed at v0.77.0))
- **Content (superseded, verbatim — the `## data-model.md Structure` section, SKILL.md before the phase-2 body edit):**

````markdown
## data-model.md Structure

This is the **single canonical `data-model.md` template**, following the deliverable
envelope in [`artifact-format.md`](../../templates/artifact-format.md). Every attribute
carries a sensitivity classification; the handling-by-level defaults appear **once per
document**; every Confidential or Restricted attribute is one **Sensitivity Details row**
(specifics + deviations only — format in [DATA-SENSITIVITY.md](references/DATA-SENSITIVITY.md)).
Density is not a gap; a gap is a missing entity, classification, or relationship.

```markdown
# Data Model: {feature_id}

> Entity definitions with relationships, per-attribute sensitivity annotations, and state machines.

## Data Sensitivity Summary  *(the coverage index)*

| Entity | Attribute | Classification | Compliance |
|--------|-----------|---------------|------------|
| User | email | Confidential | GDPR Art. 6 |
| User | passwordHash | Restricted | NIST 800-63 |

**Handling defaults (once per document — per-attribute rows record only specifics and deviations):**

| Aspect | Confidential | Restricted |
|--------|-------------|------------|
| Encryption at rest / in transit | Required (AES-256 / TLS 1.3+) | Required, strong (AES-256 / TLS 1.3+) |
| Audit logging | All access logged | All access logged + real-time anomaly alerts |
| Masking in logs/UIs | Required | Never displayed, never logged |

---

## Entity Summary

| Entity | Attributes | Relationships | Status |
|--------|------------|---------------|--------|
| User | 8 | 3 | [EXTENDS EXISTING] |
| Session | 5 | 1 | [NEW] |

---

## Entity: User [EXTENDS EXISTING]

Existing entity extended with authentication fields. **Traceability:** FR-001, FR-002, US#1

### Attributes

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| passwordHash | Text | Yes | - | Restricted | Hashed password |
| lastLoginAt | Timestamp | No | null | Internal | Last login time |

### Existing Attributes (Not Modified)

| Attribute | Type | Sensitivity | Description |
|-----------|------|-------------|-------------|
| id | UUID | Internal | Existing primary key |
| email | Email | Confidential | Existing email field |

### Sensitivity Details  *(one row per Confidential+ attribute — specifics + deviations from the level default)*

| Attribute | Level | Retention | Access | Deviations | Compliance |
|-----------|-------|-----------|--------|------------|------------|
| passwordHash | Restricted | Until account deletion; purge on delete | System-only; no user/admin read | — | NIST 800-63 (DS-001) |
| email | Confidential | Delete ≤ 30d after account closure | Users read own; admins read all | Log masking: j***@example.com | GDPR Art. 6, Art. 17 |

---

## Entity: Session [NEW]

User authentication session. **Traceability:** FR-003, US#2

### Attributes

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| id | UUID | Yes | auto | Internal | Session identifier |
| userId | Reference(User) | Yes | - | Internal | Owning user |
| token | Text(255) | Yes | - | Restricted | Session token |
| expiresAt | Timestamp | Yes | - | Internal | Expiration time |
| createdAt | Timestamp | Yes | auto | Internal | Creation time |

### Relationships

| Relationship | Cardinality | Target | Delete Behavior | Description |
|--------------|-------------|--------|-----------------|-------------|
| user | N:1 | User | Cascade | Session belongs to user |

### Sensitivity Details

| Attribute | Level | Retention | Access | Deviations | Compliance |
|-----------|-------|-----------|--------|------------|------------|
| token | Restricted | Purge on session expiry | System-only | Audit: issued/revoked events only | — |

---

## Relationships

[Cross-entity relationship documentation — see RELATIONSHIP-PATTERNS.md]

## State Machines

[State machine documentation — only when stateful entities exist; omit otherwise — see STATE-MACHINES.md]

## Validation Rules

[Entity constraints and business rules — see VALIDATION-RULES.md]
```
````
- **Kept deliberately:** the fenced `data-model.md` template → `data-model.yaml`'s `skeleton` (clean lift, verbatim); the intro paragraph (artifact-format envelope, sensitivity-classification, handling-defaults-once, "density is not a gap") + each in-template section's purpose → the schema's `overview` / `contract`; the schema's example fragments → `good`. The skill's OWN sections that follow the template — `## Validation Script` and `## Quality Checklist` (SKILL.md:265+) — are NOT part of the data-model template and stay in the skill body untouched. Net-new per-section `check` lines **authored under D7**. Nothing dropped.
- **Phase note:** schema authored at PHASE 1; the atomic SKILL-body edit (replace the `## data-model.md Structure` section with the two-arm pointer — `mochiko-cli template data-model`, else Read `plugins/mochiko/schemas/data-model.yaml`) is the PHASE 2 edit (same P2 seat), gated on V1 fidelity PASS.
- **Consumers assessed:** `patterns-entity-modeling/SKILL.md` (owns the template — body edit phase 2) · `authoring-technical-requirements/references/ARTIFACT-TEMPLATES.md` ownership note (data-model owned here — consistent, retired same wave) · `review-plan-artifacts/references/ARTIFACT-CHECKLISTS.md` Data Model checklist (additive structure-presence re-key phase 2).

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
