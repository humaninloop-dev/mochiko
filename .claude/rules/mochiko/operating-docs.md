---
paths:
  - "DECISIONS.md"
  - "ROADMAP.md"
  - "BACKLOG.md"
  - "ARCHITECTURE.md"
  - "GLOSSARY.md"
  - ".mochiko/brainstorms/index.md"
  - ".mochiko/decisions/**"
  - ".mochiko/archive/**"
---

# Operating-docs shape contract (knowledge-management module)

Touch-time quality for ad-hoc edits; full contracts + invariants: `.mochiko/memory/knowledge-management.md`.

- `DECISIONS.md` is the THIN decision index — one line per ruled decision (date · title · status · pointer); rationale lives in the pointed-at record, NEVER here. Superseded rows compress to one line, status preserved.
- `ROADMAP.md` holds exactly four pieces — thesis · Now/Next/Later (Now ≤5 · Next ≤7 · Later ≤10, linked, dated) · standing bets with revisit conditions · the stamp line. Nothing else.
- `BACKLOG.md` holds OPEN items only, ≤15 lines each, with a provenance pointer (or `provenance: unrecoverable (…)`); a closed item compresses to one line and MOVES to `.mochiko/archive/backlog-trail.md`.
- `.mochiko/archive/**` and the trail are append-only/frozen — never edit archived content.
- `.mochiko/decisions/` records follow: Status · Date · Context · Decision · Rationale · Alternatives considered.
- The brainstorms index mutates in place; statuses MUST agree with each record's Status line and the decision index.
- Closing or superseding work is one three-part move (row + trail + ROADMAP.md touch). A cap or bound trip → invoke `mochiko:grooming-operating-docs` on sight.
