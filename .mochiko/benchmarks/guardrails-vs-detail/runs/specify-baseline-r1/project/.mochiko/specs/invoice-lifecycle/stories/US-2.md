# US-2 — Manage clients (Priority: P1)

A contractor adds and edits the clients they bill, so an invoice can be addressed to a saved client instead of re-typing details each time.

**Why this priority**: An invoice must be addressed to someone; a client record is a precondition of drafting, so it ships in the core spine.

**Independent Test**: With a signed-in account, create a client, edit its details, and list clients; confirm the saved values persist and belong only to this account. Passing = a client can be created, edited, and listed with correct persisted values.

**Acceptance Scenarios**:
1. **Given** a signed-in contractor, **When** they add a client with name, email, and billing address, **Then** the client is saved and appears in their client list.
2. **Given** an existing client, **When** the contractor edits its email, **Then** the updated email is persisted and used on future invoices.
3. **Given** a client with a required field left blank, **When** the contractor tries to save, **Then** the save is refused with a clear message naming the missing field.

**Feature**: FEAT-002 (homed at derivation)
