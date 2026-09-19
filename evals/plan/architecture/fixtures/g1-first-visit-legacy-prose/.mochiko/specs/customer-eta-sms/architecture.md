# Customer ETA SMS — architecture note

*Written for FEAT-002, 2026-03-09, before the feature was built. Kept beside the spec.*

## Why the send moved out of the request

The first version sent the SMS from the assign handler. Twilio's p99 was 6–8 s in
February and the dispatcher's assign click hung for that long; two dispatchers reported
double-assigning because they thought the first click was lost. The send now rides the
`jobs` table: the assign handler commits the job and a `send_eta_sms` row in one
transaction, and the `worker` process picks the row up on its next poll.

## Flow

```mermaid
sequenceDiagram
    participant D as Dispatcher (web)
    participant A as api
    participant P as Postgres
    participant W as worker
    participant M as Mapbox
    participant T as Twilio
    D->>A: POST /jobs/{id}/assign
    A->>P: UPDATE job SET technician, assigned_at; INSERT jobs(kind=send_eta_sms)
    A-->>D: 200 (assigned)
    W->>P: SELECT ... FOR UPDATE SKIP LOCKED
    W->>M: directions(tech_last_position, job_address)
    M-->>W: eta minutes (or error → no ETA in text)
    W->>T: messages.create(to, body)
    T-->>W: sid
    W->>P: UPDATE jobs SET done_at; INSERT events(sms_sent)
```

## Failure behaviour

- Mapbox slow or down: the text goes without an ETA ("your technician is on the way").
  Never blocks the send.
- Twilio down: the row stays in the table; the worker retries with backoff (1, 2, 4, 8 min,
  then every 15 min) for 2 h, then marks `sms_failed` so the dispatcher sees it.
- Duplicate pick-up (two workers, or a crash after send): the handler checks for an
  existing `sms_sent` event on the job before calling Twilio.

## What this does not decide

Whether the `jobs` table is the long-term home for deferred work, or whether a broker
replaces it when there is more than one kind of job. That is a product-level call.
