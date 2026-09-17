# Ledgerlite

Invoicing for small studios: draft an invoice, send it, record the payment, chase the
rest. Local development runs the app on :3000 against a local Postgres.

<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v1.1.0 · 2026-07-08 · production floor · depth: high · modules: knowledge-management (core) <!-- GI-001 (fact profile) · GI-002 (depth level) -->

### Principles

- Client and invoice data MUST NOT leave the production database except through the audited export path (NON-NEGOTIABLE) <!-- GI-003 -->
- Every write to an invoice, payment, or client record MUST record who made it and when <!-- GI-004 -->
- Money handling — see `.claude/rules/mochiko/money-handling.md` <!-- GI-005 -->
- Pages a client reaches without an account — see `.claude/rules/mochiko/public-pages.md` <!-- GI-006 -->

### Technology stack

- Node 20 · Fastify · Postgres 15 · Prisma migrations · Postmark for transactional email <!-- GI-007 -->

### Quality gates

- `npm test` MUST pass before merge; coverage ≥ 70% on new code (`npm run coverage`) <!-- GI-008 -->

### Governance operations

- Ledger (waivers · amendment policy · exceptions · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup` (fact-profile changes — module attach/detach — and un-waives are governance events)
- Operating docs (knowledge-management module): sessions in `.mochiko/brainstorms/` + `index.md`; rulings land in `DECISIONS.md`; open threads in `BACKLOG.md`; direction in `ROADMAP.md`; landing ritual + invariants at `.mochiko/memory/knowledge-management.md`; groom: `mochiko:grooming-operating-docs` <!-- GI-009 -->
<!-- mochiko:output-style:begin -->
- Writing style — conversation: `full` · reports: `ultra` *(internal agent hand-offs)* · documents: `full`. Terse and plain-English by default. Set any of the three to `off`, `lite`, `full`, or `ultra` and your choice is kept when this section is regenerated; "normal mode" turns it off for one session.
<!-- mochiko:output-style:end -->
<!-- mochiko:governance:end -->

## Notes for contributors

- Run the migrations before the tests; the suite assumes a fresh schema.
- The sample studio in fixtures and screenshots is "Halfmoon Studio". Keep it that way so
  the docs stay consistent.
- Postmark is stubbed in development; set `POSTMARK_TOKEN` only when you mean it.
