# US-8 — Invoice dashboard & payment-status view (Priority: P1)

A contractor sees all invoices and their live payment status in one place, so they know who owes what and what is overdue without logging into Stripe.

**Why this priority**: Seeing payment state in one place is half the product's stated value ("see payment state without logging into Stripe"); the dashboard is where the contractor lives day to day.

**Independent Test**: With invoices across every status, open the dashboard and filter by status. Passing = each invoice shows its correct current status and amount, totals of outstanding vs paid are correct, and filtering by overdue returns exactly the overdue set.

**Acceptance Scenarios**:
1. **Given** invoices in draft, sent, viewed, paid, and overdue states, **When** the contractor opens the dashboard, **Then** each invoice is listed with its current status, client, amount, and due date.
2. **Given** the dashboard, **When** the contractor filters by `overdue`, **Then** only overdue invoices are shown and an outstanding-total reflects their sum.
3. **Given** an invoice whose payment just reconciled, **When** the contractor refreshes the dashboard, **Then** it reflects the new `paid` status without any visit to Stripe.

**Disposition**: homed (FEAT-ID assigned at derivation).
