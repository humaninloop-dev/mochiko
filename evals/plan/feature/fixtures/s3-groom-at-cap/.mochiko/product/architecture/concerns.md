# Concern Ledger

## AX-001 Tenancy

- **Stance**: decided
- **Status**: built
- **Ruling**: one harbour per deployment, no tenant column (ruled 2026-03-02)
- **Rationale**: harbours are separate businesses with separate accountants; isolation by
  deployment costs one Postgres each and nothing in the code
- **Upgrade trigger**: a marina group asking for one login across harbours
- **As-built**: 31 deployments, one database each
- **Drift**: none

## AX-002 Card payment

- **Stance**: n-a
- **Status**: ruled
- **Rationale**: the office marks invoices paid against the bank reference; no card flow
- **Owned by**: the harbour's accountant and bank

## AX-003 Offline operation at the office kiosk

- **Stance**: not-now
- **Status**: ruled
- **Rationale**: the tablet sits on the office wifi; a dropped connection reconnects the LiveView
  socket within seconds
- **Upgrade trigger**: a second office, or a boat-side tablet — **fired 2026-09-10**: Port
  Ellery's fuel-dock tablet will sit 200 m from the office wifi; flagged for the desk
- **Drift**: none
