# FEAT-007 — Waiting list

> Status: proposed
> surfaced by waiting-list (2026-09-09)

## Capability

A skipper joins the waiting list for a berth of a given size and dates; the office offers the
next free berth in list order.

## Extent

- The list is ordered by the date the skipper joined; the office sees who is next for a size.
- Not: automatic allocation; the office offers, the skipper accepts.

## Work rows

- `pending` — join the list for a size and dates; the office offers in order · acceptance: two skippers join for a 12 m berth; freeing one offers it to the first · cut by waiting-list [EPIC-001]

## Relations

- depends-on: FEAT-001 — an accepted offer becomes a booking
- composes-with: FEAT-005 — the offer goes to the skipper

## Story trace

- waiting-list: US-001, US-002
