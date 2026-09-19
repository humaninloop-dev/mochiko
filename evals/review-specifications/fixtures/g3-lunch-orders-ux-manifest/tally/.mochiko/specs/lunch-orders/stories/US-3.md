### User Story 3 - See what my children are having this week and what they had (Priority: P2)

A parent sees this week's orders per child at a glance and can look back at past weeks.

**Why this priority**: Answers "what did you have today?" and a wallet query; not needed to
place an order.

**Independent Test**: With two children and orders in two past weeks, open the history and
confirm each week lists the right meals and amounts per child.

**Acceptance Scenarios**:
1. **Given** orders exist for past weeks, **When** the parent opens order history, **Then**
   each past week lists the child, the meals, and the amount charged
2. **Given** an order for the current week, **When** the parent opens this week, **Then** each
   day shows the chosen meal per child

Feature: FEAT-006
