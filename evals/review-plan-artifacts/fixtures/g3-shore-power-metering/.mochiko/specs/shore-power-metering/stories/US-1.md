# US-1 — See my shore-power consumption within minutes (P1)

As an owner on a metered berth, I want to see what I have used so far this stay, close to real
time, so that I am not surprised by the invoice.

- **Given** a metered berth with a booking, **when** the gateway records a reading, **then** the owner's consumption view reflects it within 5 minutes.
- **Given** readings across a stay, **when** the owner opens the view, **then** they see kWh used per day and the running total.

**Independent test:** push a reading for the berth's meter; within 5 minutes the consumption endpoint shows the new total.

Feature: FEAT-027
