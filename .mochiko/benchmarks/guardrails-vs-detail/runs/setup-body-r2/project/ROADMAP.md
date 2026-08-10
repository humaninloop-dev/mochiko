# Roadmap

## Thesis

Ledgerline is production-quality invoicing + payment tracking for solo US contractors, built to
hold real financial data safely from day one. The bet: a one-person shop can ship a trustworthy
financial product if governance is automated (the machine is the reviewer) and scoped to what
actually protects the money and the data.

## Now / Next / Later

**Now**
- Governance established (v1.0.0) — see [DECISIONS.md](DECISIONS.md) · 2026-08-10

**Next**
- Resolve the data-retention / deletion policy before launch — see [BACKLOG.md](BACKLOG.md) · 2026-08-10
- Spec the Invoice lifecycle v1 feature (`/mochiko:specify`)

**Later**
- WCAG 2.1 AA manual accessibility audit (beyond automated axe) — see [BACKLOG.md](BACKLOG.md)
- Bookkeeper-seat capability (deliberately deferred, not foreclosed)
- SOC 2 posture — only if a customer contract requires it

## Standing bets & revisit conditions

- **Two-seam hexagonal only** (Stripe + DB) — revisit if a third integration needs the same
  swappability, or if the web layer starts leaking domain logic.
- **Solo-operable gates** — revisit if the team grows past one full-time engineer (a reviewer
  becomes available; gate posture can change).
- **No GDPR now** — revisit on going international or onboarding contractors who bill EU/UK clients.

_Last groomed: 2026-08-10 · baseline: 1 Now / 2 Next / 3 Later · 2 open backlog items._
