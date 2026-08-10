# Glossary

Domain language for Ledgerline. Format: `**<term>** — <definition> *(minted <date>, <source>)*`.

- **Contractor** — a Ledgerline account holder (the paying user); the tenant boundary. *(minted 2026-08-10, setup)*
- **Client** — a contractor's customer, who receives invoices and pays via Stripe; has no login in v1. *(minted 2026-08-10, setup)*
- **Payment state** — the reconciled status of an invoice (e.g. draft / sent / paid), matched against Stripe as source of truth. *(minted 2026-08-10, setup)*
