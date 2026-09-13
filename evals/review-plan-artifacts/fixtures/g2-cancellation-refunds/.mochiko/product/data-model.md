# Data Model: product baseline (Halyard)

> Entity definitions with relationships, per-attribute sensitivity annotations, and state
> machines. Baseline as of FEAT-019 landing (2026-08-27). Feature deltas live beside this file
> under `.mochiko/features/FEAT-XXX/data-model.md` until their landing folds them here.

## Data Sensitivity Summary  *(the coverage index)*

| Entity | Attribute | Classification | Compliance |
|--------|-----------|---------------|------------|
| Owner | email | Confidential | UK GDPR Art. 6 (DS-001) |
| Owner | phone | Confidential | UK GDPR Art. 6 (DS-001) |
| Vessel | registrationNo | Confidential | UK GDPR Art. 6 |
| Payment | stripePaymentIntentId | Confidential | PCI-DSS (reference only, no PAN) |
| Payment | authorisedBy | Internal | — |

**Handling defaults (once per document — per-attribute rows record only specifics and deviations):**

| Aspect | Confidential | Restricted |
|--------|-------------|------------|
| Encryption at rest / in transit | Required (AES-256 / TLS 1.3+) | Required, strong |
| Audit logging | All access logged | All access logged + anomaly alerts |
| Masking in logs/UIs | Required | Never displayed, never logged |

---

## Entity Summary

| Entity | Attributes | Relationships | Status |
|--------|------------|---------------|--------|
| Owner | 6 | 2 | built |
| Vessel | 6 | 1 | built |
| Marina | 5 | 2 | built |
| Berth | 7 | 2 | built |
| Booking | 9 | 3 | built |
| Payment | 8 | 1 | built |

---

## Entity: Owner

A person who moors a vessel. **Traceability:** FEAT-002

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| id | UUID | Yes | auto | Internal | Primary key |
| email | Email | Yes | — | Confidential | Login and notification address [PII] |
| phone | Phone | No | null | Confidential | Contact number [PII] |
| displayName | Text(80) | Yes | — | Internal | Shown to marina staff |
| createdAt | Timestamp | Yes | auto | Internal | — |
| updatedAt | Timestamp | Yes | auto | Internal | — |

### Sensitivity Details

| Attribute | Level | Retention | Access | Deviations | Compliance |
|-----------|-------|-----------|--------|------------|------------|
| email | Confidential | Delete ≤ 30 d after account closure | Owner reads own; marina staff read for active bookings | Log masking: i***@example.com | UK GDPR Art. 6, 17 (DS-001) |
| phone | Confidential | Delete ≤ 30 d after account closure | Owner reads own; marina staff read for active bookings | — | UK GDPR Art. 6, 17 (DS-001) |

## Entity: Vessel

A boat an owner registers. **Traceability:** FEAT-002

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| id | UUID | Yes | auto | Internal | Primary key |
| ownerId | Reference(Owner) | Yes | — | Internal | Owning owner |
| name | Text(80) | Yes | — | Public | Vessel name |
| lengthM | Decimal(5,2) | Yes | — | Internal | Length overall |
| registrationNo | Text(20) | No | null | Confidential | Registry number [PII] |
| createdAt | Timestamp | Yes | auto | Internal | — |

## Entity: Marina

An operator with berths. **Traceability:** FEAT-001

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| id | UUID | Yes | auto | Internal | Primary key |
| name | Text(120) | Yes | — | Public | — |
| timezone | Text(40) | Yes | Europe/London | Internal | IANA zone for cut-offs |
| createdAt | Timestamp | Yes | auto | Internal | — |
| updatedAt | Timestamp | Yes | auto | Internal | — |

## Entity: Berth

A mooring position at a marina. **Traceability:** FEAT-001, FEAT-004

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| id | UUID | Yes | auto | Internal | Primary key |
| marinaId | Reference(Marina) | Yes | — | Internal | — |
| berthClass | Enum(pontoon, finger, swing) | Yes | — | Public | Berth type |
| maxLengthM | Decimal(5,2) | Yes | — | Public | Largest vessel it takes |
| shorePower | Boolean | Yes | false | Public | Has a metered supply |
| createdAt | Timestamp | Yes | auto | Internal | — |
| updatedAt | Timestamp | Yes | auto | Internal | — |

## Entity: Booking

An owner's reservation of a berth for a period. **Traceability:** FEAT-004, FEAT-012

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| id | UUID | Yes | auto | Internal | Primary key |
| ownerId | Reference(Owner) | Yes | — | Internal | — |
| vesselId | Reference(Vessel) | Yes | — | Internal | — |
| berthId | Reference(Berth) | Yes | — | Internal | — |
| startsOn | Date | Yes | — | Internal | First night |
| endsOn | Date | Yes | — | Internal | Last night |
| status | Enum(requested, confirmed, active, completed) | Yes | requested | Internal | State machine below |
| createdAt | Timestamp | Yes | auto | Internal | — |
| updatedAt | Timestamp | Yes | auto | Internal | — |

### State machine — Booking

`requested → confirmed` (payment succeeded) · `confirmed → active` (startsOn reached) ·
`active → completed` (endsOn passed). No cancellation path in the baseline (FEAT-034 pending).

## Entity: Payment

A charge against a booking. **Traceability:** FEAT-012

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| id | UUID | Yes | auto | Internal | Primary key |
| bookingId | Reference(Booking) | Yes | — | Internal | — |
| amountMinor | Integer | Yes | — | Internal | Pence |
| currency | Text(3) | Yes | GBP | Internal | ISO 4217 |
| stripePaymentIntentId | Text(64) | Yes | — | Confidential | Stripe reference; never a PAN |
| status | Enum(pending, succeeded, failed) | Yes | pending | Internal | — |
| authorisedBy | Reference(Owner) | Yes | — | Internal | Money-moves-on-authority |
| createdAt | Timestamp | Yes | auto | Internal | — |

### Sensitivity Details

| Attribute | Level | Retention | Access | Deviations | Compliance |
|-----------|-------|-----------|--------|------------|------------|
| stripePaymentIntentId | Confidential | 7 years (financial record) | Finance lead; system | — | PCI-DSS (reference only) |

---

## Relationships

| Relationship | Cardinality | From | To | Delete Behavior |
|--------------|-------------|------|----|-----------------|
| owner–vessel | 1:N | Owner | Vessel | Cascade |
| owner–booking | 1:N | Owner | Booking | Restrict |
| marina–berth | 1:N | Marina | Berth | Restrict |
| berth–booking | 1:N | Berth | Booking | Restrict |
| booking–payment | 1:N | Booking | Payment | Restrict |

## Validation Rules

- `Booking.endsOn` ≥ `Booking.startsOn`.
- `Vessel.lengthM` ≤ `Berth.maxLengthM` for any booking.
- `Payment.amountMinor` > 0.
