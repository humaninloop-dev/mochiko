### User Story 8 - Sign in to the account (Priority: P1)

A contractor signs in to Ledgerline with an email and password, or with "Sign in with Google,"
and only sees and acts on their own account's data once signed in. Added to scope at the
selection gate: the founder ruled authentication is spine foundation, folded into this build
(not a separate spec).

**Why this priority**: Every other spine feature requires an authenticated contractor scoped to their own data; nothing in the get-paid loop is reachable without sign-in. Foundation — ordered first.

**Independent Test**: Register/sign in with email+password and separately with Google, then confirm the session lands on the signed-in contractor's own client and invoice data and cannot reach another account's. Passing = both methods authenticate, session is scoped to the account, sign-out ends access.

**Acceptance Scenarios**:
1. **Given** a contractor with valid credentials, **When** they sign in with email and password, **Then** they reach their own account and see only their own clients and invoices.
2. **Given** a contractor choosing "Sign in with Google," **When** they complete Google sign-in, **Then** they are signed in to their Ledgerline account with the same scoped access.
3. **Given** wrong credentials, **When** they attempt to sign in, **Then** access is denied with a non-specific error and no account data is exposed.

---

**Feature mapping** (assigned at derivation): homed to FEAT-009 (Authentication). Added at the selection gate by user ruling (auth folded into the build).
