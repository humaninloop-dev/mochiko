# FEAT-004 — Manual payment recording

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle-v1/`

## Parent

- [FEAT-002](FEAT-002-payment-settlement-reconciliation.md)

## Capability

Record an off-platform (cash/check) settlement so an invoice reconciles to `paid` outside Stripe,
distinguishably from a Stripe payment.

## Extent

- In: mark-as-paid with method + date; `paid` transition; manual-vs-Stripe provenance; audit-trail write; refusal of a double settlement.
- Not: Stripe collection (FEAT-003).

## Relations

- depends-on: FEAT-001 — an invoice must exist to be settled.

## Architecture

- _Pending first plan — no components built yet (greenfield)._

## Story trace

- invoice-lifecycle-v1: US-4

## Obligations

- Audit-trail write on payment-state change (GI-029).
- Tenant isolation on payment-state writes (GI-011).
