# Spec — fuel-dock

Status: accepted (2026-09-12) · capability: FEAT-006 · selected rows: 2 (EPIC-001)

## Stories

- US-001 As the dock hand, I record a diesel or petrol sale by litres and grade.
- US-002 As the dock hand, I put a sale on a boat's account or take payment on the spot.
- US-003 As the office, I see the fuel line on the boat's next invoice.
- US-004 As the harbour master, I see the tank stock after every delivery and sale.

## Functional requirements

- FR-001 A sale records litres, grade, unit price, and the boat when on account.
- FR-002 An on-account sale appears as a fuel line on the boat's next invoice.
- FR-003 Tank stock decreases per sale and increases per recorded delivery.

## Success criteria

- SC-001 A 40 L on-account sale shows on the next invoice within the same day.
- SC-002 Stock after a delivery and two sales equals delivery minus litres sold.
