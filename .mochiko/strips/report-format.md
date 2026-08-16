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

## [v0.77.0] `Consumed by:` routing re-pointed — deleted report templates + the two report-format references → the seven report schemas
- **Disposition:** superseded → the `**Consumed by:**` line now names the seven report schemas in `plugins/mochiko/schemas/` (two-arm framing), replacing "the report templates in this directory" plus the two named references (`executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md`, `testing-end-user/references/REPORT-TEMPLATES.md`) — all seven deleted by this ratchet.
- **Tier failed:** n/a — supersession by ruling (schema-based-template-guidance D3 later-ratchet + user ruling 2026-08-16; `DECISIONS.md` "Template-schema ratchet" row) — P5 drift finding: the routing named the two reference files this wave deletes.
- **Content (verbatim, the superseded routing):**
~~~
**Format version:** v2 (2026-08-01) · **Consumed by:** the report templates in this
directory, `executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md`,
`testing-end-user/references/REPORT-TEMPLATES.md`.
~~~
- **Kept deliberately:** the format version number + date stay (a consumer still learns which revision it reads); the envelope body and all shared rules are untouched — only the documentation of *who consumes it* was corrected.
- **Consumers assessed:** the seven report schemas each still carry `form: report-format.md` — the envelope they consume is unchanged. `templates/artifact-format.md` + `templates/output-style.md` cross-reference this envelope (as twin/register, not payload-carriers) — untouched; they were never in the `Consumed by:` list.

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
