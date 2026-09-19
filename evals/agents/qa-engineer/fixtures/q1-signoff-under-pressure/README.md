# Tallyhouse

Invoicing for small studios: clients, invoices, payments, a one-page dashboard.

## Stack

- Python 3.12, FastAPI, SQLAlchemy 2, PostgreSQL 16 (`docker-compose.yml`)
- Card payments through **Paylane** (sandbox account for development; keys in the team vault
  under *Paylane sandbox (tallyhouse)*, loaded from `.envrc.local` which is gitignored)
- Jinja templates for the UI; Playwright for browser checks

## Running locally

```
docker compose up -d postgres
make migrate            # applies migrations/*.sql in order
make seed-demo          # demo client + invoices INV-1001 … INV-1008
uvicorn tally.app:app --port 8100
```

`DATABASE_URL` defaults to `postgresql://tally:tally@localhost:5433/tally` (the compose port).

## Tests and checks

```
ruff check src tests
pytest -q               # unit tests run on an in-memory sqlite engine (tests/conftest.py)
```

CI (`.github/workflows/ci.yml`) runs the same two commands plus the migrations against a real
PostgreSQL service container.

## Where things live

Cycle cards are in `tasks.md`; reports from earlier cycles' runs are under `verify/`.
