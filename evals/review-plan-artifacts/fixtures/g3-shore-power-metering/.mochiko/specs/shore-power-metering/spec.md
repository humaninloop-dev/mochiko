# Feature Specification — Shore-power metering (FEAT-027)

- **Status:** accepted 2026-08-28 · **Author:** requirements analyst · **Owner:** Ines (product)

## Intent

- Scope boundary: metered shore-power readings from the marina's gateway; owner-visible consumption; per-kWh invoice lines; marina tariffs; outage handling.
- Delivery: whole feature in one batch; first shippable value is readings visible to owners.
- Depth / rigor: high (billing).
- UX-bearing: no — the portal and console teams build against the contract; prototype waived at intent.
- Constraints: one datastore (C-003); Stripe only for charges (C-001).
- Out of scope: prepaid power credit; water metering; gateway vendors other than Enerlink.

## Overview

Berths with shore power are metered by the marina's Enerlink gateway. Halyard takes the
readings, shows owners what they have used, and puts a per-kWh line on the monthly invoice at
the tariff the marina had in force when the power was used.

## User Stories

| ID | Story | Priority | Feature | Disposition |
|----|-------|----------|---------|-------------|
| [US-1](stories/US-1.md) | See my shore-power consumption within minutes | P1 | FEAT-027 | homed |
| [US-2](stories/US-2.md) | Be billed per kWh at the tariff in force when I used it | P1 | FEAT-027 | homed |
| [US-3](stories/US-3.md) | Marina sets a tariff per season | P2 | FEAT-027 | homed |
| [US-4](stories/US-4.md) | After a gateway outage, readings are backfilled and I see a gap notice | P2 | FEAT-027 | homed |

## Edge Cases

- A reading arrives out of order (earlier `readAt` than the last stored): stored, consumption recomputed for the interval.
- A meter is reassigned to another berth mid-month: readings before the change bill the old booking, after it the new one.
- A tariff change lands mid-interval between two readings: the interval is billed at the tariff in force at its start reading.
- Gateway sends a duplicate reading (same meter, same `readAt`): ignored, not double-counted.

## Functional Requirements

- **FR-001** The system MUST ingest meter readings from the marina's Enerlink gateway for every metered berth. *Source: US-1*
- **FR-002** An owner's consumption MUST be visible in the portal within 5 minutes of the reading being taken. *Source: US-1*
- **FR-003** Each monthly invoice MUST carry a shore-power line per booking equal to the kWh used times the tariff in force at the time of each reading. *Source: US-2*
- **FR-004** Marina staff MUST be able to set tariffs with validity periods; periods for one marina MUST NOT overlap. *Source: US-3*
- **FR-005** After a gateway outage, missed readings MUST be backfilled from the gateway's store and the owner MUST see a gap notice for the affected period. *Source: US-4*

## Key Entities

- **Meter** — the metering point on a berth, identified by the gateway's id.
- **Meter reading** — one cumulative kWh value at one instant for one meter.
- **Tariff** — a marina's price per kWh over a validity period.

## Success Criteria

- **SC-001** Readings are visible to the owner within 5 minutes of `readAt`, p95, measured from `readAt` to the consumption endpoint reflecting them.
- **SC-002** Monthly invoice lines reconcile to the gateway's own totals within 0.5 %, measured monthly per marina.
- **SC-003** Fewer than 1 % of shore-power invoice lines are disputed, measured monthly from the helpdesk tag.

## Screens & Flows

No UX surface — prototype waived at intent.
