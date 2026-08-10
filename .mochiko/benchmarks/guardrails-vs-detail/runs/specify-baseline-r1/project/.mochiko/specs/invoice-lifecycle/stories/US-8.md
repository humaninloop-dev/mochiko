# US-8 — Automatic payment reminders (Priority: P2)

Ledgerline automatically emails the client reminders on unpaid invoices on a cadence the contractor can see and adjust, so the contractor does not have to chase payment by hand.

**Why this priority**: Automated reminders are the effort-saving payoff of the product, but they depend on sending and status tracking being in place first, so they layer on last.

**Independent Test**: Configure a reminder cadence on an unpaid, past-due invoice, advance time to a reminder point, and confirm a reminder email is sent and stops once the invoice is paid. Passing = reminders fire per the configured cadence on unpaid invoices and cease immediately on payment.

**Acceptance Scenarios**:
1. **Given** an unpaid invoice and a configured cadence, **When** a reminder point is reached, **Then** the client receives a reminder email and the send is recorded on the invoice.
2. **Given** reminders are scheduled, **When** the invoice is paid (online or manually), **Then** no further reminders are sent.
3. **Given** a contractor adjusts the cadence, **When** the next reminder point is evaluated, **Then** the new cadence governs when the reminder is sent.

**Feature**: FEAT-009 — Automated reminders (leaf under FEAT-007 Payment follow-up). Homed at derivation; deferred (not selected this round).
