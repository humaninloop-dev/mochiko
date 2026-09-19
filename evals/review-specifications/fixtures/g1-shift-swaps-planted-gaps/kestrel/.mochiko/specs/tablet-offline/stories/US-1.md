### User Story 1 - Clock in on the tablet when the wifi is down (Priority: P1)

A staff member taps clock-in on the venue tablet during a show with no network; the tablet keeps
it and the manager sees it once the network is back.

**Why this priority**: The whole feature; hours feed pay.

**Independent Test**: Disconnect the tablet, clock in, reconnect, and confirm one clock event
appears on the manager dashboard within a minute.

**Acceptance Scenarios**:
1. **Given** the tablet has no network, **When** a staff member clocks in, **Then** the tablet
   shows the clock-in as waiting to sync
2. **Given** queued clock-ins, **When** the network returns, **Then** each appears once on the
   manager dashboard within a minute

Feature: FEAT-004
