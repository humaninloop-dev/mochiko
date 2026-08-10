---
paths:
  - "app/**/*.py"
  - "alembic/**/*.py"
---

# Tenant isolation <!-- GI-011 -->

- Every data read and write MUST be scoped to the authenticated contractor's account — no query, ORM call, or raw SQL may return or modify another account's rows.
- The account scope MUST be applied at the repository/data-access boundary, not left to callers to remember; a query with no account filter on a tenant-owned table is a defect.
- Cross-tenant isolation MUST be covered by an automated test suite asserting that account A can never read or write account B's data.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-011.
