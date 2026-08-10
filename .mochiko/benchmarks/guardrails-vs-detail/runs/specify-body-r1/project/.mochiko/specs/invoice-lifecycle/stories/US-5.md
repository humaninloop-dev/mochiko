### User Story 5 - Manually mark an invoice paid (Priority: P1)

When a client pays by check or cash, the contractor records that payment by hand — marking the
invoice paid, with the payment method and date — so the tool reflects reality even for
off-platform payments.

**Why this priority**: Contractors get paid by check and cash too; without manual marking the status view lies for a large share of real payments. Core spine.

**Independent Test**: On a sent, unpaid invoice, mark it paid as "check" with a date, and confirm it shows paid with that method recorded and appears in the paid set. Passing = status is paid, method/date captured, action attributed in the audit trail.

**Acceptance Scenarios**:
1. **Given** a sent, unpaid invoice, **When** the contractor marks it paid and selects a method (check/cash) and date, **Then** the invoice shows paid with that method and date recorded.
2. **Given** an invoice already paid online, **When** the contractor tries to also mark it paid manually, **Then** the tool prevents a second paid-recording and explains it is already paid.
3. **Given** a manual mark-paid, **When** it is recorded, **Then** the change is written to the invoice's history showing who marked it and when.

---

**Feature mapping** (assigned at derivation): homed to FEAT-006 (Manual payment recording), a leaf of FEAT-004 (Payments).
