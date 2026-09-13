# Data Model: FEAT-027 (delta over the product baseline)

> technical-analyst · 2026-09-05 · /mochiko:implement FEAT-027 design phase (round 1; unchanged
> in round 2). Entities added beside `.mochiko/product/data-model.md`; the baseline's handling
> defaults apply.

## Data Sensitivity Summary  *(the coverage index)*

No Confidential or Restricted attributes are introduced; every attribute below is Internal or Public.

---

## Entity Summary

| Entity | Attributes | Relationships | Status |
|--------|------------|---------------|--------|
| Meter | 5 | 1 | [NEW] |
| MeterReading | 6 | 1 | [NEW] |
| Tariff | 6 | 1 | [NEW] |

---

## Entities

## Entity: Meter [NEW]

The metering point on a berth. **Traceability:** FR-001, US-1

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| id | UUID | Yes | auto | Internal | Primary key |
| berthId | Reference(Berth) | Yes | — | Internal | The berth it meters |
| gatewayMeterId | Text(40) | Yes | — | Internal | Enerlink's id |
| installedAt | Timestamp | Yes | — | Internal | — |
| createdAt | Timestamp | Yes | auto | Internal | — |

### Relationships

| Relationship | Cardinality | Target | Delete Behavior |
|--------------|-------------|--------|-----------------|
| berth | N:1 | Berth | Restrict |

## Entity: MeterReading [NEW]

One cumulative kWh value at one instant for one meter. **Traceability:** FR-001, FR-002, FR-005, US-1, US-4

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| id | UUID | Yes | auto | Internal | Primary key |
| meterId | Reference(Meter) | Yes | — | Internal | — |
| readAt | Timestamp | Yes | — | Internal | Instant of the reading (gateway clock) |
| kwh | Decimal(12,3) | Yes | — | Internal | Cumulative register value |
| source | Enum(gateway, backfill) | Yes | gateway | Internal | FR-005 |
| createdAt | Timestamp | Yes | auto | Internal | Row commit; SC-001 measured readAt to visible |

### Relationships

| Relationship | Cardinality | Target | Delete Behavior |
|--------------|-------------|--------|-----------------|
| meter | N:1 | Meter | Cascade |

## Entity: Tariff [NEW]

A marina's price per kWh over a validity period. **Traceability:** FR-003, FR-004, US-2, US-3, D-009

| Attribute | Type | Required | Default | Sensitivity | Description |
|-----------|------|----------|---------|-------------|-------------|
| id | UUID | Yes | auto | Internal | Primary key |
| marinaId | Reference(Marina) | Yes | — | Internal | — |
| pencePerKwh | Integer | Yes | — | Public | Shown to owners |
| validFrom | Date | Yes | — | Public | Inclusive |
| validTo | Date | No | null | Public | Inclusive; null = open-ended |
| createdAt | Timestamp | Yes | auto | Internal | — |

### Relationships

| Relationship | Cardinality | Target | Delete Behavior |
|--------------|-------------|--------|-----------------|
| marina | N:1 | Marina | Restrict |

---

## Relationships

| Relationship | Cardinality | From | To | Delete Behavior |
|--------------|-------------|------|----|-----------------|
| berth–meter | 1:1 | Berth | Meter | Restrict |
| meter–reading | 1:N | Meter | MeterReading | Cascade |
| marina–tariff | 1:N | Marina | Tariff | Restrict |

## Validation Rules

- `MeterReading.kwh` ≥ 0; one row per (meterId, readAt).
