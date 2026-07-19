# Strip Notes — the minimalism pass's per-primitive log

<!--
Convention home (design: .mochiko/brainstorms/pattern-codification-and-minimalism/record.md,
D6 + folds S5/S10; ruling: ROADMAP Key Decisions 2026-07-18). This directory is deliberately
NON-LOADED: no command or runtime surface references it — the log must never itself cost
context. It is read at framework-maintenance time only (strip waves, dogfood re-add reviews,
validation-command-shape audits). Strip notes never live inside commands/ (files there
register as commands) and never inline in SKILL.md/command bodies.
-->

One file per primitive: `strips/<primitive>.md` (e.g. `strips/brainstorm.md` for
`commands/brainstorm.md`, `strips/loop-discipline.md` for the skill). Entries are appended
newest-first, each stamped with the plugin version that made it. Three entry types:

## Strip entry — a line that left the primitive

```
## [vX.Y.Z] <one-line description>
- **Disposition:** relocated → <single-sourced home> | deleted
- **Tier failed:** 1 (altitude — restated doctrine/pattern) | 2 (no behavior/failure named)
- **Content:** <the stripped text, verbatim or faithfully compressed>
- **Consumers assessed:** <shared primitives only: every consumer checked, per D9's guard>
```

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

Wave summaries do not live here — they land on ROADMAP/REGISTRY rows, as every build does.
