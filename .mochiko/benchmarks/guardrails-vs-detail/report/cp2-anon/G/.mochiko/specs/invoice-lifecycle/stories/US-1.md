# US-1 — Contractor account & sign-in (Priority: P1)

A contractor signs up and signs in to reach a workspace that holds only their own clients and invoices — with email+password or Sign in with Google.

**Why this priority**: Nothing else in the lifecycle can exist without an authenticated account to own the data and enforce per-account isolation; it is the foundation the whole spine stands on.

**Independent Test**: Register a new account, sign out, sign back in with each method, and confirm the workspace is empty and scoped to that account; attempt to reach another account's data and confirm it is refused. Passing = both sign-in methods work and no cross-account data is ever visible.

**Acceptance Scenarios**:
1. **Given** a visitor with no account, **When** they register with email+password, **Then** an account is created and they land in an empty workspace scoped to them.
2. **Given** a returning contractor, **When** they choose Sign in with Google, **Then** they reach their own workspace with their existing clients and invoices.
3. **Given** a signed-in contractor, **When** any request would read or write data, **Then** only rows belonging to their account are ever returned or changed.

**Feature**: FEAT-001 (homed at derivation)
