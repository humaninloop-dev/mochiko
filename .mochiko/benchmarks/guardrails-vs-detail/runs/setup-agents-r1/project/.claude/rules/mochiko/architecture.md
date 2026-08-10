---
paths:
  - "src/**"
---

# Architecture — ports & single responsibility <!-- GI-007, GI-008 -->

- External systems (Stripe, the database) MUST be accessed through small port interfaces so payment-state logic is unit-testable against fakes — no direct SDK/ORM calls from business logic. <!-- GI-007 -->
- Business logic MUST be testable without a real Stripe or a real database (fakes/ports), so payment-state rules can be asserted in isolation. <!-- GI-007 -->
- Each module, class, and function MUST have one clear responsibility; when a component has multiple reasons to change, split it. No `utils`/`helpers` dumping grounds. <!-- GI-008 -->
- Cyclomatic complexity MUST stay ≤ 10 per function (Ruff C901, CI-blocking). Function parameter count (≤5), function length, and file length are advisory (review-only), not build-blocking. <!-- GI-008 -->

> Deliberately NOT enforced: the strict 4-layer import-linter gate (layer-rules module declined, GI-016). Port boundaries are enforced by fakes-based tests and code review, not an import linter.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-007, GI-008.
