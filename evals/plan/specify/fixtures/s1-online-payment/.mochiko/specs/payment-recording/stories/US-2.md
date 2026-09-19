# US-2 — Chase an unpaid invoice on a schedule

**Priority:** P1 · **Feature:** FEAT-004 · **Disposition:** homed

As a studio owner, I set reminder offsets once so that overdue invoices are chased without
me remembering.

## Acceptance scenarios

- **Given** offsets of 3, 7, and 14 days, **When** an invoice is 7 days overdue, **Then** the
  second reminder email goes out with the public link.
- **Given** a reminder due tomorrow, **When** the invoice is paid today, **Then** no reminder
  is sent.

## Independent test

Freeze the clock, advance it past each offset, and assert one email per offset.
