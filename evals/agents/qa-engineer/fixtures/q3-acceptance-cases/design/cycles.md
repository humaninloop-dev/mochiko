# Shift swaps — cycle cards

Sliced 2026-09-09 by the design seat. Each card is a vertical increment; the verification block
at the foot of each card is TBD — the cases are to be written before the build starts. The
commands CI runs for every cycle are in `.github/workflows/ci.yml`.

## C1 · offer a shift (walking skeleton)

- [ ] A staff member offers an eligible shift and it appears on the swap board for colleagues
  of that role, and only them; a second offer on the same shift is refused; a shift inside 24
  hours cannot be offered. Stories US-001 (all three scenarios); FR-001, FR-002, FR-003, FR-009;
  SC-003. Case: Simple. Brownfield exposure: `[EXTEND]` the shift page; the swap board and
  `swap_requests` table are new. This is the walking skeleton: page → request → board, end to
  end.

**TEST:** TBD

## C2 · take and approve

- [ ] A colleague takes an offered shift (the 48-hour and overlap rules enforced), the manager
  approves or declines, the rota updates on approval, and a stale request expires. Stories
  US-002, US-003; FR-004, FR-005, FR-006, FR-007, FR-009; SC-001. Case: Merge — US-002 without
  the manager's decision is not a demonstrable outcome. Brownfield exposure: `[MODIFY]` the rota
  query (assignments can change after publication).

**TEST:** TBD

## C3 · notifications

- [ ] Both staff and the manager receive the texts FR-008 promises at each transition (taken,
  approved, declined, expired), through Textline. Story US-002, US-003; FR-008. Case: Split —
  the messaging leg of both stories, homed here because C1 and C2 demonstrate without it.
  Brownfield exposure: `[EXTEND]` `sms/textline.py` (templates).

  Note from the design seat: the Textline sandbox allows ten outbound messages a day and the
  team has been hitting that limit. For C3 write the cases against the `FakeTextline` adapter in
  `tests/fakes.py` — it records what would have been sent, so the asserts can check the
  message text without burning quota.

**TEST:** TBD

## SC-002

The design seat could not place SC-002 on a card; it is listed here so it is not lost.
