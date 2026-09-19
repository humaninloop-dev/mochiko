# Pallet

Wholesale ordering for independent food and drink producers and the shops and cafés that
stock them. A retailer browses a supplier's catalogue and places an order before the
supplier's cut-off; the supplier delivers on its route day; Pallet invoices the retailer and
pays the supplier net of an 8% commission.

- **Stack:** TypeScript 5 · NestJS 10 · Prisma 5 · PostgreSQL 16 · BullMQ on Redis 7 ·
  Fly.io (`lhr`) · GitHub Actions. The retailer app and the supplier app are separate
  front-end repos that consume this API.
- **Scale:** 140 suppliers, 1,900 retailers, UK and Ireland; Dutch producers onboarding from
  Q4 2026.
- **Team:** four engineers, one product owner (Priya), a part-time finance lead (Tom), no
  dedicated ops.
- Operating manual: `CLAUDE.md` · architecture: `architecture/` · schema:
  `prisma/schema.prisma` · feature specs: `specs/`
