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
