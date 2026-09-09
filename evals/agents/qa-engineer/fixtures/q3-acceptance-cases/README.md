# Shiftbook

Rota and shift management for cafés and small restaurants: managers publish a rota, staff see
their shifts, and (this feature) staff swap shifts among themselves with a manager's approval.

## Stack

- Python 3.12, FastAPI + HTMX server-rendered pages, SQLAlchemy 2, PostgreSQL 16
  (`docker-compose.yml`)
- SMS through **Textline**; the development account is a sandbox (10 outbound messages per
  day; keys in the team vault under *Textline sandbox (shiftbook)*, loaded from `.envrc.local`)
- Playwright for browser checks

## Running locally

```
docker compose up -d postgres
make migrate
make seed-demo          # a café with 6 staff, a published rota for next week
uvicorn shiftbook.app:app --port 8500
```

## Checks

`ruff check src tests` · `pytest -q` — both in `.github/workflows/ci.yml` with a PostgreSQL
service container.

## Where things live

Cycle cards are in `design/cycles.md`; each card closes with its verification block. The feature
spec is `spec.md`; the data model is `design/data-model.md`.
