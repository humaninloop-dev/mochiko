# Ledgerline

Invoicing and payment-tracking SaaS for US small independent contractors. Python/FastAPI +
PostgreSQL + React, deployed on Render, payments via Stripe hosted checkout.

<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v1.0.0 · 2026-08-10 · production floor · modules: a11y (legal-mandate) · knowledge-management (core + CHANGELOG + RUNBOOK) · release-gates · layer-rules <!-- GI-001 (fact profile) -->

### Principles

<!-- Universal principles: the operative line IS the governance. Scope-bound and
     procedure-shaped principles: the line is an index entry pointing at the home. -->
- Secrets MUST stay out of the repo (env vars + `.gitignore`, `gitleaks` in CI); inputs MUST be validated at FastAPI boundaries; every API boundary MUST enforce authentication AND object-level authorization scoping each read/write to the owning account — no contractor reaches another account's data (NON-NEGOTIABLE) <!-- GI-003 -->
- New code MUST ship with tests; coverage MUST NOT fall below the ratchet baseline; the invoicing critical-path smoke test MUST pass before release (NON-NEGOTIABLE) <!-- GI-004 -->
- Failures MUST NOT silently corrupt invoice/payment data; API errors MUST use one consistent surface (RFC 7807 problem+json) with correlation IDs; stack traces MUST NOT leak to clients (NON-NEGOTIABLE) <!-- GI-005 -->
- Logs MUST be structured with correlation IDs and MUST NOT carry PII or financial data — the rule extends to third-party processors: Sentry MUST scrub before egress; a health-check endpoint MUST exist (NON-NEGOTIABLE) <!-- GI-006 -->
- Payment/invoice state MUST be correct and MUST NOT report a false "paid"; the payment-status state machine MUST be tested and green before any release <!-- GI-011 -->
- Contractor financial data MUST be backed up with a verified, periodically exercised restore path <!-- GI-012 -->
- Governance enforcement MUST be automated (CI/hooks/tooling), never dependent on a second reviewer; gates MUST stay right-sized so a solo founder can ship several times a week <!-- GI-013 -->
- Functions SHOULD stay within cyclomatic complexity 10 (tunable per legitimately complex code); no catch-all "utils" dumping-ground modules <!-- GI-008 -->
- New dependencies MUST be justified and version-pinned; vulnerability scans MUST block merge on high/critical findings <!-- GI-009 -->
- Customer-facing UI MUST meet WCAG 2.1 AA; automated axe checks in CI are the day-one gate, the residual manual-audit gap is backlog-tracked, not waived (NON-NEGOTIABLE — legal mandate) <!-- GI-010 -->
- Layered architecture (hexagonal, two-seam) — see `.claude/rules/mochiko/layers.md` <!-- GI-007, GI-016 -->
- Domain-layer dependencies (registry + add-policy) — see `.claude/rules/mochiko/layers.md` <!-- GI-017 -->

### Technology stack

- Python 3.12 · FastAPI · PostgreSQL · React (frontend, part-time contributor) · deployed on Render · Stripe hosted checkout · Sentry error tracking <!-- GI-002, GI-001 -->

### Quality gates

- `ruff check .` · `ruff format --check .` MUST pass before merge <!-- GI-003, GI-008 -->
- `mypy .` MUST pass (strictness starts lenient, ratcheted up over time) <!-- GI-008 -->
- `pytest --cov --cov-fail-under=60` MUST pass; ≥80% is the warning line; ratchet MUST NOT decrease. Floor-enforcing tests included: cross-account authz-denial, error-response no-stack-trace, no-PII-in-logs <!-- GI-004, GI-003, GI-005, GI-006 -->
- `eslint .` · `prettier --check .` · `vitest run` MUST pass (frontend) <!-- GI-004, GI-008 -->
- `gitleaks detect` MUST pass (secret scan) <!-- GI-003 -->
- `pip-audit` · `npm audit --audit-level=high` MUST pass (dependency scan) <!-- GI-009 -->
- axe-based accessibility checks in CI against WCAG 2.1 AA MUST pass <!-- GI-010 -->
- `import-linter` MUST pass, scoped to the Stripe and DB port seams only <!-- GI-007 -->
- CI: GitHub Actions, all gates blocking merge to `main` <!-- GI-013 -->

### Governance operations

- Ledger (waivers · amendment policy · exceptions · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup` (fact-profile changes — module attach/detach — and un-waives are governance events); standing revisit triggers: a customer contract requiring SOC 2 (attestation), going international or billing EU/UK clients (gdpr), the pre-launch data-retention/deletion decision
<!-- mochiko:output-style:begin -->
- Writing style — conversation: `full` · reports: `ultra` *(internal agent hand-offs)* · documents: `full`. Terse and plain-English by default. Set any of the three to `off`, `lite`, `full`, or `ultra` and your choice is kept when this section is regenerated; "normal mode" turns it off for one session.
<!-- mochiko:output-style:end -->
- Path-scoped rules inject on **Read**, not Write (observed behavior, kinako dogfood 2026-07-19) — before creating a new file under `backend/` (layers.md), `.mochiko/specs/` (output-style.md), or an operating doc (operating-docs.md), read the matching `.claude/rules/mochiko/` file or read back the file you created <!-- standing line: emit whenever the set includes any rules file -->
- Operating docs (knowledge-management module): sessions in `.mochiko/brainstorms/` + `index.md`; rulings land in `DECISIONS.md`; open threads in `BACKLOG.md`; direction in `ROADMAP.md`; landing ritual + invariants at `.mochiko/memory/knowledge-management.md`; groom: `mochiko:grooming-operating-docs` <!-- GI-014 -->
- Release gates: staging→production on Render, expand-contract migrations, rollback < 15 min, destructive-migration flag + explicit approval — detail in the ledger <!-- GI-015 -->
<!-- mochiko:governance:end -->
