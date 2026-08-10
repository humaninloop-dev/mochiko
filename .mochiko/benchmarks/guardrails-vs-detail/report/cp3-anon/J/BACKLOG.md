# Backlog

> Open items only. One bounded entry per item. Closed items move to
> `.mochiko/archive/backlog-trail.md`. Product capabilities live on `FEATURES.md`, not here.

## Compliance & data

### Data-retention / deletion policy (pre-launch, blocking launch)
- **Opened:** 2026-08-10 · **Provenance:** governance-intent.md GI-021, cold-review F6
- The founder does not yet know legal obligations for invoice retention duration or
  account-deletion behavior. Resolve BEFORE first paying customer.
- MUST reconcile with GI-012 durable backups: "delete my account" vs backup retention windows —
  decided together, not separately.
- Not a gate today; not a floor waiver. A named open question.

### WCAG 2.1 AA manual accessibility audit (residual gap)
- **Opened:** 2026-08-10 · **Provenance:** governance-intent.md GI-010, cold-review F3
- Automated axe checks in CI are the day-one enforcement floor; they cannot verify full AA.
- The manual-audit gap is tracked here as an open a11y obligation (legal-mandate, unwaivable) —
  NOT a launch gate per the founder's ratification, but must not be treated as closed.

## Tooling

### mypy strictness ratchet
- **Opened:** 2026-08-10 · **Provenance:** governance-intent.md GI-008, cold-review F8
- Start lenient; tighten strictness over time. Track the ratchet steps here.
