# ledger-svc

New service, no code yet. Records the per-tenant billing ledger for notify-svc usage (events
delivered, windows moved) and exposes it to ShopLoop's invoicing.

- **Stack (decided):** Python 3.12 · FastAPI · PostgreSQL 16 · GitHub Actions · deployed on
  the ShopLoop platform (Kubernetes), secrets in the platform vault.
- **Team:** two engineers, starting next sprint. First production release targeted for Q4.
- **Governance:** the `## Governance` region of `CLAUDE.md` is empty; `.claude/rules/` is
  empty. The ratified intent is at `.mochiko/memory/governance-intent.md`.
