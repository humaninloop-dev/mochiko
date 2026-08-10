# Backlog — Ledgerline

Open items only. Closing an item moves it to `.mochiko/archive/backlog-trail.md` (never deleted).

## Governance — deferred / revisit

- **Observability depth** — 2026-08-10 — provenance: governance-intent GI-012 (waiver). Structured-log tooling, correlation/trace IDs beyond the money-path request ID, APM/dashboards, SLOs. Waived pre-launch (solo, ops-fuzzy). Resume-cold: revisit at launch or first incident; the `/health` + no-PII-in-logs essentials are already live (GI-006).
- **Data retention & deletion policy** — 2026-08-10 — provenance: governance-intent GI-020 (exclusion). Suspected statutory retention on invoices; founder cannot name the rule. Resume-cold: revisit post-launch with legal input; PII is in scope so this is a real compliance thread, not cosmetic.
- **SOC 2 attestation** — 2026-08-10 — provenance: governance-intent GI-021 (exclusion) + GI-001 (fact negative). No obligation today; a prospect mentioned it. Resume-cold: attach the `attestation` module via amend if a signed customer contract requires SOC 2.

## Infrastructure

- **Staging environment** — 2026-08-10 — provenance: governance-intent (dim 8, ratification note). Dev + prod only pre-launch. Resume-cold: add staging if it earns its keep once there are real users.

## Product — deferred scope

- **Teams / bookkeeper seat / multi-user** — 2026-08-10 — provenance: governance-intent GI-022 (exclusion). Single-tenant-per-account only for now. Resume-cold: deferred design scope; revisit post-launch. (When built, this becomes a feature on FEATURES.md, not a backlog item.)
