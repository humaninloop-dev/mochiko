---
paths:
  - "app/**/*.py"
  - "alembic/**/*.py"
---

# Invoice & payment-state integrity <!-- GI-012 -->

- An invoice and its payment state MUST NOT be silently lost or corrupted; payment state MUST reflect reality (a contractor action or a Stripe event) and reconcile against Stripe as the source of truth. <!-- GI-012 -->
- All monetary values and arithmetic MUST use `decimal.Decimal` — never floating point. <!-- GI-013 -->
- Every inbound Stripe webhook MUST have its signature verified against the signing secret, and every event MUST be processed idempotently (exactly-once); a replayed or forged event MUST NOT change payment state. <!-- GI-026 -->
- Invoice and payment state changes MUST be recorded in an append-only, immutable, traceable log (who changed what, when); historical rows MUST NOT be mutated in place. <!-- GI-029 -->

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-012, GI-013, GI-026, GI-029.
