# US-3 — Monitor invoices and their status (Priority: P1)

A contractor sees all their invoices in one list with each invoice's current lifecycle status
(draft / sent / viewed / paid / void), and opens any invoice to a detail view showing its client,
line items, amounts, due date, and status history.

**Why this priority**: The contractor "lives in this" — the list and detail are how they observe
that the spine works at all. Without a status view, a sent or paid invoice is invisible.

**Independent Test**: Seed invoices in each status for one contractor and one decoy contractor.
Load the list; verify exactly the owner's invoices appear, each with its correct status. Open one;
verify the detail shows its client, line items, amounts, due date, and status history. Passing =
accurate, tenant-scoped list and detail; failing = wrong status, missing invoice, or a decoy's
invoice leaking in.

**Acceptance Scenarios**:
1. **Given** a contractor with invoices in several statuses, **When** they open the invoice list, **Then** every one of their invoices appears with its correct current status and none belonging to another contractor.
2. **Given** the invoice list, **When** the contractor opens one invoice, **Then** the detail view shows client, line items, amounts, due date, and current status.
3. **Given** an invoice whose status has changed over time (created, sent, viewed, paid), **When** the contractor views its detail, **Then** the status history (who or what changed it, and when — including the `viewed` timestamp) is visible.

**Feature**: FEAT-005 (Invoice tracking & lifecycle view) — homed at derivation.
