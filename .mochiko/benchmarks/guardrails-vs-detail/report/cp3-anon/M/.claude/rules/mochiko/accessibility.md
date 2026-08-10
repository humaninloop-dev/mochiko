---
paths:
  - "frontend/**/*.tsx"
  - "frontend/**/*.ts"
  - "frontend/**/*.jsx"
  - "frontend/**/*.css"
---

# Accessibility — WCAG 2.1 AA (contractor-facing app) <!-- GI-010 -->

Legal-mandate (ADA / US accessibility statutes) — **unwaivable**. Scope: the contractor-facing
React app. (The client portal is out of scope for v1; clients touch only email + Stripe's hosted
page.)

- The contractor-facing UI MUST meet WCAG 2.1 AA: semantic HTML, labels for every form control, sufficient color contrast, and full keyboard navigation.
- `eslint-plugin-jsx-a11y` MUST pass (part of `eslint .`), and automated `axe-core` checks MUST run against key screens in CI.
- A manual keyboard + contrast pass MUST be performed before a release that changes UI.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-010.
