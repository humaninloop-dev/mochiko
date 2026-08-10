# Glossary

> The project's domain language. Format: `**<term>** — <definition> *(minted <date>, <source>)*`.

**contractor** — a Ledgerline account holder (plumber, electrician, freelance designer) who issues invoices *(minted 2026-08-10, setup)*
**client** — a contractor's own customer, billed on an invoice; the data subject for stored PII *(minted 2026-08-10, setup)*
**invoice** — a billing document with line items, tax rate, and due date, moving through draft/sent/viewed/paid/overdue *(minted 2026-08-10, setup)*
**payment status** — the authoritative state of an invoice's payment (must never report a false "paid") *(minted 2026-08-10, setup)*
**mark as paid** — a manual payment recording for checks/cash, outside Stripe *(minted 2026-08-10, setup)*
