# Staged FEATURES.md rows (to land at spec acceptance)

Baseline: live FEATURES.md at run open is the greenfield empty scaffold (no existing features).
All entries below are NEW (`proposed`); selected ones flip to `in-flight` at acceptance.

| ID | Feature | Status | Capability |
|----|---------|--------|------------|
| [FEAT-009](.mochiko/features/FEAT-009-authentication.md) | Authentication | proposed | Email/password + Google sign-in; scoped session foundation |
| [FEAT-001](.mochiko/features/FEAT-001-client-management.md) | Client management | proposed | Private per-contractor client records reused across invoices |
| [FEAT-002](.mochiko/features/FEAT-002-invoice-authoring.md) | Invoice authoring | proposed | Draft/edit invoices with exact totals, tax, due date, number, memo |
| [FEAT-003](.mochiko/features/FEAT-003-invoice-delivery.md) | Invoice delivery | proposed | Preview + send invoice by email with a payment link |
| [FEAT-004](.mochiko/features/FEAT-004-payments.md) | Payments | proposed | Roll-up over online + manual payment recording |
| [FEAT-005](.mochiko/features/FEAT-005-online-payment-stripe.md) | ↳ Online payment via Stripe | proposed | Hosted-checkout, webhook-confirmed auto-settlement |
| [FEAT-006](.mochiko/features/FEAT-006-manual-payment-recording.md) | ↳ Manual payment recording | proposed | Mark paid by check/cash with audit trail |
| [FEAT-007](.mochiko/features/FEAT-007-invoice-status-tracking.md) | Invoice status tracking | proposed | At-a-glance filterable status + overdue derivation |
| [FEAT-008](.mochiko/features/FEAT-008-overdue-reminders.md) | Automated overdue reminders | proposed | Auto-email overdue-unpaid clients the payment link |

Ordering note: at acceptance, in-flight and delta-carrying entries sort to the top; here the
selected set (001, 002, 003, 004→005/006) leads, deferred (007, 008) follow.
