### User Story 1 - Ask a colleague to take one of my shifts (Priority: P1)

A staff member picks one of their published shifts, picks a colleague's published shift in
return, and sends the request; the colleague is asked to answer.

**Why this priority**: The core of the feature — nothing else exists without a request.

**Independent Test**: Publish a week with two staff on different shifts, request a swap from
one to the other, and confirm the request appears on the colleague's phone and in the manager's
view.

**Acceptance Scenarios**:
1. **Given** a staff member viewing one of their published shifts, **When** they choose "Swap"
   and pick a colleague's shift, **Then** a request is created and the colleague is notified
2. **Given** a request has been sent, **When** the colleague accepts, **Then** the published
   rota shows the two shifts exchanged and both staff see the new times
3. **Given** a staff member has sent a request, **When** they withdraw it before an answer,
   **Then** the request closes and the colleague sees it was withdrawn

Feature: FEAT-006
