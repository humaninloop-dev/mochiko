# Plan Artifact Review Checklists

Detailed completeness checklists, organized **by artifact type** — analysis artifacts,
design artifacts, and the cross-artifact consistency pass. The caller (the plan lead) supplies
which artifacts are in scope for a given review; this reference does not encode a phase order
(sequencing is the lead's, not the checklist's).

Each check carries a fixed question and a severity (Critical / Important / Minor). The verdict is
derived **mechanically** from the issue counts (see SKILL.md → Verdict Criteria) — this is a
mirror checklist, not open-ended hunting.

## Scope of these checklists

These checklists grade **completeness, coverage, measurability, presence, and cross-artifact
consistency** — "is everything that should be here actually here, traceable, measurable, and
internally consistent with the decisions that were made?"

They deliberately **do not** grade cross-artifact **feasibility / buildability / contradiction** —
"can these artifacts actually be built together, or do they contradict / overreach?" That is a
distinct adversarial review owned by `mochiko:validation-feasibility`. The exact handoff is in
[the boundary table](#scope-boundary--handoff-to-validation-feasibility) at the foot of this file;
do not re-import those checks here.

---

## Analysis Artifacts

Grade these when reviewing the analysis output set (e.g. `requirements.md`,
`constraints-and-decisions.md`, `nfrs.md`).

### Checklist — Technical Requirements (`requirements.md`)

| Check | Question | Severity |
|-------|----------|----------|
| FR coverage | Is every functional requirement from the spec mapped to at least one TR? | Critical |
| Orphan TRs | Are there technical requirements with no business source? | Critical |
| Testable criteria | Does every TR have measurable acceptance criteria? | Critical |
| Dependency references | Do TRs reference relevant constraints and NFRs? | Important |
| Priority assignment | Are TR priorities consistent with source FR priorities? | Important |
| RFC 2119 language | Do requirements use MUST/SHOULD/MAY consistently? | Minor |

### Checklist — Constraints and Decisions (`constraints-and-decisions.md`)

| Check | Question | Severity |
|-------|----------|----------|
| Sourced constraints | Is every constraint traceable to a real limitation (not a preference)? | Critical |
| Alternative analysis | Were 2+ alternatives considered for each decision? | Critical |
| Rationale quality | Does each decision explain WHY, not just WHAT? | Critical |
| Constraint-decision cross-refs | Does each decision reference the constraints that shaped it? | Important |
| Constitution alignment | Do choices follow project principles? | Important |
| Infrastructure coverage | Does every constraint implying platform provisioning have a corresponding IP-XXX item? | Critical |
| IP-NFR coverage | Do NFRs requiring platform infrastructure (availability, scalability) have corresponding IP-XXX items? | Critical |
| Brownfield consideration | Was the existing stack evaluated (if brownfield)? | Important |
| Trade-off documentation | Are downsides of chosen options acknowledged? | Important |

> Whether a requirement or NFR **contradicts** a stated constraint is a feasibility concern →
> `mochiko:validation-feasibility`. This checklist grades only that the constraints/decisions are
> sourced, justified, and complete.

### Checklist — NFRs (`nfrs.md`)

| Check | Question | Severity |
|-------|----------|----------|
| NFR measurability | Does every NFR have a specific, measurable target? | Critical |
| NFR measurement method | Is the measurement approach defined? | Critical |
| NFR source tracing | Do NFR sources trace to valid TRs or business requirements? | Important |
| Category coverage | Are all relevant quality categories addressed? | Important |

> Whether the design can **meet** an NFR target, and whether NFR targets **conflict** with
> constraints or with each other, are feasibility concerns → `mochiko:validation-feasibility`.
> This checklist grades only that targets are present, measurable, and have a defined measurement
> method.

### Key Questions — Analysis

- Which functional requirements have NO corresponding technical requirement?
- Are any constraints actually disguised technology preferences?
- Can each acceptance criterion be verified in a test?
- What unknowns were NOT addressed by decisions?
- Are any decisions made without considering alternatives?
- Is the rationale convincing, or just restating the choice?
- Are there constraints implying deployment/CI/CD/environment work without corresponding IP-XXX items?
- Do NFR availability/scalability targets require platform infrastructure not captured as IP-XXX items?
- Can each NFR target actually be measured with available tooling?

---

## Design Artifacts

Grade these when reviewing the design output set (e.g. `data-model.md`, `contracts/api.yaml`,
`quickstart.md`).

### Checklist — Data Model (`data-model.md`)

| Check | Question | Severity |
|-------|----------|----------|
| Entity coverage | Is every noun from requirements modeled? | Critical |
| Attribute completeness | Do all entities have required fields? | Critical |
| Relationship definition | Are all connections documented with cardinality and delete behavior? | Critical |
| PII identification | Are sensitive fields annotated with classification? | Critical |
| Sensitivity details | Do Confidential+ attributes have full handling requirements? | Critical |
| Compliance coverage | Are relevant compliance requirements (GDPR, PCI, etc.) addressed? | Important |
| Retention policies | Are data retention and deletion policies specified? | Important |
| Encryption standards | Are encryption requirements specified for each classification level? | Important |
| Validation rules | Are constraints from requirements captured? | Important |
| State machines | Are stateful entities properly modeled? | Important |
| Standard fields | Do all entities have id, createdAt, updatedAt? | Important |
| Traceability | Can we trace entities to requirements? | Important |

### Checklist — API Contracts (`contracts/api.yaml`)

| Check | Question | Severity |
|-------|----------|----------|
| Endpoint coverage | Does every user action have an endpoint? | Critical |
| Schema completeness | Are request/response schemas defined? | Critical |
| Error handling | Are failure modes documented with specific status codes? | Critical |
| Schema-model consistency | Do schemas match the data model entities? | Critical |
| Integration boundary presence | Do endpoints wrapping external systems have x-integration documentation? | Critical |
| Failure-mode presence | Does every integration boundary have documented failure modes and fallbacks? | Critical |
| Authentication | Are auth requirements clear? | Important |
| Examples | Are realistic scenarios documented? | Important |
| Naming consistency | Do endpoints follow conventions? | Minor |

> This checklist grades that integration boundaries and their failure modes are **documented and
> present**. Whether the documented failure modes are **realistic vs aspirational**, and whether the
> design can actually be **built** against the named systems, are feasibility concerns →
> `mochiko:validation-feasibility`.

### Checklist — Integration Guide (`quickstart.md`)

| Check | Question | Severity |
|-------|----------|----------|
| Flow coverage | Are common user flows documented with examples? | Important |
| Auth documentation | Is the authentication sequence clear? | Important |
| Error documentation | Are error handling patterns explained? | Important |
| External integrations | Is the external system boundary overview present? | Important |

### Key Questions — Design

- What entities from the spec are missing?
- Are there relationships that should exist but don't?
- Did we miss any PII fields or sensitive data?
- Are encryption and retention requirements complete for all Confidential+ data?
- What user actions don't have endpoints?
- Are there error scenarios not handled?
- Do the schemas actually match our data model?
- What external systems are implied by requirements but not documented?
- Is the quickstart documentation usable?

---

## Cross-Artifact Consistency {#cross-artifact-consistency}

The single source for verifying that the artifacts agree with **each other** and honor the
decisions that were made. Run this both for a full review and for the lightweight consistency pass
of an incremental review (SKILL.md → Incremental Review Mode). It folds in the cross-artifact
consistency checklist that previously lived as a standalone template.

### Consistency checklist

| Check | Question | Severity |
|-------|----------|----------|
| Requirements-decisions alignment | Do decisions serve the requirements they reference? | Critical |
| Decisions-model consistency | Are model choices consistent with technology decisions? | Critical |
| Model-contract consistency | Do schemas reflect the data model exactly? | Critical |
| Sensitivity-contract alignment | Do API responses respect data classification (no Restricted data in responses)? | Critical |
| Integration-contract alignment | Do contract integration boundaries match the systems implied by requirements? | Critical |
| Requirement traceability | Can we trace from FR to TR to entity to endpoint? | Important |
| Constitution compliance | Do all artifacts follow project principles? | Important |
| Infrastructure completeness | Are all IP-XXX items addressable by design artifacts or flagged for separate provisioning cycles? | Important |

> "Consistency" here means **does the later artifact honor the earlier decisions** — a design that
> contradicts a *decided* approach (e.g. uses JWT when sessions were chosen) is a consistency
> failure and stays here. Whether two *constraints/requirements* contradict each other, or whether
> the design can be **built** within the constraints / meet the NFR targets, is a feasibility
> concern → `mochiko:validation-feasibility`.

### Named consistency groups

Use these as the spot-check lens when running the consistency pass quickly:

**Entity-Name Consistency**
- The artifact uses the same entity names as `data-model.md`.
- No new entities introduced that aren't in `data-model.md`.
- Spelling / casing matches exactly.

**Requirement Traceability**
- The artifact references FR-XXX / US-XXX IDs correctly.
- No orphaned requirements (mentioned but not addressed).
- No invented requirements (addressed but not in spec).

**Decision Consistency**
- The artifact honors decisions from `constraints-and-decisions.md`.
- No contradictions with chosen technologies / approaches (a design contradicting a *decided*
  approach is a Critical consistency failure here; a contradiction *between two requirements or
  constraints* is feasibility's).
- The original rationale still applies.

**Naming Conventions**
- API endpoints follow patterns established in `constraints-and-decisions.md`.
- Field names match data-model attributes.
- Error codes are consistent.

### Cross-Reference Steps

1. **Traceability** — can trace requirement → artifact.
2. **Consistency** — artifacts agree with each other and honor the decisions.
3. **Completeness** — nothing obviously missing.

### Issue classification for consistency findings

| Issue Type | Severity | Example |
|------------|----------|---------|
| Entity name mismatch | Important | "User" vs "Account" inconsistency |
| Missing requirement trace | Important | FR-003 not addressed in contracts |
| Decision contradiction (design vs decided approach) | Critical | Using JWT when the decision chose sessions |
| Minor spelling variance | Minor | "userId" vs "user_id" |

---

## Automated Validation (Tier-1 pre-assert)

Before the manual model review, run the deterministic checker for the cheap, greppable slice. It is
the strongest validation layer where it applies (a grep is ground truth); the model review handles
everything it cannot settle.

```bash
# Single file
python scripts/check-artifacts.py .mochiko/specs/<feature>/data-model.md

# Multiple files (enables the entity-consistency cross-check)
python scripts/check-artifacts.py .mochiko/specs/<feature>/requirements.md .mochiko/specs/<feature>/data-model.md

# All plan artifacts
python scripts/check-artifacts.py .mochiko/specs/<feature>/*.md
```

### Automated check coverage

| Check | Description | Applies To |
|-------|-------------|------------|
| `unresolved_markers` | Finds `[NEEDS CLARIFICATION]`, `[TBD]`, `[TODO]`, `[PLACEHOLDER]` | All files |
| `required_sections` | Verifies expected markdown headers exist | All .md artifacts |
| `traceability` | Confirms FR-XXX / US-XXX references present | All files |
| `pii_markers` | Checks if PII fields have sensitivity annotations | data-model.md |
| `entity_consistency` | Validates entity names appear across related files | When 2+ files provided |

**Note**: OpenAPI/contract files are skipped by this checker — validate them with the OpenAPI
validator bundled with the api-contracts skill.

### Exit codes

- `0` — All checks passed
- `1` — One or more checks failed

---

## Out of scope here: codebase-discovery review

Reviewing a brownfield codebase-discovery artifact (directory-scan coverage, entity/endpoint
detection completeness, collision-risk assessment) is **not** part of plan-core completeness review —
it belongs to the brownfield / discovery track and is graded there, not by this skill. Recorded as
moved (not dropped); rebind by reference when that track ports.

---

## Scope boundary — handoff to `validation-feasibility` {#scope-boundary--handoff-to-validation-feasibility}

This is the explicit seam between the two plan reviewers, so the feasibility side can mirror it
exactly. **This skill keeps the left column; `mochiko:validation-feasibility` owns the right.**

| Family | THIS skill keeps (completeness lens) | `validation-feasibility` owns (buildability lens) |
|--------|--------------------------------------|---------------------------------------------------|
| Coverage / traceability | FR→TR coverage, orphan TRs, NFR source tracing, entity coverage, endpoint coverage, FR→TR→entity→endpoint traceability, IP / IP-NFR coverage | — |
| Measurability | testable TR criteria; NFR measurable target present; measurement method defined | — |
| Presence | sourced constraints; ≥2 alternatives; rationale (WHY); trade-offs; DS annotations + sensitivity details present; integration-boundary x-integration present; failure modes documented; auth / examples present | — |
| Consistency (does the design honor the decisions?) | requirements-decisions alignment; decisions-model consistency; model-contract / schema-model consistency; sensitivity-contract alignment; integration-contract alignment; constraint-decision cross-refs; constitution compliance; decision-honored-by-design | — |
| **Contradiction** (do artifacts conflict?) | — | **TR ↔ constraint contradictions; NFR ↔ constraint conflicts; NFR ↔ NFR impossibilities** |
| **Buildability** (can it be built / met?) | — | **NFR-design feasibility (can the design meet the NFR targets?); constraint-design buildability (can the design satisfy the constraints?); integration failure modes realistic vs aspirational** |

**The one-line test:** *"is it here, traceable, measurable, and does it honor the decisions?"* →
this skill. *"can these pieces be built together without contradiction or overreach?"* →
`validation-feasibility`.
