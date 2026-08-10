# FEAT-002 — Payment settlement & reconciliation

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle-v1/` (a child is in-flight)

## Children

- [FEAT-003](FEAT-003-stripe-payment-reconciliation.md) — in-flight
- [FEAT-004](FEAT-004-manual-payment-recording.md) — in-flight

## Capability

Settle an invoice to `paid` and keep payment state reconciled with the truth — online via Stripe
and manually for off-platform payments.

## Story trace

- invoice-lifecycle-v1: US-2, US-4 (via children)

<!-- Parent: navigation + status roll-up over its leaves; never built directly. Capability-first
     mint — settlement decomposes into online (FEAT-003) and manual (FEAT-004). Roll-up: in-flight
     while any child is; delivered when all children were delivered. -->
