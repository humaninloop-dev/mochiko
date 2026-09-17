# Beacon

Uptime monitoring for small teams. `npm run dev` starts the API, the checker, and Postgres.

<!-- mochiko:governance:begin -->
## Governance

**Ratified:** v1.0.0 · 2026-08-02 · production floor · depth: high · modules: none <!-- GI-001 (fact profile) · GI-002 (depth level) -->

### Principles

- An incident MUST never be closed automatically while a probe is still failing (NON-NEGOTIABLE) <!-- GI-003 -->
- Channel secrets (webhook URLs, tokens) MUST NOT appear in logs, timelines, or error messages <!-- GI-004 -->
- Alert delivery — see `.claude/rules/mochiko/alerting.md` <!-- GI-005 -->

### Technology stack

- Node 20 · Fastify · Postgres 15 · node-cron in the checker · undici for outbound HTTP <!-- GI-006 -->

### Quality gates

- `npm test` MUST pass before merge; the checker's probe suite runs against a local target, never a public one <!-- GI-007 -->

### Governance operations

- Ledger (waivers · amendment policy · exceptions · Three-Part metadata): `.mochiko/memory/governance-ledger.md`
- Amend via `/mochiko:setup` (fact-profile changes — module attach/detach — and un-waives are governance events)
<!-- mochiko:output-style:begin -->
- Writing style — conversation: `full` · reports: `ultra` *(internal agent hand-offs)* · documents: `full`. Terse and plain-English by default. Set any of the three to `off`, `lite`, `full`, or `ultra` and your choice is kept when this section is regenerated; "normal mode" turns it off for one session.
<!-- mochiko:output-style:end -->
<!-- mochiko:governance:end -->

## Notes for contributors

- The checker and the API share one Prisma schema; run migrations from the API package.
- Never point a probe at a customer's endpoint from a test. Use the local target in
  `test/target/`.
- We declined the operating-docs module at setup; decisions go in the PR description.
