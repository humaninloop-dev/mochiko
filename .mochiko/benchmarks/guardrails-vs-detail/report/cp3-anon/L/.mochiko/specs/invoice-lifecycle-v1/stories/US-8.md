# US-8 — Void an invoice (Priority: P2)

A contractor voids an invoice that should no longer be paid — a mistake or a disputed amount — so it stops chasing the client and no longer counts as owed.

**Why this priority**: Needed to stop reminders nagging a client mid-dispute and to retire bad invoices; the lifecycle is usable without it at launch, so P2. Ruled in by the principal at the acceptance gate (resolves OQ-1).

**Independent Test**: On a `sent` unpaid invoice, void it; confirm it moves to `voided`, drops off the owed/overdue view, and no further reminders are sent; confirm a `paid` invoice cannot be voided from the app.

**Acceptance Scenarios**:
1. **Given** a `sent` unpaid invoice, **When** the contractor voids it, **Then** it becomes `voided`, is excluded from overdue/owed views, and its reminders stop.
2. **Given** a `paid` invoice, **When** the contractor views it, **Then** the void action is unavailable — voiding never reverses money Stripe already collected.
3. **Given** a `voided` invoice, **When** the dashboard renders, **Then** it shows a `voided` state and carries no overdue indicator.

**Feature**: FEAT-003 (homed) — assigned at derivation; added by principal ruling at the acceptance gate.
