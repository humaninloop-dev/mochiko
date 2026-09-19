# FEAT-009 — Labour cost forecast

> Status: in-flight
> since 2026-08-24 · live work rows in run(s): `.mochiko/specs/labour-cost/`

## Capability

The projected wage cost of a drafted week, shown against the site's weekly budget as the
manager adds and removes shifts.

## Extent

- Each staff member carries an hourly rate; the draft shows the week's cost and the budget.
- Not: the cost of a published week after the fact — drafts only, for now.

## Work rows

- `live` — cost against budget on the draft · acceptance: add a shift and the figure moves; exceed the budget and the draft says so · in labour-cost

## Relations

- depends-on: FEAT-001 — cost is computed on the draft

## Story trace

- labour-cost: US-001, US-002
