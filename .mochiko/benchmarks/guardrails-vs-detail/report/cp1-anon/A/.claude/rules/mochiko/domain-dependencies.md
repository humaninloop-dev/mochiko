---
paths:
  - "src/domain/**"
---

# Approved domain dependencies <!-- GI-018 -->

The domain layer MAY import a third-party library only when it meets BOTH qualification criteria:
**domain-relevance** (domain modeling without I/O — value objects, validation, precise
arithmetic, immutable collections) and **ubiquity** (effectively an ecosystem standard, >80%
adoption). Domain-relevance filters the candidate pool first; trust signals rank what passes.
A library performing I/O, talking to a platform, or rendering UI is inadmissible regardless of
trust.

**Add-process (the gate):** a proposed addition meeting both criteria surfaces to the human as
an explicit ruling BEFORE entering the registry; the implementation checkpoint MUST NOT
auto-approve while a cycle's `domain_deps_added` is non-empty. Full policy, trust-signal
hierarchy, and criteria: the Domain-dependency policy section of `.mochiko/memory/governance-ledger.md`.

<!-- mochiko:domain-registry:begin -->
| Dependency | Justification | Signal level | Added (by/when) | Gate |
|------------|---------------|--------------|-----------------|------|
| `pydantic` | Validation / value objects for the domain; ecosystem-standard on PyPI | 4 — quantitative (verified publisher, sustained adoption), live-verified 2026-08-10 | founder / 2026-08-10 (setup seed) | seed |
| `zod` | Runtime validation / value objects for TypeScript domain code | 4 — quantitative (sustained adoption), live-verified 2026-08-10 | founder / 2026-08-10 (setup seed) | seed |
<!-- mochiko:domain-registry:end -->

*Money uses integer cents (GI-011); Python's stdlib `Decimal` may be used where helpful and needs
no registry entry. No third-party decimal library is seeded.*

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-018.
