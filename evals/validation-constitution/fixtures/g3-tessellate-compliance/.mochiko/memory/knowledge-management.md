# Knowledge management — project-pinned copy

Pinned by `/mochiko:setup` on 2026-09-09 from the plugin's `knowledge-management` module (core
adopted whole; elective `CHANGELOG.md` adopted, `RUNBOOK.md` declined — runbooks live in the
PagerDuty service). Command landing steps and `mochiko:grooming-operating-docs` resolve against
this file, never against the plugin template.

## Document contracts

- `ROADMAP.md` — four pieces, one screen: thesis · Now/Next/Later (one line each, linked, dated) · standing bets with revisit conditions · the last-groomed stamp line. Nothing else.
- `BACKLOG.md` — open items only, one bounded entry each (title, date, provenance pointer, resume-cold context) in theme-keyed sections; a closed item compresses to one line and moves to `.mochiko/archive/backlog-trail.md`.
- `DECISIONS.md` — one line per ruled decision (date · title · status · pointer to the record); records without a session live at `.mochiko/decisions/<YYYY-MM-DD>-<slug>.md` with Status · Date · Context · Decision · Rationale · Alternatives considered.
- `CHANGELOG.md` — one entry per production promotion; the top entry's version matches the deployed tag.
- `GLOSSARY.md` — `**<term>** — <definition> *(minted <date>, <source>)*`, one line per term.

## Landing ritual

Closing or superseding work is one move with three parts at the command landing step: append the decision row · move the closed `BACKLOG.md` item to the trail · touch `ROADMAP.md` Now/Next (and, on supersession, both indexes so statuses agree).

## Invariants (run at command boundaries under fix-on-sight)

Bijection (every `.mochiko/brainstorms/` directory has an `index.md` entry) · status-agreement · open-only `BACKLOG.md` · horizon caps Now ≤ 5 / Next ≤ 7 / Later ≤ 10 · item bounds (≤ 15 lines) with an open-item-count watch · dead-pointer scan (`provenance: unrecoverable (…)` is the terminal stamp) · specs-index agreement (every `.mochiko/specs/` directory has an index entry under the same open/close contract; a row never contradicts the feature map) · orphan rule (every in-flight-class element in the architecture store keys an open feature and resolves) · index agreement (the derived `ARCHITECTURE.md` agrees with the store it renders; fixed by re-rendering, never by editing the index) · presence of all core artifacts and the adopted elective. Vacuously satisfied at zero sessions, items, and pointers.
