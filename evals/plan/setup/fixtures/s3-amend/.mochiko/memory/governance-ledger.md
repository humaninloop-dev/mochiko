# Governance Ledger

**Governance Floor:** production (asserted) · **Depth level:** high (user-declared, one-way; `high` terminal) · **Modules:** none · **Trace:** GI-001 (fact profile) · GI-002 (depth level)
**Version:** 1.2.0 (must match the region stamp)

## Waivers

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| Load and performance testing | Two maintainers; the pilot tenant set is under fifty accounts and the billing run is nightly | First tenant above five hundred accounts | GI-002 |

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and
  un-waives are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change /
  module attach or detach · MINOR — new principle or waiver change · PATCH —
  clarification.
- Approvers: the two maintainers; either may approve.

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| none yet | | | |

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-003 — Customer data containment · home: CLAUDE.md region

**Enforcement**:
- Export call sites are reviewed against the audited-export helper; `npm run audit:exports`
  lists every one of them.

**Testability**:
- Pass: every export path runs through the helper · Fail: a raw query hands customer rows
  to a caller outside it.

**Rationale**: an unaudited bulk export is how invoice data leaves without anyone noticing.

**Trace**: GI-003 (deck-kept: DATA-01)

### GI-005 — Money handling · home: rules/mochiko/money-handling.md

**Enforcement**:
- `npm run lint` fails on a float literal in a money path; review checks currency codes at
  boundaries.

**Testability**:
- Pass: amounts cross boundaries as minor units with a currency · Fail: a bare number
  reaches the invoice renderer.

**Rationale**: silent rounding drift surfaces months later as a customer dispute.

**Trace**: GI-005 (deck-kept: MONEY-02)

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| 1.0.0 | 2026-05-14 | ratified | GI-001 … GI-007 |
| 1.2.0 | 2026-06-02 | coverage floor raised from 60% to 70% | GI-007 |
