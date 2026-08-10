# FEAT-005 — Online payment collection

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle/`

## Parent

- [FEAT-004](FEAT-004-payments.md) — Payments (parent)

## Capability

A client pays a sent invoice through the processor-hosted checkout page, and the invoice flips to paid — verified, recorded exactly once, and reconciled against the processor as the source of truth. The contractor never handles a card number and never logs into the processor to see the result.

## Extent

- In: hosted-checkout payment, signature-verified and idempotent payment-notification handling, reconciliation to the processor as source of truth, invoice→paid transition, pay-link deactivation on payment (FR-019), contractor-visible paid state.
- Not: partial payments, refunds, payouts/reconciliation reporting beyond invoice state.

## Relations

- depends-on: FEAT-003 — the hosted pay link is issued at send.

## Architecture

- Components established at plan time (greenfield).

## Story trace

- invoice-lifecycle: US-5

## Obligations

- Seam: verify the send↔pay seam against FEAT-003 at build.
- EC-6 concurrent-payment coherence is shared with FEAT-006 (manual) — when FEAT-006 builds, verify online/manual cannot double-record.
