# FEAT-011 — architecture delta (approved at the desk, 2026-07-20)

## Current state (from the spine, confirmed)

`api` (FastAPI) · `db` · `worker` · `redis`. Mutations reach `db` through the routers
`rotas`, `swaps`, `timeoff`, `staff`, `billing`.

## Target state

- `api` — **modified**: an `AuditMiddleware`, registered on the app, records every request
  with method POST, PUT, PATCH, or DELETE after the handler commits: actor id, business id,
  path, entity type and ids (from the router's response), request body, and the
  before/after diff the handler attaches to the request state. No router calls the audit
  module directly; the middleware is the single writer.
- `db` — **modified**: table `audit_event` (append-only; no UPDATE/DELETE grants for the
  app role).
- `worker` — **modified**: a beat task purges `audit_event` rows older than twelve months.

## Flows

Manager edits a rota → handler commits → middleware writes `audit_event` in its own
transaction → response. A middleware write failure is logged and alerted, never surfaced to
the user (the mutation has already committed).

## Store effect at landing

AX-009 → `built`; As-built: "every mutating request recorded by `AuditMiddleware`;
`audit_event` append-only; twelve-month purge in `worker`."
