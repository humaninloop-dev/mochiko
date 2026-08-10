---
paths:
  - "frontend/**/*.tsx"
  - "frontend/**/*.jsx"
  - "frontend/**/*.ts"
---

# Accessibility (WCAG 2.1 AA baseline) <!-- GI-011 -->

The customer-facing UI is served to US users; accessibility is a legal-mandate obligation (ADA),
authored at a baseline — full manual WCAG audit deferred.

- Customer-facing UI MUST meet a WCAG 2.1 AA baseline: semantic HTML, labelled form controls,
  keyboard operability of every interactive element, and sufficient colour contrast.
- Accessibility MUST be checked automatically in CI via `eslint-plugin-jsx-a11y`, and interactive
  components SHOULD carry axe assertions in their component tests.
- Images and icons conveying meaning MUST carry text alternatives.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-011.
