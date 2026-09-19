# FEAT-004 — Time clock

> Status: delivered
> since 2026-07-06 · sticky — live rows may still be visible below

## Capability

Staff clock in and out of their shifts from the app and record breaks; a manager corrects a
missed or wrong time, with the original time kept visible.

## Extent

- Clock in and out within 30 minutes of a scheduled shift; breaks recorded while clocked in.
- A manager edits a time; the original stays on the record, marked edited.
- Not: clocking in without a scheduled shift — the button does not appear.

## Relations

- depends-on: FEAT-001 — a clock-in attaches to a published shift
- composes-with: FEAT-005 — worked time feeds the week's timesheet

## Story trace

- time-and-attendance: US-001, US-002, US-003
