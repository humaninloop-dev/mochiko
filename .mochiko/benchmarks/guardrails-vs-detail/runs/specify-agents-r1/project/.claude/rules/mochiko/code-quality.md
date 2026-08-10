---
paths:
  - "app/**/*.py"
---

# Code quality — single responsibility <!-- GI-008 -->

- Each module, class, and function SHOULD have one clear purpose; there MUST be no "utils"/"helpers" dumping ground — code lives in a named module or a new one is created.
- Cyclomatic complexity ≤10 per function and the function-length budget are emitted by ruff as **warnings** (session-tunable), not merge-blocking (session ruling GI-008, `Contested` — a solo, no-reviewer codebase; warnings are read, a hard gate is not wanted). The no-dumping-ground rule above is firm.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-008.
