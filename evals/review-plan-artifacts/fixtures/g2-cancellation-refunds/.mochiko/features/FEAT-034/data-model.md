# Data Model: FEAT-034 (delta over the product baseline)

> devils-advocate (covering for the technical analyst) · 2026-09-09 · /mochiko:implement FEAT-034
> design phase. Entities added or extended beside `.mochiko/product/data-model.md`; the
> baseline's handling defaults apply.

## Data Sensitivity Summary  *(the coverage index)*

| Entity | Attribute | Classification | Compliance |
|--------|-----------|---------------|------------|
| Refund | stripeRefundId | Confidential | PCI-DSS (reference only) |

---

## Entity Summary

| Entity | Attributes | Relationships | Status |
|--------|------------|---------------|--------|
| Refund | 10 | 2 | [NEW] |
| Booking | +3 | 0 | [EXTENDS EXISTING] |

---

## Entities

## Entity: Refund [NEW]

The money returned for one cancelled booking. **Traceability:** FR-002, FR-003, FR-006, US-1, US-2, US-3

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| id | UUID | Yes | auto | Internal | Primary key |
| bookingId | Reference(Booking) | Yes | — | Internal | — |
| amountMinor | Integer | Yes | — | Internal | Pence; 0 for tier none |
| currency | Text(3) | Yes | GBP | Internal | — |
| tier | Enum(full, half, none) | Yes | — | Internal | FR-002 |
| status | Enum(requested, submitted, succeeded, failed) | Yes | requested | Internal | State machine below |
| stripeRefundId | Text(64) | No | null | Confidential | Set when submitted |
| ownerPhone | Phone | No | null | | Number to call about the refund; copied from Owner at cancellation |
| authorisedBy | Reference(Owner) | Yes | — | Internal | Money-moves-on-authority |
| createdAt | Timestamp | Yes | auto | Internal | — |

### Relationships

| Relationship | Cardinality | Target | Delete Behavior |
|--------------|-------------|--------|-----------------|
| booking | 1:1 | Booking | Restrict |
| authoriser | N:1 | Owner | Restrict |

### State machine — Refund

`requested → submitted` (processor accepted) · `submitted → succeeded` (processor settled) ·
`submitted → failed` (processor rejected).

### Sensitivity Details

| Attribute | Level | Retention | Access | Deviations | Compliance |
|-----------|-------|-----------|--------|------------|------------|
| stripeRefundId | Confidential | 7 years (financial record) | Finance lead; system | — | PCI-DSS (reference only) |

## Entity: Booking [EXTENDS EXISTING]

Existing entity extended with cancellation. **Traceability:** FR-001, FR-005, US-1

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| status | Enum(requested, confirmed, active, completed, cancelled) | Yes | requested | Internal | `cancelled` added |
| cancelledAt | Timestamp | No | null | Internal | SC-001 is measured from this |
| cancellationReason | Text(500) | No | null | Internal | Free text from the owner |

### State machine — Booking (amended)

`confirmed → cancelled` (owner cancels before the first night). Other transitions unchanged.

---

## Relationships

| Relationship | Cardinality | From | To | Delete Behavior |
|--------------|-------------|------|----|-----------------|
| booking–refund | 1:1 | Booking | Refund | Restrict |
| owner–refund (authoriser) | 1:N | Owner | Refund | Restrict |
