# FEAT-004 — Clock-in

> Status: in-flight
> since 2026-08-18 · live work rows in run(s): `.mochiko/specs/tablet-offline/`

## Capability

Staff clock in and out of their shifts on the venue tablet and record breaks; a manager
corrects a time with the original kept.

## Extent

- Clock-in matched to the published shift; a clock-in with no shift is flagged to the manager.
- Not: payroll export — hours are visible, never exported.

## Work rows

- `live` — offline clock-in kept on the tablet and reconciled when the network returns ·
  acceptance: a clock-in made with wifi off appears on the manager dashboard within one minute
  of wifi returning · in tablet-offline

## Story trace

- clock-in: US-1, US-2
- tablet-offline: US-1
