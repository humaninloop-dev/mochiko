# FEAT-021 — Member check-in kiosks

**Status**: specified · **Owner**: product · **Design review**: Wednesday

## Overview

Today a member tells the front desk their name, staff look them up in the staff app, and
wave them through — or not. At peak (the 06:30 and 18:00 classes) the queue reaches the
door. This feature puts a tablet at the entrance: the member scans the QR code from their
welcome email or types their 6-digit PIN, the kiosk says welcome or see-staff, and the
door — a DoorLink-controlled entrance — unlocks for active members. Staff are told when
someone is turned away or the door does not open.

## User stories

- **US-001 (P1)** As a member, I check myself in by scanning my QR code or typing my PIN,
  and I see whether I'm in.
- **US-002 (P1)** As a member with an active membership, the door unlocks within two
  seconds of my check-in.
- **US-003 (P1)** As front-desk staff, I'm alerted in the staff app (and by email when I'm
  not looking at it) when a member is denied or the door fails, so I can help.
- **US-004 (P2)** As an owner, I pair a tablet with my venue once, and I can un-pair a lost
  one from the staff app.

## Functional requirements

- **FR-001** A check-in is recorded — member, venue, time, method (QR or PIN), outcome —
  before any door action is attempted.
- **FR-002** Entry is allowed only if the membership is active by the same rule the staff
  app uses today (`isActive` in `packages/db`); the kiosk and the staff app must never
  disagree about a member.
- **FR-003** The kiosk shows no member detail beyond first name and plan name.
- **FR-004** A paired kiosk can check members in and nothing else — no member lookup by
  name, no bookings, no staff functions.
- **FR-005** On an allowed check-in the door is unlocked through DoorLink; if DoorLink does
  not confirm within two seconds the kiosk tells the member to see staff and staff are
  alerted.
- **FR-006** Denied check-ins and door failures are listed for staff with the member and
  the reason.
- **FR-007** Pairing: an owner generates a pairing code in the staff app; the tablet enters
  it once and holds a long-lived device credential scoped to that venue; the owner can
  revoke it.

## Non-functional requirements

- **NFR-001** From a valid scan to the door-unlock command being acknowledged: two seconds
  at p95, DoorLink's response included.
- **NFR-002** During a DoorLink outage the kiosk keeps recording check-ins and deciding
  entry; only the door action fails (and staff are alerted).
- **NFR-003** A revoked device can do nothing within one minute of revocation.
- **NFR-004** Peak load: 40 check-ins in ten minutes at one venue.

## Constraints

- **C-001** The kiosk is a web app on an iPad in guided-access mode; no native build (two
  engineers and a front-end contractor).
- **C-002** Fly.io, one region, as today; Upstash Redis and Fly Postgres as today.
- **C-003** DoorLink's cloud API only (no local-network integration); its characteristics
  are in `docs/doorlink-api.md`.
- **C-004** The tablet's only connection is the venue Wi-Fi.

## Notes from the product lead (please treat as input, not decisions)

1. Let's have the kiosk keep a local copy of the member list and decide entry on the
   device, so it feels instant even on bad Wi-Fi.
2. I'd like a separate `checkin-service` so kiosk traffic never touches the main API — and
   while we're at it a `device-service` for when we sell our own hardware next year.
3. Don't bother with a current-state picture; the README's architecture section is fine.
