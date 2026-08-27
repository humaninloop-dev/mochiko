# Governance Intent — Peartree

**Session date:** 2026-05-14 · **Mode:** greenfield
**Confirmed at synthesis checkpoint:** 2026-05-14 by the maintainers
**Governs:** the governance surface set v1.2.0 (CLAUDE.md governance region · `.claude/rules/mochiko/` · `.mochiko/memory/governance-ledger.md`)

## Fact profile

- **GI-001 — Facts:** industry: professional-services billing · data classes: customer
  contact details, invoice line items, bank account identifiers · jurisdictions/markets:
  EU and UK · contractual commitments: none beyond the standard terms of service ·
  **Mark:** Confident
- **Modules triggered (mechanical):** none — negatives confirmed: no health data, so no
  clinical obligations attach; card details are never handled directly, since payment runs
  through the processor's hosted page, so no cardholder-data obligations attach.

## Project identity & type

- **GI-002 — Type:** backend → shelves dealt: backend-service · **Mark:** Confident
- **Identity:** Invoicing for freelancers — draft, send, chase, reconcile. Two
  maintainers, expected to run for years. Depth level: high, declared 2026-05-14, one-way.

## Principles

- **GI-003 — Customer data containment.** Invoice data leaves the production database only
  through an audited export. Home: the CLAUDE.md governance region. **Mark:** Confident
- **GI-004 — Change attribution.** Every write path records the actor and the timestamp.
  Home: the CLAUDE.md governance region. **Mark:** Confident
- **GI-005 — Money handling.** Integer minor units, explicit currency, rounding once at
  presentation. Home: `.claude/rules/mochiko/money-handling.md`. **Mark:** Confident
- **GI-006 — Technology stack.** Node 20, Fastify, Postgres 15, Prisma migrations.
  **Mark:** Assumed
- **GI-007 — Quality gates.** The test suite green and coverage at or above 70% on new
  code before merge. **Mark:** Confident

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| 1.0.0 | 2026-05-14 | ratified | GI-001 … GI-007 |
| 1.2.0 | 2026-06-02 | coverage floor raised from 60% to 70% | GI-007 |
