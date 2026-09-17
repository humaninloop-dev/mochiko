# FEAT-002 — Waitlist

**Status:** delivered (2026-06-14; extended 2026-08-12) · **Spec:** `waitlist` in the specs index

## Extent

A full course takes waitlist entries in order; a cancellation offers the freed seat to the
first entry by email; the seat is held for a 24-hour window (per-course override, 24 hours
default), then passes to the next entry. A bounced offer counts as declined. Expiry runs
from the scheduled-job runner.

## Work rows

- [x] W1 — Join and leave a waitlist — delivered 2026-06-14
- [x] W2 — Offer a freed seat to the first entry — delivered 2026-06-14
- [x] W3 — Offer window, bounce-as-decline, scheduled expiry — delivered 2026-08-12

## Dependencies

FEAT-001 (a seat to free).
