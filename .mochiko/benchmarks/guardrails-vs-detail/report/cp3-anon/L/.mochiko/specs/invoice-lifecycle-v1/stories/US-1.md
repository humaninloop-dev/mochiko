# US-1 — Manage clients (Priority: P1)

A contractor adds and maintains the people they bill — name, email, and billing details — so an invoice can be addressed to a real client without re-typing details each time.

**Why this priority**: A client is the addressee of every invoice; nothing in the lifecycle starts without one — MVP-blocking.

**Independent Test**: With a signed-in contractor and an empty client list, create a client with name + email, confirm it appears in the list and is selectable when drafting an invoice; edit it and confirm the change persists.

**Acceptance Scenarios**:
1. **Given** a signed-in contractor with no clients, **When** they add a client with a name and email, **Then** the client appears in their client list and is selectable when drafting an invoice.
2. **Given** an existing client, **When** the contractor edits the client's email, **Then** the updated email is saved and used on future invoices (not retroactively on already-sent ones).
3. **Given** a contractor tries to add a client with a blank name or malformed email, **When** they submit, **Then** the client is not created and a field-level error explains what to fix.

**Feature**: FEAT-002 (homed) — assigned at derivation.
