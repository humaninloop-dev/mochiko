# Saltmarsh

Bookings, waitlists and payments for small sailing schools. A school lists its courses
(multi-day, one instructor and one boat per day), students book and pay a deposit, and
when a course is full a waitlist takes over. About thirty schools in the UK and Ireland
use it. Two maintainers.

## Stack

TypeScript on Node 22 · Fastify · Postgres 16 (Prisma migrations) · Stripe for deposits
and balances · Resend for transactional email · deployed on Fly.io. Local development runs
the API on :3000 against a local Postgres.

## Layout

- `src/` — the API and the school dashboard; `src/schedule/` is the instructor week view
  and the day-handover flow
- `src/db/schema.sql` — the current schema as a plain dump, kept beside the Prisma
  migrations for reading
- `docs/` — operating notes; `docs/support-notes.md` is where support tickets worth
  remembering get pasted
- `.mochiko/` — thinking-session records

## Known rough edges

- The instructor week view was built before day handovers existed (handovers landed
  2026-04-28). Nobody has looked at the two together since.
- There is no scheduled-job runner; everything happens on a request.
