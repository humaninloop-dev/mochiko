# Architecture spine — Pallet (excerpt: the rows the live-stock feature touches)

**Scope**: backend-service (the retailer app and the supplier app are separate repos)

## Containers (status)

- `api` — NestJS 10 on two Fly.io machines (`lhr`); the only app with a public address — built
- `worker` — BullMQ workers on one Fly.io machine (`lhr`): invoice generation, e-mail, PDF
  rendering, accounting export, nightly retention purge — built
- `db` — PostgreSQL 16 (Fly Postgres HA pair, `lhr`) — built
- `redis` — Redis 7 (Upstash, `lhr`), BullMQ queues only — built

## Communication styles

- `api → db`: Prisma, one transaction per request; the catalogue page reads `products` for
  one supplier in a single query today
- `api → worker`: BullMQ jobs over `redis`
- `worker → Xero`, `worker → Postmark`: outbound HTTPS from the worker's static egress IP

## Concern rows touching this feature

- **AX-006 Scheduled work** — decided · built · Ruling: BullMQ repeatable jobs for periodic
  work, delayed jobs for one-off timed work; no cron on the machines (ruled 2026-04-02)
- **AX-008 Audit trail** — decided · built · Ruling: every mutation of a financial record
  writes an `audit_events` row in the same transaction (ruled 2026-05-14); stock is not a
  financial record and is not audited today
- **AX-010 Caching** — not-now · Trigger: the first read-heavy surface — a page that reads
  more than one row per product per request, or catalogue p95 above 400 ms (ruled 2026-05-14)
- **AX-011 Rate limiting** — not-now · Trigger: the first integration, inbound or outbound,
  with a documented rate limit (ruled 2026-05-14)
- **AX-012 Observability** — decided · built · Ruling: `pino` JSON logs with correlation id,
  OpenTelemetry to Grafana Cloud, alerts on 5xx rate above 1% over 5 min and queue lag above
  10 min (ruled 2026-05-14)
- **AX-013 Secrets** — decided · built · Ruling: Fly secrets per app for platform credentials
  (Stripe, Xero, Postmark), rotated quarterly (ruled 2026-04-02) · Note: per-tenant
  credentials have no ruling; nothing holds any today
