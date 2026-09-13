# Halyard — Operating Manual

Marina berth booking. Stack and team in `README.md`; product baselines under
`.mochiko/product/` (data model, contracts, constraints and decisions, architecture store).

## Governance

Ratified 2026-05-19 · production floor · depth: high

### Principles

- **One datastore.** PostgreSQL is the system of record and the only datastore. Jobs, queues,
  and locks run on it (pg-boss). No second datastore ships without a platform sign-off
  recorded on the architecture store's `AX-002` row before any code. Rationale: five
  engineers, one on-call rota, no DBA.
- **Money moves only on someone's authority.** No charge or refund is created without a named
  person's action or a recorded standing instruction. Enforcement: `authorisedBy` is non-null
  on every payment row; the API rejects a null. Rationale: a transfer nobody can account for is
  one the finance lead cannot defend.
- **Counterparty data is confidential.** Owner contact details, vessel registration numbers,
  and payment references are Confidential or Restricted and never appear in logs, exports, or
  error bodies. Data stays in the UK/EU (Fly.io `lhr`; Stripe's EU entity; Postmark EU
  region). Enforcement: the shared logger's redaction list; the contract suite asserts no
  e-mail, phone number, or payment reference in any error body. Rationale: UK GDPR and EU GDPR.
- **Every background job is idempotent.** A job carries a deterministic id and checks for
  prior completion before acting. Enforcement: the pg-boss wrapper refuses a job without a
  `singletonKey`. Rationale: a worker restart re-runs the batch, and a duplicate charge is a
  real card statement.
- **Errors.** Error responses use RFC 7807 Problem Details (`application/problem+json`) with
  `type`, `title`, `status`, `detail`, and `correlationId`. Enforcement: the shared exception
  filter is the only place an error body is built.
- **Testing.** New behaviour ships with tests; coverage MUST NOT fall below 70% (blocking in
  CI). Integration tests run against a real PostgreSQL, never an in-memory substitute.
- **Observability.** Structured JSON logs (`pino`) with a correlation id on every line;
  OpenTelemetry traces to Grafana Cloud; `/health` on every app; alerts on 5xx rate and job
  lag (`AX-011`).
