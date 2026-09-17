# FEAT-001 — Course booking

**Status:** delivered (2026-04-30) · **Spec:** `course-booking` in the specs index

## Extent

A school lists courses (title, dates, boat, capacity); a student books a seat, pays a 30%
deposit through Stripe Checkout, and pays the balance 14 days before the start. Capacity
follows the boat (2026-05-03 ruling). Cancellation and refunds follow the platform policy
with the school's override (2026-05-28 ruling).

## Work rows

- [x] W1 — List a course and its days — delivered 2026-04-30
- [x] W2 — Book a seat with a deposit — delivered 2026-04-30
- [x] W3 — Cancel with the refund policy applied — delivered 2026-06-05

## Dependencies

None. First capability on the map.
