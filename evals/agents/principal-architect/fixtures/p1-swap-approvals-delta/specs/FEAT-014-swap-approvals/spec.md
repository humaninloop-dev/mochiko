# FEAT-014 — Shift swap approvals

**Status**: specified · **Owner**: product · **Analyst draft for the architect**:
`design-draft.md`

## Overview

Staff can already see the rota. Today a swap is arranged by text message and a manager
edits the rota by hand, which is how double-bookings happen (INC-27: two people rostered
on the bar, nobody on the floor). This feature puts the swap in the product: the staff
member proposes, the colleague accepts, the manager approves or declines, the rota updates
itself, and everyone involved is told.

## User stories

- **US-001 (P1)** As staff, I propose swapping one of my shifts with a named colleague's
  shift so I can take the day off without leaving the venue short.
- **US-002 (P1)** As the colleague, I accept or decline the proposal from my phone.
- **US-003 (P1)** As the manager, I approve or decline an accepted proposal in one tap and
  the rota updates.
- **US-004 (P2)** As any party, I am told by SMS when the swap moves (accepted, approved,
  declined), with email as the fallback when I have no mobile number on file.
- **US-005 (P2)** As a manager, I can see the history of a swap — who proposed, accepted,
  decided, and when.

## Functional requirements

- **FR-001** A proposal names one shift of the proposer and one shift of the colleague at
  the same venue within the same rota week.
- **FR-002** A swap is eligible only if neither party would exceed their contracted hours
  for the week and both hold the role the shift requires. The rule exists today:
  `crewboard.domain.swaps.can_swap`.
- **FR-003** On approval the two shifts exchange assignees atomically; a swap whose shifts
  changed since the proposal is rejected with a reason.
- **FR-004** Each state change notifies the other parties (SMS, email fallback): the
  colleague on proposal, the proposer on the colleague's answer, both on the manager's
  decision, the manager on acceptance.
- **FR-005** A notification that cannot be delivered neither blocks nor reverses the state
  change; the rota is the source of truth and the app shows the current state.
- **FR-006** Every state change is recorded with actor and timestamp and is visible to the
  manager.

## Non-functional requirements

- **NFR-001** A notification for a state change is handed to the provider within 60 s of
  the change at p95, measured in the worker.
- **NFR-002** A manager's decision is persisted and visible in the app even when the SMS
  provider is unreachable.
- **NFR-003** The approve action responds to the manager within 500 ms at p95.

## Constraints

- **C-001** Render deployment as today: `api`, `worker`, managed PostgreSQL and Redis. No
  new services until Q1 2027 (budget freeze, finance 2026-07-01).
- **C-002** Twilio is the only SMS provider under contract; Postmark the only email
  provider.
- **C-003** Team of three; the feature ships in one release.

## Out of scope

- Push notifications, WhatsApp, or any channel other than SMS and email.
- Swaps across venues or across rota weeks.
