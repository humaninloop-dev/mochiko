# Knowledge-Management — project-pinned invariants (Fieldnote)

Pinned 2026-06-20 by the /mochiko:setup run (module: knowledge-management, GI-008). The
adopted set: `DECISIONS.md` · open-only `BACKLOG.md` + trail (`.mochiko/archive/backlog-trail.md`)
· `ROADMAP.md` · CLAUDE.md pointers. `ARCHITECTURE.md` at the repo root is the founder's
write-up today and is not groomed here.

## Document contracts

**`ROADMAP.md`** — thesis (2–3 lines) · Now/Next/Later (one line per item, linked) · standing
bets with revisit conditions · a last-groomed stamp line.

**`BACKLOG.md`** — open items only: bounded entries (title · date · provenance · resume-cold
context) in theme-keyed sections; closing compresses an item to a one-line DONE + pointer
moved to the trail.

**Decisions layer** — one `DECISIONS.md` line per ruled decision: date · title · status
(`ruled` / `superseded by <pointer>`) · rationale home (a session record or a
`.mochiko/decisions/<YYYY-MM-DD>-<slug>.md` file).

## Landing ritual (subtractive)

Closing or superseding work is one move with three parts, taken at the command's landing
step: append the decision row (and any per-decision record); move the closed `BACKLOG.md`
item to the trail as its one-line DONE + pointer; touch `ROADMAP.md` Now/Next. A landing that
only adds is incomplete.

## Invariants (checked at command boundaries)

- **Open-only:** no `[x]` item in `BACKLOG.md` — done items live in the trail.
- **Horizon caps:** `ROADMAP.md` Now ≤ 5 · Next ≤ 7 · Later ≤ 10; every Now item points at
  live work.
- **Item bounds:** each open backlog item ≤ 15 lines.
- **Dead-pointer scan:** every `ROADMAP.md` / `DECISIONS.md` / `BACKLOG.md` pointer resolves,
  or carries `provenance: unrecoverable (<what it was>, removed <date>)`.
- **Status-agreement:** a decision's status reads the same in `DECISIONS.md` and in its
  rationale home.
