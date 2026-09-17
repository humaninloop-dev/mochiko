# Architecture store — spine (Beacon)

**Scope:** backend-service

## Topology

| ID | Element | Kind | Status | Note |
|----|---------|------|--------|------|
| AX-001 | API + settings pages | service | as-built | Fastify, server-rendered |
| AX-002 | Postgres | store | as-built | single primary; shared by every element |
| AX-003 | Checker | worker | as-built | node-cron; runs probes; writes probe events |
| AX-004 | Notifier | worker | as-built | drains an alert queue table; one attempt per channel event; no retry |

## Concern catalog

| ID | Concern | Stance |
|----|---------|--------|
| CX-001 | Outbound HTTP timeouts | decided — 10 s everywhere, undici |
| CX-002 | Work that must survive a process restart | decided — a Postgres table is the queue; no broker |
