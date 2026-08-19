# Strip notes — `templates/report-format.md`

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

## [v0.80.0] Envelope `slice:` field deleted — slice-vocabulary purge

- **Disposition:** superseded → nothing; the field is deleted outright. No replacement key: a
  slice-scoped run has been impossible since v0.57.0 (`feature-map-layer` D4/D22 — "slices die,
  the feature is the pipeline unit"), so the envelope's scoping keys are `feature:` plus the
  per-report `cycle:`/`round:` counters.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/decisions/2026-08-19-slice-vocabulary-purge.md`).
- **Content (verbatim, the deleted envelope line):**

  ```yaml
  slice: <s#>            # only when the run is slice-scoped
  ```

- **Kept deliberately:** the rest of the envelope block unchanged — `report:`, `feature:`, and
  `round:` with its cycle-report parenthetical. The `**Format version:** v2 (2026-08-01)` stamp
  is left as-is: this edit removes a dead field rather than changing the format contract, and
  the two consumer references in the same footer are re-pointed in place (below).
- **Consumers assessed:** the two files the footer names, both edited in the same wave —
  `skills/executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md` (field-definition row) and
  `skills/testing-end-user/references/REPORT-TEMPLATES.md` (field-definition row + the storage
  paragraph's `feature/slice directory`). No other primitive referenced the field, and after
  this wave no primitive instructs writing a `slice:` frontmatter key. A repo-wide grep for the
  literal string `slice:` in `plugins/mochiko/` returns exactly one remaining hit —
  `skills/authoring-constitution/SKILL.md:68`, "governs work on a path-identifiable slice:
  layers, API surface, tests" — which is generic English prose (a colon after the noun, not a
  YAML key) and is on the ruling's Kept list.

## [v0.49.0] Deliverable list drops slices.md
- **Disposition:** superseded → slicing is a spec.md section, covered by the existing spec.md list entry
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D6)
- **Content:** "slices.md" in the not-reports deliverable list (line 7).
- **Kept deliberately:** the `slice:` frontmatter field — slice-scoped runs still exist.
- **Consumers assessed:** all report templates (envelope unchanged otherwise).

## [v0.46.0] Devolved-branch citation re-pointed
- **Disposition:** superseded → `commands/implement.md`'s cycle checkpoint (the devolved-branch home after the shape's deletion)
- **Tier failed:** n/a — supersession by ruling (ADR `.mochiko/decisions/2026-08-02-doctrine-purge-wave-1.md`; DECISIONS.md 2026-08-02 "Doctrine purge wave 1" row)
- **Content:** rule 9's "(`command-shape.md`, Layer 2)" → "(the dispatching command's devolved-branch terms — `commands/implement.md`'s cycle checkpoint)".
- **Consumers assessed:** all report templates ride the envelope unchanged.

## [v0.44.0] Format version-history block relocated (class 2, 495 B / 6 lines)
- **Disposition:** superseded → relocated **verbatim** into this note (below). In-file residue: the
  bare stamp plus the live routing (`Consumed by`), which is wiring a run consumes, not history.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim, the whole block as it stood at the scrub):**
```
**Format version:** v2 (2026-08-01 — `verbosity-caveman-ops-separation` D4: rule 2's closed
set, plus rules 8 and 9; v1 2026-07-23 — workflow-token-reduction wave 1) · **Governed by:**
the workflow-token-reduction epic record (D3 + the wave-1 rulings) and
`verbosity-caveman-ops-separation` D4, which finishes D3's intent · **Consumed by:** the
report templates in this directory, `executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md`,
`testing-end-user/references/REPORT-TEMPLATES.md`.
```
- **Kept deliberately:** the version *number* and its date stay in the file — a consumer still
  learns which revision it is reading; only the per-revision narrative left.
