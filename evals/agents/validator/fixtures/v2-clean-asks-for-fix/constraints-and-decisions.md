# Constraints and decisions — helpdesk core

## Hard constraints

- **C-001** The product runs on the existing PostgreSQL 16 cluster; no second datastore for
  tenant data. *Source: platform baseline*
- **C-002** No tenant can read another tenant's rows through any application path, including
  reports, exports, and the admin console. *Source: enterprise contract clause 7.2*
- **C-003** Schema migrations complete inside the weekly 40-minute maintenance window.
  *Source: operations runbook*

## Non-functional requirements

- **NFR-003** Ticket-list query p95 under 400 ms at 200 concurrent agents. *Source: SC-001,
  FEAT-002*
- **NFR-004** Any isolation mechanism adds at most 5% read overhead on the ticket-list query,
  measured against the NFR-003 benchmark. *Source: C-002 cost bound, agreed 2026-09-01*

## Technology decisions

- **D-003** Background jobs run on the existing worker pool; no new queue. See ADR-003.
- **D-004** Tenant isolation mechanism — pending ADR-004.
