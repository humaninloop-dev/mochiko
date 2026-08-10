# US-1 — Create a client

**Feature:** FEAT-001

As a contractor, I want to add a client with their contact details so I have someone to
address and send invoices to.

**Why this priority (P1)**: Nothing can be invoiced without a client on file; this blocks the
whole create-to-paid spine.

**Independent Test**: With a logged-in contractor account, add a client via the client form;
verify the client is saved, scoped to that account only, and appears when starting a new
invoice. Passing = client persists and is selectable; failing = not saved or visible to
another account.

**Acceptance Scenarios**:
1. **Given** a logged-in contractor, **When** they save a client with name and email, **Then** the client is stored under their account and shown in their client list — one line.
2. **Given** a client form, **When** the contractor submits without a client email, **Then** the form is rejected with a clear validation message and nothing is saved — one line.
3. **Given** two separate contractor accounts, **When** contractor A lists clients, **Then** contractor B's clients never appear — one line.
