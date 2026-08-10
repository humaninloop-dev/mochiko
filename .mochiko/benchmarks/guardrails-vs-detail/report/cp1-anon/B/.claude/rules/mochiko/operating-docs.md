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
---

# Operating docs — shape contracts <!-- GI-012 -->

Touch-time edit quality for the knowledge-management operating docs. The authoritative contracts and invariants are project-pinned at `.mochiko/memory/knowledge-management.md`; this file is the on-touch reminder.

- `ROADMAP.md` is the thin forward view — Thesis · Now/Next/Later (Now ≤5, Next ≤7, Later ≤10) · Standing bets — never a decision archive.
- `BACKLOG.md` holds open items only (no `[x]`); closing an item moves it to `.mochiko/archive/backlog-trail.md` as a one-line DONE + pointer.
- `DECISIONS.md` is a thin index over records — one line per ruled decision (date · title · status · pointer).
- Landing is one subtractive move: append the decision row, move the closed backlog item to the trail, touch ROADMAP Now/Next — statuses agreeing across the brainstorms index, the record, and the decisions index.
- Every pointer MUST resolve or carry the `provenance: unrecoverable` terminal stamp.

Full contracts, landing ritual, and invariants: `.mochiko/memory/knowledge-management.md`. Groom: `mochiko:grooming-operating-docs`.
