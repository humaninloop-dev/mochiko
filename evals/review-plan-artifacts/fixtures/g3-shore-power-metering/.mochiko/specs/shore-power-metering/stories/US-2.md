# US-2 — Be billed per kWh at the tariff in force when I used it (P1)

As an owner, I want my monthly invoice's shore-power line to reflect the price that applied
when I actually used the power, so that a tariff change does not reprice what I already used.

- **Given** 10 kWh used while the tariff was 20p and 10 kWh after it changed to 30p, **when** the month's invoice is generated, **then** the shore-power line is £5.00 (10 × 20p + 10 × 30p), never 20 × 30p.
- **Given** a booking spanning a month end, **when** each month's invoice is generated, **then** each carries only that month's kWh.

**Independent test:** seed two tariffs and readings either side of the change; the invoice line equals the sum of each interval at its own tariff.

Feature: FEAT-027
