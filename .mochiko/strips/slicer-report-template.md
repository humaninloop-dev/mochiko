# Strip notes — `templates/slicer-report-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (rulings
ratified 2026-07-23: producer disclosures machine-first; rationale lives in the deliverable).

## [v0.49.0] Template retired — slicer self-disclosure dies with the standalone slice run
- **Disposition:** superseded → none; null-exit reasoning now rides the spec's single-slice line + specify's acceptance conversation; file deleted
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` row 2026-08-02 "Task layer de-granularized + slice dissolved into specify (D1–D9)"; record `.mochiko/brainstorms/plan-task-granularity/record.md`, D5+D6)
- **Content:** the producer's per-round self-disclosure schema (drafted decomposition or null-exit recommendation, slicing rationale, Feature-Done coverage). Full text: git history at v0.48.0.
- **Consumers assessed:** the retired slice command was the sole binder; router row removed.

## [v0.22.0] Prose disclosure sections → frontmatter fields; null-exit kept first-class
- **Disposition:** contracted in place (template rewritten)
- **Tier failed:** consumption evidence (epic F-c part 2) + restatement: `slices.md` carries its own slicing rationale, foundation designation, extend obligations, and Feature-Done section
- **Content:** `## What Was Produced` → `produced:` + the `slices:` summary fields (count/foundation/stories_covered); `## What Changed This Round` → `changed_this_round:`; `## Slicing Rationale` prose (restated the overlay) → `slicing_notes:` (only homeless judgment calls); `## Feature-Done Coverage` prose → `sc_coverage:` disclosure (the map itself is in slices.md); `## Governance Alignment` → one-liner; `## Open Questions` → list (un-homeable cross-cutting story flag preserved); `## Handoff to Review` → `handoff:` pointer; optional artifacts table folded in. Preserved: the null exit as a first-class outcome — `null_exit:` field + the mandatory `## Null-exit reasoning` conditional prose block; no-self-verdict; output location.
