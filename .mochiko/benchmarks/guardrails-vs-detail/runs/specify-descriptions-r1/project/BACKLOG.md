# Backlog

Open items only. Closing an item moves it to `.mochiko/archive/backlog-trail.md`.

## Infrastructure

- **Stand up GitHub Actions CI** (2026-08-10) — provenance: setup session `.mochiko/memory/governance-intent.md` (GI-003/GI-004 enforcement substrate). Resume-cold: the quality gates (`ruff`, `black`, `pytest --cov --cov-fail-under=60`, `eslint`, `vitest`, `gitleaks`, `pip-audit`, `npm audit`, `lint-imports`) are authored but no pipeline runs them yet; without CI, most floor principles have no teeth for a solo dev with no reviewer. Wire this before the first paying customer.

## Open questions

- **Data-retention / deletion policy** (2026-08-10) — provenance: setup session `.mochiko/memory/governance-intent.md` (GI-022, GI-027). Resume-cold: founder cannot name her legal retention obligations for financial records; CCPA/CPRA deletion rights may apply if thresholds are crossed. Working assumption in the invoice-lifecycle spec is 7-year retention + soft-delete on account deletion — a placeholder, NOT the resolved policy. Research actual obligations BEFORE authoring any policy — do not guess. Amend governance once known.
- **`viewed`-status detection feasibility** (2026-08-10) — provenance: `.mochiko/specs/invoice-lifecycle/` (FR-010, US-4). Resume-cold: spec allows a `viewed` status when a client opens the hosted invoice link, but whether a view can be reliably detected is unconfirmed. Principal's ruling: if detection is unreliable, DROP `viewed` rather than show a status that may be wrong. Settle at plan time for FEAT-005.
- **Partial payments (post-v1)** (2026-08-10) — provenance: `.mochiko/specs/invoice-lifecycle/` (Open Questions). Resume-cold: held out of v1 (most competitor tools support it); revisit after launch. Interacts with the off-amount-payment handling ruled in FR-013 (currently held/flagged, not applied as partial).
