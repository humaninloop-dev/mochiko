# Concern ledger — crewboard

Walked 2026-05-06 (backend-service shelf). Rows AX-007 and AX-008 added 2026-06-02 at the
FEAT-010 desk visit.

## AX-001 Tenancy

- **Stance**: decided · **Status**: built
- **Ruling**: pooled multi-tenant; `business_id` on every tenant table, scoped by
  `TenantRepository` (ruled 2026-04-10)
- **Rationale**: 60-odd businesses and a team of three; one database per tenant would cost a
  migration per tenant per release
- **Enforcement**: `tests/tenancy/test_isolation.py` writes as business B and reads every
  tenant table as business A, asserting zero rows
- **As-built**: as ruled · **Drift**: none

## AX-002 Auth

- **Stance**: decided · **Status**: built
- **Ruling**: session cookie; roles `manager` / `staff` per venue; owners are managers of
  every venue (ruled 2026-04-10)
- **As-built**: as ruled · **Drift**: none

## AX-003 Outbound messaging

- **Stance**: decided · **Status**: built
- **Ruling**: every SMS and email is sent from `worker` by a Celery task
  (`crewboard.notify.tasks`); `api` never calls Twilio or Postmark inline, so a slow provider
  cannot stall a request and a burst cannot exhaust the web instances (ruled 2026-05-06)
- **Rationale**: Twilio's p95 is above one second; two web instances serve every venue
- **Enforcement**: `tests/notify/test_no_inline_provider.py` asserts no module under
  `crewboard.api` imports `crewboard.notify.twilio_client` or
  `crewboard.notify.postmark_client`
- **As-built** (FEAT-006, 2026-05-20): rota-publish notifications and shift reminders go
  through the `send_sms` / `send_email` tasks; nothing in `api` touches a provider
- **Drift**: none
- **Work**: —

## AX-004 Scheduled work

- **Stance**: decided · **Status**: built
- **Ruling**: Celery beat for periodic work; `apply_async(eta=…)` for one-off timed sends
  (ruled 2026-05-06)
- **As-built** (FEAT-009): shift reminders queued with `apply_async(eta=…)`, revoked and
  re-queued on rota change · **Drift**: none

## AX-005 Provider-call idempotency

- **Stance**: not-now · **Status**: ruled
- **Rationale**: Celery retries a failed send; a duplicate SMS is an annoyance, not a loss,
  and we have never seen one
- **Upgrade trigger**: the first incident in which a recipient receives the same message
  twice

## AX-006 Multi-region

- **Stance**: n-a · **Status**: ruled
- **Reason**: single-country product on Render's single region; nothing pins data to a
  region

## AX-007 Event bus

- **Stance**: not-now · **Status**: ruled
- **Rationale**: every fan-out today has one consumer and finishes inside a Celery task
- **Upgrade trigger**: a second consumer for the same domain event, or a fan-out that a
  single task cannot finish inside its window

## AX-008 Outbound integrations

- **Stance**: decided · **Status**: built
- **Ruling**: third-party calls run from `worker` tasks with Celery retry and an
  idempotency key per batch; Render static outbound IPs are enabled on `worker` and
  allow-listed at Xero and PayFlow (ruled 2026-06-02)
- **As-built** (FEAT-007, FEAT-010): as ruled · **Drift**: none

## Shelf coverage

Walked: tenancy, auth, outbound messaging, scheduled work, idempotency, multi-region,
event bus, outbound integrations. Not yet walked: observability, rate limiting, feature
flags, data retention, audit.
