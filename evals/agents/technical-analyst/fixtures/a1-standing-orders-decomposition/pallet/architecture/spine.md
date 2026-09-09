# Architecture spine — Pallet (excerpt: the rows the standing-orders feature touches)

**Scope**: backend-service (the retailer app and the supplier app are separate repos)

## Containers (status)

- `api` — NestJS 10 on two Fly.io machines (`lhr`) — built
- `worker` — BullMQ workers on one Fly.io machine (`lhr`): invoice generation, e-mail, PDF
  rendering, accounting export, nightly retention purge — built
- `db` — PostgreSQL 16 (Fly Postgres HA pair, `lhr`) — built
- `redis` — Redis 7 (Upstash, `lhr`), BullMQ queues only — built

## Communication styles

- `api → db`: Prisma, one transaction per request
- `api → worker`: BullMQ jobs over `redis` for anything that outlives a request (e-mail, PDF
  rendering, accounting export)
- `api → Stripe`: synchronous HTTPS through the `payments` module (`src/payments/`); Stripe
  SDK pinned to API version `2024-06-20`; 8 s timeout; one retry on a network error reusing
  the same idempotency key
- `Stripe → api`: webhooks at `POST /webhooks/stripe`, signature-verified, handed to the
  worker
- `worker → Postmark`: transactional e-mail through `src/mail/`

## Concern rows touching this feature

- **AX-003 Payments** — decided · built · Ruling: Stripe (EU entity) for cards and Bacs
  Direct Debit; a retailer's saved method is a Stripe PaymentMethod referenced by id; Pallet
  never holds card or bank numbers (ruled 2026-04-02) · As-built: a card charge confirms
  synchronously inside the request; a Bacs debit is *accepted* by Stripe at submission and
  can still fail up to three working days later, reported by the
  `payment_intent.payment_failed` webhook — today that only happens for one-off orders,
  where the retailer is on the page · Drift: none
- **AX-006 Scheduled work** — decided · built · Ruling: BullMQ repeatable jobs for periodic
  work and delayed jobs for one-off timed work; no cron on the machines (ruled 2026-04-02) ·
  As-built: nightly purge, hourly accounting export · Drift: none
- **AX-008 Audit trail** — decided · built · Ruling: every mutation of a financial record
  writes an `audit_events` row (actor, action, before, after) in the same transaction (ruled
  2026-05-14) · As-built: orders, invoices, payments · Drift: none
- **AX-012 Observability** — decided · built · Ruling: `pino` JSON logs with correlation id,
  OpenTelemetry to Grafana Cloud, alerts on 5xx rate above 1% over 5 min and BullMQ queue lag
  above 10 min (ruled 2026-05-14) · Drift: none
