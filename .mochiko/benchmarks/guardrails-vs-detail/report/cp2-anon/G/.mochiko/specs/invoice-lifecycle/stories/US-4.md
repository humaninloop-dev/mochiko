# US-4 — Send an invoice with a payment link (Priority: P1)

A contractor sends a finished invoice to the client by email, carrying a hosted online-payment link, and the invoice moves from draft to sent.

**Why this priority**: Sending is how the invoice reaches the client and how payment becomes possible; it is the hinge of the spine between drafting and collecting.

**Independent Test**: Send a draft to a test client address, confirm the email is delivered with a working payment link and the invoice status becomes sent. Passing = a delivered email with a valid link and a status transition to sent, recorded in the invoice's history.

**Acceptance Scenarios**:
1. **Given** a valid draft invoice, **When** the contractor sends it, **Then** the client receives an email containing the invoice and a hosted payment link, and the status becomes sent.
2. **Given** a send attempt to an address that hard-bounces, **When** the bounce is reported, **Then** the contractor is notified the invoice was not delivered and the invoice does not appear paid.
3. **Given** an invoice already sent, **When** the contractor sends it again, **Then** a fresh email is delivered without creating a duplicate invoice or losing prior history.

**Feature**: FEAT-003 — Invoicing (draft & send). Homed at derivation; sending is part of the Invoicing capability alongside drafting.
