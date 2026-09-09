# crewboard

Shift scheduling for small hospitality groups. A business (the tenant) runs one or more
venues; managers publish rotas, approve worked hours, and export them to payroll.

## Stack and deployment (Render)

- `web` — React SPA (Vite), Render static site
- `api` — FastAPI (Python 3.12), one Render web service, two instances
- `worker` — Celery 5 worker + beat, one Render background worker
- `db` — Render managed PostgreSQL 16
- `redis` — Render managed Redis 7 (Celery broker and result backend)
- External: Twilio (SMS), Postmark (email), Stripe (billing, FEAT-003), Xero (accounting
  sync, FEAT-007)

Team: three engineers; 58 businesses, 180 venues (2026-05). Budget: no new services this
quarter; vendor additions wait on the SOC 2 audit (closes 2027-Q1).

## Where things are

- `architecture/` — the product architecture store: `spine.md` and `concerns.md`
- `specs/` — feature specs; `specs/FEAT-010-payroll-export/` holds the spec and the CTO's
  sketch
- `docs/` — partner API notes
- `src/crewboard/` — the Python code

This checkout is a slice of the repo prepared for the architecture work: only the modules and tests named in the task are included. A test the ledger cites that is not in this checkout exists in the full repo and has not been removed.
