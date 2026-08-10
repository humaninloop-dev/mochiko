---
paths:
  - "ledgerline/**/*.py"
---

# Architecture layers (hexagonal, light) <!-- GI-007 -->

Ledgerline uses a light ports-and-adapters shape: the money/domain logic is isolated so it can be
tested against fakes with no network. The full four-layer ceremony is deliberately not adopted.

## Project structure

```
ledgerline/
├── domain/          # invoices, payments, tax — pure logic, no I/O, no SDK types
├── application/     # use cases; defines ports (Protocols)
├── adapters/        # Stripe, database, email — implement ports
└── infrastructure/  # FastAPI wiring, config, entry points
```

## Layer import rules

| Layer | MAY import | MUST NOT import |
|-------|------------|-----------------|
| domain | standard library + approved domain deps (registry below) | application, adapters, infrastructure |
| application | domain, port interfaces | adapters, infrastructure |
| adapters | application, domain, ports | other adapters directly |
| infrastructure | all layers (wiring only) | domain logic directly |

- All external systems (Stripe, DB, email) MUST be reached through a port (`Protocol`), one port per capability; domain types in port signatures, never SDK types.

## Domain-dependency registry

Policy: a domain-layer dependency MUST pass domain-relevance (modeling without I/O) AND ubiquity
(ecosystem standard). New additions require an explicit human ruling before entering this block;
the growth process and trust hierarchy live in the ledger's Domain-dependency policy section.

<!-- mochiko:domain-registry:begin -->
| Dependency | Justification | Signal level | Added (by/when) | Gate |
|------------|---------------|--------------|-----------------|------|
| `pydantic` | domain modeling + boundary validation, value objects | 4 (Python ecosystem standard, >80% adoption) | setup / 2026-08-10 | seed (synthesis GI-018) |
<!-- mochiko:domain-registry:end -->

(Exact money math uses the standard-library `decimal.Decimal` — stdlib, no registry entry needed.)

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-007.
