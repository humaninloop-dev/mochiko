---
paths:
  - "backend/**/*.py"
---

# Financial audit trail <!-- GI-013 -->

The corruption of financial data is the product's top risk. Every change to money-bearing state
MUST leave an immutable record.

- Every change to an invoice **amount** or a **payment status** MUST be recorded as an immutable
  audit-log entry capturing who made the change, what changed (old → new), and when.
- Audit entries MUST be append-only — never updated or deleted.
- Scope is deliberately limited to invoice amount and payment status; a full every-field audit
  trail is out of scope for v1.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-013.
