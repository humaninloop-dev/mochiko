# Architecture spine — crewboard

**Scope**: backend-service (plus the `web` SPA)
**Last confirmed**: 2026-05-14 (FEAT-007 landing)

## Containers

- `web` — React SPA, Render static site — built
- `api` — FastAPI, Render web service, two instances — built.
  Modules: `rotas`, `approvals`, `timeoff`, `staff`, `billing` (Stripe), `reports`
- `db` — PostgreSQL 16 — built
- `worker` — Celery 5 worker + beat, Render background worker — built.
  Modules: `notify` (Twilio, Postmark), `integrations.xero`
- `redis` — Redis 7, Celery broker and result backend — built

## External systems

- Twilio — SMS · Postmark — email · Stripe — subscription billing (FEAT-003)
- Xero — nightly accounting sync (FEAT-007), called from `worker`

## Communication styles

- `web → api`: HTTPS/JSON, session cookie
- `api → db`: synchronous SQL (SQLAlchemy), one transaction per request
- `api → worker`: Celery tasks over `redis` for anything that must outlive the request
- `worker → Twilio / Postmark / Xero`: synchronous HTTPS from tasks, retried by Celery
  with backoff; one task per business or per recipient so retries are independent
- `Stripe → api`: signed webhooks, verified at the edge, processed in the request

## Boundaries

- Staff and managers share one trust domain (session cookie + role per venue); scoping
  per AX-001.
- Providers sit behind `crewboard.notify` and `crewboard.integrations`; no other module
  imports a provider SDK.
- Static outbound IPs are enabled on `worker` (Render dashboard, 2026-05-12) and
  allow-listed at Xero; `api` has none.
- Reports are computed on read in `api.reports` from the primary (AX-006).
