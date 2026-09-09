# worker

Added 2026-05 when Stripe webhook processing started timing out in the request. Runs as the
`worker` process in `fly.toml` (see `[processes]`), consuming BullMQ queues on the Upstash
Redis the `REDIS_URL` secret points at.

Queues: `stripe-events` (membership status from invoice events), `email` (Postmark).
