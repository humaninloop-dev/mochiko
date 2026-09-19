# Data model — FEAT-011 Reschedule delivery window

Produced by: technical-analyst seat (`analyst-1`)

## Entities

### Shipment (existing, extended)

| Attribute | Type | Sensitivity | Note |
|---|---|---|---|
| id | uuid | Internal | existing |
| tenant_id | uuid | Internal | existing |
| carrier_ref | string | Internal | the carrier's shipment id |
| recipient_token | string | Internal | opaque token issued by the shop; the only recipient identifier |
| window_start, window_end | timestamptz | Internal | the confirmed window |
| window_version | int | Internal | increments on every change (optimistic lock) |
| window_state | enum | Internal | see state machine |

**Forbidden attributes.** No recipient contact data — name, email, phone, postal address — is
stored, logged, or transmitted by this service, on this entity or any other. Recipients are
identified only by `recipient_token`; contact data lives in the shop's system and the
carrier's. (DS-002 · Restricted · C-004.)

### WindowChange (new)

| Attribute | Type | Sensitivity | Note |
|---|---|---|---|
| id | uuid | Internal | |
| shipment_id | uuid | Internal | FK → Shipment; cascade delete |
| requested_by | enum recipient / shop / support | Internal | FR-005 |
| requested_at | timestamptz | Internal | FR-005 |
| from_window, to_window | tstzrange | Internal | |
| carrier_confirmed_at | timestamptz, nullable | Internal | FR-003 |
| carrier_confirmation_id | string, nullable | Internal | the carrier's receipt |

Retention: 24 months (NFR-005); the existing nightly retention purge deletes older rows.

## Relationships

- Shipment 1 — n WindowChange (append-only history)

## State machine — `Shipment.window_state`

`confirmed → change_pending → confirmed` on a change the carrier accepts;
`change_pending → confirmed` (unchanged window) on a carrier refusal;
`confirmed → locked` when the two-hour cut-off passes (FR-004).
