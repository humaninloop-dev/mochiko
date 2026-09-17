# Ledgerlite

Invoicing for small studios: draft an invoice, send it to the client, record what they
paid, and chase what they have not. Three people maintain it; about two hundred studios
use it, most on the free tier.

## Status

Four capabilities are delivered and in production (see `FEATURES.md`). Local development
runs the web app on :3000 against a local Postgres; `npm run dev` starts both.

## Shape

A server-rendered web app (Node, Fastify, Postgres) with transactional email through
Postmark. No mobile app. Clients receive invoices by email and open them on a public link;
clients do not have accounts and never will (see `DECISIONS.md`).
