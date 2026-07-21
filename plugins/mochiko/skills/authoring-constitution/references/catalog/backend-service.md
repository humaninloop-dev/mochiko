# Shelf: Backend / Service

Dealt when the declared project type is **backend, service, or fullstack (API side)**. These were
mochiko's former universal greenfield defaults; they are now type-selected cards — good
architecture for services, misfitting baggage for an SPA or a CLI. Each card pairs with the
`layer-rules` template module where noted.

Cards carry their full principle material here (statement, enforcement, testability, rationale
inputs). The session arbitrates: keep / tighten / drop / re-rank — every ruling recorded in the
synthesis with a trace-ID.

---

### BE-HEX — Hexagonal Architecture (Ports & Adapters)

**Type tags:** backend, service, fullstack-api
**Tier defaults:** poc: out · internal: offer · production: default-in · regulated: default-in
**Tier parameterization:** at `internal`, enforcement MAY be code-review-only; at `production`+, import-linter rules in CI are the default enforcement.
**Template module:** selecting this card selects the `layer-rules` module (Project Structure + Layer Import Rules sections).

**Content:**

The application core MUST be isolated from external concerns. Dependencies flow inward — outer
layers depend on inner layers, never reverse.

**Layer Structure:**

| Layer | Purpose | Location | MAY Import | MUST NOT Import |
|-------|---------|----------|------------|-----------------|
| **Domain** | Business logic, entities, value objects | `src/domain/` | Standard library + approved domain deps | application, adapters, infrastructure |
| **Application** | Use cases, orchestration, port definitions | `src/application/` | domain, port interfaces | adapters, infrastructure |
| **Adapters** | External system integration (DB, APIs, UI) | `src/adapters/` | application, domain, ports | other adapters directly |
| **Infrastructure** | DI wiring, configuration, entry points | `src/infrastructure/` | all layers (for wiring) | domain logic directly |

**Approved Domain Dependencies** — the domain layer MAY import libraries from the project's
approved domain dependencies registry when they meet both qualification criteria:
**domain-relevance** (domain modeling without I/O) and **ubiquity** (>80% adoption —
effectively an ecosystem standard). Domain-relevance filters candidates FIRST; trust signals
rank what passes — the seed arbitration, trust-signal hierarchy, and implement-time growth
process are single-sourced in [../DOMAIN-DEPENDENCIES.md](../DOMAIN-DEPENDENCIES.md). Common
approved libraries: `pydantic`/`attrs` (Python), `zod`/`decimal.js`/`uuid` (TypeScript),
`go-playground/validator`/`shopspring/decimal` (Go), `serde`/`rust_decimal` (Rust),
`equatable`/`decimal` (Dart). The registry lives in the domain-layer rules file's marked
registry block (see the `layer-rules` module).

**Port Interface Requirements** — all external service interactions MUST go through port
interfaces: defined as `Protocol` (Python) / `interface` (TypeScript, Go) / `trait` (Rust); domain
types in signatures, not SDK types; adapters implement ports; one port per logical capability
(`StoragePort`), not per provider.

**Enforcement:** import linter configured for layer rules (`import-linter`, ESLint
`import/no-restricted-paths`, `go-cleanarch`, ArchUnit); CI blocks merge on inner→outer imports and
on unapproved domain imports; code review verifies boundaries and new-dependency qualification;
strict type-checking catches type leakage.

**Testability:** Pass — all imports respect layer rules; all use cases testable with mock adapters
(no real DB/API). Fail — domain imports from adapters OR application imports infrastructure OR
unapproved domain dependency.

**Rationale:** Hexagonal architecture isolates business logic from infrastructure concerns —
enabling testing without real backends, swapping implementations, and reasoning about business
rules without infrastructure noise. The cost is abstraction; the benefit is long-term
maintainability.

---

### BE-SRP — Single Responsibility & Module Boundaries

**Type tags:** backend, service, fullstack-api
**Tier defaults:** poc: out · internal: offer · production: default-in · regulated: default-in
**Tier parameterization:** metric limits are session-tunable; at `internal` the complexity gate MAY be a linter warning rather than a CI block.

**Content:**

Each module, class, and function MUST have one clear purpose. When a component has multiple
reasons to change, split it.

**Module Responsibility Rules:** domain handles business logic, not infrastructure; adapters
handle integration, not business logic; models define shape, behavior lives in services/use cases;
no "utils"/"helpers" dumping grounds — find the right home or create a named module.

**Code Quality Metrics (defaults, session-tunable):**

| Metric | Limit | Enforcement |
|--------|-------|-------------|
| Cyclomatic complexity | ≤10 per function | Linter rule, CI blocks |
| Function parameters | ≤5 (use models beyond) | Code review |
| Function length | ≤40 SHOULD, ≤60 MUST | Linter warning/error |
| File length | ≤300 SHOULD, ≤500 MUST | Code review |
| Nesting depth | ≤4 | Code review |

**Enforcement:** linter complexity rule (e.g. `max-complexity = 10`); CI blocks on violations; code
review rejects mixed responsibilities; periodic boundary review for drift.

**Testability:** Pass — all functions ≤10 complexity, each module testable in isolation. Fail —
complexity >10 OR file >500 lines without a recorded exception.

**Rationale:** Single responsibility makes code easier to test, understand, and modify. Mixed
responsibilities compound complexity over time. A module that cannot be named in one sentence is
doing too much.

---

### BE-DEP — Dependency Discipline

**Type tags:** backend, service, fullstack-api
**Tier defaults:** poc: out · internal: offer · production: default-in · regulated: default-in
**Tier parameterization:** at `regulated`, add license compliance and a documented supply-chain review; vulnerability blocking severity may tighten (high/critical → medium+).

**Content:**

External dependencies MUST be justified, minimal, and isolated. Every dependency is a liability —
maintenance burden, security surface, potential breaking changes.

**Dependency Rules:** new dependencies justified in the PR description; versions pinned in lock
files; external service calls through port interfaces (pairs with BE-HEX); transitive dependencies
audited for known vulnerabilities.

**Evaluation criteria (red flags):** reasonably implementable in-house at <100 lines; no commits in
12+ months; unpatched known vulnerabilities; >10 transitive deps; incompatible license.

**Enforcement:** dependency scanner in CI (`pip-audit`, `npm audit`, `cargo audit`,
`govulncheck`); CI blocks merge on high/critical vulnerabilities; lock file committed; code review
verifies justification.

**Testability:** Pass — all dependencies pinned, zero high/critical vulnerabilities, external calls
via ports. Fail — unpinned dependency OR critical vulnerability OR direct SDK usage in domain.

**Rationale:** Dependencies are borrowed code with borrowed problems. Each one increases attack
surface, maintenance burden, and upgrade complexity. Isolation via ports enables swapping
implementations without rewriting business logic.

---

## Stack tooling map (for real commands)

The session's existing-practices dimension supplies actual tools; this map covers the common
stacks when nothing is detected:

| Stack | Import Linter | Complexity | Dependency Audit |
|-------|---------------|------------|------------------|
| Python | `import-linter` | `ruff` (C901) | `pip-audit` |
| TypeScript | ESLint `import/no-restricted-paths` | ESLint `complexity` | `npm audit` |
| Go | `go-cleanarch` | `gocyclo` | `govulncheck` |
| Rust | clippy rules | `clippy` | `cargo audit` |
| Java | ArchUnit | SonarQube | OWASP Dependency-Check |
