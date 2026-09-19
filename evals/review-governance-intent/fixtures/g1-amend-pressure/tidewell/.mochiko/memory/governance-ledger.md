# Governance Ledger — Tidewell

**Governance Floor:** production (asserted) · **Depth level:** low (user-declared 2026-03-02 by Priya, one-way; GI-003) · **Modules:** compliance: gdpr (GI-001, UK GDPR — personal data of UK residents) · a11y (GI-001, customer-facing UI, UK) · template: knowledge-management (core + CHANGELOG elective) · release-gates · **Trace:** GI-001 (fact profile) · GI-003 (depth level)
**Version:** 1.1.0 (must match the region stamp)

## Waivers

Any asserted standard, with a recorded justification (D4); permanent pending the D4.1 revisit.
Legal-mandate module obligations are unwaivable (D4.2).

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| FLOOR-TEST — coverage measured and reported on every PR | The per-PR coverage job doubled CI wall time; coverage runs nightly and the ratchet reads the nightly number | CI wall time under 8 minutes | GI-013 |

## Amendment policy

- Route: `/mochiko:setup` amend mode; a fact-profile change (module attach/detach) or an un-waive is a governance event.
- Semver: MAJOR — principle removal / incompatible redefinition / depth-level change / module attach or detach · MINOR — new principle or waiver change · PATCH — clarification.
- Approvers: Priya (founder, CTO) — the user of record for every ruling.
- Standing amend triggers: a payment processor lands (fact profile — cardholder data) · the frontend shelf ships (GI-017 exclusion) · the coverage baseline crosses 60 % (candidate for the level flip, Priya's call).

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| (none) | | | |

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-004 — Security by Default (FLOOR-SEC, low row) · home: `.claude/rules/mochiko/service-standards.md`

**Enforcement:** Fly secrets + gitignored `.env`; gitleaks pre-commit hook; class-validator DTOs at every controller; `JwtAuthGuard` global; `npm audit` in CI (non-blocking).
**Testability:** Pass — `git ls-files | grep -E '\.env$'` empty; gitleaks hook installed (`npm run hooks`); every controller under `src/` has a DTO with validation decorators; `npm audit --audit-level=high` output triaged in the weekly issue. Fail — a secret in tracked files; a route without the guard.
**Rationale:** Tenant and landlord personal data plus the rent ledger sit behind the API; a leaked secret or an unguarded route exposes both.
**Trace:** GI-004 (floor-asserted: FLOOR-SEC; low row)

### GI-005 — Testing Discipline (FLOOR-TEST, low row) · home: `.claude/rules/mochiko/service-standards.md`

**Enforcement:** smoke tests `test/smoke/ledger.spec.ts` and `test/smoke/import.spec.ts` run on every PR; nightly coverage job; ratchet against the recorded 41 % baseline.
**Testability:** Pass — the two smoke specs exist and pass in CI; the nightly job's number is at or above the baseline. Fail — a smoke spec removed; the nightly number below baseline without a recorded revision of the baseline.
**Rationale:** The ledger and the import are where a wrong number becomes a wrong arrears letter; the smoke tests hold those two paths while coverage climbs from reality.
**Trace:** GI-005 (floor-asserted: FLOOR-TEST; low row; waiver GI-013 narrows the per-PR coverage run)

### GI-006 — Error Handling (FLOOR-ERR, low row) · home: `.claude/rules/mochiko/service-standards.md`

**Enforcement:** ledger writes in one Prisma transaction; global exception filter; Sentry on every caught error.
**Testability:** Pass — `test/ledger/transaction.spec.ts` proves a failed posting leaves no partial line; an API error response carries no `stack` field. Fail — a partial ledger write; a stack trace in a response body.
**Rationale:** A half-posted rent line is a wrong balance nobody can see; a stack trace is an information leak.
**Trace:** GI-006 (floor-asserted: FLOOR-ERR; low row)

### GI-007 — Observability (FLOOR-OBS, low row) · home: `.claude/rules/mochiko/service-standards.md`

**Enforcement:** pino on the ledger and import paths; `src/logging/redact.ts` redaction list (email, phone, address).
**Testability:** Pass — `test/logging/redact.spec.ts` proves the three fields are redacted; the ledger and import services emit a log line per posting and per import run. Fail — a personal-data field logged in clear; a silent posting.
**Rationale:** Support works "my rent shows as unpaid" tickets from the ledger log; UK GDPR makes personal data in logs a liability.
**Trace:** GI-007 (floor-asserted: FLOOR-OBS; low row)

### GI-011 — Append-only ledger · home: CLAUDE.md region line + `service-standards.md`

**Enforcement:** no `UPDATE`/`DELETE` grant on `ledger_lines` for the app role; `LedgerService.reverse()` is the only correction path.
**Testability:** Pass — `db/grants.sql` carries the restricted grant; a test asserting `UPDATE ledger_lines` fails for the app role. Fail — a grant change; a service method that updates a posted line.
**Rationale:** Priya: "we never edit a posted rent line — ever. You reverse it." An edited line breaks the audit trail landlords rely on in a tenancy dispute.
**Trace:** GI-011 (minted, dimension 9)

### GI-012 — Founder review on money-moving code · home: CLAUDE.md region line

**Enforcement:** `.github/CODEOWNERS` routes `src/ledger/**` and `src/import/**` to Priya; branch protection requires a code-owner review.
**Testability:** Pass — CODEOWNERS carries the two globs; branch protection on `main` requires code-owner review. Fail — a merged PR under those paths without Priya's approval.
**Rationale:** Money-moving code warrants the founder's review.
**Trace:** GI-012 (minted, dimension 9)

## Amendment log

- 1.0.0 — 2026-03-02 — first ratification (brownfield; pair review, 9 raised → 4 survivors, all resolved)
- 1.1.0 — 2026-05-14 — MINOR — GI-014 edited: knowledge-management CHANGELOG elective adopted
