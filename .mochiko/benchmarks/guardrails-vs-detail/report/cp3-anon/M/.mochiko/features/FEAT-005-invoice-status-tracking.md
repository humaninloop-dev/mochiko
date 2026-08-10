# FEAT-005 — Invoice status tracking

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle-v1/`

## Parent

- [FEAT-002](FEAT-002-invoice-lifecycle.md)

## Capability

Show the contractor every invoice's current status at a glance — list and detail — so they can
trust the app's payment state without opening the payment processor.

## Extent

- Status list (draft / sent / viewed / paid); detail view with line items, totals, and
  payment/audit history; overdue computed from the due date; best-effort `viewed` from a
  hosted-page visit.
- Not: analytics or reporting; exports; email-open-based view tracking.

## Relations

- depends-on: FEAT-004 — reads the sent/viewed/paid state that delivery & payment produces.

## Architecture

- None yet — greenfield; realizing components filled at the first `/mochiko:plan`.

## Story trace

- invoice-lifecycle-v1: US-6, US-8

## Obligations

- (none)
