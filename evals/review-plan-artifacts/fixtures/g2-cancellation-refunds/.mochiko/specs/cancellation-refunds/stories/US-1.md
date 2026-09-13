# US-1 — Cancel a booking and see the refund amount by notice tier (P1)

As an owner, I want to cancel a booking I cannot use and see straight away how much comes back
to me, so that I can decide before I confirm.

- **Given** a confirmed booking starting in 20 days, **when** the owner cancels, **then** the booking is `cancelled`, the berth is available again for those dates, and the owner sees a 100 % refund with the tier named.
- **Given** a confirmed booking starting in 3 days, **when** the owner cancels, **then** the owner sees a 0 % refund and the booking is still cancelled.
- **Given** a booking whose first night has passed, **when** the owner tries to cancel, **then** they are refused.

**Independent test:** cancel a booking 20 days out; the response shows tier full and the berth lists as available for the dates.

Feature: FEAT-034
