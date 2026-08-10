---
paths:
  - "backend/**/*.py"
---

# Layer boundaries <!-- GI-007 -->

Pragmatic layered architecture: the domain/business logic is isolated from FastAPI routes and the
database behind a service + repository boundary. Dependencies flow inward.

## Project structure

```
backend/
├── src/
│   ├── domain/          # Entities, value objects, money math — no I/O, no framework
│   ├── services/        # Use cases / orchestration; depends on domain + repository interfaces
│   ├── repositories/    # Data-access implementations (SQLAlchemy); implement repository interfaces
│   └── api/             # FastAPI routes, request/response schemas, wiring
├── tests/
│   ├── unit/
│   └── integration/
└── pyproject.toml
```

### Layer import rules

| Layer | MAY import | MUST NOT import |
|-------|------------|-----------------|
| `domain` | standard library + approved domain deps (registry below) | `services`, `repositories`, `api`, SQLAlchemy, FastAPI |
| `services` | `domain`, repository interfaces | `api`, FastAPI, SQLAlchemy directly |
| `repositories` | `domain`, SQLAlchemy | `api`, `services` |
| `api` | `services`, `domain` (types), repository wiring | domain business logic implemented inline |

- Rules MUST be enforced by an import linter in CI (`import-linter`), not by code review.
- All external-service and database access MUST go through a repository or service interface;
  the domain layer MUST remain free of I/O.

### Domain-dependency registry

The `domain` layer MAY import third-party libraries only from the registry block below. Policy
(qualification criteria, add-process, add-gate): `.mochiko/memory/governance-ledger.md`,
Domain-dependency policy section. A proposed addition surfaces to the founder as an explicit
ruling BEFORE entering the registry.

<!-- mochiko:domain-registry:begin -->
| Dependency | Justification | Signal level | Added (by/when) | Gate |
|------------|---------------|--------------|-----------------|------|
| `pydantic` | Value objects, validation, domain modeling without I/O | 4 — ecosystem-standard (quantitative proxy) | setup seed / 2026-08-10 | seeded (session-arbitrated) |
<!-- mochiko:domain-registry:end -->

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-007.
