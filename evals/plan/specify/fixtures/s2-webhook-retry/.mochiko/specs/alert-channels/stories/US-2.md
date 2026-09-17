# US-2 — Get told when an incident opens or closes

**Priority:** P1 · **Feature:** FEAT-003 · **Disposition:** homed

As an on-call engineer, I get a message on every configured channel when an incident opens
and when it closes so that I do not have to watch the dashboard.

## Acceptance scenarios

- **Given** three channels, **When** an incident opens, **Then** three attempts are made
  within 5 seconds and each outcome lands on the timeline.
- **Given** a webhook that hangs, **When** an incident opens, **Then** that channel's
  attempt reads `alert-failed` after 10 seconds and the other channels are unaffected.

## Independent test

Local target with one hanging route and two healthy routes; open an incident; read the
timeline.
