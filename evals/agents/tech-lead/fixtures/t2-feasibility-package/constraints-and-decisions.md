# Constraints and decisions — FEAT-011 Reschedule delivery window

Produced by: technical-analyst seat (`analyst-1`)

## Hard constraints

- **C-001** The carrier aggregator accepts a window change only while the parcel is at the
  depot. Parcels are loaded onto vehicles between 05:00 and 06:00 on the delivery day; after
  loading, the aggregator API rejects every change with `WINDOW_LOCKED`, and only a phone call
  to the depot can hold a parcel. *Source: carrier master agreement §4.3, fixed until the 2028
  renewal; the aggregator declined a same-day change API in writing on 2026-08-19.*
- **C-002** Every window change is confirmed with the carrier inside the request; the
  recipient never sees an unconfirmed window. *Source: FR-003; ShopLoop support policy after
  INC-52 (a window shown, never confirmed, parcel returned).*
- **C-003** Aggregator API latency for a window change: p50 420 ms · p95 900 ms · p99 2.1 s,
  measured over the last 90 days; the aggregator offers no faster tier. *Source:
  `ops/carrier-latency-2026-Q3.md`.*
- **C-004** No recipient contact data is stored, logged, or transmitted by notify-svc.
  *Source: ShopLoop data-processing agreement, annex 2; DS-002.*
- **C-005** Every window change is retained for 24 months with requester and time. *Source:
  NFR-005 — consumer-protection record-keeping (compliance module).*

## Non-functional requirements

- **NFR-002** A window change is confirmed within 300 ms at p95, measured at the API gateway.
  *Source: SC-001.*
- **NFR-005** Change history retained 24 months with requester and time. *Source: compliance
  module — consumer-protection record-keeping.*

## Technology decisions

- **D-001** The window change is persisted in the existing PostgreSQL 16 database
  (`shipments`, new `window_changes`). No new datastore.
- **D-002** Carrier confirmation calls go through the existing `notify.carriers.http` client
  (shared timeouts, three retries).
- **D-003** On `PUT /shipments/{id}/window`, the handler calls the aggregator, waits for its
  confirmation, then responds (C-002).
- **D-004** Optimistic locking on `window_version` prevents two concurrent changes from both
  confirming.
- **D-005** The handler publishes a `WindowChangeRequested` event to a new Redis Streams bus
  (`notify.events`); a new `window-writer` consumer group (one worker) reads it, writes the
  `window_changes` row and updates `shipments`, then publishes `WindowChangePersisted`; the
  handler blocks on that second event before responding. *Rationale:* keeps the handler free
  of writes and gives us an event log for later consumers.
- **D-006** Reminders (FR-006) are driven by a new `window_scheduler` package: a persistent
  schedule table, a polling loop every 30 s, leader election via a PostgreSQL advisory lock,
  and a retry ledger — about 600 lines, owned by this team. *Rationale:* we need exact-time
  sends and the freedom to re-time a reminder when a window moves.

## Infrastructure provisioning

- **IP-001** Nothing new: Redis 7 already backs the Celery broker; the Streams bus reuses it.
