---
name: patterns-entity-modeling
description: This skill MUST be invoked when modeling a feature's domain data — extracting entities, defining attributes and conceptual types, mapping relationships (cardinality, delete behavior), documenting state machines, and classifying each attribute's data sensitivity (Public/Internal/Confidential/Restricted) — to author data-model.md. SHOULD also invoke on 'data model', 'domain model', 'model relationships', 'state machine', 'data sensitivity', or 'DS-XXX'. Conceptual entities, not OpenAPI schemas.
---

# Modeling Domain Entities

## Overview

Extract and model domain entities from requirements using Domain-Driven Design principles. This skill covers entity identification, attribute definition, relationship modeling, state machine documentation, and per-attribute data-sensitivity classification. It is the single home for the `data-model.md` artifact and its sensitivity annotations.

> **Entity necessity and shape answer the plan ladder** (`mochiko:patterns-plan-minimalism`) before an entity enters the model — the simplest-execution stops are disclosed in the plan proposal; this skill models the entities that survive it.

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

1. Add a **Sensitivity** column to every entity's attributes table (the attribute-table format in the *data-model.md Structure* template below).
2. State the **handling defaults once per document**: `data-model.md` carries the handling-by-level matrix (from [DATA-SENSITIVITY.md](references/DATA-SENSITIVITY.md)) a single time, under the summary. Per-attribute handling then follows the level default by construction.
3. For every **Confidential** or **Restricted** attribute, add one row to its entity's **Sensitivity Details** table — recording only the attribute's **specifics** (retention, access control) and any **deviations** from its level default, plus its compliance mapping. Never repeat the level-default aspects (encryption, audit, masking) per attribute.
4. Roll the classifications up into the **Data Sensitivity Summary** table at the top of `data-model.md` (entity / attribute / classification / compliance) — the artifact's **ID/coverage index**.
5. Where an attribute realizes a declared DS-XXX requirement from analysis, note it in the compliance cell for traceability.

See [DATA-SENSITIVITY.md](references/DATA-SENSITIVITY.md) for the field definitions, the handling-by-level matrix, the compact Sensitivity Details row format, and compliance-mapping examples.

## Relationship Modeling

Relationships connect entities with defined cardinality: One-to-One (1:1), One-to-Many (1:N), or Many-to-Many (N:M).

See [RELATIONSHIP-PATTERNS.md](references/RELATIONSHIP-PATTERNS.md) for detailed patterns, join entity examples, the text-diagram notation and symbol reference, and documentation formats.

## State Machine Modeling

Entities with status fields need state transition documentation.

See [STATE-MACHINES.md](references/STATE-MACHINES.md) for when to model state, patterns, diagram formats, and common workflows.

## Validation Rules

Constraints and validation rules ensure data integrity.

See [VALIDATION-RULES.md](references/VALIDATION-RULES.md) for constraint patterns, format validations, and business rule documentation.

## data-model.md Structure

The canonical `data-model.md` template — its structure, the per-attribute sensitivity shape, and a
worked example — is delivered by the `data-model` schema: run `mochiko-cli template data-model` for
the producer view, or Read `plugins/mochiko/schemas/data-model.yaml` raw when the binary is absent
(the shipped schema is the first-class source of truth, D8). This skill owns the `data-model.md`
artifact and its data-sensitivity taxonomy; the schema carries the fill-in shape. Every attribute
carries a sensitivity classification; the handling-by-level defaults appear **once per document**;
every Confidential or Restricted attribute is one Sensitivity Details row (specifics and deviations
only — format in [DATA-SENSITIVITY.md](references/DATA-SENSITIVITY.md)). Density is not a gap; a gap
is a missing entity, classification, or relationship.

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
- [ ] Handling defaults stated once per document; every Confidential or Restricted attribute has a Sensitivity Details row (specifics + deviations)
- [ ] Data Sensitivity Summary table reflects all Confidential+ attributes
- [ ] State machines documented for stateful entities
- [ ] Brownfield status indicated for each entity
- [ ] Traceability to requirements documented

## Common Mistakes

| Mistake | ❌ Bad | ✅ Good |
|---------|--------|---------|
| Anemic entities | Entity with only `id` field and no attributes | Every entity needs meaningful attributes that describe its purpose |
| Orphan entities | Entities with no relationships to other entities | Every entity connects to at least one other entity (or is explicitly standalone) |
