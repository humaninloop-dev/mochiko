---
paths:
  - "src/api/**"
  - "src/**/routes*"
  - "frontend/**"
  - "**/*.tsx"
---

# Error handling <!-- GI-005 -->

- Failures MUST NOT silently corrupt data — a failed operation MUST leave state consistent (no half-applied payment, no orphaned invoice). <!-- GI-005 -->
- API error responses MUST follow a consistent schema (RFC 7807 Problem Details JSON) and MUST include a correlation/trace id. <!-- GI-005 -->
- Stack traces MUST NOT be exposed to clients. <!-- GI-005 -->
- The React UI MUST render explicit error states for failed operations, never a blank or frozen screen. <!-- GI-005 -->

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-005.
