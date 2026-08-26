# Strip notes — `templates/advocate-report-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`; wave rulings ratified 2026-07-23:
all report formats machine-first YAML, strengths → one-line field).

## [v0.91.0] `incremental:` examples re-keyed off plan's Phase-2 mode (both sites)

- **Disposition:** superseded → a surviving incremental case. Both sites illustrated the
  `incremental:`/`scope:` field with "plan's Phase-2 incremental mode" — a mode of a command
  retired this wave, so the only example the field carried pointed at something that no longer
  runs. The field, its pairing with `scope:`, and its purpose are unchanged; only the illustration
  moved to a delta review that still happens: a re-review of the design-phase output after a fix
  round.
- **Tier failed:** n/a — supersession by ruling
  (`.mochiko/brainstorms/plan-stage-utility/record.md` D1 — plan retires, and the design phase it
  is replaced by is graded by a non-author seat; D4 — the design checkpoint and card confirm are
  where that grading lands; `DECISIONS.md` 2026-08-26 row.)
- **Content (superseded, verbatim, two sites):**
  (1) The frontmatter comment on `incremental:`: "true when this round reviews only a delta (e.g.
  plan Phase-2 incremental mode) — name the scope in `scope:`".
  (2) Usage Note 5: "set on rounds that review a delta rather than the full artifact set (e.g.
  plan's Phase-2 incremental mode), so the verdict's coverage is honest".
  Both now read "(e.g. a re-review of the design-phase output after a fix round)".
- **Kept deliberately:** the field pair itself and the reason it exists — an incremental round must
  declare its narrowed scope so the verdict's coverage is not read as full-set coverage. That
  honesty rule is the substance here and is untouched, as are all other usage notes, the findings
  schema, and the verdict values. The re-key preserves the example's *shape* (a named delta round
  within a larger run), so the field still teaches what it always taught.
- **Consumers assessed:** `mochiko:devils-advocate` fills this template, and its remit spans
  several review targets beyond the retired plan run, so nothing about the persona's scope depends
  on the example that moved. `templates/report-format.md` is the governing envelope and is
  untouched — this is an example string edit, not a field or section change.

## [v0.91.0] Shared-emit-shape line re-keyed: "the specify and plan review seats" → "specify and implement" — plan-stage retirement D1

- **Disposition:** superseded → "The shared emit shape for the specify and implement review
  seats."
- **Tier failed:** n/a — supersession by ruling (`DECISIONS.md` 2026-08-26 plan-stage-retirement
  row → `.mochiko/brainstorms/plan-stage-utility/record.md` D1: `/mochiko:plan` retires and
  `/mochiko:implement` becomes the single downstream run; wording ruled by the wave lead
  2026-08-26).
- **Content (superseded fragment, verbatim):**

  ```
  needed, and a recommended verdict. The shared emit shape for the specify and plan review
  ```

- **Kept deliberately:** the template's whole contract — the adversarial-critique framing (gaps
  found, clarifications needed, a recommended verdict), the machine-first envelope, the findings
  schema, and the conditional-prose rules. Only the pair of workflows sharing the shape is
  renamed; it is still shared by exactly two, and the reviewer seats it serves are unchanged.
- **Budget:** templates are outside the three measured budget classes (skill body, skill
  `description:`, agent `description:`) — no measurement owed.
- **Consumers assessed:** the router's `advocate-report-template` row was re-keyed to "**shared
  (specify + implement)**" in this wave's main batch — **this closes the last disagreement
  between the router and a primitive it indexes**, which this seat created there and reported.
  `mochiko:review-plan-artifacts` emits on this shape (re-scoped this wave, still emitting);
  `templates/report-format.md` carries the envelope beneath it, already clean.

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
