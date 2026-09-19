# US-4 — After a gateway outage, readings are backfilled and I see a gap notice (P2)

As an owner, I want an outage at the marina's gateway not to lose my readings or to bill me
guesswork, and to be told when a gap was filled in later.

- **Given** the gateway was unreachable for six hours, **when** it comes back, **then** the missed readings are fetched from its store and the consumption view carries a notice naming the period that was backfilled.
- **Given** a backfilled period, **when** the invoice is generated, **then** the line uses the backfilled readings, not an estimate.

**Independent test:** block the gateway for an hour, unblock; the missed readings appear and the view shows a gap notice for that hour.

Feature: FEAT-027
