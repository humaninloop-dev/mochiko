# FEAT-006 — Manual payment recording

> Status: proposed
> surfaced by invoice-lifecycle (2026-08-10) — deferred (not selected this round)

## Parent

- [FEAT-004](FEAT-004-payments.md)

## Capability

A contractor records a payment received offline (check or cash) by marking the invoice paid with a method and date, so the record matches reality even when money did not move through the online link.

## Extent

- In: manual mark-as-paid with method + date, attribution in the invoice history, guard against double-recording an already-paid invoice.
- Not: partial payments, adjustments/credits.

## Relations

- depends-on: FEAT-003 — there must be a sent invoice to mark paid.

## Architecture

- Components established at plan time (greenfield).

## Story trace

- invoice-lifecycle: US-6

## Obligations

- deferred SC-006 — waits until this feature builds.
- EC-6: when built, verify a manual mark-paid cannot double-record against a concurrent online payment (seam with FEAT-005).
