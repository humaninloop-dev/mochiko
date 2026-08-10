# FEAT-006 — Invoice status & dashboard

> Status: proposed
> surfaced by invoice-lifecycle-v1 (2026-08-10)

## Parent

- [FEAT-001](FEAT-001-invoice-lifecycle.md)

## Capability

Give the contractor one view of every invoice and its payment state — draft, sent, viewed, paid,
and an overdue indicator — so they know who owes them without ever opening Stripe.

## Extent

- In: dashboard list with per-invoice status, client, amount, due date; `viewed` marked from the payment-page open; overdue derived from due date (no stored state, no background job).
- Not: reporting/analytics, aging reports, exports (post-v1); real-time push (page-load refresh is fine).

## Relations

- depends-on: FEAT-003 — invoices must exist to be listed.
- depends-on: FEAT-005 — payment state is what the dashboard surfaces.

## Story trace

- invoice-lifecycle-v1: US-6
