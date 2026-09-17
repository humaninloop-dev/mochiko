# FEAT-005 — Skipper notifications

> Status: delivered
> since 2026-06-12 · sticky

## Capability

The product tells a skipper when something needs them — a booking confirmed or declined, an
invoice issued or due, a season pass up for renewal.

## Extent

- Email or SMS, chosen per skipper; every message names the boat and the berth.
- The due reminder quotes the amount outstanding.
- Not: push notifications or an app.
- Not: office-side alerts.

## Relations

- composes-with: FEAT-001, FEAT-002, FEAT-003 — the events that produce a message

## Architecture

- SPN-001 — Tidewatch web (`.mochiko/product/architecture/`)

## Story trace

- notifications: US-001, US-002, US-003
