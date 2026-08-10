---
paths:
  - "src/api/**"
  - "src/**/routes*"
  - "src/application/**"
---

# API authorization & tenant isolation <!-- GI-011 -->

- Every request MUST be authenticated at the API boundary; no endpoint may serve data without an authenticated principal. <!-- GI-003 -->
- A contractor MUST be able to access only their own data (clients, invoices, payments). Every data-access path MUST apply an ownership/tenant check; cross-tenant access is a defect. <!-- GI-011 -->
- All external inputs MUST be validated at the boundary (Pydantic request models) before processing. <!-- GI-003 -->

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-011 (and GI-003 for the auth/validation lines).
