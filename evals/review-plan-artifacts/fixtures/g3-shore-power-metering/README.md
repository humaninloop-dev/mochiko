# Halyard

Berth booking for marinas and the boat owners who moor with them. An owner finds a berth that
fits the vessel, books it for a season or a stay, and pays by card or direct debit; the marina
manages occupancy, tariffs, and shore-power billing from one console.

- **Stack:** TypeScript 5 · NestJS 10 · Prisma 5 · PostgreSQL 16 · pg-boss 10 (jobs on
  PostgreSQL) · Fly.io (`lhr`) · GitHub Actions. The owner portal and the marina console are
  separate front-end repos that consume this API.
- **Scale:** 62 marinas, 9,400 owners, UK and Ireland; Brittany marinas onboarding Q1 2027.
- **Team:** five engineers, one product owner (Ines), a part-time finance lead, no dedicated
  ops. Marcus is the founding engineer.
- Operating manual: `CLAUDE.md` · product baselines: `.mochiko/product/` · feature specs:
  `.mochiko/specs/` · run outputs: `.mochiko/features/`
