### User Story 2 - Come back early from a pause (Priority: P2)

A paused customer decides they want boxes again sooner and resumes from the account page.

**Why this priority**: Holidays get cut short; without this the customer waits or cancels.

**Independent Test**: Pause for four weeks, resume after one, and confirm the next cut-off
charges and packs a box.

**Acceptance Scenarios**:
1. **Given** a paused subscription, **When** the customer chooses "Resume now", **Then** the
   account page shows the subscription active with the next delivery date
2. **Given** a customer resumed after the cut-off for the coming week, **When** they view the
   next delivery, **Then** it is the following week's box

Feature: FEAT-005
