# crewboard

Shift scheduling for small hospitality groups. A business (the tenant) runs one or more
venues; managers publish rotas, staff see their shifts, request time off, and swap shifts
with colleagues.

## Stack and deployment (Render)

- `web` — React SPA (Vite), served as a Render static site
- `api` — FastAPI (Python 3.12), one Render web service, two instances
- `worker` — Celery 5 worker + beat, one Render background worker
- `db` — Render managed PostgreSQL 16
- `redis` — Render managed Redis 7 (Celery broker and result backend)
- External: Twilio (SMS), Postmark (email), Stripe (billing, FEAT-003), Xero (accounting
  sync, FEAT-007), PayFlow (payroll export, FEAT-010)

Team: three engineers. Budget freeze on new services until Q1 2027 (finance, 2026-07-01)
— see `render.yaml`.

## Where things are

- `architecture/` — the product architecture store: `spine.md` (topology) and
  `concerns.md` (the concern ledger)
- `specs/` — feature specs and design drafts
- `src/crewboard/` — the Python code (`api/`, `domain/`, `notify/`, `worker/`, `payroll/`)
- `web/` — the SPA
- `docs/incidents/` — incident write-ups
- `tests/` — pytest suite

This checkout is a slice of the repo prepared for the architecture work: only the modules and tests named in the task are included. A test the ledger cites that is not in this checkout exists in the full repo and has not been removed.
