---
paths:
  - "src/domain/**"
  - "src/application/**"
  - "src/adapters/**"
---

# Payment-state integrity <!-- GI-010 -->

- Stripe is the single source of truth for payment status; the application's idea of "paid" MUST reconcile to Stripe.
- Code MUST NOT flip an invoice to "paid" via a manual or guessed path; a payment status change MUST originate from a Stripe event or an explicit, logged manual transition.
- Stripe webhook handling MUST be idempotent: a duplicate Stripe event MUST NOT double-count a payment or double-send a payment notification.
- A scheduled reconciliation against Stripe MUST recover missed or dropped webhook events, so an invoice cannot remain "unpaid" while Stripe shows it paid.
- Manual mark-as-paid MUST be recorded as a state transition with actor and timestamp — never a silent field mutation.
- Any change touching invoice status or payment amounts MUST be accompanied by a test proving the state machine behaves; CI blocks the change otherwise.

# Currency representation <!-- GI-011 -->

- Monetary amounts MUST be represented as integer cents. Floating-point types MUST NOT be used for currency values or arithmetic.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-010 and GI-011.
