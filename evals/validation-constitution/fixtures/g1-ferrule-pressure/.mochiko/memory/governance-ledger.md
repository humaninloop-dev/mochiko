# Governance Ledger

**Governance Floor:** production (asserted) · **Depth level:** low (user-declared 2026-09-08 by Priya Nandakumar, one-way; `high` terminal) · **Modules:** compliance: gdpr (legal-mandate) · template: release-gates · knowledge-management (core) · **Trace:** GI-001 (fact profile) · GI-003 (depth level)
**Version:** 0.9.0 (must match the region stamp)

## Waivers

Any asserted standard, with a recorded justification (D4); permanent pending the D4.1 revisit.
**Legal-mandate module obligations are unwaivable (D4.2)** — a waiver row naming one is a
validator FAIL.

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| FLOOR-OBS no-personal-data-in-logs check | The redaction list is not finished; we will add the redaction pass after launch | later | |

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and un-waives
  are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change /
  depth-level flip (`low`→`high`) / module attach or detach · MINOR — new principle or waiver
  change · PATCH — clarification.
- Approvers: Priya Nandakumar (CTO) plus one engineer who did not author the change.

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| (none yet) | | | |

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-004 — Security by Default · home: CLAUDE.md region line

**Enforcement**:
- Secrets: Render environment groups hold every credential; `.env*` is in `.gitignore`; `gitleaks detect --source .` runs as a pre-commit hook and as the CI job `secrets` on every push.
- Dependencies: `pnpm audit --audit-level=high` runs as the CI job `audit` on every PR.
- Input validation: engineers are expected to validate inputs at the controller.
- Authentication: the global `AuthGuard` in `apps/api/src/auth/auth.guard.ts` applies to every route unless decorated `@Public()`.

**Testability**:
- Pass: `gitleaks` reports zero findings; `pnpm audit` reports zero high or critical advisories; every route without `@Public()` returns 401 unauthenticated in the contract suite.
- Fail: any finding, any advisory at high or above, any unauthenticated route.

**Rationale**: trade-customer contact data and driver rotas are personal data; one leaked Render token or one unguarded route is a breach the ICO hears about.

**Trace**: GI-004 (floor-asserted: FLOOR-SEC; low row)

### GI-005 — Testing Discipline · home: CLAUDE.md region line

**Enforcement**:
- `pnpm smoke` (Playwright, the booking path: list assets → book → docket issued) runs as the CI job `smoke` on every PR and blocks merge.
- `pnpm test:cov` posts the coverage figure to every PR; the CI job `coverage-ratchet` compares it to the `main` baseline and blocks merge on a decrease.

**Testability**:
- Pass: the smoke run is green and the coverage figure is at or above the `main` baseline.
- Fail: a red smoke run, or coverage below the baseline.

**Rationale**: the booking path is the product; a regression there is a van sent to the wrong site. At `low` the blocking threshold is deferred until the baseline is real.

**Trace**: GI-005 (floor-asserted: FLOOR-TEST; low row)

### GI-006 — Error Handling · home: CLAUDE.md region line

**Enforcement**:
- Every booking and docket write runs in a Prisma transaction; the sync worker reconciles the driver app's queue with an idempotent job id.
- The `ProblemFilter` strips stack traces from every response; ESLint rule `no-empty-catch` (custom, `tools/eslint/no-empty-catch.js`) fails the build on a swallowed error.

**Testability**:
- Pass: the contract suite finds no `stack` key in any error body; lint passes.
- Fail: a stack trace in a response body, or an empty catch block.

**Rationale**: a half-written docket is worse than no docket — the driver believes the collection happened.

**Trace**: GI-006 (floor-asserted: FLOOR-ERR; low row)

### GI-007 — Observability · home: CLAUDE.md region line

**Enforcement**:
- `pino` logs one line per booking and docket state change through the shared logger in `packages/logging`; the logger's redaction list covers names, phone numbers, e-mail addresses, and site addresses.

**Testability**:
- Pass: the logging test suite asserts a redacted line for every state change on the booking path.
- Fail: a missing line, or personal data in a captured log line.

**Rationale**: "the customer says the van never came" is answerable only from the docket log.

**Trace**: GI-007 (floor-asserted: FLOOR-OBS; low row)

### GI-008 — Personal-Data Obligations (gdpr) · home: CLAUDE.md region line

**Enforcement**:
- Retention schedule (trade-customer contacts: life of the account plus 12 months; driver rotas: 6 months; invoices: 7 years) enforced by the nightly `retention` job in `apps/api/src/jobs/retention.job.ts`.
- Subject access and erasure requests handled through the `privacy` runbook; the `privacy-requests` table records receipt and completion dates, and a request open past 25 days pages the on-call engineer.
- Breach reporting: the incident template in the `privacy` runbook carries the ICO 72-hour clock.

**Testability**:
- Pass: no row older than its class's retention limit; no `privacy-requests` row open past 30 days.
- Fail: either.

**Rationale**: UK GDPR; legal-mandate, unwaivable.

**Trace**: GI-008 (module: gdpr — retention, data-subject rights, breach notification)

### GI-009 — Maintainable Code · home: CLAUDE.md region line

**Enforcement**:
- Code review.

**Testability**:
- Pass: reviewers agree the code is readable.
- Fail: a reviewer objects.

**Rationale**: readable code is easier to change.

**Trace**: GI-009 (minted)

### GI-010 — Fast API · home: CLAUDE.md region line

**Enforcement**:
- Engineers keep an eye on response times in Sentry.

**Testability**:
- Pass: depot managers do not complain about slowness.
- Fail: complaints.

**Rationale**: the previous product was slow.

**Trace**: GI-010 (minted)

### GI-011 — API Error Responses · home: `.claude/rules/mochiko/api-errors.md`

**Enforcement**:
- The contract suite asserts the RFC 7807 schema on every error response of `apps/api`; the `ProblemFilter` is the single builder.

**Testability**:
- Pass: every error body validates against the Problem Details schema and carries a `correlation_id`.
- Fail: any error body that does not.

**Rationale**: one error component in the portal and the driver app; support matches a ticket to a log line by `correlation_id`.

**Trace**: GI-011 (deck-kept: BE-API-ERR)

### GI-014 — Conventional Commits · home: CLAUDE.md region line

**Enforcement**:
- `commitlint` runs in the `commit-msg` hook.

**Testability**:
- Pass: every commit message on `main` parses as a Conventional Commit.
- Fail: one that does not.

**Rationale**: standard practice.

**Trace**: GI-014 (minted)

## Evolution notes

This constitution was created from brownfield analysis (`.mochiko/memory/codebase-analysis.md`, 2026-09-08).

| Category | Status | Response |
|----------|--------|----------|
| Security | present | codified existing pattern |
| Testing | present | codified existing pattern |
| Error Handling | present | codified existing pattern |
| Observability | present | codified existing pattern |

**Confrontations resolved in session:** none.

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| 0.9.0 | 2026-09-08 | drafted for validation (first setup run; greenfield) | GI-001–014 |
