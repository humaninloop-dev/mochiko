# FEAT-008 — Invoice dashboard & status view

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle/`

## Parent

- [FEAT-002](FEAT-002-invoice-lifecycle.md)

## Capability

A contractor sees all invoices and their live payment status in one place, with outstanding/paid totals and status filtering — payment state visible without logging into Stripe.

## Extent

- In: invoice list (status, client, amount, due), outstanding & paid totals (void excluded from outstanding), filter by status, overdue-precedence display.
- Not: reporting / analytics, exports.

## Relations

- depends-on: FEAT-006 — renders reconciled payment status.

## Architecture

- _pending — established at `/mochiko:plan`._

## Story trace

- invoice-lifecycle: US-8

## Obligations

- Extend: renders FEAT-007's reminder state on the invoice detail view.
