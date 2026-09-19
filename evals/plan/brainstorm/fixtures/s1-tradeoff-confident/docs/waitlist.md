# Waitlist offers — figures and what schools have asked for

Kept for the offer-expiry discussion. Figures are across all schools, 2026-04-01 to
2026-06-30, pulled from the `waitlist_offers` table and the Resend dashboard on 2026-06-30.

## Current behaviour

- A cancellation on a full course sends an offer email to the first waitlist entry.
- The seat is held for that person. Nothing expires the hold; staff release it by hand
  from the dashboard ("Release seat" on the offer row).
- If the person declines, the next entry is offered. If they never answer, nothing happens.

## Figures (90 days)

| Measure | Value |
|---|---|
| Offers sent | 412 |
| Accepted | 231 (56%) |
| Declined | 66 (16%) |
| Never answered | 115 (28%) |
| Median time to accept | 9 h 40 m |
| 90th percentile time to accept | 41 h |
| Median hold length before a staff release (unanswered offers) | 3.1 days |
| Seats still under an unanswered hold when the course started | 61 |
| Offer emails bounced or flagged spam (Resend) | 4.5% |

Of the 61 seats lost at course start, 44 were on courses whose waitlist had two or more
further entries — people who would probably have taken the seat.

## What schools have asked for

- 11 schools asked for a deadline on the offer ("give them a day, then move on").
- 3 schools asked for a race ("email everyone on the list, first to pay gets it").
- 2 schools asked for both: a short hold for the first entry, then open it up.
- Blackwater Sailing (the largest) wants to set the deadline per course: 24 hours for
  weekend courses, longer for five-day courses booked months out.

## Things that bear on either option

- All schools are in the UK or Ireland; offers going out overnight are answered in the
  morning, which is where the 9 h 40 m median comes from.
- Deposits are taken through Stripe Checkout at acceptance; an accept is a payment, not a
  click. A race would mean several people reaching Checkout for one seat.
- Instructors need final numbers 48 hours before a course starts (kit, boat allocation,
  the refund cut-off in the refund policy is also 48 hours).
- The dashboard has no notion of a scheduled job today; everything happens on a request.
