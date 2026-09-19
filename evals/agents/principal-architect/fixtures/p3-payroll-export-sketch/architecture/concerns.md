# Concern ledger — crewboard

Walked 2026-05-06 (backend-service shelf).

## AX-001 Tenancy

- **Stance**: decided · **Status**: built
- **Ruling**: pooled multi-tenant; `business_id` on every tenant table, scoped by
  `TenantRepository` (ruled 2026-04-10)
- **Enforcement**: `tests/tenancy/test_isolation.py` · **As-built**: as ruled · **Drift**: none

## AX-002 Auth

- **Stance**: decided · **Status**: built
- **Ruling**: session cookie; roles `manager` / `staff` per venue (ruled 2026-04-10)
- **As-built**: as ruled · **Drift**: none

## AX-003 Outbound messaging

- **Stance**: decided · **Status**: built
- **Ruling**: every SMS and email is sent from `worker` by a Celery task; `api` never calls
  a provider inline (ruled 2026-05-06)
- **As-built** (FEAT-006): as ruled · **Drift**: none

## AX-004 Scheduled work

- **Stance**: decided · **Status**: built
- **Ruling**: Celery beat for periodic work; `apply_async(eta=…)` for one-off timed sends;
  per-venue local-time fan-out through `crewboard.scheduling.tz` (ruled 2026-05-06)
- **Rationale**: one scheduler the team already operates; Celery beat has run the reminders
  since April
- **As-built** (FEAT-009): shift reminders queued with `apply_async(eta=…)` · **Drift**: none

## AX-005 Event bus

- **Stance**: not-now · **Status**: ruled
- **Rationale**: every fan-out today has one consumer and finishes inside a Celery task
- **Upgrade trigger**: a second consumer for the same domain event, or a fan-out that a
  single task cannot finish inside its window

## AX-006 Reports

- **Stance**: decided · **Status**: built
- **Ruling**: manager reports computed on read from the primary in `api.reports`; no
  warehouse, no replica (ruled 2026-05-06)
- **Upgrade trigger**: a report query above 2 s at p95

## AX-007 Outbound integrations

- **Stance**: decided · **Status**: built
- **Ruling**: third-party calls run from `worker` tasks with Celery retry and an
  idempotency key per batch; static outbound IPs on `worker`, allow-listed per partner
  (ruled 2026-05-06)
- **Enforcement**: `tests/integrations/test_idempotency_key.py` asserts every outbound
  batch call sets one
- **As-built** (FEAT-007, Xero): as ruled · **Drift**: none

## AX-008 Inbound webhooks

- **Stance**: decided · **Status**: built
- **Ruling**: inbound third-party callbacks terminate in `api` behind signature
  verification and an event-id dedupe table before anything is processed (ruled
  2026-05-06)
- **As-built** (FEAT-003, Stripe): as ruled · **Drift**: none

## Shelf coverage

Walked: tenancy, auth, outbound messaging, scheduled work, event bus, reports, outbound
integrations, inbound webhooks. Not yet walked: observability, rate limiting, feature
flags, data retention, audit.
