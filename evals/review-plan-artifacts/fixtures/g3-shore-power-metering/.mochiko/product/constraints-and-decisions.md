# Constraints and Decisions: product baseline (Halyard)

> Hard boundaries and technology choices that shape every feature. Baseline as of FEAT-019
> landing (2026-08-27). Feature deltas live beside this file under
> `.mochiko/features/FEAT-XXX/constraints-and-decisions.md` and continue the numbering.

## Constraints

### Constraint Summary  *(the ID index)*

| ID | Type | Source | Severity |
|----|------|--------|----------|
| C-001 | infrastructure | AX-004 Payments ruling | blocking |
| C-002 | regulatory | UK GDPR / EU GDPR | blocking |
| C-003 | infrastructure | AX-002 Datastore ruling; CLAUDE.md one-datastore principle | blocking |
| C-004 | infrastructure | AX-007 Notifications ruling | significant |

### C-001: Stripe (EU entity) is the only payment processor

**infrastructure · blocking · source:** AX-004 — every charge and refund goes through Stripe's EU entity; Halyard never holds card or bank numbers.
**Impact:** eliminates any second processor · requires idempotency keys on every money call · shapes D-001 (jobs must be idempotent)

### C-002: Counterparty data stays in the UK/EU

**regulatory · blocking · source:** UK GDPR, EU GDPR — owner and marina data is stored and processed in the UK/EU only (Fly.io `lhr`, Stripe EU, Postmark EU).
**Impact:** eliminates US-region vendors for anything carrying owner data · shapes D-001, D-002

### C-003: PostgreSQL is the only datastore

**infrastructure · blocking · source:** AX-002 (ruled 2026-05-19); CLAUDE.md one-datastore principle — jobs, queues, and locks run on PostgreSQL; a second datastore needs a platform sign-off recorded on AX-002 before any code.
**Impact:** eliminates Redis/Kafka-class infrastructure without a sign-off · shapes D-001

### C-004: Postmark is the only outbound e-mail channel

**infrastructure · significant · source:** AX-007 — every owner-facing e-mail goes through Postmark's EU region via the worker; no SMS channel exists.
**Impact:** requires every notification to be a worker job · shapes D-001

## Decisions

### Decision Summary  *(the ID index)*

| ID | Decision | Choice | Shaped By |
|----|----------|--------|-----------|
| D-001 | Background jobs and scheduling | pg-boss on PostgreSQL | C-003, C-001 |
| D-002 | Session mechanism | Server-side sessions, cookie | C-002 |
| D-003 | Error format | RFC 7807 Problem Details | — |

### D-001: Background jobs and scheduling

**Context** (charges, e-mails, and timed work outlive a request) · **Shaped by:** C-003 · C-001

| Option | Pros | Cons |
|--------|------|------|
| BullMQ on Redis | Mature, fast | Second datastore; C-003 forbids without sign-off |
| pg-boss on PostgreSQL | One datastore; transactional enqueue with the row it belongs to | Lower throughput ceiling (fine at our scale) |

**Choice:** pg-boss — **Rationale:** transactional enqueue means a charge and its job commit together, which is what idempotent money movement needs; C-003 rules out Redis without a sign-off nobody has asked for.
**Consequences:** job throughput bounded by PostgreSQL (measured ceiling ~2k jobs/min, far above need) · `singletonKey` mandatory on every job (CLAUDE.md idempotency principle)

### D-002: Session mechanism

**Context** (owner portal and marina console need auth) · **Shaped by:** C-002

| Option | Pros | Cons |
|--------|------|------|
| JWT | Stateless | Revocation hard; token in the browser |
| Server-side session, HttpOnly cookie | Revocable; nothing sensitive in the browser | One table, one lookup per request |

**Choice:** server-side sessions — **Rationale:** revocation on account closure is a GDPR erasure obligation; a lookup per request is cheap.
**Consequences:** session table in PostgreSQL · sticky-session-free (table, not memory)

### D-003: Error format

**Context** (two front-ends, one support desk) · **Shaped by:** —

| Option | Pros | Cons |
|--------|------|------|
| Ad hoc `{code, message}` | Simple | Two front-ends already diverged on it |
| RFC 7807 Problem Details | Standard; correlation id built in | Slightly heavier body |

**Choice:** RFC 7807 — **Rationale:** support matches a ticket to a log line by correlation id; one error component in each app.
**Consequences:** shared exception filter is the only place an error body is built

## Infrastructure

### Infrastructure Summary  *(the ID index)*

| ID | Type | Source Constraint | Priority |
|----|------|-------------------|----------|
| IP-001 | compute | C-002 | MUST |
| IP-002 | storage | C-003 | MUST |

### IP-001: Fly.io machines in `lhr`

**compute · MUST · source:** C-002 — two `api` machines and one `worker` machine in Fly.io `lhr`; no other region.
**Criteria:**
- `fly regions list` shows `lhr` only

### IP-002: Fly Postgres HA pair

**storage · MUST · source:** C-003 — one PostgreSQL 16 HA pair in `lhr`, daily snapshots retained 30 days.
**Criteria:**
- Failover exercised quarterly; snapshot restore tested quarterly

## Declarations

### Declaration Summary  *(the ID index)*

| ID | Kind | Source | Downstream home |
|----|------|--------|-----------------|
| INT-001 | integration | FEAT-012 FR-003 | POST /bookings/{bookingId}/pay (`x-integration`) |
| INT-002 | integration | FEAT-004 FR-006 | worker mail jobs (Postmark) |
| DS-001 | sensitivity | FEAT-002 FR-001 | Owner.email, Owner.phone (Confidential) |

### INT-001: Stripe

**integration · criticality: hard · source:** FEAT-012 FR-003 — the product MUST integrate with Stripe for charges; its unavailability blocks confirmation.
**Authored downstream:** POST /bookings/{bookingId}/pay (`mochiko:patterns-api-contracts`)

### INT-002: Postmark

**integration · criticality: degraded · source:** FEAT-004 FR-006 — the product MUST send owner notifications through Postmark; its unavailability delays notices, never bookings.
**Authored downstream:** worker mail jobs (`mochiko:patterns-api-contracts`)

### DS-001: Owner contact data

**sensitivity · source:** FEAT-002 FR-001 — the product handles owner e-mail and phone, which are sensitive and MUST be classified and protected.
**Authored downstream:** Owner.email, Owner.phone (`mochiko:patterns-entity-modeling`)
