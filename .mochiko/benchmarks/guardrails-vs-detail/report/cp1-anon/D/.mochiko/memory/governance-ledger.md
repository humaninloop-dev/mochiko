# Governance Ledger

**Governance Floor:** production (asserted) · **Modules:** `a11y` (WCAG — legal-mandate) · `layer-rules` · `knowledge-management` (core + RUNBOOK) · `release-gates` · **Trace:** GI-001 (fact profile)
**Version:** 0.1.0 (must match the region stamp)

## Waivers

Any asserted standard, with a recorded justification (D4); permanent pending the D4.1 revisit.
**Legal-mandate module obligations are unwaivable (D4.2)** — a waiver row naming one is a validator FAIL.

| Standard | Justification | Revisit trigger | Trace |
|----------|---------------|-----------------|-------|
| FLOOR-OBS depth — structured-logging tooling, correlation/trace IDs (beyond a money-path request ID), APM/dashboards, SLOs | Solo founder, pre-launch, ops-fuzzy; the heavy observability stack is not one-person maintainable today and would be governance theater. The cheap essentials (`/health`, no-PII-in-logs) are kept as a principle (GI-006); this waiver covers only the deferred depth so the floor stays honest. | At launch (first real users) or first production incident, whichever comes first | GI-012 |

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and un-waives are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change / module attach or detach · MINOR — new principle or waiver change · PATCH — clarification.
- Approvers: the founder (solo — no second reviewer; enforcement is automated by design).

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| none yet | | | |

## Domain-dependency policy (layer-rules adopted)

Qualification (both required): **domain-relevance** (domain modeling without I/O) filters first,
then **ubiquity** (>80% ecosystem adoption). Trust-signal hierarchy and live-verification process:
`authoring-constitution/references/DOMAIN-DEPENDENCIES.md`. Add-process: a proposed addition
surfaces to the founder as an explicit ruling BEFORE entering the registry; the implement-time
cycle checkpoint MUST NOT auto-approve while `domain_deps_added` is non-empty. The list itself lives
ONLY in the `mochiko:domain-registry` block in `.claude/rules/mochiko/architecture-layers.md` — no
ledger copy, nothing to drift.

## Release gates (release-gates module) <!-- GI-017 -->

**Environments:** local dev → production (no staging pre-launch — added later if it earns its keep).
**Cadence:** ship-when-ready pre-launch; small and frequent after launch.

| Gate | Requirement | Verified by | Blocks |
|------|-------------|-------------|--------|
| Build health | No red build ships to production | GitHub Actions status | deploy |
| Money-path tests | Invoice/tax/payment tests green | `pytest` (money-path suite) | merge + deploy |
| Migration check | Migrations apply cleanly on a fresh DB; destructive migrations flagged + approved | CI migration step | deploy |
| Accessibility | Frontend a11y check passes | `eslint-plugin-jsx-a11y` + axe-core | merge |

### Rollback

- Rollback procedure MUST be documented and executable by the founder: managed-host deploy rollback (see `RUNBOOK.md`); target restore-previous-version time ≤15 minutes.
- Code rollback is safe because migrations are forward-only + backward-compatible (expand-then-contract, GI-011).
- Managed Postgres backups (daily + point-in-time restore) MUST be enabled; restore steps in `RUNBOOK.md` (GI-023).
- A release that cannot be rolled back (a destructive migration) MUST be flagged in the PR and approved explicitly.

## Knowledge Management (module) <!-- GI-014 -->

Adopted: core (whole) + RUNBOOK elective. CHANGELOG declined (continuously-deployed SaaS, no
versioned releases — durable decline). Project-pinned contracts/invariants copy at
`.mochiko/memory/knowledge-management.md`; enforcement surfaces: this ledger note + the
`.claude/rules/mochiko/operating-docs.md` rules file + the CLAUDE.md pointers. Rules-file delivery
is probe-verified via `mochiko:testing-governance-injection`.

## Accessibility module (a11y) <!-- GI-024 -->

Stratum: **legal-mandate** — attached mechanically from the fact profile (customer-facing web UI +
US/ADA jurisdiction), **unwaivable** (D4.2). Standing obligation: WCAG 2.1 AA + an automated CI a11y
check; per-screen criteria are mint-driven as real screens exist.

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-003 — Security at boundaries · home: rules/mochiko/security.md + CLAUDE.md region

**Enforcement**:
- CI runs `gitleaks detect --no-banner` (blocks merge on any committed secret) and `pip-audit` (blocks on high/critical).
- Auth + tenant-scoping verified by integration tests that assert one account cannot read another's data.
- Input validation via pydantic models at every FastAPI boundary.

**Testability**:
- Pass: zero committed secrets, zero high/critical vulns, auth on all data routes, a cross-account access test fails to leak.
- Fail: any secret detected OR critical vuln OR an unauthenticated data route OR a query not account-scoped.

**Rationale**: A cross-account data leak (one contractor seeing another's invoices) is the founder's stated breach nightmare; automated scanning + tenant-isolation tests catch it before production.

**Trace**: GI-003 (floor-asserted: FLOOR-SEC)

### GI-004 — Testing discipline · home: CLAUDE.md quality gates

**Enforcement**:
- CI runs `pytest --cov --cov-fail-under=60`; coverage ratchet compares to baseline and blocks a decrease.
- `mypy ledgerline/domain` strict on the money/domain package.

**Testability**:
- Pass: all tests pass AND coverage ≥60% AND coverage ≥ previous baseline AND a money-path smoke test exists.
- Fail: any test fails OR coverage <60% OR coverage decreased.

**Rationale**: Tests enable confident change with no second reviewer; the ratchet prevents erosion. 80% warn / 60% block balances rigor with a solo founder's pace; the money path is held higher (GI-010).

**Trace**: GI-004 (floor-asserted: FLOOR-TEST)

### GI-005 — Error handling · home: rules/mochiko/error-handling.md

**Enforcement**:
- Integration tests verify a consistent JSON error shape and that no stack trace leaks.
- Multi-step writes wrapped in transactions; tests assert consistent state after an induced failure.

**Testability**:
- Pass: all error responses match the shape, no stack trace in any response, failed writes leave state consistent.
- Fail: any inconsistent error shape OR leaked stack trace OR a partial write on failure.

**Rationale**: Silent data corruption on the money path is the top risk; consistent errors + transactional integrity keep failures visible and non-destructive.

**Trace**: GI-005 (floor-asserted: FLOOR-ERR)

### GI-006 — Observability (floor essentials) · home: rules/mochiko/observability.md

**Enforcement**:
- Integration test hits `/health` and asserts a healthy response.
- Log review / lint guard that PII fields are never logged; a money-path request ID is attached.

**Testability**:
- Pass: `/health` responds, no PII in logs, money-path requests carry a request ID.
- Fail: missing/broken health endpoint OR PII found in logs.

**Rationale**: You cannot fix what you cannot see; the cheap essentials give minimum diagnosability now. Deeper depth is waived (GI-012) rather than pretended.

**Trace**: GI-006 (floor-asserted: FLOOR-OBS — partial; depth waived GI-012)

### GI-007 — Hexagonal (light) · home: rules/mochiko/architecture-layers.md

**Enforcement**:
- Import-linter (or ruff import rules) config enforces the layer import table in CI; blocks inner→outer imports and unapproved domain deps.
- Code review (self) verifies external systems sit behind ports.

**Testability**:
- Pass: all imports respect the layer table; the money path is testable with a fake Stripe (no network in tests).
- Fail: domain imports an adapter/SDK type OR an external call bypasses a port.

**Rationale**: Isolating Stripe and the money logic behind ports makes the invoicing core testable without the network — directly serving money correctness — while the light shape avoids four-layer ceremony the solo founder rejected.

**Trace**: GI-007 (deck-kept: BE-HEX)

### GI-008 — Single responsibility & complexity · home: rules/mochiko/code-quality.md

**Enforcement**:
- Linter complexity rule (`ruff` C901, max-complexity 10) blocks in CI; length limits warn.

**Testability**:
- Pass: all functions ≤10 complexity; each module has a nameable single purpose.
- Fail: complexity >10 without a recorded exception OR a mixed-responsibility dumping-ground module.

**Rationale**: Single responsibility keeps a one-person codebase understandable and testable; the CI cap needs no reviewer.

**Trace**: GI-008 (deck-kept: BE-SRP)

### GI-009 — Dependency discipline · home: rules/mochiko/dependencies.md

**Enforcement**:
- `pip-audit` in CI blocks on high/critical; lock file committed; new deps justified in the PR.

**Testability**:
- Pass: all deps pinned, zero high/critical vulns, external calls via ports.
- Fail: unpinned dep OR critical vuln OR direct SDK use in domain code.

**Rationale**: Every dependency is borrowed code with borrowed risk; pinning + audit keep the surface small and safe with no manual effort.

**Trace**: GI-009 (deck-kept: BE-DEP)

### GI-010 — Money correctness · home: rules/mochiko/money-domain.md

**Enforcement**:
- CI: the money-path test suite is blocking for merge AND deploy (no skip). `mypy` strict over `ledgerline/domain`.
- Reconciliation tests cover the missed-webhook case; a lint/review check that money math uses `Decimal`, never `float`.

**Testability**:
- Pass: money-touching code has tests, they pass, payment status matches Stripe after a simulated missed webhook, all money math uses Decimal.
- Fail: any money-touching change without a test OR a float in money math OR displayed status that can drift from Stripe.

**Rationale**: Getting the money wrong (paid-vs-unpaid drift, wrong totals) is the one failure the founder said the product cannot survive; a hard, no-exception automated gate is the control.

**Trace**: GI-010 (minted — dim 9: "money correctness … has to have tests, and if those tests fail the merge or deploy gets blocked — no exceptions")

### GI-011 — Migration safety · home: rules/mochiko/migrations.md

**Enforcement**:
- CI applies migrations to a fresh DB before deploy; a check flags destructive operations for explicit approval.
- Expand-then-contract convention enforced by review + the destructive-op flag.

**Testability**:
- Pass: migrations apply cleanly, no unapproved destructive op, a code rollback runs against the migrated schema.
- Fail: a migration drops/truncates data without approval OR a non-backward-compatible schema change ships.

**Rationale**: A schema change quietly eating data is the founder's explicit fear; automated blocking + expand-then-contract make code rollback safe.

**Trace**: GI-011 (minted — dim 9: "I don't want a schema change quietly eating data … an automated way to block a migration that isn't reviewed or is destructive")

### GI-023 — Database backup & restore · home: CLAUDE.md region + RUNBOOK.md

**Enforcement**:
- Managed-host automatic Postgres backups (daily + point-in-time restore) enabled — a configuration check, not custom infrastructure.
- A restore procedure documented in `RUNBOOK.md`.

**Testability**:
- Pass: managed backups enabled on the production DB, restore steps present in the runbook.
- Fail: backups off OR no documented restore procedure.

**Rationale**: Losing invoice data outright is the stated #2 worst-case; managed backups + a written restore path recover from a bad deploy, disk failure, or accidental mass-delete with nothing to babysit.

**Trace**: GI-023 (reopen-born from the intent review's coverage survivor; user ruled explore-now)

### GI-024 — Accessibility (WCAG 2.1 AA) · home: rules/mochiko/accessibility.md

**Enforcement**:
- CI runs `eslint-plugin-jsx-a11y` on components plus an axe-core assertion in tests; violations block merge.

**Testability**:
- Pass: a11y check green; interactive elements keyboard-operable and labelled; AA contrast met.
- Fail: any a11y check violation.

**Rationale**: Ledgerline is a customer-facing US web product; WCAG (ADA) is a legal-mandate obligation, unwaivable. A minimal automated CI check keeps it a floor, not a project, per the founder's ruling.

**Trace**: GI-024 (module: a11y — legal-mandate; mechanical fact-profile attachment)

## Trace summary

| GI-ID | Principle | Source | Primary home | Companions present |
|-------|-----------|--------|--------------|--------------------|
| GI-003 | Security at boundaries | floor-asserted: FLOOR-SEC | rules/mochiko/security.md | index ✓ · ledger ✓ |
| GI-004 | Testing discipline | floor-asserted: FLOOR-TEST | CLAUDE.md quality gates | index ✓ · ledger ✓ |
| GI-005 | Error handling | floor-asserted: FLOOR-ERR | rules/mochiko/error-handling.md | index ✓ · ledger ✓ |
| GI-006 | Observability (essentials) | floor-asserted: FLOOR-OBS | rules/mochiko/observability.md | index ✓ · ledger ✓ · waiver GI-012 |
| GI-007 | Hexagonal (light) | deck-kept: BE-HEX | rules/mochiko/architecture-layers.md | index ✓ · ledger ✓ |
| GI-008 | Single responsibility | deck-kept: BE-SRP | rules/mochiko/code-quality.md | index ✓ · ledger ✓ |
| GI-009 | Dependency discipline | deck-kept: BE-DEP | rules/mochiko/dependencies.md | index ✓ · ledger ✓ |
| GI-010 | Money correctness | minted | rules/mochiko/money-domain.md | index ✓ · ledger ✓ |
| GI-011 | Migration safety | minted | rules/mochiko/migrations.md | index ✓ · ledger ✓ |
| GI-023 | Backup & restore | reopen-born | CLAUDE.md region + RUNBOOK.md | index ✓ · ledger ✓ |
| GI-024 | Accessibility (WCAG 2.1 AA) | module: a11y (legal-mandate) | rules/mochiko/accessibility.md | index ✓ · ledger ✓ |

Non-principle GI elements: GI-001 (fact profile), GI-002 (type/stack → CLAUDE.md tech stack),
GI-012 (waiver, above), GI-013/014/016/017 (module selections, above), GI-018 (domain-dep seed →
architecture-layers.md registry), GI-019/020/021/022 (deliberate exclusions — recorded in the
synthesis + tracked on ROADMAP/BACKLOG, no surface principle by design).

Flagged proposals: none (the a11y attachment was confronted and confirmed in-session; folded, not left as a proposal).
Waivers: GI-012.

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| 0.1.0 | 2026-08-10 | ratified (first) | GI-001…GI-024 |
