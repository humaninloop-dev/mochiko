# Knowledge-Management — project-pinned invariants (mochiko)

Pinned 2026-07-25 from `plugins/mochiko/templates/constitution-modules/knowledge-management.md`
(plugin v0.29.0) — the runtime source commands and the groom skill resolve; template changes
reach this copy only as amend offers. Hand-pinned at migration; **ratified into the ruled core
2026-08-06** by the first in-repo `/mochiko:setup` run (governance v1.0.0, GI-009 — the pin's
revisit trigger, now discharged). The adopted core set: brainstorms layer · open-only
`BACKLOG.md` + trail (`.mochiko/archive/backlog-trail.md`) · `ROADMAP.md` · `DECISIONS.md` +
`.mochiko/decisions/` · `ARCHITECTURE.md` (deferral retired 2026-08-06 — doc gained content;
the store's derived index from 2026-08-19, `product-architecture-schema` D4) ·
the `paths` rules file (`.claude/rules/mochiko/operating-docs.md`) · CLAUDE.md pointers.
Elective adopted: `CHANGELOG.md` (GI-010, release-gated). Elective declined durable:
`RUNBOOK.md` (GI-011). **Deviation carried (re-ratified 2026-08-06):** `GLOSSARY.md` deferred
until it gains content; command-boundary compliance manual until more commands run in-repo.
(The `specify.md` no-KM-landing-step deviation was struck 2026-08-06 — the landing line
shipped at v0.54.0, audit PASS; DECISIONS row + ADR `2026-08-06-specify-km-landing`.)
**Revisit trigger:** GLOSSARY.md gains content → scaffold and fold in.

## Document contracts

**`ROADMAP.md`** — four pieces, one screen, nothing else: Thesis (2–3 lines) · Now/Next/Later
(one line per item, linked; *Later* exempt until promotion) · standing bets + revisit
conditions · last-groomed stamp line with baseline figures; dates on Now/Next items and bets.

**`BACKLOG.md`** — open items only: bounded entries (title · date · provenance ·
resume-cold context) in theme-keyed sections merged on groom; closing compresses to a one-line
DONE + pointer moved to the trail; dead provenance stamped
`provenance: unrecoverable (<what it was>, removed <date>)`.

**Decisions layer** — one `DECISIONS.md` line per ruled decision: date · title · status
(`ruled` / `superseded by <pointer>`) · rationale home (session record, else
`.mochiko/decisions/<YYYY-MM-DD>-<slug>.md`). Record schema: Status · Date · Context ·
Decision · Rationale · Alternatives considered. Superseded rows compress one line per
decision, status preserved.

**`GLOSSARY.md` term format** — `**<term>** — <definition> *(minted <date>, <source>)*`, one
line per term.

## Landing ritual (subtractive)

Closing **or superseding** work is ONE move with three parts, in the same moment at the command
landing step (brainstorm close · setup/amend · specify/implement landings):

1. append the decision row (and any per-decision record);
2. move the closed `BACKLOG.md` item to the trail as its one-line DONE + pointer;
3. touch `ROADMAP.md` Now/Next — on supersession, update **both** indexes (brainstorms +
   decisions) so statuses agree.

A landing that only adds is incomplete.

## Invariants (run at command boundaries under fix-on-sight)

- **Bijection:** every directory under `.mochiko/brainstorms/` has an `index.md` entry; every
  accepted entry names its landing (a `DECISIONS.md` row, or an explicit no-graduation).
- **Status-agreement:** brainstorms-index status ↔ record `Status` line ↔ decisions-index
  status agree wherever the same decision appears.
- **Open-only:** no `[x]` item in `BACKLOG.md` — done items live in the trail.
- **Horizon caps:** `ROADMAP.md` Now ≤ 5 · Next ≤ 7 · Later ≤ 10; every *Now* item points at
  live work.
- **Item bounds:** per-open-item size bound ≤ 15 lines + an open-item-count watch against the
  last-groomed baseline (figures on the stamp lines).
- **Dead-pointer scan:** every `ROADMAP.md` / `DECISIONS.md` / `BACKLOG.md` pointer resolves,
  or carries `provenance: unrecoverable (<what it was>, removed <date>)`.
- **Orphan rule:** every in-flight-class element in the architecture store
  (`.mochiko/product/architecture/`) — `in-flight` / `modifying` / `removing (FEAT-XXX)` — keys
  an open feature and resolves; an element keying a closed or missing feature is an **orphan**,
  surfaced by the derived index's health view and cleaned at the next desk visit.
  (Supersedes the AT-D6-C In-flight-agreement invariant added 2026-08-04 from the
  `architecture-tieback` record — per `product-architecture-schema` D10, `DECISIONS.md`
  2026-08-19; landed direct by ruling, not via amend offer.)
- **Index agreement:** repo-root `ARCHITECTURE.md` is the store's derived index and agrees with
  the store it renders; a disagreement is a defect, fixed by re-rendering, never by editing the
  index. Both this and the orphan rule **bind once the store carries ruled content** — where the
  store carries no ruled content (scaffold-only or absent), `ARCHITECTURE.md` stays
  hand-maintained legacy until
  the first `/mochiko:architecture` visit reconstructs the store from it and converts it (D16);
  before that bootstrap both invariants are vacuous.
  (Added 2026-08-19, `product-architecture-schema` D4 as folded at review, S12; no-ruled-content
  carve per D16.)
- **Presence:** the adopted core artifacts above exist.
- Vacuously satisfied at zero sessions / zero items.

A tripped cap or bound invokes `mochiko:grooming-operating-docs` on sight.
