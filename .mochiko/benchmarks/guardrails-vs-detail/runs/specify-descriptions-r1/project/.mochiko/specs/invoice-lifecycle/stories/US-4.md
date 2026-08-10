# US-4 — Send an invoice with a payment link (Priority: P1)

A contractor sends a drafted invoice to the client by email, including a hosted payment link, and the invoice moves from draft to sent.

**Why this priority**: Sending is the moment the invoice reaches the payer; it is the hinge between drafting and getting paid, and defines the core happy path.

**Independent Test**: Send a draft to a test client email; inspect the delivered email. Passing = the email is authenticated (SPF/DKIM/DMARC), contains a working hosted payment link, and the invoice status is now `sent`.

**Acceptance Scenarios**:
1. **Given** a saved draft with a client email, **When** the contractor sends it, **Then** an authenticated email with the invoice details and a hosted payment link is delivered and the status becomes `sent`.
2. **Given** a sent invoice, **When** the client opens the payment link and views it, **Then** the invoice status advances to `viewed`.
3. **Given** the client's mail server rejects delivery (bounce), **When** the bounce is received, **Then** the contractor is notified the invoice was not delivered and the status does not falsely show `sent` as delivered.

**Open question (from prototype review)**: whether a client "view" of the hosted invoice link can be reliably detected. If detection is unreliable, the principal prefers dropping the `viewed` status rather than showing a status that may be wrong.

**Disposition**: homed (FEAT-ID assigned at derivation).
