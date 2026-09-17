# Fieldnote

Job dispatch for small field-service firms (plumbers, electricians, HVAC). A dispatcher
creates jobs and assigns technicians; technicians see their day on a phone web app;
customers get an SMS with the technician's ETA; new customers book through a public form.

## Stack and deployment (Fly.io)

- `api` — Go 1.22, chi router, one Fly app, two machines
- `worker` — Go 1.22, the same binary tree, one Fly machine, polls the `jobs` table
- `db` — Fly Postgres 16
- `web` — React SPA, served from `api` as static files
- External: Twilio (SMS), Stripe (subscriptions, not yet live), Mapbox (ETA)

Team: two engineers and a founder. 19 accounts, 140 technicians (2026-06).

## Where things are

- `cmd/api/`, `cmd/worker/` — the two process entry points
- `internal/` — dispatch, notify, worker, store
- `.mochiko/` — the governance and product baselines the June setup run wrote
- `ARCHITECTURE.md` — the founder's architecture write-up from 2025-11

This checkout is a slice of the repo prepared for the architecture work: only the modules
named above are included. A test or module the docs cite that is not in this checkout exists
in the full repo and has not been removed.
