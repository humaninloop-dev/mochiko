# Concern Ledger — Pallet (excerpt: the rows the supplier-payouts feature touches)

## AX-003 Payments (inbound)

- **Stance**: decided · **Status**: built
- **Ruling**: Stripe (EU entity) collects retailer payments into Pallet's own Stripe balance;
  Pallet never holds card or bank numbers (ruled 2026-04-02)
- **Enforcement**: the contract suite asserts no PAN- or account-shaped field on any schema;
  `src/payments/` is the only module that imports the Stripe SDK (lint `PLT004`)
- **Note (2026-07-22)**: legal is checking whether holding retailer funds and paying them on
  to suppliers makes Pallet a payment institution needing FCA authorisation; until legal
  answers, no product commitment may depend on Pallet holding funds for longer than the
  weekly cycle. Owner: Marcus (CTO). Open.

## AX-009 Datastore

- **Stance**: decided · **Status**: built
- **Ruling**: PostgreSQL 16 is the single system of record; a second datastore needs a
  platform sign-off recorded on this row before any code (ruled 2026-04-02)
- **Rationale**: four engineers, one on-call rota, no DBA
- **Upgrade trigger**: a workload PostgreSQL demonstrably cannot serve at our scale, with the
  measurement attached
- **Sign-offs recorded**: none

## AX-014 Outbound payments (payouts)

- **Stance**: open · **Status**: —
- **Note**: no ruling; FEAT-016 is expected to bring the first proposal to the desk
