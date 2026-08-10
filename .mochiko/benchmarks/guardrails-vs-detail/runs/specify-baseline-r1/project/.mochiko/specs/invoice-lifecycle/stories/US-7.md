# US-7 — Track invoice status through its lifecycle (Priority: P2)

A contractor sees where each invoice stands — including when a client has viewed it and when it has passed its due date — so they know what needs attention without asking.

**Why this priority**: Visibility into viewed and overdue states makes chasing payment effortless, but the spine can collect payment before this lands, so it is layered.

**Independent Test**: Open a sent invoice as the client to trigger viewed, then advance time past the due date on an unpaid invoice and confirm it shows overdue in the contractor's list. Passing = viewed registers on client open and overdue appears automatically once due date passes while unpaid.

**Acceptance Scenarios**:
1. **Given** a sent invoice, **When** the client opens it, **Then** the contractor sees the invoice marked viewed with the time it happened.
2. **Given** an unpaid invoice past its due date, **When** the due date passes, **Then** the invoice automatically shows as overdue in the contractor's list.
3. **Given** an overdue invoice, **When** it is subsequently paid, **Then** it moves to paid and no longer counts as overdue.

**Feature**: FEAT-008 — Invoice status lifecycle (leaf under FEAT-007 Payment follow-up). Homed at derivation; deferred (not selected this round).
