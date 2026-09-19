# US-3 — Marina sets a tariff per season (P2)

As marina staff, I want to set the price per kWh for a period, so that winter and summer rates
are right without calling support.

- **Given** an existing tariff to 30 September, **when** staff add a tariff from 1 October, **then** it is saved and shown as upcoming.
- **Given** an existing tariff to 30 September, **when** staff add one starting 15 September, **then** they are refused because the periods overlap.

**Independent test:** add an overlapping tariff; the request is refused with the overlap named.

Feature: FEAT-027
