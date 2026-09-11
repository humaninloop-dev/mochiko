# Derivation — lunch-orders (staged; not written to the live map)

Author: product-manager seat · 2026-09-05 · run: lunch-orders

## Map at baseline (as read at run open)

| ID | Capability | Status |
|----|------------|--------|
| FEAT-001 | Family accounts | delivered |
| FEAT-002 | Menu publishing | delivered |
| FEAT-003 | Wallet | delivered |
| FEAT-004 | Kitchen counts | proposed (unrefined) |

Ordering is not covered by any entry; history is a distinct capability (it reads orders, it
does not place them), minted separately and deferred.

## Proposed entry — FEAT-005 (new)

```markdown
# FEAT-005 — Lunch ordering

> Status: proposed
> surfaced by lunch-orders (2026-09-05)

## Capability

A parent orders a lunch per school day per child for a published week from the family wallet
and changes it until the cut-off; the kitchen receives the order.

## Extent

- One meal per child per school day for any published week, paid from the wallet at
  confirmation.
- Change or cancel a day until the Thursday 17:00 cut-off; refused after, with the reason.
- Allergen marks per item per child; an item containing a recorded allergen cannot be ordered
  for that child.
- Orders written to the kitchen's sheet within five minutes.
- A parent can order for two or more children in one sitting without paying twice.
- A meal can be rated by the parent after it has been eaten.
- The week menu loads within the parent's patience on a Monday morning.
- Not: ordering a whole term at once; staff lunches.

## Relations

- depends-on: FEAT-002 — the published menu is what is ordered from
- depends-on: FEAT-003 — the wallet is charged at confirmation

## Story trace

- lunch-orders: US-1, US-2, US-4
```

## Proposed entry — FEAT-006 (new, deferred)

```markdown
# FEAT-006 — Order history

> Status: proposed
> surfaced by lunch-orders (2026-09-05)

## Extent

- Past weeks' orders per child with meals and amounts.

## Story trace

- lunch-orders: US-3
```

## Delta — FEAT-003 Wallet

- Status: `delivered` → `proposed` while the wallet is reworked to charge at order time.
- `pending` — the wallet is charged the order total at confirmation and credited on a
  cancelled day · acceptance: the balance after an order equals the balance before minus the
  total · cut by lunch-orders

## Specs-index line (draft)

| lunch-orders | specify — stress-test pending | FEAT-005, FEAT-006, FEAT-003 | 2026-09-03 | — |
