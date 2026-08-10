# FEAT-006 — Payment tracking & reconciliation

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle/`

## Parent

- [FEAT-002](FEAT-002-invoice-lifecycle.md)

## Capability

Invoice payment state reflects reality — auto-reconciled from Stripe webhooks and settable manually for checks/cash — with an append-only audit trail, and a void path for invoices sent in error.

## Extent

- In: Stripe webhook reconciliation (signature-verified, idempotent, full-amount-only auto-mark, off-amount held/flagged), status lifecycle, manual mark-paid, pay-link deactivation on paid, void (terminal), append-only audit log, Decimal money.
- Not: partial payments, refunds.

## Relations

- depends-on: FEAT-005 — a sent invoice with a payment link.

## Architecture

- _pending — established at `/mochiko:plan`._

## Story trace

- invoice-lifecycle: US-5, US-6, US-9
