# Traceability Patterns

Cross-reference patterns, dependency chains, and completeness validation rules for the **constraint-layer artifacts** this skill authors (C-XXX, D-XXX, NFR-XXX, IP-XXX, INT-XXX, DS-XXX).

## The Traceability Web

Every artifact connects to others. No artifact stands alone. The traceability web ensures that business intent is preserved through technical translation, and that every technical decision traces to a business justification.

```
Business Specifications (FR-XXX, SC-XXX, user stories)
        │
        ▼
┌─────────────────────────────────────────────┐
│   Constraint layer (constraints-and-decisions │
│   .md + the store's NFR rows)                 │
│          C-XXX  ← sourced from → FR/SC        │
│          D-XXX  ← shaped by →    C-XXX        │
│          NFR-XXX ← sourced from → FR/SC       │
│          IP-XXX ← provisions →   C/NFR        │
│          INT-XXX · DS-XXX (thin declarations) │
└─────────────────────────────────────────────┘
        │
        ▼  (downstream — authored by the design skills, not here)
   Entities + per-attribute sensitivity → mochiko:patterns-entity-modeling
   Endpoints + schemas + x-integration   → mochiko:patterns-api-contracts
```

The constraint layer feeds the design layer; this reference covers the constraint half of the web (the box above). The design-artifact traceability — entity→FR, schema→entity, endpoint→integration, attribute→sensitivity — is authored and traced inside the design skills, and the *cross-artifact consistency between the two layers* is graded by the independent design-phase reviewer (`mochiko:review-plan-artifacts`), not self-asserted here. See **Design-layer traceability** below.

## Constraint-Layer Cross-References

### C → FR/SC (Business Traceability)

Every constraint MUST name what it comes from — a business requirement, a success criterion, or a real infrastructure fact.

**Pattern** (the source rides the statement line — see ARTIFACT-TEMPLATES.md):
```markdown
## C-001: Existing PostgreSQL Infrastructure

**infrastructure · MUST · source:** production fact (PostgreSQL 15 cluster in place) — …
```

**Validation rule:** Scan all C entries. Each MUST carry a source on its statement line — an FR-XXX / SC-XXX reference, or a named infrastructure or regulatory fact. A constraint with no source is a preference wearing a constraint's clothes.

### C → D (Constraint-Decision Link)

Constraints SHOULD reference the decisions they shaped. Decisions MUST reference the constraints that shaped them.

**Pattern (Constraint side):**
```markdown
## C-001: Existing PostgreSQL Infrastructure

**Impact:** eliminates NoSQL options · shapes D-001 (database choice)
```

**Pattern (Decision side):**
```markdown
## D-001: Primary Database

**Context** (…) · **Shaped by:** C-001 (existing PostgreSQL cluster)
```

**Validation rule:** Every D-XXX entry MUST have a non-empty `Shaped by` reference. Every C-XXX entry SHOULD reference at least one D-XXX in its Impact line.

> The C↔D link is the *traceability*; the decision *technique* that fills each D-XXX (evaluating alternatives, scoring trade-offs, ADR depth) is owned by `mochiko:patterns-technical-decisions`.

### NFR → Source (Justification)

Every NFR MUST trace to a business source that justifies the target — the FR-XXX or SC-XXX whose promise the number serves.

**Pattern:**
```markdown
## NFR-001: API Response Latency

**performance · source:** FR-001 (real-time interaction; "instant feedback" expectation, spec §3.2) — …
```

**Validation rule:** No NFR without a source on its statement line. Targets pulled from thin air are not requirements — they are guesses.

### C/NFR → IP (Infrastructure Provisioning)

Every constraint or NFR that implies platform work SHOULD reference the IP-XXX item that provisions it.

**Pattern:**
```markdown
## IP-001: Compute Provisioning

**compute · MUST · source:** C-001 (existing AWS environment), NFR-003 (10k concurrent users) — …
```

**Validation rule:** Every constraint implying platform provisioning has a corresponding IP-XXX; every IP-XXX traces back to a C-XXX or NFR-XXX on its statement line.

### INT/DS → Downstream Home (Declaration Closure)

Every INT-XXX and DS-XXX declaration names the downstream artifact that authors its structure. The declaration records *that* the concern exists; the pointer records *where* it gets built out.

**Pattern:**
```markdown
## INT-004: Identity Provider

**integration · criticality: hard · source:** FR-001 — boundary authored on POST /auth/token (`mochiko:patterns-api-contracts`)
```

**Validation rule:** Every INT-XXX resolves to an endpoint carrying `x-integration`; every DS-XXX resolves to at least one classified attribute in `data-model.md`. A declaration with no downstream boundary is an outage waiting to happen; a declaration with no classification is ungoverned sensitive data.

## Dependency Chains

Some traceability relationships form chains that must be consistent end-to-end. The constraint chain (FR/SC → C/D/NFR/IP) is owned here; its design tail (Entity, Endpoint, sensitivity, x-integration) is authored downstream in the design skills.

### Full Traceability Chain

```
FR-001 (business: "users can sign in")
  ├── C-001 (constraint: must use existing identity provider)
  ├── D-002 (decision: JWT with refresh tokens)
  ├── NFR-001 (quality: p95 < 200ms)
  ├── INT-004 (declaration: external identity provider, hard criticality)
  └── IP-002 (provisioning: identity-provider connectivity)
        └── [design tail, authored downstream]
              Entity: User (+ per-attribute sensitivity)  → patterns-entity-modeling
              Endpoint: POST /auth/token (+ x-integration) → patterns-api-contracts
```

**Reading this chain:** Business requirement FR-001 is constrained by C-001, informed by decision D-002, must meet NFR-001 latency, depends on the INT-004 integration, and requires IP-002 provisioning. The downstream design tail (the User entity with classified attributes, exposed via an endpoint that integrates with an external identity provider) is authored by the design skills, tracing back up to this constraint chain.

### Constraint Impact Chain

```
C-002 (regulatory: GDPR Art. 17 right to erasure)
  ├── D-004 (decision: soft-delete with 30-day purge)
  ├── DS-002 (declaration: user profile data is Restricted)
  └── IP-003 (provisioning: scheduled purge job)
```

**Reading this chain:** Regulatory constraint C-002 drives decision D-004, declares the sensitivity of the data it governs, and necessitates the IP-003 provisioning for the purge job. Downstream, this constraint also drives the entity retention policy and a DELETE endpoint — authored in the design skills.

## Completeness Validation (Constraint-Layer Artifacts)

### Constraint Sourcing (C → FR/SC/fact)

Check that every constraint is grounded, not asserted.

**Procedure:**
1. List all C-XXX entries
2. For each, verify the statement line carries an FR-XXX / SC-XXX reference or a named infrastructure or regulatory fact
3. Flag any constraint with no source

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
2. For each, verify: target (numeric), measurement method, source (FR-XXX / SC-XXX)
3. Flag any NFR missing elements

### Infrastructure Coverage (C/NFR → IP)

Verify every platform-implying constraint or NFR is provisioned.

**Procedure:**
1. List all C-XXX and NFR-XXX entries that imply platform work
2. For each, verify a corresponding IP-XXX exists with a matching `Source`
3. Flag any platform-implying constraint with no IP-XXX

### Declaration Closure (INT/DS → downstream)

Verify every thin declaration reaches its authoring home.

**Procedure:**
1. List all INT-XXX and DS-XXX entries
2. For each, verify the named downstream artifact exists and carries the structure (an `x-integration` boundary, a classified attribute)
3. Flag any declaration whose downstream home is absent or unnamed

## Cross-Artifact Consistency Rules (Layer-Internal)

These are producer **self-checks** on the constraint-layer artifacts before finalizing. They are not the independent gate — the cross-artifact consistency *grade* is owned by `mochiko:review-plan-artifacts` (a different agent).

### Rule 1: ID References Must Resolve

Every cross-reference (C-XXX, D-XXX, NFR-XXX, IP-XXX, INT-XXX, DS-XXX) appearing in any constraint-layer artifact MUST correspond to an actual entry in the appropriate artifact file.

**Violation example:** D-005 references "C-003" but constraints-and-decisions.md only has C-001 and C-002.

### Rule 2: Bidirectional References Should Match

If D-005 lists C-001 as a shaping constraint, then C-001 SHOULD list D-005 in its Impact section. Mismatches indicate incomplete traceability.

### Rule 3: No Contradictory Constraints

If C-001 says "must use existing PostgreSQL" and C-004 says "must support any SQL database," there is a contradiction. Constraints restrict — and two constraints that restrict incompatibly cannot both hold.

## Design-layer traceability (authored downstream — referenced, not owned here)

The traceability rules that govern the **design** artifacts are owned by the skills that author those artifacts; this skill points at them rather than restating them:

| Design-layer rule | Owner |
|-------------------|-------|
| Entity → FR (every entity traces to a business requirement) | `mochiko:patterns-entity-modeling` |
| Attribute → Sensitivity (every PII/sensitive attribute is classified) | `mochiko:patterns-entity-modeling` |
| Endpoint → User Action (every action maps to an endpoint) | `mochiko:patterns-api-contracts` |
| Schema → Entity (response schemas match entity attributes) | `mochiko:patterns-api-contracts` |
| Endpoint → Integration (external-system endpoints carry `x-integration`) | `mochiko:patterns-api-contracts` |
| Cross-artifact consistency / feasibility *grading* (schema-entity alignment, NFR-vs-design feasibility, sensitivity-response alignment, contradiction detection) | the independent design-phase reviewer (`mochiko:review-plan-artifacts`) |

The constraint-layer artifacts authored here are the **upstream anchor** every one of those design rules traces back to: keep C/D/NFR/IP/INT/DS IDs stable and resolvable so the downstream chains hold.

## Traceability Matrix Template

For complex features, produce a summary matrix. The constraint-layer producer fills the FR / Constraints / Decisions / NFRs columns; the Entities and Endpoints columns are completed downstream when the design artifacts are authored.

```markdown
## Traceability Matrix

| FR | Constraints | Decisions | NFRs | Entities | Endpoints |
|----|-------------|-----------|------|----------|-----------|
| FR-001 | C-001 | D-002 | NFR-001, NFR-004 | User | POST /auth/token |
| FR-002 | - | D-001 | NFR-001 | Order, Payment | POST /orders |
| FR-003 | C-002 | D-004 | NFR-002 | User, Order | DELETE /users/{id} |
```

This matrix provides a single view of the entire traceability web from business requirements through design, making gaps immediately visible.
