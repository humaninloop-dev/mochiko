# Constraints and decisions — Fieldnote

Bootstrapped from the delivered code by the /mochiko:setup run of 2026-06-20 (brownfield).
Rows marked *Assumed* were read from the code and the founder's write-up, not ruled at a
desk; confirm before building on them.

## Hard constraints

- C-001: One Fly app per environment, running two process groups (`api`, `worker`) from
  one binary tree; no third process without a budget ruling. Source: founder, hosting
  budget (2026-02). *Assumed.*
- C-002: Customer SMS goes out only through the registered Twilio A2P number; no other
  SMS provider and no email fallback for customer-facing messages. Source: carrier
  registration (2026-01). *Assumed.*
- C-003: Customer phone numbers never leave the `notify` package unmasked. Source: GI-004.

## Technology decisions

- D-001: Postgres via `pgx`; schema migrations with `goose`, run by the `api` process on
  boot. Consequence: `worker` never migrates.
- D-002: Deferred work rides a `jobs` table polled by the `worker` process every 5 s — no
  broker, no queue service. Chosen over Redis-backed queues (one fewer managed service under
  C-001) and over in-request sending (a Twilio timeout was blocking dispatch responses for
  up to 8 s). Consequence: at-least-once delivery; every job handler is idempotent on
  `(job_id, kind)`. *Assumed — structural; the write-up predates it.*
- D-003: HTTP layer is `chi` on `net/http`; the SPA is served as static files from `api`.
  Consequence: one origin, cookie sessions, no CORS surface. *Assumed — structural.*
- D-004: ETA computed with Mapbox Directions from the technician's last position; a Mapbox
  failure degrades to an ETA-less message, never a dropped message.

## Infrastructure provisioning

- IP-001: Fly.io — `api` (2 machines, shared-cpu-1x), `worker` (1 machine), Fly Postgres 16
  (single node, daily snapshot). Region: `lhr`.
