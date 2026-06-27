---
name: analysis-codebase
description: This skill MUST be invoked when analyzing an existing codebase during a brownfield /mochiko:setup run — detecting the technology stack, extracting architecture and conventions, and assessing Essential-Floor status — to produce `.mochiko/memory/codebase-analysis.md`. SHOULD also invoke when a setup/constitution producer needs a deterministic stack baseline (`detect-stack.sh`) or a present/partial/absent read of an existing project before authoring governance.
---

# Analyzing Codebase

## Overview

Systematically analyze an existing codebase to extract the structural information a brownfield
`/mochiko:setup` run needs: the tech stack, architecture, conventions, domain entities, and an
**assessment** of the Essential Floor (Security / Testing / Error Handling / Observability). The
deliverable is `.mochiko/memory/codebase-analysis.md` — the producer's read of "what this codebase
already is," which the constitution author and the Phase-2 human checkpoint consume.

> **Scope (this run).** Only the **Setup-Brownfield** path is wired here. The other two HIL modes
> are moved to their own clusters and are *not* carried this run:
> - **Brownfield / collision mode** (entity + endpoint extraction → JSON inventory with
>   collision risks) → lives in the **spec/plan cluster**, not wired this run.
> - **Standalone Context-report mode** (a human-facing Project Context Report) → lives in the
>   **constitution-context cluster**, not wired this run. (Its extraction *sub-procedure* is kept
>   here, folded into Setup-Brownfield — see `references/CONTEXT-GATHERING.md`.)

## When to Use

- Running `/mochiko:setup` on a project that already has code (brownfield governance)
- Establishing a deterministic tech-stack baseline before authoring a constitution
- Understanding architecture, conventions, and domain entities before imposing standards
- Producing the present/partial/absent Essential-Floor read the constitution should respond to

## When NOT to Use

- **Greenfield projects**: no existing code to analyze; go straight to `mochiko:authoring-constitution`
- **Single-file scripts**: no architectural patterns to extract
- **Documentation-only review**: use standard file reading instead
- **Before the project directory exists**: nothing to analyze yet
- **When the user already provided complete context**: skip analysis if the tech stack and patterns are already documented
- **Collision detection / JSON inventory**: that is the spec/plan-cluster Brownfield mode (not wired this run)

## Common Mistakes

| Mistake | Problem | Fix |
|---------|---------|-----|
| Assuming framework | Guessing without evidence | Verify with code patterns |
| Missing directories | Only checking standard paths | Projects vary, explore |
| Over-extracting | Analyzing every file | Focus on config and patterns |
| Ignoring governance | Missing existing decisions | Check README, CLAUDE.md, ADRs |
| Inventing findings | Documenting assumptions | Only report what is found |
| Redefining the Essential Floor | Restating the four categories here | Read the canonical definition; this skill only *assesses* status |

## Project Type Detection

Identify project type from package manager files:

| File | Project Type |
|------|--------------|
| `package.json` | Node.js/JavaScript/TypeScript |
| `pyproject.toml` / `requirements.txt` | Python |
| `go.mod` | Go |
| `Cargo.toml` | Rust |
| `pom.xml` / `build.gradle` | Java |
| `Gemfile` | Ruby |
| `pubspec.yaml` | Flutter/Dart |

## Framework Detection

### Web Frameworks

| Framework | Indicators |
|-----------|------------|
| **Express** | `express()`, `router.get()`, `app.use()` |
| **FastAPI** | `@app.get()`, `FastAPI()`, `APIRouter` |
| **Django** | `urls.py`, `views.py`, `models.py` pattern |
| **Flask** | `@app.route()`, `@bp.route()` |
| **Rails** | `routes.rb`, `app/models/`, `app/controllers/` |
| **Spring** | `@RestController`, `@GetMapping`, `@Entity` |
| **Gin/Echo** | `r.GET()`, `e.GET()` |

### ORM/Database Frameworks

| Framework | Indicators |
|-----------|------------|
| **Prisma** | `schema.prisma`, `@prisma/client` |
| **TypeORM** | `@Entity()`, `@Column()`, `DataSource` |
| **SQLAlchemy** | `Base`, `db.Model`, `Column()` |
| **Django ORM** | `models.Model`, `models.CharField` |
| **GORM** | `gorm.Model`, `db.AutoMigrate` |
| **Mongoose** | `mongoose.Schema`, `new Schema({` |
| **ActiveRecord** | `ApplicationRecord`, `has_many` |

## Architecture Pattern Recognition

| Pattern | Indicators |
|---------|------------|
| **Layered** | `src/models/`, `src/services/`, `src/controllers/` |
| **Feature-based** | `src/auth/`, `src/users/`, `src/tasks/` |
| **Microservices** | Multiple package files, docker compose |
| **Serverless** | `serverless.yml`, `lambda/`, `functions/` |
| **MVC** | `models/`, `views/`, `controllers/` |
| **Clean/Hexagonal** | `domain/`, `application/`, `infrastructure/` |

## Mode: Setup Brownfield (the wired path)

For `/mochiko:setup` on an existing codebase — comprehensive analysis combining the context
sub-procedure with domain-entity extraction and an Essential-Floor status assessment.

**What to Extract:**
- Tech stack, conventions, and architecture (the **Context-gathering sub-procedure** —
  see [references/CONTEXT-GATHERING.md](references/CONTEXT-GATHERING.md))
- Domain entities and relationships (use the **Framework / ORM detection** tables above to
  locate where entities live; document what is found — the deeper collision-risk inventory is
  the spec/plan-cluster Brownfield mode, not produced here)
- Essential-Floor **status** assessment (present / partial / absent — see below)
- Inconsistencies and strengths to preserve

**Output**: `.mochiko/memory/codebase-analysis.md`, following
[`codebase-analysis-template.md`](../../templates/codebase-analysis-template.md).

### Essential-Floor Status Assessment

The four Essential-Floor categories — **Security, Testing, Error Handling, Observability** — are
**defined canonically** in
[`authoring-constitution/references/ESSENTIAL-FLOOR.md`](../authoring-constitution/references/ESSENTIAL-FLOOR.md).
Governance owns *require-floor* (what the categories are and why they are non-negotiable). This
skill owns the other half — **assess-status**: detect each category's present / partial / absent
state in the existing codebase, with file-cited evidence. **Do not redefine the categories here.**
Read the canonical definition, then assess against it using the indicators below.

#### Security — status indicators

| Check | How to Detect | Status Values |
|-------|---------------|---------------|
| Auth at boundaries | Middleware patterns (`authenticate`, `authorize`, `requireAuth`) | present/partial/absent |
| Secrets from env | `.env.example` exists, no hardcoded credentials in code | present/partial/absent |
| Input validation | Schema validation libraries, input checking patterns | present/partial/absent |

```bash
# Auth middleware
grep -r "authenticate\|authorize\|requireAuth\|isAuthenticated" src/ 2>/dev/null

# Environment variables
ls .env.example .env.sample 2>/dev/null
grep -r "process.env\|os.environ\|os.Getenv" src/ 2>/dev/null

# Validation
grep -r "zod\|yup\|joi\|pydantic\|validator" package.json pyproject.toml 2>/dev/null
```

#### Testing — status indicators

| Check | How to Detect | Status Values |
|-------|---------------|---------------|
| Test framework configured | Config files (`jest.config.*`, `pytest.ini`, `vitest.config.*`) | present/partial/absent |
| Test files present | Files matching `*.test.*`, `*_test.*`, `test_*.*` | present/partial/absent |
| CI runs tests | Test commands in workflow files | present/partial/absent |

```bash
# Test config
ls jest.config.* vitest.config.* pytest.ini pyproject.toml 2>/dev/null

# Test files
find . -name "*.test.*" -o -name "*_test.*" -o -name "test_*.*" 2>/dev/null | head -5

# CI test commands
grep -r "npm test\|yarn test\|pytest\|go test" .github/workflows/ 2>/dev/null
```

#### Error Handling — status indicators

| Check | How to Detect | Status Values |
|-------|---------------|---------------|
| Explicit error types | Custom error classes/types defined | present/partial/absent |
| Context preservation | Error messages include context, stack traces logged | present/partial/absent |
| Appropriate status codes | API responses use correct HTTP status codes | present/partial/absent |

```bash
# Custom errors
grep -r "class.*Error\|extends Error\|Exception" src/ 2>/dev/null | head -5

# Error logging
grep -r "error.*context\|error.*stack\|logger.error" src/ 2>/dev/null | head -3

# Status codes
grep -r "status(4\|status(5\|HttpStatus\|status_code" src/ 2>/dev/null | head -3
```

#### Observability — status indicators

| Check | How to Detect | Status Values |
|-------|---------------|---------------|
| Structured logging | Logger config (winston, pino, structlog, logrus) | present/partial/absent |
| Correlation IDs | Request ID middleware, trace ID patterns | present/partial/absent |
| No PII in logs | Log sanitization, no email/password in log statements | present/partial/absent |

```bash
# Logger config
grep -r "winston\|pino\|structlog\|logrus\|zap" package.json pyproject.toml go.mod 2>/dev/null

# Correlation IDs
grep -r "requestId\|correlationId\|traceId\|x-request-id" src/ 2>/dev/null | head -3

# PII check (negative - should NOT find these in logs)
grep -r "logger.*email\|logger.*password\|log.*password" src/ 2>/dev/null
```

### Setup-Brownfield Quality Checklist

Before finalizing the analysis:

- [ ] Project identity complete (name, language, framework, entry points)
- [ ] Directory structure documented with purposes
- [ ] Architecture pattern identified with evidence
- [ ] Naming conventions documented (files, variables, functions, classes)
- [ ] All four Essential-Floor categories assessed (present/partial/absent, file-cited)
- [ ] Domain entities extracted with relationships
- [ ] External dependencies documented
- [ ] Strengths to preserve identified (minimum 2-3)
- [ ] Inconsistencies documented with severity
- [ ] Recommendations provided for constitution focus
- [ ] File paths cited for all findings
- [ ] Output written to `.mochiko/memory/codebase-analysis.md`

## Other modes (moved to other clusters — not wired this run)

- **Brownfield / collision mode** — entity + endpoint extraction into a JSON inventory with
  collision risks against a proposed spec. (Collision / spec-plan mode lives in the **spec/plan
  cluster**, not wired this run; its JSON inventory schema contract moves with it — no schema
  reference is carried here.)
- **Standalone Context-report mode** — a human-facing Project Context Report for constitution
  authoring. (Lives in the **constitution-context cluster**, not wired this run. The extraction
  *sub-procedure* it shares is kept here in `references/CONTEXT-GATHERING.md`, folded into
  Setup-Brownfield.)

## Detection Script

Run the automated detection script for fast, deterministic stack identification:

```bash
bash scripts/detect-stack.sh /path/to/project
```

**Output:**
```json
{
  "project_type": "nodejs",
  "package_manager": "npm",
  "frameworks": ["express"],
  "orms": ["prisma"],
  "architecture": ["feature-based"],
  "ci_cd": ["github-actions"],
  "files_found": {...}
}
```

The script detects:
- **Project type**: nodejs, python, go, rust, java, ruby, flutter, elixir
- **Package manager**: npm, yarn, pnpm, pip, poetry, cargo, etc.
- **Frameworks**: express, fastapi, django, nextjs, gin, rails, spring-boot, etc.
- **ORMs**: prisma, typeorm, sqlalchemy, mongoose, gorm, activerecord, etc.
- **Architecture**: clean-architecture, mvc, layered, feature-based, serverless, microservices
- **CI/CD**: github-actions, gitlab-ci, jenkins, circleci, etc.

**Usage pattern:**
1. Run script first for deterministic baseline
2. Use script output to guide deeper LLM analysis
3. Script findings are ground truth; LLM adds nuance

> **Determinism boundary.** `detect-stack.sh` is the deterministic layer (pure `bash` + `jq`,
> reads project files, JSON to stdout — no kernel, no network). The detection tables and the
> Essential-Floor assessment are the model-judgment layer on top. Keep the boundary explicit.

## Manual Detection Commands

For cases where script detection is insufficient:

```bash
# Tech stack detection
cat package.json | jq '{name, engines, dependencies}'
cat pyproject.toml
cat .tool-versions .nvmrc .python-version 2>/dev/null

# Architecture detection
ls -d src/domain src/application src/features 2>/dev/null

# CI/CD detection
ls .github/workflows/*.yml .gitlab-ci.yml 2>/dev/null

# Governance detection
ls CODEOWNERS .github/CODEOWNERS docs/CODEOWNERS 2>/dev/null
cat CODEOWNERS 2>/dev/null | head -20

# Test structure
ls -d test/ tests/ spec/ __tests__/ 2>/dev/null
```

## Related Skills

- **For brownfield constitutions**: **REQUIRED:** Use `mochiko:authoring-constitution` (brownfield branch) after analysis
- **For greenfield projects**: **OPTIONAL:** Use `mochiko:authoring-constitution` directly
- **For validation**: **OPTIONAL:** `mochiko:validation-constitution` grades the constitution (run by an independent validator — a different agent than the author)
