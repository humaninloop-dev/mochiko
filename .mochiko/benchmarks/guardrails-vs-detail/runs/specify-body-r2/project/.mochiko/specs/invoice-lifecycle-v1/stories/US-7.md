# US-7 — Automatic payment reminders (Priority: P2)

Unpaid invoices trigger reminder emails to the client on a schedule the contractor controls, so the contractor doesn't have to manually chase late payers.

**Why this priority**: High value for getting paid, but a contractor can still complete the full signup-to-paid lifecycle by hand without it — the intent ruled reminders the piece to cut last, hence P2.

**Independent Test**: Configure a reminder schedule, create a sent unpaid invoice whose reminder is due, run the reminder cycle, and confirm a reminder email is sent; then mark the invoice paid and confirm no further reminders go out.

**Acceptance Scenarios**:
1. **Given** a sent unpaid invoice and an active reminder schedule, **When** a reminder becomes due, **Then** a reminder email with the payment link is sent to the client and the send is logged.
2. **Given** an invoice that becomes paid (online or manual), **When** later reminders would have fired, **Then** no further reminders are sent for that invoice.
3. **Given** a contractor turns reminders off, **When** reminders would otherwise be due, **Then** none are sent.

**Feature**: FEAT-007 (homed) — assigned at derivation.
