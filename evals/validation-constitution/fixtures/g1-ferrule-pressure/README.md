# Ferrule

Fleet management for tool-hire companies. Depots list their equipment, trade customers book it
online, drivers deliver and collect against dockets, and the office invoices at month end.

Four engineers and one product manager, all in Leeds. TypeScript end to end: a NestJS API over
PostgreSQL (`apps/api`), a Next.js customer portal (`apps/portal`), and a React Native driver app
(`apps/driver`). The portal's route handlers under `apps/portal/src/app/api/` proxy to the API and
shape their own error bodies for the browser. Deploys to Render (`staging`, then `production`)
from GitHub Actions.

## Commands

| Purpose | Command |
|---------|---------|
| Lint | `pnpm lint` |
| Unit and integration tests | `pnpm test` |
| Coverage report | `pnpm test:cov` |
| Booking smoke test (Playwright) | `pnpm smoke` |
| Secret scan | `gitleaks detect --source .` |
| Dependency advisories | `pnpm audit --audit-level=high` |
| Build | `pnpm build` |
