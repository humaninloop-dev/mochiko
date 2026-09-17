# Alert channels

> Spec: alert-channels
> Created: 2026-08-09
> Status: closed — accepted 2026-08-14, capability-batch run delivered 2026-08-24

---

## Intent

- **Scope boundary:** the team configures where alerts go and Beacon sends them on open
  and close; nothing about retries or escalation.
- **Delivery:** whole feature specified; email and webhook ship together.
- **Depth / rigor:** production; outbound calls under the alerting rule.
- **UX-bearing:** yes — a channel settings page with a test button.
- **Constraints:** no third-party paging integrations by name; a generic webhook only.
- **Out of scope:** retries, quiet hours, escalation policies, SMS.
- **Capability frame:** one new capability — alert delivery; the incident timeline extends
  with two new event kinds.

---

## Overview

The team adds email addresses and webhook URLs as channels. When an incident opens or
closes, every channel gets one message and the timeline records the outcome.

---

## User Stories

| ID | Story (one breath) | Priority | Feature | Disposition |
|----|--------------------|----------|---------|-------------|
| [US-1](stories/US-1.md) | Configure a channel and test it | P1 | FEAT-003 | homed |
| [US-2](stories/US-2.md) | Get told when an incident opens or closes | P1 | FEAT-003 | homed |

---

## Edge Cases

- A webhook that hangs counts as failed after 10 seconds.
- A channel deleted mid-incident receives no close message.
- Two channels with the same URL are two attempts, not one.

---

## Functional Requirements

- FR-020: The system MUST send one message per channel on incident open and on close.
- FR-021: The system MUST record each attempt on the timeline as `alert-sent` or `alert-failed`.
- FR-022: The system MUST time out a webhook call at 10 seconds and record it as failed.
- FR-023: The system MUST let a user test a channel with a synthetic incident payload.

---

## Key Entities

- **AlertChannel** — kind (email · webhook), target, created-by; secret target never logged.
- **AlertAttempt** — channel, incident event, outcome, HTTP status or error class, at.

---

## Success Criteria

- SC-020: An incident open with three channels produces three attempts within 5 seconds.
- SC-021: A hung webhook shows `alert-failed` on the timeline after 10 seconds.
- SC-022: The test button on a webhook channel produces one attempt row and no incident.

---

## Screens & Flows

| ID | Screen | Purpose | Data shown | Feature |
|----|--------|---------|------------|---------|
| SCR-001 | Channels | List, add, remove, test channels | kind, masked target, last attempt outcome | FEAT-003 |
| SCR-002 | Incident page | Timeline with alert events inline | events newest-last, per-channel outcome | FEAT-002 |

| ID | Flow | Steps | Story scenario | Feature |
|----|------|-------|----------------|---------|
| FLOW-001 | Add and test a channel | SCR-001 -> add -> test -> SCR-001 | US-1 / test succeeds | FEAT-003 |
| FLOW-002 | See the alert on the incident | SCR-002 (after an open) | US-2 / three channels | FEAT-003 |

**Prototype:** `prototype/` — clickable low-fi rendering of this manifest; serve with bun
or open `prototype/index.html` directly. Flows and data are binding; layout and styling
advisory.

---

## Feature Selection

### Derived features

| FEAT-ID | Feature | New / delta | Stories | SCs verified |
|---------|---------|-------------|---------|--------------|
| FEAT-003 | Alert delivery | new (`proposed`) | US-1, US-2 | SC-020, SC-021, SC-022 |
| FEAT-002 | Incident timeline | delta on delivered — two event kinds | US-2 | SC-021 |

### Selection

- **Selected (build now, dependency order):** FEAT-002 (delta), FEAT-003
- **Deferred (`pending` row on FEAT-003):** per-channel quiet hours — ruled 2026-08-14 as a
  later increment.
- **Deferred SCs:** none

---

## Assumptions

- A failed webhook is a customer's problem to notice on the timeline; retries are a later
  feature if asked for.
