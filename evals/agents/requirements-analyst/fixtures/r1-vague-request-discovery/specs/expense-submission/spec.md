# Feature Specification — Expense Submission

- **Feature:** Expense Submission
- **Status:** Shipped (revision 4)
- **Author:** analyst seat
- **Date:** 14 March 2026

## Overview

An employee files an expense in under a minute from a receipt photo, and the expense reaches
their manager's approval queue with everything the manager needs to decide. Before this, expenses
were emailed as photos and re-keyed by finance.

## User Stories

### US-001 — File an expense from a receipt (P1)

As an employee, I want to file an expense by photographing the receipt and confirming the amount
so that I am reimbursed without re-typing what the receipt already says.

- **Given** an employee photographs a receipt, **when** the capture finishes, **then** the amount,
  currency, and date are pre-filled from the receipt and the employee can correct any of them
  before filing.
- **Given** the receipt image is unreadable, **when** capture finishes, **then** the form opens
  with empty fields and the employee can type them.
- **Given** an employee files an expense, **when** it is saved, **then** it appears in their
  manager's approval queue within 10 seconds.

**Independent test:** photograph a printed receipt for 12.40 GBP dated today; confirm the three
fields pre-fill, file it, and confirm it is in the manager's queue.

### US-002 — Withdraw a filed expense (P2)

As an employee, I want to withdraw an expense I filed by mistake so that my manager does not have
to reject it and I do not have to explain.

- **Given** an expense is filed and not yet decided, **when** the employee withdraws it, **then**
  it leaves the manager's queue and is marked withdrawn in the employee's list.
- **Given** an expense has been approved, **when** the employee looks at it, **then** there is no
  withdraw action.

**Independent test:** file an expense, withdraw it, confirm it is gone from the manager's queue.

## Functional Requirements

- **FR-001** The system MUST read amount, currency, and date from a receipt image and pre-fill
  them, with each field editable before filing. *Source: US-001*
- **FR-002** The system MUST require a category and a date incurred; the date incurred MUST NOT be
  in the future or more than 180 days in the past. *Source: US-001*
- **FR-003** The system MUST copy the employee's cost center onto the expense at filing time.
  *Source: US-001*
- **FR-004** A filed expense MUST appear in the approving manager's queue within 10 seconds of
  filing. *Source: US-001*
- **FR-005** An employee MUST be able to withdraw an expense while it is filed and undecided, and
  MUST NOT be able to withdraw it once decided. *Source: US-002*
- **FR-006** The receipt image MUST be at most 10 MB; larger images are refused with the limit
  stated. *Source: US-001*

## Success Criteria

- **SC-001** Median time from opening the capture screen to filing is under 60 seconds, measured
  from the capture-opened and filed events.
- **SC-002** At least 90% of filed expenses have the amount unchanged from the pre-filled value,
  measured monthly.
- **SC-003** Finance re-keys zero expenses from email within 30 days of a company's launch,
  measured by the count of manually created expenses.

## Assumptions

- **A-001** The receipt currency is the currency printed on the receipt; conversion to the company
  currency happens at filing using that day's rate (see `docs/data-notes.md`).
- **A-002** A receipt image is one file; multi-page receipts are filed as separate expenses.

## Open Questions

- None open. Revision 2's question on the 180-day limit was settled with finance (Tom, 2 March).

## Out of Scope

- Mileage claims — a separate form with a rate table.
- Per-diem allowances.
