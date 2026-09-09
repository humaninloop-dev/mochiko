# Architecture spine — Pallet (excerpt: the rows the supplier-payouts feature touches)

**Scope**: backend-service (the retailer app and the supplier app are separate repos)

## Containers (status)

- `api` — NestJS 10 on two Fly.io machines (`lhr`) — built
- `worker` — BullMQ workers on one Fly.io machine (`lhr`): invoice generation, e-mail, PDF
  rendering, accounting export, nightly retention purge — built
- `db` — PostgreSQL 16 (Fly Postgres HA pair, `lhr`) — built
- `redis` — Redis 7 (Upstash, `lhr`), BullMQ queues only — built

## Communication styles

- `api → db`: Prisma, one transaction per request
- `api → worker`: BullMQ jobs over `redis` for anything that outlives a request
- `api → Stripe`: synchronous HTTPS through `src/payments/`; SDK pinned to API version
  `2024-06-20`; 8 s timeout; one retry on a network error reusing the idempotency key
- `worker → Xero`: hourly export through `src/accounting/`
- `worker → Postmark`: transactional e-mail through `src/mail/`

## Concern rows touching this feature

- **AX-003 Payments (inbound)** — decided · built · Ruling: Stripe (EU entity) collects
  retailer payments into Pallet's own Stripe balance, paid out to Pallet's bank account daily;
  Pallet never holds card or bank numbers (ruled 2026-04-02) · As-built: card charges settle
  at charge; a Bacs debit is accepted at submission and can fail up to three working days
  later · Drift: none. *Note:* Stripe Connect is not enabled; whether Pallet may hold
  retailer funds on suppliers' behalf is a licensing question legal has open (see
  `concerns.md`, AX-003 note of 2026-07-22).
- **AX-008 Audit trail** — decided · built · Ruling: every mutation of a financial record
  writes an `audit_events` row in the same transaction (ruled 2026-05-14) · Drift: none
- **AX-009 Datastore** — decided · built · Ruling: PostgreSQL 16 is the single system of
  record; a second datastore needs a platform sign-off recorded on this row before any code
  (ruled 2026-04-02) · Rationale: four engineers, one on-call rota, no DBA · As-built: one
  database, Redis for queues only · Drift: none
- **AX-012 Observability** — decided · built · Ruling: `pino` JSON logs with correlation
  id, OpenTelemetry to Grafana Cloud, alerts on 5xx rate and queue lag (ruled 2026-05-14) ·
  Drift: none
- **AX-014 Outbound payments (payouts)** — open · no ruling yet; FEAT-016 is expected to
  bring the first proposal to the desk
