# Roadmap — Ledgerline

## Thesis

Ledgerline helps US solo contractors get paid: create clients, issue invoices, track payment
status against Stripe, chase the late ones. The bet is that ruthless payment-state correctness
and a low-ceremony, one-person-maintainable codebase beat feature breadth for a pre-launch
founder.

## Now / Next / Later

**Now**
- Invoice lifecycle v1 — specify then build the first feature (`/mochiko:specify`)

**Next**
- First paying customer readiness (~4 months out)

**Later**
- 200 paying contractors (year-one target)
- Staging environment (if cheap on Render) — `BACKLOG.md`
- SOC 2 readiness (only if an obligation is signed) — `BACKLOG.md`

_(Governance surface set v1.0.0 ratified 2026-08-10 — recorded in `DECISIONS.md`.)_

## Standing bets & revisit conditions

- **Render as host** — lean choice, not locked; revisit if scale or cost pressure appears.
- **Hexagonal architecture** — pay the upfront-structure cost for testable payment logic;
  revisit only if it slows the solo founder more than it protects payment correctness.
- **Coverage waiver (critical-path tests, not a %)** — revisit when the team grows past solo or
  a SOC 2 obligation is signed.

_Last groomed: 2026-08-10 · baseline: 5 open backlog items, 0 sessions, 0 features._
