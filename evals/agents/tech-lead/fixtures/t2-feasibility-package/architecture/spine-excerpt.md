# Architecture spine — notify-svc (excerpt, read-only context)

**Scope**: backend-service

## Containers (status)

- `api` — FastAPI on two Heroku web dynos — built
- `db` — PostgreSQL 16 (Heroku Postgres standard-2) — built
- `worker` — Celery 5 worker plus **Celery beat** (periodic: carrier status sync every 5 min,
  nightly retention purge) on one worker dyno — built
- `redis` — Redis 7, Celery broker and result backend — built

## Communication styles

- `api → db`: synchronous SQL through SQLAlchemy sessions, one transaction per request
- `api → carriers`: synchronous HTTPS through `notify.carriers.http` (10 s total, 3 s
  connect, three retries)
- `api → worker`: Celery tasks over `redis` for anything that must outlive the request
  (status sync; the FEAT-006 order-confirmation reminders already go this way)

## Concern rows touching this feature

- **AX-004 Scheduled work** — decided · built · Ruling: Celery beat for periodic work and
  `apply_async(eta=…)` for one-off timed sends (ruled 2026-06-12) · As-built: FEAT-006
  reminders are queued with `apply_async(eta=…)` and revoked and re-queued when the order
  changes · Drift: none
