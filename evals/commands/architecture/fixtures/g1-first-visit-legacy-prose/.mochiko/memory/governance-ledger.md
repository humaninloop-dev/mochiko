# Governance Ledger

**Governance Floor:** production (asserted) · **Depth level:** high (user-declared, one-way; `high` terminal) · **Modules:** knowledge-management · **Trace:** GI-001 (fact profile) · GI-002 (depth level)
**Version:** 1.0.0 (must match the region stamp)

## Waivers

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| Load and performance testing | Two engineers; 19 accounts; the heaviest query is the dispatcher's day view | First account above 50 technicians | GI-002 |

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and
  un-waives are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change /
  module attach or detach · MINOR — new principle or waiver change · PATCH — clarification.
- Approvers: either engineer.

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| none yet | | | |

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-003 — Tenant scoping · home: CLAUDE.md region

**Enforcement**: every query goes through `store.Scoped`; the isolation suite greps the
tree for `pool.Query(` outside `internal/store` and fails on a hit.
**Testability**: pass — the suite is green and no raw query exists; fail — a raw query on a
tenant table anywhere under `internal/`.
**Rationale**: a missed `account_id` shows one firm another firm's customers.

### GI-004 — Phone numbers out of logs · home: CLAUDE.md region

**Enforcement**: the `notify` package masks numbers before any log call; Sentry's
`before_send` strips `to` fields.
**Testability**: pass — a grep of a day's logs for the E.164 pattern finds nothing; fail —
one match.
**Rationale**: customer numbers in a third-party log store is a data-protection incident.

### GI-005 — Public rate limiting and audit of money-moving actions · home: CLAUDE.md region

**Enforcement**: a middleware on every unauthenticated route; an `audit_events` row on
every plan change or refund (none exist yet — subscriptions are not live).
**Testability**: pass — the public booking route returns 429 above the limit and every
money-moving handler writes its row; fail — either absent.
**Rationale**: the booking form is the one unauthenticated write; subscriptions will move
money.

### GI-008 — Knowledge-management module · home: `.mochiko/memory/knowledge-management.md`

**Enforcement**: the invariants in that file, checked at command landings.
**Testability**: pass — no dead pointer, no `[x]` item in `BACKLOG.md`; fail — either.
**Rationale**: two engineers cannot carry decisions in their heads across a quarter.
