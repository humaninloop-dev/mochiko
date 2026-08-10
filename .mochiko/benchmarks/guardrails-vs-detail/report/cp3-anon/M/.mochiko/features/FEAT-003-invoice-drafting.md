# FEAT-003 — Invoice drafting

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle-v1/`

## Parent

- [FEAT-002](FEAT-002-invoice-lifecycle.md)

## Capability

Draft invoices with line items, a single invoice-level tax rate, and a due date, computing
subtotal, tax, and total exactly.

## Extent

- Create and edit drafts; edit unpaid invoices after send; lock on paid.
- Exact decimal money math; draft-validity rules (line items present, due date not past).
- Not: sending or payment (FEAT-004); void/reissue of paid invoices (deferred);
  multi-jurisdiction or per-line-item tax; multi-currency.

## Relations

- depends-on: FEAT-001 — an invoice needs a client.

## Architecture

- None yet — greenfield; realizing components filled at the first `/mochiko:plan`.

## Story trace

- invoice-lifecycle-v1: US-2

## Obligations

- (none)
