# Governance Ledger

**Governance Floor:** production (asserted) · **Depth level:** high (user-declared, one-way; `high` terminal) · **Modules:** none · **Trace:** GI-001 (fact profile) · GI-002 (depth level)
**Version:** 1.1.0 (must match the region stamp)

## Waivers

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| Load and performance testing | The largest harbour has 300 berths; the heaviest page is the berth plan on one tablet | A harbour above 500 berths or a public booking portal | GI-002 |

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and
  un-waives are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change /
  module attach or detach · MINOR — new principle or waiver change · PATCH — clarification.
- Approvers: the founder and the lead engineer; either may approve.

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| none yet | | | |

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-003 — Bill a reading once · home: CLAUDE.md region

**Enforcement**:
- The import keys every reading by pedestal, channel, and timestamp; `mix billing.audit`
  lists any reading that appears on two invoices.

**Testability**:
- Pass: `mix billing.audit` prints nothing for the month · Fail: any reading id appears on
  two invoice lines.

**Rationale**: a double-billed kilowatt-hour is a refund, an apology, and a lost renewal.

**Trace**: GI-003 (deck-kept: BILL-01)

### GI-004 — Money handling · home: rules/mochiko/billing.md

**Enforcement**:
- `mix credo --strict` fails on a float literal under `lib/tidewatch/billing/`; review checks
  currency codes at boundaries.

**Testability**:
- Pass: amounts cross boundaries as minor units with a currency · Fail: a bare number reaches
  the invoice renderer.

**Rationale**: rounding drift surfaces months later as a skipper's dispute.

**Trace**: GI-004 (deck-kept: MONEY-02)

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| 1.0.0 | 2026-03-02 | ratified | GI-001 … GI-007 |
| 1.1.0 | 2026-07-03 | idempotent-import clause added to GI-003 | GI-003 |
