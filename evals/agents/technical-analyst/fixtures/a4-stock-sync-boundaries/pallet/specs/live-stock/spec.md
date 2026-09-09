# Feature Specification — Live stock (FEAT-017)

- **Status:** signed off 2026-09-07 · **Owner:** Priya (product)

## Overview

Retailers order things suppliers no longer have. The supplier then edits the order or cancels
a line, and the retailer finds out the next morning. Most suppliers already track stock in an
inventory or till system; Pallet should show that stock while the retailer orders and stop an
order for something that is out.

## User Stories

### US-001 — See stock while ordering (P1)

As a retailer, I want to see whether a product is in stock so that I order what will arrive.

- **Given** a product with 12 cases in stock, **when** the retailer views the catalogue,
  **then** it shows "in stock"; **when** stock is 0, **then** it shows "out of stock" and the
  product cannot be added to the basket.

### US-002 — Stock comes from my system (P1)

As a supplier, I want my stock levels to come from my inventory system so that I do not keep
them in two places.

- **Given** a supplier connected to their inventory system, **when** stock changes there,
  **then** the catalogue reflects it soon after.

### US-003 — Override (P2)

As a supplier, I want to override a product's stock by hand (for example, to hold some back
for a market) and see who last changed it.

### US-004 — Not connected (P2)

As a supplier without an inventory system, I want to enter stock counts by hand or leave
products as "always available".

## Functional Requirements

- **FR-001** The catalogue MUST show each product's availability (in stock / low / out of
  stock / always available) and MUST NOT allow an out-of-stock product to be added to a
  basket. *Source: US-001*
- **FR-002** The system MUST sync stock levels from a connected supplier's inventory system.
  *Source: US-002*
- **FR-003** A supplier MUST be able to override a product's stock level, with the override
  recorded against the person who made it. *Source: US-003*
- **FR-004** A supplier with no connected system MUST be able to set stock by hand or mark a
  product always available. *Source: US-004*
- **FR-005** The retailer MUST be told, in the basket and at checkout, if a product went out
  of stock since it was added. *Source: US-001*

## Success Criteria

- **SC-001** Supplier-side order edits for out-of-stock lines fall by 60% within three
  months, measured from order edit reasons.
- **SC-002** Stock shown is never stale.
- **SC-003** Connected suppliers reach 50% of active suppliers within six months.

## Quality expectations

Stock should update in near real-time. Sync must be reliable. We shouldn't hammer suppliers'
systems. Stock levels are not sensitive, so no special handling is needed. We expect 500
connected suppliers within a year.

## Assumptions

- Suppliers' systems expose current stock per product.
- Pallet product ids can be matched to the supplier's own product codes.

## Out of scope

- Reserving stock when a product is added to a basket.
- Pallet writing stock back to the supplier's system.
