# FEAT-004 — Invoice delivery & payment

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle-v1/`

## Parent

- [FEAT-002](FEAT-002-invoice-lifecycle.md)

## Capability

Send an invoice by email carrying a hosted payment link, collect payment through a
signature-verified idempotent webhook, and record manual (check/cash) payments — so an invoice's
payment state reflects reality exactly once.

## Extent

- Send / resend; hosted-link payment; exactly-once verified webhook handling; manual mark-paid;
  append-only payment audit trail; reconciliation of a manual + hosted double-pay; email-bounce
  surfacing.
- Not: partial payments (deferred, revisitable); dispute handling; recurring billing; raw card data.
- Kept as one leaf deliberately (see spec review G6): send and capture share one payment-state
  invariant; oversize at plan time is cut into vertical-slice cycles, not into two features.

## Relations

- depends-on: FEAT-003 — a payable invoice must be drafted first.

## Architecture

- None yet — greenfield; realizing components (Stripe adapter, webhook handler) filled at plan.

## Story trace

- invoice-lifecycle-v1: US-3, US-4, US-5

## Obligations

- (none)
