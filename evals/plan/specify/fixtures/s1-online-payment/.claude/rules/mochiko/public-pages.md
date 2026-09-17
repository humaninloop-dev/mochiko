---
paths:
  - "src/routes/public/**"
  - "src/views/public/**"
---

# Pages a client reaches without an account <!-- GI-006 -->

- A public invoice link MUST be an unguessable token; the numeric invoice id MUST NOT
  appear in any client-facing URL.
- A public page MUST NOT render another studio's data under any token, expired or not.
- A public page MUST be readable without JavaScript; interaction MAY require it.
- Anything a client submits on a public page MUST be rate-limited per token.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-006.
