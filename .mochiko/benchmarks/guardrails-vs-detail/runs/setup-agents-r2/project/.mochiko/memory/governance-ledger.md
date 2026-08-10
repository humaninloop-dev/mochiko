# Governance Ledger

**Governance Floor:** production (asserted) · **Modules:** compliance a11y (legal-mandate) · knowledge-management (core + CHANGELOG + RUNBOOK) · layer-rules · release-gates · **Trace:** GI-001 (fact profile)
**Version:** 1.0.0 (matches the region stamp)

## Waivers

Any asserted standard, with a recorded justification (D4); permanent pending the D4.1 revisit.
**Legal-mandate module obligations are unwaivable (D4.2).**

| Standard | Justification | Revisit trigger | Trace |
|----------|---------------|-----------------|-------|
| FLOOR-TEST numeric coverage gate (≥80% warning / ≥60% blocking) | Solo founder + ~5 hr/wk part-time UI helper; a coverage percentage would be gamed to hit the gate rather than earn real coverage. Financial correctness is protected instead by the critical financial-path test (GI-004) and the all-tests-green CI gate (GI-010). PO-D7 young-team on-ramp. | A second full-time engineer joins, OR first paying customers (~month four) — re-evaluate a real coverage ratchet | GI-019 |

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and un-waives
  are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change / module
  attach or detach · MINOR — new principle or waiver change · PATCH — clarification.
- Approvers: the founder (solo). Standing revisit triggers: targeting EU/UK users or clients
  (GDPR module), a signed attestation/SOC 2 commitment (attestation module).

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| none yet | | | |

## Domain-dependency policy (layer-rules adopted)

Qualification criteria (both required): **domain-relevance** (domain modeling without I/O — value
objects, validation, precise arithmetic) filters candidates FIRST; then **ubiquity** (>80%
adoption — effectively an ecosystem standard). Trust-signal hierarchy and craft:
`authoring-constitution/references/DOMAIN-DEPENDENCIES.md`. Add-process: a proposed addition
meeting both criteria surfaces to the founder as an explicit ruling BEFORE entering the registry;
the implement-time cycle checkpoint MUST NOT auto-approve while `domain_deps_added` is non-empty.
The list itself lives only in the `mochiko:domain-registry` block in
`.claude/rules/mochiko/layer-boundaries.md` — preserved verbatim across regenerations.

## Release Gates (release-gates module — GI-018)

**Environments:** Render — staging → production (promote staging to production once green).
**Cadence:** ship-when-ready (no fixed schedule; solo founder).

| Gate | Requirement | Verified by | Blocks |
|------|-------------|-------------|--------|
| CI green | lint + tests + secret scan + dependency audit all pass | GitHub Actions | merge to deploy branch |
| Financial-path test | invoice create + amount/tax/rounding correctness + Stripe webhook flips payment status | `pytest` (critical-path test) | merge to deploy branch |
| Migration reversibility | Alembic migration is reversible; a destructive/irreversible migration is flagged in the PR for explicit approval before it runs | commit review by founder | deploy |
| Backup restore | a database restore has been executed and verified before launch, and periodically after | manual restore drill (recorded in `RUNBOOK.md`) | launch |

### Rollback

- Rollback procedure MUST be documented and executable by the founder: redeploy the last good
  image on Render.
- Recovery time expectation: back online in ≤15 minutes.
- Releases that cannot be rolled back (e.g. destructive migrations) MUST be flagged in the PR and
  approved explicitly before running.

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-003 — Security by Default · home: CLAUDE.md

**Enforcement**:
- CI runs `gitleaks detect` and blocks merge on any secret finding.
- CI runs `pip-audit` and `npm audit`; merge blocked on high/critical vulnerabilities.
- API inputs validated with Pydantic models at every request boundary.
- Authentication enforced at every API endpoint (dependency-injected auth guard).
- `.gitignore` excludes `.env*` and local secret files; secrets loaded from environment / Render secrets.

**Testability**:
- Pass: zero secrets detected, zero high/critical vulnerabilities, every endpoint requires auth, all inputs validated.
- Fail: any secret committed OR a high/critical vulnerability OR an unauthenticated endpoint OR an unvalidated input boundary.

**Rationale**: Ledgerline holds contractors' financial and client PII; a breach or an injection is instant, permanent loss of trust. Automated scanning stands in for a review culture the solo team does not have.

**Trace**: GI-003 (floor-asserted: FLOOR-SEC)

### GI-004 — Testing Discipline · home: CLAUDE.md

**Enforcement**:
- CI runs `pytest`; merge blocked on any failing test.
- A critical financial-path test exists from day one covering invoice create, amount/tax/rounding correctness, and the Stripe payment-status webhook flipping paid/unpaid.

**Testability**:
- Pass: all tests green AND the financial-path test present and asserting amount correctness + webhook-driven status flip.
- Fail: any test red OR the financial-path test absent OR it asserts only happy-path status existence.

**Rationale**: The invoicing core is the one surface the founder will never cut quality on; a green test that checks only a status flip "is lying to me." The numeric coverage gate is waived (GI-019) because a percentage would be gamed; the real protection is this targeted test plus all-tests-green.

**Trace**: GI-004 (floor-asserted: FLOOR-TEST)

### GI-005 — Error Handling Standards · home: CLAUDE.md

**Enforcement**:
- API error responses follow RFC 7807 Problem Details; schema asserted in API integration tests.
- External calls (DB, Stripe) wrapped so failures never leave financial state partially written; correlation ID attached to every error.
- Response bodies never include stack traces (verified in tests).

**Testability**:
- Pass: every error response matches the RFC 7807 schema and carries a correlation ID; no partial-write corruption under induced failure; no stack trace in any response.
- Fail: any error missing the schema or correlation ID OR a stack trace exposed OR a failed operation leaving corrupted financial data.

**Rationale**: Corrupting financial data is the product-killing failure. Consistent, traceable errors let the founder diagnose a bad transaction and keep money state consistent.

**Trace**: GI-005 (floor-asserted: FLOOR-ERR)

### GI-006 — Observability Requirements · home: CLAUDE.md

**Enforcement**:
- Structured JSON logging via a logging wrapper with standard fields; correlation IDs propagated.
- Runtime errors reported to Sentry.
- `/health` endpoint verified in an integration test.
- Log statements reviewed (and lint-guarded where feasible) to exclude PII and secrets.

**Testability**:
- Pass: logs are structured JSON with no PII/secrets, `/health` responds, errors reach Sentry with correlation IDs.
- Fail: unstructured logs OR PII/secrets in logs OR missing `/health` OR silent unreported failures.

**Rationale**: A one-person operation cannot diagnose production issues without structured logs and error reporting. Baseline only — SLO/on-call depth is deliberately excluded (GI-021) until real traffic.

**Trace**: GI-006 (floor-asserted: FLOOR-OBS)

### GI-007 — Layer Boundaries · home: rules/mochiko/layer-boundaries.md

**Enforcement**:
- `import-linter` contract in CI enforces the layer import rules (domain imports no framework/adapters); merge blocked on violation.
- Domain-dependency additions gated by an explicit founder ruling (Domain-dependency policy above).

**Testability**:
- Pass: import-linter clean; domain layer free of I/O and framework imports; all use cases testable without a real DB.
- Fail: domain imports `services`/`repositories`/`api`/SQLAlchemy/FastAPI OR an unapproved domain dependency.

**Rationale**: Isolating the money logic from FastAPI and the DB makes it testable and reasoned about without infrastructure noise — protecting the correctness of the data whose corruption would kill the product. Pragmatic form only; full ports/adapters ceremony was dropped as over-build.

**Trace**: GI-007 (deck-kept: BE-HEX)

### GI-008 — Complexity Limit · home: CLAUDE.md

**Enforcement**:
- `ruff check --select C901 .` in CI, max-complexity 10; merge blocked on violation.

**Testability**:
- Pass: every function ≤10 cyclomatic complexity.
- Fail: any function >10 without a recorded exception.

**Rationale**: A single CI-enforced complexity ceiling keeps functions testable and modifiable. Review-enforced metrics (parameter count, file length, nesting) were dropped — there is no reviewer to run them.

**Trace**: GI-008 (deck-kept: BE-SRP)

### GI-009 — Dependency Discipline · home: CLAUDE.md

**Enforcement**:
- New dependencies justified in the commit/PR description; code review (founder self-review) verifies justification.
- Versions pinned in lock files (`uv.lock` / `package-lock.json`), committed.
- `pip-audit` / `npm audit` in CI block merge on high/critical vulnerabilities.

**Testability**:
- Pass: all dependencies pinned, zero high/critical vulnerabilities, each new dependency has a recorded justification.
- Fail: an unpinned dependency OR a high/critical vulnerability OR an unjustified new dependency.

**Rationale**: Every dependency is borrowed code with borrowed problems — attack surface and maintenance burden a solo team must keep minimal.

**Trace**: GI-009 (deck-kept: BE-DEP)

### GI-010 — CI Is the Gate · home: CLAUDE.md

**Enforcement**:
- GitHub Actions runs lint, tests, secret scan, and dependency audit on push.
- Branch protection blocks merge to the deploy branch while any check is red (no human-review gate).

**Testability**:
- Pass: no merge to the deploy branch occurs while any required check is failing.
- Fail: a merge lands over a red required check, or branch protection is not configured.

**Rationale**: With no second reviewer, CI is the only enforcement that reliably fires. "No merging red" is the founder's strongest stated non-negotiable.

**Trace**: GI-010 (minted)

### GI-011 — Accessibility Baseline · home: rules/mochiko/accessibility.md

**Enforcement**:
- `eslint-plugin-jsx-a11y` in CI (via `eslint .`); merge blocked on violations.
- axe assertions in component tests for interactive components.

**Testability**:
- Pass: jsx-a11y lint clean; interactive elements keyboard-operable; form controls labelled; contrast sufficient.
- Fail: a jsx-a11y violation OR an unlabelled control OR a keyboard-inoperable interactive element.

**Rationale**: The customer-facing UI is served to US users; ADA/WCAG is a legal-mandate obligation (unwaivable, D4.2). Authored at an automatable baseline per the founder's "sensible basics, not a full audit"; a full manual audit is deferred.

**Trace**: GI-011 (module: a11y-wcag-baseline)

### GI-012 — Data Durability · home: CLAUDE.md

**Enforcement**:
- Automated managed-Postgres backups enabled on Render.
- A restore drill executed and verified before launch, and periodically after; recorded in `RUNBOOK.md`.

**Testability**:
- Pass: automated backups configured AND at least one verified restore drill recorded before launch.
- Fail: no automated backups OR no restore ever tested (an untested backup is not a backup).

**Rationale**: Losing invoice/payment data is the other half of the top risk ("corrupting OR losing"). Managed backups are cheap; the discipline is proving a restore actually works.

**Trace**: GI-012 (minted)

### GI-013 — Financial Audit Trail · home: rules/mochiko/financial-audit.md

**Enforcement**:
- Invoice-amount and payment-status changes write an append-only audit-log entry (who, old→new, when); enforced in the service layer and asserted in integration tests.
- Audit rows have no update/delete path (append-only).

**Testability**:
- Pass: every invoice-amount and payment-status change produces an immutable audit entry with actor + timestamp + old/new; entries cannot be updated or deleted.
- Fail: a money-bearing change with no audit entry OR a mutable/deletable audit row.

**Rationale**: An immutable trail is the standard control for "money silently went wrong" and for the first payment dispute. Scoped to amount + payment status to avoid gold-plating.

**Trace**: GI-013 (minted)

### GI-014 — Tenant Isolation · home: rules/mochiko/tenant-isolation.md

**Enforcement**:
- Every data-access query scoped to the authenticated tenant (enforced in the repository layer).
- An integration test proves contractor A receives 403/404 fetching contractor B's invoice by ID; merge blocked if it fails.

**Testability**:
- Pass: no query returns cross-tenant rows; the cross-tenant access test passes (403/404).
- Fail: any endpoint returns another tenant's data by ID OR the cross-tenant test is absent/failing.

**Rationale**: Cross-tenant leakage is a total-blast-radius trust failure — "the one bug I can't ship." Authentication alone does not prevent it; object-level authorization does.

**Trace**: GI-014 (minted)

### GI-015 — Email Authentication & Link Integrity · home: CLAUDE.md

**Enforcement**:
- Sending domain configured with SPF, DKIM, and DMARC (DNS + email-provider setup), verified at launch and recorded in `RUNBOOK.md`.
- Payment links use a non-guessable, integrity-protected token.

**Testability**:
- Pass: SPF/DKIM/DMARC records present and aligned; payment-link tokens non-guessable and tamper-evident.
- Fail: missing/misaligned email-auth records OR guessable/tamperable payment links.

**Rationale**: Emailing invoices with a payment link is the core action; if mail lands in spam or a link is spoofable, contractors do not get paid and fraud becomes possible. Near-zero cost (DNS records), so authored now rather than deferred.

**Trace**: GI-015 (minted)

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| 1.0.0 | 2026-08-10 | ratified (first) | GI-001 … GI-024 |
