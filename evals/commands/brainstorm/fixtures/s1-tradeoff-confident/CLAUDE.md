# Saltmarsh

Bookings, waitlists and payments for small sailing schools. See `README.md` for the
product shape and `docs/` for the figures behind current decisions.

<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v1.1.0 · 2026-06-18 · production floor · depth: high · modules: knowledge-management (core) <!-- GI-001 (fact profile) · GI-002 (depth level) -->

### Principles

- Student payment data MUST NOT be stored outside Stripe; Saltmarsh keeps only Stripe ids and amounts (NON-NEGOTIABLE) <!-- GI-003 -->
- Every change to a booking, a seat hold, or a waitlist position MUST record who made it and when <!-- GI-004 -->
- A school's own settings win over platform defaults; a platform default MUST NOT silently override a school's choice <!-- GI-005 -->

### Technology stack

- Node 22 · Fastify · Postgres 16 · Prisma migrations · Stripe · Resend · Fly.io <!-- GI-006 -->

### Quality gates

- `npm test` MUST pass before merge; coverage ≥ 70% on new code (`npm run coverage`) <!-- GI-007 -->

### Governance operations

- Ledger (waivers · amendment policy · exceptions · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup` (fact-profile changes — module attach/detach — and un-waives are governance events)
<!-- mochiko:output-style:begin -->
- Writing style — conversation: `full` · reports: `ultra` *(internal agent hand-offs)* · documents: `full`. Terse and plain-English by default. Set any of the three to `off`, `lite`, `full`, or `ultra` and your choice is kept when this section is regenerated; "normal mode" turns it off for one session.
<!-- mochiko:output-style:end -->
- Operating docs (knowledge-management module): sessions in `.mochiko/brainstorms/` + `index.md`; rulings land in `DECISIONS.md`; open threads in `BACKLOG.md`; direction in `ROADMAP.md`; landing ritual + invariants at `.mochiko/memory/knowledge-management.md`; groom: `mochiko:grooming-operating-docs` <!-- GI-008 -->
<!-- mochiko:governance:end -->

## Notes for contributors

- Run the Prisma migrations before the tests; the suite assumes a fresh schema.
- The sample school in fixtures and screenshots is "Blackwater Sailing". Keep it that way.
- Money is integer pence everywhere. Never a float.
