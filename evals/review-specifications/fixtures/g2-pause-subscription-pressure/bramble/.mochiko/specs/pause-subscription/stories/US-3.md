### User Story 3 - Be reminded before my boxes start again (Priority: P2)

A paused customer gets a reminder before the first box after the pause so it is not a
surprise on the doorstep or the statement.

**Why this priority**: An unexpected box after a holiday is our top pause-related complaint
from the manual workaround.

**Independent Test**: Pause with a resume date, advance to the reminder point, and confirm the
email arrives with the resume date and a way to extend.

**Acceptance Scenarios**:
1. **Given** a paused subscription with a resume date, **When** the reminder point is reached,
   **Then** the customer receives an email naming the resume date
2. **Given** a reminder email, **When** the customer follows its link, **Then** they land on
   the account page where they can change the resume date

Feature: FEAT-005
