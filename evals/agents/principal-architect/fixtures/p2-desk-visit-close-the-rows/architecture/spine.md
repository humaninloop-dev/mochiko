# Architecture spine — crewboard

**Scope**: backend-service (plus the `web` SPA)
**Last confirmed**: 2026-08-30 (desk visit)

## Containers

- `web` — React SPA, Render static site — built
- `api` — FastAPI, Render web service, two instances — built.
  Modules: `rotas`, `swaps`, `timeoff`, `staff`, `billing`, `reports`, `audit`
- `db` — PostgreSQL 16 — built
- `worker` — Celery 5 worker + beat, Render background worker — built.
  Modules: `notify`, `integrations.xero`, `payroll`
- `redis` — Redis 7, Celery broker and result backend — built

## External systems

Twilio (SMS) · Postmark (email) · Stripe (billing) · Xero (accounting sync) · PayFlow
(payroll upload)

## Communication styles

- `web → api`: HTTPS/JSON, session cookie
- `api → db`: synchronous SQL, one transaction per request
- `api → worker`: Celery tasks over `redis` for anything that must outlive the request
- `worker → providers`: synchronous HTTPS from tasks, retried by Celery with backoff
- `api → Stripe`: synchronous HTTPS from the billing handlers (plan change, card update) — interactive, owner-initiated, no retry; batch provider work stays on `worker`
- `Stripe → api`: signed webhooks, verified at the edge
