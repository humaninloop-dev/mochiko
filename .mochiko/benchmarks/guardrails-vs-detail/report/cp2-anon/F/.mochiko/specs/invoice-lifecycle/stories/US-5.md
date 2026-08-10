# US-5 — Client pays online; payment auto-reconciled (Priority: P1)

A client pays an invoice through the hosted payment page, and the contractor sees it move to paid without checking Stripe.

**Why this priority**: Automatic reconciliation is the product's core promise — trusting payment state without re-checking Stripe; it is what makes payment tracking real rather than manual.

**Independent Test**: Complete a hosted payment in Stripe test mode and let the webhook fire; replay the same webhook event. Passing = the invoice becomes `paid` exactly once, money recorded as Decimal, and the replay changes nothing.

**Acceptance Scenarios**:
1. **Given** a sent invoice, **When** the client completes payment on the hosted page and Stripe sends the payment webhook, **Then** the invoice status becomes `paid` and the paid amount and date are recorded.
2. **Given** a payment webhook already processed, **When** the same event is replayed or a forged event arrives, **Then** its signature/idempotency check rejects the duplicate and payment state is unchanged.
3. **Given** a webhook that cannot be matched to an invoice, **When** it is received, **Then** it is logged for review and no invoice is silently altered.

**Disposition**: homed (FEAT-ID assigned at derivation).
