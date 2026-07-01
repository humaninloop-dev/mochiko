# Artifact Templates

Complete templates and field definitions for the **three analysis artifacts this skill authors**: `requirements.md` (TR-XXX), `constraints-and-decisions.md` (C-XXX / D-XXX / IP-XXX), and `nfrs.md` (NFR-XXX). Each template shows the full document structure with all required fields.

> **Design-artifact templates live with their canonical owners.** The `data-model.md` template (entities + per-attribute data-sensitivity taxonomy) is owned by `mochiko:patterns-entity-modeling`; the `contracts/api.yaml` template (endpoints, schemas, and the `x-integration` boundary extension) and the integration `quickstart.md` are owned by `mochiko:patterns-api-contracts`. This bundle declares the analysis requirements those design artifacts build on — it does not restate their templates.

## 1. Technical Requirements (requirements.md)

### Document Template

```markdown
# Technical Requirements: {feature_id}

> Technical decomposition of business functional requirements.

---

## Traceability Summary

| Source FR | Technical Requirements | Coverage |
|-----------|----------------------|----------|
| FR-001 | TR-001, TR-002, TR-003 | Full |
| FR-002 | TR-004, TR-005 | Full |
| FR-003 | TR-006 | Full |

---

## TR-001: [Descriptive Title]

**Source:** FR-001
**Priority:** MUST

**Description:**
System MUST [technical capability described in technology-agnostic terms].

**Acceptance Criteria:**
- [ ] [Testable condition 1]
- [ ] [Testable condition 2]
- [ ] [Testable condition 3]

**Dependencies:**
- C-001 (constraint that affects this requirement)
- NFR-002 (quality attribute that applies)

---
```

### Field Definitions

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | TR-XXX | Sequential, three-digit padded, no gaps |
| Title | Yes | Free text | Descriptive, concise |
| Source | Yes | FR-XXX reference(s) | Must reference existing FR(s) |
| Priority | Yes | MUST / SHOULD / MAY | RFC 2119 keyword |
| Description | Yes | Paragraph | Technology-agnostic; describes WHAT, not HOW |
| Acceptance Criteria | Yes | Checkbox list | Each item independently testable |
| Dependencies | No | ID references | Links to C-XXX, NFR-XXX, other TR-XXX |

### Decomposition Examples

**Business FR:** "Users must be able to sign in to their account" (FR-001)

**Technical decomposition:**

| TR | Title | Aspect |
|----|-------|--------|
| TR-001 | Authentication Flow | Credential validation, error handling |
| TR-002 | Session Management | Token issuance, expiration, refresh |
| TR-003 | Account Lockout | Brute-force protection, lockout thresholds |
| TR-004 | Authentication Audit | Login attempt logging, anomaly flags |

Each TR addresses a distinct technical concern that the single business FR implies but does not state.

### Writing Acceptance Criteria

**Good acceptance criteria are:**
- Independently testable (pass/fail, no ambiguity)
- Technology-agnostic (no specific tools or implementations)
- Complete (cover success, failure, and edge cases)

**Examples:**

```markdown
**TR-001: Authentication Flow**

**Acceptance Criteria:**
- [ ] Valid credentials result in authenticated session
- [ ] Invalid credentials return generic error (no credential-type leakage)
- [ ] Expired accounts cannot authenticate
- [ ] Authentication completes within NFR-001 latency target
- [ ] All authentication attempts logged per TR-004
```

```markdown
**TR-007: Data Export**

**Acceptance Criteria:**
- [ ] Export includes all user-owned data elements
- [ ] Export format is machine-readable and portable
- [ ] Export request completes within NFR-003 time limit
- [ ] Export excludes system-internal data (classified Internal or below)
- [ ] Export availability notified through existing notification channel
```

---

## 2. Constraints and Decisions (constraints-and-decisions.md)

### Document Template

```markdown
# Constraints and Decisions: {feature_id}

> Hard boundaries and technology choices that shape the implementation.

---

## Part 1: Hard Constraints

### Constraint Summary

| ID | Type | Source | Severity |
|----|------|--------|----------|
| C-001 | infrastructure | Existing production environment | Hard |
| C-002 | regulatory | GDPR Article 17 | Hard |
| C-003 | compatibility | Legacy API consumers | Hard |
| C-004 | migration | Zero-downtime deployment requirement | Hard |

---

### C-001: [Descriptive Title]

**Type:** infrastructure
**Source:** [Where this constraint originates -- existing system, regulation, contract]
**Severity:** Hard

**Description:**
[The hard boundary, stated as a fact, not a preference.]

**Impact:**
- Eliminates [design choice A] as an option
- Requires [design consideration B]
- Affects TR-001, TR-003
- Shapes D-001

**Verification:**
[How to confirm this constraint still applies -- who to ask, what to check.]

---

## Part 2: Technology Decisions

> **Decision *technique* is owned by `mochiko:patterns-technical-decisions`.** The field schema below is the artifact slot a decision lands in; how to evaluate alternatives, score trade-offs, and set ADR depth lives in that skill. Fill these fields with the result; do not restate the evaluation method here.

### Decision Summary

| ID | Decision | Choice | Shaped By |
|----|----------|--------|-----------|
| D-001 | Primary database | PostgreSQL 15 | C-001 |
| D-002 | Auth mechanism | JWT with refresh tokens | C-003 |
| D-003 | Caching strategy | Redis with TTL | NFR-001 |

---

### D-001: [Decision Title]

**Context:**
[What problem needed solving and why a decision was required.]

**Shaped By:**
- C-001 (constraint that narrowed options)
- NFR-001 (quality target that influenced choice)

**Options Considered:**

| Option | Pros | Cons |
|--------|------|------|
| [Option A] | [advantages] | [disadvantages] |
| [Option B] | [advantages] | [disadvantages] |
| [Option C] | [advantages] | [disadvantages] |

**Choice:** [Selected option]

**Rationale:**
[WHY this choice — not just restating the choice, but the reasoning that led to it.]

**Consequences:**
- [Trade-off 1 accepted]
- [Trade-off 2 accepted]
- [Future consideration]

**Constitution Alignment:**
[How this decision aligns with project principles, if applicable.]

---

## Part 3: Infrastructure Requirements

### Infrastructure Summary

| ID | Type | Source Constraint | Priority |
|----|------|-------------------|----------|
| IP-001 | compute | C-001 | MUST |
| IP-002 | ci-cd | C-004, NFR-002 | MUST |
| IP-003 | monitoring | NFR-001 | SHOULD |

---

### IP-001: [Descriptive Title]

**Type:** compute
**Source:** C-001, NFR-003
**Priority:** MUST

**Description:**
[What must be provisioned or configured — stated as a requirement, not implementation.]

**Acceptance Criteria:**
- [ ] [Verifiable condition 1]
- [ ] [Verifiable condition 2]

**Dependencies:**
- IP-002 (needs CI/CD before deployment)

---
```

### Field Definitions — Constraints

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | C-XXX | Sequential, three-digit padded, no gaps |
| Title | Yes | Free text | Descriptive, concise |
| Type | Yes | infrastructure / compatibility / regulatory / migration / organizational | Exactly one type |
| Source | Yes | Free text | Traceable origin -- system, regulation, contract, team |
| Severity | Yes | Hard | All constraints are hard boundaries by definition |
| Description | Yes | Paragraph | States the boundary as fact |
| Impact | Yes | Bullet list | What design choices this eliminates or forces; references to D-XXX |
| Verification | No | Free text | How to confirm constraint still applies |

### Field Definitions — Decisions

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | D-XXX | Sequential, three-digit padded, no gaps |
| Title | Yes | Free text | Descriptive, concise |
| Context | Yes | Paragraph | Problem that needed solving |
| Shaped By | Yes | C-XXX / NFR-XXX references | Constraints and NFRs that narrowed options |
| Options Considered | Yes | Table | Minimum 2 alternatives with pros/cons |
| Choice | Yes | Free text | Selected option |
| Rationale | Yes | Paragraph | WHY, not just WHAT |
| Consequences | Yes | Bullet list | Trade-offs accepted, future considerations |
| Constitution Alignment | No | Paragraph | How choice follows project principles |

### Field Definitions — Infrastructure Requirements

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | IP-XXX | Sequential, three-digit padded |
| Title | Yes | Free text | Descriptive, concise |
| Type | Yes | compute / networking / storage / ci-cd / monitoring / security / environment-config | Exactly one |
| Source | Yes | C-XXX / NFR-XXX refs | Constraints/NFRs that necessitate this |
| Priority | Yes | MUST / SHOULD / MAY | RFC 2119 |
| Description | Yes | Paragraph | WHAT to provision, not HOW |
| Acceptance Criteria | Yes | Checkbox list | Independently verifiable |
| Dependencies | No | IP-XXX refs | Other infra items this depends on |

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

## 3. Non-Functional Requirements (nfrs.md)

### Document Template

```markdown
# Non-Functional Requirements: {feature_id}

> Measurable quality attributes with specific targets.

---

## NFR Summary

| ID | Category | Target | Source |
|----|----------|--------|--------|
| NFR-001 | performance | p95 < 200ms | FR-001 (user expects instant feedback) |
| NFR-002 | availability | 99.9% monthly | Business SLA commitment |
| NFR-003 | scalability | 10,000 concurrent users | Growth projection Y1 |
| NFR-004 | security | Zero plaintext PII in logs | Compliance requirement |

---

## NFR-001: [Descriptive Title]

**Category:** performance
**Source:** [Business requirement or stakeholder that justifies this target]

**Requirement:**
[The quality attribute in plain language.]

**Target:**
[Specific, measurable numeric threshold.]

**Measurement Method:**
[How to verify the target is met -- tools, conditions, frequency.]

**Applies To:**
- TR-001 (authentication flow must meet this latency)
- TR-005 (search results must meet this latency)

---
```

### Field Definitions

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | NFR-XXX | Sequential, three-digit padded, no gaps |
| Title | Yes | Free text | Descriptive, concise |
| Category | Yes | performance / availability / scalability / security / usability / maintainability | Exactly one category |
| Source | Yes | Free text | Business requirement, SLA, or stakeholder justifying the target |
| Requirement | Yes | Paragraph | Plain language quality attribute |
| Target | Yes | Numeric | Specific, measurable threshold |
| Measurement Method | Yes | Paragraph | Tools, conditions, frequency of measurement |
| Applies To | No | TR-XXX references | Which technical requirements this NFR constrains |

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

Every NFR target needs a measurement method that specifies:

1. **What tool or technique** measures it
2. **Under what conditions** the measurement is valid
3. **How frequently** measurement occurs

**Example:**

```markdown
## NFR-001: API Response Latency

**Target:** p95 < 200ms, p99 < 500ms

**Measurement Method:**
Measured by application performance monitoring (APM) under the following conditions:
- Load: 1,000 concurrent authenticated users
- Operation mix: 70% reads, 20% writes, 10% searches
- Measurement window: rolling 24-hour periods
- Excludes: scheduled maintenance windows, bulk import operations
- Frequency: Continuous monitoring with daily reporting
```

---

## ID Numbering Rules (All Artifacts)

All artifact types follow the same numbering conventions:

1. **Three-digit padding:** TR-001, not TR-1
2. **Sequential, no gaps:** TR-001, TR-002, TR-003 (never TR-001, TR-003)
3. **Prefix identifies type:** TR- / C- / D- / NFR- / IP-
4. **Cross-references use full ID:** "See TR-001" not "See requirement 1"
5. **Grouping by concern:** Related items should be sequential where possible
