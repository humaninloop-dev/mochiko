# Northstar

Invoicing for freelancers and micro-agencies. About 4,200 paying accounts; the median
account sends 11 invoices a month and has 9 active clients.

## Layout

```
apps/web        React 18 + Vite + TypeScript — the product UI, built on Compass
apps/api        Fastify + PostgreSQL — the API; dev fixtures under apps/api/fixtures/
packages/compass  the Northstar design system: tokens, React components, catalogue docs
```

## Working agreements

- Every product surface is built from Compass. A screen that needs a pattern Compass does
  not have takes it to the design guild (Thursdays) rather than inventing one locally.
- Feature specs live under `.mochiko/specs/<feature>/`; a feature's clickable mock lives beside
  its spec at `.mochiko/specs/<feature>/prototype/` and is walked at the Monday design review
  before the stories are frozen.
- `npm run dev` from `apps/web` starts the app against the fixture API.

## Now

Recurring invoices — `.mochiko/specs/recurring-invoices/spec.md`.
