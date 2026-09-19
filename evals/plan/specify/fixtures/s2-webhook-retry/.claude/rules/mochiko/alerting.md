---
paths:
  - "src/alerts/**"
  - "src/notifier/**"
---

# Alert delivery <!-- GI-005 -->

- Every alert attempt MUST be recorded with its outcome before the next attempt starts.
- A webhook call MUST time out within 10 seconds; a hung call is a failed attempt.
- A channel that fails MUST NOT block delivery on the incident's other channels.
- The notifier MUST NOT log the webhook URL; log the channel id only.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-005.
