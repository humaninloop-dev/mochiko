# Data model — Tidewatch

Conceptual entities; the API shapes live in `contracts/api.yaml`.

## Entities

- **Berth** — a numbered mooring on a pontoon: length, draught, power channel. One harbour per
  deployment (AX-001).
- **Boat** — name, length, draught, the skipper's contact. Belongs to one skipper.
- **Skipper** — the person the product talks to: email, mobile, preferred channel.
- **Booking** — a boat on a berth for one night, confirmed or declined by the office. A longer
  visit is a run of consecutive bookings.
- **SeasonPass** — a boat's annual right to one named berth, with the pass rate for services.
- **Stay** — the period a boat is actually on a berth, opened by an arrival and closed by a
  departure (or by the month end for invoicing purposes).
- **MeterReading** — one pedestal channel reading (electricity kWh or water m³) at a timestamp,
  keyed by pedestal + channel + timestamp so a re-import is a no-op.
- **Invoice** — one per stay or per month-end cut of a stay; lines for berth nights, electricity,
  water; status draft / issued / paid.
- **InvoiceLine** — kind (berth-night / electricity / water), quantity, unit, unit price in minor
  units, amount in minor units, currency.

## Rules

- Each metered unit is billed exactly once, to the stay in which it was consumed.
- A stay that spans a month end is invoiced at the month end for what was consumed up to it and
  at departure for the remainder; the two invoices partition the readings, they never overlap.
- A season-pass boat pays the pass rate (10 % off berth nights and metered services).

## Sensitivity

- Skipper contact details: Confidential. Everything else: Internal.
