# US-1 — Contractor sign-up & sign-in (Priority: P1)

A contractor creates an account and signs back in, so their invoices and clients are private to their business.

**Why this priority**: Nothing else in the product is reachable without an authenticated, isolated account; this is the foundation every other story stands on.

**Independent Test**: Register a new account, sign out, sign back in via email+password and via Google; confirm a second account cannot see the first's data. Passing = both sign-in paths work and cross-account reads return nothing.

**Acceptance Scenarios**:
1. **Given** no account, **When** the contractor registers with email + password, **Then** the account is created and they land on an empty invoice dashboard.
2. **Given** an existing account, **When** the contractor signs in with Sign in with Google using the same verified email, **Then** they reach their own dashboard, not a new empty account.
3. **Given** contractor A is signed in, **When** they request an invoice belonging to contractor B by its id, **Then** the system denies access and reveals nothing about B's data.

**Disposition**: homed (FEAT-ID assigned at derivation).
