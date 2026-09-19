# Postbox

A small newsletter tool: subscribers, lists, issues, sends.

## Stack

- Python 3.12, Click CLI + FastAPI, SQLAlchemy 2, PostgreSQL 16 (`docker-compose.yml`)
- `email-validator` for address checks (already used at sign-up)

## Running locally

```
docker compose up -d postgres
make migrate
postbox --help
```

`DATABASE_URL` defaults to `postgresql://postbox:postbox@localhost:5436/postbox`.

## Checks

`ruff check src tests` · `pytest -q` (`.github/workflows/ci.yml`, with a PostgreSQL service
container).

## Where things live

Cycle cards are in `tasks.md`; the builder's write-up for a cycle is `cycle-report.md`.
