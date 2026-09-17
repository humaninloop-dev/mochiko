# Instructor availability model — Decision Record

**Status:** accepted 2026-07-02
**Opened:** 2026-07-02

## Topic

Instructors tell schools when they can teach by text message or a shared spreadsheet, and
schools re-key it into Saltmarsh by hand. Where should availability live, at what grain,
and what does an instructor see?

## Decisions

- **D1 — Availability is declared per week in half-day blocks (morning / afternoon).**
  `Confident`. Schools plan in half days; hour grains were asked for by nobody, and day
  grains lose the common "afternoon only" case.

- **D2 — A course day is assigned to an instructor on the session, not on each booking.**
  `Assumed`. The booking rows carry an `instructor_id` copy from the original schedule; the
  session is the thing that has an instructor. Nobody objected; the copy on bookings was
  not discussed further.

- **D3 — Instructors see their own week only; the school sees everyone.** `Confident`.
  Instructors are contractors at several schools; showing them other instructors' weeks
  leaks the school's staffing.

## Open questions

- What happens to the `instructor_id` column on bookings once assignment lives on the
  session — remove, or keep as a denormalised copy?

## Session trail

- Q1 grain: half-day chosen over hour and day (D1).
- Q2 where the assignment lives: session (D2).
- Q3 visibility: own week only (D3).
