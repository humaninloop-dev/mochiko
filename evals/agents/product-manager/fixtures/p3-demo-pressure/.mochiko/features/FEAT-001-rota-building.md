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
- Not: a staff member on more than one site's rota.
- Not: the product suggesting who should fill a shift — the manager chooses every name.

## Relations

- composes-with: FEAT-002 — availability conflicts are shown on the draft
- composes-with: FEAT-003 — approved time off blocks a shift on the draft
- composes-with: FEAT-007 — an approved swap changes who holds a published shift

## Story trace

- time-and-attendance: US-007
- rota-basics: US-001, US-002, US-003, US-005
