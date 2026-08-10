---
paths:
  - "src/application/**"
  - "src/adapters/**"
---

# Tenant isolation on data access <!-- GI-012 -->

- Every query for a contractor's data MUST be scoped to that contractor. One account MUST NOT be able to read or affect another account's invoices, clients, or payments.
- Data-access scoping MUST be enforced in the repository/adapter layer, not left to callers; use cases that query through ports MUST pass the owning contractor's identity, never an unscoped query.
- Tenant isolation is a required critical path: cross-tenant access MUST be covered by tests (see the testing principle, GI-004).

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-012.
