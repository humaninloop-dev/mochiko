# US-3 — Decline an offer, or let it lapse, and the next entrant is offered (P1)

As an owner, I want to decline an offer I no longer need so that someone else gets the berth,
and as the next entrant I want that berth offered to me straight away.

- **Given** an open offer, **when** the owner declines it, **then** the offer is closed and the next eligible entrant is issued an offer immediately, not after the original hold would have ended.
- **Given** an open offer, **when** 24 hours pass without a response, **then** the offer lapses and the next eligible entrant is issued an offer immediately.
- **Given** no further eligible entrant, **when** an offer is declined or lapses, **then** the berth returns to general availability.

**Independent test:** with two entrants, decline the first's offer; confirm the second's offer `issuedAt` is within 60 s of the decline.

Feature: FEAT-031
