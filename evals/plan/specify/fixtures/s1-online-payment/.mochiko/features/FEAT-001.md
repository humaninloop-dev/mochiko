# FEAT-001 — Invoice drafting

> Status: delivered  <!-- proposed | in-flight | delivered | retired -->
> since 2026-07-30 · sticky — live rows may still be visible below
> reconstructed-from-code (2026-07-08) — extent re-verified at the invoice-lifecycle landing

## Capability

A studio drafts an invoice — client, line items, currency, due date — edits it, and
previews the rendered PDF before anything is sent.

## Extent

- Line items with quantity, unit price, and an optional percentage discount.
- One currency per invoice, fixed at draft time.
- PDF preview identical to what the client will receive.
- Not: recurring invoices (see `ROADMAP.md` Next).
- Not: multi-currency invoices.

## Relations

- composes-with: FEAT-002 — a draft becomes sendable once it has a client and one line item.
