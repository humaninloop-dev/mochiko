# FEAT-003 — Payment recording

> Status: delivered  <!-- proposed | in-flight | delivered | retired -->
> since 2026-08-18 · sticky — live rows may still be visible below

## Capability

A studio records what a client paid against an invoice — full or partial, by any method —
and the invoice shows its remaining balance and flips to paid when the balance reaches
zero.

## Extent

- Manual entry by the studio: amount, date, method (bank transfer, cash, card, other),
  optional note.
- Partial payments accumulate; overpayment is rejected.
- Balance and status shown on the studio's invoice page and on the client's public link.
- Not: payments initiated by the client themselves.
- Not: refunds (a negative payment is rejected).

## Relations

- depends-on: FEAT-002 — a payment is recorded against a sent invoice.
- composes-with: FEAT-004 — a zero balance stops reminders.
