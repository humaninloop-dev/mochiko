# Concern Ledger — Halyard

Rows carry the NFR-XXX targets the built code must meet (grammar: `mochiko:authoring-technical-requirements`).

## AX-001 Authentication

- **Stance**: decided · **Status**: built
- **Ruling**: server-side sessions in PostgreSQL, HttpOnly cookie (D-002; ruled 2026-05-19)

## AX-002 Datastore

- **Stance**: decided · **Status**: built
- **Ruling**: PostgreSQL 16 is the single system of record and the only datastore; jobs, queues, and locks run on it. A second datastore needs a platform sign-off recorded on this row before any code (ruled 2026-05-19)
- **Upgrade trigger**: a workload PostgreSQL demonstrably cannot serve at our scale, with the measurement attached
- **Sign-offs recorded**: none

## AX-004 Payments

- **Stance**: decided · **Status**: built
- **Ruling**: Stripe (EU entity) for cards and Bacs Direct Debit; a saved method is a Stripe PaymentMethod referenced by id; Halyard never holds card or bank numbers (ruled 2026-05-19)
- **NFR-002 — Charge confirmation latency** · performance · source: FEAT-012 SC-002 · **Target:** p95 ≤ 3 s from `POST /bookings/{id}/pay` to response · **Measured:** OpenTelemetry span on the pay handler, rolling 24 h, continuous
- **As-built**: card charges confirm synchronously; a Bacs debit is accepted at submission and can fail up to three working days later via `payment_intent.payment_failed`
- **Drift**: none

## AX-007 Notifications

- **Stance**: decided · **Status**: built
- **Ruling**: every owner-facing notice is a worker job sent through Postmark EU; no SMS channel (ruled 2026-05-19)
- **NFR-003 — Notification latency** · performance · source: FEAT-004 FR-006 · **Target:** p95 ≤ 60 s from the triggering event's commit to Postmark `accepted` · **Measured:** job `createdOn` to Postmark response timestamp, rolling 24 h, continuous
- **Drift**: none

## AX-009 Scheduled work

- **Stance**: decided · **Status**: built
- **Ruling**: pg-boss scheduled and delayed jobs for timed work; no cron on the machines (D-001; ruled 2026-05-19)

## AX-011 Observability

- **Stance**: decided · **Status**: built
- **Ruling**: `pino` JSON logs with correlation id, OpenTelemetry to Grafana Cloud, alerts on 5xx rate above 1 % over 5 min and pg-boss job lag above 10 min (ruled 2026-05-19)

## AX-013 Telemetry ingest (shore power)

- **Stance**: open · **Status**: —
- **Note**: no ruling; FEAT-027 is expected to bring the first proposal to the desk
