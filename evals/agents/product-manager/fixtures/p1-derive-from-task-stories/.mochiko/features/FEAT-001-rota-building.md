# FEAT-001 — Rota building

> Status: delivered
> since 2026-03-14 · sticky — live rows may still be visible below

## Capability

A manager drafts a week's shifts for a site — who works when, in which role — and publishes
the week. Staff see the published week; an unpublished draft is visible to managers only.

## Extent

- One rota per site per week; a staff member belongs to exactly one site.
- Shifts carry a role (barista, floor, kitchen, bar) and a start and end time.
- Publishing the week makes it visible to staff and locks it; a manager can re-open and re-publish.
- Not: copying a previous week into a draft — every week starts empty.
- Not: any notification to staff on publish — staff open the app to see the week.

## Relations

- composes-with: FEAT-002 — availability conflicts are shown on the draft
- composes-with: FEAT-003 — approved time off blocks a shift on the draft

## Story trace

- rota-basics: US-001, US-002, US-003, US-005
