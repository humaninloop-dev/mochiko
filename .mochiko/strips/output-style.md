# Strip notes — `templates/output-style.md`

Entry formats: `strips/README.md`.

## [v0.82.0] "Not a grading dimension" rewritten to the artifact-format v3 position

- **Disposition:** superseded → "Style is not a grading dimension; undisclosed excess is". The
  section asserted "no reviewer gains a prose-volume dimension" and "the size-guidance signal
  is **reported, not graded** … no reviewer scores it", citing `artifact-format.md` rule 8 as
  authority — both claims false against the v3 rules 4/8 (undisclosed/unjustified overage is
  now an advisory reviewer finding). Caught by the v0.82.0 audit (finding 2): the landing's
  first pass amended the envelope but not this restating surface, which `commands/plan.md`'s
  new Register bullet puts on the live seat path.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/decisions/2026-08-22-verbosity-envelope-enforcement.md`; `DECISIONS.md`
  2026-08-22 row).
- **Content (superseded, verbatim):**

  ```
  Verbosity is never a review finding on a deliverable: `artifact-format.md` rule 8 stands and no
  reviewer gains a prose-volume dimension.
  ```

  ```
  - **Artifacts** — the size-guidance signal is **reported, not graded**: a producing seat whose
    artifact exceeds `artifact-format.md` rule 4's guidance carries one line in its report naming
    the delta (e.g. "overview 9 lines vs ≤ 3 default"). The lead reads it; no reviewer scores it.
  ```
- **Kept deliberately:** style-never-graded and brevity-never-a-finding (the section's core,
  now stated as such); the Reports bullet (rule 9's mechanical check) — widened to name
  every collecting lead, matching report-format v3.
- **Consumers assessed:** `report-format.md` rule 8 and `artifact-format.md` rule 11 point
  here for levels/switch only — unaffected; `commands/plan.md`'s Register bullet cites the
  surface table — unaffected; governance setup surfaces cite the switch line — unaffected.

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

## [v0.76.0] `governance-surfaces-template.md` contextual mentions → `governance-surfaces` schema — schema-based-template-guidance D1/D8
- **Disposition:** superseded → the `governance-surfaces` schema (`mochiko-cli template governance-surfaces`, or Read `plugins/mochiko/schemas/governance-surfaces.yaml` raw). Three contextual mentions reworded so no pointer dangles when the template file is deleted; the two-arm form is deliberately NOT forced here — these are "the surface that carries the switch line / Shape 5" citations, not read-instructions (plan §5 output-style row).
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/schema-based-template-guidance/record.md` D1/D3/D8; `DECISIONS.md` "Template-schema CLI ruled").
- **Content (superseded, verbatim):**
  - `edits it in place; it survives every regeneration (\`governance-surfaces-template.md\`). No` — The switch
  - `(\`governance-surfaces-template.md\` Shape 5) injects on **Read, not Write**, so it reinforces at` — Two delivery legs
  - `references (artifacts) · \`governance-surfaces-template.md\` + \`setup.md\` +` — Bound by
- **Kept deliberately:** the `report-format.md` / `artifact-format.md` mentions (not in-scope templates); the `setup.md` + `authoring-constitution` bindings; the Shape 1 / Shape 5 references (resolve through the schema).
- **Consumers assessed:** `output-style.md` is a shared template; this edit re-points only its own contextual mentions. P3 owns the 8 in-scope-template deletions; P5 owns only `output-style.md` (plan §6, disjoint from P3).

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
