# FEAT-005 — Online payment via Stripe

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle/`

## Parent

- [FEAT-004](FEAT-004-payments.md)

## Capability

A client pays an invoice through Stripe's hosted checkout reached from the emailed payment link;
the invoice is marked paid from Stripe's signature-verified, idempotently-processed confirmation
event — never from the browser redirect — so payment state reflects Stripe as the source of truth.

## Extent

- In: Stripe hosted checkout (no card data in Ledgerline); a "payment processing" state after redirect until the webhook confirms; webhook-driven auto-settlement; idempotent exactly-once recording; abandoned/failed checkout leaves the invoice unpaid.
- Not: partial payments (open question — see spec); refunds; payout/reconciliation reporting.

## Relations

- depends-on: FEAT-003 — payment starts from the sent invoice's emailed link
- depends-on: FEAT-002 — an invoice must exist and carry an amount

## Architecture

- (greenfield — component realized at plan/implement time)

## Story trace

- invoice-lifecycle: US-4

## Obligations

- (none)
