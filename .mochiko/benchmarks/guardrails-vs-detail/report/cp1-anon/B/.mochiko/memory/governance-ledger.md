# Governance Ledger

**Governance Floor:** production (asserted) · **Modules (compliance):** none · **Trace:** GI-001 (fact profile)
**Version:** 0.1.0 (must match the region stamp)

_Template modules adopted (recorded in the Module & exclusion record below, not the compliance-modules field above): knowledge-management (core) · release-gates · RUNBOOK (KM elective)._

## Waivers

Any asserted standard, with a recorded justification (D4); permanent pending the D4.1 revisit.
**Legal-mandate module obligations are unwaivable (D4.2).**

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| None. | | | |

*No floor category was taken below its asserted level. The observability-target and data-retention deferrals (GI-017, GI-018) are exclusions of scope beyond the floor, not floor waivers.*

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and un-waives are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change / module attach or detach · MINOR — new principle or waiver change · PATCH — clarification.
- Approvers: the founder (solo). Enforcement leans on CI and tooling, never on a second human reviewer (team reality: solo + ~5h/week part-time UI help).

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| Unpatchable transitive CVE may be acknowledged-and-proceeded after triage (recorded here per instance), rather than hard-blocking indefinitely | GI-009 | policy established at ratification; individual grants logged here as they occur | revisit each grant when an upstream fix ships |

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-003 — Security by default · home: CLAUDE.md (+ rules/mochiko/api-security.md, frontend-security.md)

**Enforcement**:
- CI runs `gitleaks detect` and blocks merge on any secret finding.
- CI runs `pip-audit` and blocks merge on high/critical vulnerabilities (see GI-009).
- Secrets loaded only from environment variables; `.env*` and local config in `.gitignore`.
- Auth enforced at API boundaries and tenant isolation checked on every data-access path (rules/mochiko/api-security.md).
- Frontend: no secret keys in the bundle; input validation / output encoding on API-touching UI (rules/mochiko/frontend-security.md).

**Testability**:
- Pass: zero secrets detected, zero high/critical CVEs (or a recorded GI-009 exception), no unauthenticated endpoint, no cross-tenant access in tests, no secret key in the built frontend bundle.
- Fail: any secret detected OR high/critical CVE without exception OR unauthenticated endpoint OR cross-tenant data access OR a secret key shipped to the client.

**Rationale**: The product holds contractors' financial and contact data; a leak or an unauthenticated path is the #2 ranked risk. Automated scanning + boundary auth catch the common failures before merge, which suits a solo team that cannot lean on human review.

**Trace**: GI-003 (floor-asserted: FLOOR-SEC)

### GI-004 — Testing discipline · home: CLAUDE.md (quality gates)

**Enforcement**:
- CI runs `pytest --cov=. --cov-fail-under=60` and blocks merge on failure or coverage <60%.
- Coverage ratchet: CI compares to the stored baseline and blocks a decrease.
- A payment-state smoke test exists and runs in CI from day one.

**Testability**:
- Pass: all tests pass AND coverage ≥60% AND coverage ≥ previous baseline AND the payment-state smoke test is present.
- Fail: any test fails OR coverage <60% OR coverage decreased OR no payment-state smoke test.

**Rationale**: Tests are the cheapest correctness insurance for a solo founder shipping financial software. Greenfield ratchet starts from zero and climbs; the 60% blocking / 80% warning split balances rigor with pragmatism. Timing concern resolved at intent review: the gate bites only once real code exists.

**Trace**: GI-004 (floor-asserted: FLOOR-TEST)

### GI-005 — Error handling · home: rules/mochiko/error-handling.md (+ CLAUDE.md index line)

**Enforcement**:
- API tests assert error responses match the RFC 7807 schema and carry a correlation id.
- Code review verifies external calls leave state consistent on failure.
- The React UI renders explicit error states (component tests).

**Testability**:
- Pass: all error responses match schema with correlation ids, no stack trace exposed, UI shows an error state on failure, no half-applied state after an induced failure.
- Fail: any error response missing required fields OR stack trace exposed OR silent data corruption on failure OR blank UI on error.

**Rationale**: "Wrong data that looks right" is the founder's stated worst case; the never-silently-corrupt clause is load-bearing for payment integrity. A consistent error surface enables client handling and debugging.

**Trace**: GI-005 (floor-asserted: FLOOR-ERR)

### GI-006 — Observability · home: CLAUDE.md

**Enforcement**:
- Structured (JSON) logging via a shared logger with correlation ids; code review verifies no PII in log statements.
- A `/health` endpoint exists and is checked in integration tests.

**Testability**:
- Pass: errors logged with context and correlation ids, no PII in logs, `/health` responds.
- Fail: silent failures OR PII in logs OR missing correlation ids OR no health endpoint.

**Rationale**: A solo operator on Render needs enough signal to diagnose production issues without a dedicated ops function. Numeric SLO/incident targets are deferred (GI-017) — presence, not targets, is the floor.

**Trace**: GI-006 (floor-asserted: FLOOR-OBS)

### GI-007 — Ports around external systems (hexagonal-lite) · home: rules/mochiko/architecture.md

**Enforcement**:
- Business logic accesses Stripe and the DB only through port interfaces; code review verifies no direct SDK/ORM calls from business logic.
- Use cases are tested against fake adapters (no real Stripe/DB in unit tests).

**Testability**:
- Pass: payment-state logic has unit tests running against fakes; no SDK/ORM import in business-logic modules.
- Fail: business logic calls Stripe/DB directly OR payment logic cannot be tested without real infrastructure.

**Rationale**: The founder's #1-ranked safety net — testing "invoice flips to paid only when Stripe confirms" against a fake Stripe. Kept in the lite form; the strict 4-layer import-linter was dropped (GI-016) as ceremony a solo dev won't maintain.

**Trace**: GI-007 (deck-kept: BE-HEX, tightened to lite)

### GI-008 — Single responsibility & complexity limits · home: rules/mochiko/architecture.md (+ CLAUDE.md quality gate)

**Enforcement**:
- `ruff check .` with C901 complexity ≤10 as a CI-blocking rule.
- Code review checks module responsibility boundaries; param-count/file-length limits are advisory.

**Testability**:
- Pass: all functions ≤10 cyclomatic complexity; each module has one nameable responsibility.
- Fail: any function >10 complexity without a recorded exception.

**Rationale**: Automated complexity gating is cheap and needs no human ceremony — the founder accepted the automated parts and demoted the stylistic size limits to advisory so a build never fails "over style."

**Trace**: GI-008 (deck-kept: BE-SRP, automated parts only)

### GI-009 — Dependency discipline · home: CLAUDE.md (quality gate) + exception registry

**Enforcement**:
- Dependencies pinned in a lock file; `pip-audit` in CI blocks merge on high/critical CVEs.
- Unpatchable transitive CVE: acknowledge-and-proceed after triage via a recorded exception (registry above).

**Testability**:
- Pass: lock file committed, zero high/critical CVEs OR each such CVE carries a recorded exception.
- Fail: unpinned dependency OR high/critical CVE with no exception.

**Rationale**: Matches the founder's security instinct; the escape hatch prevents an unfixable upstream CVE from permanently wedging shipping.

**Trace**: GI-009 (deck-kept: BE-DEP, with escape hatch)

### GI-010 — Payment-state correctness · home: rules/mochiko/financial-correctness.md

**Enforcement**:
- For Stripe-collected payments, state only transitions on a confirmed Stripe signal; webhooks are recorded, never silently dropped.
- Manual mark-as-paid is a supported transition carrying GI-021 idempotency + GI-022 audit trail.
- Payment-state tests (Stripe path, manual path, duplicate webhook) block CI.

**Testability**:
- Pass: no path marks a Stripe-collected invoice paid without confirmation; manual mark-as-paid works and is audited; the CI payment-state suite passes.
- Fail: an invoice can be marked paid without Stripe confirmation on the Stripe path OR a webhook is dropped silently OR the payment-state suite is absent/failing.

**Rationale**: The founder's #1 non-negotiable — a tool that lies about payment state loses contractors. Scoped at ratification so the product's own manual (check/cash) close path is first-class, not a violation.

**Trace**: GI-010 (minted)

### GI-011 — Tenant data isolation · home: rules/mochiko/api-security.md

**Enforcement**:
- Every data-access path applies an ownership/tenant check; tests assert a contractor cannot read another's data.

**Testability**:
- Pass: cross-tenant access attempts fail in tests on every resource (clients, invoices, payments).
- Fail: any resource readable across tenants.

**Rationale**: The data-handling item the founder most wanted enforceable — contractors see only their own financial data.

**Trace**: GI-011 (minted)

### GI-020 — Money is exact decimal, never float · home: rules/mochiko/financial-correctness.md

**Enforcement**:
- Monetary values use exact decimal (`Decimal` / integer minor units); code review + tests reject float money; rounding explicit at display.

**Testability**:
- Pass: arithmetic tests (e.g. 0.10 + 0.20 == 0.30) hold; no float type on monetary fields.
- Fail: any monetary value stored/computed as float.

**Rationale**: Float money is a classic correctness defect directly under the #1 risk; cheap and non-negotiable per the founder.

**Trace**: GI-020 (minted, intent-review survivor S1)

### GI-021 — Payment-state transitions are idempotent · home: rules/mochiko/financial-correctness.md

**Enforcement**:
- Each Stripe event id processed at most once (recorded/deduplicated); re-applying an event is a no-op. Manual path equally safe.
- A CI-blocking test proves a redelivered duplicate event does not double-apply.

**Testability**:
- Pass: replaying a processed Stripe event causes no additional state change; double mark-as-paid does not double-count.
- Fail: a duplicate event or repeated manual mark-as-paid changes state twice.

**Rationale**: Stripe delivers at-least-once; without idempotency "never dropped" degrades into "silently applied twice." Mechanism (processed-events table) left to implementation; the guarantee is the principle.

**Trace**: GI-021 (minted, reopen-born intent-review survivor S2)

### GI-022 — Payment-state-change audit trail · home: rules/mochiko/financial-correctness.md

**Enforcement**:
- Every payment-state change records actor + timestamp; the manual mark-as-paid path especially.

**Testability**:
- Pass: each state change produces an audit record with who + when.
- Fail: any state change (esp. manual) with no audit record.

**Rationale**: The manual path has no Stripe record behind it; if a contractor disputes what happened, the audit log is the only evidence.

**Trace**: GI-022 (minted, intent-review survivor S3)

## Release Gates

**Environments:** local → Render (single production environment; no separate staging this early).
**Cadence:** ship-when-ready (no fixed schedule pre-launch).

| Gate | Requirement | Verified by | Blocks |
|------|-------------|-------------|--------|
| Green CI | tests pass, coverage ≥60%, lint clean, secret scan clean, dep scan clean | GitHub Actions | merge + release |
| Payment-state suite | Stripe, manual, and duplicate-webhook tests pass | GitHub Actions (`pytest`) | merge + release |
| Migration check | DB migrations backward-compatible so a rollback does not strand the schema | code review | deploy |

### Rollback

- Rollback procedure MUST be documented and executable by the founder: redeploy the previous commit/image from the Render dashboard (see `RUNBOOK.md`).
- Rollback time expectation: previous version restored in ≤15 minutes.
- Releases that cannot be rolled back (e.g. a destructive migration) MUST be flagged in the PR and approved explicitly.

**Trace**: GI-013 (module selection: release-gates)

## Module & exclusion record

- **GI-012 — knowledge-management (core):** adopted whole; scaffolded (see project-pinned copy at `.mochiko/memory/knowledge-management.md`).
- **GI-013 — release-gates:** adopted (section above).
- **GI-014 — RUNBOOK (KM elective):** adopted as a growing stub (`RUNBOOK.md`).
- **GI-015 — CHANGELOG (KM elective):** declined; revisit at launch.
- **GI-016 — layer-rules:** declined (BE-HEX kept lite; import-linter enforcement dropped).
- **GI-017 — Observability numeric SLO/incident targets:** deferred (beyond-floor; presence floor GI-006 holds).
- **GI-018 — Data retention & deletion policy:** deferred.
- **GI-019 — SOC 2:** out of scope; revisit if a customer requires it.
- **GI-023 — Reminder email provider + failure handling:** deferred to specify/plan.
- **GI-024 — Frontend floor beyond the two firmed security clauses:** known-thin spot (not a waiver).
- **GI-025 — US state privacy (CCPA/CPRA):** deferred (under thresholds; solo pre-launch).

## Trace summary

| GI-ID | Principle | Source | Primary home | Companions present |
|-------|-----------|--------|--------------|--------------------|
| GI-003 | Security by default | floor-asserted: FLOOR-SEC | CLAUDE.md region line | index ✓ · ledger ✓ · rules (api-security, frontend-security) ✓ |
| GI-004 | Testing discipline | floor-asserted: FLOOR-TEST | CLAUDE.md quality gates | index ✓ · ledger ✓ |
| GI-005 | Error handling | floor-asserted: FLOOR-ERR | rules/mochiko/error-handling.md | index ✓ · ledger ✓ |
| GI-006 | Observability | floor-asserted: FLOOR-OBS | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-007 | Ports (hexagonal-lite) | deck-kept: BE-HEX | rules/mochiko/architecture.md | index ✓ · ledger ✓ |
| GI-008 | SRP & complexity | deck-kept: BE-SRP | rules/mochiko/architecture.md | index ✓ · ledger ✓ |
| GI-009 | Dependency discipline | deck-kept: BE-DEP | CLAUDE.md quality gate | index ✓ · ledger ✓ · exception registry ✓ |
| GI-010 | Payment-state correctness | minted | rules/mochiko/financial-correctness.md | index ✓ · ledger ✓ |
| GI-011 | Tenant data isolation | minted | rules/mochiko/api-security.md | index ✓ · ledger ✓ |
| GI-020 | Money exact decimal | minted | rules/mochiko/financial-correctness.md | index ✓ · ledger ✓ |
| GI-021 | Idempotent transitions | minted | rules/mochiko/financial-correctness.md | index ✓ · ledger ✓ |
| GI-022 | Payment-state audit trail | minted | rules/mochiko/financial-correctness.md | index ✓ · ledger ✓ |
| GI-012 | knowledge-management | module | CLAUDE.md pointer | index ✓ · ledger ✓ · pinned copy ✓ |
| GI-013 | release-gates | module | CLAUDE.md summary line | index ✓ · ledger ✓ |

Flagged proposals: none
Waivers: none

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| 0.1.0 | 2026-08-10 | ratified (greenfield) | GI-001…GI-025 |
