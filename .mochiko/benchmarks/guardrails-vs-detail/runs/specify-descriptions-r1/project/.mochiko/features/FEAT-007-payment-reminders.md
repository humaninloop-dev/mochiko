# FEAT-007 — Payment reminders

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle/`

## Parent

- [FEAT-002](FEAT-002-invoice-lifecycle.md)

## Capability

Unpaid invoices past due are auto-flagged overdue and the client is reminded on a fixed cadence until the invoice is paid or voided.

## Extent

- In: auto overdue detection, reminders at due / +3 / +7 (cap 3, stop on paid/void/disabled/bounced), per-invoice off-switch, reminder state visible on invoice detail.
- Not: custom cadences, per-account reminder templates.

## Relations

- depends-on: FEAT-006 — needs status lifecycle and the email path.

## Architecture

- _pending — established at `/mochiko:plan`._

## Story trace

- invoice-lifecycle: US-7

## Obligations

- Extend: surface each invoice's reminder state (next reminder / off) on the invoice detail view (FEAT-008 renders it).
