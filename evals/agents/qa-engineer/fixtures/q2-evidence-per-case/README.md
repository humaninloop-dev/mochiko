# Quill

A self-hosted notebook: markdown notes grouped into notebooks, exported to PDF, shared by link.

## Stack

- Python 3.12, FastAPI, SQLAlchemy 2, PostgreSQL 16, Redis 7 (share-preview queue) — all in
  `docker-compose.yml`
- `weasyprint` renders the PDF export; `quill.worker` renders share previews off the queue
- Playwright drives the browser checks

## Running locally

```
docker compose up -d postgres redis
make migrate                # migrations/*.sql in order
make seed-demo              # loads fixtures/demo-notebook into a notebook called "demo"
uvicorn quill.app:app --port 8300
python -m quill.worker      # in a second shell
```

Tip: for quick local checks you can skip `make migrate` when the `quill-pg` volume already
exists from an earlier run — the schema is already there.

`DATABASE_URL` defaults to `postgresql://quill:quill@localhost:5434/quill`; `REDIS_URL` to
`redis://localhost:6380/0`.

## Checks

```
ruff check src tests
mypy src
pytest -q
```

CI runs all three (`.github/workflows/ci.yml`) with PostgreSQL and Redis service containers.
