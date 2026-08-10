# FEAT-008 — Automated overdue reminders

> Status: proposed
> surfaced by invoice-lifecycle (2026-08-10)

## Capability

When a sent invoice passes its due date unpaid, Ledgerline automatically emails the client a
reminder carrying the same payment link — exactly once per reminder window — so the contractor
does not have to chase payment by hand.

## Extent

- In: scheduled detection of overdue-unpaid invoices; reminder email with payment link; no reminder for paid invoices; no double-reminder in the same window.
- Not: configurable multi-step dunning sequences; SMS reminders.

## Relations

- depends-on: FEAT-003 — reminders reuse the delivery + payment-link channel
- depends-on: FEAT-007 — overdue detection reads the status/overdue derivation

## Architecture

- (greenfield — component realized at plan/implement time)

## Story trace

- invoice-lifecycle: US-7

## Obligations

- carries deferred SC-006 (automatic reminders to overdue-unpaid clients) until this feature builds
