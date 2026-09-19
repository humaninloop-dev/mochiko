# Peartree

Invoicing for freelancers: draft an invoice, send it, chase it, mark it paid. Two
maintainers. Local development runs the app on :4000 against a local Postgres.

<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v1.2.0 · 2026-06-02 · production floor · depth: high · modules: none <!-- GI-001 (fact profile) · GI-002 (depth level) -->

### Principles

- Customer invoice data MUST NOT leave the production database except through an audited export (NON-NEGOTIABLE) <!-- GI-003 -->
- Every write path MUST record who made the change and when <!-- GI-004 -->
- Money handling — see `.claude/rules/mochiko/money-handling.md` <!-- GI-005 -->

### Technology stack

- Node 20 · Fastify · Postgres 15 · Prisma migrations <!-- GI-006 -->

### Quality gates

- `npm test` MUST pass before merge <!-- GI-007 -->
- Coverage ≥ 70% on new code (`npm run coverage`) <!-- GI-007 -->

### Governance operations

- Ledger (waivers · amendment policy · exceptions · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup` (fact-profile changes — module attach/detach — and un-waives are governance events)
<!-- mochiko:output-style:begin -->
- Writing style — conversation: `lite` · reports: `ultra` *(internal agent hand-offs)* · documents: `full`. Terse and plain-English by default. Set any of the three to `off`, `lite`, `full`, or `ultra` and your choice is kept when this section is regenerated; "normal mode" turns it off for one session.
<!-- mochiko:output-style:end -->
<!-- mochiko:governance:end -->

## Notes for contributors

- Run the migrations before the tests; the suite assumes a fresh schema.
- The sample tenant in fixtures and screenshots is "Wren Studio". Keep it that way so the
  docs stay consistent.
