# Ledgerline

Invoicing and payment-tracking SaaS for US small independent contractors. Backend: Python/FastAPI
on Postgres. Frontend: React. Payments: Stripe hosted checkout. Multi-tenant; solo-founder team.

<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v1.0.0 · 2026-08-10 · production floor · modules: compliance a11y (legal-mandate) · knowledge-management (core + CHANGELOG + RUNBOOK) · layer-rules · release-gates <!-- GI-001 (fact profile) -->

### Principles

<!-- Universal principles: the operative line IS the governance. Scope-bound principles: the line
     is an index entry pointing at the home. -->
- Security by default — secrets MUST stay out of the repo (env vars + `.gitignore`); CI MUST run secret scanning; all external inputs MUST be validated at API boundaries; authentication MUST be enforced at every API endpoint; dependency vulnerability scanning MUST block merge on high/critical (NON-NEGOTIABLE) <!-- GI-003 -->
- Testing discipline — all tests MUST pass in CI before merge, and a smoke test on the critical financial path MUST exist from day one (NON-NEGOTIABLE) <!-- GI-004 -->
- Error handling — failures MUST NOT silently corrupt financial data; the API MUST return a consistent error contract (RFC 7807); stack traces MUST NOT leak; errors MUST carry correlation IDs (NON-NEGOTIABLE) <!-- GI-005 -->
- Observability — logs MUST be structured JSON, MUST NOT contain PII or secrets; a `/health` endpoint MUST exist; runtime errors MUST be reported to Sentry (NON-NEGOTIABLE) <!-- GI-006 -->
- CI is the gate — merge to the deploy branch MUST be blocked while any CI check (lint, tests, secret scan, dependency audit) is red <!-- GI-010 -->
- Data durability — invoice and payment data MUST be covered by automated database backups, and a restore MUST be tested before launch and periodically after <!-- GI-012 -->
- Dependency discipline — new dependencies MUST be justified in the commit/PR and versions pinned in lock files <!-- GI-009 -->
- Complexity limit — cyclomatic complexity MUST be ≤10 per function, enforced in CI (quality gates) <!-- GI-008 -->
- Email integrity — invoice email MUST be sent from a domain configured with SPF, DKIM, and DMARC; payment links MUST be integrity-protected <!-- GI-015 -->
- Layer boundaries — see `.claude/rules/mochiko/layer-boundaries.md` <!-- GI-007 -->
- Accessibility — see `.claude/rules/mochiko/accessibility.md` <!-- GI-011 -->
- Financial audit trail — see `.claude/rules/mochiko/financial-audit.md` <!-- GI-013 -->
- Tenant isolation — see `.claude/rules/mochiko/tenant-isolation.md` <!-- GI-014 -->

### Technology stack

- Python 3.12 · FastAPI · Postgres · SQLAlchemy + Alembic (reversible migrations) · Pydantic (validation) <!-- GI-002 -->
- React (TypeScript) frontend · Stripe hosted checkout (no card data stored) <!-- GI-002 -->
- Hosting: Render (staging → production) · CI: GitHub Actions · Error tracking: Sentry <!-- GI-002 -->

### Quality gates

- `ruff check .` and `ruff format --check .` MUST pass before merge <!-- GI-003 -->
- `ruff check --select C901 .` MUST pass — cyclomatic complexity ≤10 per function <!-- GI-008 -->
- `pytest` MUST pass, including the critical financial-path test (invoice create, amount/tax/rounding correctness, and the Stripe webhook flipping payment status) <!-- GI-004 -->
- `eslint .` (incl. `eslint-plugin-jsx-a11y`) and `prettier --check .` MUST pass before merge <!-- GI-011 -->
- `gitleaks detect` MUST pass — no secrets committed <!-- GI-003 -->
- `pip-audit` and `npm audit` MUST report no high/critical vulnerabilities <!-- GI-009 -->
- Coverage: numeric coverage gate is waived (see ledger, GI-019) — protection is the financial-path test + all-tests-green gate <!-- GI-019 -->

### Governance operations

- Ledger (waivers · amendment policy · exceptions · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup` (fact-profile changes — module attach/detach — and un-waives are governance events); standing revisit triggers: targeting EU/UK users or clients (GDPR), a signed attestation/SOC 2 commitment
<!-- mochiko:output-style:begin -->
- Writing style — conversation: `full` · reports: `ultra` *(internal agent hand-offs)* · documents: `full`. Terse and plain-English by default. Set any of the three to `off`, `lite`, `full`, or `ultra` and your choice is kept when this section is regenerated; "normal mode" turns it off for one session.
<!-- mochiko:output-style:end -->
- Path-scoped rules inject on **Read**, not Write (observed behavior) — before creating a new file under `backend/`, `frontend/`, or `.mochiko/specs/`, read the matching `.claude/rules/mochiko/` file or read back the file you created
- Release gates: Render staging → production; rollback = redeploy last good image (≤15 min); Alembic migrations reversible; destructive migrations flagged for explicit approval — detail in the ledger <!-- GI-018 -->
- Operating docs (knowledge-management module): sessions in `.mochiko/brainstorms/` + `index.md`; rulings land in `DECISIONS.md`; open threads in `BACKLOG.md`; direction in `ROADMAP.md`; landing ritual + invariants at `.mochiko/memory/knowledge-management.md`; groom: `mochiko:grooming-operating-docs` <!-- GI-016 -->
<!-- mochiko:governance:end -->
