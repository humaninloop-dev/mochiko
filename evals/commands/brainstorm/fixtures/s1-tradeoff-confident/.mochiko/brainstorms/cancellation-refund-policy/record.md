# Cancellation and refund policy — Decision Record

**Status:** accepted 2026-05-28
**Opened:** 2026-05-27

## Topic

Schools each had their own refund wording, several of them contradicting what Stripe
actually let them do. One platform policy with a school override, or leave it per school?

## Decisions

- **D1 — One platform default policy, overridable per school within limits.** `Confident`.
  Thirty of the forty schools used one of two near-identical wordings; the rest had no
  wording at all.

- **D2 — Full refund 14 days or more before the course starts.** `Confident`. Matches the
  most common existing wording and the time it takes to refill a seat from a waitlist.

- **D3 — Between 14 days and 48 hours: 50% credit against a future course, not a cash
  refund.** `Contested`. Recommendation was a 50% cash refund for simplicity; the user
  chose credit because schools keep the cash and students keep a reason to come back.

- **D4 — Inside 48 hours: no refund, unless the school cancels the course.** `Confident`.
  Instructors and boats are committed by then.

## Open questions

- Transfers: can a student hand their seat to someone else instead of cancelling?

## Session trail

- Q1 platform default vs per-school: default with override (D1).
- Q2 free-cancellation window: 14 days (D2).
- Q3 middle window: credit over cash after pushback (D3).
- Q4 late window: no refund (D4).
