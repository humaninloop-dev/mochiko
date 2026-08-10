---
paths:
  - "backend/**/*.py"
  - "backend/**/domain/**"
  - "backend/**/adapters/**"
---

# Layered architecture — hexagonal, two-seam <!-- GI-007, GI-016 -->

## Project structure

```
ledgerline/
├── backend/
│   ├── domain/          # invoicing/payment logic — no Stripe SDK, no DB/ORM imports
│   │   └── ports/       # interfaces for the Stripe and DB seams
│   ├── adapters/        # Stripe + PostgreSQL adapters implementing the ports
│   └── web/             # FastAPI routes/handlers
├── frontend/            # React UI
├── tests/
│   ├── unit/
│   └── integration/
└── pyproject.toml
```

Ports-and-adapters scoped to the two seams that must stay swappable and testable without live
infrastructure: **Stripe** and the **database**. No multi-layer import ceremony beyond these
seams (the founder's explicit "no ceremony" tightening of BE-HEX).

- The domain / invoicing logic MUST NOT import Stripe SDK types or database/ORM types directly;
  it depends on a port (interface) for each seam.
- Stripe and PostgreSQL access MUST live behind an adapter implementing the port; swapping the
  provider (Render→elsewhere, or a Stripe test double) MUST NOT touch domain code.
- Tests for invoicing/payment logic MUST run against port fakes — no live Stripe or DB required.
- Enforcement: `import-linter` contracts covering the domain→Stripe and domain→DB boundaries,
  blocking in CI. No import rules on the web layer or elsewhere.

| Layer | MAY import | MUST NOT import |
|-------|------------|-----------------|
| domain (invoicing logic) | standard library, registry deps below | Stripe SDK, database/ORM drivers |
| adapters (Stripe, DB) | domain ports, provider SDKs | — |
| web/API | domain, adapters | — (no import-linter constraint here) |

## Domain-Dependency Registry <!-- GI-017 -->

The domain layer MAY import libraries in the registry block below — seeded at setup
(session-arbitrated), grown at implement time under the human-gated add-process. Craft and policy
single source: the governance ledger's Domain-dependency policy section.

**Policy preamble** (setup-owned, regenerated): a candidate qualifies by domain-relevance first
(models/validates the invoicing domain without I/O), then ubiquity (ecosystem-standard, high
adoption). Adding a row requires a human ruling before the entry lands; implement-time additions
must be disclosed in the cycle report (`domain_deps_added`) and the checkpoint never auto-approves
while such an addition is pending.

<!-- mochiko:domain-registry:begin -->
| Dependency | Justification | Signal level | Added (by/when) | Gate |
|------------|---------------|--------------|-----------------|------|
| `pydantic` | FastAPI-native domain modeling/validation without I/O; ecosystem standard (>80% adoption) | 1 — ecosystem standard | setup / 2026-08-10 | session-arbitrated seed |
<!-- mochiko:domain-registry:end -->

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-007 / GI-016 / GI-017.
