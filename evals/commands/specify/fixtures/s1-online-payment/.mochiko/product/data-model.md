# Product data model — Ledgerlite

## Studio

| Attribute | Type | Notes | Sensitivity |
|-----------|------|-------|-------------|
| id | uuid | | Internal |
| name | string | | Internal |
| bank_details | text | shown on the public invoice page | Confidential |

## Client

| Attribute | Type | Notes | Sensitivity |
|-----------|------|-------|-------------|
| id | uuid | | Internal |
| studio_id | uuid | owner | Internal |
| name, email | string | | Confidential |

## Invoice

| Attribute | Type | Notes | Sensitivity |
|-----------|------|-------|-------------|
| id | uuid | never in a client URL | Internal |
| public_token | string | unguessable, fixed for life | Restricted |
| status | enum | draft → sent → paid; voided from any | Internal |
| currency | ISO 4217 | fixed at draft | Internal |
| total_minor | integer | derived from line items | Internal |
| due_on | date | | Internal |

## LineItem — description, quantity, unit_minor, discount_pct · belongs to Invoice

## Payment

| Attribute | Type | Notes | Sensitivity |
|-----------|------|-------|-------------|
| invoice_id | uuid | | Internal |
| amount_minor | integer | > 0, ≤ balance | Internal |
| method | enum | bank_transfer · cash · card · other | Internal |
| paid_on | date | | Internal |
| recorded_by | uuid | studio user | Internal |

## ReminderSchedule — studio_id, offsets (up to three integers)

State machine, Invoice: `draft` → `sent` (send) → `paid` (balance reaches 0); `voided` from
`draft` or `sent`. Delete behavior: invoices are never deleted, only voided.
