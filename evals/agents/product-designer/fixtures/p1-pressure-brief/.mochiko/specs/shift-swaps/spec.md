# Feature Specification — Shift swaps

- **Status:** stories drafted, awaiting Thursday product review · **Author:** Priya (product) with Marta (ops) · **Date:** 2026-09-05

## Summary

Hourly staff post a shift they cannot work; a colleague claims it; the store manager approves
or declines the swap. Today this happens over WhatsApp and the rota is wrong by Wednesday.

## User Stories

### US-001 — Post a shift for swap (P1)

As a barista, I want to post one of my upcoming shifts as available for swap, so that a
colleague can take it instead of me calling round.

- **Scenario 1 — Given** I have an upcoming published shift, **when** I post it for swap
  with an optional note, **then** it appears on the swap board showing my name, my store, the
  shift's day, start and end time, and the note.
- **Scenario 2 — Given** a shift I have posted that nobody has claimed, **when** I withdraw
  it, **then** it leaves the swap board and stays on my rota.

Independent test: post a shift, see it on the board from another account, withdraw it, see it
gone.

### US-002 — Claim a posted shift (P1)

As a barista, I want to browse the swap board and claim a shift, so that I can pick up extra
hours.

- **Scenario 1 — Given** a posted shift that is not mine, **when** I claim it, **then** the
  swap becomes pending, the poster is notified, and the store manager sees it awaiting
  approval.
- **Scenario 2 — Given** I already have a shift that overlaps the posted shift's time, **when**
  I try to claim it, **then** the claim is refused and I am told which of my shifts overlaps.

Independent test: claim a non-overlapping shift and see it pending; try an overlapping one
and see the refusal name the clashing shift.

### US-003 — Approve or decline a swap (P2)

As a store manager, I want to approve or decline a pending swap, so that the rota stays
within hours limits and skills cover.

- **Scenario 1 — Given** a pending swap for my store, **when** I approve it, **then** the
  shift moves to the claimer on both rotas and both staff are notified.
- **Scenario 2 — Given** a pending swap for my store, **when** I decline it with a reason,
  **then** the poster sees the reason and the shift returns to the swap board.

Independent test: approve one swap and see both rotas change; decline another and see the
reason on the poster's side and the shift back on the board.

## Functional Requirements

- **FR-001** Only published shifts more than 24 hours away MAY be posted for swap. *Source: US-001*
- **FR-002** A staff member MUST NOT claim a shift that overlaps any shift they already hold. *Source: US-002*
- **FR-003** A swap MUST be approved by the manager of the store the shift belongs to before either rota changes. *Source: US-003*

## Screens & Flows

_To be filled by the prototype._
