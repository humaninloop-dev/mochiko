# US-7 — Automatic overdue detection & payment reminders (Priority: P2)

Unpaid invoices past their due date are flagged overdue automatically, and the client receives reminder emails on a set cadence until the invoice is paid.

**Why this priority**: Chasing payment is the pain the product removes, but the core loop (draft → send → get paid) ships and proves value without automated reminders; reminders complete the experience rather than gate it.

**Independent Test**: Create an invoice with a past due date and run the scheduled job; pay it and confirm reminders stop. Passing = status flips to `overdue` automatically, reminders send at due / +3 / +7 days (max 3, per-invoice off-switch honored), and no reminder sends after `paid`.

**Acceptance Scenarios**:
1. **Given** a sent invoice whose due date has passed unpaid, **When** the overdue check runs, **Then** the invoice status becomes `overdue` automatically without any contractor action.
2. **Given** an overdue invoice with reminders enabled, **When** the reminder schedule reaches due / +3 / +7 days, **Then** an authenticated reminder email is sent to the client, capped at three reminders total.
3. **Given** an invoice with reminders turned off, or one that becomes `paid`, **When** the reminder job runs, **Then** no further reminder is sent for that invoice.

**Folded from prototype review**: reminder state (next reminder date, or reminders-off) must be visible on the invoice detail screen, not only behind a settings link — it is how the contractor checks whether a client is being chased.

**Disposition**: homed (FEAT-ID assigned at derivation).
