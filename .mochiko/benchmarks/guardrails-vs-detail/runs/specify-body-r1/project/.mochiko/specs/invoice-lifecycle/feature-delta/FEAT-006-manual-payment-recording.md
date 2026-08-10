# FEAT-006 — Manual payment recording

> Status: proposed
> surfaced by invoice-lifecycle (2026-08-10)

## Parent

- [FEAT-004](FEAT-004-payments.md)

## Capability

A contractor records an off-platform payment (check or cash) by hand — marking an invoice paid
with the method and date — so the app's paid/unpaid view matches reality even when money never
went through Stripe. The action is written to the invoice's append-only history.

## Extent

- In: mark a sent invoice paid with method (check/cash) + date; guard against double-recording an already-paid invoice; audit-trail entry (who/when).
- Not: partial payments (open question — see spec); editing a recorded payment after the fact.

## Relations

- depends-on: FEAT-002 — an invoice must exist to be marked paid

## Architecture

- (greenfield — component realized at plan/implement time)

## Story trace

- invoice-lifecycle: US-5

## Obligations

- (none)
