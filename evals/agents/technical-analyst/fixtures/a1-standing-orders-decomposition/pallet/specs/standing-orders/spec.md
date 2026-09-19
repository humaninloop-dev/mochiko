# Feature Specification — Standing orders (FEAT-014)

- **Status:** signed off by the product owner 2026-09-08 · **Author:** requirements analyst ·
  **Owner:** Priya (product)

## Overview

Cafés order the same bread, milk, and coffee every week. Today they re-place the order by
hand before each supplier's cut-off, and a missed cut-off is a morning without bread. A
standing order lets a retailer say "this basket, from this supplier, every Tuesday and Friday"
and have Pallet place and pay for it automatically.

## User Stories

### US-001 — Set up a standing order (P1)

As a retailer, I want to turn a basket into a standing order on chosen route days so that I
stop re-placing the same order.

- **Given** a basket for one supplier, **when** the retailer chooses route days and confirms,
  **then** the standing order is saved and shows its next occurrence date.

**Independent test:** create a standing order for Tuesday and Friday; the next occurrence
shows the coming Tuesday.

### US-002 — It just happens (P1)

As a retailer, I want each occurrence placed and paid without me doing anything, so that the
delivery arrives even when I forget.

- **Given** a standing order with an occurrence due, **when** the supplier's cut-off passes,
  **then** an order exists for the route day, the supplier sees it with their other orders,
  and the retailer's saved payment method has been charged.
- **Given** the charge fails, **when** the occurrence runs, **then** the retailer is told the
  same day and the order is still placed.

**Independent test:** set a standing order; advance to the cut-off; confirm an order and a
charge exist.

### US-003 — Skip or pause (P1)

As a retailer, I want to skip the next occurrence or pause the standing order so that a bank
holiday or a closure does not produce a delivery.

- **Given** a standing order, **when** the retailer skips the next occurrence before the
  cut-off, **then** no order is placed for that date and the following one is unchanged.
- **Given** a paused standing order, **when** an occurrence date passes, **then** nothing is
  placed.

**Independent test:** skip Friday; confirm no Friday order and a Tuesday order.

### US-004 — Change the basket (P2)

As a retailer, I want to change quantities or products in a standing order so that it
follows what I sell.

- **Given** a standing order, **when** the retailer changes a quantity, **then** the change
  applies from the next occurrence and the history shows who changed it and when.

### US-005 — Supplier sees what is coming (P2)

As a supplier, I want to see standing-order demand for the next four weeks so that I can plan
production.

- **Given** standing orders from several retailers, **when** the supplier opens the forecast,
  **then** it shows quantities per product per route day for the next four weeks.

## Functional Requirements

- **FR-001** A retailer MUST be able to create a standing order from a basket for a single
  supplier, choosing one or more of that supplier's route days. *Source: US-001*
- **FR-002** The system MUST place and charge each occurrence automatically at the supplier's
  cut-off on the day before each chosen route day, using the retailer's saved payment method.
  *Source: US-002*
- **FR-003** A retailer MUST be able to skip the next occurrence up to the cut-off, and pause
  or resume a standing order at any time. *Source: US-003*
- **FR-004** A retailer MUST be able to change the products and quantities of a standing
  order; changes apply from the next occurrence. *Source: US-004*
- **FR-005** The retailer MUST be notified the same day when an occurrence's charge fails;
  the order is still placed. *Source: US-002*
- **FR-006** The retailer MUST receive a summary of each placed occurrence. *Source: US-002*
- **FR-007** A supplier MUST be able to see forecast standing-order demand per product per
  route day for the next four weeks. *Source: US-005*

## Success Criteria

- **SC-001** Standing orders are placed reliably at the cut-off.
- **SC-002** 30% of retailers with three or more repeat orders to the same supplier set up a
  standing order within 60 days of launch, measured from the standing-orders table.
- **SC-003** Fewer than 1 in 200 occurrences leads to a support ticket, measured monthly from
  the helpdesk tag.

## Quality expectations

The portal should feel fast when a retailer edits a standing order. Placement must be highly
available on cut-off mornings — that is when the money is. Payment data must be secure. The
supplier forecast should be accurate.

## Assumptions

- Every retailer has a saved payment method, so every occurrence can be charged at placement.
- Suppliers keep their cut-off time up to date.
- Prices are those in the catalogue.

## Out of scope

- Standing orders spanning more than one supplier.
- Retailer approval before each occurrence (a later feature).

## Open questions

None outstanding at sign-off.
