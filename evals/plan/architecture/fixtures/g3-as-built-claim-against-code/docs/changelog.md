# Changelog — Fieldnote

## 2026-09-10 — Stripe subscriptions (FEAT-006)

Accounts can subscribe by card. Stripe Checkout, webhook intake at `/webhooks/stripe`,
entitlements gating the dispatcher UI, plan changes audit-logged. Final validation clean.
Store fold deferred to the next desk visit.

## 2026-08-22 — Rate limiting on the public edge

`RateLimit` middleware on `/book/{slug}` (per IP and per slug). Closes the July scraper
incident.

## 2026-08-05 — Partner webhooks (FEAT-005)

Signed `job.completed` events to accounting partners, retried for 24 h.

## 2026-07-10 — Technician day view (FEAT-004)
