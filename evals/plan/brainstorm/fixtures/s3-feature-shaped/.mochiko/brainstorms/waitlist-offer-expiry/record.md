# Waitlist offer expiry — Decision Record

**Status:** accepted 2026-07-16
**Opened:** 2026-07-09

## Topic

A freed seat on a full course is offered to the first waitlist entry and held until they
answer; 28% never do and 61 seats were lost at course start last quarter under a live hold.
Fixed hold window, or first-to-claim race?

## Decisions

- **D1 — A fixed 24-hour hold for the first entry, then the next in line.** `Confident`.
  The 90th-percentile accept time is 41 hours but the median is under 10; 24 hours catches
  the morning-after answer without stranding the seat for days. A race would put several
  people into Stripe Checkout for one seat.

- **D2 — Schools may set a per-course window; 24 hours is the platform default.**
  `Contested`. Recommendation was no knob (the standing "fewer knobs" bet); the user chose
  the override because five-day courses booked months out genuinely need longer, and the
  bet's revisit condition covers it.

- **D3 — A bounced offer email counts as declined at once.** `Assumed`. Folded in from the
  bounce figures; the user did not object and it was not explored further.

- **D4 — Expiry runs from a scheduled job — the system's first.** `Unsure`. Nothing today
  runs off a clock; the alternative (expire lazily when the next request touches the offer)
  was left as the fallback if the job proves awkward on Fly.

## Open questions

- With two entries left and both declining inside the window, does the seat reopen to the
  public immediately or wait for the window to lapse?

## Session trail

- Q1 window vs race: window (D1).
- Q2 knob or default: default with override after pushback (D2).
- Q3 bounces: declined (D3).
- Q4 mechanism: scheduled job, provisionally (D4).
