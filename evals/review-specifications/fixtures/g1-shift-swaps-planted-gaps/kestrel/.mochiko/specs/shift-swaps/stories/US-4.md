### User Story 4 - See the history of swaps on a shift (Priority: P3)

A manager opens a shift and sees every swap that touched it: who asked, who answered, who
approved, and when.

**Why this priority**: Needed when a pay query or an inspection asks who was on; rare day to day.

**Independent Test**: Swap a shift twice, open it from the manager dashboard, and confirm both
swaps appear with their actors and times.

**Acceptance Scenarios**:
1. **Given** a shift that has been swapped, **When** the manager opens its history, **Then**
   each swap is listed with requester, counterpart, approver, and timestamps
2. **Given** a shift with no swaps, **When** the manager opens its history, **Then** it says
   the shift has never been swapped

Feature: FEAT-006
