---
paths:
  - "ledgerline/**/*.py"
---

# Single responsibility & complexity <!-- GI-008 -->

- Each module, class, and function MUST have one clear purpose; split a component with multiple reasons to change.
- Cyclomatic complexity MUST stay ≤10 per function (ruff C901 / mccabe), enforced as a CI block.
- No "utils"/"helpers" dumping grounds — give logic a named home.
- Function length ≤60 and file length ≤500 are targets (session-tunable), warned by the linter.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-008.
