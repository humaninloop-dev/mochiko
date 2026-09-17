# Concern Ledger

## AX-001 Multi-tenancy

- **Stance**: decided · **Status**: built
- **Ruling**: pooled, `account_id` on every tenant table, scoped by `store.Scoped` (ruled 2026-06-24)
- **Rationale**: 31 accounts; a silo per account would multiply the Fly bill and the migration work for no isolation a customer has asked for
- **Upgrade trigger**: an account whose contract requires a dedicated database, or 500 accounts
- **Targets**: NFR-003 (from FEAT-001 SC-004) — no unscoped query on a tenant table, enforced by the `internal/store` isolation suite
- **As-built**: `account_id` on 9 tables; `store.Scoped` the only door; suite green
- **Drift**: none

## AX-002 Identity & auth

- **Stance**: decided · **Status**: built
- **Ruling**: email + password, session cookie scoped to the account; no third-party IdP (ruled 2026-06-24)
- **Rationale**: dispatchers and technicians are invited by the account owner; SSO has one prospect asking (Northgate) and is the trigger below
- **Upgrade trigger**: a signed contract requiring SSO, or 100 accounts
- **As-built**: `internal/web/session.go`; argon2id; 30-day cookie
- **Drift**: none

## AX-003 Authorization

- **Stance**: decided · **Status**: built
- **Ruling**: two roles per account — dispatcher, technician — checked at the router group; technicians see only their own jobs (ruled 2026-06-24)
- **As-built**: as ruled · **Drift**: none

## AX-004 Background work

- **Stance**: decided · **Status**: built
- **Ruling**: a `jobs` table polled by `worker` every 5 s with SKIP LOCKED; at-least-once; every handler idempotent on its own event; no broker (ruled 2026-06-24)
- **Rationale**: two kinds of deferred work, one consumer; a broker is a second managed service C-001 does not pay for
- **Upgrade trigger**: a job kind that needs ordering across accounts, or a second consumer for the same row
- **Targets**: NFR-001 (from FEAT-002 SC-003) — ETA SMS within 30 s p95 of assignment, from the worker's `sms_sent` log line; NFR-004 (from FEAT-002 SC-002) — no job in `assigned` older than 15 min without an `sms_sent` or `sms_failed` event
- **As-built**: `internal/worker/poll.go`; two kinds (`send_eta_sms`, `partner_webhook`)
- **Drift**: none

## AX-005 Notifications

- **Stance**: decided · **Status**: built
- **Ruling**: customer SMS through the registered Twilio number from `worker` only; dispatcher notifications by email; no in-app inbox (ruled 2026-06-24)
- **As-built**: `internal/notify`; numbers masked in logs · **Drift**: none

## AX-006 API surface

- **Stance**: decided · **Status**: in-flight (FEAT-005)
- **Ruling**: internal JSON API only, no versioning; outbound partner webhooks (`job.completed`) HMAC-signed and retried from `worker`; no inbound public API (ruled 2026-07-14)
- **Rationale**: two accounting partners asked for job-completed events; nobody has asked to call us
- **Upgrade trigger**: a partner that needs to call us, or a third partner
- **Work**: FEAT-005

## AX-007 Deployment & environments

- **Stance**: decided · **Status**: built
- **Ruling**: Fly.io, one app per environment (`fieldnote-prod`, `fieldnote-staging`), two process groups from one binary tree, `lhr` only (ruled 2026-06-24)
- **As-built**: `fly.toml` as ruled · **Drift**: none

## AX-008 Billing & entitlements

- **Stance**: not-now · **Status**: ruled
- **Rationale**: 19 accounts on hand-raised invoices at the time; Stripe in test mode; entitlements are "everything on" for every account
- **Upgrade trigger**: the 25th account, or the first account that asks to pay by card

## AX-009 Data lifecycle

- **Stance**: not-now · **Status**: ruled
- **Rationale**: daily Fly snapshot, seven-day retention; no export or delete-by-account path; no customer has asked
- **Upgrade trigger**: revisit after the Q2 retention decision (founder, by end of June)

## AX-010 Observability

- **Stance**: open · **Status**: ruled
- **Note**: walked 2026-06-24. Fly logs and Sentry; a request id on every log line since v1.1.0; no traces; no alert on `jobs` backlog — a stuck worker in May was noticed by a customer

## AX-011 Feature flags & rollout

- **Stance**: open · **Status**: ruled
- **Note**: walked 2026-06-24. Every deploy goes to every account; the partner webhooks were switched on per account by a column on `account`

## AX-012 Product analytics

- **Stance**: open · **Status**: ruled
- **Note**: opened 2026-07-14. The growth team runs the activation funnel from the separate `fieldnote-growth` repo (a nightly Postgres read replica into their warehouse); the product repo emits no events of its own

## AX-013 Security baseline

- **Stance**: open · **Status**: ruled
- **Note**: opened 2026-07-14. Secrets in Fly secrets; no rate limit on `/book/{slug}`; no audit log; the booking form was hit 4,000 times in one hour on 2026-07-09 by a scraper and the dispatcher inbox filled with junk jobs

## Shelf coverage

Backend-service shelf, 13 dimensions: all 13 on the ledger (AX-001 to AX-013). Last full
walk 2026-06-24; AX-012 and AX-013 opened 2026-07-14 and not yet dealt.
