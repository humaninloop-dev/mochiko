# Knowledge Management — project-pinned copy (Ledgerline)

Pinned at setup from `templates/constitution-modules/knowledge-management.md`. Command landing
steps and `mochiko:grooming-operating-docs` resolve against THIS file at runtime, never the plugin
template. A plugin-template change reaches this project only as an amend offer, never silent.

## Document contracts

- **`ROADMAP.md`** — four pieces, one screen: (1) Thesis (2–3 lines) · (2) Now / Next / Later, one
  line per item linked to its BACKLOG item or session record (Later is non-committed, link-exempt
  until promoted) · (3) Standing bets & revisit conditions · (4) nothing else. Dates on Now/Next
  and bets; a last-groomed stamp line with baseline figures. Never a decision archive.
- **`BACKLOG.md`** — open items only; one bounded entry per item (title, date, provenance pointer,
  resume-cold context) in theme-keyed sections that merge on groom. Closing compresses to a
  one-line DONE + pointer and MOVES to the trail — never deleted. Dead provenance gets
  `provenance: unrecoverable (<what>, removed <date>)`. Boundary with the feature map: product
  capabilities live on the map; BACKLOG carries defects, tooling, process.
- **Decisions layer** — one `DECISIONS.md` line per ruled decision: date · title · status
  (`ruled` / `superseded by <pointer>`) · pointer to rationale (session record, else
  `.mochiko/decisions/<YYYY-MM-DD>-<slug>.md`). Record schema (record-less decisions):
  Status · Date · Context · Decision · Rationale · Alternatives considered. Superseded rows
  compress to one line each, status preserved.
- **`GLOSSARY.md`** term format: `**<term>** — <definition> *(minted <date>, <source>)*`, one line each.
- **`ARCHITECTURE.md`** — the living system view (components, boundaries, data flow); records the
  resulting system, present tense; decisions record the changes. Folded at plan/implement landings
  on structural change via `mochiko:authoring-architecture`.

## Landing ritual (subtractive — one move, three parts)

Closing OR superseding work is ONE move at the command landing step:
1. append the decision row (and any per-decision record);
2. move the closed BACKLOG item to the trail as its one-line DONE + pointer;
3. touch ROADMAP Now/Next — and on supersession update BOTH indexes (brainstorms + decisions) so
   statuses agree.

A landing that only adds is incomplete. A `/mochiko:feature` lane acceptance is a landing event too.

## Invariants (run at command boundaries under fix-on-sight; vacuously satisfied at zero)

- **Bijection:** every `.mochiko/brainstorms/` dir has an `index.md` entry; every accepted entry names its landing.
- **Specs-index agreement:** every `.mochiko/specs/` dir has a `.mochiko/specs/index.md` entry; a row's status never contradicts the feature map (closed = its FEAT-IDs read `delivered`).
- **Status-agreement:** brainstorms-index status ↔ record Status line ↔ decisions-index status agree.
- **Open-only:** no `[x]` item in `BACKLOG.md` — done items live in the trail.
- **Horizon caps:** ROADMAP Now ≤ 5 · Next ≤ 7 · Later ≤ 10; every Now item points at live work.
- **Item bounds:** per-open-item ≤ 15 lines + an open-item-count watch against the last-groomed baseline.
- **Dead-pointer scan:** every ROADMAP / DECISIONS / BACKLOG pointer resolves, or carries the `provenance: unrecoverable` stamp.
- **In-flight agreement:** every `ARCHITECTURE.md` In-flight pointer targets an open feature and resolves.
- **Presence:** all core artifacts exist (electives only when adopted — here: RUNBOOK adopted, CHANGELOG declined).

A tripped cap or bound invokes `mochiko:grooming-operating-docs` on sight.

## Adopted set (Ledgerline)

Core: whole. Electives: RUNBOOK adopted; CHANGELOG declined (durable). Collision rulings: none —
clean names (greenfield).
