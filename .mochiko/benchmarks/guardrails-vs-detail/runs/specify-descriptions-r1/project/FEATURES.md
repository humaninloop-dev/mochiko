# Features

> The system as capabilities — the capability peer of `ARCHITECTURE.md`.
> Entries: `.mochiko/features/FEAT-XXX-<slug>.md` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

| ID | Feature | Status | Capability |
|----|---------|--------|------------|
| [FEAT-002](.mochiko/features/FEAT-002-invoice-lifecycle.md) | Invoice lifecycle | in-flight | End-to-end life of an invoice: draft → deliver → pay → track → follow up (parent) |
| [FEAT-003](.mochiko/features/FEAT-003-client-management.md) | ↳ Client management | in-flight | Maintain the clients a contractor bills |
| [FEAT-004](.mochiko/features/FEAT-004-invoice-authoring.md) | ↳ Invoice authoring | in-flight | Draft invoices with line items, tax, due date, gap-free numbering |
| [FEAT-005](.mochiko/features/FEAT-005-invoice-delivery.md) | ↳ Invoice delivery | in-flight | Send invoices by authenticated email with a hosted payment link |
| [FEAT-006](.mochiko/features/FEAT-006-payment-tracking.md) | ↳ Payment tracking & reconciliation | in-flight | Reconcile payment (Stripe + manual), audit trail, void |
| [FEAT-007](.mochiko/features/FEAT-007-payment-reminders.md) | ↳ Payment reminders | in-flight | Auto-overdue + reminder emails on a cadence |
| [FEAT-008](.mochiko/features/FEAT-008-invoice-dashboard.md) | ↳ Invoice dashboard & status view | in-flight | See all invoices and live status in one place |
| [FEAT-001](.mochiko/features/FEAT-001-contractor-accounts.md) | Contractor accounts & authentication | in-flight | Register, sign in (email/Google), per-account isolation (foundation) |
