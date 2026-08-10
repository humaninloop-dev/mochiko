# US-8 — Manage clients (Priority: P1)

A contractor adds and maintains a saved client record — name and email, optionally a mailing
address — and selects that client when creating an invoice, instead of retyping details each time.

**Why this priority**: Surfaced on the prototype walk — invoicing starts from a saved client, not a
free-text field. It is a prerequisite for US-1 (a contractor invoices *against* a client), so it is
foundation for the slice.

**Independent Test**: For one contractor, add a client with name and email (and optionally an
address); verify it persists and is tenant-scoped (a decoy contractor cannot see it). Create an
invoice and verify the client is chosen from the saved list, not retyped. Passing = a saved,
tenant-scoped client selectable at invoice creation; failing = clients not saved, retyped per
invoice, or a decoy's client visible.

**Acceptance Scenarios**:
1. **Given** a logged-in contractor, **When** they add a client with a name and email (and optionally a mailing address), **Then** the client is saved and appears in their client list.
2. **Given** at least one saved client, **When** the contractor creates an invoice, **Then** they select the client from their saved list rather than retyping name and email.
3. **Given** two contractors, **When** each views their client list, **Then** each sees only their own clients (tenant isolation).

**Feature**: FEAT-007 (Client records).
