---
paths:
  - "**/*"
---

# Security at boundaries <!-- GI-004 -->

- Authentication MUST be enforced at every HTTP boundary; a route without `requireSession` or
  `requireMaker` MUST be listed in `src/auth/public-routes.ts` with a reason.
- Sessions MUST be server-side (Redis) with a 12-hour absolute lifetime; no session token is
  ever placed in a URL.
- All external inputs MUST be validated with a Zod schema before any handler logic runs.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-004.
