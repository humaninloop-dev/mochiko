# US-2 — Client pays via Stripe-hosted checkout without an account (Priority: P1)

A client opens the payment link from their invoice, pays through Stripe-hosted checkout without
creating any account or logging in, and the invoice reconciles to `paid` against Stripe as the
source of truth. Opening the link marks the invoice `viewed`.

**Why this priority**: This is the other half of the spine — getting money in and having the
system trust that a `paid` is really paid. The whole product bet (contractors trust payment state
without re-checking Stripe) rests here.

**Independent Test**: Take a `sent` invoice with a payment link. Open the link (verify status
becomes `viewed`). Complete a Stripe test-mode payment. Verify the invoice reconciles to `paid`
only on a signature-verified Stripe event, and that replaying the same event does not double-apply.
Passing = `paid` reflects a real settled Stripe payment, exactly once; failing = status changes
without a verified event, or a replay changes state.

**Acceptance Scenarios**:
1. **Given** a `sent` invoice, **When** the client opens the payment link, **Then** they reach Stripe-hosted checkout with no login or account step, the invoice is marked `viewed`, and a `viewed` row with a timestamp is written to its status history.
2. **Given** the client completes payment, **When** Stripe sends the signature-verified payment event, **Then** the invoice reconciles to `paid` and the contractor sees `paid` without re-checking Stripe.
3. **Given** a payment event already processed, **When** the same event is replayed or re-delivered, **Then** payment state is unchanged (exactly-once).

**Feature**: FEAT-003 (Stripe-hosted payment & reconciliation) — homed at derivation.
