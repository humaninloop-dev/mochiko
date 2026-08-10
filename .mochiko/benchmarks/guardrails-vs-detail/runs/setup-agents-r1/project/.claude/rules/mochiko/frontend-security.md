---
paths:
  - "frontend/**"
  - "src/web/**"
  - "**/*.tsx"
  - "**/*.jsx"
---

# Frontend security <!-- GI-003 (frontend expression) -->

- No secret keys may ship in the frontend bundle. Only Stripe's publishable key may reach the client; all secret keys stay server-side. <!-- GI-003 -->
- Any UI surface that touches the API MUST validate input and encode output (XSS defense) — never render untrusted data as HTML without escaping. <!-- GI-003 -->

> The remaining frontend floor surface (richer client-side error-state coverage, UI observability) is a recorded known-thin spot (GI-024) — not a waiver; the two clauses above are the firmed, non-waivable UI-side security floor.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-003.
