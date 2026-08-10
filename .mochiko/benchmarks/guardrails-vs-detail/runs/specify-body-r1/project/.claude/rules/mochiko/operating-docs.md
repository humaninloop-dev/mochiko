---
paths:
  - "ROADMAP.md"
  - "BACKLOG.md"
  - "DECISIONS.md"
  - "ARCHITECTURE.md"
  - "GLOSSARY.md"
  - "FEATURES.md"
  - "CHANGELOG.md"
  - "RUNBOOK.md"
  - ".mochiko/brainstorms/**"
  - ".mochiko/decisions/**"
  - ".mochiko/specs/**"
  - ".mochiko/features/**"
---

# Operating-docs shape contracts <!-- GI-015 -->

Editing any operating doc: honor its shape contract in `.mochiko/memory/knowledge-management.md`
(the project-pinned authority). Touch-time edit quality only — omission-class drift is caught at
command boundaries, not here.

- `ROADMAP.md` is the thin forward view (Thesis · Now/Next/Later · Standing bets · nothing else) — never a decision archive.
- `BACKLOG.md` holds open items only; closing moves the item to `.mochiko/archive/backlog-trail.md`.
- `DECISIONS.md` is a thin index over records; rationale lives in the record, not the index line.
- Closing or superseding work is one subtractive landing move (see the project-pinned copy).

Metadata: `.mochiko/memory/governance-ledger.md`, GI-015; invariants: `.mochiko/memory/knowledge-management.md`.
