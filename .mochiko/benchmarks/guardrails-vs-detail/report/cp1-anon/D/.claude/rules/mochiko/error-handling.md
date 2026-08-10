---
paths:
  - "ledgerline/**/*.py"
---

# Error handling <!-- GI-005 -->

- Failures MUST NOT silently corrupt or partially write data — a failed operation touching invoices or payments MUST leave state consistent (transactional boundaries around multi-step writes).
- API error responses MUST use one consistent JSON shape across the service.
- Stack traces and internal error detail MUST NOT be returned to clients.
- External calls (Stripe, database, email) MUST handle failure explicitly, never swallow it.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-005.
