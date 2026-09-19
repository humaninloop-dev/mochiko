# Implementation Cycles: FEAT-031 — Berth waitlist

> Generated from the spec folder and the design inputs the cards were authored from: spec.md, features/FEAT-031/sufficiency-report.md, and the design phase's constraints-and-decisions.md, data-model.md, contracts/
> Structure: `mochiko:patterns-vertical-tdd` (cycle-card shape, slicing judgment)

## Overview

| Metric | Value |
|--------|-------|
| Cycles | 4 |
| Stories covered | US-1, US-2, US-3 |

## Cycle Format

Each card is one vertical increment: a coherent bundle of named test cases in the `**TEST:**`
grammar that demonstrate together. The builder implements the card test-first, decomposing at
build time; the cycle is done when its named cases show green against real infrastructure. The
card's checkbox is the progress surface. Growth on the standing booking path: no walking
skeleton is needed beyond Cycle 1's thin join path. `[P]` derives from dependencies.

---

### - [ ] Cycle 1: Join and see position

- **Stories:** US-1 — the thinnest new path: an owner joins a full berth class and sees their position
- **Depends on:** —
- **Case:** Simple
- **Brownfield exposure:** none

**TEST:** Joining a full berth class creates an entry at position 1
- **Covers**: US-1 / FR-001
- **Setup**: Seed a marina with two finger berths, both booked 2027-07-01 to 2027-07-14
- **Action**: `curl -s -X POST localhost:3000/api/v1/marinas/$MARINA/waitlist -d '{"vesselId":"'$VESSEL'","berthClass":"finger","fromDate":"2027-07-01","toDate":"2027-07-14"}'`
- **Assert**: Response status: 201
- **Assert**: Console contains "\"position\":1"
- **Capture**: console

**TEST:** Joining when a berth is available is refused
- **Covers**: US-1 / FR-001 scenario 3
- **Setup**: Seed a marina with one free finger berth for the range
- **Action**: `curl -s -X POST localhost:3000/api/v1/marinas/$MARINA/waitlist -d '{"vesselId":"'$VESSEL'","berthClass":"finger","fromDate":"2027-07-01","toDate":"2027-07-14"}'`
- **Assert**: Response status: 409
- **Capture**: console

---

### - [ ] Cycle 2: Offer on release, accept and pay

- **Stories:** US-2 — a berth release issues an offer to the first fitting entrant; accepting charges and books
- **Depends on:** C1
- **Case:** Merge — the offer and the accept demonstrate nothing separately; one bundle shows the loop
- **Brownfield exposure:** extends `src/bookings/`

Tasks:
- add `src/waitlist/offer.service.ts` with `issueOffer(berthId)` and the `offer_locks` polling loop
- add `src/waitlist/accept.controller.ts`
- migration `prisma/migrations/0031_offers/migration.sql`
- wire `mail.offer-issued` job in `src/mail/templates.ts`

**TEST:** Releasing a booked berth issues an offer within 60 s
- **Covers**: US-2 / FR-003 / SC-002
- **Setup**: Seed two entrants on the finger waitlist; the first's vessel fits the berth
- **Action**: `psql $DATABASE_URL -c "update bookings set status='completed' where id='$BOOKING'"` then `sleep 60`
- **Assert**: Console contains "offer issued" (within 60s)
- **Assert**: `psql $DATABASE_URL -c "select count(*) from offers where status='open' and berth_id='$BERTH'"` prints 1
- **Capture**: console, logs

**TEST:** Accepting an open offer charges the saved method and confirms the booking
- **Covers**: US-2 / FR-004
- **Setup**: An open offer for the entrant; Stripe test-mode saved card on the owner
- **Action**: `curl -s -X POST localhost:3000/api/v1/offers/$OFFER/accept`
- **Assert**: Response status: 200
- **Assert**: Console contains "bookingId"
- **Assert**: `psql $DATABASE_URL -c "select status from bookings where offer_id='$OFFER'"` prints confirmed
- **Capture**: console

---

### - [ ] Cycle 3: Decline hands the berth on `[P]`

- **Stories:** US-3 — a declined offer hands the berth to the next entrant
- **Depends on:** C2
- **Case:** Simple
- **Brownfield exposure:** none

**TEST:** Declining an open offer issues the next entrant's offer
- **Covers**: US-3 / FR-005
- **Setup**: Two entrants; the first holds an open offer issued at T
- **Action**: `curl -s -X POST localhost:3000/api/v1/offers/$OFFER/decline`
- **Assert**: Response status: 200
- **Assert**: The second entrant's offer `issuedAt` is T + 24 h, after the declined offer's hold elapses
- **Capture**: console

---

### - [ ] Cycle 4: Availability forecast digest `[P]`

- **Stories:** US-5 — marina ops see projected berth availability for 90 days
- **Depends on:** C1
- **Case:** Simple
- **Brownfield exposure:** none

**TEST:** Forecast unit suite passes
- **Covers**: FR-009
- **Action**: `npm test -- --grep forecast`
- **Assert**: Console contains "passing"
- **Capture**: console
