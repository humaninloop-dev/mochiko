### User Story 7 - Automatic overdue reminders (Priority: P2)

When a sent invoice passes its due date without being paid, the tool automatically emails the
client a reminder with the same payment link, so the contractor doesn't have to chase payment
by hand.

**Why this priority**: Reduces the manual chasing that contractors hate, but is an automation on top of the working spine; the loop closes without it, so it can follow the first build.

**Independent Test**: With a sent invoice whose due date is in the past and which is unpaid, run the reminder cycle and confirm one reminder email is delivered to the client and recorded on the invoice; confirm a paid invoice gets no reminder. Passing = exactly-one reminder to overdue-unpaid, none to paid.

**Acceptance Scenarios**:
1. **Given** a sent invoice past its due date and unpaid, **When** the reminder cycle runs, **Then** a reminder email carrying the payment link is delivered to the client and recorded on the invoice.
2. **Given** an invoice that has already been paid, **When** the reminder cycle runs, **Then** no reminder is sent for it.
3. **Given** an invoice that already received a reminder today, **When** the reminder cycle runs again the same day, **Then** it is not reminded twice in the same cycle window.

---

**Feature mapping** (assigned at derivation): homed to FEAT-008 (Automated overdue reminders). Deferred (not in the first-build selection).
