# Governance Ledger

**Governance Floor:** production (asserted) · **Depth level:** high (user-declared, one-way; `high` terminal) · **Modules:** knowledge-management (core; electives declined) · **Trace:** GI-001 (fact profile) · GI-002 (depth level)
**Version:** 1.1.0 (must match the region stamp)

## Waivers

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| Load and performance testing | Three maintainers; the largest studio issues under two hundred invoices a month | First studio above two thousand invoices a month | GI-002 |

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and
  un-waives are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change /
  module attach or detach · MINOR — new principle or waiver change · PATCH —
  clarification.
- Approvers: any two of the three maintainers.

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| none yet | | | |

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-003 — Client data containment · home: CLAUDE.md region

**Enforcement**:
- Export call sites are reviewed against the audited-export helper; `npm run audit:exports`
  lists every one of them.

**Testability**:
- Pass: every export path runs through the helper · Fail: a raw query hands client rows to
  a caller outside it.

**Rationale**: an unaudited bulk export is how client data leaves without anyone noticing.

**Trace**: GI-003 (deck-kept: DATA-01)

### GI-005 — Money handling · home: rules/mochiko/money-handling.md

**Enforcement**:
- `npm run lint` fails on a float literal in a money path; review checks currency codes at
  boundaries.

**Testability**:
- Pass: amounts cross boundaries as minor units with a currency · Fail: a bare number
  reaches the invoice renderer.

**Rationale**: silent rounding drift surfaces months later as a client dispute.

**Trace**: GI-005 (deck-kept: MONEY-02)

### GI-006 — Public pages · home: rules/mochiko/public-pages.md

**Enforcement**:
- The public router is the only router mounted without the session plugin; a route added
  there is reviewed against the token rule.

**Testability**:
- Pass: every public URL in the test suite carries a token and no numeric id · Fail: an
  invoice renders under a URL that embeds its id.

**Rationale**: clients have no accounts, so the link is the whole access control.

**Trace**: GI-006 (deck-kept: ACCESS-03)

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| 1.0.0 | 2026-07-08 | ratified | GI-001 … GI-009 |
| 1.1.0 | 2026-08-01 | public-pages rule gains the rate-limit line | GI-006 |
