---
paths:
  - "src/**"
---

# Layered (hexagonal) architecture <!-- GI-007 -->

The application core MUST be isolated from external concerns. Dependencies flow inward — outer
layers depend on inner layers, never the reverse.

```
src/
  domain/          # business logic, entities, value objects (no I/O)
  application/     # use cases, port (interface) definitions
  adapters/        # DB repositories, Stripe adapter, webhook handlers
  infrastructure/  # DI wiring, config, FastAPI entry points
```

| Layer | Location | MAY import | MUST NOT import |
|-------|----------|------------|-----------------|
| Domain (business logic, entities, value objects) | `src/domain/` | standard library + approved domain deps | application, adapters, infrastructure |
| Application (use cases, port definitions) | `src/application/` | domain, port interfaces | adapters, infrastructure |
| Adapters (DB, Stripe, external I/O) | `src/adapters/` | application, domain, ports | other adapters directly |
| Infrastructure (DI wiring, config, entry points) | `src/infrastructure/` | all layers (wiring only) | domain logic directly |

- Stripe and PostgreSQL MUST sit behind port interfaces so payment-state and persistence logic is testable without real Stripe or a real database.
- All external-service interactions MUST go through port interfaces (one port per logical capability); domain types in signatures, not SDK types.
- Enforcement: `lint-imports` (import-linter) in CI blocks inner→outer imports and unapproved domain imports.

# Single responsibility & complexity <!-- GI-008 -->

- Each module, class, and function MUST have one clear purpose; split a component with multiple reasons to change.
- No "utils"/"helpers" dumping-ground modules — find the right home or create a named module.
- Cyclomatic complexity MUST be ≤10 per function (`ruff` C901); CI blocks on violation.
- File length (≤500 lines), parameter count (≤5), and nesting depth (≤4) are advisory (no CI gate — no reviewer to arbitrate them).

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-007 and GI-008.
