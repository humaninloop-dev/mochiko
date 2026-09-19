# Specification — Note capture (FEAT-001)

**Status:** accepted (selection ratified 2026-08-20) · No UX surface — HTTP API only,
so no Screens & Flows section.

## User stories

### US-001 — Create a note (P1)

As an API consumer, I create a note so my text is durably stored.

- **Given** a running service, **When** I POST /notes with `{"text": "milk"}`,
  **Then** I receive 201 with the note's id and the note is readable afterwards.
- **Given** a running service, **When** I POST /notes with an empty body,
  **Then** I receive 400 and nothing is stored.

Independent test: POST then GET round-trip against a fresh database file.

### US-002 — Fetch a note by id (P1)

As an API consumer, I fetch a stored note by its id.

- **Given** a stored note, **When** I GET /notes/{id}, **Then** I receive 200 with its
  text and created_at.
- **Given** no note with that id, **When** I GET /notes/{id}, **Then** I receive 404.

Independent test: GET of a seeded id and of a missing id.

## Functional requirements

- FR-001: The system MUST persist created notes across process restarts.
- FR-002: The system MUST reject a create request whose text is empty or missing
  with HTTP 400.
- FR-003: The system MUST return 404 for a fetch of a nonexistent note id.
- FR-004: The system MUST assign each note a unique id and a created_at timestamp.

## Success criteria

- SC-001: A created note is retrievable with identical text after service restart.
- SC-002: Empty-text creates are rejected 400 and leave the store unchanged.
- SC-003: Fetch returns 200+body for existing ids, 404 otherwise.

## Edge cases

- Text at 10,000 characters: accepted (no length limit in v1).
- Concurrent creates: ids never collide (unique constraint).
