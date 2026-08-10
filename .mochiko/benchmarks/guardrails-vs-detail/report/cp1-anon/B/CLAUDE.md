# Ledgerline

Invoicing and payment-tracking SaaS for solo US independent contractors.

<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v0.1.0 · 2026-08-10 · production floor · modules: none <!-- GI-001 (fact profile) -->

### Principles

- Security by default: secrets MUST stay out of the repo (env vars + `.gitignore`), secret scanning MUST block merge, dependency vulnerability scanning MUST block merge on high/critical, and auth MUST be enforced at all boundaries (NON-NEGOTIABLE) <!-- GI-003 -->
- Testing discipline: every change MUST ship with tests; coverage <60% MUST block merge (warn <80%); the coverage baseline MUST NOT decrease; a payment-state smoke test MUST exist (NON-NEGOTIABLE) <!-- GI-004 -->
- Error handling: failures MUST NOT silently corrupt data; errors MUST surface in a consistent form (RFC 7807 JSON on the API, error states in the UI) with correlation IDs and no leaked stack traces — see `.claude/rules/mochiko/error-handling.md` (NON-NEGOTIABLE) <!-- GI-005 -->
- Observability: logs MUST be structured with correlation IDs, MUST NOT contain PII, and a `/health` endpoint MUST exist (NON-NEGOTIABLE) <!-- GI-006 -->
- Financial correctness (payment state, money precision, webhook idempotency, audit trail) — see `.claude/rules/mochiko/financial-correctness.md` <!-- GI-010, GI-020, GI-021, GI-022 -->
- API authorization & tenant isolation — see `.claude/rules/mochiko/api-security.md` <!-- GI-011 -->
- Frontend security (no secrets in the bundle, input validation / XSS defense) — see `.claude/rules/mochiko/frontend-security.md` <!-- GI-003 (frontend expression) -->
- Architecture: ports around external systems, single-responsibility modules — see `.claude/rules/mochiko/architecture.md` <!-- GI-007, GI-008 -->
- Dependency discipline: dependencies pinned in a lock file; `pip-audit` blocks high/critical CVEs (unpatchable-CVE escape hatch via the ledger exception registry) <!-- GI-009 -->

### Technology stack

- Python 3.12+ · FastAPI · PostgreSQL · SQLAlchemy · Pydantic · React (TypeScript) frontend · Stripe (hosted checkout) · deploy on Render <!-- GI-002 -->
- Tooling: pytest · Ruff (lint + format, incl. C901 complexity) · pip-audit · gitleaks · GitHub Actions CI <!-- GI-002 -->

### Quality gates

- `pytest` MUST pass before merge <!-- GI-004 -->
- Coverage ≥ 60% blocking / ≥ 80% warning on new code (`pytest --cov=. --cov-fail-under=60`); baseline MUST NOT decrease <!-- GI-004 -->
- `ruff check .` MUST pass before merge (includes cyclomatic complexity ≤ 10, rule C901) <!-- GI-008 -->
- Secret scan `gitleaks detect` MUST pass before merge <!-- GI-003 -->
- `pip-audit` MUST pass with no high/critical CVEs before merge (unpatchable-CVE exception via the ledger) <!-- GI-009 -->
- Payment-state tests — Stripe-confirmed path, manual mark-as-paid path, and duplicate-webhook (idempotency) — MUST pass before merge <!-- GI-010, GI-021 -->

### Governance operations

- Ledger (waivers · amendment policy · exceptions · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup` (fact-profile changes — module attach/detach — and un-waives are governance events)
<!-- mochiko:output-style:begin -->
- Writing style — conversation: `full` · reports: `ultra` *(internal agent hand-offs)* · documents: `full`. Terse and plain-English by default. Set any of the three to `off`, `lite`, `full`, or `ultra` and your choice is kept when this section is regenerated; "normal mode" turns it off for one session.
<!-- mochiko:output-style:end -->
- Path-scoped rules inject on **Read**, not Write (observed behavior, kinako dogfood 2026-07-19) — before creating a new file under `src/`, `frontend/`, `tests/`, or `.mochiko/specs/`, read the matching `.claude/rules/mochiko/` file or read back the file you created <!-- standing line: emit whenever the set includes any rules file -->
- Operating docs (knowledge-management module): sessions in `.mochiko/brainstorms/` + `index.md`; rulings land in `DECISIONS.md`; open threads in `BACKLOG.md`; direction in `ROADMAP.md`; landing ritual + invariants at `.mochiko/memory/knowledge-management.md`; groom: `mochiko:grooming-operating-docs` <!-- GI-012 -->
- Release gates: green CI (tests + coverage + lint + secret/dep scans) before ship; environments, rollback, and gate table in the ledger <!-- GI-013 -->
<!-- mochiko:governance:end -->
