# US-3 — Send an invoice with a payment link (Priority: P1)

A contractor sends a draft invoice to the client by email; the email carries the invoice details and a hosted Stripe payment link the client can pay from without an account.

**Why this priority**: Sending is the hand-off from contractor to client and the gateway to getting paid; a lifecycle that never sends is just a form — MVP-blocking.

**Independent Test**: Send a `draft` invoice to a client email; confirm an email is dispatched containing a working payment link, and the invoice moves to `sent` with a recorded send timestamp.

**Acceptance Scenarios**:
1. **Given** a `draft` invoice with a client that has an email, **When** the contractor sends it, **Then** an invoice email with a hosted payment link is dispatched and the invoice status becomes `sent`.
2. **Given** a `sent` invoice, **When** the contractor resends it, **Then** the client receives the email again against the same payment link without creating a duplicate invoice.
3. **Given** the email provider rejects or bounces the send, **When** the contractor sends, **Then** the invoice does not silently show as `sent`; the failure is surfaced so the contractor can retry.

**Feature**: FEAT-004 (homed) — assigned at derivation.
