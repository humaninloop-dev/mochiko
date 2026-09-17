# Fieldnote architecture

*Written by Dan, 2025-11-03. Last touched 2026-01-12 (added the Mapbox note).*

## The shape

Fieldnote is one Go binary behind Fly's proxy. A request comes in from the dispatcher's
browser (the React app is served as static files from the same process), hits the chi
router, and everything the request needs happens inside that request: we write the job to
Postgres, we work out the ETA, and we send the customer their SMS before the response goes
back. There are no background processes. If something needs to happen later, it happens on
the next request that touches the job. This keeps the deployment to a single Fly app and one
machine, which is all we can afford to babysit.

## Data

One Postgres database on Fly. Every table that belongs to a customer account carries
`account_id`, and every query goes through the `store` package so the scoping is never
forgotten. There is no separate reporting database; the dispatcher's job list is a plain
query with an index on `(account_id, scheduled_for)`.

## Messaging

SMS goes out through Twilio. When a dispatcher assigns a job, the handler in `dispatch`
calls the Twilio client directly with the customer's number and the ETA text. We retry once
inside the request if Twilio times out; if the second attempt fails the dispatcher sees an
error and re-sends by hand from the job page.

## ETA

The ETA text uses Mapbox's directions API from the technician's last known position to the
job address. Mapbox is called from the same request as the SMS, before it, so a Mapbox
failure means the SMS goes out without an ETA ("your technician is on the way").

## Auth

Dispatchers and technicians log in with email and password; a session cookie scoped to the
account. Technicians only see their own jobs. There is no SSO and nobody has asked for it.

## Public booking form

`/book/{account_slug}` is unauthenticated. It writes a job in state `requested` and emails
the dispatcher. It sits on the same router as everything else.

## What we have deliberately not built

- No queue, no worker, no cron. Everything is request-driven.
- No feature flags. We deploy to everyone.
- No metrics beyond Fly's dashboard and Sentry.
