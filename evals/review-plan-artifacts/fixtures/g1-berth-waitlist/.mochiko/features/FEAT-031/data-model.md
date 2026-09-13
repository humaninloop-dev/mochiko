# Data Model: FEAT-031 (delta over the product baseline)

> technical-analyst · 2026-09-09 · /mochiko:implement FEAT-031 design phase. Entities added or
> extended beside `.mochiko/product/data-model.md`; the baseline's handling defaults apply.

## Data Sensitivity Summary  *(the coverage index)*

| Entity | Attribute | Classification | Compliance |
|--------|-----------|---------------|------------|
| Offer | stripePaymentIntentId | Confidential | PCI-DSS (reference only) |

---

## Entity Summary

| Entity | Attributes | Relationships | Status |
|--------|------------|---------------|--------|
| WaitlistEntry | 11 | 3 | [NEW] |
| Offer | 8 | 2 | [NEW] |
| Notification | 6 | 1 | [NEW] |
| Booking | +1 | +1 | [EXTENDS EXISTING] |

---

## Entities

## Entity: WaitlistEntry [NEW]

An owner's standing request for a berth class at a marina over a date range. **Traceability:** FR-001, FR-002, FR-006, US-1

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| id | UUID | Yes | auto | Internal | Primary key |
| ownerId | Reference(Owner) | Yes | — | Internal | — |
| vesselId | Reference(Vessel) | Yes | — | Internal | Fit is checked against its lengthM |
| marinaId | Reference(Marina) | Yes | — | Internal | — |
| berthClass | Enum(pontoon, finger, swing) | Yes | — | Internal | — |
| fromDate | Date | Yes | — | Internal | — |
| toDate | Date | Yes | — | Internal | — |
| position | Integer | Yes | computed | Internal | 1-based, by createdAt within (marinaId, berthClass) |
| contactEmail | Email | Yes | — | | Address the offer notice goes to; copied from Owner at join |
| status | Enum(waiting, offered, fulfilled, left, expired) | Yes | waiting | Internal | — |
| createdAt | Timestamp | Yes | auto | Internal | Join time; orders the queue |

### Relationships

| Relationship | Cardinality | Target | Delete Behavior |
|--------------|-------------|--------|-----------------|
| owner | N:1 | Owner | Restrict |
| vessel | N:1 | Vessel | Restrict |
| marina | N:1 | Marina | Restrict |

## Entity: Offer [NEW]

A berth held for one entrant for a bounded time. **Traceability:** FR-003, FR-004, FR-005, US-2, US-3

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| id | UUID | Yes | auto | Internal | Primary key |
| waitlistEntryId | Reference(WaitlistEntry) | Yes | — | Internal | — |
| berthId | Reference(Berth) | Yes | — | Internal | The held berth |
| issuedAt | Timestamp | Yes | auto | Internal | SC-002 is measured to this |
| expiresAt | Timestamp | Yes | issuedAt + 24 h | Internal | C-005 |
| status | Enum(open, accepted, declined) | Yes | open | Internal | State machine below |
| stripePaymentIntentId | Text(64) | No | null | Confidential | Set on accept |
| createdAt | Timestamp | Yes | auto | Internal | — |

### Relationships

| Relationship | Cardinality | Target | Delete Behavior |
|--------------|-------------|--------|-----------------|
| waitlistEntry | N:1 | WaitlistEntry | Cascade |
| berth | N:1 | Berth | Restrict |

### State machine — Offer

`open → accepted` (charge succeeded) · `open → declined` (owner declines).

### Sensitivity Details

| Attribute | Level | Retention | Access | Deviations | Compliance |
|-----------|-------|-----------|--------|------------|------------|
| stripePaymentIntentId | Confidential | 7 years (financial record) | Finance lead; system | — | PCI-DSS (reference only) |

## Entity: Notification [NEW]

A record of each owner-facing notice sent for a waitlist event. **Traceability:** FR-007

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| id | UUID | Yes | auto | Internal | Primary key |
| ownerId | Reference(Owner) | Yes | — | Internal | — |
| channel | Enum(postmark) | Yes | postmark | Internal | — |
| template | Text(40) | Yes | — | Internal | Postmark template alias |
| sentAt | Timestamp | No | null | Internal | — |
| createdAt | Timestamp | Yes | auto | Internal | — |

### Relationships

| Relationship | Cardinality | Target | Delete Behavior |
|--------------|-------------|--------|-----------------|
| owner | N:1 | Owner | Cascade |

## Entity: Booking [EXTENDS EXISTING]

Existing entity extended with the offer link. **Traceability:** FR-004, US-2

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| offerId | Reference(Offer) | No | null | Internal | Set when the booking was created from an accepted offer |

---

## Relationships

| Relationship | Cardinality | From | To | Delete Behavior |
|--------------|-------------|------|----|-----------------|
| entry–offer | 1:N | WaitlistEntry | Offer | Cascade |
| offer–booking | 1:1 | Offer | Booking | Restrict |
| berth–offer | 1:N | Berth | Offer | Restrict |

## Validation Rules

- `WaitlistEntry.toDate` ≥ `WaitlistEntry.fromDate`.
- One `waiting` or `offered` WaitlistEntry per (ownerId, marinaId, berthClass, fromDate, toDate).
- At most one `open` Offer per berthId at a time.
- `Offer.expiresAt` = `issuedAt` + 24 h (C-005).
