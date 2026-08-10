# Ledgerline

SaaS invoicing + payment-tracking for small independent US contractors.

<!-- Content outside the governance markers is user territory — setup never touches it. -->

<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v1.0.0 · 2026-08-10 · production floor · modules: a11y (WCAG, legal-mandate) <!-- GI-001 (fact profile) -->

### Principles

- Security by default: secrets out of the repo, auth + input validation on every endpoint, no raw cardholder data, dependency + secret scanning in CI (NON-NEGOTIABLE) <!-- GI-003 -->
- Never handle raw card numbers or CVV — all card entry goes through Stripe hosted checkout (NON-NEGOTIABLE) <!-- GI-014 -->
- Testing discipline: tests before merge, coverage ≥60% blocking / ≥80% warning, ratchet, a smoke test on the invoicing critical path (NON-NEGOTIABLE) <!-- GI-004 -->
- Error handling: failures never silently corrupt data, RFC 7807 error bodies, correlation IDs, no stack traces to users (NON-NEGOTIABLE) <!-- GI-005 -->
- Observability: structured JSON logs with correlation IDs, `/health` endpoint, no PII in logs (NON-NEGOTIABLE) <!-- GI-006 -->
- Transactional email (invoices, reminders) MUST be authenticated (SPF/DKIM/DMARC) and handle bounces <!-- GI-030 -->
- Tenant isolation — see `.claude/rules/mochiko/tenant-isolation.md` <!-- GI-011 -->
- Invoice & payment-state integrity (webhook trust · money as Decimal · append-only audit trail) — see `.claude/rules/mochiko/payment-integrity.md` <!-- GI-012, GI-013, GI-026, GI-029 -->
- Hexagonal layering (domain isolated behind ports) — see `.claude/rules/mochiko/layers.md` <!-- GI-007 -->
- Code quality (single responsibility, complexity budget) — see `.claude/rules/mochiko/code-quality.md` <!-- GI-008 -->
- Dependency discipline — see `.claude/rules/mochiko/dependencies.md` <!-- GI-009 -->
- Accessibility (WCAG 2.1 AA on the contractor-facing app) — see `.claude/rules/mochiko/accessibility.md` <!-- GI-010 -->

### Technology stack

- Python 3.12 · FastAPI · PostgreSQL (Render managed) · pydantic (validation + value objects) · `decimal.Decimal` for all money <!-- GI-002, GI-013, GI-018 -->
- React · TypeScript · Vite · deployed on Render <!-- GI-002 -->

### Quality gates

- `ruff check .` and `black --check .` MUST pass before merge <!-- GI-008 -->
- `pytest --cov --cov-fail-under=60` MUST pass; coverage ≥80% is the warning target, and the baseline MUST NOT decrease <!-- GI-004 -->
- `eslint .` (includes `eslint-plugin-jsx-a11y`) and `vitest run` MUST pass before merge <!-- GI-004, GI-010 -->
- `gitleaks detect --no-banner` MUST find no secrets; `pip-audit` and `npm audit --audit-level=high` MUST report no high/critical vulnerabilities <!-- GI-003 -->
- `lint-imports` MUST pass — it blocks the load-bearing seam (domain importing the Stripe SDK or DB drivers) and warns on other layer-boundary violations <!-- GI-007 -->

### Governance operations

- Ledger (waivers · amendment policy · exceptions · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup` (fact-profile changes — module attach/detach — and un-waives are governance events)
<!-- mochiko:output-style:begin -->
- Writing style — conversation: `full` · reports: `ultra` *(internal agent hand-offs)* · documents: `full`. Terse and plain-English by default. Set any of the three to `off`, `lite`, `full`, or `ultra` and your choice is kept when this section is regenerated; "normal mode" turns it off for one session.
<!-- mochiko:output-style:end -->
- Path-scoped rules inject on **Read**, not Write (observed behavior) — before creating a new file under `app/`, `frontend/`, or `.mochiko/specs/`, read the matching `.claude/rules/mochiko/` file or read back the file you created <!-- standing line: emit whenever the set includes any rules file -->
- Operating docs (knowledge-management module): sessions in `.mochiko/brainstorms/` + `index.md`; rulings land in `DECISIONS.md`; open threads in `BACKLOG.md`; direction in `ROADMAP.md`; landing ritual + invariants at `.mochiko/memory/knowledge-management.md`; groom: `mochiko:grooming-operating-docs` <!-- GI-015 -->
- Release gates: dev → prod on Render, ship-when-ready; backup-before-migration + reversible migrations + scheduled backup with tested restore; rollback via Render redeploy-previous — detail in the ledger <!-- GI-016, GI-028 -->
<!-- mochiko:governance:end -->
