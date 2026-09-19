# Spec — Time and attendance

- **Status:** specify — stories drafted, feature derivation pending
- **Author:** ops lead (first draft, 2026-06-08). Story review by the requirements analyst
  is booked for 2026-06-15; please do not wait for it to derive the features — the founder
  wants the map delta before then.
- **Feature derivation:** product seat.

## Intent

Site managers spend Sunday evening working out who actually worked what from paper sign-in
sheets and WhatsApp messages, then typing hours into a spreadsheet for the accountant. Two
of our three-site groups have said they will churn to a competitor that "does clock-in" if
we do not. We want one feature: time and attendance.

## Stories

Written as tickets — the ops lead's words. Priorities are the ops lead's.

### US-001 — Add a clock-in / clock-out button (P1)

Add a big clock-in button on the staff app home screen; it becomes clock-out once tapped.
Record the time. Only show it within 30 minutes of a scheduled shift.

### US-002 — Break buttons (P1)

Add "start break" and "end break" buttons while clocked in. Record the break so it is taken
off the worked hours.

### US-003 — Managers fix a wrong time (P1)

Let a manager edit a clock-in or clock-out time when someone forgot to tap. Keep the original
time visible so it is obvious it was edited.

### US-004 — CSV export for the accountant (P1)

Export the week's worked hours per person as a CSV the accountant can open. Columns: name,
site, role, hours, overtime hours.

### US-005 — Text staff a reminder before their shift (P2)

Send an SMS one hour before a shift starts ("You're on at 14:00 at Northgate"). Set up
Twilio for it. Managers keep saying no-shows happen because people forget.

### US-006 — Fix: rota shows UTC times for the Manchester sites (P1)

The Manchester group sees shifts an hour off since the clocks changed. Fix the timezone
handling on the rota view.

### US-007 — Copy last week (P2)

Add a "copy last week" action on an empty draft so the manager does not rebuild the same
week every Sunday.

### US-008 — Move the shifts table to Postgres (P1)

Before we add clock-ins we need to get the `shifts` table out of the SQLite sidecar and into
Postgres properly, or the clock-in records will have nothing reliable to join to.

### US-009 — Overtime flag (P2)

When someone's worked hours in a week go over their contracted hours, flag it on the week's
hours view so the manager sees it before export.

### US-010 — Manager sign-off before export (P1)

Managers should approve each person's hours for the week before the CSV can be exported, so
the accountant never gets unapproved numbers.

### US-011 — Later: send hours straight to Xero (P3)

Not for this release. Eventually, push approved hours into Xero payroll instead of the CSV.
Listing it so it is not forgotten.

## Proposed features (for the map) — engineering lead, 2026-06-09

To save the product seat some time, here is how engineering would carve the map for this
spec. The groupings follow the sprint plan so each feature lands in one sprint:

- **Feature: Sprint 14 — clock-in foundations** — US-008, US-001, US-002
- **Feature: Sprint 15 — hours and export** — US-003, US-004, US-010
- **Feature: Nice-to-haves** — US-005, US-007, US-009, US-011, US-006

Or, if the map prefers one entry per spec, a single **Time & attendance** feature covering
all eleven stories.
