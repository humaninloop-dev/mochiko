# FEAT-003 — Invoicing (draft & send)

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle/`

## Capability

A contractor drafts an invoice for a client — line items, a per-invoice tax rate, a due date, with subtotal/tax/total computed in exact decimal — and issues it to the client by authenticated email carrying a hosted payment link, moving it from draft to sent.

## Extent

- In: draft authoring (line items, per-invoice tax rate, due date), exact-decimal totals, send-by-email with a hosted payment link, authenticated outbound email with bounce handling, draft→sent transition, sent-invoice immutability (void + reissue, FR-018).
- Not: recurring invoices, estimates/quotes, multi-currency, a tax engine or per-line-item tax.

## Relations

- depends-on: FEAT-002 — an invoice is addressed to a client.
- composes-with: FEAT-005 — the pay link issued at send is what the client pays through.

## Architecture

- Components established at plan time (greenfield).

## Story trace

- invoice-lifecycle: US-3, US-4

## Obligations

- Seam: the hosted-payment link created at send is consumed by FEAT-005; verify the send↔pay seam when FEAT-005 builds.
