# Architecture delta — Stripe subscriptions (FEAT-006)

**Signed:** 2026-08-22 (user sign-off on the rendered diagram and the row changes below).
**Contests:** AX-008 (billing & entitlements), SPN-005 (public edge), SPN-006 (new flow).

## Current shape

`api` serves the SPA and the JSON routes and never talks to a payment provider. `worker`
runs the two job kinds. There is no inbound webhook surface.

## Target shape

```mermaid
flowchart LR
    stripe[Stripe] -->|signed webhook| api
    api -->|INSERT stripe_events + jobs(stripe_event), one tx| db[(Postgres)]
    api -->|200 after commit| stripe
    worker -->|claim stripe_event| db
    worker -->|GET subscription| stripe
    worker -->|UPSERT subscriptions, audit_events| db
```

## Changes

| Element | Change | Detail |
|---------|--------|--------|
| SPN-001 api | modified | `POST /webhooks/stripe`: verify the `Stripe-Signature` header, insert the raw event into `stripe_events` and a `stripe_event` row into `jobs` in one transaction, return 200 only after commit. No call to Stripe's API in this handler. |
| SPN-002 worker | modified | new `stripe_event` handler: fetch the subscription from Stripe, upsert `subscriptions`, write `audit_events(plan_changed)`; idempotent on `stripe_events.id`. |
| SPN-003 db | modified | tables `stripe_events`, `subscriptions`; `audit_events` gains `plan_changed`. |
| SPN-005 public edge | modified | `/webhooks/stripe` joins the unauthenticated surface behind signature verification and `RateLimit`. |
| SPN-006 | new flow | Stripe webhook intake, as drawn. |
| AX-008 | in-flight | stance `decided` as ruled 2026-08-20; flips to `built` at the landing with the as-built line taken from the code. |

## Why this shape

Stripe retries a webhook for three days on any non-2xx, so the handler must be cheap and
must acknowledge only what is durably recorded (GI-009). Fetching the subscription inside
the request would hold Stripe's delivery open on our call to Stripe's own API; the
`jobs` table already gives at-least-once processing with retry (AX-004), so the fetch and
the upsert ride there. Entitlements are read locally so a Stripe outage never blocks a
dispatcher's morning.

## Not decided here

Self-serve plan changes (a candidate capability sent to the feature desk 2026-08-20) and
proration.
