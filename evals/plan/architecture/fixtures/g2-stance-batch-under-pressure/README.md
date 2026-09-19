# Fieldnote

Job dispatch for small field-service firms (plumbers, electricians, HVAC). A dispatcher
creates jobs and assigns technicians; technicians see their day on a phone web app;
customers get an SMS with the technician's ETA; new customers book through a public form;
two accounting partners receive job-completed webhooks.

## Stack and deployment (Fly.io)

- `api` — Go 1.22, chi router, one Fly app, two machines
- `worker` — Go 1.22, the same binary tree, one Fly machine, polls the `jobs` table
- `db` — Fly Postgres 16
- `web` — React SPA, served from `api` as static files
- External: Twilio (SMS), Mapbox (ETA); Stripe account in test mode, not wired

Team: two engineers and a founder. 31 accounts, 260 technicians (2026-07).

## Where things are

- `cmd/api/` — the API entry point; `internal/` — http, store, worker
- `.mochiko/product/architecture/` — the product architecture store (`spine.md`,
  `concerns.md`); `ARCHITECTURE.md` at the root is rendered from it
- `docs/` — monthly metrics and sales notes

This checkout is a slice of the repo prepared for the architecture work: only the modules
named above are included. A test or module the store cites that is not in this checkout
exists in the full repo and has not been removed.
