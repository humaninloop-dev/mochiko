# Architecture store — spine (Ledgerlite)

**Scope:** backend-service (the server-rendered web app rides the same service)

## Topology

| ID | Element | Kind | Status | Note |
|----|---------|------|--------|------|
| AX-001 | Web app | service | as-built | Fastify, server-rendered views, one process |
| AX-002 | Postgres | store | as-built | single primary, Prisma migrations |
| AX-003 | Email sender | adapter | as-built | Postmark; stubbed in development |
| AX-004 | PDF renderer | component | as-built | in-process, runs on preview and send |
| AX-005 | Reminder scheduler | job | as-built | in-process interval inside AX-001 |

## Concern catalog

| ID | Concern | Stance |
|----|---------|--------|
| CX-001 | No background workers outside the web process | decided — one process until a job exceeds 5 s |
| CX-002 | Third-party callbacks into the service | open — nothing calls us today |
