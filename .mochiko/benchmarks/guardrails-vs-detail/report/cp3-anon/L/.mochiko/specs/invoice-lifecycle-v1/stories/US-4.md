# US-4 — Client pays online (Priority: P1)

A client opens the payment link from the invoice email and pays with a card through Stripe's hosted page; the contractor's invoice reflects the payment without the contractor touching Stripe.

**Why this priority**: Getting paid online, hands-off, is the product's core promise — signup to a paid invoice without the founder in the loop — MVP-blocking.

**Independent Test**: Follow a sent invoice's payment link, complete a test payment on Stripe's hosted page, and confirm the invoice flips to `paid` with the paid amount and timestamp recorded from the confirmed Stripe event (not from the redirect alone).

**Acceptance Scenarios**:
1. **Given** a `sent` invoice with a payment link, **When** the client completes payment on the hosted page, **Then** the invoice becomes `paid` and records the amount and time from the confirmed payment event.
2. **Given** a client opens the payment page but abandons before paying, **When** they leave, **Then** the invoice stays unpaid and (if detectable from the hosted page open) is marked `viewed`.
3. **Given** a duplicate or replayed payment notification arrives, **When** it is processed, **Then** the invoice is not double-marked or double-counted; the paid state is recorded once.

**Feature**: FEAT-005 (homed) — assigned at derivation.
