---
paths:
  - "ROADMAP.md"
  - "BACKLOG.md"
  - "DECISIONS.md"
  - "ARCHITECTURE.md"
  - "FEATURES.md"
  - "GLOSSARY.md"
  - "RUNBOOK.md"
  - ".mochiko/brainstorms/**"
  - ".mochiko/decisions/**"
  - ".mochiko/features/**"
---

# Operating docs (knowledge-management) <!-- GI-014 -->

Shape contracts for touch-time edits (omission-class drift is caught at command boundaries, not here):

- `ROADMAP.md` — four pieces, one screen: Thesis · Now/Next/Later (linked) · Standing bets · nothing else. Not a decision archive.
- `BACKLOG.md` — open items only, bounded entries in theme-keyed sections; closing an item moves it to the trail, never deletes.
- `DECISIONS.md` — one line per ruled decision (date · title · status · pointer to rationale).
- Landing ritual is subtractive and one move: append the decision row · move the closed backlog item to the trail · touch ROADMAP Now/Next.
- Full invariants + ritual: `.mochiko/memory/knowledge-management.md`; groom via `mochiko:grooming-operating-docs`.

Metadata: `.mochiko/memory/governance-ledger.md`, GI-014.
