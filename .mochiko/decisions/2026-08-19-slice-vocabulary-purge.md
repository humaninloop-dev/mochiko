# Slice vocabulary purged from shipped primitives

**Status:** ruled · **Date:** 2026-08-19
**Context:** a user dogfood run (external project, post-v0.75.0) generated both cycles and "slices" — the model reified "slice" as a unit noun alongside cycles. The slice as pipeline unit died at v0.57.0 (`feature-map-layer` D4/D22: "slices die, the feature is the pipeline unit"), but the v0.75.0 cycle re-anchor (`vertical-tdd-complexity-and-qa-role`) deliberately kept "vertical slice" as shape language, and several older surfaces still carried slice-unit residue: the `SLICE-IDENTIFICATION.md` reference filename, "Each card is one vertical slice" in the `tasks` schema, the dead `slice: <s#>` report-envelope field (a slice-scoped run has been impossible since v0.57.0), "slice review seats" in the advocate template, the router's stale "vertical-slice identification (foundation vs feature)" row (foundation/feature card types died at v0.75.0 D3), and `authoring-architecture`'s "Slice-scoped landing" paragraph with its stale `.mochiko/specs/<feature>/architecture.md` path. This vocabulary trains a fresh lead to mint slices. Observation logged against the v0.75.0 first-live-run watch.

**Decision (user-approved wave plan, 2026-08-19):** purge "slice" wherever it names a pipeline/TDD unit; generic-English uses survive untouched. Replacement unit language: "vertical increment" / "bundle" / "cycle" as context demands. Specifics:

- `patterns-vertical-tdd`: description drops "vertical-slice" ("cycle cards"); body unit-noun uses re-worded; `references/SLICE-IDENTIFICATION.md` renamed `BUNDLE-IDENTIFICATION.md` with its unit-noun uses purged; `TEST-GRAMMAR.md` anti-pattern line re-worded. The `'vertical slice'` SHOULD-trigger phrase in the description is **kept deliberately** — it routes users still using the legacy term.
- `schemas/tasks.yaml`: "Each card is one vertical slice" becomes "one vertical increment" (both sites).
- `templates/report-format.md`: the `slice: <s#>` envelope field deleted; echoes in `executing-tdd-cycle/references/CYCLE-REPORT-FORMAT.md` and `testing-end-user/references/REPORT-TEMPLATES.md` dropped.
- `agents/devils-advocate.md`: the stale "its Delivery Slices section included" clause dropped (`spec.yaml` has no such section).
- `templates/advocate-report-template.md` + router `skills/mochiko/SKILL.md`: "specify + plan + slice" seat lists drop slice; the router's `patterns-vertical-tdd` row re-keyed to bundle identification (also retiring the stale foundation-vs-feature clause).
- `skills/authoring-feature-map/SKILL.md`: table term "Vertical slice (cycle)" becomes "Cycle".
- `skills/authoring-architecture/SKILL.md`: "Slice-scoped landing" paragraph re-worded feature-scoped; stale feature-artifact path corrected to the current `.mochiko/features/FEAT-XXX/` home.

**Kept (generic English, out of scope):** `review-plan-artifacts` "greppable slice" (×2) · `check-artifacts.py` "judgment slice" · `governance-intent.yaml` "agenda slice" · `INTERROGATION-AGENDA.md` "agenda slice" · `authoring-constitution` "path-identifiable slice" · `specify.md` migration-history line naming "slice-form specs" (correctly historical).

**Rationale:** doctrine vocabulary is the model's training signal at run time; a unit noun that survived its unit invites reification. A total word ban is overkill — only unit-noun uses mislead.

**Alternatives:** total word ban (rejected — generic English uses are harmless and a ban costs clarity); leave as-is and rely on leads to know slices are dead (rejected — the dogfood run is the counterexample).
