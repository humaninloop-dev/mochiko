# Swap items

> Spec: swap-items
> Created: 2026-08-11
> Status: in-flight — accepted 2026-08-15, FEAT-003 row `live`

---

## Intent

- **Scope boundary:** a customer swaps up to three items in next week's box for others of the
  same value from the published swap list, before the Friday cut-off.
- **Delivery:** whole feature in one run.
- **Depth / rigor:** high — swaps change the packhouse manifest.
- **UX-bearing:** yes — a swap screen in the customer web app.
- **Constraints:** swap list published with Monday's menu; no swaps of different value.
- **Out of scope:** adding extra items at a cost; standing swap preferences.

---

## Overview

Customers who cannot eat something in the box cancel more often than they swap. Swaps let them
keep the box and lose the item.

---

## User Stories

| ID | Story (one breath) | Priority | Feature | Disposition |
|----|--------------------|----------|---------|-------------|
| [US-1](stories/US-1.md) | Swap an item in next week's box for another of the same value | P1 | FEAT-003 | homed |

---

## Functional Requirements

- **FR-001** A customer MUST be able to swap up to three items in next week's box for items on
  the swap list of the same value, until the Friday cut-off. *Source: US-1*
- **FR-002** A swap MUST appear on the packhouse manifest for that customer's box. *Source: US-1*

---

## Success Criteria

- **SC-001** 30 % of customers who gave a "cannot eat" survey response use a swap within two
  months of launch, measured from the swap log against the survey list.

---

## Screens & Flows

| ID | Screen | Purpose | Data shown | Feature |
|----|--------|---------|------------|---------|
| SCR-001 | Next week's box | swap an item for one on the swap list | box items, swap list, swaps left | FEAT-003 |

| ID | Flow | Steps | Story scenario | Feature |
|----|------|-------|----------------|---------|
| FLOW-001 | Swap an item | SCR-001 → choose item → choose replacement → confirm | US-1 / scenario 1 | FEAT-003 |

**Prototype:** `prototype/` — archived with the accepted spec; not kept in the working tree.

---

## Feature Selection

### Derived features

| FEAT-ID | Feature | New / delta | Stories | SCs verified |
|---------|---------|-------------|---------|--------------|
| FEAT-003 | Box customisation | new (`in-flight`) | US-1 | SC-001 |

### Selection

- **Selected (build now, dependency order):** FEAT-003
- **Deferred (`proposed` on the map):** none
- **Deferred SCs:** none
