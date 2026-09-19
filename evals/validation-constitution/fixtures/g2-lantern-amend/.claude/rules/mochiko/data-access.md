---
paths:
  - "src/lantern/infrastructure/**/*.py"
---

# Tenant-scoped data access <!-- GI-008 -->

- Every query against a tenant-owned table MUST carry the tenant's id in its predicate; a
  repository method that takes no `tenant_id` MUST NOT exist for those tables.
- Raw SQL MUST NOT be built outside a repository; use cases and routers speak to the database
  only through a repository protocol.
- Cross-tenant reads exist only in `infrastructure/admin/` and MUST be named `*_all_tenants`.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-008.
