# Features

> The system as capabilities — the capability peer of the architecture store
> (`.mochiko/product/architecture/`).
> Entries: one file per capability under `.mochiko/features/` (linked per line).
> Statuses: `proposed` · `in-flight` · `delivered` · `retired`.

| ID | Capability | Status | Hook |
|----|------------|--------|------|
| [FEAT-004](.mochiko/features/FEAT-004.md) | Payment reminders | delivered | Chase unpaid invoices by email on a schedule |
|  | ↳ `pending` per-client reminder cadence | cut by payment-recording | A gentler schedule for long-standing clients |
| [FEAT-003](.mochiko/features/FEAT-003.md) | Payment recording | delivered | Record what a client paid against an invoice and show the balance |
| [FEAT-002](.mochiko/features/FEAT-002.md) | Invoice sending | delivered | Email an invoice with a public link the client can open |
| [FEAT-001](.mochiko/features/FEAT-001.md) | Invoice drafting | delivered | Draft, edit, and preview an invoice before it is sent |
| [FEAT-005](.mochiko/features/FEAT-005.md) | Online payments | proposed (unrefined) | Clients settle an invoice from its public link |
