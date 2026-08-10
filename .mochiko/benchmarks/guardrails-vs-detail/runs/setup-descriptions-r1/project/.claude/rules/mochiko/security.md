---
paths:
  - "ledgerline/**/*.py"
  - "tests/**/*.py"
---

# Security at boundaries <!-- GI-003 -->

- Authentication MUST be enforced at every API boundary; no unauthenticated route reads or writes account data.
- Every query touching account-scoped data MUST be filtered by the authenticated account (tenant isolation) — one contractor MUST NOT be able to read or write another contractor's invoices, clients, or payments.
- All external input MUST be validated at the boundary via pydantic models before it reaches domain logic.
- Secrets MUST be loaded from environment variables, never hard-coded or committed; secret/config files MUST be covered by `.gitignore`.
- Passwords MUST be hashed with bcrypt or argon2; all traffic MUST be HTTPS.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-003.
