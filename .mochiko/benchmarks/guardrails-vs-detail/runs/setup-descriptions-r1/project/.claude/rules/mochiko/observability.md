---
paths:
  - "ledgerline/**/*.py"
---

# Observability (floor essentials) <!-- GI-006 -->

- A `/health` (or `/healthz`) endpoint MUST exist and report service liveness.
- Logs MUST NOT contain PII (names, emails, addresses) or secrets/tokens.
- Log at appropriate levels; log failures with enough context to diagnose.

Deeper observability (structured-log tooling, correlation/trace IDs beyond a money-path request ID,
APM/dashboards, SLOs) is **waived** pre-launch — see the waiver in the ledger (GI-012), revisited at
launch. A minimal request ID on the money/payment path is kept for debuggability.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-006.
