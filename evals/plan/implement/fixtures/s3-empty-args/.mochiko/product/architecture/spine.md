# Architecture store — spine (notely)

## Topology

| Element | Kind | Status | Feature key |
|---------|------|--------|-------------|
| api-service | container (stdlib HTTP, single process) | ruled | — |
| notes-db | datastore (SQLite file) | ruled | — |

Flow: client → api-service → notes-db. Synchronous request/response only; no queues,
no background workers.

## Concern catalog

- AX-001 — Persistence: SQLite file beside the binary; write-through on create;
  NFR-001: a created note survives process restart (graded by FEAT-001 SC-001).
  Status: ruled.
- AX-002 — Logging: one structured line per request (method, path, status, ms) to
  stdout. NFR-002: every 4xx/5xx carries a reason field. Status: ruled.
- AX-003 — Auth: n-a in v1 — local single-user deployment (C-001). Trigger to revisit:
  any network exposure beyond localhost.
