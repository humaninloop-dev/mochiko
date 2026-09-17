# Knowledge-Management — project-pinned invariants (Ledgerlite)

Pinned 2026-07-08 from `plugins/mochiko/templates/constitution-modules/knowledge-management.md`
(plugin v0.92.0) by the first `/mochiko:setup` run (governance v1.0.0, GI-009). The command
landing steps and the groom skill resolve against this copy at runtime; a template change
reaches it only as an amend offer. Adopted core set: brainstorms layer · open-only
`BACKLOG.md` + trail (`.mochiko/archive/backlog-trail.md`) · `ROADMAP.md` · `DECISIONS.md` +
`.mochiko/decisions/` · `ARCHITECTURE.md` (the store's derived index) · the `paths` rules file
(`.claude/rules/mochiko/operating-docs.md`) · CLAUDE.md pointers. Electives declined:
`CHANGELOG.md`, `RUNBOOK.md`.

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
Decision · Rationale · Alternatives considered.

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
  last-groomed baseline.
- **Dead pointers:** a pointer to a missing file or a missing index row is a defect.

A tripped cap or bound invokes `mochiko:grooming-operating-docs` on sight.
