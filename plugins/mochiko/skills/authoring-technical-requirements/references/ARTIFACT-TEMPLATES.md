# Artifact Templates

Templates and field definitions for the **two constraint-layer surfaces this skill authors**: `constraints-and-decisions.md` (C-XXX / D-XXX / IP-XXX plus the thin INT-XXX / DS-XXX declarations), and the NFR-XXX grammar whose rows are homed on the architecture store's concern rows (§2 — no `nfrs.md` file exists). Both follow the deliverable envelope in [`artifact-format.md`](../../../templates/artifact-format.md): the statement carries the content (no separate Description paragraph), entries are one line each, upstream text is cited by ID and never re-quoted, and each artifact's summary table is its **ID index** — the coverage surface reviewers verify against. Register: `full` per that envelope's rule 11 — with every ID, numeric target and constraint clause a never-compress item, and compression stopping wherever it would make a requirement ambiguous.

> **Design-artifact templates live with their canonical owners.** The `data-model.md` template (entities + per-attribute data-sensitivity taxonomy) is owned by `mochiko:patterns-entity-modeling`; the `contracts/api.yaml` template (endpoints, schemas, and the `x-integration` boundary extension) and the integration `quickstart.md` are owned by `mochiko:patterns-api-contracts`. This bundle declares the analysis requirements those design artifacts build on — it does not restate their templates.

## 1. Constraints and Decisions (constraints-and-decisions.md)

### Document Template

```markdown
# Constraints and Decisions: {feature_id}

> Hard boundaries and technology choices that shape the implementation.

## Part 1: Hard Constraints

### Constraint Summary  *(the ID index)*

| ID | Type | Source | Severity |
|----|------|--------|----------|
| C-001 | infrastructure | Existing production environment | blocking |
| C-002 | regulatory | GDPR Article 17 | blocking |

---

### C-001: [Descriptive Title]

**infrastructure · blocking · source:** [where it originates] — [the boundary stated as a one-to-two-line fact, not a preference].

**Impact:** eliminates [design choice A] · requires [consideration B] · shapes D-001
**Verify:** [how to confirm it still applies — one line]  *(omit if self-evident)*

---

## Part 2: Technology Decisions

> **Decision *technique* is owned by `mochiko:patterns-technical-decisions`.** The field schema below is the artifact slot a decision lands in; how to evaluate alternatives, score trade-offs, and set ADR depth lives in that skill. Fill these fields with the result; do not restate the evaluation method here.

### Decision Summary  *(the ID index)*

| ID | Decision | Choice | Shaped By |
|----|----------|--------|-----------|
| D-001 | Primary database | PostgreSQL 15 | C-001 |
| D-002 | Auth mechanism | JWT with refresh tokens | C-003 |

---

### D-001: [Decision Title]

**Context** ([one-to-two lines: the problem needing a decision]) · **Shaped by:** C-001 · NFR-001

| Option | Pros | Cons |
|--------|------|------|
| [Option A] | [one line] | [one line] |
| [Option B] | [one line] | [one line] |

**Choice:** [selected option] — **Rationale** (≤ 3 lines): [WHY this choice — the reasoning, not a restatement of the choice].
**Consequences:** [trade-off 1 accepted] · [trade-off 2] · [future consideration]
**Governance alignment:** [one line — only when a project principle applies; omit otherwise]

---

> **No structural-decision subsection.** Topology choices — component boundaries, interaction
> style, responsibility placement — are recorded in the architecture store's delta, whose ruling
> is its own decision record. They never appear as D-XXX rows here. Every D-XXX in this artifact
> has one origin: the design-phase author (or, for a build-time decision, the gated
> `baseline-delta.md` path).

---

## Part 3: Infrastructure Requirements

### Infrastructure Summary  *(the ID index)*

| ID | Type | Source Constraint | Priority |
|----|------|-------------------|----------|
| IP-001 | compute | C-001 | MUST |
| IP-002 | ci-cd | C-004, NFR-002 | MUST |

---

### IP-001: [Descriptive Title]

**compute · MUST · source:** C-001, NFR-003 — [what must be provisioned, one-to-two lines; WHAT, not HOW].

**Criteria:**
- [Verifiable condition, one line]

**Deps:** IP-002  *(omit if none)*

---

## Part 4: Declarations (INT-XXX / DS-XXX)

### Declaration Summary  *(the ID index)*

| ID | Kind | Source | Downstream home |
|----|------|--------|-----------------|
| INT-001 | integration | FR-001 | POST /auth/token (`x-integration`) |
| DS-001 | sensitivity | FR-004 | User.email (Confidential) |

---

### INT-001: [External System]

**integration · criticality: hard · source:** FR-001 — the feature MUST integrate with [system]; its unavailability is [criticality].

**Authored downstream:** [endpoint carrying the `x-integration` boundary] (`mochiko:patterns-api-contracts`)

---

### DS-001: [Data Class]

**sensitivity · source:** FR-004 — the feature handles [data], which is sensitive and MUST be classified and protected.

**Authored downstream:** [entity.attribute] (`mochiko:patterns-entity-modeling`)

---
```

### Field Definitions — Constraints

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | C-XXX | Sequential, three-digit padded, no gaps |
| Title | Yes | Free text | Descriptive, concise |
| Type | Yes | infrastructure / compatibility / regulatory / migration / organizational | On the statement line; exactly one type |
| Source | Yes | Free text | On the statement line; traceable origin — system, regulation, contract, team |
| Severity | Yes | blocking / significant / minor | On the statement line |
| Statement | Yes | One-to-two lines | States the boundary as fact — no separate Description paragraph |
| Impact | Yes | One line, `·`-separated (or a short bullet list) | What this eliminates or forces; references to the D-XXX it shapes |
| Verify | No | One line | How to confirm the constraint still applies |

### Field Definitions — Decisions

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | D-XXX | Sequential, three-digit padded, no gaps |
| Title | Yes | Free text | Descriptive, concise |
| Context | Yes | One-to-two lines | The problem that needed solving |
| Shaped By | Yes | C-XXX / NFR-XXX references | On the context line; constraints and NFRs that narrowed options |
| Options | Yes | Table, one line per option | Minimum 2 alternatives with pros/cons |
| Choice | Yes | Free text | Selected option |
| Rationale | Yes | ≤ 3 lines | WHY, not just WHAT |
| Consequences | Yes | One line, `·`-separated (or a short bullet list) | Trade-offs accepted, future considerations |
| Governance alignment | No | One line | Only when a project principle applies — omit otherwise |

### Field Definitions — Infrastructure Requirements

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | IP-XXX | Sequential, three-digit padded |
| Title | Yes | Free text | Descriptive, concise |
| Type | Yes | compute / networking / storage / ci-cd / monitoring / security / environment-config | On the statement line; exactly one |
| Source | Yes | C-XXX / NFR-XXX refs | On the statement line; constraints/NFRs that necessitate this |
| Priority | Yes | MUST / SHOULD / MAY | RFC 2119, on the statement line |
| Statement | Yes | One-to-two lines | WHAT to provision, not HOW — no separate Description paragraph |
| Criteria | Yes | Bullet list, one line each | Independently verifiable |
| Deps | No | IP-XXX refs | Other infra items this depends on |

### Field Definitions — Declarations

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | INT-XXX / DS-XXX | Sequential per prefix, three-digit padded, no gaps |
| Kind | Yes | integration / sensitivity | On the statement line |
| Source | Yes | FR-XXX / SC-XXX reference | On the statement line; the business promise the declaration serves |
| Criticality | INT only | hard / degraded / optional | How the feature behaves when the external system is unavailable |
| Statement | Yes | One line | *That* the concern exists — never its downstream structure |
| Authored downstream | Yes | Artifact + owning skill | Where the boundary or classification is built out; a declaration with no named home is incomplete |

### Infrastructure Types

| Type | Scope |
|------|-------|
| compute | Containers, serverless, VMs, orchestration |
| networking | DNS, load balancers, VPN, firewall rules |
| storage | Databases, object storage, caches (provisioning, not schema) |
| ci-cd | Build pipelines, deployment automation, environments |
| monitoring | APM, logging, alerting, health checks |
| security | IAM, certificates, secrets management |
| environment-config | Environment variables, feature flags, config management |

### Constraint Types

| Type | Definition | Examples |
|------|------------|---------|
| **infrastructure** | Existing systems, platforms, or environments that cannot change | "Production database is PostgreSQL 15 on AWS RDS" |
| **compatibility** | Existing consumers, APIs, or interfaces that must continue working | "Mobile app v2.x expects REST API v1 responses" |
| **regulatory** | Laws, regulations, or compliance mandates | "GDPR requires right-to-erasure within 30 days" |
| **migration** | Deployment, transition, or coexistence requirements | "Zero-downtime migration required; both old and new schemas must coexist" |
| **organizational** | Team, process, or business constraints | "Maximum 3-person team for initial implementation" |

### Distinguishing Constraints from Preferences

| Statement | Constraint? | Why |
|-----------|-------------|-----|
| "Must use PostgreSQL" | Maybe | Only if existing production infrastructure mandates it |
| "Must use React" | Probably not | Unless existing codebase and team skills make alternatives infeasible |
| "Must support IE11" | Yes | If contractual obligation exists |
| "Must deploy to AWS" | Yes | If organization has AWS-only policy |
| "Should use TypeScript" | No | Preference, not hard boundary |

**Test:** "If we violated this, what concrete thing would break or what rule would we violate?" If the answer is vague, it is a preference, not a constraint.

---

## 2. Non-Functional Requirements (NFR-XXX — homed on architecture-store concern rows)

**There is no `nfrs.md` document template here.** An NFR-XXX lives as fields on its concern row in
the architecture store, so a concern has one home — stance, pattern, targets, as-built, drift
together. The **row shape** is the store's (`mochiko-cli template architecture-store`, or Read
`plugins/mochiko/schemas/architecture-store.yaml` raw when the binary is absent); what follows is
the **grammar** this skill owns and the store row carries: the required fields, the categories,
and what a measurement method must name. A new or changed target reaches the store as part of a
design-time store delta, written at the user's sign-off — never edited into ruled truth in place.

The trace chain resolves to the business source: `FR-XXX / SC-XXX → NFR-XXX`, and `Applies to:`
cites the C-XXX or IP-XXX the target constrains.

### Field Definitions

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | NFR-XXX | Sequential, three-digit padded, no gaps |
| Title | Yes | Free text | Descriptive, concise |
| Category | Yes | performance / availability / scalability / security / usability / maintainability | On the statement line; exactly one category |
| Source | Yes | FR-XXX / SC-XXX reference | On the statement line; the business promise the target serves — an SLA or stakeholder gloss may ride alongside, never instead |
| Requirement | Yes | One line | The quality attribute — the statement IS the description |
| Target | Yes | Numeric | Specific, measurable threshold |
| Measured | Yes | Compact line or short list | Tool, conditions, frequency of measurement |
| Applies to | No | C-XXX / IP-XXX references | Which constraints or provisioning items this NFR constrains |

### NFR Categories with Examples

| Category | Bad (Vague) | Good (Measurable) |
|----------|-------------|-------------------|
| **performance** | "System must be fast" | "p95 response time < 200ms under 1000 concurrent users, measured by APM" |
| **availability** | "System must be reliable" | "99.9% uptime measured monthly, excluding scheduled maintenance" |
| **scalability** | "Must handle growth" | "Must support 10,000 concurrent users with linear resource scaling to 50,000" |
| **security** | "Must be secure" | "Zero plaintext PII in logs; all data classified confidential+ encrypted AES-256-equivalent at rest" |
| **usability** | "Must be easy to use" | "New users complete primary workflow within 3 minutes without documentation" |
| **maintainability** | "Must be maintainable" | "Mean time to deploy hotfix < 2 hours from commit to production" |

### Writing Measurement Methods

Every target's `Measured:` line names **what tool**, **under what conditions**, and **how frequently**. Compact example — the fields as they ride a concern row, not as a document section:

```markdown
NFR-001 — API Response Latency

**performance · source:** FR-001 (real-time interaction expectation) — API responses feel instantaneous under production load.

**Target:** p95 < 200ms, p99 < 500ms
**Measured:** APM, rolling 24h windows, continuous — at 1,000 concurrent users (70% read / 20% write / 10% search); excludes maintenance windows and bulk imports
**Applies to:** C-001 · IP-002
```

---

## ID Numbering Rules (All Artifacts)

All artifact types follow the same numbering conventions:

1. **Three-digit padding:** C-001, not C-1
2. **Sequential, no gaps:** C-001, C-002, C-003 (never C-001, C-003)
3. **Prefix identifies type:** C- / D- / NFR- / IP- / INT- / DS-
4. **Cross-references use full ID:** "See C-001" not "See constraint 1"
5. **Grouping by concern:** Related items should be sequential where possible
