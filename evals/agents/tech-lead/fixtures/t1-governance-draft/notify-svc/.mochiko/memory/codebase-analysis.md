# Codebase analysis — notify-svc

- **Date:** 2026-09-06 · **Method:** `detect-stack.sh` plus a read of `pyproject.toml`,
  `.github/workflows/ci.yml`, `.pre-commit-config.yaml`, and `src/notify/`.
- **Stack:** Python 3.12 · FastAPI 0.115 · SQLAlchemy 2 + Alembic · PostgreSQL 16 · Celery 5 on
  Redis 7 · Heroku (two web dynos, one worker) · GitHub Actions.
- **Team:** three engineers; one at senior level; no dedicated ops.

## Essential floor — as found

| Category | Status | Evidence |
|---|---|---|
| Security | partial | secrets via Heroku config vars; `gitleaks` in `.pre-commit-config.yaml` and in `ci.yml` (`gitleaks detect`); no dependency scan; pydantic validation in 14 of 19 handlers — `src/notify/api/webhooks.py` and `src/notify/api/admin.py` read `request.json()` directly |
| Testing | present | `pytest --cov=notify --cov-fail-under=70` in `ci.yml`, blocking; current coverage 72%; tests under `tests/` mirror `src/notify/`; PostgreSQL service container in CI; `tests/quarantine.txt` checked by `scripts/check_quarantine.py` in CI |
| Error handling | partial | shared handler in `src/notify/api/errors.py` returns `application/problem+json` for `NotifyError` subclasses; uncaught exceptions return FastAPI's default 500 body; no correlation id on any response |
| Observability | absent | `print()` and `logging.basicConfig` plain text to stdout; no request id, no structured fields; no `/health` route (Heroku pings `/`); no metrics; alerting is customers emailing support |

## Patterns worth codifying

- Alembic: 31 of 33 revisions carry a working `downgrade()`; two are `pass` stubs (`0012`,
  `0027`). Nothing checks this today.
- Every carrier module imports `notify.carriers.http` — a shared `httpx.Client` with
  `timeout=httpx.Timeout(10.0, connect=3.0)` and three retries via `tenacity`.
- List endpoints: 6 of 7 paginate with `limit`/`cursor`; `GET /admin/tenants` returns all rows.
- Timestamps: all 41 date-time columns are `timestamptz`; two API responses emit naive strings.
