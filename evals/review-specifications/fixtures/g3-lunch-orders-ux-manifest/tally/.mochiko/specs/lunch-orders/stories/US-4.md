### User Story 4 - Order safely for a child with an allergy (Priority: P2)

A parent ordering for a child with a recorded allergy sees which items contain it and cannot
order those for that child.

**Why this priority**: The safety reason the school adopted pre-ordering; the kitchen must
never receive an unsafe order.

**Independent Test**: With a child recorded as allergic to milk, open the week menu for that
child and confirm every item containing milk is marked and cannot be chosen.

**Acceptance Scenarios**:
1. **Given** a child with a recorded allergen, **When** the parent views the week menu for that
   child, **Then** items containing the allergen are marked and cannot be selected
2. **Given** a family with two children, one allergic, **When** the parent switches child,
   **Then** the marks change to match the selected child

Feature: FEAT-005
