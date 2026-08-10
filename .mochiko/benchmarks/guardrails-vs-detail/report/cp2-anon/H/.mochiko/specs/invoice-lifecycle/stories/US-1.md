### User Story 1 - Create a client record (Priority: P1)

A contractor adds a client (name, email, optional business details) so they have someone to
invoice. The client record is reused across every invoice sent to that client.

**Why this priority**: A client is the required subject of an invoice — nothing in the get-paid loop can start without one. Blocks US-2 through US-7.

**Independent Test**: Sign in as a contractor with no clients, add one client, and confirm it appears in the client list and is selectable when drafting an invoice. Passing = the saved client persists and is scoped to this contractor only.

**Acceptance Scenarios**:
1. **Given** a signed-in contractor on an empty client list, **When** they add a client with a name and a valid email, **Then** the client is saved and shown in their client list.
2. **Given** a contractor adding a client, **When** they submit without a name or with a malformed email, **Then** the record is rejected with a field-level message and nothing is saved.
3. **Given** two contractors each with their own clients, **When** contractor A views their client list, **Then** only A's clients appear and B's are never visible.

---

**Feature mapping** (assigned at derivation): homed to FEAT-001 (Client management).
