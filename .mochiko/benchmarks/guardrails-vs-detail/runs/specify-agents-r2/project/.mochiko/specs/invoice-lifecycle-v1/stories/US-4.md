# US-4 — Get paid through the hosted payment page

**Feature:** FEAT-004

As a contractor, I want an invoice to become `paid` automatically when my client pays through
the hosted page, so I can trust the app's payment state without checking the payment processor.

**Why this priority (P1)**: This is the "get paid" end of the spine and the highest-trust part
of the product — payment state must reflect reality exactly once.

**Independent Test**: Simulate a completed hosted payment for a sent invoice via a signed
provider event; verify the invoice flips to `paid` exactly once, records the paid amount and
timestamp, and a replayed event does not double-apply. Passing = single `paid` transition on a
verified event; failing = state change on an unverified/forged event, or double application on
replay.

**Acceptance Scenarios**:
1. **Given** a sent invoice, **When** the client completes payment and a signature-verified provider event arrives, **Then** the invoice becomes `paid` with the amount and timestamp recorded — one line.
2. **Given** a provider event whose signature does not verify, **When** it is received, **Then** payment state does not change and the event is rejected — one line.
3. **Given** a payment event already applied, **When** the same event is delivered again, **Then** the invoice stays `paid` with no second payment recorded (idempotent) — one line.
