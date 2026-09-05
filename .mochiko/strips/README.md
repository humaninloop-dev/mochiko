# Strip Notes — the minimalism pass's per-primitive log

<!--
Convention home (design: .mochiko/brainstorms/pattern-codification-and-minimalism/record.md,
D6 + folds S5/S10; ruling: ROADMAP Key Decisions 2026-07-18). This directory is deliberately
NON-LOADED: no command or runtime surface references it — the log must never itself cost
context. It is read at framework-maintenance time only (strip waves, dogfood re-add reviews,
validation-command-shape audits). Strip notes never live inside commands/ (files there
register as commands) and never inline in SKILL.md/command bodies.

LOCATION (amended 2026-07-19, user ruling): `.mochiko/strips/` — repo-side, beside the other
operational layers (.mochiko/transform/, .mochiko/brainstorms/). Strip notes are operational
maintenance logs and must NEVER live under `plugins/` — the plugin directory is the shipped
artifact, and anything inside it distributes with the plugin whether loaded or not. (The
original D6 ruling placed them at plugins/mochiko/strips/; relocated out of the shippable
tree once the first wave made the leak visible.) A future wave that writes a strip note
anywhere under `plugins/` is a defect — fix on sight.
-->

One file per primitive: `strips/<primitive>.md` (e.g. `strips/brainstorm.md` for
`commands/brainstorm.md`, `strips/loop-discipline.md` for the skill). Entries are appended
newest-first, each stamped with the plugin version that made it. Four entry types:

## Strip entry — a line that left the primitive

```
## [vX.Y.Z] <one-line description>
- **Disposition:** relocated → <single-sourced home> | deleted
- **Tier failed:** 1 (altitude — restated doctrine/pattern) | 2 (no behavior/failure named)
- **Content:** <the stripped text, verbatim or faithfully compressed>
- **Consumers assessed:** <shared primitives only: every consumer checked, per D9's guard>
```

## Supersession-by-ruling entry — a line a ruling retired (not a minimalism strip)

```
## [vX.Y.Z] <one-line description>
- **Disposition:** superseded → <the home that now carries it, or the rewrite that replaced it>
- **Tier failed:** n/a — supersession by ruling (<record or ADR + the decision ID>)
- **Content:** <the superseded text, verbatim or faithfully compressed>
- **Kept deliberately:** <what survived the same edit, and why — omit when nothing did>
- **Consumers assessed:** <shared primitives only, as above>
```

The ground is a **decision**, never a tier: the line was not too verbose, it was made wrong —
or made someone else's — by a ruling. Cite the ruling; a tier number here is a category error.
The *Kept deliberately* field exists because a doctrine reversal usually spares part of what it
touches, and an unrecorded survivor reads to the next auditor as an oversight.
(Coined uniformly across the v0.31.0 shape-v4 wave; formalized here so later entries have a bar.)

## Survivor-provenance entry — a kept line whose right to exist was contested

```
## [vX.Y.Z] KEPT: <the line, compressed>
- **Tier-2 evidence:** <the behavior it produces / the failure it prevents — dogfood
  defect, session ruling — with a link>
```

## Re-add entry — a stripped line that came back (D7)

```
## [vX.Y.Z] RETURNED: <one-line description>
- **Evidence:** <link to the dogfood log that demanded it>
    — or —
- **Evidence:** override — no linked evidence
```

The override marker is honest by design: override clusters are a hunt signal. Any version
bump containing re-adds triggers the validator audit on the touched primitives, and an
override-count threshold (build-time parameter) flags a primitive for audit regardless
(fold S5).

## Schema content is recorded by the migration log, not here (from v0.107.0)

Schema content — what the former `plugins/mochiko/schemas/*.yaml` and
`plugins/mochiko/skills/*/schema.yaml` carried — is recorded by the migration log at
`plugins/mochiko/migrations/`, never by a strip entry. The log carries the verbatim prior content
by construction, so a strip entry would be a second, weaker copy of a record that already exists;
a schema-content edit is a new migration file instead, and the migration carries the ruling anchor
where a supersession or tombstone of protected content demands one.

The 50 snapshot files deleted at v0.107.0 — 20 under `plugins/mochiko/schemas/` and 30
`skills/*/schema.yaml` — therefore take no entries in this directory. Their record is the log plus
the wave-6 ruling (`.mochiko/brainstorms/cli-schema-delivery/record.md` D2 and D9 wave 6). The
human-readable projection of the log lives at `.mochiko/schema-views/`, regenerated and never
hand-edited. Body prose in a `SKILL.md`, a command `.md`, a `references/` file or a template is
unaffected: those are primitive edits and still take entries here.

Wave summaries do not live here — they land on ROADMAP/REGISTRY rows, as every build does.
