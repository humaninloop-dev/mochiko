### User Story 5 - Swap with a colleague at another of the group's venues (Priority: P3)

A staff member who works at two of a group's cinemas asks a colleague at the other cinema to
take a shift there.

**Why this priority**: Only the Regal group has staff who work across venues today.

**Independent Test**: With a staff member attached to two venues, request a swap across them
and confirm the request reaches the colleague.

**Acceptance Scenarios**:
1. **Given** a staff member attached to two venues, **When** they request a swap with a
   colleague at the other venue, **Then** the request is created
2. **Given** a cross-venue request is accepted, **When** the manager of the receiving venue
   approves it, **Then** both venues' rotas update
