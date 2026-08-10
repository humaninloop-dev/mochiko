# US-6 — Mark an invoice paid manually (Priority: P1)

A contractor records a check or cash payment by marking an invoice paid by hand, so offline payments are reflected in the same place as online ones.

**Why this priority**: Contractors are paid by check and cash constantly; without a manual path the tracking view is wrong for a large share of real payments. The intent names manual mark-paid as always available.

**Independent Test**: Mark a `sent` invoice paid manually with a date and method; confirm the audit trail. Passing = status becomes `paid`, the manual method/date are recorded, and the change is written to the append-only log with who and when.

**Acceptance Scenarios**:
1. **Given** a sent or overdue invoice paid by check, **When** the contractor marks it paid and enters the date, **Then** the status becomes `paid` and the payment is recorded as a manual payment.
2. **Given** an invoice already marked paid, **When** the contractor attempts to mark it paid again, **Then** the system prevents a duplicate payment record and shows it is already paid.
3. **Given** any manual mark-paid action, **When** it is saved, **Then** an append-only audit entry records who changed the status, when, and from which state.

**Disposition**: homed (FEAT-ID assigned at derivation).
