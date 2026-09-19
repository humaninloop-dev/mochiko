### User Story 2 - Change or cancel an order before the cut-off (Priority: P1)

A parent changes a day's meal or cancels a day before the week's cut-off; after it, they are
told why they cannot.

**Why this priority**: Plans change; without this a wrong order is a wasted meal and a charge
the parent disputes.

**Independent Test**: Place an order, change one day before the cut-off and confirm the kitchen
sheet updates; try again after the cut-off and confirm the refusal names the cut-off.

**Acceptance Scenarios**:
1. **Given** an order and a cut-off not yet passed, **When** the parent cancels Wednesday,
   **Then** Wednesday's price returns to the wallet and the kitchen count drops by one
2. **Given** the cut-off has passed, **When** the parent tries to change any day, **Then** the
   change is refused with the cut-off time shown

Feature: FEAT-005
