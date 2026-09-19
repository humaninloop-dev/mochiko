# FEAT-005 — Timesheets

> Status: delivered
> since 2026-07-31 · sticky — live rows may still be visible below

## Capability

Each person's worked hours for the week, built from the time clock; overtime against
contracted hours is flagged; a manager signs the week off per person and exports it.

## Extent

- Weekly hours per person per site from clock records; breaks deducted; overtime flagged.
- A manager approves each person's week; only approved weeks export.
- CSV export of approved hours per person per week, one file per site.
- Not: pay calculation — hours only, no rates, no pay shown to anyone.
- Not: a staff member seeing their own week's hours — managers only.

## Work rows

- `pending` — approved hours pushed into Xero payroll · acceptance: a week's approved hours appear in Xero without a file · cut by time-and-attendance

## Relations

- depends-on: FEAT-004 — hours come from clock records

## Story trace

- shift-cover: US-009 (extend obligation: a swapped shift counts for the person who worked it — lands with FEAT-007's rota update, still in-flight)
- time-and-attendance: US-004, US-009, US-010, US-011
