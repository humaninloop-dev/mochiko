# notify-svc — Operating Manual

notify-svc is the parcel-tracking and delivery-window service behind the ShopLoop checkout.
Python 3.12 · FastAPI · PostgreSQL 16 · Celery 5 on Redis · Heroku · GitHub Actions. Three
engineers, one at senior level, no dedicated ops.

## Governance

**Draft 2 — 2026-09-08 — prepared by the platform working group for adoption at Thursday's
team meeting.** Written against `.mochiko/memory/codebase-analysis.md`.

### Principles

- **Security — secrets.** Secrets MUST come from environment variables injected by the deploy
  pipeline; no secret in the repo or in a config file. Enforcement: the `gitleaks` pre-commit
  hook and the CI secret scan (`gitleaks detect`) block the push and the merge. Pass: zero
  findings. Rationale: one leaked carrier API key rotates every tenant's integration at once.
- **Security — input.** Every request handler MUST validate its body and query parameters with
  a pydantic model before touching the database. Pass: no handler reads `request.json()` or a
  raw query string directly. Rationale: unvalidated input reached the carrier lookup in INC-41.
- **Security — review.** Every pull request MUST include a written threat model covering the
  change, reviewed and signed off by two senior engineers before merge. Enforcement: the PR
  template carries a threat-model section; the merge is blocked until both sign-offs are
  recorded. Pass: two sign-offs present on every merged PR. Rationale: threat modelling catches
  design-level flaws that code review misses.
- **Testing — coverage.** New behaviour MUST ship with tests that fail before the change and
  pass after it; test coverage MUST NOT fall below 60% (blocking). Enforcement: CI runs
  `pytest --cov --cov-fail-under=60` on every pull request and blocks merge on failure. Pass:
  suite green and coverage ≥ 60%. Rationale: the suite is the only regression net a
  three-person team can afford.
- **Testing — layout.** Test files MUST live under `tests/` mirroring `src/notify/` and be
  named `test_*.py`. Enforcement: `pytest` collection is restricted to that pattern in
  `pyproject.toml`; a misnamed file is never collected and the reviewer flags it. Pass: every
  module under `src/notify/` has a `tests/` counterpart. Rationale: predictable placement keeps
  the suite discoverable.
- **Error handling — format.** Error responses MUST use RFC 7807 Problem Details
  (`application/problem+json`) with `type`, `title`, `status`, and `correlation_id`.
  Enforcement: the shared exception handler in `src/notify/api/errors.py` is the only place a
  4xx/5xx body is built; the contract suite asserts the schema on every error response. Pass:
  every error response in the contract suite validates. Rationale: consistent errors let the
  ShopLoop frontend render one error component and let support match a ticket to a log line.
- **Error handling — traces.** Stack traces MUST NOT appear in any production response.
  Enforcement: `DEBUG=false` is asserted at startup in production; the contract suite asserts
  no `traceback` key on any error body. Pass: both assertions hold. Rationale: traces leak
  paths, versions, and occasionally secrets.
- **Data — time.** All timestamps MUST be stored and exchanged in UTC with an explicit offset.
  Enforcement: the `check_utc` migration lint in CI rejects any `timestamp without time zone`
  column; the contract suite asserts `Z`-suffixed values on every date-time field. Pass: lint
  and suite green.
- **Data — migrations.** Database migrations MUST be reversible: every Alembic revision ships
  a working `downgrade()`. Pass: `downgrade()` runs cleanly on a copy of production data.
  Rationale: a failed deploy at 02:00 needs a one-command rollback, not a hand-written repair.

### Observability

Not covered in this draft. We'll add monitoring later, once the carrier integrations settle;
for now the on-call engineer checks the Heroku logs when a customer reports a problem.

### Rules files

- `.claude/rules/api.md` — handler and outbound-call conventions (`src/notify/api/**`,
  `src/notify/carriers/**`)
- `.claude/rules/testing.md` — test conventions (`tests/**`)
