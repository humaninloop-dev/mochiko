# Ledgerline

SaaS invoicing and payment-tracking for US solo contractors. FastAPI + PostgreSQL backend,
React/TypeScript frontend, Stripe-hosted checkout for payment collection.

<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v1.0.0 · 2026-08-10 · production floor · modules: none (compliance) — accessibility adopted as a standard (GI-025); PCI / SOC 2 / US-state-privacy watches in the ledger <!-- GI-001 (fact profile) -->

### Principles

- Security by default: secrets out of the repo (env vars + `.gitignore`), input validated at every boundary, auth enforced at every API boundary (NON-NEGOTIABLE) <!-- GI-003 -->
- Testing discipline: the payment state machine, auth, and tenant-isolation paths MUST have tests; coverage MUST NOT decrease (ratchet); a critical-path smoke test exists from day one (NON-NEGOTIABLE) <!-- GI-004 -->
- Error handling: failures MUST NOT silently corrupt data (especially payment state); API errors use RFC 7807, carry correlation IDs, and never leak stack traces or customer data (NON-NEGOTIABLE) <!-- GI-005 -->
- Observability: structured JSON logs with correlation IDs and a `/health` endpoint; error tracking via Sentry (NON-NEGOTIABLE) <!-- GI-006 -->
- Card data MUST never touch Ledgerline servers — Stripe-hosted checkout only, to preserve PCI SAQ-A eligibility (NON-NEGOTIABLE) <!-- GI-026 -->
- Customer PII (names, emails, addresses) and invoice amounts MUST NOT appear in log lines or in error payloads sent to telemetry (NON-NEGOTIABLE) <!-- GI-013 -->
- Data durability: automated daily database backups with point-in-time recovery, and a periodic restore check MUST verify backups restore <!-- GI-024 -->
- Payment-state integrity — see `.claude/rules/mochiko/payments.md` <!-- GI-010 -->
- Currency represented as integer cents, never floating point — see `.claude/rules/mochiko/payments.md` <!-- GI-011 -->
- Tenant isolation on all data access — see `.claude/rules/mochiko/data-access.md` <!-- GI-012 -->
- Layered (hexagonal) architecture, dependencies flow inward — see `.claude/rules/mochiko/architecture-layers.md` <!-- GI-007 -->
- Single responsibility and complexity limits — see `.claude/rules/mochiko/architecture-layers.md` <!-- GI-008 -->
- Dependency discipline: justified, pinned, vuln-scanned in CI (quality gate below; metadata in the ledger) <!-- GI-009 -->
- Accessibility: WCAG 2.1 AA on core contractor flows — see `.claude/rules/mochiko/accessibility.md` <!-- GI-025 -->
- Approved domain-dependency registry — see `.claude/rules/mochiko/domain-dependencies.md` <!-- GI-018 -->

### Technology stack

- Python 3.12 / FastAPI · PostgreSQL · React + TypeScript · Stripe (hosted checkout) · deployed on Render · CI on GitHub Actions <!-- GI-002 -->

### Quality gates

- `pytest` MUST pass before merge <!-- GI-004 -->
- `ruff check .` and `ruff format --check .` MUST pass before merge <!-- GI-008 -->
- `mypy .` MUST pass before merge <!-- GI-002 -->
- `npm run lint` and `npm test` MUST pass before merge (frontend) <!-- GI-004 -->
- `lint-imports` MUST pass — no inner→outer or unapproved-domain imports <!-- GI-007 -->
- `pip-audit` and `npm audit` MUST block merge on high/critical vulnerabilities; lock files committed <!-- GI-009 -->
- Secret scanning (`gitleaks detect`) MUST block merge on findings <!-- GI-003 -->
- A release is blocked unless tests, lint, and type checks all pass (release-gates) <!-- GI-017 -->
- No numeric coverage-percentage gate (waived — see ledger GI-014); the critical-path test rule above replaces it <!-- GI-014 -->

### Governance operations

- Ledger (waivers · amendment policy · exceptions · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup` (fact-profile changes — module attach/detach — and un-waives are governance events); standing watches: a signed SOC 2/attestation obligation, any feature that would collect card data, or crossing US-state-privacy thresholds each re-open the fact profile
<!-- mochiko:output-style:begin -->
- Writing style — conversation: `full` · reports: `ultra` *(internal agent hand-offs)* · documents: `full`. Terse and plain-English by default. Set any of the three to `off`, `lite`, `full`, or `ultra` and your choice is kept when this section is regenerated; "normal mode" turns it off for one session.
<!-- mochiko:output-style:end -->
- Path-scoped rules inject on **Read**, not Write (observed behavior) — before creating a new file under `src/`, `frontend/`, `.mochiko/specs/`, or an operating doc (`ROADMAP.md` · `BACKLOG.md` · `DECISIONS.md` · `CHANGELOG.md` · `ARCHITECTURE.md` · `GLOSSARY.md` · `FEATURES.md`), read the matching `.claude/rules/mochiko/` file or read back the file you created
- Operating docs (knowledge-management module): sessions in `.mochiko/brainstorms/` + `index.md`; rulings land in `DECISIONS.md`; open threads in `BACKLOG.md`; direction in `ROADMAP.md`; release notes in `CHANGELOG.md`; landing ritual + invariants at `.mochiko/memory/knowledge-management.md`; groom: `mochiko:grooming-operating-docs` <!-- GI-016 -->
- Release gates: tests + lint + type checks green blocks every release; detail in the ledger <!-- GI-017 -->
<!-- mochiko:governance:end -->
