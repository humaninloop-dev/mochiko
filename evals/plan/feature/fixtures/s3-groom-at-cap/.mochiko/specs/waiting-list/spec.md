# Spec — waiting-list

Status: accepted (2026-09-09) · capability: FEAT-007 · selected rows: 1 (EPIC-001)

## Stories

- US-001 As a skipper, I join the waiting list for a berth size and dates.
- US-002 As the office, I see who is next for a size and offer a freed berth in order.

## Functional requirements

- FR-001 The list orders by join date within a berth size.
- FR-002 Freeing a berth shows the next skipper for its size; the office sends the offer.

## Success criteria

- SC-001 Two skippers join for 12 m; freeing a 12 m berth offers it to the earlier one.
