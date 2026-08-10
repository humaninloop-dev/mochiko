# US-3 — Draft an invoice (Priority: P1)

A contractor drafts an invoice for a client — adding line items (description, quantity, unit price), a per-invoice tax rate, and a due date — and sees the totals computed as they go.

**Why this priority**: The invoice itself is the product's central object; without drafting there is nothing to send or collect on. Core spine.

**Independent Test**: Create a draft with two line items and a tax rate, verify subtotal, tax, and total are computed exactly, then reopen the saved draft and confirm the values persist. Passing = totals match exact-decimal arithmetic and the draft persists.

**Acceptance Scenarios**:
1. **Given** a signed-in contractor with a saved client, **When** they add two line items and a tax rate and save, **Then** the invoice persists as a draft with subtotal, tax, and total computed exactly.
2. **Given** a draft invoice, **When** a line item quantity or price changes, **Then** the subtotal, tax, and total recompute to match without rounding drift.
3. **Given** a draft with no line items, **When** the contractor tries to send it, **Then** sending is refused until at least one line item exists.

**Feature**: FEAT-003 (homed at derivation)
