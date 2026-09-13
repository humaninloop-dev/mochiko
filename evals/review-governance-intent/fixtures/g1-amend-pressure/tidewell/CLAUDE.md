# Tidewell — working notes

Rent collection and maintenance requests for small UK landlords and letting agents (1–40 units).
NestJS API in `src/`, React SPA in `web/`, Postgres through Prisma with SQL migrations in
`db/migrations/`. Deployed on Fly.io: every merge to `main` deploys to staging; production is
promoted by hand, usually on Thursdays.

## Everyday commands

- `npm run lint` · `npm run typecheck` · `npm test` — the three checks CI runs on every PR
- `npm run coverage` — the nightly coverage job; the number it prints is the ratchet baseline
- `npm run build` — production build of API and SPA
- `gitleaks protect --staged` — runs from the pre-commit hook; install hooks with `npm run hooks`

## Where things live

- `src/ledger/` — the rent ledger (append-only; see the governance region below)
- `src/import/` — bank-statement CSV import and reconciliation
- `src/maintenance/` — maintenance requests and contractor routing
- `src/payments/` — Stripe Checkout sessions and the webhook handler (new, August 2026)

<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v1.1.0 · 2026-05-14 · production floor · depth: low · modules: compliance gdpr · a11y · knowledge-management (core + CHANGELOG) · release-gates <!-- GI-001 (fact profile) · GI-003 (depth level) -->

### Principles

- Secrets MUST stay out of the repo: Fly secrets and `.env` (gitignored) only; gitleaks runs on every commit (NON-NEGOTIABLE) <!-- GI-004 -->
- Every request MUST be validated at the controller boundary and every route MUST sit behind the JWT guard <!-- GI-004 -->
- A posted ledger line MUST NOT be edited or deleted; a correction is a reversing entry (NON-NEGOTIABLE) <!-- GI-011 -->
- Any change under `src/ledger/` or `src/import/` MUST carry Priya's approval on the PR <!-- GI-012 -->
- Errors MUST be surfaced, never swallowed; ledger writes MUST be transactional; stack traces MUST NOT reach a client <!-- GI-006 -->
- Logs MUST exist on the ledger and import paths and MUST NOT carry tenant or landlord personal data — the redaction list is in `src/logging/redact.ts` <!-- GI-007 -->

### Technology stack

- Node 22 · NestJS 10 · TypeScript 5.5 · Prisma 5 on Postgres 16 · React 18 with Vite · pino · Sentry · Fly.io <!-- GI-002 -->

### Quality gates

- Per PR: `npm run lint`, `npm run typecheck`, `npm test` MUST pass; `npm audit` runs and is triaged weekly (non-blocking at the declared level) <!-- GI-004, GI-005 -->
- Coverage is measured nightly and MUST NOT fall below the recorded baseline (41 % at ratification) <!-- GI-005, waiver GI-013 -->
- Release gates: staging soak of one working day before promotion; migrations reversible or flagged in the PR <!-- GI-015 -->

### Governance operations

- Ledger (waivers · amendment policy · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup`; a fact-profile change or an un-waive is a governance event
- Writing style — conversation: `full` · reports: `full` · documents: `full`
- Operating docs (knowledge-management module): sessions in `.mochiko/brainstorms/` + `index.md`; rulings in `DECISIONS.md`; open threads in `BACKLOG.md`; direction in `ROADMAP.md`; releases in `CHANGELOG.md` <!-- GI-014 -->
<!-- mochiko:governance:end -->
