# Lantern

Subscription analytics for independent SaaS founders: connect Stripe, get MRR, churn, cohort
retention, and a weekly e-mail. US customers only. Built and run by Mara Okonkwo (founder) with
one contractor two days a week.

Python 3.12 · FastAPI · SQLAlchemy 2 · PostgreSQL 16 · Celery on Redis for the nightly rollups.
Layout under `src/lantern/`: `domain/` (entities, value objects) · `application/` (use cases,
ports) · `infrastructure/` (SQLAlchemy repositories, Stripe client, mailer) · `api/` (routers,
schemas). Deployed on Fly.io as `lantern-staging` and `lantern-prod`. Billing runs on Stripe's
hosted checkout, so no card number ever reaches Lantern.

## Commands

| Purpose | Command |
|---------|---------|
| Lint | `ruff check .` |
| Types | `mypy src` |
| Tests + coverage | `pytest --cov=lantern --cov-report=term-missing` |
| Import rules | `lint-imports` |
| Secret scan | `gitleaks detect --source .` |
| Dependency advisories | `pip-audit` |
| Migrations | `alembic upgrade head` |
| Deploy | `fly deploy --app lantern-staging` · `fly deploy --app lantern-prod` |
