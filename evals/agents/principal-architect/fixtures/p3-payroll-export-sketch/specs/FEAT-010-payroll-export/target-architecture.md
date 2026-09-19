# FEAT-010 target architecture — sketch (CTO, 2026-06-01)

I want us set up for the next few services, so here is the shape I'd like the delta to
land on.

## Components

1. **Event bus** — Redpanda (Kafka API). `api` publishes `ShiftApproved`, `ShiftAmended`,
   and `ShiftVoided` events. Event-driven is where the industry has gone; a bus decouples
   everything from day one, and we will want it for the next service anyway.
2. **payroll-service** (new, Python) — consumes the events and keeps its own read model of
   approved hours (CQRS). Owns the nightly export to PayFlow. Also owns the approvals state
   machine — we move it out of `api` so the service is the source of truth for hours — and
   serves the manager's hours reports (by venue, by week), since it has the data anyway.
3. **Job runner** — inside payroll-service: a `job` table with `job_attempt`, exponential
   backoff, a dead-letter table, and an admin endpoint to replay. Home-grown so we control
   it.
4. **api-gateway** — Kong in front of `api` and `payroll-service`. By next year we'll have
   three or four services and I don't want to retrofit routing and auth then.
5. **PayFlow** — external. payroll-service handles PayFlow's `batch.accepted` and
   `batch.rejected` webhooks on `/webhooks/payflow`.

## Tables (payroll-service's own database)

```sql
CREATE TABLE approved_hours_rm (shift_id bigint PRIMARY KEY, venue_id bigint, staff_id bigint,
  hours numeric(5,2), approved_at timestamptz, exported_run_id bigint);   -- read model
CREATE TABLE payroll_export_run (id bigserial PRIMARY KEY, venue_id bigint, night date,
  batch_id text UNIQUE, state text, attempts int DEFAULT 0, last_error text,
  created_at timestamptz DEFAULT now());
CREATE TABLE payroll_line (run_id bigint REFERENCES payroll_export_run, shift_id bigint,
  staff_id bigint, hours numeric(5,2), PRIMARY KEY (run_id, shift_id));
CREATE TABLE job (id bigserial, kind text, payload jsonb, run_after timestamptz, state text);
CREATE TABLE job_attempt (job_id bigint, n int, error text, at timestamptz);
CREATE TABLE dead_letter (job_id bigint, moved_at timestamptz);
```

## Endpoints (payroll-service)

- `POST /exports/{venue_id}/run` — body `{ "night": "2026-06-01" }` → 202
  `{ "run_id": 123 }`
- `GET /exports?venue_id=&from=&to=` → `[ { "run_id", "night", "state", "attempts",
  "last_error" } ]`
- `GET /reports/hours?venue_id=&week=` → `{ "rows": [ { "staff_id", "hours" } ] }`
- `POST /webhooks/payflow` — PayFlow callback, body per their docs
- `POST /approvals/{shift_id}` — moved from `api`; body `{ "hours": 7.5, "note": "" }`

## Diagram

```
 web ─▶ api-gateway ─▶ api ─▶ db
                  │      └─▶ event bus (Redpanda)
                  └─▶ payroll-service ─▶ payroll-db
                            │  ▲
                            │  └── event bus
                            ├─▶ PayFlow (upload)
                            ◀── PayFlow (webhooks)
 worker ─▶ redis   (unchanged)
```
