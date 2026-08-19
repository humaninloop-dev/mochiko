# Strip notes — `templates/artifact-format.md`

Entry formats: `strips/README.md`.

## [v0.81.0] Pipeline artifact chain drops `nfrs.md` — product-architecture-schema D12

- **Disposition:** deleted from the enumeration. The envelope's opening line lists the
  deliverables that follow it; `nfrs.md` is no longer a deliverable (D12 homes NFR-XXX on the
  architecture store's concern rows). The store is **not** added to the chain: it is a standing
  baseline governed by its own schema and the desk's loop, not a per-run pipeline deliverable
  following this envelope.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/product-architecture-schema/record.md` D12; `DECISIONS.md` 2026-08-19).
- **Content (superseded, verbatim):**

  ```
  pipeline artifact chain (`spec.md` · `requirements.md` · `constraints-and-decisions.md` ·
  `nfrs.md` · `data-model.md` · `contracts/api.yaml` · `quickstart.md` · `plan.md` ·
  `tasks.md`) and setup's `codebase-analysis.md`.
  ```
- **Kept deliberately:** every other artifact in the chain — `quickstart.md` included, user-ruled
  keep at D12 — plus `codebase-analysis.md` and the reports/records/governance carve-outs. All
  the envelope's rules (dense by construction, statement-carries-the-content, per-artifact ID
  index, density-is-not-a-gap) are untouched and still govern every surviving artifact.
- **Consumers assessed:** shared envelope, referenced by every artifact template and authoring
  skill. Its consumers cite the envelope's *rules*, not its chain enumeration, so no downstream
  restatement changed — verified by grep across `plugins/`. The one file that also enumerated the
  chain, `schemas/tasks.yaml`'s generated-from pointer, was re-keyed in the same pass (logged in
  `strips/tasks-template.md`). Found at the V4 delta pass (B2) — the first pass swept skills and
  the router but not the shared envelopes.

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

## [v0.49.0] Artifact chain drops task-mapping.md + slices.md
- **Disposition:** superseded → both artifacts retired (mapping content on cycle cards; slicing a spec section)
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D3+D6)
- **Content:** "`task-mapping.md` · " and " · `slices.md`" in the deliverable-chain enumeration.
- **Consumers assessed:** authoring skills + review checklists named in the footer (all co-edited or retired this wave).

## [v0.44.0] Format version-history block relocated (class 2, 744 B / 10 lines)
- **Disposition:** superseded → relocated **verbatim** into this note (below). In-file residue: the
  bare stamp plus the live routing (`Consumed by`), which is wiring a run consumes, not history.
- **Tier failed:** n/a — supersession by ruling (`verbosity-caveman-ops-separation` D7 + S4; the `DECISIONS.md` 2026-08-01 row above)
- **Content (verbatim, the whole block as it stood at the scrub):**
```
**Format version:** v2 (2026-08-01 — `verbosity-caveman-ops-separation` D1/D5: rule 4's
reported-not-graded size signal, plus rule 11's register binding; v1 2026-07-24 —
workflow-token-reduction wave 2) · **Governed by:**
the workflow-token-reduction epic record (D4 + the wave-2 rulings R1–R4/T1–T4) and
`verbosity-caveman-ops-separation` (D1 · D5 as folded at review, S6) ·
**Consumed by:** the artifact templates in this directory, the artifact-authoring skills
(`authoring-requirements`, `authoring-user-stories`, `authoring-technical-requirements`,
`patterns-entity-modeling`, `patterns-api-contracts`, `patterns-vertical-tdd`,
`authoring-slices`, `analysis-codebase`), and the review-skill checklists that grade the
artifacts.
```
- **Kept deliberately:** the version *number* and its date stay in the file — a consumer still
  learns which revision it is reading; only the per-revision narrative left.
