### User Story 4 - Skip just one week without pausing (Priority: P3)

A customer wants no box next week but does not want to set up a pause.

**Why this priority**: Frequent ask, but a workaround exists.

**Independent Test**: Skip next week and confirm no charge and no box for that week only.

**Acceptance Scenarios**:
1. **Given** an active subscription, **When** the customer skips next week, **Then** that week
   is not charged and the following week is delivered as normal

Status: rejected — single-week skip is delivery scheduling (FEAT-002), already delivered as
"skip a delivery"; nothing new to build.
