# US-2 — Receive an offer and accept it within the hold (P1)

As an owner on a waitlist, I want to be offered a berth when one frees and to accept it with one
action so that the booking and the payment happen without me re-entering anything.

- **Given** an entrant at position 1 whose vessel fits, **when** a berth of the class frees for the range, **then** an offer is issued to that entrant within a minute and they are notified.
- **Given** an open offer, **when** the owner accepts within the hold, **then** the saved payment method is charged, the booking exists in `confirmed`, and the entry is fulfilled.
- **Given** an open offer, **when** the charge is declined, **then** the offer stays open for the remainder of the hold and the owner is told to try another method.

**Independent test:** free a berth; confirm an offer row exists within 60 s; accept; confirm a confirmed booking and a succeeded payment.

Feature: FEAT-031
