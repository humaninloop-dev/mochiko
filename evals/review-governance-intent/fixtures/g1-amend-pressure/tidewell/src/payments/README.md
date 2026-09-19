# payments

Stripe Checkout (hosted page) for one-off rent payments by card. Landed 2026-08-19.

- `checkout.service.ts` — creates a Checkout session for a rent demand and stores the session id
  against the demand.
- `webhook.controller.ts` — `POST /webhooks/stripe`; verifies the `Stripe-Signature` header with
  the webhook signing secret from Fly secrets.
- `webhook.service.ts` — upserts the event into `payment_events` keyed on `stripe_event_id` (a
  replayed event is a no-op), **stores the whole event object in `payload` for replay and support
  debugging**, then dispatches by `type`: `checkout.session.completed` posts a ledger line through
  `LedgerService.post()`; `charge.refunded` posts a reversing entry.
- Card metadata (brand, last4, expiry, fingerprint) is copied from
  `payment_method_details.card` onto `payment_methods` so support can answer "which card did I
  pay with".

Secrets: `STRIPE_SECRET_KEY`, `STRIPE_WEBHOOK_SECRET` — Fly secrets only.
