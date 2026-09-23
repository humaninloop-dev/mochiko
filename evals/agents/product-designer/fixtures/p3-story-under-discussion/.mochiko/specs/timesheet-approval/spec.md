# Feature Specification — Timesheet approval

- **Status:** stories drafted · client walkthrough Wednesday · **Author:** Hannah (PM) · **Date:** 2026-09-07

## Summary

Contractors submit a week of hours; someone approves it; finance bills from approved hours.
Today approval is an email thread and finance bills from whatever was in the sheet on Monday.

## User Stories

### US-001 — Submit my week (P1)

As a contractor, I want to submit my week's timesheet, so that it can be approved and billed.

- **Scenario 1 — Given** I have entries for the week, **when** I submit it, **then** the week
  is locked for editing, shows as submitted, and the approver is notified.
- **Scenario 2 — Given** I have submitted a week that has not yet been approved, **when** I
  recall it, **then** it is editable again and no longer shows as submitted.

Independent test: submit a week and see it locked; recall it and see it editable.

### US-002 — Approve or reject a week (P1) — *under discussion*

As an account manager, I want to approve a submitted timesheet, so that its hours can be
billed to my client.

- **Scenario 1 — Given** a submitted timesheet, **when** I approve it, **then** its hours are
  locked for billing and the contractor is notified.
- **Scenario 2 — Given** a submitted timesheet, **when** I reject it with a comment, **then**
  the contractor sees the comment, the week is editable again, and they can resubmit.

Independent test: approve one week and see it locked; reject another with a comment and see the
contractor's view carry the comment.

#### Discussion (open — owner: Hannah, decision after Wednesday)

- **Dana (AM):** I only know my own projects. When Ximena's week has BRM, OKD and MFG on it I
  can vouch for the BRM rows and nothing else. Approval has to be per project — I approve my
  rows, Marco approves his, the week is "approved" when every AM has signed their part.
- **Marco (AM):** Per-project approval means a week can sit half-approved for days and finance
  can't bill any of it. One approver per week — whoever owns the largest project on it — and
  they check with the others if they need to. That is how the email thread works today.
- **Hannah:** The story as written says "a submitted timesheet", which reads as the whole week.
  Not ruling until the client has seen it. The data model currently has one `approved_by` per
  week (Dana's note in `timesheets/models.py`).

### US-003 — See where my weeks stand (P2)

As a contractor, I want to see the status of my recent weeks, so that I know what has been
billed and what needs my attention.

- **Scenario 1 — Given** I have weeks in different states, **when** I open my timesheets,
  **then** I see each week with its status (draft, submitted, approved, rejected) and total
  hours.

Independent test: with one week in each state, open the list and see all four with the right
status and totals.

## Functional Requirements

- **FR-001** A submitted week MUST NOT be editable by the contractor until it is recalled or
  rejected. *Source: US-001*
- **FR-002** Finance MUST bill only approved hours. *Source: US-002*
- **FR-003** Hours MUST be entered in quarter-hour increments. *Source: US-001*

## Screens & Flows

_To be filled by the prototype._
