### User Story 4 - Client pays online via Stripe (Priority: P1)

A client opens the payment link, pays the invoice through Stripe's hosted checkout, and the
invoice is marked paid automatically once Stripe confirms the payment — without the contractor
touching anything.

**Why this priority**: Online payment is the fastest path to "paid" and the reason a contractor sends a link at all; it closes the get-paid loop. Core spine.

**Independent Test**: Open a sent invoice's payment link, complete a Stripe test-mode payment, and confirm the invoice flips to paid from the confirmed Stripe event (not from the browser redirect alone). Passing = invoice paid, amount matches, a duplicate/forged event does not double-record.

**Acceptance Scenarios**:
1. **Given** a sent invoice, **When** the client completes payment on Stripe's hosted checkout, **Then** the invoice is marked paid on Stripe's confirmed event and the contractor sees it as paid.
2. **Given** a payment confirmation event, **When** the same event is delivered more than once, **Then** the invoice is recorded paid exactly once and the amount is not double-counted.
3. **Given** the client abandons or fails checkout, **When** they return without completing payment, **Then** the invoice stays unpaid and no partial payment is recorded.

---

**Feature mapping** (assigned at derivation): homed to FEAT-005 (Online payment via Stripe), a leaf of FEAT-004 (Payments).
