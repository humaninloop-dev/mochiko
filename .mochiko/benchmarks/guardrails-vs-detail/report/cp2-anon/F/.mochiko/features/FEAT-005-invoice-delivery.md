# FEAT-005 — Invoice delivery

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle/`

## Parent

- [FEAT-002](FEAT-002-invoice-lifecycle.md)

## Capability

A contractor sends an invoice to the client by authenticated email that includes a summary and a Stripe hosted payment link; the invoice becomes sent (and viewed if a view is detectable).

## Extent

- In: authenticated email (SPF/DKIM/DMARC), invoice summary + hosted payment link (no PDF), sent status, bounce surfacing.
- Not: SMS delivery, in-app client inbox, PDF attachment.

## Relations

- depends-on: FEAT-004 — a drafted invoice to send.

## Architecture

- _pending — established at `/mochiko:plan`._

## Story trace

- invoice-lifecycle: US-4

## Obligations

- `viewed` status contingent on reliable view detection (open question, FR-010) — drop rather than show unreliably.
