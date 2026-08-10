# Strip notes — `agents/devils-advocate.md`

Entry formats: `strips/README.md`. Wave context: the command-waves' ≥3-consumer escalation queue
(D9's guard), ruled library-wide at the skill-succinctness pass's wave-1 open (R4b,
user-approved 2026-07-25; design: `.mochiko/brainstorms/skill-succinctness-strip/record.md`).
This was the sole remaining agent instance — the other five agents' Skills-Available paragraphs
were ruled in their command waves.

## [v0.63.0] Frontmatter `description:` examples stripped → prose-only agent description
- **Disposition:** superseded → prose-only agent description (variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/devils-advocate.md`); the `<example>` blocks were removed from the frontmatter `description:` block scalar, the prose framing (routing content) kept.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark, agents-arm user ruling (b) 2026-08-10 — `DECISIONS.md` benchmark-verdict row 2026-08-10; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `report/final-verdict.md`).
- **Content:** faithfully compressed. **2 `<example>` blocks removed** from the `description:` value:
  1. Context: a drafted feature specification needs an adversarial gap review — commentary claimed the example demonstrated that a spec-review request triggers adversarial review of requirements completeness with a verdict.
  2. Context: a reviewer is needed to pressure-test requirements and produce an evidence-backed verdict — commentary claimed it demonstrated that a readiness question triggers a structured adversarial review that returns a verdict, never a rubber-stamp.

  Description parsed-value char delta: **1,301 → 315** (chars of the parsed block-scalar value; regex/block-scalar parse, not `wc -c` bytes). Verbatim removed text survives in three homes: (a) git history of `plugins/mochiko/agents/devils-advocate.md`; (b) the pre-edit original state in this tree plus the after-state variant at `.mochiko/benchmarks/guardrails-vs-detail/variants/agents/devils-advocate.md`; (c) archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately:** the prose framing of the `description:` (the routing content that staffs the agent — "Adversarial reviewer who stress-tests specifications by finding gaps, challenging assumptions … returns a severity-ranked gap report with clarifying questions and a recommended verdict.") — and the entire agent body, byte-for-byte untouched.
- **Consumers assessed:** grep of `plugins/mochiko/commands/` and `plugins/mochiko/skills/` for `devils-advocate`: `skills/*/SKILL.md` reference(s) only; no command references the agent by name. Routing/staffing contract intact — the agent name and the description's prose framing are unchanged; only the illustrative `<example>` blocks were removed (benchmark: 0 route misses over 20+ staffings).
- **Standing watch:** an F-X1-class review-evidence omission at the first live runs re-opens ruling (b).
- **Protected-content reconciliation:** the prior entries touch the `skills:` frontmatter list and body sections only — [v0.49.0] roster drop (frontmatter `skills:` + Skills-Available bullets), [v0.25.0] "Skills Available" paragraphs (body), [v0.25.0] "What You Hunt For" catalog bullets (body). None touches the frontmatter `description:` value or any `<example>` block. No overlap.

## [v0.49.0] Roster drops review-task-artifacts + review-slices
- **Disposition:** superseded → the two absorbing skills already on the roster (review-plan-artifacts, review-specifications)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D4+D9)
- **Content:** frontmatter `skills:` entries `review-task-artifacts`, `review-slices` + their two Skills-Available bullets ("completeness review of `task-mapping.md` / `tasks.md`", "completeness review of the `slices.md` decomposition overlay").
- **Consumers assessed:** router agent row (re-keyed same wave) · both dispatching commands.

## [v0.25.0] Six per-mount "Skills Available" paragraphs → one-liners (ruled precedent form)
- **Disposition:** relocated → each mounted skill's own `description:` (the declared single source); the agent keeps the ruled precedent form (task-architect et al.): a single-source framing + one routing line per mount
- **Tier failed:** 1 (each paragraph restated its mounted skill's description — the review-brainstorm and review-governance-intent instances at near-full length)
- **Content:** six paragraphs (~30 lines) summarizing review-specifications / review-plan-artifacts / review-task-artifacts / review-brainstorm / review-slices / review-governance-intent scope, severity classification, and verdict/status formats
- **Consumers assessed:** all six mounted skills' delivered descriptions verified live same day (R1 measurement pass); the agent file is the only consuming surface

## [v0.25.0] "What You Hunt For" catalog bullets → category names + the existing pointer
- **Disposition:** relocated → `review-specifications`' Gap Categories section. **Audit catch (wave-1 audit, 2026-07-25):** the home initially held only the question-framing taxonomy, not the five defect classes — the surviving pointer line had been dishonest since before this wave; the five-class table landed in Gap Categories at fix time, making the relocation (and the pointer) true
- **Tier failed:** 1 (persona keeps the five hunt-category names — what the agent cares about; the per-category bullets were the skill's catalog copied, contradicting the file's own single-source line)
- **Content:** 3–4 example bullets under each of Missing Requirements / Ambiguities / Edge Cases / Assumption Gaps / Contradictions and Conflicts (~20 lines)
- **Consumers assessed:** `review-specifications` untouched; the agent file is the only consuming surface
