# Features

> The system as capabilities — the capability peer of `ARCHITECTURE.md`.
> Entries: `.mochiko/features/FEAT-XXX-<slug>.md` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

| ID | Feature | Status | Capability |
|----|---------|--------|------------|
| [FEAT-001](.mochiko/features/FEAT-001-invoice-lifecycle.md) | Invoice lifecycle | in-flight | Solo contractor from new client to paid invoice, no manual bookkeeping |
| [FEAT-002](.mochiko/features/FEAT-002-client-management.md) | ↳ Client management | in-flight | Maintain the clients a contractor bills |
| [FEAT-003](.mochiko/features/FEAT-003-invoice-drafting.md) | ↳ Invoice drafting & void | in-flight | Draft an invoice with line items, tax, due date; void an unpaid one |
| [FEAT-004](.mochiko/features/FEAT-004-invoice-sending.md) | ↳ Invoice sending & delivery | in-flight | Email an invoice with a hosted Stripe payment link |
| [FEAT-005](.mochiko/features/FEAT-005-payment-capture.md) | ↳ Payment capture & recording | in-flight | Record payment — online via Stripe or manual — on an audit trail |
| [FEAT-006](.mochiko/features/FEAT-006-invoice-status-dashboard.md) | ↳ Invoice status & dashboard | in-flight | See every invoice's payment state without opening Stripe |
| [FEAT-007](.mochiko/features/FEAT-007-payment-reminders.md) | ↳ Payment reminders | in-flight | Automatic reminder emails on unpaid invoices, stop when paid or void |
