# Architecture — Ledgerline

> The living system view — components, boundaries, data flow. Present tense, current state.
> Folded at plan/implement landings on structural change (`mochiko:authoring-architecture`).

## Current state

Greenfield — no components built yet. The intended shape (from governance, GI-007) is a light
ports-and-adapters backend:

- **domain** — invoices, payments, tax; pure logic, no I/O, exact `Decimal` money math.
- **application** — use cases; defines ports (Protocols).
- **adapters** — Stripe (payment), PostgreSQL (persistence), email (reminders) — behind ports.
- **infrastructure** — FastAPI wiring, config, entry points; `/health` endpoint.
- **frontend** — React SPA (WCAG 2.1 AA).

External systems: Stripe (hosted checkout — no card data on Ledgerline), a managed Postgres, an
email provider. The first `/mochiko:specify` + `/mochiko:plan` runs draw the real container-level
delta.

## In-flight

_(none)_
