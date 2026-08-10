---
paths:
  - "app/**/*.py"
  - "pyproject.toml"
  - "requirements*.txt"
  - "frontend/package.json"
  - "frontend/**/*.ts"
  - "frontend/**/*.tsx"
---

# Dependency discipline <!-- GI-009 -->

- A new dependency MUST be justified in the PR description; a dependency reasonably writable in-house at under ~100 lines SHOULD be written instead of added.
- Versions MUST be pinned and lock files (`requirements`/`pyproject` lock, `package-lock.json`) committed.
- External service calls MUST go through the port interfaces defined by the layering rule (`layers.md`) — no direct SDK use in the domain.
- `pip-audit` and `npm audit --audit-level=high` MUST report no high/critical vulnerabilities (blocking in CI).

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-009.
