# Pallet — Operating Manual

Wholesale ordering platform. Stack and team in `README.md`; architecture in `architecture/`;
schema in `prisma/schema.prisma`.

## Governance

Ratified 2026-06-30 · production floor · depth: high

### Principles

- **Financial records are immutable.** Orders, invoices, payments, and payouts cannot be
  edited once issued; a correction is a new document (credit note, reversal), never an edit.
  Every financial record is retained for seven years from the end of the tax year it falls in.
  Enforcement: database triggers reject `UPDATE` on issued rows of the financial tables; the
  nightly retention purge skips them. Rationale: HMRC and Irish Revenue record-keeping rules,
  and a supplier dispute two years on has to reconstruct exactly what was charged.
- **Money moves only on someone's authority.** No payment or payout is created without either
  a named person's action or a recorded standing instruction that names who set it up and
  when. Enforcement: `authorised_by` is non-null on every payment and payout row (a user id or
  a standing-instruction id); the API rejects a null. Rationale: one transfer nobody can
  account for is one the finance lead cannot defend.
- **Counterparty data is confidential.** Collect the minimum. Contact details, bank details,
  and trading relationships of a retailer or supplier are Confidential or Restricted and never
  appear in logs, exports, or error bodies. Data stays in the UK/EU (Fly.io `lhr`; Stripe's EU
  entity). Enforcement: the shared logger's redaction list; the export allowlist; the
  contract suite asserts no e-mail or account number in any error body. Rationale: UK GDPR
  and EU GDPR; the platform holds every counterparty's commercial relationships.
- **Every background job is idempotent.** A job carries a deterministic id and checks for
  prior completion before acting, so a re-run after a worker restart changes nothing.
  Enforcement: BullMQ `jobId` is mandatory (lint rule `PLT002`); the job base class refuses a
  job without one. Rationale: a worker restart mid-batch re-runs the batch, and a duplicated
  order is a real van at a real café.
- **One datastore.** PostgreSQL is the system of record. No new datastore ships without a
  platform sign-off recorded in `architecture/spine.md`. Rationale: four engineers cannot run
  three databases.
- **Errors.** Error responses use RFC 7807 Problem Details (`application/problem+json`) with
  `type`, `title`, `status`, `detail`, and `correlation_id`. Enforcement: the shared exception
  filter is the only place an error body is built; the contract suite asserts the schema.
  Rationale: one error component in the apps; support matches a ticket to a log line.
- **Testing.** New behaviour ships with tests; coverage MUST NOT fall below 75% (blocking in
  CI). Integration tests run against a real PostgreSQL, never an in-memory substitute.
- **Observability.** Structured JSON logs (`pino`) with a correlation id on every line;
  OpenTelemetry traces to Grafana Cloud; `/health` on every app; alerts on 5xx rate and queue
  lag (`architecture/spine.md`, AX-012).
