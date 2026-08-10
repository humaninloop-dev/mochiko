---
paths:
  - "app/**/*.py"
---

# Hexagonal layering (ports & adapters) <!-- GI-007 -->

Dependencies flow inward. The domain core is isolated behind ports so invoice/payment logic and
tenant-scoping are unit-testable without Postgres or real Stripe.

- The domain layer (`app/domain/`) MUST NOT import the Stripe SDK, database drivers, or any adapter/infrastructure module. External collaborators are reached only through port interfaces — at minimum a **Stripe port** and a **repository port**. This rule is **blocking** in CI (`lint-imports`).
- Application use cases (`app/application/`) MAY import the domain and port interfaces; they MUST NOT import adapters or infrastructure directly.
- Other inward-dependency / layer-boundary violations beyond the load-bearing seam above are emitted as **warnings** (session ruling GI-007, `Contested`) — advisory, not merge-blocking.
- Adapters implement ports; one port per logical capability, domain types in signatures (not SDK types).

The domain layer MAY import only third-party libraries listed in the registry block below.

<!-- mochiko:domain-registry:begin -->
## Approved domain dependencies

Qualification: domain-relevance (modeling without I/O) filters first, then ubiquity; each entry
cites its trust-signal level. Additions require a human ruling BEFORE entering this block
(`domain_deps_added` cycle-report disclosure; the checkpoint never auto-approves while it is
non-empty). Policy: `.mochiko/memory/governance-ledger.md` → Domain-dependency policy.

| Dependency | Justification | Signal level | Added (by/when) | Gate |
|------------|---------------|--------------|-----------------|------|
| pydantic | Validation + value objects, no I/O; ecosystem-standard on FastAPI stacks | 4 — quantitative proxy + criteria | setup / 2026-08-10 | ruled in session (GI-018) |

*(Money arithmetic uses stdlib `decimal.Decimal` — no dependency required. `attrs` was considered and dropped in session: redundant with pydantic, GI-019.)*
<!-- mochiko:domain-registry:end -->

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-007.
