### User Story 2 - Answer a swap request a colleague sent me (Priority: P1)

A staff member sees a request from a colleague, with both shifts side by side, and accepts or
declines it.

**Why this priority**: A request nobody can answer is a message, not a swap.

**Independent Test**: Send a request to a staff member, accept it from their phone, and confirm
the requester is told and the rota reflects the exchange.

**Acceptance Scenarios**:
1. **Given** a staff member has an open request, **When** they open it, **Then** they see
   their own shift, the offered shift, and who is asking
2. **Given** an open request, **When** the staff member declines, **Then** the requester is
   told and the request closes
3. **Given** an open request, **When** the staff member accepts, **Then** the requester is
   told and the exchange is recorded

Feature: FEAT-006
