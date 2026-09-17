# FEAT-003 — Mooring invoicing

> Status: delivered
> since 2026-05-15 · sticky

## Capability

One invoice per stay: berth nights plus the electricity and water read from the pontoon
pedestal, sent to the skipper and marked paid by the office against the bank reference.

## Extent

- A stay that spans a month end is invoiced at the month end for what was consumed and at departure for the rest.
- Electricity and water come from the nightly pedestal readings; a reading is billed once.
- The invoice shows one line per charge kind with quantity, unit, unit price, and amount.
- Not: a per-day breakdown of metered charges.
- Not: card payment; the office marks an invoice paid against the bank reference.

## Relations

- depends-on: FEAT-001 — the stay being billed came from a confirmed booking or a pass
- composes-with: FEAT-002 — the pass rate is applied on pass holders' invoices
- composes-with: FEAT-005 — the issued invoice and the due reminder go to the skipper

## Architecture

- SPN-001 — Tidewatch web (`.mochiko/product/architecture/`)
- SPN-004 — Nightly pedestal import (`.mochiko/product/architecture/`)

## Story trace

- mooring-invoicing: US-001, US-002, US-003, US-004, US-006

## Obligations

- deferred SC-004 (the invoice PDF matches the accountant's export line for line) — verify when INT-001 changes
