# Strip notes — `agents/product-manager.md`

Entry formats: `strips/README.md`.

## [v0.63.0] Frontmatter `description:` examples stripped → prose-only agent description
- **Disposition:** superseded → prose-only agent description (variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/product-manager.md`); the `<example>` blocks were removed from the frontmatter `description:` block scalar, the prose framing (routing content) kept.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark, agents-arm user ruling (b) 2026-08-10 — `DECISIONS.md` benchmark-verdict row 2026-08-10; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `report/final-verdict.md`).
- **Content:** faithfully compressed. **3 `<example>` blocks removed** from the `description:` value:
  1. Context: user stories are drafted and the built capabilities behind them need naming — commentary claimed the example demonstrated that deriving durable capabilities from stories, against the real map and never blind to it, is the product-manager's core producer work.
  2. Context: a drafted story doesn't clearly belong to any capability the product needs — commentary claimed it demonstrated that saying no to a story with stated reasoning, as a recommendation rather than a silent drop, is the product-manager's discipline.
  3. Context: more features are on the table than the team should build now — commentary claimed it demonstrated that portfolio advice with the trade-offs shown, while the selection ruling stays with the user, is the product-manager's judgment.

  Description parsed-value char delta: **2,138 → 437** (chars of the parsed block-scalar value; regex/block-scalar parse, not `wc -c` bytes). Verbatim removed text survives in three homes: (a) git history of `plugins/mochiko/agents/product-manager.md`; (b) the pre-edit original state in this tree plus the after-state variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/product-manager.md`; (c) archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately:** the prose framing of the `description:` (the routing content that staffs the agent — "Senior Product Manager who owns the product's capability layer … Recommends and gives reasons; selection is always the user's ruling. Authors the feature map; does not grade its own output.") — and the entire agent body, byte-for-byte untouched.
- **Consumers assessed:** grep of `plugins/mochiko/commands/` and `plugins/mochiko/skills/` for `product-manager`: `commands/specify.md` (staffing reference) and `skills/*/SKILL.md`. Routing/staffing contract intact — the agent name and the description's prose framing are unchanged; only the illustrative `<example>` blocks were removed (benchmark: 0 route misses over 20+ staffings).
- **Standing watch:** an F-X1-class review-evidence omission at the first live runs re-opens ruling (b).
- **Protected-content reconciliation:** no prior strip entries exist for this primitive; no `KEPT:` / protected / `DECISIONS.md`-traceable line touches the `description:` value or its `<example>` blocks. No overlap.
