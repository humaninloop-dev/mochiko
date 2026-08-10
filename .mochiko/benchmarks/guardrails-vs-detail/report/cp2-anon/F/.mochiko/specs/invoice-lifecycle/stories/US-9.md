# US-9 — Void a sent invoice (Priority: P2)

A contractor voids an invoice that was sent in error or for a cancelled job, so a wrong invoice has a clean terminal state instead of lingering as owed.

**Why this priority**: Wrong amounts and cancelled jobs happen in the first week; a clean, audit-logged void is needed, but the core loop (draft → send → get paid) ships without it. Added by principal ruling at spec review (disposition #3).

**Independent Test**: Void a sent invoice; confirm it accepts no payment and drops out of outstanding totals. Passing = status becomes `void` (terminal), the payment link is dead, an audit row records the void, and outstanding totals exclude it.

**Acceptance Scenarios**:
1. **Given** a sent or overdue invoice sent in error, **When** the contractor voids it, **Then** its status becomes `void`, its payment link is deactivated, and an append-only audit entry records who voided it and when.
2. **Given** a voided invoice, **When** a payment (online or manual) is attempted against it, **Then** the payment is refused and the invoice stays `void`.
3. **Given** the dashboard outstanding total, **When** an invoice is voided, **Then** it is excluded from the outstanding total.

**Disposition**: homed to FEAT-006 (assigned at derivation; no in-place editing after send — void is the correction path).
