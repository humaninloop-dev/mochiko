# US-7 — Automated overdue reminder emails (Priority: P2)

The system automatically emails a client reminders when their invoice is overdue — at 3, 7, and 14
days past the due date — so the contractor doesn't have to chase manually. A contractor can turn
reminders off for a specific invoice.

**Why this priority**: The nudge is a primary reason a contractor would pay for Ledgerline
(principal ruling on the prototype walk — reminders are core v1). It depends on the invoice spine
and the computed-overdue read, so it is prioritized just below them, not cut.

**Independent Test**: Seed a `sent` invoice past its due date with reminders on, and one with
reminders off. Advance time across the 3/7/14-day marks; verify a reminder email is sent at each
mark for the on-invoice and none for the off-invoice, and that the invoice being paid or voided
before a mark cancels the remaining reminders. Passing = reminders fire on the cadence only for
eligible, reminders-on invoices and stop on settlement; failing = reminders on paid/void invoices,
wrong cadence, or the per-invoice toggle ignored.

**Acceptance Scenarios**:
1. **Given** a `sent`, reminders-on invoice that has been overdue for 3 days, **When** the reminder schedule runs, **Then** the client is emailed an overdue reminder, and again at 7 and 14 days overdue.
2. **Given** an overdue invoice, **When** it is paid or voided before the next mark, **Then** no further reminders are sent.
3. **Given** an invoice with reminders turned off, **When** it becomes overdue, **Then** no reminder emails are sent.

**Feature**: FEAT-008 (Overdue reminder emails). Depends on FEAT-001 and the computed-overdue read
in FEAT-005.

> Note: originally drafted as a rejection candidate (reminder cadence was undecided at intent). The
> principal reversed this on the prototype walk — reminders are core v1 — and ruled the cadence
> (3/7/14 days overdue) and the per-invoice off toggle. Homed, not rejected.
