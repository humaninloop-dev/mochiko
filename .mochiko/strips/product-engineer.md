# Strip notes — `agents/product-engineer.md`

Entry formats: `strips/README.md`.

## [v0.63.0] Frontmatter `description:` examples stripped → prose-only agent description
- **Disposition:** superseded → prose-only agent description (variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/product-engineer.md`); the `<example>` blocks were removed from the frontmatter `description:` block scalar, the prose framing (routing content) kept.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark, agents-arm user ruling (b) 2026-08-10 — `DECISIONS.md` benchmark-verdict row 2026-08-10; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `report/final-verdict.md`).
- **Content:** faithfully compressed. **3 `<example>` blocks removed** from the `description:` value:
  1. Context: a feature's user stories are being drafted and the experience needs to be visible before design starts — commentary claimed the example demonstrated that making a drafted experience clickable at low fidelity, story by story, is the product-engineer's core producer work.
  2. Context: the project has an existing design system and the mock should read as part of the product — commentary claimed it demonstrated that honoring an existing design system at low fidelity, without over-polishing into false precision, is the product-engineer's judgment.
  3. Context: a reviewer found flows in the mock that no story scenario covers — commentary claimed it demonstrated that keeping the prototype an honest rendering of the stories, surfacing gaps instead of inventing scope, is the product-engineer's discipline.

  Description parsed-value char delta: **2,208 → 391** (chars of the parsed block-scalar value; regex/block-scalar parse, not `wc -c` bytes). Verbatim removed text survives in three homes: (a) git history of `plugins/mochiko/agents/product-engineer.md`; (b) the pre-edit original state in this tree plus the after-state variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/product-engineer.md`; (c) archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately:** the prose framing of the `description:` (the routing content that staffs the agent — "Staff-level Product Engineer who makes intended user experiences tangible before they are built … Produces the prototype and its manifest; does not grade its own output.") — and the entire agent body, byte-for-byte untouched.
- **Consumers assessed:** grep of `plugins/mochiko/commands/` and `plugins/mochiko/skills/` for `product-engineer`: `skills/*/SKILL.md` reference(s) only; no command references the agent by name. Routing/staffing contract intact — the agent name and the description's prose framing are unchanged; only the illustrative `<example>` blocks were removed (benchmark: 0 route misses over 20+ staffings).
- **Standing watch:** an F-X1-class review-evidence omission at the first live runs re-opens ruling (b).
- **Protected-content reconciliation:** no prior strip entries exist for this primitive; no `KEPT:` / protected / `DECISIONS.md`-traceable line touches the `description:` value or its `<example>` blocks. No overlap.
