# Feature Specification — Lending

- **Status:** US-001–US-003 mocked and agreed 2026-08-20 · US-004, US-005 added 2026-09-08 (no screens yet) · **Author:** Rosa (volunteer lead) · **Date:** 2026-09-08

## User Stories

### US-001 — Find an item (P1)

As a member, I want to browse and search the catalogue, so that I can see what the library has
and whether it is available.

- **Scenario 1 — Given** the catalogue, **when** I search for a word in an item's name, **then**
  I see matching items with whether each is available now.

### US-002 — Borrow an item (P1)

As a member, I want to borrow an available item, so that I can take it home for three weeks.

- **Scenario 1 — Given** an item with a copy available, **when** I borrow it, **then** it shows
  on my loans with a due date three weeks out.

### US-003 — See my loans (P1)

As a member, I want to see everything I have out and when it is due, so that I return things
on time.

- **Scenario 1 — Given** I have loans, **when** I open my loans, **then** I see each item, its
  due date, and whether it is overdue.

### US-004 — Reserve an item that is out (P1) — *new 2026-09-08*

As a member, I want to reserve an item that is currently on loan, so that I get it when it
comes back.

- **Scenario 1 — Given** an item with no copy available, **when** I reserve it, **then** I see
  my place in the queue and the earliest date I might get it.
- **Scenario 2 — Given** I am next in the queue and a copy is returned, **when** I open my
  reservations, **then** I see it is ready for me to collect and the date the hold ends.
- **Scenario 3 — Given** a reservation of mine, **when** I cancel it, **then** it is removed and
  the queue moves up.

### US-005 — Renew a loan (P2) — *new 2026-09-08*

As a member, I want to renew a loan once before it is due, so that I can keep an item a bit
longer.

- **Scenario 1 — Given** a loan that has not been renewed and an item nobody is waiting for,
  **when** I renew it, **then** its due date moves three weeks later and it shows as renewed.
- **Scenario 2 — Given** a loan on an item that someone is waiting for, **when** I try to renew
  it, **then** I am told I cannot and why.

## Functional Requirements

- **FR-001** A member MUST NOT hold more than eight items at once. *Source: US-002*
- **FR-002** A returned item with a reservation queue MUST be held for the next member for
  seven days. *Source: US-004*
- **FR-003** A loan MAY be renewed once; a renewal MUST be refused when a reservation queue
  exists on the item. *Source: US-005*

## Screens & Flows

Mock: `prototype/` — open `prototype/index.html`.

### Screens

| ID | Screen | Purpose | Data shown | Story |
|---|---|---|---|---|
| SCR-001 | Catalogue | Browse and search items | item name, category, available now (y/n) | US-001 |
| SCR-002 | Item detail | See one item and borrow it | name, category, copies available, due-back date if none available | US-002 |
| SCR-003 | My loans | See what I have out | item name, due date, overdue flag | US-003 |
| SCR-004 | Reservations | *coming soon* | — | — |

### Flows

| ID | Steps | Scenario |
|---|---|---|
| FLOW-001 | SCR-001 → type a word → results filter | US-001 Scenario 1 |
| FLOW-002 | SCR-001 → open item → SCR-002 → Borrow → SCR-003 shows it | US-002 Scenario 2 |
| FLOW-003 | nav My loans → SCR-003 | US-003 Scenario 1 |
