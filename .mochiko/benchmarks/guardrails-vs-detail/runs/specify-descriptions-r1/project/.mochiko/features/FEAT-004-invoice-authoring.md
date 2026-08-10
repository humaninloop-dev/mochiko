# FEAT-004 — Invoice authoring

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle/`

## Parent

- [FEAT-002](FEAT-002-invoice-lifecycle.md)

## Capability

A contractor drafts an invoice for a client with line items, a single tax rate, and a due date; totals compute exactly and a gap-free number is assigned at send.

## Extent

- In: line items, single tax rate (rounded half-up), due date, exact Decimal totals, per-account gap-free numbering assigned at send, draft save + unsaved-state indication, draft deletion, new-client jump from editor.
- Not: multi-currency, recurring, partial payments, PDF export.

## Relations

- depends-on: FEAT-003 — an invoice is addressed to a client.

## Architecture

- _pending — established at `/mochiko:plan`._

## Story trace

- invoice-lifecycle: US-3
