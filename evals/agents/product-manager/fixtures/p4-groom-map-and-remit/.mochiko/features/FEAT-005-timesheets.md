# FEAT-005 — Timesheets

> Status: delivered
> since 2026-07-31 · sticky — live rows may still be visible below

## Capability

Each person's worked hours for the week, built from the time clock; overtime against
contracted hours is flagged; a manager signs the week off per person and exports it to the
site's payroll provider.

## Extent

- Weekly hours per person per site from clock records; breaks deducted; overtime flagged.
- A manager approves each person's week; only approved weeks export.
- Exports approved hours to any payroll provider — Xero, Sage, QuickBooks — or as CSV.
- Not: pay calculation — hours only, no rates.

## Work rows

- `live` — CSV export of approved hours per person per week · acceptance: the accountant opens the file and sees name, site, role, hours, overtime hours · in timesheets-export
- `pending` — approved hours pushed into Xero payroll · acceptance: a week's approved hours appear in Xero without a file · cut by time-and-attendance

## Relations

- depends-on: FEAT-004 — hours come from clock records

## Story trace

- time-and-attendance: US-004, US-009, US-010, US-011
