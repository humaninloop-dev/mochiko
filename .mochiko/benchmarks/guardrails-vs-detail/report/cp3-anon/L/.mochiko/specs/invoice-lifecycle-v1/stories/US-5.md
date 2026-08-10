# US-5 — Record a manual payment (Priority: P1)

A contractor who is paid by cash or check marks the invoice paid themselves, so the invoice's payment state is accurate regardless of how the money arrived.

**Why this priority**: Contractors are frequently paid outside the card rail; without manual mark-paid the status board lies and reminders would chase already-paid invoices — MVP-blocking.

**Independent Test**: On a `sent` (unpaid) invoice, mark it paid manually with a payment date; confirm it flips to `paid`, records that it was paid manually, and stops any pending reminders.

**Acceptance Scenarios**:
1. **Given** a `sent` unpaid invoice, **When** the contractor marks it paid manually with a date, **Then** the invoice becomes `paid`, recorded as a manual payment, and no further reminders are sent.
2. **Given** an invoice already `paid` online, **When** the contractor views it, **Then** the manual mark-paid action is unavailable so it cannot be double-recorded.
3. **Given** a contractor marks an invoice paid by mistake, **When** they undo within the same session, **Then** the invoice returns to its prior unpaid status and the audit trail records both the mark and the reversal.

**Feature**: FEAT-005 (homed) — assigned at derivation.
