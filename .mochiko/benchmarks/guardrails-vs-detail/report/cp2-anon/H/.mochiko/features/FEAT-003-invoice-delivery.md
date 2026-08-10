# FEAT-003 — Invoice delivery

> Status: in-flight
> since 2026-08-10 · owning spec: `.mochiko/specs/invoice-lifecycle/`

## Capability

Contractors send an invoice to a client by authenticated email carrying a link to view and pay
it. The contractor previews the email before it goes out; sending advances the invoice from draft
to sent and surfaces delivery failures (bounces) rather than silently claiming delivery.

## Extent

- In: preview-then-send by email; email carries an unguessable per-invoice payment-link token; draft → sent transition; sent invoices locked from edits; void cancels an invoice and invalidates its link; bounce/undeliverable reporting; resend without duplicating the invoice.
- Not: automated overdue reminders (FEAT-008); client-view ("viewed") tracking (see FEAT-007 / open question); editing a sent invoice (correction is void-and-reissue).

## Relations

- depends-on: FEAT-002 — only a saved invoice can be sent

## Architecture

- (greenfield — component realized at plan/implement time)

## Story trace

- invoice-lifecycle: US-3

## Obligations

- (none)
