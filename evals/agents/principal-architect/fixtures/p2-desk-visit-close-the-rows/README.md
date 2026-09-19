# crewboard

Shift scheduling for small hospitality groups. A business (the tenant) runs one or more
venues; managers publish rotas, staff see their shifts, request time off, and swap shifts
with colleagues.

## Stack and deployment (Render)

- `web` — React SPA (Vite), Render static site
- `api` — FastAPI (Python 3.12), one Render web service, two instances
- `worker` — Celery 5 worker + beat, one Render background worker
- `db` — Render managed PostgreSQL 16
- `redis` — Render managed Redis 7
- External: Twilio, Postmark, Stripe, Xero, PayFlow

Team: three engineers; 64 businesses, 410 venues (2026-08). Budget freeze on new services
until Q1 2027.

## Where things are

- `architecture/` — the product architecture store: `spine.md` and `concerns.md`. The
  concern ledger is walked at desk visits with the CTO.
- `specs/` — feature specs and approved architecture deltas
- `src/crewboard/` — the Python code
- `docs/` — changelog, monthly metrics, incident write-ups

This checkout is a slice of the repo prepared for the architecture work: only the modules and tests named in the task are included. A test the ledger cites that is not in this checkout exists in the full repo and has not been removed.
