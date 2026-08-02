# Strip notes — `templates/output-style.md`

Entry formats: `strips/README.md`.

**Wave context (v0.44.0 — the D7 leakage scrub).** `verbosity-caveman-ops-separation` D7 as
folded at review (S4): **full scrub** of ops leakage from the shipped tree, with no
changelog-worthy detail lost — every removed block is preserved verbatim below. Ruling:
`DECISIONS.md` 2026-08-01 "Output verbosity, caveman & ops separation ruled" row.

**The leak test this wave used, recorded so a future sweep inherits it: *whose artifact does the
pointer name?*** Mochiko's own ops records — `.mochiko/strips/`, `.mochiko/brainstorms/`,
`.mochiko/decisions/`, `.mochiko/archive/` — are leaks: they resolve to nothing in an installed
plugin. Adopter runtime paths (`.mochiko/specs/`, `.mochiko/memory/`) and the KM module's
document contracts are the **user's** artifacts and are untouchable. A prefix-based sweep on
`.mochiko/` would gut the KM module and the brainstorm command; 101 of this tree's 146
`.mochiko/` references were correctly left alone on that test.

## [v0.46.0] Chat-surface binding re-pointed
- **Disposition:** superseded → each command's ground rules (the chat register's binding site after the shape's deletion)
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** the per-surface table's "(`command-shape.md` Layer 1)" → "(every command's ground rules bind it here)"; the footer's "**Bound by:** `command-shape.md` Layer 1 (chat)" → "each command's ground rules (chat)".
- **Consumers assessed:** 6 commands (each ground-rules block now names this file).

## [v0.44.0] Style version-history block relocated (class 2, 719 B / 8 lines)
- **Disposition:** superseded → relocated **verbatim** into this note (below). In-file residue: the
  bare stamp plus the live routing (`Bound by`), which is wiring a run consumes, not history.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim, the whole block as it stood at the scrub):**
```
**Style version:** v1 (2026-08-01 — `verbosity-caveman-ops-separation`
(`.mochiko/brainstorms/verbosity-caveman-ops-separation/record.md`) D1 · D2 · D3 · D5, with the
review folds S3 (clause manifest, disclose-once), S9 (adoption boundary), S10 (per-surface
switch values), S11 (ban-as-principle), S12 (plain wins), S13 (failure narratives `full`)) ·
**Bound by:** `command-shape.md` Layer 1 (chat) · `report-format.md` (reports) ·
`artifact-format.md` with the artifact templates and authoring references (artifacts) ·
`governance-surfaces-template.md` + `setup.md` + `authoring-constitution` (Shape 1's switch line,
the always-loaded carrier · Shape 5's `paths`-scoped rules file, edit-time reinforcement).
```
- **Kept deliberately:** the version *number* and its date stay in the file — a consumer still
  learns which revision it is reading; only the per-revision narrative left.
