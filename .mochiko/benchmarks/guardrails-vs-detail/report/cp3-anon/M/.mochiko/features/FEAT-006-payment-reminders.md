# FEAT-006 — Payment reminders

> Status: proposed
> surfaced by invoice-lifecycle-v1 (2026-08-10) — deferred by user ruling; first in line after the spine

## Parent

- [FEAT-002](FEAT-002-invoice-lifecycle.md)

## Capability

Send reminder emails for unpaid, sent invoices on contractor-configured intervals, stopping the
moment an invoice is paid.

## Extent

- Reminder scheduling and send; stop-on-paid.
- Not: dunning escalation; SMS reminders.
- Reminder interval defaults, bounds, and per-invoice opt-out are undecided (deferred).

## Relations

- depends-on: FEAT-004 — reminders fire against sent, unpaid invoices.

## Architecture

- None yet — not built; realizing components decided when this feature is selected and planned.

## Story trace

- invoice-lifecycle-v1: US-7

## Obligations

- Deferred SC-006 (reminders sent only while unpaid, cease on payment) travels here until built.
- Reminder cadence defaults/bounds/opt-out unresolved — settle at the spec that selects this feature.
