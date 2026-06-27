---
name: authoring-constitution
description: This skill MUST be invoked when authoring or amending a project constitution (the governance document at `.mochiko/memory/constitution.md`) — whether writing enforceable principles, defining governance, setting quality gates, or establishing an amendment/version policy. Handles BOTH modes in one place: greenfield (opinionated defaults from scratch — Essential Floor plus recommended architectural principles) and brownfield (codifying an existing codebase's patterns — Essential Floor assessed against the code plus an Emergent Ceiling, informed by `.mochiko/memory/codebase-analysis.md`). SHOULD also invoke when the authoring work concerns principle enforcement, testability, rationale, the Three-Part Rule, RFC 2119 keywords, a SYNC IMPACT report, an Essential Floor, or an Emergent Ceiling. This is the single constitution-authoring skill for both new and existing projects — there is no separate brownfield skill.
---

# Authoring Constitution

## Overview

Write project constitutions that teams actually follow. Every principle must be enforceable, testable, and justified; vague aspirations are rejected in favor of actionable constraints with measurable criteria. The artifact lives at `.mochiko/memory/constitution.md`.

This skill produces a **reviewable** governance document. Its quality is graded by an **independent validator** (a separate agent running `validation-constitution`), and accepted at a named human gate — the dispatch sequencing, the produce→validate→revise loop, and the human acceptance gate are owned by the `setup` command lead, not by this skill.

## Two modes, one shared core

Constitution authoring runs in one of two modes. **Pick the mode first**; both then converge on the same shared core.

| Mode | Use when | Adds on top of the shared core |
|------|----------|--------------------------------|
| **greenfield** | A new project with no existing code to honor — author *opinionated defaults*. | Essential Floor I–IV written from defaults **+** recommended architectural principles (V+). |
| **brownfield** | An existing codebase — *codify what is already there*. Requires `.mochiko/memory/codebase-analysis.md` (produced upstream by `analysis-codebase`). | Essential Floor *assessed against the code* (present/partial/absent) **+** an Emergent Ceiling that codifies good existing patterns **+** Evolution Notes. |

**The shared core is authored once, identically, in both modes** — the Three-Part Principle Rule, RFC 2119 keywords, the SYNC IMPACT REPORT, and the Mandatory Constitution Sections below. **Brownfield does NOT restate the shared core; it defers to this common path.** Only the *floor-and-above* layer differs by mode:

- **greenfield → [references/RECOMMENDED-PATTERNS.md](references/RECOMMENDED-PATTERNS.md)** for the architectural defaults.
- **brownfield → [references/EMERGENT-CEILING-PATTERNS.md](references/EMERGENT-CEILING-PATTERNS.md)** for the pattern library.
- **both → [references/ESSENTIAL-FLOOR.md](references/ESSENTIAL-FLOOR.md)** — the one canonical definition of the four NON-NEGOTIABLE floor categories (greenfield writes them from defaults; brownfield assesses the codebase against them).

> **Mode prerequisite (lead-owned).** Brownfield mode consumes `.mochiko/memory/codebase-analysis.md`. Ensuring `analysis-codebase` has run and wiring that handoff is the `setup` lead's job — this skill assumes the analysis is present when invoked in brownfield mode.

## When to Use

- Authoring a constitution for a **new** project that needs governance (greenfield mode).
- Authoring a constitution for an **existing** codebase, codifying its conventions without disrupting working code (brownfield mode).
- Amending an existing constitution: adding/redefining principles, quality gates, or governance, with a SYNC IMPACT bump.
- Establishing enforcement mechanisms, testability criteria, and amendment/version policy.

## When NOT to Use

- **Reviewing/grading an existing constitution** → that is the independent validator's job (`validation-constitution`), run by a different agent.
- **Analyzing the codebase** (detecting stack, strengths-to-preserve, gaps) → run `analysis-codebase` first; brownfield mode consumes its output.
- **Project too small for formal governance** — single-file scripts and throwaway prototypes do not need constitutions.

## Common Mistakes (shared core)

| Mistake | Problem | Fix |
|---------|---------|-----|
| Vague principles | "Code should be clean" has no enforcement | Add specific, measurable criteria: "Functions MUST be ≤40 lines" |
| Missing enforcement | Principles without verification become suggestions | Every principle needs CI automation, code review checklist, or audit process |
| Untestable criteria | "Good architecture" can't be verified | Define binary pass/fail: "No domain imports from infrastructure layer" |
| No rationale | Future maintainers don't know why rules exist | Explain the failure mode prevented and success enabled |
| Skipping SYNC IMPACT | Constitution changes without audit trail | Always update the SYNC IMPACT REPORT header with version changes |
| CLAUDE.md drift | AI assistants operate with outdated guidance | Include the CLAUDE.md Synchronization section; keep both files versioned together |

---

# Shared core (both modes)

## The Three-Part Principle Rule

Every principle MUST have three components. A principle without all three is incomplete and should not be accepted.

### 1. Enforcement

How compliance is verified. Without enforcement, a principle is a suggestion.

```markdown
**Enforcement**:
- CI runs `ruff check .` and blocks merge on violations
- Code review MUST verify test files accompany new functionality
- Quarterly audit checks exception registry for staleness
```

**Enforcement Types**:

| Type | Examples | Strength |
|------|----------|----------|
| **CI Automated** | Linting, tests, coverage gates | Strongest—no human judgment needed |
| **Code Review** | Architecture compliance, security review | Strong—explicit checklist item |
| **Tooling** | Pre-commit hooks, IDE plugins | Medium—can be bypassed |
| **Audit** | Quarterly review, compliance check | Weaker—periodic, not continuous |

### 2. Testability

What pass/fail looks like. A principle without testable criteria is merely an aspiration.

```markdown
**Testability**:
- Pass: `flutter analyze` exits with code 0
- Pass: All functions have ≤10 cyclomatic complexity
- Fail: Any file exceeds 400 lines without documented exception
```

**Testability Requirements**:
- Binary outcome (pass or fail)
- Measurable threshold where applicable
- Observable without subjective judgment
- Reproducible by any team member

### 3. Rationale

Why this constraint exists. Future maintainers need this to evaluate if the rule is still relevant.

```markdown
**Rationale**: Tests written after implementation tend to validate what was built rather than what was intended. Test-first ensures requirements drive implementation, catches defects early, and produces inherently testable, modular code.
```

**Rationale Requirements**:
- Explains the failure mode this prevents
- Describes the success this enables
- Provides context for future evaluation
- Justifies the enforcement overhead

## Principle Writing Format

```markdown
### I. [Principle Name]

[Declarative statement of the constraint using RFC 2119 keywords]

- [Specific rule 1]
- [Specific rule 2]
- [Specific rule 3]

**Enforcement**:
- [How compliance is verified]
- [Specific commands or processes]

**Testability**:
- [Pass/fail criteria]
- [Measurable thresholds]

**Rationale**: [Why this constraint exists—what failure it prevents, what success it enables]
```

## RFC 2119 Keywords

Use precise language for requirements:

| Keyword | Meaning | Example |
|---------|---------|---------|
| **MUST** | Absolute requirement; no exceptions | "Tests MUST pass before merge" |
| **MUST NOT** | Absolute prohibition | "Secrets MUST NOT be committed" |
| **SHOULD** | Recommended; valid exceptions exist | "Functions SHOULD be under 40 lines" |
| **SHOULD NOT** | Discouraged; valid exceptions exist | "Magic numbers SHOULD NOT appear" |
| **MAY** | Optional; implementation choice | "Teams MAY adopt additional linting rules" |

See [references/RFC-2119-KEYWORDS.md](references/RFC-2119-KEYWORDS.md) for detailed usage.

## Mandatory Constitution Sections

Every constitution MUST include these sections, regardless of mode:

### 1. SYNC IMPACT REPORT (Header)

Track changes as an HTML comment at file top. This provides an audit trail of constitution evolution.

```html
<!--
SYNC IMPACT REPORT
==================
Version change: X.Y.Z → A.B.C (MAJOR|MINOR|PATCH: Brief rationale)

Modified principles: [List or "None (enforcement details updated)"]

Added sections:
- [New section name]

Removed sections:
- [Removed section name] (or "None")

Configuration changes:
- [File/path change]: [old] → [new]
- [Structural change description]

Templates requiring updates:
- CLAUDE.md: [Status - updated ✅ or pending ⚠️]
- [Other templates]: [Status]

Follow-up TODOs:
- [Any deferred items] (or "None")

Previous reports:
- X.Y.Z (YYYY-MM-DD): [One-line summary of that version's changes]
- W.X.Y (YYYY-MM-DD): [One-line summary]
- ...
-->
```

**Version History Best Practice**: Maintain a rolling log of previous versions in the SYNC IMPACT REPORT. This provides:
- Quick reference for what changed when
- Context for understanding current state
- Audit trail for compliance reviews

Example from a mature constitution:
```html
<!--
Previous reports:
  - 3.1.0 (YYYY-MM-DD): Added CLAUDE.md synchronization mandate
  - 3.0.0 (YYYY-MM-DD): Adopted hexagonal architecture, added strategic abstraction principle
  - 2.1.0 (YYYY-MM-DD): Added unification trigger to API consistency principle
  - 2.0.0 (YYYY-MM-DD): Added API consistency principle
  - 1.8.0 (YYYY-MM-DD): Added exception registry and process
-->
```

See [references/SYNC-IMPACT-FORMAT.md](references/SYNC-IMPACT-FORMAT.md) for the complete format.

### 2. Core Principles

Numbered principles (I, II, III...) with Enforcement/Testability/Rationale.

**Naming Conventions**:
- Use Roman numerals (I, II, III, IV, V...)
- Name captures the constraint domain
- Mark non-negotiable principles explicitly: `(NON-NEGOTIABLE)`

**Common Principle Categories**:

| Category | Examples |
|----------|----------|
| **Development Process** | Test-First, Code Review, Documentation |
| **Code Quality** | Linting, Complexity Limits, Coverage |
| **Architecture** | Layer Rules, Dependency Flow, Module Boundaries |
| **Security** | Auth, Secrets, Input Validation |
| **Operations** | Observability, Error Handling, Performance |
| **Governance** | Versioning, Dependencies, Exceptions |

> The **content** of the Core Principles is where the two modes diverge — see the Greenfield branch and Brownfield branch below. Every project, in either mode, MUST cover the four Essential Floor categories ([references/ESSENTIAL-FLOOR.md](references/ESSENTIAL-FLOOR.md)).

### 3. Technology Stack

Document mandated technology choices with rationale:

```markdown
## Technology Stack

| Category | Choice | Rationale |
|----------|--------|-----------|
| Language | Python 3.12 | Type hints, performance, ecosystem |
| Framework | FastAPI | Async-first, Pydantic integration |
| Testing | pytest | Fixtures, parametrization, plugins |
| Linting | ruff | Fast, replaces multiple tools |
```

In brownfield mode this table is populated **from the codebase analysis** rather than chosen fresh.

### 4. Quality Gates

Define automated checks that block merge:

```markdown
## Quality Gates

| Gate | Requirement | Measurement | Enforcement |
|------|-------------|-------------|-------------|
| Static Analysis | Zero errors | `ruff check .` | CI automated |
| Type Checking | Zero errors | `pyright` | CI automated |
| Test Suite | All pass | `pytest` | CI automated |
| Coverage | ≥80% | `pytest --cov-fail-under=80` | CI automated |
| Security | No vulnerabilities | `pip-audit` | CI automated |
```

### 5. Governance

Define how the constitution itself evolves:

```markdown
## Governance

### Amendment Process
1. Propose change via PR to constitution file
2. Document rationale for change
3. Review impact on existing code
4. Obtain team consensus
5. Update version per semantic versioning

### Version Policy
- **MAJOR**: Principle removal or incompatible redefinition
- **MINOR**: New principle or significant expansion
- **PATCH**: Clarification or wording improvement

### Exception Registry
Approved exceptions MUST be recorded in `docs/constitution-exceptions.md` with:
- Exception ID, Principle, Scope, Justification
- Approved By, Date, Expiry, Tracking Issue

**Version**: X.Y.Z | **Ratified**: YYYY-MM-DD | **Last Amended**: YYYY-MM-DD
```

### 6. CLAUDE.md Synchronization

Define synchronization requirements. This is critical because AI coding assistants read CLAUDE.md as their primary instruction source. **This section's SPEC is constitution content and stays here.** The *operational* act of propagating constitution changes into CLAUDE.md (executing the sync) is handled by a separate cluster's `syncing-claude-md` skill — see the documented stub under Related, below. Writing the constitution's own synchronization section is part of authoring; performing the sync is not.

```markdown
## CLAUDE.md Synchronization

The `CLAUDE.md` file at repository root MUST remain synchronized with this constitution.
It serves as the primary agent instruction file and MUST contain all information
necessary for AI coding assistants to operate correctly.

**Mandatory Sync Artifacts**:

| Constitution Section | CLAUDE.md Section | Sync Rule |
|---------------------|-------------------|-----------|
| Core Principles (I-X) | Principles Summary | MUST list all principles with enforcement keywords |
| Layer Import Rules | Architecture section | MUST replicate layer rules table |
| Technology Stack | Technical Stack | MUST match exactly |
| Quality Gates | Quality Gates | MUST match exactly |
| Development Workflow | Development Workflow | MUST match branch/review rules |
| Project Management | Project Management | MUST include tool conventions |

**Synchronization Process**:

When amending this constitution:

1. Update constitution version and content
2. Update CLAUDE.md to reflect all changes in the Mandatory Sync Artifacts table
3. Verify CLAUDE.md version matches constitution version
4. Include both files in the same commit
5. PR description MUST note "Constitution sync: CLAUDE.md updated"

**Enforcement**:

- Code review MUST verify CLAUDE.md is updated when constitution changes
- CLAUDE.md MUST display the same version number as the constitution
- Sync drift between files is a blocking issue for PRs that modify either file

**Rationale**: If CLAUDE.md diverges from the constitution, agents will operate with
outdated or incorrect guidance, undermining the governance this constitution establishes.
```

---

# Greenfield branch (opinionated defaults)

Use when authoring a constitution for a **new** project. The job is to install good defaults from day one — it is easier to start with discipline than to retrofit it.

## Greenfield Core Principles

1. **Essential Floor (I–IV).** Write the four NON-NEGOTIABLE categories from opinionated defaults: **I. Security**, **II. Testing**, **III. Error Handling**, **IV. Observability**. The full category requirements and example principles are in [references/ESSENTIAL-FLOOR.md](references/ESSENTIAL-FLOOR.md) (the canonical definition, shared with brownfield mode).

2. **Recommended architectural principles (V+).** Beyond the floor, greenfield constitutions SHOULD include architectural principles. See [references/RECOMMENDED-PATTERNS.md](references/RECOMMENDED-PATTERNS.md) for:
   - **Hexagonal Architecture** (Ports & Adapters) — layer rules, dependency flow, port interfaces
   - **Single Responsibility & Module Boundaries** — complexity limits, separation of concerns
   - **Dependency Discipline** — justification, isolation, vulnerability scanning

These patterns establish good foundations from day one. They are opinionated defaults, not codified observations — greenfield has no existing code to read.

---

# Brownfield branch (codify existing patterns)

Use when authoring a constitution for an **existing codebase**, using the **Essential Floor + Emergent Ceiling** approach. The core insight: existing codebases have implicit conventions worth preserving (Emergent Ceiling) but may lack foundational governance in critical areas (Essential Floor). Codify both — respecting what works while establishing governance.

Brownfield mode reuses the entire shared core above (Three-Part Rule, RFC 2119, SYNC IMPACT, mandatory sections) **without restating it**. What follows is the brownfield-unique layer only.

## Brownfield Input

Brownfield authoring is driven by `.mochiko/memory/codebase-analysis.md`. Read it for:
- **"Strengths to Preserve"** → Emergent Ceiling candidates.
- **Gap / Essential-Floor status** → which floor categories are present, partial, or absent.

If the analysis is missing, the work devolves into guessing about existing patterns — the `setup` lead sequences `analysis-codebase` ahead of brownfield authoring.

## Essential Floor (NON-NEGOTIABLE) — assessed against the code

Every constitution MUST include principles for the four floor categories. The canonical definition lives in [references/ESSENTIAL-FLOOR.md](references/ESSENTIAL-FLOOR.md):

| Category | Minimum Requirements | Default Enforcement |
|----------|---------------------|---------------------|
| **Security** | Auth at boundaries, secrets via env/secret managers, input validation, secret scanning in CI | Integration tests, code review, secret scanning tools |
| **Testing** | Automated tests exist, coverage ≥80% (configurable), ratchet rule (coverage MUST NOT decrease) | CI test gate, coverage threshold with warning/blocking levels |
| **Error Handling** | Explicit handling, RFC 7807 Problem Details format, correlation IDs in responses | Schema validation in tests, code review |
| **Observability** | Structured logging, correlation IDs, APM integration, no PII in logs | Config verification, log audit, APM dashboards |

**Writing Essential Floor principles in brownfield mode:**

- If the codebase **has** the capability → the principle codifies the existing pattern with enforcement.
- If the codebase **lacks** the capability → the principle states "MUST implement" and references a roadmap gap.

## Emergent Ceiling (FROM CODEBASE)

Beyond the floor, identify **existing good patterns** worth codifying:

1. **Read the codebase analysis** — look for the "Strengths to Preserve" section.
2. **Identify patterns** — naming conventions, architecture patterns, error formats.
3. **Codify as principles** — with enforcement mechanisms.

See [references/EMERGENT-CEILING-PATTERNS.md](references/EMERGENT-CEILING-PATTERNS.md) for the pattern library with examples.

**Common Pattern Categories:**

| Pattern Category | What to Look For |
|------------------|------------------|
| **Code Quality** | Documentation requirements, API annotations, deprecation handling |
| **Architecture** | Layer rules, dependency injection, module boundaries |
| **API Design** | Response formats, versioning, pagination |
| **Authorization** | Role-based access, permission checks |
| **Resilience** | Retry policies, circuit breakers, timeouts |
| **Configuration** | Strongly-typed options, feature flags |
| **Error Handling** | Error display guidelines, data resilience |
| **Observability** | Log levels, context requirements, crash reporting |
| **Product Analytics** | Event categories, naming conventions, funnel tracking |
| **Naming Conventions** | File/class/variable naming, directory structure |

## Brownfield Constitution Structure

```markdown
# [Project] Constitution

<!-- SYNC IMPACT REPORT -->

## Core Principles

### Essential Floor Principles
I. Security by Default
II. Testing Discipline
III. Error Handling Standards
IV. Observability Requirements

### Emergent Ceiling Principles
V. [Pattern from codebase]
VI. [Pattern from codebase]
...

## Technology Stack
[From codebase analysis]

## Quality Gates
[From codebase analysis + essential floor requirements]

## Governance
[Standard governance section]

## Evolution Notes

This constitution was created from brownfield analysis.

**Essential Floor Status** (from codebase-analysis.md):
| Category | Status | Gap |
|----------|--------|-----|
| Security | partial | GAP-001 |
| Testing | partial | GAP-002 |
| Error Handling | present | - |
| Observability | absent | GAP-003 |

See `.mochiko/memory/evolution-roadmap.md` for the improvement plan.
```

> **Roadmap stub (moved-to-other-cluster).** The Evolution Notes section *spec* stays here — the constitution should document its gap status. But producing `evolution-roadmap.md` (the improvement plan) is the roadmap cluster's job, not ported this run. Reference `.mochiko/memory/evolution-roadmap.md` as a **documented stub**: write the gap-status table now; the linked roadmap is filled in when that cluster lands.

## Brownfield Quality Checklist

Additional checks for brownfield constitutions (beyond the shared-core checklist):

- [ ] All four Essential Floor categories have principles
- [ ] Existing good patterns identified and codified
- [ ] Gap references included where the codebase lacks capability
- [ ] Technology stack matches the codebase analysis
- [ ] Quality gates reflect current + target state
- [ ] Evolution Notes section documents brownfield context and gap status

## Brownfield Common Mistakes

### Mistake 1: Skipping Essential Floor categories

**Problem**: Assuming the codebase "doesn't need" security or observability principles because it "seems simple" or "works fine."
**Why it's wrong**: Essential Floor categories exist because these are areas where missing governance causes the most damage. A working codebase without security principles will eventually have a security incident.
**Fix**: Always include all four categories. If the codebase lacks capability in an area, write the principle with "MUST implement" and reference a roadmap gap.

### Mistake 2: Codifying bad patterns from the codebase

**Problem**: Treating all existing patterns as worth preserving. The Emergent Ceiling captures whatever is in the codebase, including anti-patterns.
**Why it's wrong**: Some patterns are historical accidents, workarounds, or technical debt. Codifying them locks in bad practices.
**Fix**: Only codify patterns that are **intentionally good**. Ask: "Would I recommend this pattern for a new project?" If no, it's technical debt, not an Emergent Ceiling pattern.

### Mistake 3: Skipping codebase analysis

**Problem**: Writing the brownfield constitution without first consuming `.mochiko/memory/codebase-analysis.md`.
**Why it's wrong**: Without the analysis, the process devolves into guessing. Good patterns get missed and Essential Floor status assessments become inaccurate.
**Fix**: Always work from the analysis output — its "Strengths to Preserve" (Emergent Ceiling input) and gap identification (Essential Floor status).

### Mistake 4: Writing aspirational instead of enforceable principles

**Problem**: Writing principles like "Code SHOULD be clean" or "Security SHOULD be considered" without concrete enforcement.
**Why it's wrong**: These are unenforceable and untestable. They provide no governance value and will be ignored.
**Fix**: Apply the shared-core Three-Part Rule: specific behavior, enforcement mechanism, and rationale. "Security SHOULD be considered" becomes "Authentication MUST use JWT with rotation, enforced by middleware check, because centralized auth prevents bypass."

### Mistake 5: Ignoring Evolution Notes

**Problem**: Creating the constitution without documenting the brownfield context and gap status.
**Why it's wrong**: Future maintainers won't know which principles reflect existing capability vs. aspirational targets. They can't prioritize improvements.
**Fix**: Always include the Evolution Notes section with the Essential Floor status table. This makes gaps visible and actionable.

---

## Related (cross-cluster stubs — referenced, not mounted)

- **Independent validation** — after authoring, the constitution is graded by `validation-constitution`, run by a **separate validator agent** (never co-mounted with this skill). The enforced produce→validate→revise loop and the human acceptance gate are owned by the `setup` command lead.
- **`analysis-codebase`** (in-cluster) — produces `.mochiko/memory/codebase-analysis.md`, the brownfield-mode input. Run before brownfield authoring (lead-sequenced).
- **`syncing-claude-md`** (documented stub, *moved-to-other-cluster*) — performs the operational CLAUDE.md propagation. **Not ported this run.** The CLAUDE.md Synchronization *section spec* stays here as constitution content; the execution skill is referenced as a planned stub, not a live mount.
- **`authoring-roadmap` / `evolution-roadmap`** (documented stub, *moved-to-other-cluster*) — turn the brownfield gap status into an improvement plan at `.mochiko/memory/evolution-roadmap.md`. **Not ported this run.** The Evolution Notes section spec stays; the roadmap producer is a planned stub.
