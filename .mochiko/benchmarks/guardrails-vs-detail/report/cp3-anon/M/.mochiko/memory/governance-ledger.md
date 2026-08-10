# Governance Ledger

**Governance Floor:** production (asserted) · **Modules:** a11y (WCAG, legal-mandate) · knowledge-management · release-gates · layer-rules · **Trace:** GI-001 (fact profile)
**Version:** 1.0.0 (matches the region stamp)

## Waivers

Any asserted standard, with a recorded justification (D4); permanent pending the D4.1 revisit.
Legal-mandate module obligations are unwaivable (D4.2).

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| None. | | | |

*(No floor category waived — all four accepted at the asserted level. Note: CI is not yet stood
up; this is greenfield, not a waiver — standing up GitHub Actions to run the gates is the first
foundational task, tracked in `BACKLOG.md`.)*

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and un-waives are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change / module attach or detach · MINOR — new principle or waiver change · PATCH — clarification.
- Approvers: Priya (solo founder). Enforcement leans on CI/tooling, never on code review (no reviewer).

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| none yet | | | |

## Release gates (release-gates module) <!-- GI-016 -->

**Environments:** local dev → production (Render). No staging at launch (nice-to-have; deferred).
**Cadence:** ship-when-ready; post-launch a few small releases/week.

| Gate | Requirement | Verified by | Blocks |
|------|-------------|-------------|--------|
| Quality gates | `ruff`/`black`/`pytest --cov`/`eslint`/`vitest` pass; coverage ≥60% | CI (GitHub Actions) | merge |
| Invoicing smoke | critical-path invoicing smoke test green | CI | release |
| Migration safety | migration reversible AND an automated Postgres backup taken immediately before it runs | pre-migrate script + `alembic` downgrade check | deploy |
| Backup + restore | scheduled DB backups exist AND a restore has been test-executed (proving recoverability) | Render backup schedule + recorded restore-drill | release (drill periodic) <!-- GI-028 --> |

### Rollback

- Rollback procedure MUST be documented and executable by Priya: Render → redeploy the previous good deploy; for a bad migration, restore the pre-migration Postgres backup.
- Rollback expectation: restore the previous version in ≤30 minutes (solo operator).
- A release with a non-reversible/destructive migration MUST be flagged in the PR and approved explicitly.

## Knowledge-management module <!-- GI-015 -->

Adopted whole (core) + electives CHANGELOG.md and RUNBOOK.md. Operating-docs layer scaffolded at
setup finalize; project-pinned invariants at `.mochiko/memory/knowledge-management.md`; shape
contracts injected via `.claude/rules/mochiko/operating-docs.md`; groom via
`mochiko:grooming-operating-docs`. Never-overwrite floor holds.

## Domain-dependency policy (layer-rules module) <!-- GI-017 -->

- Qualification: **domain-relevance** (modeling without I/O) filters candidates first, then **ubiquity** (ecosystem-standard, >80% adoption). Both required.
- Trust-signal hierarchy: (1) official curation, (2) semi-official stewardship, (3) credible community curation, (4) quantitative proxies + criteria. Python domain libs generally rest on level 4 + criteria.
- Add-process + gate: a proposed addition meeting both criteria surfaces to a human ruling BEFORE entering the registry; the cycle checkpoint MUST NOT auto-approve while `domain_deps_added` is non-empty.
- The registry list lives ONLY in the `mochiko:domain-registry` block inside `.claude/rules/mochiko/layers.md` — preserved verbatim across regenerations; no ledger copy. Seeded: pydantic (GI-018); attrs dropped (GI-019).

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-003 — Security by default · home: CLAUDE.md region line

**Enforcement**:
- CI runs `gitleaks detect --no-banner` (secret scan), `pip-audit` + `npm audit --audit-level=high` (deps, block high/critical); pydantic validates all request bodies at API boundaries; auth dependency enforced on every router; login endpoint rate-limited; secrets loaded from env, config with secrets in `.gitignore`.

**Testability**:
- Pass: zero secrets detected, zero high/critical vulnerabilities, every endpoint requires auth, login rate-limited, no card-data fields. · Fail: any secret, any high/critical vuln, any unauthenticated endpoint, or unthrottled login.

**Rationale**: Financial PII; a breach ends the company. Automated scanning + boundary validation catch issues before production without depending on a reviewer.

**Trace**: GI-003 (floor-asserted: FLOOR-SEC)

### GI-014 — No raw cardholder data (Stripe-hosted-only) · home: CLAUDE.md region line

**Enforcement**:
- No schema/field for card number or CVV anywhere; all card entry via Stripe hosted checkout; code review + absence-of-field check. A temporal backstop re-opens GI-001 if a direct card-handling flow is proposed.

**Testability**:
- Pass: no card-data field/column exists; card entry only through Stripe hosted flow. · Fail: any raw PAN/CVV field or direct card handling.

**Rationale**: Keeps cardholder data entirely out of scope (no PCI-DSS attachment); card data never touches Ledgerline's servers.

**Trace**: GI-014 (minted)

### GI-004 — Testing discipline · home: CLAUDE.md region line + quality gates

**Enforcement**:
- CI runs `pytest --cov --cov-fail-under=60` (backend) and `vitest run` (frontend); coverage ratchet compares to baseline; a smoke test on the invoicing critical path runs on every push.

**Testability**:
- Pass: all tests pass AND coverage ≥60% AND coverage ≥ previous baseline AND invoicing smoke green. · Fail: any test fails, coverage <60%, coverage decreased, or smoke red.

**Rationale**: Tests are the only safety net for a solo dev with no reviewer; the invoicing path is the product's whole value.

**Trace**: GI-004 (floor-asserted: FLOOR-TEST)

### GI-005 — Error handling · home: CLAUDE.md region line

**Enforcement**:
- FastAPI exception handlers emit RFC 7807 `application/problem+json` bodies with a correlation ID; schema test verifies error-body shape; React renders error states, never raw errors; no stack traces in responses.

**Testability**:
- Pass: every error response matches the problem+json schema and carries a correlation ID; no stack trace exposed. · Fail: any error missing required fields or leaking a stack trace.

**Rationale**: Failures must never silently corrupt invoice/payment data; consistent errors enable client handling and debugging.

**Trace**: GI-005 (floor-asserted: FLOOR-ERR)

### GI-006 — Observability · home: CLAUDE.md region line

**Enforcement**:
- Structured JSON logging with a correlation ID on every request/error; `/health` endpoint verified in an integration test; a lint/review check for PII in log statements; Sentry captures errors.

**Testability**:
- Pass: all errors logged with context + correlation ID, `/health` responds, no PII in logs. · Fail: silent failure, PII in a log, or missing `/health`.

**Rationale**: A one-person shop cannot debug production blind. (SLO/incident-process maturity is out of scope, GI-021 — the floor items themselves stand.)

**Trace**: GI-006 (floor-asserted: FLOOR-OBS)

### GI-030 — Transactional email deliverability · home: CLAUDE.md region line

**Enforcement**:
- SPF, DKIM, DMARC DNS records configured and verified for the sending domain; a bounce/complaint webhook records delivery failures.

**Testability**:
- Pass: DNS auth records present + valid; bounces recorded and surfaced. · Fail: missing/invalid SPF-DKIM-DMARC, or bounces silently dropped.

**Rationale**: Emailing invoices and reminders IS the product; mail in spam means the contractor doesn't get paid.

**Trace**: GI-030 (minted, reopen-born S6)

### GI-011 — Tenant isolation · home: rules/mochiko/tenant-isolation.md

**Enforcement**:
- Account scope applied at the repository/data-access boundary; an automated cross-tenant test suite asserts account A cannot reach account B's data; review checks new queries on tenant-owned tables carry the account filter.

**Testability**:
- Pass: every tenant-owned query is account-scoped; cross-tenant test suite green. · Fail: any query returns/modifies another account's rows, or a tenant table query with no account filter.

**Rationale**: A cross-tenant leak of financial data is company-ending — the founder's hardest line; it must be test-catchable, not a review vibe.

**Trace**: GI-011 (minted)

### GI-012 — Invoice & payment-state integrity · home: rules/mochiko/payment-integrity.md

**Enforcement**:
- Payment state reconciled against Stripe as source of truth; tests assert state transitions match Stripe events; DB constraints prevent orphaned/lost invoices.

**Testability**:
- Pass: no invoice lost; payment state matches Stripe in reconciliation tests. · Fail: an invoice disappears or state diverges from Stripe.

**Rationale**: The product's value is trusting payment state without re-checking Stripe.

**Trace**: GI-012 (minted)

### GI-013 — Money uses `decimal.Decimal` · home: rules/mochiko/payment-integrity.md

**Enforcement**:
- Lint/review rule against float arithmetic on money fields; money columns stored as exact numeric; value objects use `Decimal`.

**Testability**:
- Pass: no float used for money anywhere. · Fail: any float money value or arithmetic.

**Rationale**: Float money arithmetic corrupts payment state; supports GI-012.

**Trace**: GI-013 (minted)

### GI-026 — Stripe webhook trust · home: rules/mochiko/payment-integrity.md

**Enforcement**:
- Signature-verification middleware on the webhook endpoint (rejects unsigned/invalid); event handling keyed by Stripe event ID for exactly-once processing; a test suite replays events to assert idempotency and rejects forged signatures.

**Testability**:
- Pass: invalid-signature events rejected; a replayed event changes state exactly once. · Fail: an unsigned/forged or replayed event changes payment state.

**Rationale**: "Stripe is source of truth" is meaningless if the webhook itself isn't authentic and exactly-once; a forged/replayed event would violate GI-012.

**Trace**: GI-026 (minted, reopen-born S1)

### GI-029 — Invoice/payment audit trail · home: rules/mochiko/payment-integrity.md

**Enforcement**:
- Append-only change-log table for invoice + payment state changes (actor, timestamp, before/after); a test asserts historical rows are never updated in place.

**Testability**:
- Pass: every state change appends a log row; no in-place mutation of history. · Fail: a state change with no log row, or a mutated historical row.

**Rationale**: Financial disputes require "who changed what, when"; supports GI-012.

**Trace**: GI-029 (minted, reopen-born S5)

### GI-007 — Hexagonal layering · home: rules/mochiko/layers.md

**Enforcement**:
- `lint-imports` (import-linter) in CI **blocks** the load-bearing rule (domain importing the Stripe SDK or DB drivers) and **warns** on other layer-boundary violations (session ruling GI-007, `Contested`); external calls go through ports.

**Testability**:
- Pass: domain imports no SDK/driver/adapter; use cases testable with mock ports. · Fail: domain imports the Stripe SDK or a DB driver (blocking).

**Rationale**: Isolation is what makes invoice/payment logic and tenant-scoping unit-testable without Postgres or real Stripe; the founder relaxed full-purity enforcement to fit a solo workflow while keeping the correctness seam blocking.

**Trace**: GI-007 (deck-kept: BE-HEX)

### GI-008 — Code quality (single responsibility) · home: rules/mochiko/code-quality.md

**Enforcement**:
- ruff emits complexity (≤10) and function-length findings as **warnings** (session-tunable, non-blocking — GI-008 `Contested`, twice-recorded basis); the no-"utils"-dumping-ground rule is enforced by review/structure and is firm.

**Testability**:
- Pass: no utils dumping ground; complexity warnings reviewed. · Fail: a utils/helpers grab-bag module appears.

**Rationale**: Keeps a solo codebase understandable; the founder chose advisory complexity over a gate that blocks judgment calls before a demo, accepting it is advisory-only with no reviewer.

**Trace**: GI-008 (deck-kept: BE-SRP)

### GI-009 — Dependency discipline · home: rules/mochiko/dependencies.md

**Enforcement**:
- `pip-audit` + `npm audit --audit-level=high` block high/critical in CI; lock files committed; new-dep justification in the PR; external calls via ports.

**Testability**:
- Pass: deps pinned, zero high/critical vulns, no direct SDK use in domain. · Fail: unpinned dep, high/critical vuln, or SDK in domain.

**Rationale**: Every dependency is a liability; discipline keeps the surface small and swappable.

**Trace**: GI-009 (deck-kept: BE-DEP)

### GI-010 — Accessibility (WCAG 2.1 AA) · home: rules/mochiko/accessibility.md

**Enforcement**:
- `eslint-plugin-jsx-a11y` (part of `eslint .`) + automated `axe-core` checks in CI on key screens; a manual keyboard + contrast pass before UI-changing releases. **Legal-mandate — unwaivable (D4.2).**

**Testability**:
- Pass: jsx-a11y clean, axe checks pass, keyboard + contrast verified. · Fail: any a11y-lint or axe violation, or a screen not keyboard-navigable.

**Rationale**: ADA applies to the customer-facing (contractor-facing) app; accessibility built in from the start is cheap, retrofitting is not.

**Trace**: GI-010 (module: a11y-wcag)

### GI-028 — Backup & tested restore · home: ledger Release gates section (region pointer)

**Enforcement**:
- Render managed-Postgres scheduled backups enabled; an automated backup is taken immediately before every migration; a restore is periodically test-executed and the drill recorded in `RUNBOOK.md`.

**Testability**:
- Pass: scheduled backups exist AND a restore has been successfully test-executed within the drill interval. · Fail: no scheduled backup, or no restore has ever been verified.

**Rationale**: Data loss is the founder's top fear; an application bug corrupts data just as dead as a bad migration, and a backup that has never been restored is not a backup.

**Trace**: GI-028 (minted, reopen-born S4; realized as release-gates module content)

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| 1.0.0 | 2026-08-10 | ratified | GI-001–GI-030 |
