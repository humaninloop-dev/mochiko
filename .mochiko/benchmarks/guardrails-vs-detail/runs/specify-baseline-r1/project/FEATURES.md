# Features

> The system as capabilities — the capability peer of `ARCHITECTURE.md`.
> Entries: `.mochiko/features/FEAT-XXX-<slug>.md` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

| ID | Feature | Status | Capability |
|----|---------|--------|------------|
| [FEAT-001](.mochiko/features/FEAT-001-contractor-accounts.md) | Contractor accounts & access | in-flight | Sign up/in (email+password or Google) into an isolated per-account workspace |
| [FEAT-002](.mochiko/features/FEAT-002-client-management.md) | Client management | in-flight | Maintain the clients a contractor bills |
| [FEAT-003](.mochiko/features/FEAT-003-invoicing.md) | Invoicing (draft & send) | in-flight | Draft invoices with exact-decimal totals and issue them by email with a payment link |
| [FEAT-004](.mochiko/features/FEAT-004-payments.md) | Payments | in-flight | Record payment against an invoice and keep payment state true |
| [FEAT-005](.mochiko/features/FEAT-005-online-payment.md) | ↳ Online payment collection | in-flight | Hosted-checkout payment, verified + idempotent, flips the invoice to paid |
| [FEAT-006](.mochiko/features/FEAT-006-manual-payment.md) | ↳ Manual payment recording | proposed | Mark an invoice paid for offline (check/cash) payment |
| [FEAT-007](.mochiko/features/FEAT-007-payment-follow-up.md) | Payment follow-up | proposed | Keep the contractor informed and chase payment automatically |
| [FEAT-008](.mochiko/features/FEAT-008-status-lifecycle.md) | ↳ Invoice status lifecycle | proposed | Viewed tracking and automatic overdue |
| [FEAT-009](.mochiko/features/FEAT-009-automated-reminders.md) | ↳ Automated reminders | proposed | Configurable reminder cadence, stop on payment |
