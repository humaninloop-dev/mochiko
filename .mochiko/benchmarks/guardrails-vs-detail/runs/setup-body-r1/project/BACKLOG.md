# Backlog — Ledgerline

Open items only. Closing an item moves it to `.mochiko/archive/backlog-trail.md`. Product
capabilities live on `FEATURES.md`, not here.

## Compliance & legal

- **Data retention & deletion policy** — 2026-08-10 · provenance: governance-intent.md GI-022 ·
  Source the US financial-record retention requirement (and any state deletion obligation) for
  invoice data; author a policy before it bites. NOT a launch gate; open question the founder
  must resolve. Revisit trigger: before scaling / when the requirement is sourced.
- **SOC 2 readiness** — 2026-08-10 · provenance: governance-intent.md GI-001 watch · A prospect
  raised SOC 2; no obligation signed. If one is signed, an amend run attaches the `attestation`
  module. No work until then.

## Ops & tooling

- **RUNBOOK** — 2026-08-10 · provenance: governance-intent.md GI-023 · Deferred until after first
  deploy, when there is something real to write down. Re-offered at the amend run post-deploy.
- **Staging environment** — 2026-08-10 · provenance: governance-intent.md GI-017 / release-gates ·
  Prod-only at launch; add a staging env if it proves cheap on Render. Auto-rollback likewise
  deferred past launch unless effectively free on Render.
- **Governance-injection probe** — 2026-08-10 · provenance: setup finalize offer
  (`mochiko:testing-governance-injection`) · Run an empirical rules-delivery probe once the repo
  is the harness's active root (fresh subagent, Write-then-Read a stub under each governed path,
  confirm `.claude/rules/mochiko/` files inject as their `paths` promise and the region reaches
  spawns). Re-runnable standalone; findings route to an amend run.
