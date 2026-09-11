### User Story 1 - Pause my boxes until a date I choose (Priority: P1)

A customer opens their account page, chooses "Pause", picks the date boxes should start again,
and confirms; no boxes arrive and nothing is charged until then.

**Why this priority**: The whole feature — every other story assumes a pause exists.

**Independent Test**: Pause a test subscription for three weeks and confirm no charge and no
packhouse manifest line for those weeks, then a box on the resume week.

**Acceptance Scenarios**:
1. **Given** an active subscription, **When** the customer picks a resume date and confirms,
   **Then** the account page shows the subscription as paused until that date
2. **Given** a paused subscription, **When** a weekly cut-off passes inside the pause,
   **Then** no charge is made and no box is packed for that customer
3. **Given** a paused subscription, **When** the resume date arrives, **Then** the next box is
   charged and packed as normal

Feature: FEAT-005
