# FEAT-007 — Shift swaps

> Status: in-flight
> since 2026-07-22 · live work rows in run(s): `.mochiko/specs/shift-cover/`

## Capability

A staff member offers a published shift to colleagues at their site; an eligible colleague
takes it; the manager approves, and the published rota shows the change.

## Extent

- An offer is visible only to eligible colleagues: same role, no availability or time-off conflict.
- On approval the rota updates; on decline the shift stays with the original holder, with the decline visible.
- Not: a weekly summary of swaps — cut as a pending row below.

## Work rows

- `live` — offer, take, approve · acceptance: offer a Saturday shift, a colleague takes it, approve; the published week shows the colleague · in shift-cover
- `live` — eligibility: same role, no availability or time-off conflict · acceptance: a floor colleague and a colleague on approved leave see nothing; a free barista sees the offer · in shift-cover
- `pending` — weekly cover summary on the manager's week view · acceptance: after three swaps the week view shows 3 · cut by shift-cover

## Relations

- composes-with: FEAT-002, FEAT-003 — eligibility reads availability and approved time off
- composes-with: FEAT-008 — eligible colleagues are told of an offer

## Story trace

- shift-cover: US-001, US-003, US-005, US-009
