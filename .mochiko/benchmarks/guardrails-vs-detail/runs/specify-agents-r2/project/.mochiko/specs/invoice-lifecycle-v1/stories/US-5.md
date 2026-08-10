# US-5 — Mark an invoice paid manually

**Feature:** FEAT-004

As a contractor, I want to mark an invoice as paid myself when a client pays by check or cash,
so the app reflects reality even for payments that never touch the hosted page.

**Why this priority (P1)**: Contractors are paid off-platform often; without manual mark-paid
the status list lies, defeating the "see who's paid" success moment.

**Independent Test**: On a sent invoice, record a manual payment (method + optional note);
verify status becomes `paid` with the recorded method and timestamp, and the action is written
to the audit trail. Passing = `paid` with method recorded and an audit entry; failing = no
audit entry or a paid invoice re-payable.

**Acceptance Scenarios**:
1. **Given** a sent, unpaid invoice, **When** the contractor marks it paid with a method (check/cash) and date, **Then** the invoice becomes `paid` with method and timestamp recorded — one line.
2. **Given** an invoice already `paid` via the hosted page, **When** the contractor opens it, **Then** manual mark-paid is unavailable so the same invoice cannot be paid twice — one line.
3. **Given** a manual mark-paid action, **When** it is applied, **Then** an append-only audit record captures who marked it, when, and the method — one line.
