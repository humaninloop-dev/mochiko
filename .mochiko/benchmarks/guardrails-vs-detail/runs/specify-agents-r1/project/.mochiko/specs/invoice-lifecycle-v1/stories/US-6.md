# US-6 — See overdue invoices at a glance (Priority: P2)

A contractor sees which sent-but-unpaid invoices are past their due date, shown as an overdue
indicator in the list and detail, so they know who to chase — without overdue being a state an
invoice is migrated into.

**Why this priority**: Chasing late payers is a core contractor pain, but the indicator is a
computed read over data the spine already carries; it can follow the first slice.

**Independent Test**: Seed sent invoices with due dates in the past and in the future, plus a paid
past-due invoice. Load the list and detail; verify only unpaid, past-due, sent invoices show the
overdue indicator, that no stored status changed, and that a paid past-due invoice is not flagged.
Passing = accurate computed overdue indication; failing = overdue shown on paid/void invoices, or
overdue stored as a status.

**Acceptance Scenarios**:
1. **Given** an unpaid invoice in `sent` or `viewed` status whose due date has passed, **When** the contractor views the list or detail, **Then** it shows an overdue indicator while its stored status is unchanged.
2. **Given** an unpaid invoice whose due date is in the future, **When** the contractor views it, **Then** no overdue indicator is shown.
3. **Given** a `paid` or `void` invoice past its due date, **When** the contractor views it, **Then** no overdue indicator is shown.

**Feature**: FEAT-005 (Invoice tracking & lifecycle view) — homed at derivation (overdue extent).
