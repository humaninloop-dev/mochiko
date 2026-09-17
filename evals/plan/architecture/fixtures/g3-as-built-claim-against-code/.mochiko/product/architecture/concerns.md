# Concern Ledger

## AX-001 Multi-tenancy

- **Stance**: decided · **Status**: built
- **Ruling**: pooled, `account_id` on every tenant table, scoped by `store.Scoped` (ruled 2026-06-24)
- **Rationale**: 38 accounts; a silo per account would multiply the Fly bill and the migration work for no isolation a customer has asked for
- **Upgrade trigger**: an account whose contract requires a dedicated database, or 500 accounts
- **Targets**: NFR-003 (from FEAT-001 SC-004) — no unscoped query on a tenant table, enforced by the `internal/store` isolation suite
- **As-built**: `account_id` on 12 tables; `store.Scoped` the only door; suite green (checked 2026-08-20)
- **Drift**: none

## AX-002 Identity & auth

- **Stance**: decided · **Status**: built
- **Ruling**: email + password, session cookie scoped to the account; no third-party IdP (ruled 2026-06-24)
- **Upgrade trigger**: a signed contract requiring SSO, or 100 accounts — **fired 2026-08-20** (Northgate signed; routed to the feature desk)
- **As-built**: `internal/web/session.go`; argon2id; 30-day cookie (checked 2026-08-20)
- **Drift**: none

## AX-003 Authorization

- **Stance**: decided · **Status**: built
- **Ruling**: two roles per account — dispatcher, technician — checked at the router group (ruled 2026-06-24)
- **As-built**: as ruled · **Drift**: none

## AX-004 Background work

- **Stance**: decided · **Status**: built
- **Ruling**: a `jobs` table polled by `worker` every 5 s with SKIP LOCKED; at-least-once; every handler idempotent on its own event; no broker (ruled 2026-06-24)
- **Upgrade trigger**: a job kind that needs ordering across accounts, or a second consumer for the same row
- **Targets**: NFR-001 (from FEAT-002 SC-003) — ETA SMS within 30 s p95 of assignment; NFR-004 (from FEAT-002 SC-002) — no job in `assigned` older than 15 min without an `sms_sent` or `sms_failed` event
- **As-built**: `internal/worker/poll.go`; kinds `send_eta_sms`, `partner_webhook` (checked 2026-08-20)
- **Drift**: none

## AX-005 Notifications

- **Stance**: decided · **Status**: built
- **Ruling**: customer SMS through the registered Twilio number from `worker` only; dispatcher notifications by email (ruled 2026-06-24)
- **As-built**: `internal/notify`; numbers masked in logs · **Drift**: none

## AX-006 API surface

- **Stance**: decided · **Status**: built
- **Ruling**: internal JSON API only; outbound partner webhooks HMAC-signed and retried from `worker`; inbound webhooks only from Stripe, verified at the edge (ruled 2026-07-14, amended 2026-08-20)
- **Upgrade trigger**: a partner that needs to call us, or a third partner
- **As-built** (FEAT-005): `partner_webhook` kind, per-account endpoints, delivery log · **Drift**: none

## AX-007 Deployment & environments

- **Stance**: decided · **Status**: built
- **Ruling**: Fly.io, one app per environment, two process groups from one binary tree, `lhr` only (ruled 2026-06-24)
- **As-built**: `fly.toml` as ruled · **Drift**: none

## AX-008 Billing & entitlements

- **Stance**: decided · **Status**: in-flight (FEAT-006)
- **Ruling**: Stripe subscriptions, one per account; the webhook is verified at the edge in `api`, its event recorded and handed to `worker` through the `jobs` table; `api` never calls Stripe inside the webhook request; entitlements are read from a local `subscriptions` table, never from Stripe at request time; every plan change writes an `audit_events` row (ruled 2026-08-20)
- **Rationale**: the 25-account trigger fired in July; Stripe's retry semantics make the edge-verify-then-enqueue shape the one that survives a slow Stripe API without holding the webhook request open; the audit row is GI-004's
- **Targets**: NFR-005 (from FEAT-006 SC-002) — a subscription change is reflected in entitlements within 60 s p95 of Stripe's event; NFR-006 (from GI-009) — a webhook is acknowledged only after its event row is committed
- **Approved delta**: `.mochiko/specs/stripe-subscriptions/architecture-delta.md` (signed 2026-08-22)
- **Work**: FEAT-006

## AX-009 Data lifecycle

- **Stance**: not-now · **Status**: ruled
- **Rationale**: daily Fly snapshot, seven-day retention; export and deletion by account owed to Northgate under the contract's 30-day clause
- **Upgrade trigger**: the Northgate go-live date (2026-11-01), or any account's written export or deletion request

## AX-010 Observability

- **Stance**: decided · **Status**: built
- **Ruling**: request id on every log line; Sentry with the id as a tag; a `jobs` backlog alert at 50 rows or 10 min age (ruled 2026-08-20)
- **As-built**: `internal/http` middleware; Fly metrics alert · **Drift**: none

## AX-011 Feature flags & rollout

- **Stance**: not-now · **Status**: ruled
- **Rationale**: two engineers, one deploy a day; the per-account column served FEAT-005's rollout
- **Upgrade trigger**: a surface that must roll out to fewer than all accounts and cannot ride a per-account column, or 5 live flags

## AX-012 Product analytics

- **Stance**: n-a — handled elsewhere · **Status**: ruled
- **Owned by**: the `fieldnote-growth` repo — a nightly read replica into the growth team's warehouse; the product repo emits no events

## AX-013 Security baseline

- **Stance**: decided · **Status**: built
- **Ruling**: `RateLimit` on every unauthenticated route (per IP and per account slug); `audit_events` row on every money-moving action; secrets in Fly secrets (ruled 2026-08-20)
- **As-built**: `RateLimit` middleware on `/book/{slug}` and `/webhooks/stripe` (2026-08-22); `audit_events` written by FEAT-006's plan-change path · **Drift**: none

## Shelf coverage

Backend-service shelf, 13 dimensions: all 13 on the ledger (AX-001 to AX-013). Last full
walk 2026-08-20.
