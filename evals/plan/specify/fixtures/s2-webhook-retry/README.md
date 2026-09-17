# Beacon

Uptime monitoring for small teams: Beacon probes your endpoints every minute, opens an
incident when a probe fails, and tells you on the channels you chose — email, or a webhook
to whatever chat or paging tool you run. Two maintainers; roughly ninety teams on it.

## Status

Three capabilities delivered (see `FEATURES.md`). The first two were reconstructed from
the code when governance was set up in August; the third went through a full
specification. Local development: `npm run dev` starts the API, the checker, and a local
Postgres via docker compose.

## Shape

A Node API with server-rendered settings pages, a checker process that runs the probes,
and a notifier that sends alerts. Postgres for everything durable. No mobile app.
