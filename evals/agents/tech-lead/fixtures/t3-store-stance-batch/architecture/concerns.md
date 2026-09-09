# Concern Ledger — notify-svc

## AX-001 Tenancy

- **Stance**: decided
- **Status**: built <!-- flipped at the FEAT-009 landing, 2026-09-08; was: in-flight (FEAT-009) -->
- **Ruling**: pooled multi-tenant, `tenant_id` row-level scoping (ruled 2026-05-20)
- **Rationale**: one database per tenant costs a migration per tenant per release; pooled
  with row scoping fits 40 tenants and a three-person team
- **Upgrade trigger**: first contract requiring a dedicated database OR more than 500 tenants
- **Enforcement**: `tests/tenancy/test_isolation.py` writes as tenant B, reads every tenant
  table as tenant A, and asserts zero rows; runs in CI
- **As-built** (FEAT-009, 2026-09-08): pooled; `tenant_id` on all nine tenant tables;
  PostgreSQL row-level security enabled on every tenant table, with the tenant set per
  request by `SET LOCAL app.tenant_id` in the session factory, so a query that forgets its
  filter still returns nothing. Written from the FEAT-009 design (delta D-2).
- **Drift**: none
- **Work**: FEAT-009

## AX-002 Auth

- **Stance**: decided · **Status**: built
- **Ruling**: shop-issued recipient tokens (opaque, 30-day) verified by `notify.auth`; admin
  via OIDC (ruled 2026-05-20)
- **Enforcement**: `tests/auth/` and the contract suite · **As-built**: as ruled ·
  **Drift**: none

## AX-003 Secrets

- **Stance**: decided
- **Status**: built <!-- flipped at the FEAT-009 landing; was: modifying (FEAT-009) -->
- **Ruling**: Heroku config vars, rotated quarterly; `gitleaks` pre-commit and CI
- **As-built** (FEAT-009): carrier keys moved to per-tenant rows in `tenant_credentials`,
  encrypted with the app key via `pgcrypto`
- **Drift**: none

## AX-004 Scheduled work

- **Stance**: decided · **Status**: built
- **Ruling**: Celery beat and `apply_async(eta=…)` (ruled 2026-06-12) · **As-built**:
  FEAT-006 reminders · **Drift**: none

## AX-005 Outbound email

- **Stance**: decided · **Status**: built
- **Ruling**: Postmark through `notify.mail` (ruled 2026-06-12)
- **Work**: — <!-- FEAT-007 key cleared at the 2026-09-08 landing; FEAT-007 closed 2026-07-30 -->

---

## Batch 2026-09-08 — shelf walk, reliability and security dimensions

## AX-006 Idempotency

- **Stance**: decided
- **Status**: ruled
- **Ruling**: every mutating endpoint accepts an `Idempotency-Key` header; a repeated key
  within 24 h returns the stored response (ruled 2026-09-08)
- **Rationale**: ShopLoop's checkout retries on timeout; without a key, a retried "create
  shipment" made two carrier bookings in INC-48 and the carrier charged for both
- **Upgrade trigger**: n/a — decided
- **Enforcement**: one contract test per mutating endpoint sends the same request twice and
  asserts one row and one carrier call; `notify.api.idempotency` middleware is the single
  implementation
- **Work**: FEAT-012 (queued)

## AX-007 Read caching

- **Stance**: not-now
- **Status**: ruled
- **Rationale**: the tracking page is the only hot read and it is served in 80 ms at p95
  today; a cache adds an invalidation path for a problem we do not have
- **Upgrade trigger**: when the read load becomes a problem

## AX-008 Rate limiting

- **Stance**: decided
- **Status**: ruled
- **Ruling**: 60 requests per minute per recipient token and 600 per minute per tenant,
  enforced in the `api` container (ruled 2026-09-08)
- **Rationale**: team preference; agreed on the call
- **Upgrade trigger**: n/a — decided
- **Enforcement**: `tests/api/test_rate_limit.py` sends 61 requests inside a minute and
  asserts a 429 on the last; both limits live in one settings module
- **Work**: FEAT-012 (queued)

## AX-009 Outbound webhook signing

- **Stance**: decided
- **Status**: ruled
- **Ruling**: every webhook notify-svc sends to a shop carries an HMAC-SHA256 signature over
  the body in `X-Notify-Signature`, keyed per tenant (ruled 2026-09-08)
- **Rationale**: a shop must be able to reject a forged delivery-status callback; two shops
  asked for it, and it is the item the security questionnaire keeps failing on
- **Upgrade trigger**: n/a — decided
- **Work**: FEAT-013 (queued)

## AX-010 Multi-region

- **Stance**: n-a — handled elsewhere
- **Status**: ruled
- **Owned by**: ShopLoop platform (`shoploop/platform-infra`, "Regions" runbook) — notify-svc
  deploys in the platform's single region; a second region is the platform's call
- **Rationale**: the service holds no data that region law pins; latency is dominated by the
  carrier, not by us

## AX-011 Feature flags

- **Stance**: decided
- **Status**: ruled
- **Ruling**: a `flags` table read once per request, cached 30 s, keyed by tenant; no
  third-party flag service (drafted 2026-09-08)
- **Rationale**: we roll a feature to one pilot tenant before all; a table beats a vendor for
  40 tenants and two flags a quarter
- **Upgrade trigger**: more than 20 live flags OR a flag needing per-recipient targeting
- **Enforcement**: `notify.flags.get(tenant, name)` is the only read path; a `ruff` rule
  flags a direct read of the table outside it
- **Author**: tech-lead seat (drafted during the walk; see the batch note)
