# Architecture spine — crewboard

**Scope**: backend-service (plus the `web` SPA)
**Last confirmed**: 2026-07-02 (FEAT-010 landing)

## Containers

- `web` — React SPA, Render static site — built
- `api` — FastAPI, Render web service, two instances — built.
  Modules: `rotas`, `swaps`, `timeoff`, `staff`, `billing` (Stripe), `reports`
- `db` — PostgreSQL 16 — built
- `worker` — Celery 5 worker + beat, Render background worker — built.
  Modules: `notify` (Twilio, Postmark), `integrations.xero`, `payroll` (PayFlow upload)
- `redis` — Redis 7, Celery broker and result backend — built

## External systems

- Twilio — SMS to staff and managers
- Postmark — transactional email
- Stripe — subscription billing (FEAT-003)
- Xero — nightly accounting sync (FEAT-007)
- PayFlow — nightly payroll upload (FEAT-010)

## Communication styles

- `web → api`: HTTPS/JSON, session cookie
- `api → db`: synchronous SQL (SQLAlchemy), one transaction per request
- `api → worker`: Celery tasks over `redis` for anything that must outlive the request —
  rota-publish notifications (FEAT-006), shift reminders (FEAT-009), nothing else today
- `worker → Twilio / Postmark / Xero / PayFlow`: synchronous HTTPS from tasks, retried by
  Celery with backoff
- `api → Twilio / Postmark`: **never** — no provider is called from a request handler
  (AX-003)
- `Stripe → api`: signed webhooks, verified at the edge, processed in the request

## Boundaries

- Staff and managers share one trust domain (session cookie + role per venue); venue and
  business scoping per AX-001.
- Providers sit behind `crewboard.notify` (Twilio, Postmark), `crewboard.integrations`
  (Xero), and `crewboard.payroll` (PayFlow). No other module imports a provider SDK.
- The worker runs one queue (`celery`) with concurrency 8 since 2026-08-28.
