---
paths:
  - "ROADMAP.md"
  - "BACKLOG.md"
  - "DECISIONS.md"
  - "ARCHITECTURE.md"
  - "GLOSSARY.md"
  - "FEATURES.md"
  - ".mochiko/brainstorms/**"
  - ".mochiko/decisions/**"
  - ".mochiko/features/**"
---

# Operating docs — shape contracts <!-- GI-016 -->

Touch-time edit quality for the knowledge-management operating docs. Full invariants and the
landing ritual are project-pinned at `.mochiko/memory/knowledge-management.md` — resolve against
that copy, not this file.

- `ROADMAP.md` — four pieces, one screen: Thesis · Now/Next/Later (each linked to a BACKLOG item
  or record) · Standing bets & revisit conditions · nothing else. Horizon caps: Now ≤5, Next ≤7,
  Later ≤10. Never a decision archive.
- `BACKLOG.md` — open items only, one bounded entry each (≤15 lines) in theme-keyed sections. No
  `[x]` items — closed items move to `.mochiko/archive/backlog-trail.md`.
- `DECISIONS.md` — a thin index over records: one line per ruled decision (date · title · status ·
  pointer to the rationale home). Superseded rows compress to one line, status preserved.
- Landing is subtractive: closing or superseding work appends the decision row, moves the closed
  BACKLOG item to the trail, and touches ROADMAP Now/Next — one move, same moment.
- Every pointer MUST resolve, or carry the `provenance: unrecoverable (<what>, removed <date>)`
  terminal stamp.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-016.
