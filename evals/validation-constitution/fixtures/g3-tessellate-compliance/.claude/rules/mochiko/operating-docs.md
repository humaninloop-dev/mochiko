---
paths:
  - "DECISIONS.md"
  - "ROADMAP.md"
  - "BACKLOG.md"
  - "ARCHITECTURE.md"
  - "GLOSSARY.md"
  - "CHANGELOG.md"
  - ".mochiko/brainstorms/index.md"
  - ".mochiko/decisions/**"
  - ".mochiko/archive/**"
---

# Operating-docs shape contract (knowledge-management module)

Touch-time quality for ad-hoc edits; full contracts + invariants: `.mochiko/memory/knowledge-management.md`.

- `DECISIONS.md` is the thin decision index — one line per ruled decision (date · title · status · pointer); rationale lives in the pointed-at record, never here.
- `ROADMAP.md` holds exactly four pieces — thesis · Now/Next/Later (Now ≤5 · Next ≤7 · Later ≤10, linked, dated) · standing bets with revisit conditions · the stamp line. Nothing else.
- `BACKLOG.md` holds open items only, ≤15 lines each, with a provenance pointer; a closed item compresses to one line and moves to `.mochiko/archive/backlog-trail.md`.
- `CHANGELOG.md` gains an entry at every production promotion; the top entry's version matches the deployed task definition's tag.
- `.mochiko/archive/**` and the trail are append-only; never edit archived content.
- Closing or superseding work is one three-part move (row + trail + `ROADMAP.md` touch). A cap or bound trip invokes `mochiko:grooming-operating-docs` on sight.
