# US-2 — Draft an invoice (Priority: P1)

A contractor creates an invoice for a client — line items (description, quantity, unit price), a tax rate, and a due date — and saves it as a draft they can revise before sending.

**Why this priority**: The invoice is the core artifact of the product; drafting is the entry point to the whole lifecycle — MVP-blocking.

**Independent Test**: With a signed-in contractor and one client, create a draft invoice with two line items, a tax rate, and a due date; confirm the computed total (line items + tax) is correct and the invoice is saved in `draft` status and editable.

**Acceptance Scenarios**:
1. **Given** a contractor with at least one client, **When** they add line items, a tax rate, and a due date and save, **Then** the invoice is stored as `draft` with a correct computed subtotal, tax, and total.
2. **Given** a draft invoice, **When** the contractor edits a line item or the due date, **Then** the totals recompute and the invoice remains a `draft`.
3. **Given** a contractor tries to save an invoice with no line items or a due date in the past, **When** they submit, **Then** the save is rejected with a message naming the problem.

**Feature**: FEAT-003 (homed) — assigned at derivation.
