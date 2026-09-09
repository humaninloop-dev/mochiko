# INC-31 — Notification backlog during rota publish (2026-08-20)

**Impact**: for about an hour on Monday morning, 35–40 minutes between a manager publishing
a rota and staff receiving the SMS, across all venues; three swap decisions and two
time-off decisions were notified 35–40 minutes late. Two staff at Harbour Kitchen received
the "rota published" SMS twice.

**Cause**: the Monday 09:00 publish burst (4,100 SMS across 61 venues) hit a worker with
concurrency 2; Twilio's p95 per send is 1.4 s; the single `celery` queue serialised every
other task behind the burst. The duplicates: two `send_sms` tasks timed out waiting on
Twilio after the message had already been accepted, and Celery retried them. We had not
seen a duplicate send before this.

**Mitigation (2026-08-20)**: swap and time-off decisions send their SMS directly from the
`api` request (`api/swaps.py`, `api/timeoff.py`), bypassing the queue. The AX-003 guard
test was marked skip for those two modules.

## Follow-ups

- [x] Resize the worker: concurrency 2 → 8, plan starter → standard (2026-08-28). The
      publish burst now drains in under six minutes.
- [ ] Revert the inline sends and re-enable the AX-003 guard (owner: FEAT-014, since it
      touches the same endpoint).
- [ ] Duplicate sends: decide whether provider sends need an idempotency key (owner: TBD).
