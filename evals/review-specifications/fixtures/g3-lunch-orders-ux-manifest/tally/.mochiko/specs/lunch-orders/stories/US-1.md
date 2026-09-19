### User Story 1 - Order a week of lunches for one of my children (Priority: P1)

A parent picks a child, sees the coming week's menu, chooses a meal for each school day, and
confirms; the wallet is charged and the kitchen has the order.

**Why this priority**: The feature is this story; everything else adjusts or shows an order.

**Independent Test**: With a topped-up wallet and a published menu, order five days for one
child and confirm the wallet drops by the total and the order shows on the kitchen sheet.

**Acceptance Scenarios**:
1. **Given** a published menu and a wallet that covers the week, **When** the parent picks a
   meal per day and confirms, **Then** the order is placed and the wallet shows the new balance
2. **Given** an order is placed, **When** the parent changes Tuesday's meal before Sunday
   midnight, **Then** the change is saved and the kitchen has the new choice
3. **Given** the wallet holds less than the order total, **When** the parent confirms, **Then**
   they are asked to top up and the order is held until they do

Feature: FEAT-005
