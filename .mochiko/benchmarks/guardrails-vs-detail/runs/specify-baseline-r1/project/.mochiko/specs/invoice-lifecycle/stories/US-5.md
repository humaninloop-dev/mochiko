# US-5 — Get paid online and see it reflected (Priority: P1)

A client pays a sent invoice through the hosted payment page, and the contractor sees the invoice flip to paid without ever logging into the payment processor.

**Why this priority**: Collecting payment and trusting the reflected state is the whole reason the product exists; this closes the spine end to end.

**Independent Test**: Pay a sent invoice through the hosted page in test mode, then confirm the invoice shows paid to the contractor and reconciles against the processor as source of truth; replay the payment notification and confirm the state does not change again. Passing = a real payment flips status to paid exactly once and survives a replayed notification.

**Acceptance Scenarios**:
1. **Given** a sent invoice, **When** the client completes payment on the hosted page, **Then** the invoice status becomes paid for the contractor and the paid amount matches the invoice total.
2. **Given** a payment already recorded, **When** the same payment notification is delivered again, **Then** the invoice stays paid exactly once with no double-recording.
3. **Given** a forged or unverified payment notification, **When** it arrives, **Then** it is rejected and no invoice changes to paid.

**Feature**: FEAT-005 — Online payment collection (leaf under FEAT-004 Payments). Homed at derivation.
