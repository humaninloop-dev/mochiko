### User Story 1 - Swap an item in next week's box (Priority: P1)

A customer opens next week's box, swaps an item they cannot eat for one on the swap list of the
same value, and the packhouse packs the swapped box.

**Why this priority**: The whole feature; the alternative is a cancellation.

**Independent Test**: Swap the leeks for carrots before the cut-off and confirm the packhouse
manifest line shows carrots for that customer.

**Acceptance Scenarios**:
1. **Given** next week's box and the swap list, **When** the customer swaps one item, **Then**
   the box preview shows the new item and the manifest updates
2. **Given** three swaps already made, **When** the customer tries a fourth, **Then** it is
   refused with the limit shown

Feature: FEAT-003
