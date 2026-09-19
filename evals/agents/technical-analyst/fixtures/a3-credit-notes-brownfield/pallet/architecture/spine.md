# Architecture spine — Pallet (excerpt: the rows the credit-notes feature touches)

**Scope**: backend-service (the retailer app and the supplier app are separate repos)

## Containers (status)

- `api` — NestJS 10 on two Fly.io machines (`lhr`) — built
- `worker` — BullMQ workers on one Fly.io machine (`lhr`): invoice generation, e-mail, PDF
  rendering (`documents`), accounting export (`accounting`), nightly retention purge — built
- `db` — PostgreSQL 16 (Fly Postgres HA pair, `lhr`) — built
- `redis` — Redis 7 (Upstash, `lhr`), BullMQ queues only — built
- `objects` — Tigris object storage (`lhr`) for rendered PDFs — built

## Communication styles

- `api → db`: Prisma, one transaction per request
- `api → worker`: BullMQ jobs over `redis` for anything that outlives a request
- `api → Stripe`: synchronous HTTPS through `src/payments/`; SDK pinned to API version
  `2024-06-20`; 8 s timeout; one retry on a network error reusing the idempotency key
- `worker → Xero`: hourly export through `src/accounting/` (see AX-004)
- `worker → Postmark`: transactional e-mail through `src/mail/`

## Concern rows touching this feature

- **AX-003 Payments** — decided · built · Ruling: Stripe (EU entity) for cards and Bacs
  Direct Debit (ruled 2026-04-02) · As-built: refunds go back to the original payment through
  Stripe's Refund API — a card refund lands in 5–10 days and Stripe refuses one more than 180
  days after the charge; a Bacs refund is a bank transfer that can fail if the account has
  closed, reported by the `refund.failed` webhook · Drift: none
- **AX-004 Accounting export** — decided · built · Ruling: Xero is the ledger; every issued
  financial document and every payment is pushed by the hourly `accounting.export` job;
  failures land in `accounting_export_failures` and retry with backoff, five attempts, then
  alert (ruled 2026-05-14) · As-built: invoices, payments, monthly statements; Xero's limits
  are 60 calls/min and 5,000/day per tenant · Drift: none
- **AX-007 Documents** — decided · built · Ruling: one rendering pipeline for every PDF the
  platform issues (`src/documents/`, `documents.render` job, Handlebars templates, Tigris);
  no second PDF path (ruled 2026-06-02) · As-built: invoice and monthly statement · Drift: none
- **AX-008 Audit trail** — decided · built · Ruling: every mutation of a financial record
  writes an `audit_events` row in the same transaction (ruled 2026-05-14) · Drift: none
