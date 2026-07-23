# Strip notes — `templates/advocate-report-template.md`

Entry formats: `strips/README.md`. Wave context: workflow-token-reduction wave 1 (design:
`.mochiko/brainstorms/workflow-token-reduction/record.md`; wave rulings ratified 2026-07-23:
all report formats machine-first YAML, strengths → one-line field).

## [v0.22.0] Prose report shape → machine-first findings YAML; What's Strong → `strengths:` field
- **Disposition:** contracted in place (template rewritten); `What's Strong` prose section → the one-line `strengths:` frontmatter field (user-ruled: keep the anti-rubber-stamp discipline at one line)
- **Tier failed:** consumption evidence (epic F-c part 2): round reports are consumed in-round by the lead's verdict and relayed as gap lists; no downstream stage reads them
- **Content:** the markdown Gaps Found table (ID/Type/Description/Severity) → the `findings:` YAML list (same taxonomy: Missing/Ambiguous/EdgeCase/Assumption/Contradiction; severities unchanged); the `## Verdict` prose block (Status/Rationale) → `verdict:` + `verdict_basis:` fields (same three states: ready/needs-revision/critical-gaps); `## What's Strong` free prose → `strengths:` one-liner. Preserved: Clarifications-Needed with concrete options + why-it-matters (gate fuel), the recommended-not-clearing verdict doctrine. Added: `incremental:`/`scope:` fields (review-plan-artifacts' Phase-2 incremental mode, formerly an inline divergent shape in that skill).
- **Re-add trigger:** a lead verdict or producer revision demonstrably starved by the one-line findings compression (evidence-gated, marked override).
