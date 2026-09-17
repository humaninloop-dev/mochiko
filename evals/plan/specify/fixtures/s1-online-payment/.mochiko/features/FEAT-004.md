# FEAT-004 — Payment reminders

> Status: delivered  <!-- proposed | in-flight | delivered | retired -->
> since 2026-08-18 · sticky — live rows may still be visible below

## Capability

Ledgerlite chases unpaid invoices by email on a schedule the studio sets once, and stops
the moment the invoice is paid.

## Extent

- One global schedule per studio: days-after-due offsets, up to three reminders.
- Reminder email reuses the invoice's public link.
- A paid or voided invoice drops out of the schedule immediately.
- Not: a schedule per client (pending row below).
- Not: SMS or any channel other than email.

## Work rows

- `pending` — per-client reminder cadence · acceptance: a studio sets a longer offset for a
  named client and the next reminder honours it · cut by payment-recording

## Relations

- depends-on: FEAT-003 — the balance decides whether a reminder still goes out.
