# Knowledge Management — project-pinned copy (Ledgerline)

Pinned at setup (v1.0.0, 2026-08-10) from the `knowledge-management` module template. Command
landing steps and `mochiko:grooming-operating-docs` resolve against THIS file, never the plugin
template. A plugin-template change reaches this project only as an amend offer.

**Adopted:** core (whole) + `CHANGELOG.md` elective. **Declined/deferred:** `RUNBOOK.md`
(re-offer at the amend run after first deploy — GI-016b / GI-023).

## Document contracts

**`ROADMAP.md` — four pieces, one screen, nothing else:** (1) **Thesis** — 2–3 lines on what
the project is becoming and the core bet; (2) **Now / Next / Later** — one line per item, each
linked to its BACKLOG item or session record (*Later* is non-committed, exempt from the link
rule until promoted to *Next*); (3) **Standing bets & revisit conditions** — the few strategic
contested/provisional bets, each with the condition that reopens it; (4) **nothing else** —
decision rows, trails, and rationale prose live in the decisions layer. Dates on Now/Next items
and bets; a last-groomed stamp line carries the groom date + baseline figures.

**`BACKLOG.md` — open items only:** one bounded entry per item (title, date, provenance pointer,
resume-cold context) in theme-keyed sections that merge on groom. Closing an item never deletes
it: it compresses to the one-line DONE + pointer form and **moves to the trail**
(`.mochiko/archive/backlog-trail.md`). Dead provenance gets the terminal stamp
`provenance: unrecoverable (<what it was>, removed <date>)`. Boundary with the feature map:
product capabilities live on the map as `proposed` entries; `BACKLOG.md` carries everything else
— defects, tooling, process.

**Decisions layer — a thin index over records:** one `DECISIONS.md` line per **ruled decision**
— date · title · status (`ruled` / `superseded by <pointer>`) · pointer to the rationale home
(a session record where one exists, else a per-decision record at
`.mochiko/decisions/<YYYY-MM-DD>-<slug>.md`). Record schema (record-less decisions only): **Status
· Date · Context · Decision · Rationale · Alternatives considered**. At groom, a superseded row
compresses to one line per superseded decision, status preserved.

**`GLOSSARY.md` term format:** `**<term>** — <definition> *(minted <date>, <source>)*`, one line
per term.

## Landing ritual (subtractive — enforced floor)

Closing **or superseding** work is ONE move with three parts, executed in the same moment at the
command landing step (brainstorm close · setup/amend · specify/plan/implement landings):

1. append the decision row (and any per-decision record);
2. move the closed `BACKLOG.md` item to the trail as its one-line DONE + pointer;
3. touch `ROADMAP.md` Now/Next — and, on supersession, update **both** indexes (brainstorms +
   decisions) so statuses agree.

A landing that only adds is incomplete: subtraction is part of the same move.

## Invariants (run at command boundaries under fix-on-sight)

- **Bijection:** every directory under `.mochiko/brainstorms/` has an `index.md` entry; every
  accepted entry names its landing (a `DECISIONS.md` row, or an explicit no-graduation).
- **Specs-index agreement:** every directory under `.mochiko/specs/` has a `.mochiko/specs/index.md`
  entry, same open/close contract; a row's status never contradicts the feature map (closed = its
  selected FEAT-IDs read `delivered`).
- **Status-agreement:** brainstorms-index status ↔ record `Status` line ↔ decisions-index status
  agree.
- **Open-only:** no `[x]` item in `BACKLOG.md` — done items live in the trail.
- **Horizon caps:** `ROADMAP.md` Now ≤ 5 · Next ≤ 7 · Later ≤ 10; every *Now* item points at
  live work.
- **Item bounds:** per-open-item size bound (default ≤ 15 lines) + an open-item-count watch
  against the last-groomed baseline (baseline figures on the stamp line).
- **Dead-pointer scan:** every `ROADMAP.md` / `DECISIONS.md` / `BACKLOG.md` pointer resolves, or
  carries the `provenance: unrecoverable` terminal stamp.
- **In-flight agreement:** every `ARCHITECTURE.md` In-flight pointer targets an open feature and
  resolves.
- **Presence:** all core artifacts exist (electives only when adopted).
- Vacuously satisfied at zero sessions / items / pointers.

A tripped cap or bound invokes `mochiko:grooming-operating-docs` on sight; the groom attaches to
already-firing boundaries, never to user initiative.
