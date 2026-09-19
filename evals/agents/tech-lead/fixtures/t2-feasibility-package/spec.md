# Feature Specification — Reschedule delivery window (FEAT-011) — excerpt

- **Status:** design phase · **Author:** requirements-analyst seat · **Date:** 2026-09-07

## User Stories (in scope)

### US-001 — Move my delivery window (P1)

As a recipient, I want to move my delivery to another offered window so that I am home when
the parcel arrives.

- **Given** a shipment with a confirmed window, **when** the recipient picks another offered
  window, **then** the shipment shows the new window and the carrier's confirmation.
- **Given** the current window starts in less than two hours, **when** the recipient tries to
  move it, **then** the page refuses with a message naming the two-hour cut-off.

**Independent test:** move a window twice; confirm the carrier's record shows the last one.

### US-002 — Same-day rescue (P1)

As a recipient who is unexpectedly out, I want to move today's delivery to tomorrow up to two
hours before the window starts, so that the parcel is not returned to the depot.

- **Given** today's window starts at 14:00, **when** the recipient moves it at 11:30, **then**
  the carrier holds the parcel and the new window is confirmed.

**Independent test:** on a delivery day, move a 14:00 window at 11:30 and confirm the carrier
record shows the hold.

### US-003 — Reminder before the window (P2)

As a recipient, I want a reminder 24 hours before my window so that I can still move it in
time.

## Functional Requirements

- **FR-001** The system MUST offer the recipient the carrier's available windows for the
  shipment and accept one of them. *Source: US-001*
- **FR-002** The system MUST accept a window change up to two hours before the start of the
  current window, on any day including the delivery day. *Source: US-002*
- **FR-003** The system MUST show the recipient the new window only after the carrier has
  accepted it. *Source: US-001*
- **FR-004** The system MUST refuse a change inside the two-hour cut-off with a message naming
  the cut-off. *Source: US-001*
- **FR-005** The system MUST record every window change with who requested it and when, and
  retain the record for 24 months. *Source: NFR-005 (compliance)*
- **FR-006** The system MUST send a reminder 24 hours before the start of the confirmed window,
  and re-time it when the window moves. *Source: US-003*

## Success Criteria

- **SC-001** A window change is confirmed to the recipient within 300 ms at the 95th
  percentile, measured at the API gateway over one week of production traffic.
- **SC-002** 90% of same-day rescue attempts made before the cut-off succeed, measured from
  the change log over the first month.
