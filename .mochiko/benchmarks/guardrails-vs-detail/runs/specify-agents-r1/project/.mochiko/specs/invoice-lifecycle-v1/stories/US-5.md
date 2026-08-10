# US-5 — Void an incorrect sent invoice (Priority: P2)

A contractor who sent an invoice with a mistake voids it, which stops it from being payable and
records who voided it and when, so a wrong invoice can be killed without deleting the record.

**Why this priority**: Mistakes on sent invoices happen and need a clean kill path. Follows the
spine — a contractor can work around a bad invoice manually until this ships.

**Independent Test**: Take a `sent` invoice. Void it. Verify status becomes `void`, its payment
link no longer accepts payment, the record is retained (not deleted), and the audit trail records
who voided it and when. Attempt to void a `paid` invoice; verify it is refused. Passing = a voided,
non-payable, retained invoice with an audit entry; failing = void deletes the record, a voided
invoice is still payable, or a paid invoice can be voided.

**Acceptance Scenarios**:
1. **Given** a `sent` (or overdue) invoice, **When** the contractor voids it, **Then** its status becomes `void`, it is no longer payable, and the record is retained.
2. **Given** a void action, **When** the invoice is later inspected, **Then** the audit trail shows who voided it and when.
3. **Given** a `paid` invoice, **When** the contractor attempts to void it, **Then** the action is refused.

**Feature**: FEAT-006 (Invoice void) — homed at derivation; deferred to a fast-follow at selection.
The audit-trail write on void is a cross-cutting obligation (GI-029), not owned by this feature.
