# Concern ledger — crewboard

Walk history: 2026-05-06 (AX-001 – AX-006), 2026-06-02 (AX-007 – AX-008),
2026-07-20 (AX-009 – AX-012), 2026-08-30 (AX-013 – AX-020 opened; walk paused at AX-013 when
the visit ran out of time).

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
- **As-built** (FEAT-014, 2026-09-05): the INC-31 inline sends reverted; guard test green
- **Drift**: none

## AX-004 Scheduled work

- **Stance**: decided · **Status**: built
- **Ruling**: Celery beat for periodic work; `apply_async(eta=…)` for timed sends (ruled
  2026-05-06) · **As-built**: as ruled · **Drift**: none

## AX-005 Provider-call idempotency

- **Stance**: decided · **Status**: in-flight (FEAT-014)
- **Ruling**: every provider send carries an idempotency key derived from the domain event
  (ruled 2026-09-01, after INC-31)
- **Work**: FEAT-014

## AX-006 Multi-region

- **Stance**: n-a · **Status**: ruled · **Reason**: single-country product; Render single
  region

## AX-007 Event bus

- **Stance**: not-now · **Status**: ruled
- **Rationale**: every fan-out has one consumer and fits in a Celery task
- **Upgrade trigger**: a second consumer for the same domain event, or a fan-out a single
  task cannot finish inside its window

## AX-008 Outbound integrations

- **Stance**: decided · **Status**: built
- **Ruling**: third-party calls run from `worker` tasks with Celery retry and an
  idempotency key per batch; static outbound IPs on `worker` (ruled 2026-06-02)
- **As-built** (FEAT-007, FEAT-010): as ruled · **Drift**: none

## AX-009 Audit log

- **Stance**: decided · **Status**: in-flight (FEAT-011)
- **Ruling**: every mutating request is recorded — actor, business, entity, before/after —
  by an `api` middleware, retained twelve months; the record is append-only (ruled
  2026-07-20)
- **Rationale**: two businesses asked "who changed this rota"; a per-router call was
  rejected at the desk because a router that forgets the call silently drops a category
- **Enforcement**: `tests/audit/test_every_mutation_audited.py` (planned) sends one
  mutating request per router and asserts an audit row for each
- **Approved delta**: `specs/FEAT-011-audit-log/architecture-delta.md`
- **Work**: FEAT-011

## AX-010 Search

- **Stance**: not-now · **Status**: ruled
- **Rationale**: staff lists are small and PostgreSQL `ILIKE` serves the staff picker in
  under 300 ms for every business we have
- **Upgrade trigger**: a business with more than 50 venues, OR the staff-picker p95 above
  800 ms for any business

## AX-011 Reports

- **Stance**: decided · **Status**: in-flight (FEAT-008)
- **Ruling**: manager reports computed on read from the primary; no warehouse, no replica
  (ruled 2026-06-01)
- **Upgrade trigger**: a report query above 2 s at p95, or a report that needs more than a
  week of history joined across venues
- **Work**: FEAT-008

## AX-012 Data retention

- **Stance**: not-now · **Status**: ruled
- **Rationale**: no customer has asked; rotas are small
- **Upgrade trigger**:

---

## Opened 2026-08-30 — walk paused here

## AX-013 Feature flags

- **Stance**: open · **Status**: ruled
- **Notes from the walk**: we roll a feature to one pilot business before all of them;
  about two flags a quarter; today a flag is an environment variable and a redeploy

## AX-014 Localisation

- **Stance**: open · **Status**: ruled
- **Notes from the walk**: UK only; one Irish prospect (English UI, EUR); no request for a
  second language

## AX-015 Experimentation (A/B)

- **Stance**: open · **Status**: ruled
- **Notes from the walk**: no product ask; the PM measures adoption from the metrics doc

## AX-016 Rate limiting

- **Stance**: open · **Status**: ruled
- **Notes from the walk**: a public rota widget (embeddable, token-authenticated) is
  planned for Q4; INC-19 (2026-03) was a runaway integration client that took an `api`
  instance to 100 % CPU for twenty minutes

## AX-017 Backups and restore

- **Stance**: open · **Status**: ruled
- **Notes from the walk**: Render daily snapshots, seven-day retention; a restore has never
  been rehearsed; point-in-time recovery is available on the current plan

## AX-018 Identity federation (SSO)

- **Stance**: open · **Status**: ruled
- **Notes from the walk**: one enterprise prospect — a 40-venue group — asked for Google
  Workspace SSO for its managers; contract value would be our largest

## AX-019 Data partitioning, export, and deletion

- **Stance**: open · **Status**: ruled
- **Notes from the walk**: three GDPR subject-access and deletion requests so far, each
  handled by hand with SQL; `business_id` is on every table but nothing exports or deletes
  by business; the SSO prospect's procurement asked for "data export on termination"

## AX-020 Observability

- **Stance**: open · **Status**: ruled
- **Notes from the walk**: Render logs and Sentry; no tracing; no alert on Celery queue
  lag — INC-31 was reported by a customer forty minutes in

## Shelf coverage

Backend-service shelf, 22 dimensions: 20 on the ledger. Not yet on the ledger: secrets
management, dependency and supply-chain policy.
