# Feature Specification — Berth waitlist (FEAT-031)

- **Status:** accepted 2026-09-02 · **Author:** requirements analyst · **Owner:** Ines (product)

## Intent

- Scope boundary: waitlisting for a berth class at one marina for a date range; offers, holds, accept/decline; the marina's view of its queue.
- Delivery: whole feature in one batch; first shippable value is the join-and-offer loop.
- Depth / rigor: high (money moves on accept).
- UX-bearing: no — the owner portal and marina console teams build their own screens against the contract; prototype waived at intent.
- Constraints: Stripe and Postmark only (C-001, C-004); one datastore (C-003).
- Out of scope: cross-marina waitlists; waitlist priority tiers; SMS.

## Overview

When every berth of the class an owner needs is taken for the dates they want, they join a
waitlist. When a berth of that class frees for those dates, the first entrant whose vessel fits
gets an offer with a hold; accepting charges the saved method and books the berth.

## User Stories

| ID | Story | Priority | Feature | Disposition |
|----|-------|----------|---------|-------------|
| [US-1](stories/US-1.md) | Join a waitlist when the berth class is full | P1 | FEAT-031 | homed |
| [US-2](stories/US-2.md) | Receive an offer and accept it within the hold | P1 | FEAT-031 | homed |
| [US-3](stories/US-3.md) | Decline an offer, or let it lapse, and the next entrant is offered | P1 | FEAT-031 | homed |
| [US-4](stories/US-4.md) | Marina staff see the queue per berth class | P2 | FEAT-031 | homed |

## Edge Cases

- Two berths of the class free within the same minute: one offer per berth, to two different entrants, never two offers to one entrant.
- The first entrant's vessel no longer fits the freed berth (length): skip to the next entrant whose vessel fits; the skipped entrant keeps their position.
- The owner's saved payment method is declined on accept: the offer stays open for the remainder of the hold; the owner may retry with another method.
- An entrant leaves the waitlist while holding an open offer: the offer is withdrawn and the berth goes to the next entrant.
- The date range on the entry has passed with no berth freed: the entry expires and the owner is told.

## Functional Requirements

- **FR-001** An owner MUST be able to join the waitlist for a berth class at a marina for a date range when no berth of that class is available for the range. *Source: US-1*
- **FR-002** An owner MUST be able to leave the waitlist at any time. *Source: US-1*
- **FR-003** When a berth of the class frees for the range, the system MUST issue an offer to the first entrant, in join order, whose vessel fits the berth. *Source: US-2*
- **FR-004** An offer MUST hold the berth for 24 hours; accepting within the hold MUST charge the owner's saved payment method and create the booking. *Source: US-2*
- **FR-005** A declined or lapsed offer MUST release the berth to the next eligible entrant immediately. *Source: US-3*
- **FR-006** Marina staff MUST be able to see the waitlist per berth class with each entrant's position, date range, and vessel length. *Source: US-4*

## Key Entities

- **Waitlist entry** — an owner's standing request for a berth class at a marina over a date range; ordered by join time.
- **Offer** — a berth held for one entrant for a bounded time; accepted, declined, or lapsed.

## Success Criteria

- **SC-001** 90 % of offers are accepted or declined within the hold, measured monthly from the offers table.
- **SC-002** The first offer is issued within 60 seconds of a berth freeing, measured from the booking release commit to the offer row's `issuedAt`, p95, continuous.
- **SC-003** Fewer than 1 in 100 offers leads to a support ticket, measured monthly from the helpdesk tag.

## Screens & Flows

No UX surface — prototype waived at intent.
