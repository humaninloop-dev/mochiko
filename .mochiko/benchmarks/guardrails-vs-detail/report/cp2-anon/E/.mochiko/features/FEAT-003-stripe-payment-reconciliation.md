# FEAT-003 — Stripe-hosted payment & reconciliation

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle-v1/`

## Parent

- [FEAT-002](FEAT-002-payment-settlement-reconciliation.md)

## Capability

Accept a client's payment through Stripe-hosted checkout with no client account, and reconcile the
invoice to `paid` against Stripe as the source of truth, exactly once.

## Extent

- In: pay-without-account happy path; `viewed` on link open; signature-verified webhook; idempotent (exactly-once) reconciliation to `paid`; audit-trail write.
- Not: manual settlement (FEAT-004); any raw card-data handling (Stripe-hosted only).

## Relations

- depends-on: FEAT-001 — an invoice must exist to be paid.

## Architecture

- _Pending first plan — no components built yet (greenfield)._

## Story trace

- invoice-lifecycle-v1: US-2

## Obligations

- Audit-trail write on payment-state change (GI-029).
- Webhook signature verification + idempotency, exactly-once (GI-026).
- Never handle raw card data — Stripe-hosted checkout only (GI-014).
- Tenant isolation on payment-state writes (GI-011).
