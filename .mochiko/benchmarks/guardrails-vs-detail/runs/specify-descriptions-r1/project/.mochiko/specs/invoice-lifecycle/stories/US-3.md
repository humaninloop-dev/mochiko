# US-3 — Draft an invoice (Priority: P1)

A contractor drafts an invoice for a client with line items, a single tax rate, and a due date, and sees the total before sending.

**Why this priority**: The invoice is the product's central object; without drafting there is nothing to send, track, or get paid.

**Independent Test**: Create a draft with two line items and a tax rate, save, reopen. Passing = line totals, tax, and grand total are computed exactly and persist; the invoice carries a sequential number.

**Acceptance Scenarios**:
1. **Given** a client exists, **When** the contractor adds line items (description, quantity, unit price), a tax rate, and a due date, **Then** the invoice shows each line total, the tax amount, and a grand total computed exactly to the cent.
2. **Given** a saved draft, **When** the contractor reopens it, **Then** all line items, the tax rate, the due date, and the assigned invoice number are unchanged.
3. **Given** the account's last invoice number was N, **When** a new invoice is created, **Then** it is assigned number N+1 with no gap.

**Folded from prototype review**: the editor must let the contractor jump to add a new client without losing the in-progress draft (new clients are common), and must make unsaved-draft state visible so a draft is never mistaken for saved.

**Disposition**: homed (FEAT-ID assigned at derivation).
