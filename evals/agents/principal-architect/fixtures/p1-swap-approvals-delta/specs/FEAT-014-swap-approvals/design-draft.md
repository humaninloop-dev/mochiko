# FEAT-014 — design draft (analyst, 2026-09-08)

For the architect: a first cut at the data model and the components so the design phase
can move quickly. The current-state picture is taken from `architecture/spine.md`.

## Data model sketch

- `swap_request` — id, business_id, venue_id, proposer_shift_id, colleague_shift_id,
  state (proposed | accepted | approved | declined | rejected), created_at
- `swap_event` — id, swap_request_id, actor_id, from_state, to_state, at (FR-006)
- `notification_outbox` — id, business_id, recipient_id, channel (sms | email | push |
  whatsapp), payload, attempts, next_attempt_at, dedupe_key, status (pending | sent |
  failed)
- `notification_channel_config` — business_id, channel, enabled, provider_settings (json),
  so each business can switch on push or WhatsApp when we add them (roadmap: 2027)

## Components

The clean way to do notifications is decoupled: writers drop a row into
`notification_outbox`, and a small **`notifier`** service polls the outbox every five
seconds and fans each row out to a channel adapter (Twilio, Postmark, later push and
WhatsApp). This is the standard modern pattern and it keeps `api` and `worker` out of the
messaging business.

`notifier` also owns the swap eligibility check (FR-002): the message text depends on why a
swap was rejected, so the notifier needs to compute the reason to word the message. `api`
keeps its own copy of the check for the approve endpoint, and `web` keeps the client-side
copy it has today so the button greys out early.

`notifier` handles retries itself — `attempts` and `next_attempt_at` on the outbox row,
exponential backoff up to five attempts — and `dedupe_key` so a re-polled row is not sent
twice.

For the manager's approve action, the `api` endpoint calls Twilio directly and returns 200
only once Twilio confirms delivery, so the manager knows for certain that the notification
went out.

## Component diagram (target)

```
 web ──▶ api ──▶ db ◀── worker ◀── redis
          │        ▲
          │        │ (polls outbox every 5 s)
          ├──────▶ notifier ──▶ Twilio / Postmark / (push) / (WhatsApp)
          │
          ├──▶ billing ──▶ Stripe
          ├──▶ payroll-export ──▶ PayFlow
          ├──▶ xero-sync ──▶ Xero
          └──▶ reports ──▶ db
```

No architecture change — this just adds tables and a helper service; the existing boxes
stay where they are.
