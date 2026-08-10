---
paths:
  - "src/domain/**"
  - "src/**/invoic*"
  - "src/**/payment*"
  - "src/**/webhook*"
  - "src/**/money*"
---

# Financial correctness <!-- GI-010, GI-020, GI-021, GI-022 -->

- Money MUST be represented and computed as exact decimal (e.g. `Decimal` / integer minor units), never floating point — invoice amounts, line items, totals, and the cash-flow view included. Rounding MUST be explicit at display boundaries. <!-- GI-020 -->
- For payments Stripe collects, an invoice MUST NOT be marked paid without a confirmed Stripe signal, and payment webhooks MUST NOT be silently dropped. <!-- GI-010 -->
- Manual mark-as-paid (checks / cash) is a first-class, deliberate transition — it MUST be supported, and it MUST carry the audit trail and idempotency guarantees below. <!-- GI-010 -->
- Payment-state transitions driven by Stripe webhooks MUST be idempotent: each Stripe event id MUST be processed at most once, and re-applying the same event MUST produce no additional state change. The manual mark-as-paid path MUST be equally safe (a double mark-as-paid, or marking paid on an already-settled invoice, MUST NOT double-count). <!-- GI-021 -->
- Every payment-state change MUST record who made it and when — with particular force on the manual mark-as-paid path, which has no Stripe record behind it. <!-- GI-022 -->

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-010, GI-020, GI-021, GI-022.
