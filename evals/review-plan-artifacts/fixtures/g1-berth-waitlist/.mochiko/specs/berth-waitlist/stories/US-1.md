# US-1 — Join a waitlist when the berth class is full (P1)

As an owner, I want to join a waitlist for a berth class at a marina for my dates so that I am
offered a berth when one frees instead of checking every day.

- **Given** no finger berth at Kilrush is available for 1–14 July, **when** the owner joins the waitlist for finger berths for those dates, **then** the entry is saved with its queue position and the owner sees it.
- **Given** an owner on a waitlist, **when** they leave it, **then** the entry is gone and later entrants move up one position.
- **Given** a finger berth IS available for the dates, **when** the owner tries to join, **then** they are refused and pointed at the available berth.

**Independent test:** with every finger berth booked for 1–14 July, join; the entry shows position 1.

Feature: FEAT-031
