# US-4 — Record an off-platform payment (Priority: P2)

A contractor whose client paid by cash or check marks the invoice paid manually, capturing that
the settlement happened outside Stripe, so the invoice reconciles to `paid` like any other.

**Why this priority**: Not everything goes through Stripe; contractors take cash and checks. Needed
for a complete experience, but the Stripe spine can ship first without it.

**Independent Test**: Take a `sent` invoice. Mark it paid manually with a method (cash/check) and a
date. Verify status becomes `paid`, the settlement is recorded as manual (not a Stripe event), and
the audit trail records who marked it and when. Passing = a manually-settled `paid` invoice with an
audit entry; failing = no manual path, or manual `paid` indistinguishable from a Stripe `paid`.

**Acceptance Scenarios**:
1. **Given** a `sent` (or overdue) invoice paid off-platform, **When** the contractor marks it paid and records the method and date, **Then** the invoice becomes `paid` and the settlement is recorded as manual.
2. **Given** a manual mark-as-paid, **When** the invoice is later inspected, **Then** the audit trail shows the contractor, the timestamp, and that settlement was manual (not Stripe).
3. **Given** an already-`paid` invoice, **When** the contractor attempts to mark it paid again, **Then** the action is refused (no double settlement).

**Feature**: FEAT-004 (Manual payment recording) — homed at derivation.
