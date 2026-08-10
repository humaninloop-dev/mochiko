# US-6 — See invoice statuses at a glance

**Feature:** FEAT-005

As a contractor, I want a list of my invoices showing each one's current status, so I can see
who has paid and who hasn't without logging into the payment processor.

**Why this priority (P1)**: This is the principal's explicit success moment — trusting the
app's payment state at a glance is the product's core promise.

**Independent Test**: With invoices in draft, sent, and paid states plus one past its due date
and unpaid, load the list; verify each shows the correct status and the past-due unpaid one
carries an overdue indicator. Passing = every status correct and overdue flagged; failing = a
stale or wrong status shown.

**Acceptance Scenarios**:
1. **Given** invoices in mixed states, **When** the contractor opens the invoice list, **Then** each invoice shows its current status (draft / sent / viewed / paid) — one line.
2. **Given** a sent invoice past its due date and still unpaid, **When** the list renders, **Then** that invoice is flagged overdue (computed from the due date, not a separate stored state) — one line.
3. **Given** an invoice, **When** the contractor opens its detail view, **Then** they see its line items, total, status, and payment history for that invoice — one line.
