---
paths:
  - "ledgerline/domain/**/*.py"
  - "ledgerline/**/invoic*.py"
  - "ledgerline/**/payment*.py"
  - "ledgerline/**/tax*.py"
---

# Money correctness <!-- GI-010 -->

- Any code touching invoice totals, tax math, or payment status MUST have accompanying tests; these tests are blocking for both merge and deploy — no "fix it after".
- Monetary amounts MUST use `decimal.Decimal` (or an equivalent exact type), never `float`.
- Payment status MUST derive from Stripe as the source of truth. Displayed status MUST be reconciled with Stripe (webhook-driven, with a periodic safety poll) so it never silently drifts.
- Reconciliation tests MUST cover the missed-webhook failure mode (the poll catches what a dropped webhook missed).
- `mypy` MUST pass strict over `ledgerline/domain`.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-010.
