# Strip notes — `skills/testing-end-user/`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`, D3 + D6a/b; rulings ratified
2026-07-23). Skill-succinctness wave-1 entries atop (design:
`.mochiko/brainstorms/skill-succinctness-strip/record.md`, batch-ratified 2026-07-25): body
246 → 208 lines, 38 cut = 15% — in the 10–40 previously-stripped band.

## [v0.25.0] Evidence Types capture-method table (4 rows)
- **Disposition:** relocated → `references/EVIDENCE-CAPTURE.md` (already catalogues all four types with full capture mechanics — verified before landing; type names kept in the pointer line)
- **Tier failed:** 1 (index copy of the reference's own sections)
- **Content:** console/screenshot/logs/timing → capture-method rows
- **Consumers assessed:** TEST-GRAMMAR grammar seam untouched (vocabulary stays with `patterns-vertical-tdd`); 7 consumer files checked, none reference the table

## [v0.25.0] Quality-gate YAML report-format example (12 lines)
- **Disposition:** relocated → `references/REPORT-TEMPLATES.md` (the declared report-format home since v0.22.0; `quality_gates` documented there at lines 30/46/89/108 — verified before landing)
- **Tier failed:** 1 (format example restating the home's field table)
- **Content:** the three-gate `quality_gates:` YAML block + its two explanation lines
- **Consumers assessed:** none reference the example

## [v0.25.0] Common Mistakes densified: 6 subsections → 6-row table (net −27 lines)
- **Disposition:** compressed in place (densification, zero deletions — every mistake/failure/fix survives as a row; wave-2 artifact-densification precedent)
- **Tier failed:** n/a — no content left the skill; form only
- **Content:** the six What-goes-wrong/Fix subsections (setup validation, background cleanup, evidence truncation, PASS-without-asserts, proceeding-after-reject, skipped checkpoint)
- **Consumers assessed:** none reference the subsection headings

## [v0.22.0] Per-outcome report scaffolds → machine-first verification-report file
- **Disposition:** relocated/contracted → `references/REPORT-TEMPLATES.md` (rewritten): the persisted per-cycle/final-validation report is YAML frontmatter (per-task results, quality gates, classification, recommendation) with a `## Failures` section only on FAIL/PARTIAL/TIMEOUT/ERROR
- **Tier failed:** consumption evidence (epic F-c): sole live consumer is the lead's verdict; kinako's 16 verification reports (~9.9k B avg) carried the full Setup/Actions/Asserts scaffold per report, ×16
- **Content:** the five per-outcome markdown templates (Success minimal / Failure rich / Partial / Timeout / Error) with per-report `**Description**/**Result**/**Duration**/**Recommendation**` blocks, full assertion + actions tables and analysis on every non-success. Preserved: rich-on-failure (S8 — the failure detail is the `## Failures` section), checkpoint presentation formats, truncation rules + full-log pointers, evidence-capture discipline. The prior "Reports are not persisted to disk" storage contradiction with implement.md's per-cycle files resolved: the checkpoint presentation is in-memory; the per-cycle aggregate file persists.

## [v0.22.0] `references/TESTING-EVIDENCE.md` archived (deleted)
- **Disposition:** deleted (D6b) — full content in git history (`plugins/mochiko/skills/testing-end-user/references/TESTING-EVIDENCE.md`, removed at v0.22.0); index line removed from SKILL.md Reference Files
- **Tier failed:** 2 (provenance, not procedure — the RED/GREEN/REFACTOR hardening record for the skill's anti-rationalization content; 4,444 B shipped with no runtime consumer)
- **Content (compressed):** the TDD build record: RED-phase pressure scenarios (simple-CLI / time-pressure / repeated-test / background-process / partial-success) with 8 verbatim captured rationalizations; GREEN-phase verification that the skill's Red Flags + Common Rationalizations tables counter each; REFACTOR-phase loophole closure.
