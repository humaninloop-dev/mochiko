# Glossary — Ledgerline

**Contractor** — a solo independent tradesperson or freelancer; the Ledgerline account holder (the tenant) *(minted 2026-08-10, setup)*
**Client** — the contractor's customer, to whom an invoice is issued; not a Ledgerline account holder *(minted 2026-08-10, setup)*
**Invoice** — a billable record a contractor issues to a client (line items, amounts in integer cents, tax, due date, payment status) *(minted 2026-08-10, setup)*
**Payment state** — an invoice's payment status, whose source of truth is Stripe, reconciled back to Ledgerline *(minted 2026-08-10, setup)*
**Tenant** — the isolation unit for data access; one contractor account, whose data no other account may read or affect *(minted 2026-08-10, setup)*
