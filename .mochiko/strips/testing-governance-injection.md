# Strip notes — `skills/testing-governance-injection/`

Entry formats: `strips/README.md`. Wave context: skill-succinctness wave 1 (design:
`.mochiko/brainstorms/skill-succinctness-strip/record.md`, batch-ratified 2026-07-25): body
99 → 96 lines, 3 cut = 3% — **deeply under the 30–70 never-stripped band by ruling**: the skill
was authored 2026-07-19 under current conventions and the tiers condemn essentially nothing;
D1 forbids cutting to reach a band. The KEPT entry below is the under-band pass's product.

## [v0.63.0] Guardrails cut — body deletions + slim description (benchmark verdict)
- **Disposition:** superseded → benchmark-ruled guardrails body + slim description (`.mochiko/benchmarks/guardrails-vs-detail/variants/`)
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark verdict — `DECISIONS.md` 2026-08-10 benchmark-verdict row; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `.mochiko/benchmarks/guardrails-vs-detail/report/final-verdict.md`)
- **Content (faithfully compressed).** Body 5,798 → 3,540 chars (−2,258, −39%). Description 1,037 → 483 chars. Sections removed:
  - **## When to Use** (the three-bullet list) — removed; When NOT to Use kept.
  - **## Build the probe plan (before spawning anything)** — the three-step probe-plan procedure, including the quantified default bounds (one introspection probe per distinct glob set + one negative control + ≤2 behavioral probes) and the throwaway-stub marking convention.
  - **## The two probe types** — the introspection-probe (cheap model, raw-context-only, Write-then-Read) and behavioral-probe (capable model, ≤2, zero-governance-mention) definitions.
  - Old description verbatim: "This skill MUST be invoked when empirically probing that an accepted governance surface set actually DELIVERS — verifying that `.claude/rules/mochiko/` files inject into agent context on the paths their `paths` frontmatter promises (and nowhere else), that the CLAUDE.md governance region reaches spawned agents, and that injected rules actually change behavior — via throwaway file stubs and disposable probe subagents, with unconditional stub cleanup. SHOULD also invoke when the work is a "governance injection probe", "injection smoke test", "verify rules injection", "probe rule delivery", building a rules-file "trigger matrix", or a post-scaffold regression check of rules delivery. Offered at a `/mochiko:setup` run's finalize; re-runnable standalone any time after (real files replace stubs as probe surfaces). Findings are observed harness behavior versioned to the run — input to an amend run, never auto-fixed. Empirical delivery testing only — static structure/trace grading is `validation-constitution`, not this skill."
  - Verbatim removed text survives in three places: (a) git history of the original `plugins/mochiko/skills/testing-governance-injection/SKILL.md`; (b) the before/after pair in this tree — `.mochiko/benchmarks/guardrails-vs-detail/variants/body/testing-governance-injection/SKILL.md` (after) and the pre-edit original (before, in git); (c) archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (the guardrails keep-set):** goal/output contract (Overview + Report routing); the unconditional-cleanup floor with `git status` verification; the versioned-harness-behavior-never-doctrine floor; both-matrix-directions findings (under- and over-injection) with the Read-not-Write known behavior; the Common Mistakes table (which retains the contaminated-lead, Write-first, zero-expectations, negative-control, leftover-stub, and version-stamp guards). No floor line added to this skill (not a floor-line home).
- **Protected-content reconciliation.** The `[v0.25.0] KEPT: the entire remaining body` whole-skill survivor ruling named several behaviors as kept; this guardrails cut REMOVES two, recorded here as superseded-by-this-ruling:
  - the quantified probe-plan bounds ("one introspection probe per glob set + negative control + ≤2 behavioral") — REMOVED with ## Build the probe plan. Superseded. (The negative-control *obligation* itself survives as a Common Mistakes row; the per-glob-set / ≤2 *quantification* does not.)
  - the two-probe-types definitions (introspection vs behavioral) — REMOVED. Superseded.
  The other v0.25.0-KEPT behaviors survive: the contaminated-lead rule, the zero-expectations brief, both-matrix-directions findings, unconditional cleanup with git-status verification, and version-stamped findings (in Overview / matrix / cleanup / Common Mistakes).
- **Consumers assessed:** no command references this skill and no agent declares it in `skills:` (grep clean). It is offered at `/mochiko:setup` finalize and re-runnable standalone; the kept goal/floors/routing leave that invocation intact.

## [v0.25.0] Overview kinako-defect narrative trimmed (4 lines)
- **Disposition:** relocated → the matrix section (in-file), which owns the operative known-behavior check (Read-not-Write, kinako 2026-07-19); the Overview keeps the one-line motivation with the dogfood citation
- **Tier failed:** 1 (the defect detail was stated in both places; the token/duration figures were provenance)
- **Content:** the two-defect enumeration (Read-not-Write injection; `paths` scoped to the mechanism's home layer while an orchestrating layer wrote blind) + ~220k-token/~3-min run figures
- **Consumers assessed:** 2 consumer files checked at wave open; none reference the narrative

## [v0.25.0] KEPT: the entire remaining body (whole-skill survivor ruling)
- **Tier-2 evidence:** contested as a whole at the under-band pass and kept — every section names
  behavior or cites dogfood evidence: probe-plan bounds are quantified (one introspection probe
  per glob set + negative control + ≤2 behavioral), the contaminated-lead rule names its false
  positive, the zero-expectations brief names its echo failure, both matrix directions are
  findings (under- and over-injection), cleanup is unconditional with a named verification, and
  findings are version-stamped never doctrine. Common Mistakes is already a table. Session
  ruling: batch-2 ratification 2026-07-25.
