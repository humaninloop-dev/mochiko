# Specification — Note search (FEAT-002)

**Status:** accepted (selection ratified 2026-08-26) · No UX surface — HTTP API only,
so no Screens & Flows section.

## User stories

### US-101 — Search notes by query (P1)

As an API consumer, I search my notes by a query string and get matching notes ranked
by recency.

- **Given** stored notes "buy milk" and "call mom", **When** I GET
  /notes/search?q=milk, **Then** I receive 200 with exactly the "buy milk" note.
- **Given** no matching note, **When** I GET /notes/search?q=zebra, **Then** I receive
  200 with an empty result list.
- **Given** a missing q parameter, **When** I GET /notes/search, **Then** I receive
  400.

Independent test: seed two notes, query for each term and for a miss.

### US-102 — Index stays fresh (P1)

As an API consumer, a note I just created is findable without restarting anything.

- **Given** a running service, **When** I POST a new note and immediately GET
  /notes/search?q=<its text>, **Then** the new note appears in the results within 2
  seconds.

Independent test: create-then-search round trip under the 2-second bound.

## Functional requirements

- FR-101: The system MUST return notes whose text contains the query term,
  case-insensitive, ranked newest first.
- FR-102: The system MUST reject a search without a q parameter with HTTP 400.
- FR-103: The system MUST keep the search index fresh: a **background index worker**
  rebuilds the index off the request path, so a created note is searchable within 2
  seconds without blocking the create response.

## Success criteria

- SC-101: Matching notes returned, non-matching excluded, newest first.
- SC-102: Missing q rejected 400.
- SC-103: Create-to-searchable latency ≤ 2 s at v1 scale (≤10k notes) with creates
  never blocked on indexing.

## Edge cases

- Query term shorter than 2 characters: 400 (avoids full scans).
- Note created while a rebuild is in flight: picked up by the next rebuild tick.
