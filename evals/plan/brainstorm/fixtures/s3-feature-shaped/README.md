# Saltmarsh

Bookings, waitlists and payments for small sailing schools. A school lists its courses
(multi-day, one instructor and one boat per day), students book and pay a deposit, and
when a course is full a waitlist takes over with a 24-hour offer window. About forty-five
schools in the UK and Ireland use it. Two maintainers.

## Stack

TypeScript on Node 22 · Fastify · Postgres 16 (Prisma migrations) · Stripe for deposits
and balances · Resend for transactional email · deployed on Fly.io. Local development runs
the API on :3000 against a local Postgres.

## Pricing

Schools set their own course prices; Saltmarsh takes a platform fee per booking. Figures
and margins the maintainers use for decisions are in [docs/pricing.md](docs/pricing.md).
There are no platform-wide discounts today.

## Layout

- `src/` — the API and the school dashboard
- `docs/` — operating notes and figures the maintainers keep for decisions
- `FEATURES.md` + `.mochiko/features/` — the capability map and its entries
- `.mochiko/specs/` — feature specifications, indexed at `.mochiko/specs/index.md`
- `.mochiko/` — thinking-session records, the pinned operating-docs contract, the archive trail
- `ROADMAP.md` · `BACKLOG.md` · `DECISIONS.md` — the living operating docs
