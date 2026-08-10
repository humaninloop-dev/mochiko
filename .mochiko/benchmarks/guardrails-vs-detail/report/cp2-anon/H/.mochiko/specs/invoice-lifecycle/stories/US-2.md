### User Story 2 - Draft an invoice (Priority: P1)

A contractor creates an invoice for one of their clients — adding line items (description,
quantity, unit price), a tax rate, a due date, an invoice number, and an optional memo/notes
line — and sees the running subtotal, tax, and total before saving it as a draft. The client
can be picked from the list or added inline without leaving the editor.

**Why this priority**: The invoice itself is the core artifact of the get-paid loop; without a correct, saved draft there is nothing to send or collect on. Blocks US-3 through US-7.

**Independent Test**: As a contractor with at least one client, create an invoice with two line items and a tax rate, save it, reopen it, and confirm the persisted totals match a hand calculation. Passing = totals are exact to the cent and the draft reloads unchanged.

**Acceptance Scenarios**:
1. **Given** a contractor with a client, **When** they add two line items and a tax rate and save, **Then** the invoice is stored as a draft with subtotal, tax, and total computed exactly (to the cent).
2. **Given** a contractor drafting an invoice, **When** they enter a negative quantity or a non-numeric price, **Then** the line is rejected with a field-level message and the total is not recalculated from bad input.
3. **Given** a saved draft invoice, **When** the contractor reopens it, **Then** every line item, the tax rate, the due date, the invoice number, the memo, and the totals are shown exactly as saved.

---

**Feature mapping** (assigned at derivation): homed to FEAT-002 (Invoice authoring).
