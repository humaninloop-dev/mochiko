# FEAT-007 — Payment reminders

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle-v1/`

## Parent

- [FEAT-001](FEAT-001-invoice-lifecycle.md)

## Capability

Automatically email a client reminders on unpaid invoices on a contractor-controlled schedule, so
the contractor doesn't chase late payers by hand; reminders stop the moment an invoice is paid.

## Extent

- In: on/off toggle; one global reminder schedule (default: on due date, then every 7 days until paid, max 3); reminders stop on paid (online or manual) and on void; each send logged.
- Not: per-invoice schedule override (deferred, post-v1); SMS reminders; reminder A/B or templating.

## Relations

- depends-on: FEAT-004 — reminders re-send the payment link.
- depends-on: FEAT-005 — a paid invoice must halt its reminders.
- depends-on: FEAT-003 — voiding an invoice halts its reminders (disputed-invoice handling, US-8).

## Story trace

- invoice-lifecycle-v1: US-7
