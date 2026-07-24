# Artifact Templates

Templates and field definitions for the **three analysis artifacts this skill authors**: `requirements.md` (TR-XXX), `constraints-and-decisions.md` (C-XXX / D-XXX / IP-XXX), and `nfrs.md` (NFR-XXX). All three follow the deliverable envelope in [`artifact-format.md`](../../../templates/artifact-format.md): the statement carries the content (no separate Description paragraph), entries are one line each, upstream text is cited by ID and never re-quoted, and each artifact's summary table is its **ID index** — the coverage surface reviewers verify against.

> **Design-artifact templates live with their canonical owners.** The `data-model.md` template (entities + per-attribute data-sensitivity taxonomy) is owned by `mochiko:patterns-entity-modeling`; the `contracts/api.yaml` template (endpoints, schemas, and the `x-integration` boundary extension) and the integration `quickstart.md` are owned by `mochiko:patterns-api-contracts`. This bundle declares the analysis requirements those design artifacts build on — it does not restate their templates.

## 1. Technical Requirements (requirements.md)

### Document Template

```markdown
# Technical Requirements: {feature_id}

> Technical decomposition of business functional requirements.

## Traceability Summary  *(the ID index)*

| Source FR | Technical Requirements | Coverage |
|-----------|----------------------|----------|
| FR-001 | TR-001, TR-002, TR-003 | Full |
| FR-002 | TR-004, TR-005 | Full |

---

## TR-001: [Descriptive Title]

**FR-001 · MUST** — System MUST [technical capability in technology-agnostic terms; the statement is the description].

**Criteria:**
- [Testable condition, one line]
- [Testable condition, one line]

**Deps:** C-001 · NFR-002  *(omit if none)*

---
```

### Field Definitions

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | TR-XXX | Sequential, three-digit padded, no gaps |
| Title | Yes | Free text | Descriptive, concise |
| Source | Yes | FR-XXX reference(s) | On the statement line; must reference existing FR(s) |
| Priority | Yes | MUST / SHOULD / MAY | RFC 2119 keyword, on the statement line |
| Statement | Yes | One-to-two lines | Technology-agnostic; WHAT, not HOW — no separate Description paragraph |
| Criteria | Yes | Bullet list, one line each | Each item independently testable |
| Deps | No | ID references | C-XXX, NFR-XXX, other TR-XXX — cited by ID, never re-quoted |

### Decomposition Examples

**Business FR:** "Users must be able to sign in to their account" (FR-001)

| TR | Title | Aspect |
|----|-------|--------|
| TR-001 | Authentication Flow | Credential validation, error handling |
| TR-002 | Session Management | Token issuance, expiration, refresh |
| TR-003 | Account Lockout | Brute-force protection, lockout thresholds |
| TR-004 | Authentication Audit | Login attempt logging, anomaly flags |

Each TR addresses a distinct technical concern the single business FR implies but does not state.

### Writing Criteria

Good criteria are independently testable (pass/fail), technology-agnostic, and cover success, failure, and edge cases — one line each:

```markdown
## TR-001: Authentication Flow

**FR-001 · MUST** — System MUST validate credentials and establish an authenticated session.

**Criteria:**
- Valid credentials result in an authenticated session
- Invalid credentials return a generic error (no credential-type leakage)
- Expired accounts cannot authenticate
- Authentication completes within the NFR-001 latency target
- All attempts logged per TR-004
```

---

## 2. Constraints and Decisions (constraints-and-decisions.md)

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

**Impact:** eliminates [design choice A] · requires [consideration B] · affects TR-001, TR-003 · shapes D-001
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
| Impact | Yes | One line, `·`-separated (or a short bullet list) | What this eliminates or forces; references to affected TR-XXX and shaped D-XXX |
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

## NFR Summary  *(the ID index)*

| ID | Category | Target | Source |
|----|----------|--------|--------|
| NFR-001 | performance | p95 < 200ms | FR-001 (user expects instant feedback) |
| NFR-002 | availability | 99.9% monthly | Business SLA commitment |

---

## NFR-001: [Descriptive Title]

**performance · source:** [business requirement or stakeholder justifying the target] — [the quality attribute, one line].

**Target:** [specific, measurable numeric threshold]
**Measured:** [tool + conditions + frequency — compact; a multi-condition method uses a short bullet list]
**Applies to:** TR-001 · TR-005  *(omit if it applies globally)*

---
```

### Field Definitions

| Field | Required | Format | Rules |
|-------|----------|--------|-------|
| ID | Yes | NFR-XXX | Sequential, three-digit padded, no gaps |
| Title | Yes | Free text | Descriptive, concise |
| Category | Yes | performance / availability / scalability / security / usability / maintainability | On the statement line; exactly one category |
| Source | Yes | Free text | On the statement line; business requirement, SLA, or stakeholder justifying the target |
| Requirement | Yes | One line | The quality attribute — the statement IS the description |
| Target | Yes | Numeric | Specific, measurable threshold |
| Measured | Yes | Compact line or short list | Tool, conditions, frequency of measurement |
| Applies to | No | TR-XXX references | Which technical requirements this NFR constrains |

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

Every target's `Measured:` line names **what tool**, **under what conditions**, and **how frequently**. Compact example:

```markdown
## NFR-001: API Response Latency

**performance · source:** FR-001 (real-time interaction expectation) — API responses feel instantaneous under production load.

**Target:** p95 < 200ms, p99 < 500ms
**Measured:** APM, rolling 24h windows, continuous — at 1,000 concurrent users (70% read / 20% write / 10% search); excludes maintenance windows and bulk imports
**Applies to:** TR-001 · TR-005
```

---

## ID Numbering Rules (All Artifacts)

All artifact types follow the same numbering conventions:

1. **Three-digit padding:** TR-001, not TR-1
2. **Sequential, no gaps:** TR-001, TR-002, TR-003 (never TR-001, TR-003)
3. **Prefix identifies type:** TR- / C- / D- / NFR- / IP-
4. **Cross-references use full ID:** "See TR-001" not "See requirement 1"
5. **Grouping by concern:** Related items should be sequential where possible
