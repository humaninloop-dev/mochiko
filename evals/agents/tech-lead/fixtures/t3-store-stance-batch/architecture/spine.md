# Architecture Spine — notify-svc

**Scope**: backend-service

## Containers

- `api` — FastAPI, two Heroku web dynos — built
- `db` — PostgreSQL 16 — built
- `worker` — Celery 5 worker + beat, one dyno — built
- `redis` — Redis 7, Celery broker — built

## Boundaries

- Recipient traffic enters `api` with a shop-issued token (AX-002); admin traffic with OIDC.
- Every tenant table carries `tenant_id`; scoping per AX-001.
