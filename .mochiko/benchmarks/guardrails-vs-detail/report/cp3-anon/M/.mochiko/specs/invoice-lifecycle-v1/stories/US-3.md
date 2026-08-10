# US-3 — Send an invoice with a payment link

**Feature:** FEAT-004

As a contractor, I want to send a draft invoice to my client by email carrying a hosted
payment link, so the client can pay online without me handling card details.

**Why this priority (P1)**: Sending is the hand-off from contractor to client and the gate to
getting paid; without it the spine cannot complete.

**Independent Test**: Send a draft invoice to a test client address; verify an email is
dispatched containing a working hosted payment link and the invoice status moves `draft` →
`sent`. Passing = email sent, link resolves to the hosted payment page for that invoice's exact
amount; failing = no email, broken link, or wrong amount.

**Acceptance Scenarios**:
1. **Given** a draft invoice with a total, **When** the contractor clicks Send, **Then** the client receives an email with a hosted payment link for the exact invoice amount and the status becomes `sent` — one line.
2. **Given** an already-sent invoice, **When** the contractor sends it again, **Then** the client is re-notified without creating a duplicate charge or a second payable link — one line.
3. **Given** the email provider fails to accept the message, **When** the contractor sends, **Then** the invoice does not silently show `sent`; the failure is surfaced and the invoice stays `draft` (or is flagged send-failed) — one line.
