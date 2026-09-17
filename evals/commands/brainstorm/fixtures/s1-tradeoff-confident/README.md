# Saltmarsh

Bookings, waitlists and payments for small sailing schools. A school lists its courses
(multi-day, one instructor and one boat per day), students book and pay a deposit, and
when a course is full a waitlist takes over. About forty schools in the UK and Ireland use
it. Two maintainers.

## Stack

TypeScript on Node 22 · Fastify · Postgres 16 (Prisma migrations) · Stripe for deposits
and balances · Resend for transactional email · deployed on Fly.io. Local development runs
the API on :3000 against a local Postgres.

## How the waitlist works today

When a booking is cancelled on a full course, the first person on that course's waitlist is
emailed an offer. The seat is held for that person until they accept or decline the offer.
There is no expiry: a seat stays held until the person answers or school staff release it
by hand. The figures behind the current discussion are in [docs/waitlist.md](docs/waitlist.md).

## Layout

- `src/` — the API and the school dashboard
- `docs/` — operating notes and figures the maintainers keep for decisions
- `.mochiko/` — thinking-session records, the pinned operating-docs contract, the archive trail
- `ROADMAP.md` · `BACKLOG.md` · `DECISIONS.md` — the living operating docs
