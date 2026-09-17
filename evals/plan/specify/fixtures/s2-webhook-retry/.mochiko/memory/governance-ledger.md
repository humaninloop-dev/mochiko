# Governance Ledger

**Governance Floor:** production (asserted) · **Depth level:** high (user-declared, one-way; `high` terminal) · **Modules:** none (knowledge-management declined 2026-08-02, durable) · **Trace:** GI-001 (fact profile) · GI-002 (depth level)
**Version:** 1.0.0 (must match the region stamp)

## Waivers

| Standard | Justification | Revisit trigger (optional) | Trace |
|----------|---------------|----------------------------|-------|
| Multi-region deployment | Two maintainers; every customer accepts a single-region status | First customer contract naming a region SLA | GI-002 |

## Amendment policy

- Route: `/mochiko:setup` amend mode; fact-profile changes (module attach/detach) and
  un-waives are governance events.
- Semver: MAJOR — principle removal / incompatible redefinition / floor-level change /
  module attach or detach · MINOR — new principle or waiver change · PATCH —
  clarification.
- Approvers: both maintainers.

## Exception registry

| Exception | Principle (GI-ID) | Granted | Expires/revisit |
|-----------|-------------------|---------|-----------------|
| none yet | | | |

## Principles (Three-Part metadata, keyed by GI-ID)

### GI-003 — No auto-close on a failing probe · home: CLAUDE.md region

**Enforcement**:
- The incident close path reads the latest probe result; a failing result rejects the close.

**Testability**:
- Pass: a close attempt during a failing probe returns 409 · Fail: an incident closes while
  the probe still fails.

**Rationale**: a silently closed incident is worse than no monitoring.

**Trace**: GI-003 (deck-kept: RELIABILITY-02)

### GI-005 — Alert delivery · home: rules/mochiko/alerting.md

**Enforcement**:
- The notifier writes the attempt row inside the same transaction that marks the outcome;
  review checks the 10-second timeout on every outbound call.

**Testability**:
- Pass: a hung webhook shows as a failed attempt after 10 s · Fail: an attempt with no
  recorded outcome.

**Rationale**: an unrecorded attempt cannot be retried honestly or explained to a customer.

**Trace**: GI-005 (deck-kept: ALERT-01)

## Amendment log

| Version | Date | Change | GI delta |
|---------|------|--------|----------|
| 1.0.0 | 2026-08-02 | ratified (brownfield) | GI-001 … GI-007 |
