# Derivation — shift-swaps (staged; not written to the live map)

Author: product-manager seat · 2026-09-04 · run: shift-swaps

## Map at baseline (as read at run open)

| ID | Capability | Status |
|----|------------|--------|
| FEAT-001 | Rota publishing | delivered |
| FEAT-002 | Availability | delivered |
| FEAT-003 | Time off | delivered |
| FEAT-004 | Clock-in | in-flight (tablet-offline) |

No capability on the map covers a staff-initiated change to a published shift, so shift swaps
is minted new; nothing to dedup against.

## Proposed entry — FEAT-006 (new)

```markdown
# FEAT-006 — Shift swaps

> Status: proposed
> surfaced by shift-swaps (2026-09-04)

## Capability

A staff member exchanges one of their published shifts for a colleague's at the same venue,
with the venue manager's approval; a staff member can also offer a shift to everyone at the
venue and the first eligible colleague to claim it takes it.

## Extent

- One-for-one exchange between two named staff at one venue, manager-approved.
- A shift offered to all staff at the venue, claimed by the first eligible colleague.
- Every state change recorded with actor and time.
- Not: swaps between venues.

## Relations

- depends-on: FEAT-001 — the published rota accepts a swap-driven reassignment

## Story trace

- shift-swaps: US-1, US-2, US-3, US-4
```

## Delta — FEAT-001 Rota publishing

- `pending` — a published shift accepts a swap-driven reassignment, recorded as a rota change
  with path `swap` · acceptance: an approved swap shows on the published rota with both actors
  in `rota_change` · cut by shift-swaps

## Specs-index line (draft)

| shift-swaps | specify — stress-test pending | FEAT-006 | 2026-09-02 | — |
