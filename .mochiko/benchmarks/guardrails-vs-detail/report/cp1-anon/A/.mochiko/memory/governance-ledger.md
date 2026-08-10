# Governance Ledger

**Governance Floor:** production (asserted) · **Modules:** none (compliance) — template modules adopted: knowledge-management (core + CHANGELOG elective), layer-rules, release-gates · **Trace:** GI-001 (fact profile)
**Version:** 1.0.0 (matches the CLAUDE.md region stamp)

## Waivers

Any asserted standard, with a recorded justification (D4); permanent pending the D4.1 revisit.
**Legal-mandate module obligations are unwaivable (D4.2).**

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| FLOOR-TEST numeric coverage-percentage gate (≥80% warning / ≥60% blocking) | Solo founder refuses coverage-percentage theater; risk-targeted testing preferred. The category is NOT dropped — replaced by a required critical-path test rule (payment state machine, auth, tenant isolation MUST have tests, CI-enforced), with the coverage ratchet and a day-one smoke test retained. | Team grows past solo, OR a SOC 2 / attestation obligation is signed | GI-014 |

## Watches (fact-profile temporal backstops — not waivers)

| Watch | Consequence when tripped | Trace |
|-------|--------------------------|-------|
| A signed SOC 2 / ISO 27001 / customer security-addendum obligation | `attestation` module attaches (amend run, governance event) | GI-001 |
| Any feature that would collect or proxy card data on Ledgerline servers | PCI SAQ-A eligibility lost; `pci-dss` module attaches (amend run, governance event) | GI-026 |
| Crossing US-state-privacy applicability thresholds (CCPA/CPRA, VA/CO/CT), or new consumer-facing data collection | State-privacy obligation reassessed; a privacy module may attach | GI-001 |
| Data retention & deletion policy (OPEN QUESTION, GI-022) | Founder must source the US financial-record retention requirement before it bites; not a launch gate today | GI-022 |

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and un-waives are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change / module attach or detach · MINOR — new principle or waiver change · PATCH — clarification.
- Approvers: the founder (solo; no second reviewer — CI is the enforcer).

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| none yet | | | |

## Domain-dependency policy (layer-rules adopted)

Qualification criteria (domain-relevance filters first, then ubiquity >80%) and the trust-signal
hierarchy: `authoring-constitution/references/DOMAIN-DEPENDENCIES.md`. Add-process + gate: a
proposed addition meeting both criteria surfaces to the human as an explicit ruling BEFORE
entering the registry; the implementation checkpoint MUST NOT auto-approve while a cycle's
`domain_deps_added` is non-empty. The registry list itself lives ONLY in the
`mochiko:domain-registry` block of `.claude/rules/mochiko/domain-dependencies.md` — preserved
verbatim across regenerations; no ledger copy exists.

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-003 — Security by Default · home: CLAUDE.md

**Enforcement**:
- CI runs `gitleaks detect` (secret scanning) and blocks merge on findings.
- CI runs `pip-audit` / `npm audit` and blocks on high/critical vulnerabilities.
- Input validation at all boundaries via pydantic (API) and zod (UI); auth enforced at every API boundary.

**Testability**:
- Pass: zero secrets in the repo, zero high/critical vulnerabilities, auth on every API endpoint, external inputs validated.
- Fail: any secret detected OR high/critical vulnerability OR an unauthenticated endpoint OR an unvalidated boundary input.

**Rationale**: A breach of contractors' financial and contact data is the founder's #1 worst-case (trust gone, business dead). The founder is self-described weak on security, so the pipeline — not manual review — carries the enforcement.

**Trace**: GI-003 (floor-asserted: FLOOR-SEC)

### GI-004 — Testing Discipline (critical-path) · home: CLAUDE.md

**Enforcement**:
- CI runs `pytest` (backend) and `npm test` (frontend); required tests exist for the payment state machine, auth, and tenant-isolation paths.
- Coverage ratchet: CI fails if coverage decreases from the baseline. No numeric coverage-percentage gate (waived — GI-014).
- A day-one critical-path smoke test runs in CI.

**Testability**:
- Pass: all tests green AND the payment/auth/tenant-isolation paths have tests AND coverage ≥ previous baseline.
- Fail: any test fails OR a change to a critical path lacks a test OR coverage decreased.

**Rationale**: "I'd rather ship late than ship a payment bug." Risk-targeted tests on the paths that break money or leak data beat a coverage vanity number for a solo founder.

**Trace**: GI-004 (floor-asserted: FLOOR-TEST) · waiver GI-014

### GI-005 — Error Handling Standards · home: CLAUDE.md

**Enforcement**:
- API error responses follow RFC 7807 Problem Details with correlation/trace IDs; schema validated in API tests.
- Code review / tests verify external calls (Stripe, DB) are wrapped so failures never silently corrupt state — especially payment state.
- No stack traces or customer data in error responses (ties GI-013).

**Testability**:
- Pass: all error responses match the schema, carry correlation IDs, and expose no stack trace or customer data; no failure path silently mutates payment state.
- Fail: any error response missing required fields OR a leaked stack trace OR a silent-corruption path.

**Rationale**: Wrong payment state (invoice shows paid when unpaid) is a top failure the founder named; consistent, non-corrupting error handling protects the contractor's reputation with their own clients.

**Trace**: GI-005 (floor-asserted: FLOOR-ERR)

### GI-006 — Observability Requirements · home: CLAUDE.md

**Enforcement**:
- Structured JSON logs with standard fields and correlation IDs, enforced by a logging wrapper.
- A `/health` endpoint (FastAPI) verified in integration tests.
- Sentry for error tracking; no PII in logs or payloads (GI-013).

**Testability**:
- Pass: errors logged with context and correlation IDs, `/health` responds, no PII in logs.
- Fail: silent failures OR missing correlation IDs OR PII in logs.

**Rationale**: A one-person shop cannot diagnose production issues without structured logs and error tracking. SLOs and formal targets are deliberately out of scope for now (GI-020).

**Trace**: GI-006 (floor-asserted: FLOOR-OBS)

### GI-007 — Layered (Hexagonal) Architecture · home: .claude/rules/mochiko/architecture-layers.md

**Enforcement**:
- `lint-imports` (import-linter) in CI blocks inner→outer imports and unapproved domain imports.
- Stripe and PostgreSQL behind port interfaces; use cases testable with mock adapters.

**Testability**:
- Pass: all imports respect layer rules; payment-state and use-case logic testable without real Stripe or DB.
- Fail: domain imports adapters/infrastructure OR application imports infrastructure OR an unapproved domain import.

**Rationale**: The one place the founder pays the upfront-structure tax — it buys tested payment correctness without hitting real Stripe, the founder's #1 concern.

**Trace**: GI-007 (deck-kept: BE-HEX) · selects layer-rules module

### GI-008 — Single Responsibility & Complexity · home: .claude/rules/mochiko/architecture-layers.md

**Enforcement**:
- `ruff` C901 complexity rule (≤10) blocks CI; no "utils/helpers" dumping grounds (verified at authoring/refactor time).
- File length, parameter count, nesting depth are advisory (no CI gate — no reviewer to arbitrate).

**Testability**:
- Pass: all functions ≤10 cyclomatic complexity; each module has one nameable purpose.
- Fail: complexity >10 without a recorded exception.

**Rationale**: The complexity gate is cheap and CI-automated; the softer metrics are left advisory because the founder refuses bikeshedding with no reviewer present.

**Trace**: GI-008 (deck-kept: BE-SRP)

### GI-009 — Dependency Discipline · home: CLAUDE.md (quality gate)

**Enforcement**:
- `pip-audit` / `npm audit` in CI block merge on high/critical vulnerabilities; lock files committed.
- New dependencies justified in the PR description; external calls via ports (pairs with GI-007).

**Testability**:
- Pass: all dependencies pinned, zero high/critical vulnerabilities, external calls via ports.
- Fail: an unpinned dependency OR a high/critical vulnerability OR direct SDK usage in the domain layer.

**Rationale**: "Exactly the security corner I'm weak on — I want the pipeline doing this for me." Automated dependency hygiene shores up the founder's self-identified gap.

**Trace**: GI-009 (deck-kept: BE-DEP)

### GI-010 — Payment-State Integrity · home: .claude/rules/mochiko/payments.md

**Enforcement**:
- CI blocks any change to invoice status / payment amounts lacking a state-machine test.
- Webhook handlers are idempotent (tested against duplicate events); a scheduled reconciliation against Stripe recovers missed webhooks; manual mark-as-paid logs actor + timestamp.

**Testability**:
- Pass: duplicate Stripe events do not double-count or double-notify; a dropped webhook is recovered by reconciliation; no code path flips "paid" without a Stripe event or a logged manual transition.
- Fail: a non-idempotent handler OR a guessed status flip OR a status/amount change without a test OR no missed-webhook recovery.

**Rationale**: "I never want the app's idea of 'paid' to drift from Stripe's." Payment-state drift is a top-two failure mode for the product.

**Trace**: GI-010 (minted)

### GI-011 — Currency as Integer Cents · home: .claude/rules/mochiko/payments.md

**Enforcement**:
- Money represented as integer cents throughout; code review / type checks reject float currency types; lint/tests catch float arithmetic on money values.

**Testability**:
- Pass: all monetary values are integer cents; no floating-point currency arithmetic.
- Fail: any float type or float arithmetic on a currency value.

**Rationale**: "Money math is exact, no floats for currency." Prevents rounding errors in financial records.

**Trace**: GI-011 (minted)

### GI-012 — Tenant Isolation · home: .claude/rules/mochiko/data-access.md

**Enforcement**:
- Every data query scoped to the owning contractor at the repository/adapter layer; required tenant-isolation tests in CI (a critical path under GI-004).

**Testability**:
- Pass: no query returns another contractor's data; cross-tenant access is covered by a failing-then-passing test.
- Fail: any unscoped query OR a use case that can read/affect another account's data OR missing tenant-isolation tests.

**Rationale**: "One account seeing another's invoices is the nightmare." Multi-tenant financial data demands hard isolation.

**Trace**: GI-012 (minted)

### GI-013 — No Customer Data in Logs or Telemetry · home: CLAUDE.md

**Enforcement**:
- Logging wrapper and Sentry integration strip/forbid PII (names, emails, addresses) and invoice amounts; code review / tests verify no customer data in log lines or error payloads.

**Testability**:
- Pass: no PII or invoice amounts in any log line or telemetry payload.
- Fail: any customer PII or amount in a log or Sentry payload.

**Rationale**: A hard rule the founder named — customer financial data must not leak through observability. Strengthens GI-005/GI-006.

**Trace**: GI-013 (minted)

### GI-018 — Approved Domain Dependencies · home: .claude/rules/mochiko/domain-dependencies.md

**Enforcement**:
- The registry block in the domain rules file is the sole allowlist; `lint-imports` blocks unapproved domain imports; additions require an explicit human ruling before entry, disclosed via `domain_deps_added`.

**Testability**:
- Pass: every domain third-party import is a registry entry meeting both criteria; no unruled additions.
- Fail: an unapproved domain import OR an addition that skipped the gate.

**Rationale**: An empty registry reads as prohibition; a curated seed set (pydantic, zod) admits the ecosystem standards domain code actually needs while keeping the boundary explicit.

**Trace**: GI-018 (module: layer-rules — domain registry) · seeds GI-018 (`pydantic`), GI-019 (`zod`)

### GI-024 — Data Durability & Recoverability · home: CLAUDE.md

**Enforcement**:
- Managed Postgres (Render) automated daily backups with point-in-time recovery enabled; a periodic restore check verifies backups restore.

**Testability**:
- Pass: daily backups present, PITR enabled, a documented periodic restore check has succeeded.
- Fail: backups disabled OR PITR off OR no restore ever verified.

**Rationale**: Data loss of contractors' financial records is the #1 worst-case; managed backups + PITR cost near-nothing on Render. (Folded from cold-review survivor S5.)

**Trace**: GI-024 (minted — cold-review fold S5)

### GI-025 — Accessibility (WCAG 2.1 AA, core flows) · home: .claude/rules/mochiko/accessibility.md

**Enforcement**:
- Core contractor flows (invoice create/send, payment-status view) meet WCAG 2.1 AA; accessibility checks (e.g. axe) run against those flows in CI/frontend tests.

**Testability**:
- Pass: core flows pass WCAG 2.1 AA checks (semantic markup, keyboard operability, contrast, labelled controls).
- Fail: a core-flow accessibility violation at AA level.

**Rationale**: Accessibility is worth doing and reduces real legal exposure. Adopted as a scopable standard (not an unwaivable legal mandate) so a solo founder can sequence it — the WCAG 2.1 AA commitment on core flows stands. (Reclassified from legal-mandate by cold-review survivor S3.)

**Trace**: GI-025 (deck-kept: DECK-1, reclassified — adopted standard, waivable D4)

### GI-026 — Card Data Never Touches Our Servers (PCI SAQ-A) · home: CLAUDE.md

**Enforcement**:
- Stripe-hosted checkout only; no custom card form or card-data proxy. Any PR that would collect/store card data is a governance event (see Watches).

**Testability**:
- Pass: no code path receives, stores, or transmits raw card data; checkout is Stripe-hosted.
- Fail: any endpoint or form handling raw card data.

**Rationale**: Keeps Ledgerline at the lightest PCI level (SAQ-A) and prevents a future feature from silently dragging it into full PCI scope. (Folded from cold-review survivor S2.)

**Trace**: GI-026 (minted — cold-review fold S2)

### GI-017 — Release Gates · home: CLAUDE.md

**Deployment reality (from the always-interrogated deployment dimension):**
- **Target:** Render (managed; lean choice, not locked).
- **Environments:** production only at launch; a staging environment is deferred (added later if it proves cheap on Render) — a recorded decision, not an omission.
- **Cadence:** continuous — a release ships whenever a change is ready; solo self-merge, no fixed schedule.

**Enforcement (the release-blocking bar):**
- A release is blocked unless ALL pass in GitHub Actions: `pytest` + `npm test` (tests), `ruff check .` + `ruff format --check .` + `npm run lint` (lint/format), `mypy .` (types). This is the "pipeline is the reviewer" gate — distinct from a per-merge check only in that it also blocks the deploy step, not just merge.

**Rollback:**
- Manual rollback: redeploy the last known-good release on Render. Expected recovery is minutes-scale at launch traffic (~200 contractors, low volume). Automated rollback is deferred past launch unless it is effectively free on Render — a recorded decision.

**Testability**:
- Pass: no release proceeds while tests, lint/format, or type checks are red; a bad deploy is recoverable by redeploying the last good release.
- Fail: a release shipped with any gate red, OR no known-good release to roll back to.

**Rationale**: "The pipeline is my reviewer." Formalizes the one-person release bar and keeps a low-traffic product recoverable without standing up rollback automation it does not yet need.

**Trace**: GI-017 (module: release-gates)

### GI-016 — Knowledge-Management (operating docs) · home: CLAUDE.md (pointer)

**Enforcement**:
- Operating-docs layer scaffolded (`.mochiko/brainstorms/` + `index.md`, `DECISIONS.md`, `BACKLOG.md`, `ROADMAP.md`, `CHANGELOG.md`); landing ritual + invariants at `.mochiko/memory/knowledge-management.md`; groom via `mochiko:grooming-operating-docs`.

**Testability**:
- Pass: the operating docs exist and closing work follows the landing ritual.
- Fail: decisions/open work living only in conversation context.

**Rationale**: Solo founder with no teammate to carry context — "if it's not written down it's gone." Core adopted whole; CHANGELOG elective adopted; RUNBOOK deferred until after first deploy (GI-016b / GI-023).

**Trace**: GI-016 (module: knowledge-management — core + CHANGELOG)

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| 1.0.0 | 2026-08-10 | ratified (greenfield) | GI-001 … GI-026 |
