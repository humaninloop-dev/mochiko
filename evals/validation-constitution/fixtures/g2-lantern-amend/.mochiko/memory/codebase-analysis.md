# Codebase Analysis

> Generated: 2026-04-14T09:20:00Z
> Mode: brownfield-setup
> Status: confirmed

---

## Part 1: Inventory (Factual)

### Project Identity

| Aspect | Value | Source |
|--------|-------|--------|
| Name | lantern | `pyproject.toml` |
| Primary Language | Python 3.12 | `pyproject.toml` (`requires-python`) |
| Framework | FastAPI 0.115 | `pyproject.toml` |
| Package Manager | uv | `uv.lock` |
| Entry Points | `src/lantern/api/main.py` · `src/lantern/worker.py` | detected |

### Directory Structure

```
lantern/
├── src/lantern/
│   ├── domain/            # entities, value objects, domain errors
│   ├── application/       # use cases, ports (Protocols), rollups/
│   ├── infrastructure/    # SQLAlchemy repositories, Stripe client, Postmark mailer, admin/
│   └── api/               # FastAPI routers, request/response schemas, problem.py
├── tests/                 # pytest; unit/ integration/
├── alembic/               # migrations
└── fly.toml
```

### Detected Patterns

#### Architecture Pattern

| Pattern | Evidence |
|---------|----------|
| Hexagonal (ports and adapters) | `application/ports/*.py` Protocols; `infrastructure/` implements them; `importlinter` contract in `pyproject.toml` (`domain` imports nothing above it) |

#### Error Handling Pattern

| Pattern | Evidence |
|---------|----------|
| Typed domain errors mapped to RFC 7807 in one handler | `domain/errors.py`; `api/problem.py` (`problem_handler`) |
| Bare `except Exception: pass` in two Celery tasks | `application/rollups/nightly.py:88`, `:142` |

#### Test Pattern

| Aspect | Value |
|--------|-------|
| Framework | pytest 8 + pytest-cov |
| Location | `tests/unit`, `tests/integration` (real PostgreSQL via testcontainers) |
| Naming | `test_*.py` |
| Coverage Config | `pyproject.toml` (`[tool.coverage]`, no `fail_under`) |

### External Dependencies

| Service | Access Pattern | Config Location |
|---------|----------------|-----------------|
| Stripe | hosted checkout + webhooks (`infrastructure/stripe/`) | Fly secrets |
| Postmark | weekly e-mail (`infrastructure/mail/`) | Fly secrets |
| PostgreSQL | SQLAlchemy 2, one engine per app | `DATABASE_URL` committed in `fly.toml` with the password (see Inconsistencies) |

## Part 2: Assessment (Judgment)

### Strengths to Preserve

1. **Repository protocols per aggregate** — every tenant-owned table is reached through a `Protocol` in `application/ports/` and one SQLAlchemy implementation; the shape makes tenant scoping reviewable in one place.
2. **One problem handler** — `api/problem.py` is the only place an error body is built.

### Inconsistencies Found

| Area | Finding | Severity | Location |
|------|---------|----------|----------|
| Secrets | The `DATABASE_URL` connection string, password included, is committed in `fly.toml` | high | `fly.toml:14` |
| Data access | Two report use cases build raw SQL strings and hand them to the `ReportStore` port, bypassing the repositories | high | `application/reports/cohort.py`, `application/reports/churn.py` |
| Error handling | Two swallowed exceptions in the nightly rollup | high | `application/rollups/nightly.py:88`, `:142` |
| CI | GitHub Actions runs `ruff` and `mypy` only; `pytest` is not in the workflow | medium | `.github/workflows/ci.yml` |

### Essential Floor Status

| Category | Check | Status | Evidence |
|----------|-------|--------|----------|
| Security | Auth at boundaries | present | `api/deps.py` `require_tenant` on every router |
| Security | Secrets from env | partial | Stripe and Postmark keys in Fly secrets; `DATABASE_URL` with its password committed in `fly.toml` |
| Security | Input validation | present | Pydantic request schemas on every route |
| Testing | Test framework configured | present | `pyproject.toml` |
| Testing | Test files present | present | 214 tests under `tests/` |
| Testing | CI runs tests | absent | `.github/workflows/ci.yml` runs `ruff` and `mypy` only |
| Error Handling | Explicit error types | present | `domain/errors.py` |
| Error Handling | Context preservation | partial | two swallowed exceptions (above) |
| Error Handling | Appropriate status codes | present | `api/problem.py` |
| Observability | Structured logging | absent | `logging.basicConfig` plain text |
| Observability | Correlation IDs | absent | none |
| Observability | No PII in logs | partial | tenant e-mail logged at `api/routers/auth.py:41` |

**Category rollup:** Security partial · Testing partial · Error Handling partial · Observability absent

### Recommended Constitution Focus

1. Move `DATABASE_URL` out of `fly.toml` into Fly secrets before anything else, then put `pytest` in CI before asserting any coverage threshold.
2. Codify the repository protocols and close the raw-SQL bypass in `application/reports/`.
3. Structured logging with a correlation id is the largest gap; treat it as a MUST-implement.

## Appendix: Detection Method

| Aspect | Method Used |
|--------|-------------|
| Tech Stack | `detect-stack.sh` |
| Architecture | directory pattern matching + `importlinter` contract |
| Conventions | file sampling |
