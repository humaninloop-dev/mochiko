---
paths:
  - "ledgerline/**/*.py"
  - "pyproject.toml"
  - "requirements*.txt"
  - "poetry.lock"
---

# Dependency discipline <!-- GI-009 -->

- New third-party dependencies MUST be justified in the PR description; versions MUST be pinned in the lock file.
- `pip-audit` MUST run in CI and blocks merge on high/critical vulnerabilities.
- External service calls MUST go through port interfaces (pairs with the architecture layers rule), not direct SDK use in domain code.
- Prefer the standard library and existing dependencies before adding a new one.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-009.
