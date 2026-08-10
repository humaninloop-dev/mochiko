# Features

> The system as capabilities — the capability peer of `ARCHITECTURE.md`.
> Entries: `.mochiko/features/FEAT-XXX-<slug>.md` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

| ID | Feature | Status | Capability |
|----|---------|--------|------------|
| [FEAT-007](.mochiko/features/FEAT-007-client-records.md) | Client records | in-flight | Saved clients invoices are issued against |
| [FEAT-001](.mochiko/features/FEAT-001-invoice-authoring-sending.md) | Invoice authoring & sending | in-flight | Create, tax-total, number, and send invoices; email the payment link |
| [FEAT-005](.mochiko/features/FEAT-005-invoice-tracking-lifecycle-view.md) | Invoice tracking & lifecycle view | in-flight | List, detail, status history, computed overdue |
| [FEAT-002](.mochiko/features/FEAT-002-payment-settlement-reconciliation.md) | Payment settlement & reconciliation | in-flight | Settle invoices to paid and keep state reconciled with the truth |
| [FEAT-003](.mochiko/features/FEAT-003-stripe-payment-reconciliation.md) | ↳ Stripe-hosted payment & reconciliation | in-flight | Client pays via Stripe-hosted checkout; exactly-once reconcile to paid |
| [FEAT-004](.mochiko/features/FEAT-004-manual-payment-recording.md) | ↳ Manual payment recording | in-flight | Record cash/check settlement outside Stripe |
| [FEAT-006](.mochiko/features/FEAT-006-invoice-void.md) | Invoice void | proposed | Void a wrong sent invoice, retained + audited |
| [FEAT-008](.mochiko/features/FEAT-008-overdue-reminder-emails.md) | Overdue reminder emails | proposed | Auto-remind clients at 3/7/14 days overdue |
