---
paths:
  - "backend/**/*.py"
---

# Tenant isolation <!-- GI-014 -->

Ledgerline is multi-tenant: many contractors share one deployment and each MUST see only their
own data. Authentication (a valid session) is not sufficient — authorization MUST be scoped to
the authenticated tenant.

- Every data-access query MUST be scoped to the authenticated contractor (tenant) — no query may
  return or mutate another tenant's rows.
- A contractor MUST NOT be able to read or mutate another contractor's records by object ID
  (no BOLA/IDOR).
- Cross-tenant access MUST be covered by an integration test proving contractor A receives 403 or
  404 when fetching contractor B's invoice by ID.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-014.
