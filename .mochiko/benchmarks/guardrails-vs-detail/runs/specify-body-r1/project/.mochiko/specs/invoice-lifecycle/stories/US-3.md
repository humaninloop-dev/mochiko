### User Story 3 - Send an invoice with a payment link (Priority: P1)

A contractor sends a draft invoice to the client by email. The email carries a link the client
can open to view the invoice and pay it online. Once sent, the invoice moves out of draft.

**Why this priority**: Sending is the hand-off that turns a private draft into a request for payment — the whole point of the tool. Blocks US-4 (online payment) and US-7 (reminders).

**Independent Test**: Send a draft invoice to a test client address, confirm an email is delivered carrying a working link to that specific invoice, and confirm the invoice's state changes from draft to sent. Passing = email delivered, link resolves to the right invoice, state advanced.

**Acceptance Scenarios**:
1. **Given** a draft invoice, **When** the contractor chooses to send it, **Then** they are shown a preview of the email the client will receive before it goes out, and on confirming, an authenticated email is delivered to the client's address carrying a link to that invoice and the invoice becomes "sent".
2. **Given** a send attempt, **When** the email provider reports the address as undeliverable (bounce), **Then** the contractor is shown that delivery failed and the invoice does not silently appear as sent-and-delivered.
3. **Given** an already-sent invoice, **When** the contractor sends it again, **Then** the client receives the invoice again and no duplicate invoice record is created.

---

**Feature mapping** (assigned at derivation): homed to FEAT-003 (Invoice delivery).
