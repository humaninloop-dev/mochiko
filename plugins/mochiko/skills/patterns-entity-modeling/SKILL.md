---
name: patterns-entity-modeling
description: This skill MUST be invoked when modeling a feature's domain data — extracting entities from requirements, defining attributes and conceptual types, mapping relationships (cardinality and delete behavior), documenting state machines, and classifying each attribute's data sensitivity (the 4-level Public/Internal/Confidential/Restricted taxonomy) — to author the canonical data-model.md. SHOULD also invoke when the design work involves "extract entities", "define data model", "domain model", "model relationships", "cardinality", "state machine", "data attributes", "classify data sensitivity", "DS-XXX", or per-attribute PII / encryption / retention classification. Authors the data model and its per-attribute sensitivity annotations — conceptual entities and their data, not REST/OpenAPI request/response schemas (those are patterns-api-contracts).
---

# Modeling Domain Entities

## Overview

Extract and model domain entities from requirements using Domain-Driven Design principles. This skill covers entity identification, attribute definition, relationship modeling, state machine documentation, and per-attribute data-sensitivity classification. It is the single home for the `data-model.md` artifact and its sensitivity annotations.

## When to Use

- Creating data-model.md from requirements or specifications
- Extracting entities from user stories and functional requirements
- Defining attributes, types, and constraints for entities
- Modeling relationships between entities with cardinality
- Documenting state machines for stateful entities
- Classifying the sensitivity of each attribute (Public / Internal / Confidential / Restricted) and its handling requirements
- Brownfield analysis of existing data models

## When NOT to Use

- **API contract design** - Use `mochiko:patterns-api-contracts` instead (it consumes this skill's data-model.md; schemas trace to entities)
- **Database schema migration** - This skill is conceptual, not implementation
- **When data model already exists and is complete** - Don't duplicate work
- **Pure validation rules** - Model entities first, then add validation
- **Technical architecture decisions** - Use `mochiko:patterns-technical-decisions`
- **Declaring *which* data is sensitive as an analysis requirement (DS-XXX)** - that declaration is `mochiko:authoring-technical-requirements`; this skill applies the per-attribute classification and carries it in the data-model.md

## Entity Extraction

### Identification Heuristics

Look for entities in:

| Source | Pattern | Example |
|--------|---------|---------|
| **User stories** | "As a [Role]..." | User, Admin, Guest |
| **Subjects** | "The [Entity] must..." | Task, Order, Product |
| **Actions** | "...create a [Entity]" | Comment, Message, Report |
| **Possessives** | "[Entity]'s [attribute]" | User's profile, Order's items |
| **Status mentions** | "[Entity] status" | TaskStatus, OrderState |

### Entity vs. Attribute Decision

```
IF concept has its own lifecycle → Entity
IF concept only exists within another → Attribute
IF concept connects two entities → Relationship (possibly join entity)
IF concept has just one value → Attribute

Examples:
- "user email" → Attribute of User (just one value)
- "user address" → Could be Entity (if reused) or Attribute (if embedded)
- "order items" → Separate entity (has own lifecycle)
- "task status" → Enum/attribute (limited values)
```

### Brownfield Entity Status

When modeling in brownfield projects:

| Status | Meaning | Action |
|--------|---------|--------|
| `[NEW]` | Entity doesn't exist | Create full definition |
| `[EXTENDS EXISTING]` | Adding to existing entity | Document new fields only |
| `[REUSES EXISTING]` | Using existing as-is | Reference only |
| `[RENAMED]` | Avoiding collision | Document new name + reason |

## Attribute Definition

### Standard Attributes

Every entity typically needs:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| id | Identifier | Yes | Primary key |
| createdAt | Timestamp | Yes | Creation time |
| updatedAt | Timestamp | Yes | Last modification |
| deletedAt | Timestamp | No | Soft delete marker |

### Attribute Format

```markdown
| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| id | UUID | Yes | auto-generated | Internal | Unique identifier |
| email | Email | Yes | - | Confidential | User's email address |
| name | Text(100) | No | null | Public | Display name |
| role | Enum[admin,member,guest] | Yes | member | Internal | Access level |
| isVerified | Boolean | Yes | false | Internal | Email verified flag |
```

### Conceptual Types

Use conceptual types (not database-specific):

| Conceptual Type | Description |
|-----------------|-------------|
| `Identifier` / `UUID` | Unique identifier |
| `Text` / `Text(N)` | String with optional max length |
| `Email` | Email format string |
| `URL` | URL format string |
| `Integer` | Whole number |
| `Decimal` / `Decimal(P,S)` | Decimal with precision |
| `Boolean` | True/false |
| `Timestamp` | Date and time |
| `Date` | Date only |
| `Enum[values]` | Fixed set of values |
| `JSON` | Structured data |
| `Reference(Entity)` | Foreign key reference |

## Data Sensitivity Classification

Every attribute carries a **sensitivity classification**. This skill owns the data-sensitivity taxonomy applied in `data-model.md`; `mochiko:authoring-technical-requirements` only *declares* which data is sensitive as an analysis concern (a DS-XXX requirement) and points here for the per-attribute classification and template.

### Classification Levels

Classify every attribute into exactly one of four levels:

| Level | Definition | Examples |
|-------|------------|----------|
| **Public** | Freely shareable, no access controls needed | Product names, public profile info |
| **Internal** | Organization-internal, basic access controls | Transaction IDs, internal status codes |
| **Confidential** | Sensitive, role-based access required | Email addresses, billing addresses, standard PII |
| **Restricted** | Highly sensitive, strict access and audit | Passwords, SSNs, payment card numbers, credentials |

**PII maps onto these levels — it is not a separate axis:** standard PII (email, phone, address) classifies **Confidential**; highly sensitive PII (SSN, credentials, payment cards, health records) classifies **Restricted**. Tag PII in the attribute's classification, not as a parallel marker.

### Classification Decision Tree

```
Is the data publicly available or intended for public sharing?
├── Yes → PUBLIC
└── No
    ├── Is it internal operational data with no PII?
    │   ├── Yes → INTERNAL
    │   └── No
    │       ├── Is it PII, financial, or business-sensitive?
    │       │   ├── Yes → Is it highly sensitive (credentials, SSN, payment cards)?
    │       │   │   ├── Yes → RESTRICTED
    │       │   │   └── No → CONFIDENTIAL
    │       │   └── No → INTERNAL
    │       └── When in doubt → CONFIDENTIAL (classify up, not down)
```

### Annotating Sensitivity

1. Add a **Sensitivity** column to every entity's attributes table (see Attribute Format above).
2. For every **Confidential** or **Restricted** attribute, add a **Sensitivity Details** block specifying encryption (at rest / in transit), retention, access control, audit, masking, and compliance mapping.
3. Roll the classifications up into the **Data Sensitivity Summary** table at the top of `data-model.md` (entity / attribute / classification / compliance).
4. Where an attribute realizes a declared DS-XXX requirement from analysis, note it in the compliance/description cell for traceability.

See [DATA-SENSITIVITY.md](references/DATA-SENSITIVITY.md) for the full sensitivity-details field definitions, the handling-by-level matrix, the Sensitivity Details block format, and compliance-mapping examples.

## Relationship Modeling

Relationships connect entities with defined cardinality: One-to-One (1:1), One-to-Many (1:N), or Many-to-Many (N:M).

See [RELATIONSHIP-PATTERNS.md](references/RELATIONSHIP-PATTERNS.md) for detailed patterns, join entity examples, and documentation formats.

### Relationship Diagram (Text)

```markdown
## Entity Relationships

```
User ──1:N──▶ Task (owns)
User ──1:N──▶ Session (has)
User ◀──N:M──▶ Project (via ProjectMember)
Task ──N:1──▶ Project (belongs to)
```
```

## State Machine Modeling

Entities with status fields need state transition documentation.

See [STATE-MACHINES.md](references/STATE-MACHINES.md) for patterns, diagram formats, and common workflows.

### When to Model State

Model state machines when:
- Entity has a `status` or `state` field
- Requirements mention workflow or lifecycle
- Specific actions change entity state
- Certain actions only valid in certain states

## Validation Rules

Constraints and validation rules ensure data integrity.

See [VALIDATION-RULES.md](references/VALIDATION-RULES.md) for constraint patterns, format validations, and business rule documentation.

## data-model.md Structure

This is the **single canonical `data-model.md` template**. Every attribute carries a sensitivity classification; every Confidential or Restricted attribute carries a Sensitivity Details block (format in [DATA-SENSITIVITY.md](references/DATA-SENSITIVITY.md)).

```markdown
# Data Model: {feature_id}

> Entity definitions with relationships, per-attribute sensitivity annotations, and state machines.

---

## Data Sensitivity Summary

| Entity | Attribute | Classification | Compliance |
|--------|-----------|---------------|------------|
| User | email | Confidential | GDPR Art. 6 |
| User | passwordHash | Restricted | NIST 800-63 |
| Payment | cardNumber | Restricted | PCI-DSS |
| Order | transactionId | Internal | - |
| Product | name | Public | - |

### Classification Levels

| Level | Definition | Examples |
|-------|------------|----------|
| **Public** | Freely shareable, no access controls needed | Product names, public profile info |
| **Internal** | Organization-internal, basic access controls | Transaction IDs, internal status codes |
| **Confidential** | Sensitive, role-based access required | Email addresses, billing addresses, standard PII |
| **Restricted** | Highly sensitive, strict access and audit | Passwords, SSNs, payment card numbers, credentials |

---

## Entity Summary

| Entity | Attributes | Relationships | Status |
|--------|------------|---------------|--------|
| User | 8 | 3 | [EXTENDS EXISTING] |
| Session | 5 | 1 | [NEW] |

---

## Entity: User [EXTENDS EXISTING]

> Existing entity extended with authentication fields.

**Traceability:** FR-001, FR-002, US#1

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

### Sensitivity Details (Confidential+ attributes)

#### passwordHash (Restricted)

| Aspect | Requirement |
|--------|-------------|
| Encryption at Rest | Required — AES-256 |
| Encryption in Transit | Required — TLS 1.3+ |
| Retention Period | Retain until account deletion; purge on delete |
| Access Control | System-only; no user or admin read access |
| Audit | All access logged; real-time alerts on anomalous access |
| Masking | Never displayed; never logged |
| Compliance | NIST 800-63 (credential storage) |

#### email (Confidential)

| Aspect | Requirement |
|--------|-------------|
| Encryption at Rest | Required — AES-256 |
| Encryption in Transit | Required — TLS 1.3+ |
| Retention Period | Delete within 30 days of account closure |
| Access Control | Authenticated users read own; admins read all |
| Audit | All access logged with timestamp and user |
| Masking | Displayed masked in logs: j***@example.com |
| Compliance | GDPR Art. 6 (consent via signup), Art. 17 (deletion within 30 days) |

---

## Entity: Session [NEW]

> User authentication session.

**Traceability:** FR-003, US#2

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

### Sensitivity Details (Confidential+ attributes)

#### token (Restricted)

| Aspect | Requirement |
|--------|-------------|
| Encryption at Rest | Required — AES-256 |
| Encryption in Transit | Required — TLS 1.3+ |
| Retention Period | Purge on session expiry |
| Access Control | System-only |
| Audit | Issued / revoked events logged |
| Masking | Never logged in full |
| Compliance | - |

---

## Relationships

[Cross-entity relationship documentation — see RELATIONSHIP-PATTERNS.md]

---

## State Machines

[State machine documentation if applicable — see STATE-MACHINES.md]

---

## Validation Rules

[Entity constraints and business rules — see VALIDATION-RULES.md]
```

## Validation Script

Run the bundled structural linter as a **producer self-check** before handing the data model off. This is a heuristic, kernel-free Tier-2 check that confirms the *shape* is present (entities have ids, audit fields, attribute tables, relationships, state transitions, and sensitivity annotations). It is **not** the independent grade — the substantive review (right entities, sound cardinality, correct state machines, accurate sensitivity classification) is owned by an independent reviewer, never this skill.

```bash
python scripts/validate-model.py .mochiko/specs/<feature>/data-model.md
```

The script emits `checks`/`summary` JSON (exit 0 = all passed, 1 = one or more failed) covering: entity format, required attributes, relationships, state machines, validation rules, audit fields, id fields, and data-sensitivity annotation presence.

## Quality Checklist

Before finalizing entity model, verify:

- [ ] Every noun from requirements evaluated for entity status
- [ ] Each entity has id, createdAt, updatedAt fields
- [ ] All attributes have type, required flag, sensitivity, description
- [ ] Relationships include cardinality, direction, and delete behavior
- [ ] Every attribute classified Public / Internal / Confidential / Restricted
- [ ] Every Confidential or Restricted attribute has a Sensitivity Details block
- [ ] Data Sensitivity Summary table reflects all Confidential+ attributes
- [ ] State machines documented for stateful entities
- [ ] Brownfield status indicated for each entity
- [ ] Traceability to requirements documented

## Common Mistakes

### Missing Entities
❌ Skipping entities mentioned in requirements ("we'll add that later")
✅ Evaluate every noun from requirements for entity status

### Anemic Entities
❌ Entity with only `id` field and no attributes
✅ Every entity needs meaningful attributes that describe its purpose

### Implementation Types
❌ Using `VARCHAR(255)`, `INT(11)`, `BIGINT` in data model
✅ Use conceptual types: `Text(100)`, `Integer`, `Identifier`

### Undefined Relationships
❌ Reference attributes without relationship documentation
✅ Every `Reference(Entity)` needs cardinality and relationship description

### Hidden State Machines
❌ Status/state fields without transition documentation
✅ Every status field needs state machine diagram and valid transitions

### Unclassified Sensitivity
❌ Email, phone, address, password fields with no classification
✅ Classify every attribute (Public / Internal / Confidential / Restricted); give Confidential+ a Sensitivity Details block

### Under-classifying When Unsure
❌ Defaulting ambiguous data to Internal to avoid the handling burden
✅ When in doubt, classify up (Confidential), not down

### Orphan Entities
❌ Entities with no relationships to other entities
✅ Every entity connects to at least one other entity (or is explicitly standalone)
