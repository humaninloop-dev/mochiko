# FEAT-007 — Shift swaps

> Status: delivered
> since 2026-08-14 · sticky — live rows may still be visible below

## Capability

Staff and managers move shifts between people after the week is published: a staff member
offers a shift, a manager posts an unfilled one, eligible colleagues take or claim, the
manager approves, and everyone involved is told.

## Extent

- An offer is visible only to eligible colleagues: same role, no availability or time-off conflict.
- On approval the rota updates; on decline the shift stays with the original holder, with the decline visible.
- A manager posts an unfilled published shift to the site's staff (merged from FEAT-006, 2026-08-20).
- Two claims on a posted shift: the first is on top; the manager approves one and the other is told.
- A posted shift can carry a note from the manager ("needs someone who can close").
- Eligible staff get a push notification within a minute of an offer or a posting.
- The manager's week view counts the week's swaps and claims.
- A declined claimant can re-claim if the shift is re-posted.
- Not: cover from a linked site in the group — pending row below.

## Work rows

- `pending` — cover a shift from a linked site in the group · acceptance: a barista at Canal Street takes an offer at Northgate; both rotas update · cut by shift-cover · **disputed — see `reviews/story-review-notes.md`**

## Relations

- composes-with: FEAT-002, FEAT-003 — eligibility reads availability and approved time off
- composes-with: FEAT-008 — eligible colleagues are told of an offer or a posting

## Story trace

- shift-cover: US-001, US-002, US-003, US-004, US-005, US-009
