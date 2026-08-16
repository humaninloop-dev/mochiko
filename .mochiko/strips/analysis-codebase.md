# Strip notes — `skills/analysis-codebase/`

Entry formats: `strips/README.md`. Wave context: skill-succinctness pilot wave (R6; design:
`.mochiko/brainstorms/skill-succinctness-strip/record.md`, doctrine D1–D5 + batch R1–R7; per-hunk
proposals user-ratified 2026-07-25). Baseline 313 file / 309 body lines at v0.23.0 → 175 file /
171 body lines at v0.24.0 (138 body lines cut, 45%). Landed in two passes: the initial land
dropped the Manual-Detection deletion (caught as a phantom strip entry by the independent audit,
fixed same day — the audit trail is the wave's ROADMAP row). Consumers assessed for every entry below:
`commands/setup.md`, `agents/principal-architect.md`, router `skills/mochiko/SKILL.md`,
`skills/authoring-constitution/` (+ its `references/ESSENTIAL-FLOOR.md`),
`templates/artifact-format.md` — grep 2026-07-25: none reference any stripped section;
`detect-stack.sh` references (setup.md, router) untouched.

## [v0.74.0] `codebase-analysis-template.md` read-pointers → `codebase-analysis` schema (two-arm) — schema-based-template-guidance D1/D8
- **Disposition:** superseded → `mochiko-cli template codebase-analysis`, or Read `plugins/mochiko/schemas/codebase-analysis.yaml` raw (D8-first-class). Two sites: `SKILL.md` Output pointer + `references/CONTEXT-GATHERING.md` scope-note mention.
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/schema-based-template-guidance/record.md` D1/D3/D8; `DECISIONS.md` "Template-schema CLI ruled").
- **Content (superseded, verbatim):**
  - `following` / `[\`codebase-analysis-template.md\`](../../templates/codebase-analysis-template.md).` — `SKILL.md` Output
  - `its findings flow into \`.mochiko/memory/codebase-analysis.md\` (per` / `> \`codebase-analysis-template.md\`).` — `references/CONTEXT-GATHERING.md`
- **Kept deliberately:** `references/CONTEXT-GATHERING.md`'s later "in the codebase-analysis template" conceptual phrase (line 13 — no file path, not a dead pointer); the `artifact-format.md` pointer; `detect-stack.sh` and all descriptive text.
- **Consumers assessed:** n/a (single-writer skill + its reference).

## [v0.63.0] Guardrails cut — detection/procedure prose removed, assessment contract kept; slim description

- **Disposition:** superseded → benchmark-ruled guardrails body + slim description (`variants/body/analysis-codebase/`, `variants/descriptions/analysis-codebase/`), one merged edit.
- **Tier failed:** n/a — supersession by ruling (guardrails-vs-detail benchmark verdict; `DECISIONS.md` 2026-08-10 benchmark-verdict row; `.mochiko/brainstorms/validator-scope-and-verbosity/record.md` Benchmark execution; `report/final-verdict.md`)
- **Content (faithfully compressed — section-level inventory, body 8,962 → 6,509 chars, −27%):**
  - **Removed whole:** `## When to Use` (trigger list — carried by the description) · the four Essential-Floor per-category indicator subsections `#### Security — status indicators`, `#### Testing — status indicators`, `#### Error Handling — status indicators`, `#### Observability — status indicators` (see reconciliation below).
  - **Shortened:** `## Detection Script` (855 → 458 chars — the `detect-stack.sh` invocation and the determinism-boundary note kept; extra prose trimmed).
  - **Kept intact:** `# Analyzing Codebase`, `## Overview`, `## When NOT to Use`, `## Common Mistakes`, `## Mode: Setup Brownfield (the wired path)`, `### Essential-Floor Status Assessment` (the assess-status contract and intent-blind/waiver-blind rule), `### Setup-Brownfield Quality Checklist`, `## Other modes (moved to other clusters — not wired this run)`, `## Related Skills`.
  - Old description (new slim form is 349 chars; **old verbatim, 462 chars**): "This skill MUST be invoked when analyzing an existing codebase during a brownfield /mochiko:setup run — detecting the technology stack, extracting architecture and conventions, and assessing Essential-Floor status — to produce `.mochiko/memory/codebase-analysis.md`. SHOULD also invoke when a setup/constitution producer needs a deterministic stack baseline (`detect-stack.sh`) or a present/partial/absent read of an existing project before authoring governance."
  - Verbatim removed text survives in: git history of the SKILL.md; the before/after pair in `variants/`; archive branch `worktree-brainstorm-validator-scope`.
- **Kept deliberately (guardrails keep-set):** the goal/output contract (Overview + the `.mochiko/memory/codebase-analysis.md` deliverable), the assess-status-vs-require-floor boundary and the intent-blind/waiver-blind rule (`### Essential-Floor Status Assessment`), the anti-patterns (`## Common Mistakes`), the quality checklist, the determinism boundary, and the `references/CONTEXT-GATHERING.md` and template pointers.
- **Protected-content reconciliation (MANDATORY):** the two prior `[v0.24.0] KEPT:` entries reconciled —
  - **`[v0.24.0] KEPT: the four Essential-Floor check tables (what-to-assess criteria per category)`** — **superseded by this ruling.** The guardrails cut removes the four `#### … status indicators` subsections that carried the per-category assessment criteria (auth-at-boundaries, secrets-from-env, test-framework-configured, correlation-IDs, …). The canonical category definitions remain in `authoring-constitution/references/ESSENTIAL-FLOOR.md` (unchanged), which `### Essential-Floor Status Assessment` still points to as the source to assess against; the mochiko-side assess-hint tables are the superseded content. Verbatim in the three homes above. **DEVIATION FLAGGED (see build report):** the surviving sentence "assess against it using the indicators below" now has no indicators below it — the benchmark variant left a dangling internal pointer. Shipped byte-identical to the ruled variant (not the shipper's to rewrite); routed to the T4 audit.
  - **`[v0.24.0] KEPT: Common Mistakes table`** — **survives intact** (`## Common Mistakes`, including the CODEOWNERS governance row). Not touched by this cut.
- **Consumers assessed (1 consuming command):** `commands/setup.md` — dispatches the skill for the brownfield analysis artifact; relies on the `.mochiko/memory/codebase-analysis.md` output and the present/partial/absent status, not on any removed section anchor. Contract intact.


## [v0.24.0] "Scope (this run)" blockquote in Overview
- **Disposition:** deleted — in-file triplication; single home is the "Other modes" section (compressed same wave, below)
- **Tier failed:** 1 (restated in-file: same not-wired scope note appeared in Overview, a When-NOT-to-Use bullet, and the Other-modes section)
- **Content:** only Setup-Brownfield wired; collision mode → spec/plan cluster; Context-report mode → constitution-context cluster, sub-procedure kept in `references/CONTEXT-GATHERING.md`
- **Consumers assessed:** see header — none

## [v0.24.0] Project-type / web-framework / ORM / architecture detection tables (~48 lines)
- **Disposition:** deleted; the Mode section's entity bullet reworded ("use the tables above" → "models/, schema files, ORM annotations"); the determinism-boundary note also reworded ("The detection tables and the Essential-Floor assessment" → "Framework/architecture inference and the Essential-Floor assessment") — collateral dangling reference caught by the independent audit
- **Tier failed:** 2 (generic model knowledge — `package.json`→Node, `@app.get()`→FastAPI; no failure named that an unaided model produces; the deterministic layer is `detect-stack.sh`, whose findings the skill already declares ground truth)
- **Content:** four indicator tables mapping package-manager files → project type, framework/ORM code signatures, and directory layouts → architecture patterns
- **Consumers assessed:** see header — none

## [v0.24.0] Four Essential-Floor example-grep bash blocks (~44 lines)
- **Disposition:** deleted (the four per-category check tables KEPT — see survivor entry below)
- **Tier failed:** 2 (each block re-encoded its check table's "How to Detect" column as example greps the model synthesizes on demand; illustrations, not contracts — grep spelling does not change a present/partial/absent judgment)
- **Content:** ready-to-run grep/ls/find snippets per floor category (auth middleware, env secrets, validation libs; test config/files/CI; custom errors, error logging, status codes; logger config, correlation IDs, PII-in-logs negative check)
- **Consumers assessed:** see header — none

## [v0.24.0] "The script detects:" value-vocabulary list
- **Disposition:** deleted
- **Tier failed:** 2 (redundant encoding — the JSON example directly above shows the same fields; the model consumes actual script output at runtime, not the possible-value enumeration)
- **Content:** six bullets enumerating possible values for project_type / package_manager / frameworks / orms / architecture / ci_cd
- **Consumers assessed:** see header — none

## [v0.24.0] Manual Detection Commands section (~21 lines)
- **Disposition:** deleted; sole non-obvious hint (CODEOWNERS) relocated → Common Mistakes "Ignoring governance" fix cell
- **Tier failed:** 2 (generic shell one-liners — `cat package.json | jq`, `ls` directory probes — the model produces unaided; conditional on "script insufficient" but names no failure)
- **Content:** example bash for tech-stack / architecture / CI-CD / governance / test-structure detection when `detect-stack.sh` is insufficient
- **Consumers assessed:** see header — none

## [v0.24.0] "Other modes" section compressed 10 → 6 lines
- **Disposition:** relocated → itself (compressed in place; now the single in-file home for the not-wired scope note, absorbing the deleted Overview blockquote)
- **Tier failed:** 1 (the per-mode parentheticals restated the same cluster-routing fact twice per bullet)
- **Content:** same routing facts — collision mode → spec/plan cluster (JSON inventory schema moves with it), Context-report mode → constitution-context cluster (extraction sub-procedure kept in `references/CONTEXT-GATHERING.md`)
- **Consumers assessed:** see header — none

## [v0.24.0] KEPT: the four Essential-Floor check tables (what-to-assess criteria per category)
- **Tier-2 evidence:** contested during the pilot's bash-block strip and kept — they carry the assessment criteria themselves (auth-at-boundaries, secrets-from-env, test-framework-configured, correlation-IDs, …) with detection hints; deleting them would leave present/partial/absent status with no defined checks, un-anchoring the tier-blind assessment that `ESSENTIAL-FLOOR.md` (require-floor side) expects this skill to own (assess-status side). Session ruling: pilot ratification 2026-07-25.

## [v0.24.0] KEPT: Common Mistakes table
- **Tier-2 evidence:** contested (suspected generic-table) and kept — every row names a concrete failure and its fix (assuming framework without evidence, inventing findings, redefining the Essential Floor instead of assessing it, ignoring existing governance); gained "CODEOWNERS" in the governance row as the relocation target of the Manual-Detection strip. Session ruling: pilot ratification 2026-07-25.
