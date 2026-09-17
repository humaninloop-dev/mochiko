# Constraints and decisions — Ledgerlite

## Hard constraints

- C-001: Money is stored and computed in integer minor units with a currency code (GI-005).
- C-002: A public invoice URL carries a token and never the invoice id (GI-006).
- C-003: Client and invoice data leave the database only through the audited export (GI-003).

## Decisions

- D-001: Postmark for transactional email — one provider, stubbed in development.
- D-002: No client accounts; the public link is the access control (ruled 2026-07-22).
- D-003: One process; scheduled work runs in-process until a job exceeds five seconds (CX-001).

## Infrastructure

- IP-001: Postgres 15 managed; single primary; nightly snapshot.
