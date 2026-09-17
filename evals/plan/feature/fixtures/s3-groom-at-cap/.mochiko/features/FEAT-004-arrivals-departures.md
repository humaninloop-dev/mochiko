# FEAT-004 — Arrivals & departures

> Status: delivered
> since 2026-09-05 · sticky

## Capability

The office logs a boat in on arrival and out on departure; the board on the office tablet
shows today's movements.

## Extent

- An arrival is logged against its booking or pass, with the time.
- A departure, logged with one tap from the board, closes the stay and starts the departure invoice.
- The board on the office tablet lists today's expected arrivals and logged departures and rolls to the new day at midnight.
- Not: AIS or any automatic detection — the office taps.

## Relations

- depends-on: FEAT-001 — an arrival is logged against a booking
- depends-on: FEAT-002 — or against a season pass
- composes-with: FEAT-003 — a departure starts the departure invoice

## Architecture

- SPN-001 — Tidewatch web (`.mochiko/product/architecture/`)
- SPN-003 — Office kiosk, the arrivals board (`.mochiko/product/architecture/`)

## Story trace

- arrivals-board: US-001, US-002, US-003, US-004
