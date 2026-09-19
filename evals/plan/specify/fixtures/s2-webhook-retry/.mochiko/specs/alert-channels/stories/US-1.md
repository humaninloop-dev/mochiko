# US-1 — Configure a channel and test it

**Priority:** P1 · **Feature:** FEAT-003 · **Disposition:** homed

As a team admin, I add a webhook URL as a channel and test it so that I know alerts will
reach our chat tool.

## Acceptance scenarios

- **Given** the channels page, **When** I add a webhook URL and press test, **Then** one
  attempt row appears with its HTTP status and no incident is created.
- **Given** a channel whose test failed, **When** I view the list, **Then** the last outcome
  reads failed with the error class, never the URL.

## Independent test

Add a channel pointing at the local target, test, assert one attempt row.
