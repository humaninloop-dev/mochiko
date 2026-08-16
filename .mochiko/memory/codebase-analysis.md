# Codebase Analysis

> Generated: 2026-08-06T00:00:00Z
> Mode: brownfield-setup
> Status: draft

---

## Part 1: Inventory (Factual)

### Project Identity

| Aspect | Value | Source |
|--------|-------|--------|
| Name | mochiko | `plugins/mochiko/.claude-plugin/plugin.json` |
| Primary Language | Markdown (Claude Code plugin primitives); incidental Bash + Python validation scripts | file survey; `detect-stack.sh` returned `project_type: unknown` |
| Framework | Claude Code plugin system (commands / skills / agents / templates) | `plugins/mochiko/.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json` |
| Package Manager | none — no `package.json`, `pyproject.toml`, `requirements.txt`, or lockfiles | `detect-stack.sh` output (all manifests false) |
| Plugin Version | 0.53.0 (marketplace metadata lags at 0.10.0) | `plugins/mochiko/.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json` |
| Entry Points | 5 slash commands: `brainstorm.md`, `setup.md`, `specify.md`, `plan.md`, `implement.md` | `plugins/mochiko/commands/` |

This is a **prose/primitive library, not an executable application**. The product is a set of
markdown artifacts Claude Code loads natively. The only executable code is 6 helper scripts
embedded in skills (1 Bash, 5 Python), all standalone validators with no shared dependencies:

- `plugins/mochiko/skills/analysis-codebase/scripts/detect-stack.sh`
- `plugins/mochiko/skills/patterns-api-contracts/scripts/validate-openapi.py`
- `plugins/mochiko/skills/authoring-requirements/scripts/validate-requirements.py`
- `plugins/mochiko/skills/authoring-user-stories/scripts/validate-user-stories.py`
- `plugins/mochiko/skills/review-plan-artifacts/scripts/check-artifacts.py`
- `plugins/mochiko/skills/patterns-entity-modeling/scripts/validate-model.py`

### Directory Structure

```
mochiko/
├── CLAUDE.md                  # Operating manual: constraints, conventions, landing ritual
├── ROADMAP.md                 # Thin forward view: thesis · Now/Next/Later · bets · stamp (51 lines)
├── DECISIONS.md               # Thin decision index: 123 rows, one line per ruling (133 lines)
├── BACKLOG.md                 # Open items only, theme-keyed (442 lines, 54 open items)
├── ARCHITECTURE.md            # Living current-state map of the shipped plugin (307 lines)
├── README.md                  # Install + user-facing pipeline guide
├── plugins/mochiko/           # THE PRODUCT — the shipped plugin
│   ├── .claude-plugin/plugin.json   # Manifest, v0.53.0
│   ├── commands/              # 5 command supervisors (goal + harness model)
│   ├── agents/                # 9 personas (producer/validator split)
│   ├── skills/                # 28 skill dirs, 80 files (SKILL.md + references/ + scripts/)
│   └── templates/             # 14 templates + constitution-modules/
├── .mochiko/                  # Repo-side knowledge plane (the repo's own machinery)
│   ├── brainstorms/           # 42 session dirs + index.md (newest-first, status-tracked)
│   ├── decisions/             # 26 ADR-style records (YYYY-MM-DD-slug.md)
│   ├── strips/                # 64 per-primitive strip/supersession ledgers + README.md
│   ├── memory/                # knowledge-management.md (hand-pinned KM invariants)
│   └── archive/               # Frozen: pre-migration ROADMAP.md, REGISTRY.md, backlog-trail.md
├── .claude/
│   ├── settings.json          # Tracked: enables context7 plugin
│   ├── settings.local.json    # UNTRACKED, NOT gitignored — contains live auth token (see Inconsistencies)
│   └── rules/mochiko/         # 2 path-scoped rules: operating-docs.md, primitive-edits.md
└── .claude-plugin/marketplace.json  # Marketplace listing
```

### Detected Patterns

#### Architecture Pattern

| Pattern | Evidence |
|---------|----------|
| Kernel-free plugin: commands are the orchestrators; no engine, no brain code | `ARCHITECTURE.md` system overview; CLAUDE.md "No kernel infrastructure" constraint |
| Producer ↔ validator pairing (author ≠ grader) | 9 agents split producer/reviewer roles (`agents/validator.md`, `agents/devils-advocate.md`); CLAUDE.md axis 5 |
| Skill classification: user-invoked router + model-invoked procedures | `plugins/mochiko/skills/mochiko/SKILL.md` (router); MUST/SHOULD trigger-phrase descriptions in skill frontmatter |
| Two-plane split: shipped plugin (`plugins/mochiko/`) vs repo knowledge plane (`.mochiko/`) | directory structure; `ARCHITECTURE.md` scope statement |
| Governed-edit ceremony: strips ledger + independent audit for any primitive edit | `.mochiko/strips/README.md`, `.claude/rules/mochiko/primitive-edits.md` |

#### Naming Conventions

| Element | Convention | Example |
|---------|------------|---------|
| Skill dirs | kebab-case, verb-noun class prefixes (`authoring-`, `review-`, `patterns-`, `validation-`, `testing-`, `analysis-`, `grooming-`) | `plugins/mochiko/skills/authoring-constitution/` |
| Skill files | `SKILL.md` + `references/` (SCREAMING-KEBAB refs) + optional `scripts/` | `analysis-codebase/references/CONTEXT-GATHERING.md` |
| Agents/commands/templates | kebab-case `.md`, role or artifact names (pipeline artifact templates now ship as `.yaml` schemas under `schemas/`, v0.74.0) | `agents/principal-architect.md`, `plugins/mochiko/schemas/spec.yaml` |
| Decision records | `YYYY-MM-DD-slug.md` | `.mochiko/decisions/2026-07-18-transformer-cluster-retired.md` |
| Brainstorm sessions | `.mochiko/brainstorms/<topic-slug>/record.md` | `agent-decoupling/` |
| Commit messages | Imperative summary + version stamp / decision IDs in parens | `Build code-minimalism ladder + review lens (v0.53.0)` |

#### Error Handling Pattern

| Pattern | Evidence |
|---------|----------|
| Not applicable as a code pattern — no application code. Process-level analogue: default-FAIL verdicts, defect fix-on-sight rules, recorded waivers | `agents/validator.md` ("Defaults to FAIL"); CLAUDE.md brainstorms-index defect rule |

#### Test Pattern

| Aspect | Value |
|--------|-------|
| Framework | None — zero test files (`test_*`/`*_test*`/`*.test.*` search: 0 hits), no pytest/jest config |
| CI | None — no `.github/`, no workflow files, no CI platform config |
| Linting/Formatting | None — no `.editorconfig`, `.prettierrc`, ruff/eslint config; CLAUDE.md (parent dir) references pytest/black/pylint "if configured" — none are |
| Process analogue | Quality is enforced procedurally: independent validator audits per primitive edit, strip notes, human gates — not executable tests |

### Domain Entities

No ORM, schema files, or data models — the "entities" of this repo are its primitive classes:

| Entity | Location | Count | Relationships |
|--------|----------|-------|---------------|
| Command supervisor | `plugins/mochiko/commands/` | 5 | dispatches agents; binds skills + templates |
| Agent persona | `plugins/mochiko/agents/` | 9 | declares `skills:`; persona carries judgment, skill carries procedure |
| Skill | `plugins/mochiko/skills/` | 28 dirs | user-invoked router → model-invoked procedures; producer ↔ validator pairs |
| Template | `plugins/mochiko/templates/` | 14 + modules | bound by skills/commands as output contracts |
| Decision row / ADR / strip entry / brainstorm record | root docs + `.mochiko/` | 123 rows · 26 ADRs · 64 strip files · 42 sessions | KM landing ritual links all four surfaces |

### External Dependencies

| Service | Access Pattern | Config Location |
|---------|---------------|-----------------|
| GitHub (hosting + marketplace distribution) | `git push`; users install via `/plugin marketplace add humaninloop-dev/mochiko` | `README.md` install section |
| context7 MCP plugin | Enabled for doc lookups during sessions | `.claude/settings.json` |
| Anthropic API gateway (NSW education proxy) | `ANTHROPIC_BASE_URL` + auth token for local sessions | `.claude/settings.local.json` (untracked) |
| Removed submodules: `human-in-loop`, `agent-skills-research` | Reference-only; removed 2026-07-21 so plugin installs cleanly | CLAUDE.md "Reference sources" |

No runtime services, no databases, no user data, no PII. The repo stores and processes no data
classes beyond its own prose artifacts.

---

## Part 2: Assessment (Judgment)

### Strengths to Preserve

1. **Disciplined decision provenance**: 123 decision rows, 26 ADRs, 42 indexed brainstorm records, and 64 strip ledgers form an unusually complete audit trail. Every removal from a shipped primitive is version-stamped and independently audited (`.mochiko/strips/README.md`). Any constitution must codify — not replace — this working system.
2. **Operating docs honor their own contracts**: ROADMAP.md is 51 lines with exactly the four ruled pieces; BACKLOG.md carries open items only (the single `[x]` grep hit is the rule text itself, line 7); DECISIONS.md is a thin index. The KM invariants at `.mochiko/memory/knowledge-management.md` are being followed manually, not just asserted.
3. **Author ≠ grader is structural, not aspirational**: validator/devils-advocate personas exist as separate agents, and CLAUDE.md pins the independence rule at every landing. This is the repo's substitute for a test suite and it is live.
4. **Single-sourcing discipline**: rationale lives in exactly one home (record or ADR) with thin indexes pointing at it; templates like `artifact-format.md` are referenced rather than restated.

### Inconsistencies Found

| Area | Finding | Severity | Location |
|------|---------|----------|----------|
| Secrets hygiene | `.claude/settings.local.json` contains a live `ANTHROPIC_AUTH_TOKEN` and is untracked but **not** in `.gitignore` — one `git add -A` from being committed | high | `.claude/settings.local.json`, `.gitignore` |
| Version skew | `marketplace.json` metadata version 0.10.0 vs plugin.json 0.53.0 | low | `.claude-plugin/marketplace.json` |
| Doc version lag | ARCHITECTURE.md header cites v0.48.0; plugin is at v0.53.0 (doc updates only at component-changing landings, so possibly intentional) | low | `ARCHITECTURE.md` line 3 |
| Governance bootstrap gap | Mochiko ships `/mochiko:setup` but has never run it on itself: no `mochiko:governance:begin` region in CLAUDE.md, no `.mochiko/memory/governance-ledger.md`. KM invariants were hand-pinned 2026-07-25 instead — with a recorded revisit trigger naming "the first in-repo setup/amend run" | medium (known + recorded, this run is the trigger) | CLAUDE.md, `.mochiko/memory/knowledge-management.md` |
| Script quality gates | 6 helper scripts have no tests, no lint config, no CI — currently held to no executable standard | low | `plugins/mochiko/skills/*/scripts/` |

### Essential Floor Status

Read against `authoring-constitution/references/ESSENTIAL-FLOOR.md` categories. The indicators
are application-shaped (auth middleware, HTTP codes, logging); this codebase ships no
application, so most checks are **absent by inapplicability** — recorded as `absent` per the
intent-blind rule, with the inapplicability noted as evidence. The session, not this analysis,
decides what that means.

| Category | Check | Status | Evidence (file-cited, one line) |
|----------|-------|--------|--------------------------------|
| Security | Auth at boundaries | absent | No service boundaries exist; no middleware anywhere in repo |
| Security | Secrets from env | partial | No hardcoded secrets in tracked files, but live token sits in un-gitignored `.claude/settings.local.json` |
| Security | Input validation | partial | Validation scripts check artifact formats (`validate-requirements.py` regex/JSON checks); no untrusted-input surface exists |
| Testing | Test framework configured | absent | No pytest/jest/vitest config anywhere; `detect-stack.sh`: all manifests false |
| Testing | Test files present | absent | 0 files matching `test_*` / `*_test*` / `*.test.*` |
| Testing | CI runs tests | absent | No `.github/`, no CI config of any platform |
| Error Handling | Explicit error types | absent | No application code; scripts exit with JSON results, no custom error classes |
| Error Handling | Context preservation | absent | Not applicable — no runtime errors to contextualize; process analogue (default-FAIL + fix lists) exists in `agents/validator.md` |
| Error Handling | Appropriate status codes | absent | No API surface |
| Observability | Structured logging | absent | No logger anywhere; nothing runs long enough to log |
| Observability | Correlation IDs | absent | No requests to correlate |
| Observability | No PII in logs | absent | No logs; also no PII in the repo at all |

**Category rollup:** Security partial · Testing absent · Error Handling absent · Observability absent

**Honest framing for the gate reader:** the absent statuses are facts about a prose library,
not neglect. The repo's real quality floor is procedural (validator audits, strips, human
gates) and is `present` by any reasonable reading — but that floor has no category in the
canonical Essential Floor, which is application-shaped. The one genuinely actionable security
finding is the un-gitignored token file.

### Recommended Constitution Focus

Based on this analysis, the constitution should:

1. **Codify the procedural quality floor as this project's floor**: author ≠ grader audits, strip-note ceremony, KM landing ritual, and default-FAIL validation are the working equivalents of tests and CI here — make them the asserted floor rather than importing application-shaped categories that don't bind.
2. **Close the secrets gap immediately**: require `.claude/settings.local.json` (and any secret-bearing local file) in `.gitignore`; assert no credentials in tracked files as a MUST.
3. **Resolve the bootstrap deviation on schedule**: the hand-pinned KM module names this setup run as its revisit trigger — the constitution should either ratify the pinned core set or re-ratify the recorded deviation, and decide ARCHITECTURE.md/GLOSSARY.md status (ARCHITECTURE.md has since gained content; GLOSSARY.md remains absent).
4. **Decide the helper-script standard explicitly**: either waive executable-code gates for the 6 standalone scripts (recorded waiver) or require a minimal check (smoke run in CI) — currently the standard is silently undefined.
5. **Guard the existing doc contracts, don't duplicate them**: ROADMAP/BACKLOG/DECISIONS semantics already match the KM module's roles and are enforced by `.claude/rules/mochiko/operating-docs.md` — the constitution should point at these homes, not restate them (single-sourcing is a live convention here).

---

## Appendix: Detection Method

| Aspect | Method Used |
|--------|-------------|
| Tech Stack | `detect-stack.sh` (returned `unknown` — correct for a prose library); manual file survey |
| Architecture | ARCHITECTURE.md + CLAUDE.md read; directory pattern confirmation |
| Entities | Primitive-class inventory (no ORM to scan) |
| Conventions | File sampling: skill frontmatter, agent frontmatter, git log (25 commits), doc contracts vs actual doc shapes |
| Essential Floor | Canonical indicator grep/find sweeps; every status file-cited above |
