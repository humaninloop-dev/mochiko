### User Story 1 - Hide an item from parents without deleting it (Priority: P1)

The kitchen marks an item hidden; parents no longer see it anywhere; the kitchen can bring it back.

**Why this priority**: The whole feature; a recalled item shown to parents is a safety failure.

**Independent Test**: Hide an item and confirm it is absent from the parent week menu and from
the menu API response, then un-hide it and confirm it returns.

**Acceptance Scenarios**:
1. **Given** a published menu, **When** the kitchen hides an item, **Then** the parent week menu
   no longer lists it
2. **Given** a hidden item, **When** the kitchen un-hides it, **Then** parents see it again with
   its price and allergens

Feature: FEAT-002
