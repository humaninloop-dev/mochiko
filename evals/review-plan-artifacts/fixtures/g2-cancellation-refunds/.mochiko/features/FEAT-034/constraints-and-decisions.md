# Constraints and Decisions: FEAT-034 (delta over the product baseline)

> technical-analyst · 2026-09-10 · /mochiko:implement FEAT-034 design phase. Rows added beside
> `.mochiko/product/constraints-and-decisions.md`, numbering continued. Baseline rows C-001–C-004,
> D-001–D-003, IP-001–IP-002 apply unchanged.

## Constraints

### Constraint Summary  *(the ID index)*

| ID | Type | Source | Severity |
|----|------|--------|----------|
| C-005 | regulatory | Card-scheme refund rules (Visa, Mastercard) via Stripe | blocking |
| C-006 | infrastructure | Stripe refund lifecycle | significant |
| C-007 | organizational | CLAUDE.md money-moves-on-authority principle | blocking |

### C-005: A card refund returns to the original card

**regulatory · blocking · source:** card-scheme rules as enforced by Stripe — a refund of a card payment is returned to the card that paid; a refund to any other instrument is a scheme violation and Stripe will not create it.
**Impact:** eliminates cash-out to another card or account · shapes D-004

### C-006: Stripe refunds are asynchronous

**infrastructure · significant · source:** Stripe refund lifecycle (memory-asserted) — a refund is created immediately but settles through `refund.updated` webhooks; 5–10 business days to a cardholder, up to 14 for Bacs.
**Impact:** requires a submitted-then-settled state · requires webhook handling on the standing Stripe boundary · shapes D-005

### C-007: A refund carries a named authoriser

**organizational · blocking · source:** CLAUDE.md money-moves-on-authority principle — every refund row carries `authorisedBy` (the cancelling owner); the API rejects a null.
**Impact:** requires `authorisedBy` on Refund · shapes D-005

## Decisions

### Decision Summary  *(the ID index)*

| ID | Decision | Choice | Shaped By |
|----|----------|--------|-----------|
| D-004 | Refund vehicle | Marina credit note; no card refund | C-005 |
| D-005 | Refund state tracking | Hand-rolled outbox table, state machine, and poller | C-006, C-007 |
| D-006 | Refund audit log | EventStoreDB in `lhr` | — |

### D-004: Refund vehicle

**Context** (how the refunded amount gets back to the owner) · **Shaped by:** C-005

| Option | Pros | Cons |
|--------|------|------|
| Stripe refund to the original payment | Owner gets cash back; scheme-compliant | Stripe fees are not returned; settlement takes days |
| Marina credit note redeemable against a future booking at the same marina | Cash stays with the marina; instant; no processor call | Owner must rebook to use it |

**Choice:** marina credit note — **Rationale:** keeps the cash with the marina, is instant, and removes a Stripe round-trip from the cancel path.
**Consequences:** a credit note per refund · redemption logic on the booking path (later feature)

### D-005: Refund state tracking

**Context** (a refund is submitted now and settles later; the row must move with the processor) · **Shaped by:** C-006 · C-007

| Option | Pros | Cons |
|--------|------|------|
| `refund_outbox` table written in the cancel transaction; a worker poller drains it and advances the Refund state machine on webhook events | Fully under our control | Poller interval and stale-row handling to write |

**Choice:** `refund_outbox` with a poller — **Rationale:** we own the state machine end to end and can reason about every transition.
**Consequences:** poller every [TBD] seconds · stale-row sweep · webhook handler maps `refund.updated` to the state machine

### D-006: Refund audit log

**Context** (finance wants to replay how a refund reached its state) · **Shaped by:** —

| Option | Pros | Cons |
|--------|------|------|
| Append-only `refund_events` table in PostgreSQL | One datastore; trivially queryable | No built-in projections |
| EventStoreDB instance in `lhr` | Purpose-built event store; replay and projections out of the box | New datastore to run and back up |

**Choice:** EventStoreDB — **Rationale:** replay and projections come for free, and finance's ledger view (FR-006) is a projection.
**Consequences:** an EventStoreDB instance (IP-003) · a projection worker · two backup regimes

## Infrastructure

### Infrastructure Summary  *(the ID index)*

| ID | Type | Source Constraint | Priority |
|----|------|-------------------|----------|
| IP-003 | storage | D-006 | MUST |

### IP-003: EventStoreDB instance in `lhr`

**storage · MUST · source:** D-006 — one EventStoreDB 23 instance in `lhr` with daily backups retained 30 days.
**Criteria:**
- Instance region is `lhr`
- Backup restore tested before go-live

## Declarations

### Declaration Summary  *(the ID index)*

| ID | Kind | Source | Downstream home |
|----|------|--------|-----------------|
| INT-003 | integration | FR-003 | POST /bookings/{bookingId}/cancel (`x-integration`) |
| DS-002 | sensitivity | FR-004 | Refund.ownerPhone (Confidential) |

### INT-003: Stripe refunds

**integration · criticality: hard · source:** FR-003 — the feature MUST integrate with Stripe's Refund API; its unavailability blocks refund submission, never cancellation.
**Authored downstream:** POST /bookings/{bookingId}/cancel (`mochiko:patterns-api-contracts`)

### DS-002: Refund contact number

**sensitivity · source:** FR-004 — the feature handles the owner's contact number for refund queries, which is sensitive and MUST be classified and protected.
**Authored downstream:** Refund.ownerPhone (`mochiko:patterns-entity-modeling`)
