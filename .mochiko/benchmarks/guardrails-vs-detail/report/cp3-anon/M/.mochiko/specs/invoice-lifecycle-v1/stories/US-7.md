# US-7 — Send payment reminders

**Feature:** FEAT-006

As a contractor, I want reminders to go out for unpaid invoices on intervals I can configure,
so I get paid without chasing clients myself.

**Why this priority (P2)**: Valuable for cash flow but not part of the create-to-paid spine;
the spine ships and earns money without it. Reminder defaults and bounds are deferred (see
Assumptions).

**Independent Test**: Configure a reminder interval on an unpaid sent invoice and advance time
past it; verify a reminder email goes out, and that a paid invoice never triggers one. Passing
= reminder sent only while unpaid and only per configured schedule; failing = reminder for a
paid invoice or off-schedule.

**Acceptance Scenarios**:
1. **Given** a sent, unpaid invoice with a configured reminder interval, **When** the interval elapses, **Then** a reminder email is sent to the client — one line.
2. **Given** an invoice that becomes paid, **When** a reminder was scheduled, **Then** no further reminders are sent for it — one line.
