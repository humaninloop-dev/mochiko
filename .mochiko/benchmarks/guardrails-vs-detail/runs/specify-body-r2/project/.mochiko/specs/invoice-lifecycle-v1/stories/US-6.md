# US-6 — Track invoice status at a glance (Priority: P1)

A contractor sees all their invoices and each one's payment state on a dashboard — draft, sent, viewed, paid, and an overdue indicator — so they know who owes them without logging into Stripe.

**Why this priority**: Seeing payment state without opening Stripe is an explicit core promise; the dashboard is where the contractor lives day to day — MVP-blocking.

**Independent Test**: With invoices in several states (draft, sent, viewed, paid, and one past its due date), open the dashboard and confirm each shows the correct state and the past-due unpaid one shows an overdue indicator.

**Acceptance Scenarios**:
1. **Given** invoices in mixed states, **When** the contractor opens the dashboard, **Then** each invoice shows its current status and its client, amount, and due date.
2. **Given** an unpaid invoice whose due date has passed, **When** the dashboard renders, **Then** that invoice shows an overdue indicator derived from the due date (no separate stored state required).
3. **Given** an invoice becomes paid, **When** the dashboard is next viewed, **Then** it reflects `paid` and drops any overdue indicator.

**Feature**: FEAT-006 (homed) — assigned at derivation.
