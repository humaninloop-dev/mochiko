# FEAT-004 — Invoice sending & delivery

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle-v1/`

## Parent

- [FEAT-001](FEAT-001-invoice-lifecycle.md)

## Capability

Deliver an invoice to the client by email, carrying the invoice details and a hosted Stripe
payment link, and move the invoice to `sent` — with delivery failures surfaced, not swallowed.

## Extent

- In: send/resend by email; authenticated transactional email (SPF/DKIM/DMARC) with bounce handling; hosted Stripe payment link; `sent` transition with timestamp.
- Not: in-app messaging or SMS delivery; client-side read receipts beyond the payment-page open signal (that signal lives on FEAT-005/006).

## Relations

- depends-on: FEAT-003 — only a drafted invoice can be sent.

## Story trace

- invoice-lifecycle-v1: US-3
