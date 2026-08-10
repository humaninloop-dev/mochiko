# US-8 — See when a client viewed the invoice

**Feature:** FEAT-005

As a contractor, I want to know when a client has opened/viewed the invoice, so I can tell the
difference between "hasn't seen it" and "seen it and not paid" before I follow up.

**Why this priority (P2)**: Useful context for follow-up but not required to get paid; the
signal is best-effort (see Assumptions — derived from a hosted-page visit, not email-open
tracking, which is unreliable).

**Independent Test**: Simulate a client visiting the hosted invoice/payment page for a sent
invoice; verify the invoice shows `viewed` and that a never-opened invoice stays `sent`.
Passing = `viewed` set only on a genuine hosted-page visit; failing = `viewed` set with no
visit or never set after one.

**Acceptance Scenarios**:
1. **Given** a sent invoice, **When** the client opens the hosted payment page for it, **Then** the invoice status shows `viewed` to the contractor — one line.
2. **Given** a sent invoice never opened by the client, **When** the contractor checks it, **Then** it remains `sent`, not `viewed` — one line.
