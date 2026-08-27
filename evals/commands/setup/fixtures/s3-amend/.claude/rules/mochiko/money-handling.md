---
paths:
  - "src/billing/**"
  - "src/api/invoices/**"
---

# Money handling <!-- GI-005 -->

- Amounts MUST be stored and computed as integer minor units; floating point MUST NOT
  appear anywhere on a money path.
- Every amount MUST carry its ISO 4217 currency code; a bare number MUST NOT cross a
  module boundary.
- Rounding MUST happen once, at presentation, half-up.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-005.
