# FEAT-001 — Rota building

> Status: delivered
> since 2026-03-14 · sticky — live rows may still be visible below

## Capability

A manager drafts a week's shifts for a site — who works when, in which role — and publishes
the week. Staff see the published week; an unpublished draft is visible to managers only.

## Extent

- One rota per site per week; a staff member belongs to exactly one site.
- Shifts carry a role (barista, floor, kitchen, bar) and a start and end time.
- Publishing makes the week visible to staff and locks it; a manager can re-open and re-publish.
- A manager can copy the previous week into an empty draft.
- Not: any notification to staff on publish or change — staff open the app to see the week.
- Not: a manager changing who holds a published shift other than by re-opening the week.

## Relations

- composes-with: FEAT-002 — availability conflicts are shown on the draft
- composes-with: FEAT-003 — approved time off blocks a shift on the draft

## Story trace

- time-and-attendance: US-007
- rota-basics: US-001, US-002, US-003, US-005
