# Strip notes — `templates/report-format.md`

Entry formats: `strips/README.md`.

<!-- Wave context: wave 6 of the CLI schema-delivery build (v0.107.0) — the end state. No schema
file ships in the plugin: the 20 files under `plugins/mochiko/schemas/` and the 30
`skills/*/schema.yaml` were deleted, and every delivery they served now has a CLI form. Ruling for
the [v0.107.0] entry below: `.mochiko/brainstorms/cli-schema-delivery/record.md` D9 wave 6, with
the `DECISIONS.md` 2026-09-05 row and that session's `wave6-plan.md`. Pre-edit verbatim text:
`git show 62aa99d:plugins/mochiko/templates/report-format.md`. -->

## [v0.107.0] rule 9's file citation of the implement schema — now cited by rule id alone

- **Disposition:** superseded → the bare rule ids `impl.escalation-batching` /
  `impl.finding-severity-routing`
- **Tier failed:** n/a — supersession by ruling (`.mochiko/brainstorms/cli-schema-delivery/record.md`
  D9 wave 6; `DECISIONS.md` 2026-09-05)
- **Content:** "`plugins/mochiko/schemas/implement.yaml`'s `impl.escalation-batching` /
  `impl.finding-severity-routing` rules"
- **Kept deliberately:** both rule ids, which are the citation that matters and still resolve in
  the delivered implement rules; and the whole of rule 9 — the prose-on-a-clean-report defect,
  the devolved-branch terms it keys to, the any-lead-collecting-any-report-class extension, and
  the bounce-to-seat remedy. Only the file path in front of the ids left, because the file is gone
  and the ids are the durable handle.
- **Consumers assessed:** `report-format.md` is the shared envelope every workflow report follows.
  The two ids are cited here only as the source of the devolved-branch terms; no consumer reads
  the path. Nothing else in the templates directory names a schema file.

## [v0.91.0] Dead `commands/plan.md` pointer re-homed; `plan.md` struck from the not-a-report list

- **Disposition:** superseded → `commands/implement.md`. Two sites named surfaces the plan-stage
  retirement killed. The footer site was the material one: `commands/plan.md` was deleted this
  same wave, so the Consumed-by line had become a **dead pointer** — a GI-005 defect, not a
  cosmetic staleness. The report-envelope binding it recorded did not disappear with the command;
  it moved to implement's Reports tool, which now names this file and the `sufficiency-report.md`
  that rides the envelope, so the pointer re-homes rather than being dropped.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/plan-stage-utility/record.md` D1 — implement becomes the single
  downstream run; D4 and the Build surface — the sufficiency verdict lands as a report under
  `templates/report-format.md` and implement's Reports tool gains the envelope binding; `plan.md`
  the summary artifact dies with no restatement artifact; `DECISIONS.md` 2026-08-26 row.)
- **Content (superseded, verbatim, two sites):**
  (1) The opening not-a-report carve: "(Deliverables — spec.md, plan.md, tasks.md, the working
  code — are not reports and are not governed here.)" Now: "(Deliverables — spec.md, tasks.md,
  the design-phase deltas, the working code — are not reports and are not governed here.)"
  (2) The Consumed-by footer's closing member: "`commands/plan.md`'s report-envelope binding."
  Now: "`commands/implement.md`'s report-envelope binding."
- **Kept deliberately:** all nine shared rules, the envelope block, the `report:` type enumeration
  (including `feasibility` and `disclosure`, whose producing surfaces survive re-scoped), the
  machine-first doctrine, and the "Who reads a report" section are untouched — this is a pointer
  and example edit only, and the format version stays v3. Rule 9's *other* reference to
  `commands/implement.md` (the cycle checkpoint's devolved-branch terms) was already live and is
  unchanged; only the footer pointer moved. The carve's purpose — marking the deliverable/report
  boundary — is unchanged; the boundary itself did not move, only two of the examples naming it.
- **Consumers assessed:** the re-pointed target was verified live before the edit rather than
  assumed — `commands/implement.md` carries the Reports tool binding this file by path, listing
  `sufficiency-report.md` alongside the cycle, verification, final-validation, and built-vs-signed
  diff reports, so the new pointer resolves. The three other Consumed-by members
  (`executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md`,
  `testing-end-user/references/REPORT-TEMPLATES.md`, and the report templates in this directory)
  are unaffected. `templates/artifact-format.md` is the sibling envelope governing the deliverable
  side of the carve; it was re-keyed in the same wave and the two lists now agree.

## [v0.82.0] Rule 9's mechanical bounce widened to every lead-collected report (format v3)

- **Disposition:** superseded → the widened rule 9: the prose-on-a-clean-report defect check was
  enforceable only at `commands/implement.md`'s cycle checkpoint (cycle/verification reports),
  leaving plan-run reviewer reports (feasibility, review, disclosure) with no mechanical
  enforcement hook — the kinako EPIC-001 run's 86KB feasibility review is the evidence. Any
  lead collecting any report class now reads it mechanically and bounces an envelope-breaking
  report to its seat for re-issue.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/decisions/2026-08-22-verbosity-envelope-enforcement.md`; `DECISIONS.md`
  2026-08-22 row).
- **Content (superseded — the enforcement scoping, faithfully compressed):** rule 9 previously
  grounded the defect only in "the deterministic-and-clean clearing conditions (the dispatching
  command's devolved-branch terms — `commands/implement.md`'s cycle checkpoint)", with no
  obligation on any other collecting lead.
- **Kept deliberately:** rule 2's closed prose set for cycle and verification reports —
  unchanged; the read-mechanically instruction (status and section headings, never prose
  quality) — unchanged; the artifact-format rule-8 twin cross-reference — unchanged. The
  Consumed-by addition (`commands/plan.md`'s report-envelope binding) rides the decision row.
- **Consumers assessed** (re-run at the audit fix round — the first pass asserted "templates
  unchanged" without asking whether the newly-bound classes' templates could carry their
  mandated content, audit finding 4): `commands/implement.md`'s cycle checkpoint reads
  unchanged (its devolved-branch terms still cite this rule); `commands/plan.md` gains its
  binding in the same landing; `templates/feasibility-report-template.md` **gains the
  `hunt_coverage` field in the same landing** so the widened bounce and `review-feasibility`'s
  disclosure floor are jointly satisfiable (strip:
  `.mochiko/strips/feasibility-report-template.md` [v0.82.0]); rule 9 also gains the
  clarifying clause that "unsanctioned" reads against each class's own payload home for
  classes outside rule 2's closed set.

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
