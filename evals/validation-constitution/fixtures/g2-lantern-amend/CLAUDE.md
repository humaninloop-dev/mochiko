# Lantern — Founder's Notes

Lantern turns a founder's Stripe account into the five numbers they actually check. Stack and
commands in `README.md`. The nightly rollup is the one job that must never double-count; read
`src/lantern/application/rollups/README.md` before touching it.

## How I work

- Small PRs, one use case each. The contractor reviews mine; I review theirs.
- Every customer is a `tenant`. Nothing is ever queried without one.

<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v1.0.1 · 2026-09-10 · production floor · depth: high · modules: none <!-- GI-001 (fact profile) · GI-003 (depth level) -->

### Principles

- Secrets MUST stay out of the repository: loaded from Fly secrets, never committed; `gitleaks` blocks merge on any finding and `pip-audit` blocks merge on any high or critical advisory (NON-NEGOTIABLE) <!-- GI-004 -->
- `pytest` MUST pass before merge; coverage MUST be maintained at an appropriate level and the baseline MUST NOT decrease (NON-NEGOTIABLE) <!-- GI-005 -->
- A failure MUST NOT silently corrupt a rollup or a tenant's numbers; every error response is RFC 7807 Problem Details with a `correlation_id`, and no stack trace reaches a response body (NON-NEGOTIABLE) <!-- GI-006 -->
- Logs MUST be structured JSON with a `correlation_id` on every line, MUST NOT contain personal data, and `/healthz` MUST answer on every app (NON-NEGOTIABLE) <!-- GI-007 -->
- Tenant-scoped data access — see `.claude/rules/mochiko/data-access.md` <!-- GI-004 -->
- Layer rules — see `.claude/rules/mochiko/layers.md` <!-- GI-011 -->
- Services MUST use the repository pattern <!-- GI-010 -->
- Commits MUST be GPG-signed <!-- GI-013 -->

### Technology stack

- Python 3.12 · FastAPI 0.115 · SQLAlchemy 2.0 · Alembic · Celery 5 on Redis 7 <!-- GI-002 -->
- PostgreSQL 16 on Fly.io Postgres · Stripe (hosted checkout and webhooks) · Postmark for the weekly e-mail <!-- GI-002 -->

### Quality gates

- `ruff check .` and `mypy src` MUST pass before merge <!-- GI-002 -->
- `pytest --cov=lantern` MUST pass before merge; the coverage baseline MUST NOT decrease <!-- GI-005 -->
- `lint-imports` MUST pass before merge <!-- GI-011 -->
- `gitleaks detect --source .` and `pip-audit` MUST pass before merge <!-- GI-004 -->

### Governance operations

- Ledger (waivers · amendment policy · exceptions · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup` (fact-profile changes — module attach/detach — and un-waives are governance events)
<!-- mochiko:output-style:begin -->
- Writing style — conversation: `full` · reports: `ultra` *(internal agent hand-offs)* · documents: `full`. Terse and plain-English by default. Set any of the three to `off`, `lite`, `full`, or `ultra` and your choice is kept when this section is regenerated; "normal mode" turns it off for one session.
<!-- mochiko:output-style:end -->
- Path-scoped rules inject on **Read**, not Write (observed behavior, kinako dogfood 2026-07-19) — before creating a new file under `src/` or `tests/`, read the matching `.claude/rules/mochiko/` file or read back the file you created
- Release gates: `staging` soak then a manual `lantern-prod` deploy; rollback is `fly releases rollback` within 10 minutes — detail in the ledger <!-- GI-012 -->
- Evolution notes (brownfield floor status and gap references): the ledger's Evolution notes section <!-- GI-013 -->
<!-- mochiko:governance:end -->

## Reminders to self

- Rotate the Postmark token in October.
- The contractor's Fly access expires with the contract; renew or revoke on the last Friday.
