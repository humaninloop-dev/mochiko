# Features

> The system as capabilities — the capability peer of `ARCHITECTURE.md`.
> Entries: `.mochiko/features/FEAT-XXX-<slug>.md` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

| ID | Feature | Status | Capability |
|----|---------|--------|------------|
| [FEAT-009](.mochiko/features/FEAT-009-authentication.md) | Authentication | in-flight | Email/password + Google sign-in; scoped session foundation |
| [FEAT-001](.mochiko/features/FEAT-001-client-management.md) | Client management | in-flight | Private per-contractor client records reused across invoices |
| [FEAT-002](.mochiko/features/FEAT-002-invoice-authoring.md) | Invoice authoring | in-flight | Draft invoices with exact totals, tax, due date, number, memo; overdue flag on the list |
| [FEAT-003](.mochiko/features/FEAT-003-invoice-delivery.md) | Invoice delivery | in-flight | Preview + send by email with an unguessable payment link; void-and-reissue |
| [FEAT-004](.mochiko/features/FEAT-004-payments.md) | Payments | in-flight | Roll-up over online + manual payment recording |
| [FEAT-005](.mochiko/features/FEAT-005-online-payment-stripe.md) | ↳ Online payment via Stripe | in-flight | Hosted-checkout, webhook-confirmed auto-settlement |
| [FEAT-006](.mochiko/features/FEAT-006-manual-payment-recording.md) | ↳ Manual payment recording | in-flight | Mark paid by check/cash with audit trail |
| [FEAT-007](.mochiko/features/FEAT-007-invoice-status-tracking.md) | Invoice status tracking | proposed | Filterable status dashboard over the invoice list |
| [FEAT-008](.mochiko/features/FEAT-008-overdue-reminders.md) | Automated overdue reminders | proposed | Auto-email overdue-unpaid clients the payment link |
