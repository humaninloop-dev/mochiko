# Governance Ledger

**Governance Floor:** production (asserted) · **Modules:** a11y (legal-mandate) · knowledge-management (core + CHANGELOG + RUNBOOK) · release-gates · layer-rules · **Trace:** GI-001 (fact profile)
**Version:** 1.0.0 (matches the CLAUDE.md region stamp)

## Waivers

Any asserted standard, with a recorded justification (D4); permanent pending the D4.1 revisit.
**Legal-mandate module obligations are unwaivable (D4.2)** — a waiver row naming one is a
validator FAIL.

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| None. | All four floor categories are principled; no floor standard was waived. | — | — |

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and un-waives
  are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change / module
  attach or detach · MINOR — new principle or waiver change · PATCH — clarification.
- Approvers: the founder (solo). Standing amend triggers: a customer contract requiring SOC 2
  (attestation module) · going international or onboarding contractors who bill EU/UK clients
  (gdpr module) · the pre-launch data-retention/deletion decision (GI-021).

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| none yet | | | |

## Domain-dependency policy (layer-rules adopted)

- Qualification: domain-relevance filters first (models/validates the invoicing domain without
  I/O), then ubiquity (ecosystem-standard, high adoption).
- Trust-signal hierarchy + full craft: `authoring-constitution/references/DOMAIN-DEPENDENCIES.md`.
- Add-process + gate: a human ruling precedes any registry entry; implement-time additions are
  disclosed in the cycle report (`domain_deps_added`), and the checkpoint never auto-approves
  while an addition is pending. The registry list itself lives ONLY in the
  `mochiko:domain-registry` block of `.claude/rules/mochiko/layers.md` — no ledger copy.

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-003 — Security floor (secrets · input validation · authN + object-level authZ · dep scan) · home: CLAUDE.md region

**Enforcement**:
- `gitleaks detect` in CI (secrets); pydantic validation at FastAPI boundaries; auth middleware on
  every route; `pytest` integration tests asserting cross-account access is denied (object-level
  authorization) on the invoicing critical path; `pip-audit` / `npm audit` block merge on
  high/critical. All in GitHub Actions, blocking merge to `main`.

**Testability**:
- Pass: no secret in the tree; unvalidated payloads rejected at the boundary; a request for
  another account's invoice returns 403/404 (never the data); no high/critical dependency
  vulnerability. Fail: any of these breached.

**Rationale**: leaking or cross-leaking contractor financial data is the top-of-mind
business-ending risk; broken object-level authorization (IDOR) is its most common vector, so
authorization — not just authentication — is a security-floor obligation, given the stated
single-tenant-per-account guarantee.

**Trace**: GI-003 (floor-asserted: FLOOR-SEC)

### GI-004 — Testing floor (coverage ratchet + critical-path smoke) · home: CLAUDE.md region

**Enforcement**:
- `pytest --cov --cov-fail-under=60` (≥80% warning) blocking in CI; ratchet — new-code coverage
  MUST NOT drop the baseline; an invoicing critical-path smoke test runs on every build.

**Testability**:
- Pass: coverage ≥ 60% on new code and ≥ baseline; smoke test green. Fail: coverage below floor
  or baseline, or smoke test red.

**Rationale**: the invoicing core is the code the founder "won't ship shaky"; automated coverage
+ smoke replace the reviewer the solo shop does not have.

**Trace**: GI-004 (floor-asserted: FLOOR-TEST)

### GI-005 — Error-handling floor (no silent corruption · consistent surface · no leaked traces) · home: CLAUDE.md region

**Enforcement**:
- RFC 7807 problem+json error handler in FastAPI; correlation IDs on requests; a `pytest` test
  asserting error responses carry no stack trace / internal detail; transactional writes so a
  failure rolls back rather than half-writing invoice/payment rows.

**Testability**:
- Pass: every error path returns problem+json with a correlation ID and no stack trace; a failed
  write leaves no partial invoice/payment state. Fail: a raw traceback reaches a client, or a
  partial write persists.

**Rationale**: silent corruption of invoice/payment data drives the contractor's decisions on bad
numbers; leaked traces expose internals and PII.

**Trace**: GI-005 (floor-asserted: FLOOR-ERR)

### GI-006 — Observability floor (structured logs · no PII/financial data · health check · scrubbed Sentry) · home: CLAUDE.md region

**Enforcement**:
- Structured JSON logging with correlation IDs; a log-scrubbing helper + a `pytest` test asserting
  known PII/financial fields never serialize into a log line; a `/healthz` endpoint for Render;
  Sentry configured with `before_send` scrubbing and no request bodies.

**Testability**:
- Pass: logs are structured, carry correlation IDs, contain no PII/financial fields (test green);
  `/healthz` returns 200; Sentry payloads are scrubbed. Fail: PII/financial data in a log line or
  a Sentry payload; no health endpoint.

**Rationale**: a solo operator needs machine-readable signal and a liveness probe; the no-PII rule
must extend to third-party processors (Sentry) or financial data egresses uncontrolled.

**Trace**: GI-006 (floor-asserted: FLOOR-OBS)

### GI-007 — Hexagonal two-seam architecture · home: rules/mochiko/layers.md

**Enforcement**:
- `import-linter` contracts covering domain→Stripe and domain→DB boundaries, blocking in CI; no
  import rules elsewhere.

**Testability**:
- Pass: domain code imports neither Stripe SDK nor DB/ORM types directly; invoicing tests run on
  port fakes without live infrastructure. Fail: a direct provider import in the domain, or a test
  requiring live Stripe/DB.

**Rationale**: test invoicing logic without live Stripe/DB and keep the Render/Stripe seams
swappable — scoped to two seams to avoid ceremony a solo shop cannot afford.

**Trace**: GI-007 (deck-kept: BE-HEX)

### GI-008 — Single-responsibility / complexity bound · home: CLAUDE.md region

**Enforcement**:
- Linter cyclomatic-complexity check (start ≤ 10, tunable) in `ruff`, blocking in CI; no "utils"
  catch-all modules.

**Testability**:
- Pass: no function exceeds the configured complexity; no module named/used as a `utils` dumping
  ground. Fail: complexity over threshold without an approved bump, or a catch-all utils module.

**Rationale**: keeps a one-person codebase legible and changeable; the threshold is tunable for
legitimately gnarly code.

**Trace**: GI-008 (deck-kept: BE-SRP)

### GI-009 — Dependency discipline · home: CLAUDE.md region

**Enforcement**:
- Versions pinned/locked; `pip-audit` / `npm audit --audit-level=high` block merge on
  high/critical; new deps justified (domain deps additionally gated — GI-017).

**Testability**:
- Pass: lockfiles present, no high/critical vulnerability, each dep has a justification. Fail: an
  unpinned or unjustified dep, or a high/critical vulnerability.

**Rationale**: dependency risk is the cheapest breach vector to close; a solo shop cannot audit
transitive trees by hand.

**Trace**: GI-009 (deck-kept: BE-DEP)

### GI-010 — Accessibility (WCAG 2.1 AA) · home: CLAUDE.md region · module: a11y / legal-mandate (unwaivable, D4.2)

**Enforcement**:
- Automated axe-based checks in CI against WCAG 2.1 AA (day-one enforcement floor — a known
  subset); the residual gap to full AA is a backlog-tracked manual-audit obligation, NOT a waiver.

**Testability**:
- Pass: axe CI checks green; the manual-audit gap is an open, tracked BACKLOG item. Fail: axe
  checks red, or the AA obligation treated as closed by automation alone.

**Rationale**: customer-facing UI served to US users carries an ADA accessibility obligation;
legal-mandate obligations are unwaivable, so automation is the enforcement floor, not the ceiling.

**Trace**: GI-010 (module: a11y-wcag-2.1-AA)

### GI-011 — Payment-status integrity · home: CLAUDE.md region

**Enforcement**:
- Tests on the payment-status state machine; the invoicing critical-path smoke test; the state
  machine MUST be green before a release cut.

**Testability**:
- Pass: state-machine tests + smoke green; no transition can report "paid" without the settling
  event. Fail: a false "paid" reachable, or state-machine tests red at release.

**Rationale**: "if the app says paid and it isn't, I'm done" — payment accuracy is the product's
trust core.

**Trace**: GI-011 (minted)

### GI-012 — Financial-data durability · home: CLAUDE.md region

**Enforcement**:
- Automated PostgreSQL backups with a verified restore path (backup existence check + a periodic
  restore check).

**Testability**:
- Pass: backups exist on schedule and a periodic restore succeeds. Fail: no backup, or an
  unverified/failing restore.

**Rationale**: losing contractor financial data "kills trust instantly" — durability is distinct
from the leak-prevention case (GI-003).

**Trace**: GI-012 (minted)

### GI-013 — Solo-operable, ship-frequently enforcement · home: CLAUDE.md region

**Enforcement**:
- All gates automated (CI/hooks/tooling), none dependent on a second reviewer; gates right-sized —
  the staging soak is smoke-level and skippable for small changes, complexity limits tunable.

**Testability**:
- Pass: every governing check runs without a human reviewer; a small change can ship without a
  blocking soak. Fail: a gate requires a second person, or blocks routine shipping.

**Rationale**: "operable and shippable by one person … I'd refuse gates so strict I can't ship";
the machine is the reviewer.

**Trace**: GI-013 (minted)

### GI-014 — Knowledge management (operating-docs layer) · home: rules/mochiko/operating-docs.md + CLAUDE.md pointer · module: knowledge-management

**Enforcement**:
- Command-boundary invariants under fix-on-sight (bijection · specs-index agreement ·
  status-agreement · open-only BACKLOG · horizon caps · item bounds · dead-pointer scan ·
  in-flight agreement · presence); `mochiko:grooming-operating-docs` on a tripped cap/bound;
  the subtractive landing ritual at each landing.

**Testability**:
- Pass: the invariants hold at command boundaries (vacuously at zero sessions/items). Fail: a dead
  pointer, a status disagreement, a `[x]` left in BACKLOG, or a horizon cap exceeded.

**Rationale**: "keeping a long-lived thing coherent in one head is exactly my problem"; prose-only
carriers and add-without-subtract obligations are the failure this module was redesigned to kill.

**Trace**: GI-014 (module: knowledge-management)

### GI-015 — Release gates · home: CLAUDE.md region pointer + ledger

**Enforcement** (Release Gates detail):

**Environments:** staging → production, both on Render.
**Cadence:** on-merge continuous to staging; manual promotion to production.

| Gate | Requirement | Verified by | Blocks |
|------|-------------|-------------|--------|
| Staging soak | smoke-level check green (skippable for small changes — GI-013) | invoicing critical-path smoke test | promotion to production |
| Migration check | expand-contract migration; reversible OR flagged destructive | migration review + CI | deploy |
| Changelog | entry present for a user-facing change | PR check | release cut |

**Rollback**:
- Rollback procedure MUST be documented and executable by the founder: redeploy the previous
  Render image; run the contract-phase down-migration where applicable.
- Restore previous version in ≤ 15 minutes.
- Releases that cannot be rolled back (destructive migrations) MUST be flagged in the PR and
  approved explicitly.

**Testability**:
- Pass: staging soak green (or waived-small), migration reversible-or-flagged, changelog present,
  rollback documented with a ≤15-min expectation. Fail: any gate unmet, or a destructive migration
  unflagged.

**Rationale**: a deployed/operated financial product needs a real release process; expand-contract
+ fast rollback protect financial data across migrations.

**Trace**: GI-015 (module: release-gates)

### GI-016 — Layer-rules module scope · home: rules/mochiko/layers.md

**Enforcement**: as GI-007 (`import-linter`, two seams); this record notes the module's scope
ruling — two-seam only, no full multi-layer ceremony.

**Testability**: Pass: import-linter contracts exist for exactly the Stripe and DB seams. Fail:
web-layer or other import ceremony present, or a seam contract missing.

**Rationale**: the founder's "no ceremony" tightening — govern the seams that must stay swappable,
nothing more.

**Trace**: GI-016 (module: layer-rules)

### GI-017 — Domain-dependency registry seed (`pydantic`) · home: rules/mochiko/layers.md (registry block)

**Enforcement**: registry block + add-policy (human ruling before any entry; `domain_deps_added`
disclosure at implement time; checkpoint holds while pending).

**Testability**: Pass: domain imports only registry-listed libraries; each row carries
justification, signal level, provenance, gate. Fail: an unlisted domain import, or a row with
blank metadata.

**Rationale**: keep the domain layer's dependency surface deliberate and minimal; `pydantic` is
the FastAPI-native validation standard.

**Trace**: GI-017 (module: layer-rules / domain-dependency seed)

## Non-principle-bearing intents (exclusions & deferrals — recorded, not enforced)

- **GI-018** — No SOC 2 / attestation work until a customer contract requires it (attestation
  module revisit trigger).
- **GI-019** — No multi-tenancy / teams governance at launch; a bookkeeper seat is deliberately
  not foreclosed. (Note: cross-account data isolation IS governed — GI-003 — this exclusion is
  only about multi-user-per-account.)
- **GI-020** — No formal SLOs or incident-response process now; error tracking via Sentry;
  ops-maturity revisited after launch. FLOOR-OBS met at asserted level (GI-006).
- **GI-021** — Data retention / deletion policy DEFERRED — an open question to resolve before
  launch (not a gate today, not a floor waiver). Its resolution MUST reconcile with GI-012 backup
  retention windows (deletion vs durable backups).

## Trace summary

| GI-ID | Principle | Source | Primary home | Companions present |
|-------|-----------|--------|--------------|--------------------|
| GI-003 | Security floor | floor-asserted: FLOOR-SEC | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-004 | Testing floor | floor-asserted: FLOOR-TEST | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-005 | Error-handling floor | floor-asserted: FLOOR-ERR | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-006 | Observability floor | floor-asserted: FLOOR-OBS | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-007 | Hexagonal two-seam | deck-kept: BE-HEX | rules/mochiko/layers.md | index ✓ · ledger ✓ |
| GI-008 | Complexity bound / SRP | deck-kept: BE-SRP | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-009 | Dependency discipline | deck-kept: BE-DEP | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-010 | Accessibility WCAG 2.1 AA | module: a11y | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-011 | Payment-status integrity | minted | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-012 | Financial-data durability | minted | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-013 | Solo-operable enforcement | minted | CLAUDE.md region line | index ✓ · ledger ✓ |
| GI-014 | Knowledge management | module: knowledge-management | rules/mochiko/operating-docs.md | index ✓ · ledger ✓ |
| GI-015 | Release gates | module: release-gates | CLAUDE.md region pointer + ledger | index ✓ · ledger ✓ |
| GI-016 | Layer-rules scope | module: layer-rules | rules/mochiko/layers.md | index ✓ · ledger ✓ |
| GI-017 | Domain-dependency seed | module: layer-rules | rules/mochiko/layers.md registry | index ✓ · ledger ✓ |

Flagged proposals: none — the cold-review folds (F1/F3/F4/F5 floor-expression strengthenings,
F6/F7/F8 notes) were ratified by the founder, not left as open proposals.
Waivers: none.
Non-principle-bearing (recorded, not on the map): GI-001 (fact profile), GI-002 (type),
GI-018/GI-019/GI-020/GI-021 (exclusions & deferrals).

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| 1.0.0 | 2026-08-10 | ratified | GI-001 … GI-021 |
