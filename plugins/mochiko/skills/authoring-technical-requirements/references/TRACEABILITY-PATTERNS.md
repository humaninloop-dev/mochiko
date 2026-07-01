# Traceability Patterns

Cross-reference patterns, dependency chains, and completeness validation rules for the **analysis artifacts** this skill authors (TR-XXX, C-XXX, D-XXX, NFR-XXX, IP-XXX).

## The Traceability Web

Every artifact connects to others. No artifact stands alone. The traceability web ensures that business intent is preserved through technical translation, and that every technical decision traces to a business justification.

```
Business Specifications (FR-XXX, user stories)
        │
        ▼
┌─────────────────────────────────────────────┐
│          Technical Requirements (TR-XXX)      │
│          ← maps to → FR-XXX                   │
│          ← constrained by → C-XXX             │
│          ← qualified by → NFR-XXX             │
│          ← shaped by → D-XXX                  │
│          ← provisioned by → IP-XXX            │
└─────────────────────────────────────────────┘
        │
        ▼  (downstream — authored by the design skills, not here)
   Entities + per-attribute sensitivity → mochiko:patterns-entity-modeling
   Endpoints + schemas + x-integration   → mochiko:patterns-api-contracts
```

The analysis artifacts feed the design layer; this reference covers the analysis half of the web (the box above). The design-artifact traceability — entity→FR, schema→entity, endpoint→integration, attribute→sensitivity — is authored and traced inside the design skills, and the *cross-artifact consistency between the two layers* is graded by the independent plan reviewer (`mochiko:validation-plan-artifacts`), not self-asserted here. See **Design-layer traceability** below.

## Analysis Cross-References

### TR → FR (Business Traceability)

Every technical requirement MUST trace to at least one business functional requirement.

**Pattern:**
```markdown
## TR-001: Authentication Flow

**Source:** FR-001, FR-003
```

**Validation rule:** Scan all TR entries. Each MUST have a non-empty `Source` field referencing at least one FR-XXX.

**Reverse check:** Scan all FRs from the business specification. Each FR MUST appear as a source in at least one TR. If an FR has no corresponding TR, it was missed during translation.

### TR → C (Constraint Awareness)

Technical requirements SHOULD reference constraints that affect their implementation.

**Pattern:**
```markdown
## TR-005: Data Storage

**Dependencies:**
- C-001 (must use existing PostgreSQL cluster)
- C-003 (zero-downtime migration required)
```

**Why this matters:** When architects read a TR, they need to see which constraints apply immediately, not discover them later in a separate document.

### TR → NFR (Quality Binding)

Technical requirements SHOULD reference NFRs that set quality targets for their operation.

**Pattern:**
```markdown
## TR-001: Authentication Flow

**Dependencies:**
- NFR-001 (p95 latency < 200ms applies to this flow)
- NFR-004 (zero plaintext PII in error responses)
```

### C → D (Constraint-Decision Link)

Constraints SHOULD reference the decisions they shaped. Decisions MUST reference the constraints that shaped them.

**Pattern (Constraint side):**
```markdown
## C-001: Existing PostgreSQL Infrastructure

**Impact:**
- Eliminates NoSQL options
- Shapes D-001 (database choice)
```

**Pattern (Decision side):**
```markdown
## D-001: Primary Database

**Shaped By:**
- C-001 (must use existing PostgreSQL cluster)
```

**Validation rule:** Every D-XXX entry MUST have a non-empty `Shaped By` field. Every C-XXX entry SHOULD reference at least one D-XXX in its Impact section.

> The C↔D link is the *traceability*; the decision *technique* that fills each D-XXX (evaluating alternatives, scoring trade-offs, ADR depth) is owned by `mochiko:patterns-technical-decisions`.

### NFR → Source (Justification)

Every NFR MUST trace to a business source that justifies the target.

**Pattern:**
```markdown
## NFR-001: API Response Latency

**Source:** FR-001 implies real-time user interaction;
           stakeholder expectation of "instant feedback" in spec section 3.2
```

**Validation rule:** No NFR without a source. Targets pulled from thin air are not requirements — they are guesses.

### C/NFR → IP (Infrastructure Provisioning)

Every constraint or NFR that implies platform work SHOULD reference the IP-XXX item that provisions it.

**Pattern:**
```markdown
## IP-001: Compute Provisioning

**Source:** C-001 (existing AWS environment), NFR-003 (10,000 concurrent users)
```

**Validation rule:** Every constraint implying platform provisioning has a corresponding IP-XXX; every IP-XXX traces back to a C-XXX or NFR-XXX in its `Source`.

## Dependency Chains

Some traceability relationships form chains that must be consistent end-to-end. The analysis chain (FR → TR → C/D/NFR/IP) is owned here; its design tail (Entity, Endpoint, sensitivity, x-integration) is authored downstream in the design skills.

### Full Traceability Chain

```
FR-001 (business: "users can sign in")
  └── TR-001 (technical: authentication flow)
        ├── C-001 (constraint: must use existing identity provider)
        ├── D-002 (decision: JWT with refresh tokens)
        ├── NFR-001 (quality: p95 < 200ms)
        └── IP-002 (provisioning: identity-provider connectivity)
              └── [design tail, authored downstream]
                    Entity: User (+ per-attribute sensitivity)  → patterns-entity-modeling
                    Endpoint: POST /auth/token (+ x-integration) → patterns-api-contracts
```

**Reading this chain:** Business requirement FR-001 is implemented by TR-001, which is constrained by C-001, informed by decision D-002, must meet NFR-001 latency, and requires IP-002 provisioning. The downstream design tail (the User entity with classified attributes, exposed via an endpoint that integrates with an external identity provider) is authored by the design skills, tracing back up to this analysis chain.

### Constraint Impact Chain

```
C-002 (regulatory: GDPR Art. 17 right to erasure)
  ├── TR-015 (technical: data deletion workflow)
  ├── D-004 (decision: soft-delete with 30-day purge)
  └── IP-003 (provisioning: scheduled purge job)
```

**Reading this chain:** Regulatory constraint C-002 drives TR-015 and decision D-004, and necessitates the IP-003 provisioning for the purge job. Downstream, this constraint also drives the entity retention policy and a DELETE endpoint — authored in the design skills.

## Completeness Validation (Analysis Artifacts)

### Forward Traceability (FR → TR)

Check that every business requirement has technical coverage.

**Procedure:**
1. List all FR-XXX from the business specification
2. For each FR, find TR entries that reference it as Source
3. Flag any FR with zero TR coverage

**Output format:**
```markdown
## Forward Traceability Check

| FR | Technical Requirements | Status |
|----|----------------------|--------|
| FR-001 | TR-001, TR-002, TR-003 | Covered |
| FR-002 | TR-004, TR-005 | Covered |
| FR-003 | (none) | **GAP — missing TR** |
```

### Backward Traceability (TR → FR)

Check that every technical requirement traces to a business source.

**Procedure:**
1. List all TR-XXX entries
2. For each TR, verify Source field references valid FR(s)
3. Flag any TR with no source (orphan)

### Decision Traceability (D → C)

Check that every decision references the constraints that shaped it.

**Procedure:**
1. List all D-XXX entries
2. For each, verify Shaped By field references valid C-XXX or NFR-XXX entries
3. Flag any decision without constraint references

### NFR Measurability Check

Verify every NFR has all three required elements.

**Procedure:**
1. List all NFR-XXX entries
2. For each, verify: target (numeric), measurement method, source
3. Flag any NFR missing elements

### Infrastructure Coverage (C/NFR → IP)

Verify every platform-implying constraint or NFR is provisioned.

**Procedure:**
1. List all C-XXX and NFR-XXX entries that imply platform work
2. For each, verify a corresponding IP-XXX exists with a matching `Source`
3. Flag any platform-implying constraint with no IP-XXX

## Cross-Artifact Consistency Rules (Analysis-Internal)

These are producer **self-checks** on the analysis artifacts before finalizing. They are not the independent gate — the cross-artifact consistency *grade* is owned by `mochiko:validation-plan-artifacts` (a different agent).

### Rule 1: ID References Must Resolve

Every cross-reference (TR-XXX, C-XXX, D-XXX, NFR-XXX, IP-XXX) appearing in any analysis artifact MUST correspond to an actual entry in the appropriate artifact file.

**Violation example:** TR-005 references "C-003" but constraints-and-decisions.md only has C-001 and C-002.

### Rule 2: Bidirectional References Should Match

If TR-005 lists C-001 as a dependency, then C-001 SHOULD list TR-005 in its Impact section. Mismatches indicate incomplete traceability.

### Rule 3: No Contradictory Constraints

If C-001 says "must use existing PostgreSQL" and a TR says "must support any SQL database," there is a contradiction. Constraints restrict — TRs must be compatible with all applicable constraints.

## Design-layer traceability (authored downstream — referenced, not owned here)

The traceability rules that govern the **design** artifacts are owned by the skills that author those artifacts; this analysis skill points at them rather than restating them:

| Design-layer rule | Owner |
|-------------------|-------|
| Entity → FR (every entity traces to a business requirement) | `mochiko:patterns-entity-modeling` |
| Attribute → Sensitivity (every PII/sensitive attribute is classified) | `mochiko:patterns-entity-modeling` |
| Endpoint → User Action (every action maps to an endpoint) | `mochiko:patterns-api-contracts` |
| Schema → Entity (response schemas match entity attributes) | `mochiko:patterns-api-contracts` |
| Endpoint → Integration (external-system endpoints carry `x-integration`) | `mochiko:patterns-api-contracts` |
| Cross-artifact consistency / feasibility *grading* (schema-entity alignment, NFR-vs-design feasibility, sensitivity-response alignment, contradiction detection) | the independent plan reviewer (`mochiko:validation-plan-artifacts`) |

The analysis artifacts authored here are the **upstream anchor** every one of those design rules traces back to: keep TR/C/D/NFR/IP IDs stable and resolvable so the downstream chains hold.

## Traceability Matrix Template

For complex features, produce a summary matrix. The analysis producer fills the FR / TR / Constraints / Decisions / NFRs columns; the Entities and Endpoints columns are completed downstream when the design artifacts are authored.

```markdown
## Traceability Matrix

| FR | TRs | Constraints | Decisions | NFRs | Entities | Endpoints |
|----|-----|-------------|-----------|------|----------|-----------|
| FR-001 | TR-001, TR-002, TR-003 | C-001 | D-002 | NFR-001, NFR-004 | User | POST /auth/token |
| FR-002 | TR-004, TR-005 | - | D-001 | NFR-001 | Order, Payment | POST /orders |
| FR-003 | TR-006, TR-007, TR-008 | C-002 | D-004 | NFR-002 | User, Order | DELETE /users/{id} |
```

This matrix provides a single view of the entire traceability web from business requirements through design, making gaps immediately visible.
