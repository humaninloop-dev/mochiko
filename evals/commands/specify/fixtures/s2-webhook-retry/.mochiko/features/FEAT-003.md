# FEAT-003 — Alert delivery

> Status: delivered  <!-- proposed | in-flight | delivered | retired -->
> since 2026-08-24 · sticky — live rows may still be visible below

## Capability

When an incident opens or closes, Beacon tells the team on the channels they configured
— email addresses and webhook URLs — and records each send on the incident.

## Extent

- Channels: email (through the platform sender) and generic JSON webhook.
- One attempt per channel per incident event; the outcome lands on the timeline as
  `alert-sent` or `alert-failed`.
- A channel can be tested from its settings page with a synthetic incident.
- Not: retrying a failed attempt — a failed webhook is recorded and left.
- Not: quiet hours (pending row below).

## Work rows

- `pending` — per-channel quiet hours · acceptance: a channel with quiet hours set receives
  nothing inside the window and one summary at its end · cut by alert-channels

## Relations

- depends-on: FEAT-002 — every attempt is a timeline event.
- depends-on: FEAT-001 — the open and close transitions trigger delivery.
