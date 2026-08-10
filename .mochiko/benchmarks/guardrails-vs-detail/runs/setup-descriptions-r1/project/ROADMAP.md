# Roadmap — Ledgerline

## Thesis

Ledgerline is invoicing and payment tracking that US independent contractors can trust with their
income. The core bet: get the money path unimpeachably correct (invoices, tax, payment status
reconciled with Stripe) and keep everything else lean enough for one founder to maintain.

## Now / Next / Later

**Now**
- Governance established (v0.1.0) — see [DECISIONS.md](DECISIONS.md) 2026-08-10.

**Next**
- First `/mochiko:specify` — the "Invoice lifecycle v1" feature (invoices, payment status, reminders).

**Later**
- Observability depth (structured logging, correlation IDs, APM, SLOs) — waived pre-launch (ledger GI-012); revisit at launch. — see [BACKLOG.md](BACKLOG.md)
- Data retention & deletion policy (suspected statutory retention on invoices) — revisit with legal input post-launch. — see [BACKLOG.md](BACKLOG.md)
- Staging environment — add if it earns its keep post-launch. — see [BACKLOG.md](BACKLOG.md)
- SOC 2 attestation — only if a customer contract requires it. — see [BACKLOG.md](BACKLOG.md)
- Teams / bookkeeper seat / multi-user — deferred design scope. — see [BACKLOG.md](BACKLOG.md)
- Frontend accessibility per-screen criteria — mint as real screens exist (the WCAG 2.1 AA floor + CI check is already live, GI-024).

## Standing bets & revisit conditions

- **Lean, automated-only governance** (2026-08-10) — no control requires a second human; every gate is CI-automated. Revisit if the team grows past the solo founder.
- **Managed host, not self-run ops** (2026-08-10) — Render or Railway, not yet locked. Revisit at launch when a host is chosen.

_Last groomed: 2026-08-10 · baseline: 5 open backlog items, 1 Now / 1 Next / 6 Later._
