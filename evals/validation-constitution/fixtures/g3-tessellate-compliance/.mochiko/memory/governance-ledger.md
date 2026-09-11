# Governance Ledger

**Governance Floor:** production (asserted) · **Depth level:** high (user-declared 2026-09-09 by Inês Carvalho; one-way, `high` terminal) · **Modules:** compliance: gdpr (legal-mandate) · pci-dss (legal-mandate) · a11y (legal-mandate) · template: release-gates · knowledge-management (core + CHANGELOG elective) · **Trace:** GI-001 (fact profile) · GI-003 (depth level)
**Version:** 1.0.0 (must match the region stamp)

## Waivers

Any asserted standard, with a recorded justification (D4); permanent pending the D4.1 revisit.
**Legal-mandate module obligations are unwaivable (D4.2)** — a waiver row naming one is a
validator FAIL.

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| pci-dss quarterly ASV scan (Requirement 11.3.2) | Too expensive pre-launch; the first paying customer funds it | first paying customer | GI-012 |

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and un-waives
  are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change /
  depth-level flip (`low`→`high`) / module attach or detach · MINOR — new principle or waiver
  change · PATCH — clarification.
- Approvers: Inês Carvalho (founder) plus the on-call engineer, who did not author the change.

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-004 — Security by Default · home: CLAUDE.md region line; companion `.claude/rules/mochiko/security.md`

**Enforcement**:
- AWS Secrets Manager holds every credential; `.env*` is in `.gitignore`; `npm run scan:secrets` (gitleaks) runs as a pre-commit hook and as the CI job `secrets`, blocking merge on any finding.
- `npm audit --audit-level=high` runs as the CI job `audit` and blocks merge on any high or critical advisory.
- Zod schemas on every loader and action; `requireSession` / `requireMaker` on every route not listed in `src/auth/public-routes.ts`; the route-audit test fails on an unlisted public route.

**Testability**:
- Pass: zero gitleaks findings; zero high/critical advisories; the route-audit test green.
- Fail: any of the three.

**Rationale**: card data and two GDPR regimes; one leaked secret or one unguarded route is a reportable breach.

**Trace**: GI-004 (floor-asserted: FLOOR-SEC; high row)

### GI-005 — Testing Discipline · home: CLAUDE.md region line

**Enforcement**:
- `npm test` and `npm run e2e` run as CI jobs on every PR and block merge on failure.
- `npm run test:cov` enforces `--coverage.thresholds.lines=70` on new code and posts the figure; the CI job `coverage-ratchet` blocks merge on a decrease from the `main` baseline; 80% is the warning line.

**Testability**:
- Pass: both suites green; new-code coverage ≥ 70%; figure at or above the baseline.
- Fail: any of the three.

**Rationale**: checkout and payouts are the product; a regression there is a customer charged twice or a maker paid wrong.

**Trace**: GI-005 (floor-asserted: FLOOR-TEST; high row)

### GI-006 — Error Handling · home: CLAUDE.md region line

**Enforcement**:
- Every order, payment, and payout write runs in a Prisma transaction; payouts are computed from the settled amount in `src/payouts/settle.ts`.
- `src/errors/problem.ts` is the single builder of every error body (RFC 7807, `correlation_id` from the request middleware); the e2e suite asserts no `stack` key in any error body.

**Testability**:
- Pass: every error body validates against the Problem Details schema with `correlation_id`; no `stack` key anywhere.
- Fail: either.

**Rationale**: a half-written payment is a customer charged with no order.

**Trace**: GI-006 (floor-asserted: FLOOR-ERR; high row; BE-API-ERR folded)

### GI-007 — Observability · home: CLAUDE.md region line

**Enforcement**:
- `pino` JSON with `correlation_id` bound by the request middleware; the redaction list covers names, addresses, e-mail, bank details; the log test suite runs a PAN-pattern check over every captured line.
- `/healthz` on `web` and `worker`, checked by the ECS target group.

**Testability**:
- Pass: every captured line parses as JSON with `correlation_id`; the PAN-pattern check finds nothing; `/healthz` returns 200.
- Fail: any of the three.

**Rationale**: a card-number fragment in a log is a PCI breach; an uncorrelated payment failure is a support ticket nobody can answer.

**Trace**: GI-007 (floor-asserted: FLOOR-OBS; high row)

### GI-008 — Personal-Data Obligations (gdpr) · home: CLAUDE.md region line

**Enforcement**:
- Lawful-basis and retention classes recorded per table in `docs/data-map.md`; the nightly `retention` job enforces the classes; erasure requests fan out to the replica and the backup-scrub queue, with a 90-day completion check.
- Subject requests tracked in the `privacy_requests` table; a request open past 25 days pages on-call.
- The incident template carries the CNPD 72-hour clock.

**Testability**:
- Pass: no row past its retention class; no request open past 30 days; every erasure has a backup-scrub completion within 90 days.
- Fail: any of the three.

**Rationale**: two GDPR regimes; legal-mandate, unwaivable.

**Trace**: GI-008 (module: gdpr — lawful basis, data-subject rights, erasure propagation, breach notification)

### GI-009 — Cardholder Data (pci-dss) · home: `.claude/rules/mochiko/cardholder-data.md`

**Enforcement**:
- The custom ESLint rule `tsl/no-pan-escape` fails the build on an exported function whose parameter type is `Pan`; the log test suite's PAN-pattern check; `compliance/pci/` is checked by the quarterly compliance job for a dated ASV report and an annual SAQ D.

**Testability**:
- Pass: lint green; the PAN-pattern check finds nothing; a dated ASV report exists for the current quarter.
- Fail: any of the three.

**Trace**: GI-009 (module: pci-dss — PAN handling, scope confinement, ASV/SAQ evidence)

### GI-010 — Accessibility (a11y) · home: CLAUDE.md region line

**Enforcement**:
- `npm run a11y` (axe-core over every route) runs as the CI job `a11y` and blocks merge on any serious or critical finding; the accessibility statement is published at `/accessibility` before launch.

**Testability**:
- Pass: zero serious or critical axe findings; `/accessibility` returns 200.
- Fail: either.

**Rationale**: the European Accessibility Act applies to consumer e-commerce from 2025-06-28; legal-mandate, unwaivable.

**Trace**: GI-010 (module: a11y — WCAG 2.2 AA, accessibility statement)

### GI-011 — Fast Pages · home: CLAUDE.md region line

**Enforcement**:
- Engineers watch the Lighthouse scores.

**Testability**:
- Pass: pages feel quick.
- Fail: complaints.

**Rationale**: slow pages bounce.

**Trace**: GI-011 (minted)

### GI-013 — Release Gates · home: CLAUDE.md region line (summary); this entry (detail)

**Environments:** `staging` → `prod` (ECS on Fargate) · **Cadence:** `staging` on every merge; `prod` promoted manually after the soak.

| Gate | Requirement | Verified by | Blocks |
|------|-------------|-------------|--------|
| Staging soak | four hours on `staging` with no new Sentry issue | Sentry release view | promotion to prod |
| End-to-end | `npm run e2e` against `staging` | GitHub Actions `e2e-staging` job | promotion to prod |
| Accessibility | `npm run a11y` against `staging` | GitHub Actions `a11y-staging` job | promotion to prod |
| PCI evidence | a dated ASV report for the current quarter under `compliance/pci/` | GitHub Actions `pci-evidence` job | promotion to prod |

**Rollback:** ECS task-definition rollback to the previous revision by the on-call engineer within 15 minutes; a release carrying a destructive migration is flagged in the PR and promoted only after an RDS snapshot.

**Enforcement**: the three CI jobs; the soak is a manual check recorded on the promoting PR.
**Testability**: Pass: the jobs green and the soak note present · Fail: any missing.
**Rationale**: a bad promotion during a launch week is a marketplace with no checkout.

**Trace**: GI-013 (module: release-gates)

### GI-014 — Knowledge-Management Core + CHANGELOG · home: `.mochiko/memory/knowledge-management.md`

**Enforcement**:
- The project-pinned invariants file is the runtime source; command landing steps and `mochiko:grooming-operating-docs` resolve against it. Rules-file carrier: `.claude/rules/mochiko/operating-docs.md`. `CHANGELOG.md` codified into the elective role.

**Testability**:
- Pass: the invariants hold at command boundaries; the top `CHANGELOG.md` entry matches the deployed tag.
- Fail: an invariant trips without a groom, or a promotion lands with no entry.

**Rationale**: seven people and a launch; decisions need a home that is not Slack.

**Trace**: GI-014 (module: knowledge-management — core + elective-changelog)

### GI-016 — British English · home: CLAUDE.md region line

**Enforcement**:
- `cspell` with the `en-GB` dictionary runs in CI over `docs/`.

**Testability**:
- Pass: `cspell` green.
- Fail: a US spelling in `docs/`.

**Rationale**: consistency.

**Trace**: GI-016 (minted)

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| 1.0.0 | 2026-09-09 | ratified (first setup run; greenfield) | GI-001–016 |
