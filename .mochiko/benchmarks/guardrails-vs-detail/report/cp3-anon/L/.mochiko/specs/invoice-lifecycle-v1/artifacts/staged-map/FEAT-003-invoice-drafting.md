# FEAT-003 — Invoice drafting & void

> Status: proposed
> surfaced by invoice-lifecycle-v1 (2026-08-10)

## Parent

- [FEAT-001](FEAT-001-invoice-lifecycle.md)

## Capability

Compose an invoice for a client — line items (description, quantity, unit price), a tax rate, and
a due date — with money computed exactly and held as a revisable draft until sent; and void an
unpaid invoice to retire a mistaken or disputed bill.

## Extent

- In: create/edit draft invoices; line-item math and tax with `Decimal` money; due date; validation (no empty invoice, no past due date); void an unpaid invoice (stops reminders, excludes from owed/overdue, never reverses Stripe-collected funds).
- Not: recurring invoices, estimates/quotes, multi-currency (all firm out of scope for v1); editing a sent invoice in place (immutable — void + reissue).

## Relations

- depends-on: FEAT-002 — an invoice is addressed to a client.

## Story trace

- invoice-lifecycle-v1: US-2, US-8
