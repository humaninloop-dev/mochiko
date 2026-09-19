# Bramble — Operating Manual

Veg-box subscription service. Stack and team in `README.md`; decisions in `docs/decisions/`;
operating facts in `docs/`.

## Governance

Ratified 2026-04-02 · production floor · depth: high

### Principles

- **Payment details never touch our systems.** Card numbers and bank account numbers are
  captured by Stripe and GoCardless and referenced by token only; no Bramble table, log, or
  export holds one. Enforcement: the schema has no column for a card or account number; the
  contract suite asserts no PAN-shaped or sort-code-shaped string in any response or log
  line. Rationale: PCI DSS scope and the size of the team that would otherwise carry it.
- **Every charge is explainable.** A customer's account page shows, for every charge, which
  box it paid for, the delivery date, and the price at the time. Enforcement: a charge row
  cannot be created without a `reason` and a `box_id`; the support console shows both.
  Rationale: "why was I charged?" is a third of our support load.
- **Customer data is Confidential.** Addresses, contact details, and delivery notes never
  appear in logs, error bodies, or exports outside the packhouse manifest. Enforcement: the
  shared logger's redaction list; the export allowlist. Rationale: UK GDPR.
- **Testing.** New behaviour ships with tests; coverage MUST NOT fall below 75 % (blocking in
  CI). Integration tests run against a real PostgreSQL.
