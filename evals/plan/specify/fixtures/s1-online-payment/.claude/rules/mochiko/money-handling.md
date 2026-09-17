---
paths:
  - "src/billing/**"
  - "src/routes/invoices/**"
  - "src/routes/payments/**"
---

# Money handling <!-- GI-005 -->

- Amounts MUST be stored and computed as integer minor units; floating point MUST NOT
  appear anywhere on a money path.
- Every amount MUST carry its ISO 4217 currency code; a bare number MUST NOT cross a
  module boundary.
- Rounding MUST happen once, at presentation, half-up.
- A payment MUST never exceed the invoice balance; overpayment is rejected, not credited.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-005.
