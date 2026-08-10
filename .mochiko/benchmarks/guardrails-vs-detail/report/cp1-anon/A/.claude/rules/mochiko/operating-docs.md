---
paths:
  - "ROADMAP.md"
  - "BACKLOG.md"
  - "DECISIONS.md"
  - "CHANGELOG.md"
  - "ARCHITECTURE.md"
  - "GLOSSARY.md"
  - "FEATURES.md"
  - ".mochiko/brainstorms/**"
  - ".mochiko/decisions/**"
  - ".mochiko/specs/index.md"
---

# Operating docs — shape contracts (edit-time) <!-- GI-016 -->

When editing any operating doc, honor its shape contract (full contracts + invariants:
`.mochiko/memory/knowledge-management.md`):

- `ROADMAP.md` — four pieces, one screen: Thesis · Now/Next/Later (linked) · Standing bets · nothing else. Never a decision archive.
- `BACKLOG.md` — open items only; one bounded entry per item in theme sections; no `[x]` items (done items move to `.mochiko/archive/backlog-trail.md`).
- `DECISIONS.md` — a thin index over records: one line per ruled decision (date · title · status · rationale pointer).
- `CHANGELOG.md` — release notes, newest first.
- `ARCHITECTURE.md` — the resulting system view (components, boundaries, data flow); decisions record *changes*, this records the *system*.
- `GLOSSARY.md` — `**<term>** — <definition> *(minted <date>, <source>)*`, one line per term.
- Closing or superseding work is the subtractive landing ritual (one three-part move), not an ad-hoc edit.

Rules inject on Read, not Write — this is edit-time reinforcement. Omission-class drift is caught
only at the command-boundary invariants, not here.
