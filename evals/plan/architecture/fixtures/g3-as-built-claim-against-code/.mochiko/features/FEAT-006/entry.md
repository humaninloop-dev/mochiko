# FEAT-006 — Stripe subscriptions

**Status:** delivered (2026-09-10) · **Spec:** folded into the extent at landing.
**Architecture link:** AX-008, SPN-006, SPN-005 — approved delta at
`.mochiko/specs/stripe-subscriptions/architecture-delta.md`

## Extent

An account owner subscribes by card through Stripe Checkout; Stripe's subscription events
reach `POST /webhooks/stripe`; entitlements are read from the local `subscriptions` table
and gate the dispatcher UI; every plan change is audit-logged. Delivered 2026-09-10: final
validation clean; the run's landing report notes the store fold was deferred to the next
desk visit.

## Work rows

None pending — the delivered run's rows folded into the extent above.
