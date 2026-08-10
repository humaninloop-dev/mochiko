# Strip notes — `agents/principal-architect`

Entry formats: `strips/README.md`. Wave context: the plan cluster wave (v0.15.0). Shared agent,
**2 consumers** (`setup` producer + `plan` feasibility reviewer) — under the D9 3-consumer threshold,
ruled in-wave; the strip below is Tier-2-tested against both consumers (the `task-architect` precedent
for the 2-consumer allowance).

## [v0.63.0] Frontmatter `description:` examples stripped → prose-only agent description
- **Disposition:** superseded → prose-only agent description (variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/principal-architect.md`); the `<example>` blocks were removed from the frontmatter `description:` block scalar, the prose framing (routing content) kept.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark, agents-arm user ruling (b) 2026-08-10 — `DECISIONS.md` benchmark-verdict row 2026-08-10; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `report/final-verdict.md`).
- **Content:** faithfully compressed. **3 `<example>` blocks removed** from the `description:` value:
  1. Context: starting a new project, governance principles must be established — commentary claimed the example demonstrated that greenfield governance establishment is the principal-architect's core responsibility.
  2. Context: technical artifacts exist and must be verified buildable together — commentary claimed it demonstrated that cross-artifact feasibility review catches impossible combinations no single artifact reveals in isolation.
  3. Context: an existing codebase's patterns must be codified into governance — commentary claimed it demonstrated that brownfield governance requires understanding existing patterns before imposing new standards (the brownfield path lives in authoring-constitution's brownfield branch).

  Description parsed-value char delta: **2,353 → 737** (chars of the parsed block-scalar value; regex/block-scalar parse, not `wc -c` bytes). Verbatim removed text survives in three homes: (a) git history of `plugins/mochiko/agents/principal-architect.md`; (b) the pre-edit original state in this tree plus the after-state variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/principal-architect.md`; (c) archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately:** the prose framing of the `description:` (the routing content that staffs the agent — the governance-standards + cross-artifact-feasibility framing, greenfield/brownfield authoring, codebase analysis) — and the entire agent body, byte-for-byte untouched.
- **Consumers assessed:** grep of `plugins/mochiko/commands/` and `plugins/mochiko/skills/` for `principal-architect`: `skills/*/SKILL.md` reference(s) only; no command references the agent by name. Routing/staffing contract intact — the agent name and the description's prose framing are unchanged; only the illustrative `<example>` blocks were removed (benchmark: 0 route misses over 20+ staffings).
- **Standing watch:** an F-X1-class review-evidence omission at the first live runs re-opens ruling (b).
- **Protected-content reconciliation:** the two prior [v0.15.0] entries touch the **body** only — the "Skills Available" scope duplication (relocated to the three skills' `description:` fields) and the `KEPT:` persona sections (Three-Part Rule + Essential Floor Knowledge). Neither touches the frontmatter `description:` value or its `<example>` blocks. The KEPT persona survivors and the agent body are untouched by this edit. No overlap.

## [v0.15.0] "Skills Available" scope duplication
- **Disposition:** scope enumeration relocated → the three skills themselves
  (`mochiko:authoring-constitution`, `mochiko:analysis-codebase`, `mochiko:review-feasibility`), each
  of whose `description:` single-sources its scope; the skill **names + a one-line reach-for-it hint
  are kept** (the team-form function — teammates ignore `skills:` frontmatter; the load-bearing
  "greenfield-or-brownfield-branch, no separate brownfield skill" and "never the constitution" nuances
  are preserved in the hints)
- **Tier failed:** 1 (a second home for each skill's scope)
- **Content:** the three full scope paragraphs in the "Skills Available" bullets — the
  `authoring-constitution` paragraph ("Write governance principles with enforcement, testability, and
  rationale — formulating a ratified statement … three-part principles … greenfield … brownfield
  branch — there is no separate brownfield skill"), the `analysis-codebase` paragraph ("Analyze
  existing codebases for patterns, architecture, and essential-floor status …"), and the
  `review-feasibility` paragraph ("Adversarially hunt cross-artifact contradictions, impossibilities,
  and buildability conflicts … 3-state `feasible / needs-revision / infeasible` verdict … operates
  over those artifacts, never the constitution.").
- **Consumers assessed:** **setup + plan (both).** Setup (producer seat): `authoring-constitution` +
  `analysis-codebase` scopes are single-sourced in those skills' `description:` fields — a teammate
  spawned as the constitution producer learns its skills from the in-body names (teammates ignore
  `skills:` frontmatter) and reaches the full scope in the skill; the strip holds. Plan (feasibility
  reviewer seat): `review-feasibility`'s scope is single-sourced in its `description:` — a teammate
  spawned as the feasibility reviewer learns the skill name + hint and reaches the six-class procedure
  in the skill; the strip holds. One instance of the 7-agent library-wide "Skills Available" pattern;
  ruling in-wave is D9-authorized (2 consumers), consistent with the `task-architect` 2-consumer
  ruling (v0.14.0).

## [v0.15.0] KEPT: the "Three-Part Rule" + "Essential Floor Knowledge" persona sections
- **Tier-2 evidence:** persona altitude (what the architect cares about — its judgment lens), with
  explicit single-source references: the four Essential-Floor categories are named as persona, their
  canonical definitions deferred ("lives in `authoring-constitution`'s `references/ESSENTIAL-FLOOR.md`
  … rather than working from a copy in this persona"); the Three-Part Rule is stated as a value
  ("Without all three, reject it or fix it"), not the ledger's GI-keyed Three-Part metadata *schema*,
  which lives in `authoring-constitution`. Tested against both consumers — load-bearing persona for
  setup (constitution producer), inert-but-harmless persona for plan (feasibility reviewer). Matches
  the `task-architect` persona keep; distinct from the "Skills Available" scope catalog stripped above.
