# US-2 — Draft an invoice

**Feature:** FEAT-003

As a contractor, I want to draft an invoice for a client with line items, a tax rate, and a
due date, and see the total computed for me, so the amount owed is correct before I send it.

**Why this priority (P1)**: The invoice itself is the core artifact of the product; a correct
total is the highest-rigor part of the money path.

**Independent Test**: Create a draft invoice with two line items and a tax rate; verify the
displayed total equals the exact sum of line items plus tax, and the invoice is saved in
`draft` status. Passing = total is exact to the cent and status is `draft`; failing = rounding
error or wrong status.

**Acceptance Scenarios**:
1. **Given** a selected client, **When** the contractor adds line items (description, quantity, unit price), a tax rate, and a due date, **Then** the invoice saves as `draft` with a subtotal, tax, and total computed exactly — one line.
2. **Given** a draft with line items, **When** the contractor edits a quantity or unit price, **Then** the subtotal, tax, and total recompute to match, still exact to the cent — one line.
3. **Given** a draft invoice form, **When** the contractor submits with no line items or a due date in the past, **Then** the invoice is rejected with a clear message and stays unsent — one line.
