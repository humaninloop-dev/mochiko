# Tessellate — Team Notes

Tessellate is the marketplace for independent ceramicists. Stack and commands in `README.md`;
the checkout and payments boundary is drawn in `docs/payments-boundary.md` — read it before
touching `src/checkout/` or `src/payments/`.

## Things we keep saying in review

- Prices are integer cents with a currency code; VAT is computed once, in `src/tax/`.
- A maker's payout is never computed from the order total; it is computed from the settled amount.

<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v1.0.0 · 2026-09-09 · production floor · depth: high · modules: gdpr · a11y <!-- GI-001 (fact profile) · GI-003 (depth level) -->

### Principles

- Secrets MUST stay out of the repository: loaded from AWS Secrets Manager, never committed; `npm run scan:secrets` blocks merge on any finding and `npm audit --audit-level=high` blocks merge on any high or critical advisory (NON-NEGOTIABLE) <!-- GI-004 -->
- `npm test` and `npm run e2e` MUST pass before merge; coverage MUST be ≥ 80% (warning) and ≥ 70% (blocking) on new code, and the baseline MUST NOT decrease (NON-NEGOTIABLE) <!-- GI-005 -->
- A failure MUST NOT silently corrupt an order, a payment, or a payout; every error response is RFC 7807 Problem Details with a `correlation_id`, and no stack trace reaches a response body (NON-NEGOTIABLE) <!-- GI-006 -->
- Logs MUST be structured JSON with a `correlation_id` on every line, MUST NOT contain personal data or any part of a card number, and `/healthz` MUST answer on every service (NON-NEGOTIABLE) <!-- GI-007 -->
- Personal data of customers and makers MUST have a recorded lawful basis and a retention class; subject access, portability, and erasure requests MUST be fulfilled within 30 days and erasure MUST propagate to every replica and backup within 90 days; a personal-data breach MUST be reported to the lead supervisory authority within 72 hours of discovery <!-- GI-008 -->
- Cardholder data — see `.claude/rules/mochiko/cardholder-data.md` <!-- GI-009 -->
- Every customer-facing page MUST meet WCAG 2.2 AA; `npm run a11y` blocks merge on any serious or critical axe finding <!-- GI-XXX -->
- Security at boundaries — see `.claude/rules/mochiko/security.md` <!-- GI-004 -->
- Pages SHOULD load quickly <!-- GI-011 -->
- All internal documentation MUST be written in British English <!-- GI-016 -->

### Technology stack

- TypeScript 5.5 · Node 22 · Remix 2 · Prisma 5 · PostgreSQL 16 (RDS) · Redis 7 (ElastiCache) <!-- GI-002 -->
- Adyen (API-only card integration, tokeniser in `src/payments/`) · Sendcloud for labels · AWS ECS on Fargate <!-- GI-002 -->

### Quality gates

- `npm run lint` MUST pass before merge <!-- GI-002 -->
- `npm test` and `npm run e2e` MUST pass before merge; `npm run test:cov` enforces ≥ 70% blocking on new code and the baseline MUST NOT decrease <!-- GI-005 -->
- `npm run a11y` MUST pass before merge <!-- GI-010 -->
- `npm run scan:secrets` and `npm audit --audit-level=high` MUST pass before merge <!-- GI-004 -->

### Governance operations

- Ledger (waivers · amendment policy · exceptions · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup` (fact-profile changes — module attach/detach — and un-waives are governance events)
<!-- mochiko:output-style:begin -->
- Writing style — conversation: `full` · reports: `ultra` *(internal agent hand-offs)* · documents: `full`. Terse and plain-English by default. Set any of the three to `off`, `lite`, `full`, or `ultra` and your choice is kept when this section is regenerated; "normal mode" turns it off for one session.
<!-- mochiko:output-style:end -->
- Path-scoped rules inject on **Read**, not Write (observed behavior, kinako dogfood 2026-07-19) — before creating a new file under `src/`, `app/`, or `tests/`, read the matching `.claude/rules/mochiko/` file or read back the file you created
- Operating docs (knowledge-management module): sessions in `.mochiko/brainstorms/` + `index.md`; rulings land in `DECISIONS.md`; open threads in `BACKLOG.md`; direction in `ROADMAP.md`; releases in `CHANGELOG.md`; landing ritual + invariants at `.mochiko/memory/knowledge-management.md`; groom: `mochiko:grooming-operating-docs` <!-- GI-014 -->
- Release gates: `staging` soak, then a manual promotion to `prod`; rollback is an ECS task-definition rollback within 15 minutes — detail in the ledger <!-- GI-013 -->
<!-- mochiko:governance:end -->

## On-call

- PagerDuty schedule `tessellate-primary`; the payments runbook is pinned in `#payments`.
