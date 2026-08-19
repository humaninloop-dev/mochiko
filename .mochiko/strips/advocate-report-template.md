# Strip notes — `templates/advocate-report-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`; wave rulings ratified 2026-07-23:
all report formats machine-first YAML, strengths → one-line field).

## [v0.80.0] Seat list corrected to the two live seats — slice-vocabulary purge

- **Disposition:** superseded → "the specify and plan review seats" — the two seats whose skills
  actually bind this template.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/decisions/2026-08-19-slice-vocabulary-purge.md`; the `tasks` seat's retirement is
  the pre-existing `DECISIONS.md` 2026-08-02 row, built at v0.49.0).
- **Content (verbatim, the superseded sentence):**

  ```
  The shared emit shape for the specify / plan / tasks /
  slice review seats.
  ```

  Replaced by:

  ```
  The shared emit shape for the specify and plan review
  seats.
  ```

- **Kept deliberately:** everything else in the preamble — the artifact-under-review framing,
  the envelope pointer, and the "this file carries only the review payload" boundary. The whole
  template body (frontmatter schema, Clarifications section, six usage notes) is untouched.
- **Consumers assessed:** verified by grep for `advocate-report-template` across
  `plugins/mochiko/`. Exactly two skills bind it — `review-specifications` (the specify seat)
  and `review-plan-artifacts` (the plan seat, which per its own description grades the cycle
  cards, so `tasks.md` review rides the plan seat and is not a seat of its own). Neither named
  seat still exists as a skill: `review-slices` folded into `review-specifications` and
  `review-task-artifacts` into `review-plan-artifacts` at the v0.49.0 task-de-granularization
  build (`DECISIONS.md` 2026-08-02), and `plugins/mochiko/skills/` contains neither. The router
  row in `skills/mochiko/SKILL.md` carried the same stale list and was corrected in the same
  wave (`.mochiko/strips/mochiko.md`).

## [v0.22.0] Prose report shape → machine-first findings YAML; What's Strong → `strengths:` field
- **Disposition:** contracted in place (template rewritten); `What's Strong` prose section → the one-line `strengths:` frontmatter field (user-ruled: keep the anti-rubber-stamp discipline at one line)
- **Tier failed:** consumption evidence (epic F-c part 2): round reports are consumed in-round by the lead's verdict and relayed as gap lists; no downstream stage reads them
- **Content:** the markdown Gaps Found table (ID/Type/Description/Severity) → the `findings:` YAML list (same taxonomy: Missing/Ambiguous/EdgeCase/Assumption/Contradiction; severities unchanged); the `## Verdict` prose block (Status/Rationale) → `verdict:` + `verdict_basis:` fields (same three states: ready/needs-revision/critical-gaps); `## What's Strong` free prose → `strengths:` one-liner. Preserved: Clarifications-Needed with concrete options + why-it-matters (gate fuel), the recommended-not-clearing verdict doctrine. Added: `incremental:`/`scope:` fields (review-plan-artifacts' Phase-2 incremental mode, formerly an inline divergent shape in that skill).
- **Re-add trigger:** a lead verdict or producer revision demonstrably starved by the one-line findings compression (evidence-gated, marked override).
