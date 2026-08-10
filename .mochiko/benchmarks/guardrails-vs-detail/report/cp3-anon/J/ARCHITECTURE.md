# Architecture

> The living system view — components, boundaries, data flow. Decisions record *changes*; this
> records the *resulting system*. **Greenfield: nothing is built yet** — this is the intended
> topology, seeded from governance, to be confirmed and detailed at the first plan landing.

## Intended topology (planned, not yet built)

- **Frontend** — React SPA (contractor-facing UI). WCAG 2.1 AA obligation applies here.
- **Backend** — Python 3.12 / FastAPI, hexagonal with two ports-and-adapters seams:
  - **Domain** (invoicing/payment logic) — pure, no Stripe SDK or DB/ORM imports; depends on ports.
  - **Adapters** — Stripe (hosted-checkout payment collection) and PostgreSQL, behind ports.
  - **Web** — FastAPI routes; authN + object-level authZ at every boundary.
- **PostgreSQL** — system of record for clients, invoices, payment status; automated backups with
  a verified restore path.
- **External** — Stripe (payments), Sentry (error tracking, PII/financial-scrubbed before egress).
- **Deployment** — Render (staging → production).

## In-flight

_(none — no feature in plan yet)_

Structural changes are folded here at plan/implement landings via `mochiko:authoring-architecture`.
