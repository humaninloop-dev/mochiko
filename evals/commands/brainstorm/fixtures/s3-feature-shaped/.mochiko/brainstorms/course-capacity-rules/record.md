# Course capacity rules — Decision Record

**Status:** accepted 2026-05-03
**Opened:** 2026-05-03

## Topic

Two schools overbooked a boat by editing course capacity after bookings existed. Where does
capacity live and who can change it?

## Decisions

- **D1 — Capacity is a property of the boat.** `Confident`. A boat has a fixed number of
  berths and a safety rating; a course borrows the boat's number.

- **D2 — A school may lower a course's capacity below its boat's, never raise it above.**
  `Confident`. Lowering covers a private-charter day or a nervous-beginner group.

- **D3 — Existing bookings are never displaced by a capacity change.** `Assumed`. Lowering
  capacity below the current booking count is rejected at save; nobody asked what a school
  should do instead.

## Open questions

- None recorded.

## Session trail

- Q1 where capacity lives: boat (D1).
- Q2 school override: lower only (D2).
- Q3 existing bookings: protected (D3).
