# Lunch orders

> Spec: lunch-orders
> Created: 2026-09-03
> Status: draft — stress-test pending

---

## Intent

- **Scope boundary:** a parent orders school lunches for each of their children for a coming
  week from the published menu, paying from the family wallet, and can change an order until
  the cut-off.
- **Delivery:** whole feature in one run.
- **Depth / rigor:** high — money and children's dietary data.
- **UX-bearing:** yes — the ordering screens in the parent web app.
- **Constraints:** the kitchen's weekly menu is a Google Sheet the catering manager edits;
  orders are written back into a second tab of that sheet.
- **Out of scope:** ordering for a whole term at once; the kitchen's own view of counts; staff
  lunches.

---

## Overview

Parents pre-order lunches so the kitchen cooks the right amount and a child with an allergy
gets the right plate; today ordering is a paper slip handed in on Friday. Every family has a
card on file in the wallet, so an order is charged the moment it is placed.

---

## User Stories

| ID | Story (one breath) | Priority | Feature | Disposition |
|----|--------------------|----------|---------|-------------|
| [US-1](stories/US-1.md) | Order a week of lunches for one of my children | P1 | FEAT-005 | homed |
| [US-2](stories/US-2.md) | Change or cancel an order before the cut-off | P1 | FEAT-005 | homed |
| [US-3](stories/US-3.md) | See what my children are having this week and what they had | P2 | FEAT-006 | homed |
| [US-4](stories/US-4.md) | Order safely for a child with an allergy | P2 | FEAT-005 | homed |

---

## Edge Cases

- Menu not yet published for the chosen week → the parent sees "menu coming" and cannot order.
- Child leaves the school mid-week → remaining orders that week are credited to the wallet.

---

## Functional Requirements

- **FR-001** A parent MUST be able to place a lunch order per child per school day for any
  published week. *Source: US-1*
- **FR-002** The parent MUST see the whole published menu for the week, including items the
  kitchen has marked hidden, so that nothing on offer is missed. *Source: US-1*
- **FR-003** The order total MUST be charged to the family wallet when the order is confirmed.
  *Source: US-1*
- **FR-004** A parent MUST be able to change or cancel an order until the cut-off; a change
  after the cut-off MUST be refused with the reason shown. *Source: US-2*
- **FR-005** The cut-off for a week's orders MUST be 17:00 on the Thursday of the week before.
  *Source: US-2*
- **FR-006** Orders MUST be written to the kitchen's sheet within five minutes of placement
  through the Sheets API. *Source: US-1*
- **FR-007** The system MUST show allergen information for every item before it can be
  ordered, as the School Food Standards require. *Source: US-4*
- **FR-008** A child's dietary and allergy details MUST never appear on a screen unless that
  child's parent is signed in. *Source: governance — pupil data is Confidential*
- **FR-009** Parents MUST be able to rate each meal after it has been eaten. *Source: —*
- **FR-010** The menu MUST load fast. *Source: US-1*

---

## Key Entities

- **Order** — child, school week, one item per school day, total, placed at, last changed at.
- **Menu item** — existing; name, price, allergens, hidden (yes/no).
- **Family wallet** — existing; balance.

---

## Success Criteria

- **SC-001** 95 % of a week's orders are placed before the cut-off, measured from the order
  log over the first term.
- **SC-002** Parents find ordering easy.
- **SC-003** The kitchen's per-day counts equal the placed orders on every school day,
  measured by the catering manager's daily reconciliation.
- **SC-004** Paper slips fall to zero at the pilot schools within one term, counted by the
  school office.

---

## Screens & Flows

| ID | Screen | Purpose | Data shown | Feature |
|----|--------|---------|------------|---------|
| SCR-001 | Week menu | choose a meal for each school day for the selected child | days, items, price, allergen marks, wallet balance | — |
| SCR-002 | Child picker | choose which child to order for | children on the family account with class | — |
| SCR-003 | Order summary | confirm and pay | one line per day, total, wallet balance | — |
| SCR-004 | Order history | past weeks' orders | week, child, items, amount | — |

| ID | Flow | Steps | Story scenario | Feature |
|----|------|-------|----------------|---------|
| FLOW-001 | Order a week | SCR-002 → pick child → SCR-001 → pick a meal per day → SCR-003 → Pay → confirmation | US-1 / scenario 1 | — |
| FLOW-002 | See past orders | SCR-001 → "Order history" → SCR-004 | US-3 / scenario 1 | — |
| FLOW-003 | Order for a child with an allergy | SCR-002 → pick child → SCR-001 with marked items disabled | US-3 / scenario 3 | — |

**Prototype:** `prototype/` — clickable low-fi rendering of this manifest; open
`prototype/index.html` directly.

---

## Feature Selection

### Derived features

| FEAT-ID | Feature | New / delta | Stories | SCs verified |
|---------|---------|-------------|---------|--------------|
| FEAT-005 | Lunch ordering | new (`proposed`) | US-1, US-2, US-4 | SC-001, SC-002, SC-003, SC-004 |
| FEAT-006 | Order history | new (`proposed`) | US-3 | — |
| FEAT-003 | Wallet | delta — the wallet is charged at order time; status back to `proposed` while the wallet is reworked for this | — | — |

### Filter rejections

*(none)*

### Selection

- **Selected (build now, dependency order):** FEAT-003 delta, FEAT-005
- **Deferred (`proposed` on the map):** FEAT-006 — carries US-3
- **Deferred SCs:** none

---

## Assumptions

- Every family has a card on file, so every order is charged at placement.

---

## Open Questions

- Should we cache the menu in the browser so it loads fast?
- How many weeks ahead may a parent order?

---

## Technical notes

Orders live in PostgreSQL; a worker pushes new and changed orders to the kitchen sheet every
five minutes; the week's menu is cached in Redis for an hour; the week-menu endpoint should
hold p95 under 400 ms on a Monday morning.
