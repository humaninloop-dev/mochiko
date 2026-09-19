# FEAT-002 — Note search

**Status:** selected · **Spec:** [.mochiko/specs/note-search/spec.md](.mochiko/specs/note-search/spec.md)
**Architecture link:** — (not yet filled; no store delta exists for this capability)

## Extent

Full-text search over stored notes through the HTTP API, with an index kept fresh in
the background.

## Work rows (selected for this run — ratified scope, selection source: the spec's
accepted selection, 2026-08-26)

- [ ] W1 — Search notes by query string (US-101; acceptance criteria SC-101, SC-102)
  — selected
- [ ] W2 — Index stays fresh after creates (US-102; acceptance criteria SC-103) —
  selected

## Dependencies

FEAT-001 (delivered) — search reads the notes store FEAT-001 built.
