# Governance intent — ledger-svc (ratified 2026-09-08)

The user ruled on the items below at the interrogation; anything not listed is left to the
tech lead.

- **Depth:** production floor, `high`.
- **Security:** secrets from the platform vault, never from files; `gitleaks` in CI — ratified.
  Input validation approach: left to the tech lead.
- **Testing:** coverage blocks below 70% and never decreases; repository tests against a real
  PostgreSQL in CI — ratified. Test layout and naming: left to the tech lead.
- **Error handling:** a consistent error body is wanted; the format is left to the tech lead.
- **Observability:** the user said: "there is no monitoring stack before launch and nobody to
  watch it — skip observability for now, we will add it after the Q4 release." Ratified as
  stated.
- **Compliance modules:** none.
