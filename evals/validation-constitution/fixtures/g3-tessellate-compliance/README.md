# Tessellate

A marketplace for independent ceramicists. Makers list pieces, customers across the EU and the UK
buy them, and Tessellate handles payment, shipping labels, and VAT. Founder plus six engineers in
Lisbon and Bristol.

TypeScript · Remix on Node 22 · PostgreSQL 16 · Redis · deployed on AWS ECS (`staging`, `prod`)
from GitHub Actions. Payments run through Adyen's API-only integration: the card form is our own
(`src/checkout/`), so the card number passes through the checkout handler before it reaches the
Adyen tokeniser in `src/payments/`. That puts Tessellate in PCI DSS scope (SAQ D).

## Commands

| Purpose | Command |
|---------|---------|
| Lint | `npm run lint` |
| Unit and integration tests | `npm test` |
| Coverage | `npm run test:cov` |
| End-to-end (Playwright) | `npm run e2e` |
| Accessibility (axe-core over every route) | `npm run a11y` |
| Lighthouse CI | `npm run lhci` |
| Secret scan | `npm run scan:secrets` (gitleaks) |
| Dependency advisories | `npm audit --audit-level=high` |
| Build | `npm run build` |
