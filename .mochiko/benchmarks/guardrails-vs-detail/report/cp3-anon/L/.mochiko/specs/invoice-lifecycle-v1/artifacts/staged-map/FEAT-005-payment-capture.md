# FEAT-005 — Payment capture & recording

> Status: proposed
> surfaced by invoice-lifecycle-v1 (2026-08-10)

## Parent

- [FEAT-001](FEAT-001-invoice-lifecycle.md)

## Capability

Record when and how an invoice was paid — online through Stripe's hosted checkout, or marked paid
manually for cash/check — from the confirmed payment event, deduplicated, on an append-only audit
trail; card data never touches Ledgerline.

## Extent

- In: Stripe hosted checkout; payment confirmed from the trusted webhook event (not the redirect); manual mark-paid with reversal; idempotent against duplicate/replayed events; append-only payment audit trail.
- Not: partial payments (firm out of scope v1); refunds/chargebacks handling; raw card capture (Stripe-hosted only).

## Relations

- depends-on: FEAT-004 — payment happens against a sent invoice's link.

## Story trace

- invoice-lifecycle-v1: US-4, US-5
