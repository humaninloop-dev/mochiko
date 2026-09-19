# Derivation — pause-subscription (staged; not written to the live map)

Author: product-manager seat · 2026-09-03 · run: pause-subscription

## Map at baseline (as read at run open)

| ID | Capability | Status |
|----|------------|--------|
| FEAT-001 | Box subscription | delivered |
| FEAT-002 | Delivery scheduling | delivered |
| FEAT-003 | Box customisation | in-flight (swap-items) |
| FEAT-004 | Referrals | delivered |

Pausing is not covered by FEAT-001 (its extent excludes stopping boxes temporarily) nor by
FEAT-002 (a skip is one week, not a period), so it is minted new.

## Proposed entry — FEAT-005 (new)

```markdown
# FEAT-005 — Subscription pause

> Status: proposed
> surfaced by pause-subscription (2026-09-03)

## Capability

A customer pauses their subscription for a chosen period and resumes on the chosen date or
earlier; nothing is charged or delivered while paused.

## Extent

- Pause with a resume date from the account page; resume early.
- A reminder before the first box after the pause.
- Not: a single-week skip (FEAT-002); cancellation (FEAT-001).

## Relations

- depends-on: FEAT-001 — the weekly charge is suppressed while paused
- composes-with: FEAT-004 — referral credits keep accruing while paused

## Story trace

- pause-subscription: US-1, US-2, US-3
```

## Specs-index line (draft)

| pause-subscription | specify — stress-test pending | FEAT-005 | 2026-09-01 | — |
