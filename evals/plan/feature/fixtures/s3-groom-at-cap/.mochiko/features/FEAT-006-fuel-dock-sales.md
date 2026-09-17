# FEAT-006 — Fuel dock sales

> Status: proposed
> surfaced by fuel-dock (2026-09-12)

## Capability

Diesel and petrol sold at the fuel pump, paid on the spot or put on the skipper's account,
with the tank stock tracked.

## Extent

- A sale records litres, grade, and price, against a boat when it is on account.
- Tank stock counts down per sale and up per delivery.
- Not: fuel cards or loyalty schemes.

## Work rows

- `pending` — a pump sale on account or paid, on the skipper's statement · acceptance: a 40 L diesel sale on account appears on the boat's next invoice as a fuel line · cut by fuel-dock [EPIC-001]
- `pending` — tank stock per delivery and sale · acceptance: a 5 000 L delivery then two sales leaves the stock at 5 000 minus the litres sold · cut by fuel-dock [EPIC-001]

## Relations

- composes-with: FEAT-003 — an on-account sale rides the boat's invoice

## Story trace

- fuel-dock: US-001, US-002, US-003, US-004
