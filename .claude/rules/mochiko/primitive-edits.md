---
paths:
  - "plugins/mochiko/commands/**"
  - "plugins/mochiko/skills/**"
  - "plugins/mochiko/agents/**"
  - "plugins/mochiko/templates/**"
---

# Primitive-edit ceremony (strip / supersede → record → check)

Editing a shipped primitive is a **landing, not an ad-hoc edit**. Any change that REMOVES or
SUPERSEDES content — even one line, even an "obvious" cleanup — obliges both moves before the
change is done. Full contracts: `.mochiko/strips/README.md` · `templates/command-shape.md`.

- **Record** — a version-stamped entry in `.mochiko/strips/<primitive>.md` (one file per primitive,
  newest-first; stamp = the `plugin.json` version that made it):
  - a **strip entry** for an altitude / duplication cut — `Disposition: relocated → <home> | deleted`,
    `Tier failed: 1 (altitude) | 2 (no behavior named)`, verbatim content;
  - a **supersession-by-ruling entry** for a decision — `Disposition: superseded`,
    `Tier failed: n/a — supersession by ruling`, citing the ruling: a `DECISIONS.md` row + a
    `.mochiko/decisions/` ADR when no session record exists, verbatim content, `Kept deliberately`,
    `Consumers assessed`.

  An edit whose only trace is the changed file, with nothing in `.mochiko/strips/`, is **incomplete**.

- **Check** — the independent **author ≠ grader** audit: `mochiko:validator` grading against
  `templates/command-shape.md` as the explicit checklist for a command (the dedicated
  `validation-command-shape` skill was deleted at v0.45.0), the matching `validation-*` /
  `review-*` skill otherwise. The editor never grades their own edit — dispatch a separate
  validator.

**Protected content leaves ONLY by ruling.** A line in a record's protected set, marked `KEPT:`, or
traceable to a `DECISIONS.md` row may be removed only as a recorded supersession-by-ruling. A silent
deletion is exactly what the audit's preserved-responsibilities check reads as a regression — and it
has already caught one drop of the same line at the v0.34.0 plan pilot.

Pure additions ride the decision row (no strip note). Never edit `.mochiko/archive/**` or the
backlog trail — frozen / append-only.
