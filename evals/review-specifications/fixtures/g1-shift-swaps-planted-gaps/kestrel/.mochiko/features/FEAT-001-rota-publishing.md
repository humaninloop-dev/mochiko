# FEAT-001 — Rota publishing

> Status: delivered
> since 2026-03-02 · sticky — live rows may still be visible below

## Capability

A venue manager drafts a week's shifts by role and time, checks eligibility and availability
conflicts, and publishes; staff see the published week and any later change to it.

## Extent

- Shifts carry a role (usher, box office, bar, duty manager), start, end, and venue.
- Publishing notifies staff per decision 0007; a change to a published shift is recorded with
  its actor and path.
- Not: any staff-initiated change to a published shift.

## Relations

- composes-with: FEAT-002 — availability conflicts flagged on the draft
- composes-with: FEAT-003 — approved time off blocks a shift on the draft

## Story trace

- rota-basics: US-1, US-2, US-3
- rota-changes: US-1, US-2
