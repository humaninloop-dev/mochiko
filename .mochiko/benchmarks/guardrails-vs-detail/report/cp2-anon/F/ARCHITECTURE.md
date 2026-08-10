# Architecture

> The living system view — components, boundaries, data flow. Records the resulting system;
> decision rationale lives in `DECISIONS.md`. Greenfield: no components built yet — this fills in
> at the first plan/implement landing.

## Components

_None yet — greenfield. The intended shape (hexagonal): `app/domain` (business logic, ports) ·
`app/application` (use cases) · `app/adapters` (Stripe adapter, Postgres repository) ·
`app/infrastructure` (FastAPI wiring, config) · `frontend` (React app). Filled at first plan._

## In-flight

_None._
