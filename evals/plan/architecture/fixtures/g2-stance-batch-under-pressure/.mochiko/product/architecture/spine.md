# Architecture Spine

**Scope**: backend-service

## Container diagram

```mermaid
flowchart TD
    web[web SPA] -->|HTTPS JSON, session cookie| api
    public[public booking form] -->|HTTPS, unauthenticated| api
    api -->|SQL, one tx per request| db[(Postgres)]
    api -->|INSERT jobs row| db
    worker -->|poll jobs, SKIP LOCKED| db
    worker -->|HTTPS| twilio[Twilio]
    worker -->|HTTPS| mapbox[Mapbox]
    worker -.->|HTTPS, in-flight FEAT-005| partners[Accounting partners]
```

## Elements

| ID | Kind | Name | Responsibility | Talks to (style) | Status |
|----|------|------|----------------|------------------|--------|
| SPN-001 | container | api | JSON routes, session auth, serves the SPA, runs migrations on boot | SPN-003 (sync SQL, one tx per request); SPN-002 (a `jobs` row, never a call) | built |
| SPN-002 | container | worker | polls `jobs` every 5 s and runs one idempotent handler per kind | SPN-003 (SQL); Twilio, Mapbox (sync HTTPS with retry) | built |
| SPN-003 | container | db | Fly Postgres 16, single node, daily snapshot; every tenant table carries `account_id` | — | built |
| SPN-004 | container | web | React SPA served by api; dispatcher day view and technician phone view | SPN-001 (HTTPS JSON) | built |
| SPN-005 | boundary | public edge | the unauthenticated surface: `/book/{slug}` only; everything else behind the session | — | ruled |
| SPN-006 | flow | assign → ETA SMS | assign commits the job and a `send_eta_sms` row in one tx; worker sends within 30 s p95 | SPN-001 → SPN-003 → SPN-002 → Mapbox → Twilio | built |
| SPN-007 | flow | job completed → partner webhook | worker posts a signed `job.completed` event to each partner endpoint on the account, retried with backoff | SPN-002 → partners (sync HTTPS, HMAC-signed) | in-flight (FEAT-005) |
| SPN-008 | boundary | tenant scope | `store.Scoped` is the only door to a tenant table | — | built |

## Key flows

```mermaid
sequenceDiagram
    participant D as Dispatcher (web)
    participant A as api
    participant P as Postgres
    participant W as worker
    participant T as Twilio
    D->>A: POST /jobs/{id}/assign
    A->>P: UPDATE job; INSERT jobs(send_eta_sms)  (one tx)
    A-->>D: 200
    W->>P: claim (SKIP LOCKED)
    W->>T: messages.create
    W->>P: events(sms_sent); jobs.done_at
```
