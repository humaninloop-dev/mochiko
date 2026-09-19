# Shift swaps

> Spec: shift-swaps
> Created: 2026-09-02
> Status: draft — stress-test pending

---

## Intent

- **Scope boundary:** a staff member swaps one of their own published shifts with a named
  colleague at the same venue; the colleague answers; the venue manager sees the result.
- **Delivery:** whole feature in one run.
- **Depth / rigor:** high — a swap changes who is on shift, which touches pay and licensing.
- **UX-bearing:** yes — a swap request screen and an answer screen in the staff app, and a
  manager view.
- **Constraints:** swap history is kept as a CSV file on the venue's shared drive for
  twelve months.
- **Out of scope:** offering a shift with no counterpart (that is open shifts); swaps between
  venues; a manager reassigning a shift (already covered by rota changes).

---

## Overview

Staff at our venues swap shifts constantly — a Saturday matinee for a Thursday evening, a bar
shift for an usher shift when someone's exam moves. Today this happens on a group chat and
the manager finds out when the wrong person clocks in, or does not. Every venue manager we
spoke to during the pilot interviews named "who is actually on tonight" as the thing they
check most and trust least, and two of the four Regal cinemas keep a paper swap book by the
box office. Shift swaps moves the request, the answer, and the manager's view into Kestrel so
the published rota is right by the time the doors open. Every staff member carries the
Kestrel app and has push notifications on, so a swap request reaches its counterpart within
seconds and most swaps settle the same day.

---

## User Stories

| ID | Story (one breath) | Priority | Feature | Disposition |
|----|--------------------|----------|---------|-------------|
| [US-1](stories/US-1.md) | Ask a colleague to take one of my shifts in exchange for one of theirs | P1 | FEAT-006 | homed |
| [US-2](stories/US-2.md) | Answer a swap request a colleague sent me | P1 | FEAT-006 | homed |
| [US-3](stories/US-3.md) | See and approve the swaps at my venue before they land | P2 | FEAT-006 | homed |
| [US-4](stories/US-4.md) | See the history of swaps on a shift | P3 | FEAT-006 | homed |
| [US-5](stories/US-5.md) | Swap with a colleague who works at another of the group's venues | P3 | — | rejected — cross-venue swaps are out of scope at intent |

---

## Edge Cases

- Counterpart declines → requester is notified and the request closes.
- Requester's shift is cancelled by the manager while a request is open → request closes.
- Both shifts in the swap are the same day → allowed.

---

## Functional Requirements

- **FR-001** A staff member MUST be able to request a swap of one of their published shifts
  for one published shift of a named colleague at the same venue. *Source: US-1*
- **FR-002** The colleague MUST be able to accept or decline the request; a request with no
  answer MUST expire after a reasonable time. *Source: US-2*
- **FR-003** The system MUST notify the venue manager promptly when a swap is accepted.
  *Source: US-3*
- **FR-004** An accepted swap MUST take effect on the published rota the moment both staff
  have accepted. *Source: US-1, US-2*
- **FR-005** The system MUST reject a swap that would leave either person with fewer than
  eleven hours' rest between two shifts, as UK law requires. *Source: US-1*
- **FR-006** A swap MUST NOT change the published rota until the venue manager has approved
  it. *Source: US-3*
- **FR-007** The system MUST record every change of a swap request's state with the actor and
  the time. *Source: governance — every change to a published rota is attributable*
- **FR-008** The swap request screen MUST offer a dark-mode toggle. *Source: —*
- **FR-009** Push notifications for swap events MUST be retried up to three times through the
  notification worker's queue before being marked failed. *Source: US-2*

---

## Key Entities

- **Swap request** — requester, counterpart, the two shifts, state (requested · accepted ·
  declined · approved · expired), timestamps per state.
- **Shift** — existing; a published shift carries role, start, end, venue, assignee.

---

## Success Criteria

- **SC-001** 80 % of swap requests receive an answer within 24 hours, measured from the swap
  log over the first eight weeks at a venue.
- **SC-002** Venue managers spend half as much time on swap admin within three months of
  launch, measured by asking them.
- **SC-003** Managers find the swap flow intuitive.
- **SC-004** Zero changes to a published shift without a matching audit record, measured by
  a nightly comparison of published-shift changes against the `rota_change` table.

---

## Screens & Flows

Screens: to be mocked by the designer once the spec is approved — Figma link to follow.

---

## Feature Selection

### Derived features

| FEAT-ID | Feature | New / delta | Stories | SCs verified |
|---------|---------|-------------|---------|--------------|
| FEAT-006 | Shift swaps | new (`proposed`) | US-1, US-2, US-3, US-4 | SC-001, SC-003 |
| FEAT-001 | Rota publishing | delta on delivered — a published shift accepts a swap-driven reassignment | — | SC-004 |

### Filter rejections

- US-5 — rejected: cross-venue swaps are out of scope at intent

### Selection

- **Selected (build now, dependency order):** FEAT-001 delta, FEAT-006
- **Deferred (`proposed` on the map):** none
- **Deferred SCs:** none

---

## Assumptions

- Any colleague at the venue is eligible to take any shift offered in a swap.
- A staff member sees colleagues' names and shifts when choosing whom to ask.

---

## Open Questions

- Should we cache the published rota on the device so swaps show up when the venue wifi is
  down?
- Whether a staff member may hold more than a reasonable number of open swap requests at once.

---

## Technical notes

Swap state lives in a `swap_requests` table keyed to the two `shift` rows; notifications go
through the existing notification worker (Celery, Redis broker); the swap list endpoint
should hold p95 under 300 ms at the venue's busiest hour; the manager view polls every 30 s.
