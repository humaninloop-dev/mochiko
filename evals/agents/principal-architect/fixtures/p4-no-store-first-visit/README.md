# tapestry

Membership management for independent gyms and studios: members, plans, Stripe billing,
class bookings, and a staff web app for the front desk and the owner.

## Architecture

A single Express monolith (`apps/api`) with a PostgreSQL database; the staff web app is a
React SPA. Stripe webhooks are handled inside the monolith. There are no background jobs —
everything happens in the request. Deployed on Fly.io in one region (London).

*(Written 2026-02 at the seed round.)*

## Repo layout

- `apps/api` — Express + TypeScript
- `apps/staff-web` — React (Vite), the staff app
- `apps/worker` — BullMQ worker
- `packages/db` — Prisma schema and client, shared by `api` and `worker`
- `specs/` — feature specs
- `docs/` — partner API notes

Team: two engineers and a contractor (front end).
