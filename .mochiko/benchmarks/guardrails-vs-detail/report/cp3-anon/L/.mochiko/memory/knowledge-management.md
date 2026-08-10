# Knowledge Management — project-pinned invariants (Ledgerline)

Project-pinned copy of the knowledge-management module's Document-contracts + Landing-ritual +
Invariants. Command landing steps and `mochiko:grooming-operating-docs` resolve against THIS file
at runtime. A plugin upgrade reaches this project only as an amend offer, never silent enforcement.

## Document contracts

**`ROADMAP.md` — four pieces, one screen:** (1) Thesis (2–3 lines); (2) Now / Next / Later (one
line/item, each linked to its BACKLOG item or session record — Later is non-committed, link-exempt
until promoted); (3) Standing bets & revisit conditions; (4) nothing else — decision rows/trails/
rationale live in the decisions layer. Dates on Now/Next items + bets; a last-groomed stamp line.

**`BACKLOG.md` — open items only:** one bounded entry per item (title, date, provenance pointer,
resume-cold context) in theme-keyed sections that merge on groom. Closing compresses to one-line
DONE + pointer and moves to `.mochiko/archive/backlog-trail.md`. Dead provenance →
`provenance: unrecoverable (<what>, removed <date>)`. Product capabilities live on the feature map;
BACKLOG carries defects, tooling, process.

**Decisions layer — thin index over records:** one `DECISIONS.md` line per ruled decision — date ·
title · status (`ruled` / `superseded by <pointer>`) · pointer to rationale (session record, else
`.mochiko/decisions/<YYYY-MM-DD>-<slug>.md`). Record schema (record-less only): Status · Date ·
Context · Decision · Rationale · Alternatives considered. Superseded rows compress one line each,
status preserved.

**`GLOSSARY.md` term format:** `**<term>** — <definition> *(minted <date>, <source>)*`, one/line.

## Landing ritual (subtractive — one move, three parts)

Closing OR superseding work is ONE move at the command landing step:
1. append the decision row (+ any per-decision record);
2. move the closed `BACKLOG.md` item to the trail as its one-line DONE + pointer;
3. touch `ROADMAP.md` Now/Next — and on supersession update BOTH indexes (brainstorms + decisions)
   so statuses agree.

A landing that only adds is incomplete. A `/mochiko:feature` lane acceptance is a landing event —
same ritual.

## Invariants (run at command boundaries, fix-on-sight; vacuous at zero)

- **Bijection:** every `.mochiko/brainstorms/` dir has an `index.md` entry; every accepted entry names its landing.
- **Specs-index agreement:** every `.mochiko/specs/` dir has a `.mochiko/specs/index.md` entry; a row's status never contradicts the feature map (closed = selected FEAT-IDs read `delivered`).
- **Status-agreement:** brainstorms-index status ↔ record `Status` ↔ decisions-index status agree.
- **Open-only:** no `[x]` item in `BACKLOG.md` — done lives in the trail.
- **Horizon caps:** `ROADMAP.md` Now ≤ 5 · Next ≤ 7 · Later ≤ 10; every Now item points at live work.
- **Item bounds:** per-open-item ≤ 15 lines + an open-item-count watch against the last-groomed baseline.
- **Dead-pointer scan:** every ROADMAP/DECISIONS/BACKLOG pointer resolves or carries the `provenance: unrecoverable` stamp.
- **In-flight agreement:** every `ARCHITECTURE.md` In-flight pointer targets an open feature and resolves.
- **Presence:** all core artifacts exist (electives only when adopted — here CHANGELOG.md + RUNBOOK.md).
- Vacuously satisfied at zero sessions / items / pointers.

A tripped cap or bound invokes `mochiko:grooming-operating-docs` on sight (attached to firing
command boundaries, never user initiative).
