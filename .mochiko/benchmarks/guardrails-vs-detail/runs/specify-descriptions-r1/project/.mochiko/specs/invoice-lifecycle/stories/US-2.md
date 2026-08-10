# US-2 — Manage clients (Priority: P1)

A contractor adds and edits the clients they bill, so an invoice can be addressed to a real payer with a name and email.

**Why this priority**: An invoice cannot be sent without a client to address and email it to; client records are a hard prerequisite for the send path.

**Independent Test**: Create a client with name + email, edit the email, list clients. Passing = the client persists, edits save, and the list shows only this account's clients.

**Acceptance Scenarios**:
1. **Given** a signed-in contractor, **When** they create a client with a name and email address, **Then** the client is saved to their account and appears in their client list.
2. **Given** a client with a typo in the email, **When** the contractor edits and saves the email, **Then** the corrected email is stored and used for the next invoice sent to that client.
3. **Given** a new client form, **When** the contractor submits it with a malformed email, **Then** the system rejects the save and explains which field is invalid.

**Disposition**: homed (FEAT-ID assigned at derivation).
