---
paths:
  - "BACKLOG.md"
  - "ROADMAP.md"
  - "DECISIONS.md"
  - "ARCHITECTURE.md"
  - "GLOSSARY.md"
  - "CHANGELOG.md"
  - "RUNBOOK.md"
  - ".mochiko/brainstorms/**"
  - ".mochiko/decisions/**"
  - ".mochiko/specs/index.md"
---

# Operating docs — shape contracts (edit-time) <!-- GI-014 -->

These are touch-time edit rules only; they are structurally blind to omission, so omission-class
drift is caught at command-boundary invariants, not here. Full contracts + invariants:
`.mochiko/memory/knowledge-management.md`.

- `ROADMAP.md` — four pieces, one screen: Thesis · Now/Next/Later (one line each, linked) ·
  Standing bets & revisit conditions · nothing else. Never a decision archive.
- `BACKLOG.md` — open items only; one bounded entry per item (≤ 15 lines) in theme-keyed
  sections. Closing an item compresses it to a one-line DONE + pointer and MOVES it to
  `.mochiko/archive/backlog-trail.md` — never `[x]` left in place.
- `DECISIONS.md` — a thin index: one line per ruled decision (date · title · status · pointer to
  rationale home). Rationale lives in the record, not here.
- `GLOSSARY.md` term format: `**<term>** — <definition> *(minted <date>, <source>)*`.
- Landing is subtractive and one move: append the decision row · move the closed backlog item to
  the trail · touch `ROADMAP.md` Now/Next — statuses agreeing across indexes.

Metadata (enforcement · testability · rationale): `.mochiko/memory/governance-ledger.md`, GI-014.
