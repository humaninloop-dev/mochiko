# Data model — FEAT-013 Webhook subscriptions

Produced by: technical-analyst seat (`analyst-1`)

## Entities

### Subscription (new)

| Attribute | Type | Sensitivity | Note |
|---|---|---|---|
| id | uuid | Internal | |
| tenant_id | uuid | Internal | |
| url | string | Internal | https only |
| secret_ref | string | Restricted | reference into `tenant_credentials`; **the plaintext secret is shown once at creation and is never stored or retrievable** (AX-003) |
| events | array of enum | Internal | subscribed event types |
| status | enum active / paused | Internal | |

### Event (new)

| Attribute | Type | Sensitivity | Note |
|---|---|---|---|
| id | uuid | Internal | |
| tenant_id, shipment_id | uuid | Internal | |
| type | enum | Internal | `status.changed` for now |
| payload | jsonb | Internal | no recipient contact data (C-004 carries over) |
| occurred_at | timestamptz | Internal | SC-001 start of clock |

### Delivery (new)

| Attribute | Type | Sensitivity | Note |
|---|---|---|---|
| id | uuid | Internal | |
| subscription_id, event_id | uuid | Internal | |
| attempt | int | Internal | FR-003 |
| status | enum pending / delivered / dead | Internal | FR-004 |
| response_code | int, nullable | Internal | FR-004 |
| next_attempt_at | timestamptz, nullable | Internal | FR-003 |

Retention: 7 days (FR-004); the nightly purge deletes older rows.

### DeliveryMetricsRollup (new)

| Attribute | Type | Sensitivity | Note |
|---|---|---|---|
| subscription_id | uuid | Internal | |
| hour | timestamptz | Internal | bucket |
| delivered, failed, dead | int | Internal | counts |
| p95_latency_ms | int | Internal | |

Hourly rollups per subscription, computed by a Celery beat task and kept for 90 days.

## Relationships

- Subscription 1 — n Delivery; Event 1 — n Delivery; Subscription 1 — n DeliveryMetricsRollup
