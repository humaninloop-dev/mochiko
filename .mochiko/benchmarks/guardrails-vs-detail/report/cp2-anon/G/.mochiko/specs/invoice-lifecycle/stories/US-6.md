# US-6 — Manually mark an invoice paid (Priority: P2)

A contractor who was paid by check or cash marks the invoice paid by hand, so the record matches reality even when money did not move through the online link.

**Why this priority**: Real contractors are still paid offline; the complete experience needs it, but the online spine can ship first, so it layers on after.

**Independent Test**: On a sent invoice, mark it paid manually with a method and date, and confirm status becomes paid and the manual action is attributed in the invoice history. Passing = manual mark-as-paid flips status and records who marked it, when, and by what method.

**Acceptance Scenarios**:
1. **Given** a sent invoice paid offline, **When** the contractor marks it paid and records the method, **Then** the invoice becomes paid and the history notes it was a manual payment.
2. **Given** an invoice already paid online, **When** the contractor attempts a manual mark-paid, **Then** the action is refused or shown as already paid, never double-counting the payment.

**Feature**: FEAT-006 — Manual payment recording (leaf under FEAT-004 Payments). Homed at derivation; deferred (not selected this round).
