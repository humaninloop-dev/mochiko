### User Story 3 - See and approve the swaps at my venue before they land (Priority: P2)

A venue manager sees accepted swaps for the coming week and approves or refuses each one
before the rota changes.

**Why this priority**: Managers asked for a say on licensing-sensitive shifts; most swaps will
be waved through.

**Independent Test**: Have two staff accept a swap, then approve it from the manager dashboard
and confirm the rota changes only after approval.

**Acceptance Scenarios**:
1. **Given** an accepted swap at the manager's venue, **When** the manager opens the swaps
   list, **Then** the swap shows both staff, both shifts, and the roles involved
2. **Given** an accepted swap, **When** the manager refuses it with a note, **Then** both
   staff are told with the note and the rota is unchanged

Feature: FEAT-006
