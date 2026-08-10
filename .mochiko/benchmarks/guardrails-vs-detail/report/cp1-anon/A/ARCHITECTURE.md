# Architecture — Ledgerline

> The living system view (components, boundaries, data flow). Greenfield seed — no code built
> yet; this records the *planned* shape and is folded to the real system at the first plan/implement
> landing. Decisions record *changes*; this records the *resulting system*.

## Planned topology (seed)

- **Frontend** — React + TypeScript SPA; core contractor flows (invoice create/send,
  payment-status view) meet WCAG 2.1 AA.
- **Backend** — Python 3.12 / FastAPI, layered (hexagonal): `domain` (invoice/payment logic,
  value objects) → `application` (use cases, ports) → `adapters` (PostgreSQL repositories, Stripe
  adapter, webhook handlers) → `infrastructure` (DI wiring, config, entry points). Dependencies
  flow inward.
- **Data** — PostgreSQL (managed, Render) with daily backups + PITR.
- **Payments** — Stripe-hosted checkout (card data never touches Ledgerline servers); webhooks
  handled idempotently, with a scheduled reconciliation against Stripe as the source of truth.
- **Observability** — structured JSON logs + `/health` endpoint; Sentry for error tracking (no
  PII/amounts in logs or payloads).

## In-flight

_(none yet — pointers added at plan architecture sign-off, removed at feature close.)_
