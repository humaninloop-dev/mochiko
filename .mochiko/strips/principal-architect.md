# Strip notes — `agents/principal-architect`

Entry formats: `strips/README.md`. Wave context: the plan cluster wave (v0.15.0). Shared agent,
**2 consumers** (`setup` producer + `plan` feasibility reviewer) — under the D9 3-consumer threshold,
ruled in-wave; the strip below is Tier-2-tested against both consumers (the `task-architect` precedent
for the 2-consumer allowance).

## [v0.15.0] "Skills Available" scope duplication
- **Disposition:** scope enumeration relocated → the three skills themselves
  (`mochiko:authoring-constitution`, `mochiko:analysis-codebase`, `mochiko:review-feasibility`), each
  of whose `description:` single-sources its scope; the skill **names + a one-line reach-for-it hint
  are kept** (the team-form function — teammates ignore `skills:` frontmatter; the load-bearing
  "greenfield-or-brownfield-branch, no separate brownfield skill" and "never the constitution" nuances
  are preserved in the hints)
- **Tier failed:** 1 (a second home for each skill's scope)
- **Content:** the three full scope paragraphs in the "Skills Available" bullets — the
  `authoring-constitution` paragraph ("Write governance principles with enforcement, testability, and
  rationale — formulating a ratified statement … three-part principles … greenfield … brownfield
  branch — there is no separate brownfield skill"), the `analysis-codebase` paragraph ("Analyze
  existing codebases for patterns, architecture, and essential-floor status …"), and the
  `review-feasibility` paragraph ("Adversarially hunt cross-artifact contradictions, impossibilities,
  and buildability conflicts … 3-state `feasible / needs-revision / infeasible` verdict … operates
  over those artifacts, never the constitution.").
- **Consumers assessed:** **setup + plan (both).** Setup (producer seat): `authoring-constitution` +
  `analysis-codebase` scopes are single-sourced in those skills' `description:` fields — a teammate
  spawned as the constitution producer learns its skills from the in-body names (teammates ignore
  `skills:` frontmatter) and reaches the full scope in the skill; the strip holds. Plan (feasibility
  reviewer seat): `review-feasibility`'s scope is single-sourced in its `description:` — a teammate
  spawned as the feasibility reviewer learns the skill name + hint and reaches the six-class procedure
  in the skill; the strip holds. One instance of the 7-agent library-wide "Skills Available" pattern;
  ruling in-wave is D9-authorized (2 consumers), consistent with the `task-architect` 2-consumer
  ruling (v0.14.0).

## [v0.15.0] KEPT: the "Three-Part Rule" + "Essential Floor Knowledge" persona sections
- **Tier-2 evidence:** persona altitude (what the architect cares about — its judgment lens), with
  explicit single-source references: the four Essential-Floor categories are named as persona, their
  canonical definitions deferred ("lives in `authoring-constitution`'s `references/ESSENTIAL-FLOOR.md`
  … rather than working from a copy in this persona"); the Three-Part Rule is stated as a value
  ("Without all three, reject it or fix it"), not the ledger's GI-keyed Three-Part metadata *schema*,
  which lives in `authoring-constitution`. Tested against both consumers — load-bearing persona for
  setup (constitution producer), inert-but-harmless persona for plan (feasibility reviewer). Matches
  the `task-architect` persona keep; distinct from the "Skills Available" scope catalog stripped above.
