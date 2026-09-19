# Knowledge-Management — project-pinned invariants (Tidewatch)

Pinned 2026-03-02 from the plugin's knowledge-management module (plugin v0.29.0); ratified into
the ruled core 2026-07-03 (governance v1.1.0). Adopted core: open-only `BACKLOG.md` + trail
(`.mochiko/archive/backlog-trail.md`) · `ROADMAP.md` · `DECISIONS.md` · `ARCHITECTURE.md` as
the store's derived index · CLAUDE.md pointers. No brainstorms layer yet (zero sessions).

## Document contracts

**`ROADMAP.md`** — four pieces, one screen: Thesis (2–3 lines) · Now/Next/Later (one line per
item, linked) · standing bets + revisit conditions · last-groomed stamp line.

**`BACKLOG.md`** — open items only: bounded entries (title · date · provenance · resume-cold
context); closing compresses to a one-line DONE + pointer moved to the trail.

**Decisions layer** — one `DECISIONS.md` line per ruled decision: date · title · status
(`ruled` / `superseded by <pointer>`) · rationale home.

## Landing ritual (subtractive)

Closing or superseding work is ONE move with three parts, in the same moment at the command
landing step:

1. append the decision row (and any per-decision record);
2. move the closed `BACKLOG.md` item to the trail as its one-line DONE + pointer;
3. touch `ROADMAP.md` Now/Next.

A landing that only adds is incomplete.

## Invariants (run at command boundaries under fix-on-sight)

- **Open-only:** no `[x]` item in `BACKLOG.md` — done items live in the trail.
- **Horizon caps:** `ROADMAP.md` Now ≤ 5 · Next ≤ 7 · Later ≤ 10; every *Now* item points at
  live work.
- **Item bounds:** per-open-item size bound ≤ 15 lines.
- **Dead-pointer scan:** every `ROADMAP.md` / `DECISIONS.md` / `BACKLOG.md` pointer resolves.
- **Orphan rule:** every in-flight-class element in the architecture store — `in-flight` /
  `modifying` / `removing (FEAT-XXX)` — keys an open feature and resolves; an element keying a
  closed or missing feature is an orphan, surfaced by the derived index's health view and
  cleaned at the next desk visit.
- **Index agreement:** repo-root `ARCHITECTURE.md` agrees with the store it renders; fix by
  re-rendering, never by editing the index.
- **Presence:** the adopted core artifacts above exist.
