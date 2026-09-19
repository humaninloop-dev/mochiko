# Ferrule — Working Notes

Ferrule is the fleet-management platform for tool-hire depots. Read `README.md` for the stack and
the commands, and `docs/domain.md` for the vocabulary (depot, asset, booking, run, docket).

## Conventions the team keeps

- Feature branches off `main`; squash-merge with a PR description that names the booking
  scenarios it changes.
- The driver app is offline-first. Every screen must survive a lost connection without losing a
  docket, and the sync worker owns reconciliation, never a screen.
- Money is integer pence everywhere. `Decimal` never crosses a module boundary.

<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v1.0.0 · 2026-09-08 · production floor · depth: low · modules: gdpr <!-- GI-001 (fact profile) · GI-003 (depth level) -->

### Principles

- Secrets MUST stay out of the repository: loaded from Render environment groups, never committed; every `.env*` file is gitignored; `gitleaks` runs on every commit locally and on every push in CI, a dependency advisory scan runs on every PR, and all external inputs MUST be validated before processing (NON-NEGOTIABLE) <!-- GI-004 -->
- `pnpm smoke` MUST stay green on the booking path, coverage MUST be reported on every PR, and the coverage baseline MUST NOT decrease (NON-NEGOTIABLE) <!-- GI-005 -->
- A failure MUST NOT silently corrupt a booking or a docket; every caught error is logged or re-thrown, never swallowed, and no stack trace reaches a response body (NON-NEGOTIABLE) <!-- GI-006 -->
- Logs MUST exist on the booking and docket paths and MUST NOT contain personal data (NON-NEGOTIABLE) <!-- GI-007 -->
- Personal data of customers and drivers MUST be kept no longer than the retention schedule in the ledger; subject access and erasure requests MUST be fulfilled within 30 days; a personal-data breach MUST be reported to the ICO within 72 hours of discovery <!-- GI-008 -->
- Code SHOULD be maintainable and readable <!-- GI-009 -->
- API responses SHOULD be fast <!-- GI-010 -->
- API error responses — see `.claude/rules/mochiko/api-errors.md` <!-- GI-011 -->
- Error responses MUST use RFC 7807 Problem Details with `type`, `title`, `status`, `detail`, and `correlation_id`; the `ProblemFilter` is the only place an error body is built <!-- GI-011 -->
- Commit messages MUST follow Conventional Commits <!-- GI-014 -->

### Technology stack

- TypeScript 5.5 · Node 22 · NestJS 10 (`apps/api`) · Next.js 15 (`apps/portal`) · React Native 0.75 with Expo (`apps/driver`) <!-- GI-002 -->
- PostgreSQL 16 on Render · Prisma 5 · BullMQ on Redis for run planning and invoicing jobs <!-- GI-002 -->

### Quality gates

- `pnpm lint` MUST pass before merge <!-- GI-002 -->
- `pnpm test` MUST pass before merge; `pnpm test:cov` reports coverage on every PR and the baseline MUST NOT decrease <!-- GI-005 -->
- `pnpm smoke` MUST pass before merge <!-- GI-005 -->
- Security scan: `[SECURITY_COMMAND]` MUST pass before merge <!-- GI-004 -->

### Release gates

**Environments:** `staging` → `production` on Render; promotion is a manual approval step in GitHub Actions.
**Cadence:** every merge deploys to `staging`; `production` is promoted by the on-call engineer, weekdays only.

| Gate | Requirement | Verified by | Blocks |
|------|-------------|-------------|--------|
| Staging soak | two hours on `staging` with no new Sentry issue | Sentry release view | promotion to production |
| Migration check | `prisma migrate diff` reports a reversible migration | GitHub Actions `migrate-check` job | deploy |
| Smoke | `pnpm smoke` against `staging` | GitHub Actions `smoke` job | promotion to production |

**Rollback:** redeploy the previous Render release from the dashboard; the on-call engineer owns it.

### Governance operations

- Ledger (waivers · amendment policy · exceptions · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup` (fact-profile changes — module attach/detach — and un-waives are governance events)
<!-- mochiko:output-style:begin -->
- Writing style — conversation: `full` · reports: `ultra` *(internal agent hand-offs)* · documents: `full`. Terse and plain-English by default. Set any of the three to `off`, `lite`, `full`, or `ultra` and your choice is kept when this section is regenerated; "normal mode" turns it off for one session.
<!-- mochiko:output-style:end -->
- Operating docs (knowledge-management module): sessions in `.mochiko/brainstorms/` + `index.md`; rulings land in `DECISIONS.md`; open threads in `BACKLOG.md`; direction in `ROADMAP.md`; landing ritual + invariants at `.mochiko/memory/knowledge-management.md`; groom: `mochiko:grooming-operating-docs` <!-- GI-XXX -->
<!-- mochiko:governance:end -->

## Handy

- Render dashboard: the `ferrule` team; ask Priya for access.
- The Playwright smoke run needs `SMOKE_DEPOT_ID` set to the Leeds demo depot.
