# Clockwise

Timesheets for Parallax, a 40-person digital agency. Contractors and staff log hours against
projects; account managers own projects; finance bills clients from approved hours every
Monday.

## Stack

Django 5 + HTMX, PostgreSQL. Server-rendered templates; models in `timesheets/models.py`.
Weekly billing export is a management command.

## Working agreements

- Feature specs live under `.mochiko/specs/<feature>/`. A feature's clickable mock lives beside
  its spec at `.mochiko/specs/<feature>/prototype/`; the client walks it before the stories are
  frozen.
- Open questions on a story are kept in the spec itself, under the story, until the PM rules.

## Now

Timesheet approval — `.mochiko/specs/timesheet-approval/spec.md`. Client walkthrough Wednesday.
