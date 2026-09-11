# Governance Ledger

**Governance Floor:** production (asserted) · **Depth level:** high (user-declared; `low` on 2026-04-14, flipped to `high` on 2026-09-10 by Mara Okonkwo; one-way, `high` terminal) · **Modules:** compliance: none (GI-001 negatives confirmed) · template: release-gates · evolution-notes · layer-rules · **Trace:** GI-001 (fact profile) · GI-003 (depth level)
**Version:** 1.0.1 (must match the region stamp)

## Waivers

Any asserted standard, with a recorded justification (D4); permanent pending the D4.1 revisit.
**Legal-mandate module obligations are unwaivable (D4.2)** — a waiver row naming one is a
validator FAIL.

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| None. | | | |

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach), un-waives,
  and the depth-level flip (a `high`-mode rerun) are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change /
  depth-level flip (`low`→`high`) / module attach or detach · MINOR — new principle or waiver
  change · PATCH — clarification.
- Approvers: [ROLE]

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| (none yet) | | | |

## Domain-dependency policy

Qualification: domain-relevance first (a library the domain layer would import for its own
concepts, never for I/O), then ubiquity; trust signals ranked per the plugin's
`DOMAIN-DEPENDENCIES.md`. Add-process: a proposed dependency is a human ruling before a
registry entry — the implement checkpoint never auto-approves while `domain_deps_added` is
non-empty. The list itself lives only in the domain rules file's `mochiko:domain-registry`
block; no copy here.

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-004 — Security by Default · home: CLAUDE.md region line

**Enforcement**:
- Fly secrets hold every credential; `fly.toml` is checked clean by the CI job `secrets` (`gitleaks detect --source .`), which blocks merge on any finding.
- `pip-audit` runs as the CI job `audit` and blocks merge on any high or critical advisory.
- Pydantic request schemas on every route; `require_tenant` dependency on every router (`api/deps.py`).

**Testability**:
- Pass: `gitleaks` zero findings; `pip-audit` zero high/critical; every router carries `require_tenant`.
- Fail: any finding, any high/critical advisory, a router without the dependency.

**Rationale**: the `fly.toml` password (GI-016) was one push from public; a cross-tenant read is the end of the company.

**Trace**: GI-004 (floor-asserted: FLOOR-SEC; high row from AM-1; fold of GI-016)

### GI-005 — Testing Discipline · home: CLAUDE.md region line

**Enforcement**:
- `pytest --cov=lantern` runs as the CI job `test` on every PR and blocks merge on failure.
- The CI job `coverage-ratchet` compares the figure to the `main` baseline and blocks merge on a decrease; coverage is kept at an appropriate level.

**Testability**:
- Pass: tests green; coverage at or above the baseline and at an appropriate level.
- Fail: a red run, or coverage below the baseline.

**Rationale**: a wrong rollup is a founder hiring on bad data.

**Trace**: GI-005 (floor-asserted: FLOOR-TEST; high row from AM-1)

### GI-006 — Error Handling · home: CLAUDE.md region line

**Enforcement**:
- `api/problem.py` is the single builder of every error body (RFC 7807, `correlation_id` from the request middleware).
- `ruff` rule `BLE001` plus the repo's `no-bare-pass` check fail the build on a swallowed exception.

**Testability**:
- Pass: every error body validates against the Problem Details schema and carries `correlation_id`; lint green.
- Fail: any error body that does not; a swallowed exception.

**Rationale**: the two swallowed exceptions in the nightly rollup silently double-counted a cohort for three weeks in March.

**Trace**: GI-006 (floor-asserted: FLOOR-ERR; high row from AM-1)

### GI-007 — Observability · home: CLAUDE.md region line

**Enforcement**:
- `structlog` JSON renderer configured in `lantern/logging.py`; the request middleware binds `correlation_id` to every line; the processor chain drops `email` and `name` keys.
- `/healthz` on `api` and `worker`, checked by Fly's HTTP health check.

**Testability**:
- Pass: every captured log line parses as JSON with `correlation_id`; no `email` key in any line; `/healthz` returns 200.
- Fail: any of the three.

**Rationale**: the March double-count was invisible for three weeks because nothing correlated a rollup to a tenant.

**Trace**: GI-007 (floor-asserted: FLOOR-OBS; high row from AM-1; waiver GI-009 retired)

### GI-008 — Tenant-Scoped Data Access · home: `.claude/rules/mochiko/data-access.md`

**Enforcement**:
- The repository base class requires `tenant_id` on every read of a tenant-owned table; a custom `ruff` plugin (`LNT001`) fails the build on `session.execute` outside `infrastructure/`.

**Testability**:
- Pass: `LNT001` zero findings; the tenant-isolation integration suite (`tests/integration/test_isolation.py`) green.
- Fail: either.

**Rationale**: a cross-tenant read is the end of the company.

**Trace**: GI-008 (minted)

### GI-010 — Repositories per Aggregate · home: CLAUDE.md region line

**Enforcement**:
- `importlinter` contract forbids `sqlalchemy` imports in `application/` and `domain/`; `lint-imports` runs in CI.

**Testability**:
- Pass: `lint-imports` green.
- Fail: a contract violation.

**Rationale**: Industry best practice.

**Trace**: GI-010 (minted)

### GI-011 — Layer Rules · home: `.claude/rules/mochiko/layers.md`

**Enforcement**:
- `importlinter` layers contract (`domain` < `application` < `infrastructure`, `api` may import `application` only); `lint-imports` blocks merge.
- Domain-dependency registry block in the layers rules file, seeded with `pydantic` and `attrs`; additions gated by a human ruling.

**Rationale**: the hexagonal shape is what makes tenant scoping reviewable in one place; an unguarded import erodes it one convenience at a time.

**Trace**: GI-011 (deck-kept: BE-HEX; module: layer-rules)

### GI-012 — Release Gates · home: CLAUDE.md region line (summary); this entry (detail)

**Environments:** `lantern-staging` → `lantern-prod` (Fly.io) · **Cadence:** `staging` on every merge; `prod` by a manual `fly deploy` after the soak.

| Gate | Requirement | Verified by | Blocks |
|------|-------------|-------------|--------|
| Staging soak | 24 hours on `lantern-staging` with no new Sentry issue | Sentry release view | prod deploy |
| Migration reversibility | every Alembic revision has a working `downgrade()` | CI job `migrate-roundtrip` (`alembic upgrade head && alembic downgrade -1 && alembic upgrade head`) | merge |

**Rollback:** `fly releases rollback --app lantern-prod` by the founder, within 10 minutes; a release with a destructive migration is flagged in the PR and deployed only after a database snapshot.

**Enforcement**: the two CI jobs above; the soak is a manual check recorded in the PR before `fly deploy`.
**Testability**: Pass: both jobs green and the soak note present on the deploying PR · Fail: either missing.
**Rationale**: 140 tenants read the weekly e-mail on Monday; a bad Friday deploy is 140 wrong e-mails.

**Trace**: GI-012 (module: release-gates)

### GI-013 — Signed Commits · home: CLAUDE.md region line

**Enforcement**:
- GitHub branch protection requires signed commits on `main`.

**Testability**:
- Pass: every commit on `main` is signature-verified.
- Fail: one that is not.

**Rationale**: the contractor's laptop was stolen in June.

**Trace**: GI-013 (minted)

## Evolution notes

This constitution was created from brownfield analysis (`.mochiko/memory/codebase-analysis.md`, 2026-04-14).

**Essential Floor Status** (assessed against the codebase):

| Category | Status | Response |
|----------|--------|----------|
| Security | present | codified existing pattern (GI-004) |
| Testing | present | codified existing pattern (GI-005) |
| Error Handling | partial | codified the single problem handler; the swallowed exceptions are a MUST-fix (GI-006) |
| Observability | absent | MUST-implement |

**Confrontations resolved in session:** none.

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| 1.0.0 | 2026-04-14 | ratified (first setup run; brownfield) | GI-001–016 |
| 1.0.1 | 2026-09-10 | PATCH — wording clarifications; depth level recorded as high; layer rules file added | GI-003 · GI-011 |
