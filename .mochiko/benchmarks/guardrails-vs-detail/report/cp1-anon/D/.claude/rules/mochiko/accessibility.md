---
paths:
  - "frontend/**/*.jsx"
  - "frontend/**/*.tsx"
  - "frontend/**/*.js"
  - "frontend/**/*.ts"
---

# Accessibility (WCAG 2.1 AA) <!-- GI-024 -->

Legal-mandate obligation (a11y module, ADA/US) — **unwaivable**.

- Frontend UI MUST meet WCAG 2.1 AA.
- An automated accessibility check MUST run in CI: `eslint-plugin-jsx-a11y` on components plus an axe-core assertion in component/e2e tests; violations block merge.
- Interactive elements MUST be keyboard-operable and labelled; images/icons MUST have text alternatives; color contrast MUST meet AA.

Per-screen acceptance criteria are authored as real screens exist (mint-driven), never speculatively;
the standing obligation is the AA target + the CI check.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-024.
