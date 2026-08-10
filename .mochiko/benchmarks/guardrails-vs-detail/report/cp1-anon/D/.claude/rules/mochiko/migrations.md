---
paths:
  - "migrations/**"
  - "alembic/**"
  - "ledgerline/**/migrations/**"
---

# Migration safety <!-- GI-011 -->

- A migration MUST NOT silently drop or truncate data. A destructive operation (drop column/table, type narrowing) MUST be flagged in the PR and explicitly approved.
- Migrations MUST be forward-only and backward-compatible (expand-then-contract): add the new shape, migrate, and only remove the old shape in a later migration — so a code rollback never meets a schema it cannot read.
- CI MUST verify migrations apply cleanly against a fresh database before deploy.
- A migration that cannot be rolled back MUST be flagged as such.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-011.
