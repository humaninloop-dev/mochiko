# Feature — Shift swaps

- **Status:** approved for build 2026-09-08
- **Author:** Dan (product), reviewed by Ines (ops lead)

## Overview

Staff cover for each other constantly and today it happens in a group chat that the manager
cannot see. A shift swap lets a staff member offer one of their published shifts to eligible
colleagues; a colleague takes it; the manager approves; the rota updates and both people get a
text. Managers keep the final say because a swap can put an untrained person on a till or push
someone over their weekly hours.

## User stories

### US-001 · request a swap (P1)

As a staff member, I want to offer one of my upcoming shifts to colleagues, so that I can
get cover without chasing people myself.

- **Given** I am signed in and have a published shift more than 24 hours away, **when** I
  choose *Offer swap* on it and confirm, **then** the shift appears on the swap board as
  *offered by me*, and colleagues eligible for that shift's role see it.
- **Given** a shift starts in less than 24 hours, **when** I open it, **then** *Offer swap* is
  not available and the page says to talk to the manager.
- **Given** I already have an open swap request on a shift, **when** I open it again, **then**
  I can withdraw the request but not create a second one.

Independent test: offer next Tuesday's 07:00–15:00 barista shift from Amira's account; the board
shows it to Ben (barista) and not to Chloe (kitchen).

### US-002 · take a swap (P1)

As a colleague, I want to take an offered shift, so that I can pick up hours.

- **Given** an offered shift for my role, **when** I choose *Take this shift*, **then** the
  request moves to *awaiting approval*, the offer disappears from the board for everyone else,
  and the offering colleague and the manager are notified.
- **Given** taking the shift would put me over 48 hours that week, **when** I choose *Take
  this shift*, **then** the swap is refused with the reason shown.
- **Given** an offered shift overlaps one I already work, **when** I open it, **then** *Take
  this shift* is not available.

Independent test: from Ben's account take Amira's offered Tuesday shift; the board no longer
shows it to Dana (also a barista); Amira and the manager each receive a text.

### US-003 · approve a swap (P1)

As a manager, I want to approve or decline a swap, so that the rota stays safe.

- **Given** a swap awaiting approval, **when** I approve it, **then** the rota shows the taking
  colleague on that shift, the request is *completed*, and both staff receive a text with the
  outcome.
- **Given** a swap awaiting approval, **when** I decline it with a reason, **then** the offer
  returns to the board and both staff receive a text with the reason.
- **Given** a swap that has been awaiting approval for 12 hours, **when** the shift is now
  less than 24 hours away, **then** the request expires, the shift stays with the original
  person, and both staff receive a text.

Independent test: approve the Amira → Ben swap from the manager's account; next week's rota
shows Ben on Tuesday 07:00; both phones receive the approval text.

## Functional requirements

- **FR-001** The system MUST let a staff member offer any of their own published shifts whose
  start is more than 24 hours away. (US-001)
- **FR-002** The system MUST show an offered shift only to staff who hold the shift's role and
  do not already work an overlapping shift. (US-001, US-002)
- **FR-003** The system MUST allow at most one open swap request per shift. (US-001)
- **FR-004** The system MUST refuse a take that would put the taker over 48 rostered hours in
  that week, stating the reason. (US-002)
- **FR-005** The system MUST move a taken request to *awaiting approval* and hide the offer
  from the board. (US-002)
- **FR-006** The system MUST let a manager approve or decline a request awaiting approval;
  approval updates the rota, decline returns the offer to the board with the reason. (US-003)
- **FR-007** The system MUST expire a request awaiting approval once the shift is less than
  24 hours away. (US-003)
- **FR-008** The system MUST send an SMS to the offering staff member and the manager when a
  shift is taken, and to both staff members on approval, decline, or expiry. (US-002, US-003)
- **FR-009** The system MUST record every state change on a swap request with who made it and
  when. (US-003)

## Success criteria

- **SC-001** 90 % of swap requests reach *completed* or *declined* within 12 hours of being
  taken, measured over the first month from the request history.
- **SC-002** Staff find the swap flow intuitive.
- **SC-003** An offered shift is visible to eligible colleagues within 5 seconds of being
  offered, measured on the swap board page.

## Screens and flows

- **SCR-001 Swap board** — offered shifts for the signed-in person's role, newest first, each
  with *Take this shift*.
- **SCR-002 Shift page** — a single shift with *Offer swap* / *Withdraw* / *Take this shift*
  depending on state and viewer.
- **SCR-003 Approvals** (manager) — requests awaiting approval with *Approve* / *Decline*.
- **FLOW-001 Offer a swap** — SCR-002 → confirm → SCR-001 shows the offer.
- **FLOW-002 Take and approve** — SCR-001 → *Take this shift* → SCR-003 (manager) → *Approve*
  → rota updated.

## Out of scope

Three-way swaps; partial shifts; swaps between sites.
